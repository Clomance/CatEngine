use std::{mem::{transmute, MaybeUninit}};

use cat_engine_basement::winapi::{VirtualKeyCode, MouseButton, window::{Window, quit}};

use crate::{graphics::{Graphics}, object::{Objects, ObjectManager}};



pub enum SystemStatus{
    Next,
    Pause,
    Stop,
    Exit
}



#[derive(Debug,Clone,Copy)]
pub enum SystemEvent{
    Update,
    Keyboard{
        state:bool,
        key:VirtualKeyCode,
    },
    CharacterInput(char),
    MouseMove([u16;2]),
    MouseButton{
        state:bool,
        position:[u16;2],
        button:MouseButton,
    },
    Resize([u16;2]),
    Move([i16;2]),
    Destroy,
}



pub trait System<'s,'a>:'static{
    type Objects;
    type SharedData;

    /// Sets up system's layers, objects, etc.
    /// 
    /// Called after the `System::create` function.
    fn set_up(
        &'s mut self,
        shared:&mut Self::SharedData,
        object_manager:ObjectManager<'a>,
    )->Self::Objects;

    /// Processes the system events.
    fn handle(
        &'s mut self,
        objects:&mut Self::Objects,
        event:SystemEvent,
        window:&Window,
        shared:&mut Self::SharedData,
        manager:SystemManager<'a>
    )->SystemStatus;

    fn destroy(
        &'s mut self,
        shared:&mut Self::SharedData,
        graphics:&mut Graphics
    );
}



pub trait StartSystem<'s,'a>:System<'s,'a>{
    type CreateParameters:'s;

    fn create_shared_data(create_parameters:&mut Self::CreateParameters)->Self::SharedData;

    fn create(
        create_parameters:&mut Self::CreateParameters,
        window:&Window,
        shared:&mut Self::SharedData,
    )->Self;
}



pub struct ExtendedSystemData<S,O>{
    system:S,
    object_references:O,
    object_storage:usize,
    graphics:*mut Graphics,
}

impl<S,O> ExtendedSystemData<S,O>{
    pub fn new(
        system:S,
        object_references:O,
        object_storage:usize,
        graphics:&mut Graphics
    )->ExtendedSystemData<S,O>{
        Self{
            system,
            object_references,
            object_storage,
            graphics
        }
    }
}



/// Данные и функции системы.
pub (crate) struct SystemTable{
    /// Ссылка на данные самой системы.
    system_data:*mut (),
    /// Ссылка на расширенные данные системы.
    extended_system_data:*mut (),
    /// Ссылка на индекс хранилища объектов.
    object_storage:*mut usize,
    /// Ссылка на сохранённые ссылки на объекты.
    object_references:*mut (),
    handle:fn(
        system_data:*mut (),
        object_references:*mut (),
        event:SystemEvent,
        window:&Window,
        shared:*mut (),
        manager:SystemManager
    )->SystemStatus,
    destroy:fn(extended_system_data:*mut (),shared:*mut ())
}

impl SystemTable{
    pub fn new<'s,'a,S:System<'s,'a>>(system:S,graphics:&mut Graphics)->SystemTable{
        // Делаем затычку для сохранённых ссылок на объекты
        let empty_object_references:S::Objects=unsafe{MaybeUninit::uninit().assume_init()};
        // Упаковываем данные
        let boxed_data=Box::new(ExtendedSystemData::new(system,empty_object_references,0usize,graphics));

        let extended_system_data_reference=Box::leak(boxed_data);

        Self{
            system_data:&mut extended_system_data_reference.system as *mut _ as *mut (),
            extended_system_data:extended_system_data_reference as *mut _ as *mut (),
            object_storage:&mut extended_system_data_reference.object_storage as *mut usize,
            object_references:&mut extended_system_data_reference.object_references as *mut _ as *mut (),

            // Подгоняем lifetime
            handle:unsafe{transmute(handle_wrapper::<S> as usize)},
            destroy:destroy_wrapper::<S>,
        }
    }

    pub fn set_object_references<R>(&mut self,references:R){
        unsafe{
            (self.object_references as *mut R).write(references)
        }
    }

    pub fn system_data<'s,S>(&'s mut self)->&'static mut S{
        unsafe{
            &mut *(self.system_data as *mut S)
        }
    }

    pub fn object_storage_index(&mut self)->&mut usize{
        unsafe{
            &mut *self.object_storage
        }
    }

    pub fn handle(&mut self,event:SystemEvent,window:&Window,shared:*mut (),manager:SystemManager)->SystemStatus{
        (self.handle)(self.system_data,self.object_references,event,window,shared,manager)
    }

    pub fn destroy(&mut self,shared:*mut ()){
        (self.destroy)(self.extended_system_data,shared)
    }
}



pub struct Systems<D>{
    /// Общие данные.
    shared:D,
    /// Активные системы.
    active:Vec<SystemTable>,
    /// Неактивные системы.
    paused:Vec<SystemTable>
}

impl<D> Systems<D>{
    pub fn new(shared:D)->Systems<D>{
        Self{
            shared,
            active:Vec::new(),
            paused:Vec::new(),
        }
    }

    pub fn push<'s,'a,S:System<'s,'a>>(&mut self,system:S,objects:&mut Objects,graphics:&mut Graphics){
        let mut system_table=SystemTable::new(system,graphics);

        objects.push_new(system_table.object_storage_index());
        let object_storage=objects.get_storage(*system_table.object_storage_index());

        let system=system_table.system_data::<S>();

        let shared=unsafe{
            transmute(&mut self.shared)
        };

        // Подгоняем lifetime
        let object_manager=unsafe{
            ObjectManager::new(transmute(object_storage),transmute(graphics))
        };

        let object_references=system.set_up(shared,object_manager);

        system_table.set_object_references(object_references);

        self.active.push(system_table)
    }

    pub fn shared_data(&mut self)->&mut D{
        &mut self.shared
    }

    pub fn handle(&mut self,event:SystemEvent,window:&Window,objects:&mut Objects,graphics:&mut Graphics){
        let mut c=0;
        unsafe{
            while c<self.active.len(){
                let active_systems:&'static mut Vec<SystemTable>=std::mem::transmute(&mut self.active);
                let pause_systems:&'static mut Vec<SystemTable>=std::mem::transmute(&mut self.paused);

                let system=self.active.get_unchecked_mut(c);
                let manager=SystemManager::new(c,active_systems,pause_systems,objects,graphics);

                let status=system.handle(
                    event,
                    window,
                    &mut self.shared as *mut D as *mut (),
                    std::mem::transmute_copy(&manager)
                );

                match status{
                    SystemStatus::Stop=>{
                        let mut system=self.active.remove(c);
                        objects.remove(*system.object_storage);

                        system.destroy(&mut self.shared as *mut D as *mut ());
                    }

                    SystemStatus::Pause=>{
                        let system=self.active.remove(c);
                        self.paused.push(system)
                    }

                    SystemStatus::Exit=>{
                        quit(0);
                        break
                    }

                    SystemStatus::Next=>c+=1,
                }
            }
        }
    }
}



pub struct SystemManager<'a>{
    current_system:usize,

    active_systems:&'a mut Vec<SystemTable>,
    paused_systems:&'a mut Vec<SystemTable>,

    objects:&'a mut Objects,
    graphics:&'a mut Graphics,
}

impl<'a> SystemManager<'a>{
    pub (crate) fn new(
        current_system:usize,
        active_systems:&'a mut Vec<SystemTable>,
        paused_systems:&'a mut Vec<SystemTable>,
        objects:&'a mut Objects,
        graphics:&'a mut Graphics
    )->SystemManager<'a>{
        Self{
            current_system,
            active_systems,
            paused_systems,
            objects,
            graphics,
        }
    }

    pub fn graphics(&mut self)->&mut Graphics{
        self.graphics
    }

    pub fn object_manager(&mut self)->ObjectManager<'a>{
        unsafe{
            let system=&self.active_systems[self.current_system];
            let object_storage=self.objects.get_storage(*system.object_storage);
            ObjectManager::new(
                std::mem::transmute(object_storage),
                &mut *(self.graphics as *mut Graphics)
            )
        }
    }

    pub fn push<'s,S:System<'s,'a>>(
        &mut self,
        system:S,
        shared:&mut S::SharedData,
    ){
        let mut system_table=SystemTable::new(system,self.graphics);

        self.objects.push_new(system_table.object_storage_index());
        let object_storage=self.objects.get_storage(*system_table.object_storage_index());

        let system=system_table.system_data::<S>();

        // Подгоняем lifetime
        let object_manager:ObjectManager<'static>=unsafe{
            ObjectManager::new(transmute(object_storage),transmute(self.graphics as *mut Graphics))
        };

        let object_references=system.set_up(shared,object_manager);

        system_table.set_object_references(object_references);

        self.active_systems.push(system_table)
    }

    pub fn unpause(&mut self,index:usize){
        let system=self.paused_systems.remove(index);

        self.active_systems.push(system);
    }
}



fn handle_wrapper<'s,'a,S:System<'s,'a>>(
    system_data:*mut (),
    object_references:*mut (),
    event:SystemEvent,
    window:&Window,
    shared:*mut (),
    manager:SystemManager<'a>
)->SystemStatus{
    let system=unsafe{
        &mut *(system_data as *mut S)
    };

    let object_references=unsafe{
        &mut *(object_references as *mut S::Objects)
    };

    let shared=unsafe{
        &mut *(shared as *mut S::SharedData)
    };

    system.handle(object_references,event,window,shared,manager)
}

fn destroy_wrapper<'s,'a,S:System<'s,'a>>(
    extended_system_data:*mut (),
    shared:*mut ()
){
    let extended_system_data=extended_system_data as *mut ExtendedSystemData<S,S::Objects>;
    {
        let extended_system_data=unsafe{
            &mut *extended_system_data
        };

        let shared=unsafe{
            &mut *(shared as *mut S::SharedData)
        };

        let graphics:&'a mut Graphics=unsafe{
            &mut *extended_system_data.graphics
        };

        extended_system_data.system.destroy(shared,graphics);
    }

    // Вызываем ленивый деконструктор
    drop(unsafe{Box::from_raw(extended_system_data)});
}