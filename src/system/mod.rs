use crate::object::{
    Objects,
    ObjectEvent,
    ObjectStorage,
    ObjectManager,
};

use crate::graphics::{
    Graphics,
};

use cat_engine_basement::{
    winapi::{
        window::quit,
        window::Window,
        VirtualKeyCode,
        MouseButton,
    },
};

pub trait System<'s,'a>:'s{
    type Objects;
    type SharedData;

    /// Sets up system's layers, objects, etc.
    /// 
    /// Called after the `System::create` function.
    fn set_objects(
        &mut self,
        shared:&mut Self::SharedData,
        object_manager:ObjectManager<'a>,
    )->Self::Objects;

    /// Processes the system events.
    fn handle(
        &mut self,
        objects:&mut Self::Objects,
        event:SystemEvent,
        window:&Window,
        shared:&mut Self::SharedData,
        manager:SystemManager<'a>
    )->SystemStatus;

    fn destroy(
        &mut self,
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
        let system=&self.active_systems[self.current_system];
        let object_storage=self.objects.get_storage(system.object_storage);
        unsafe{
            ObjectManager::new(
                std::mem::transmute(object_storage),
                &mut *(self.graphics as *mut Graphics)
            )
        }
    }

    pub fn push<'s,S:System<'s,'a>+'s>(
        &mut self,
        system:S,
        shared:&'s mut S::SharedData,
    ){
        let mut boxed_system=Box::new(system);

        let object_storage_id=self.objects.create_storage();
        let object_storage=self.objects.get_storage(object_storage_id);

        let object_manager=unsafe{
            ObjectManager::new(
                std::mem::transmute(object_storage as *mut ObjectStorage),
                std::mem::transmute(self.graphics as *mut Graphics)
            )
        };

        let object_storage_references=boxed_system.as_mut().set_objects(shared,object_manager);
        let object_storage_references=Box::new(object_storage_references);

        let data=Box::leak(boxed_system);

        let system=SystemTable{
            data:data as *mut _ as *mut (),
            object_storage_references:Box::leak(object_storage_references) as *mut S::Objects as *mut (),
            object_storage:object_storage_id,
            handle:unsafe{std::mem::transmute(system_handle_wrapper::<S> as usize)},
            destroy:system_destroy_wrapper::<S>,
        };

        self.active_systems.push(system);
    }

    pub fn unpause(&mut self,index:usize){
        let system=self.paused_systems.remove(index);

        self.active_systems.push(system);
    }
}

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

pub (crate) struct SystemTable{
    data:*mut (),
    pub object_storage_references:*mut (),
    pub object_storage:usize,
    handle:fn(*mut (),*mut (),SystemEvent,&Window,*mut (),SystemManager)->SystemStatus,
    destroy:fn(*mut (),*mut (),*mut (),*mut Graphics),
}

impl SystemTable{
    #[inline(always)]
    fn handle(
        &self,
        event:SystemEvent,
        window:&Window,
        shared:*mut (),
        system_manager:SystemManager
    )->SystemStatus{
        (self.handle)(
            self.data,
            self.object_storage_references,
            event,
            window,
            shared,
            system_manager
        )
    }

    #[inline(always)]
    fn destroy(&self,shared:*mut (),graphics:*mut Graphics){
        (self.destroy)(
            self.data,
            self.object_storage_references,
            shared,
            graphics
        )
    }
}

pub (crate) struct Systems<D>{
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

    pub fn push<'s,'a,S:System<'s,'a>+'s>(&mut self,system:S,objects:&mut Objects,graphics:&mut Graphics){
        let boxed_system=Box::new(system);
        let data=Box::leak(boxed_system);

        let object_storage_id=objects.create_storage();
        let object_storage=objects.get_storage(object_storage_id);

        let object_manager=unsafe{
            ObjectManager::new(
                std::mem::transmute(object_storage as *mut ObjectStorage),
                std::mem::transmute(graphics as *mut Graphics)
            )
        };

        let object_reference=data.set_objects(unsafe{std::mem::transmute(&mut self.shared)},object_manager);
        let object_storage_references=Box::new(object_reference);

        let system=SystemTable{
            data:data as *mut _ as *mut (),
            object_storage:object_storage_id,
            object_storage_references:Box::leak(object_storage_references) as *mut S::Objects as *mut (),
            handle:unsafe{std::mem::transmute(system_handle_wrapper::<S> as usize)},
            destroy:system_destroy_wrapper::<S>
        };

        self.active.push(system);
    }

    pub fn object_handle(&mut self,event:ObjectEvent,objects:&mut Objects){
        for system in &self.active{
            objects.handle(system.object_storage,event.clone())
        }
    }

    pub fn handle(&mut self,event:SystemEvent,window:&Window,objects:&mut Objects,graphics:&mut Graphics){
        let mut c=0;
        unsafe{
            while c<self.active.len(){
                let active_systems:&'static mut Vec<SystemTable>=std::mem::transmute(&mut self.active);
                let pause_systems:&'static mut Vec<SystemTable>=std::mem::transmute(&mut self.paused);

                let system=self.active.get_unchecked(c);
                let manager=SystemManager::new(c,active_systems,pause_systems,objects,graphics);

                let status=system.handle(
                    event,
                    window,
                    &mut self.shared as *mut D as *mut (),
                    std::mem::transmute_copy(&manager)
                );

                match status{
                    SystemStatus::Stop=>{
                        let system=self.active.remove(c);
                        objects.remove_storage(system.object_storage,graphics);

                        system.destroy(&mut self.shared as *mut D as *mut (),graphics);
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

    pub fn shared_data(&mut self)->&mut D{
        &mut self.shared
    }
}

fn system_handle_wrapper<'s,'a,S:System<'s,'a>>(
    system:*mut (),
    objects:*mut (),
    event:SystemEvent,
    window:&Window,
    shared:*mut (),
    manager:SystemManager<'a>
)->SystemStatus{
    unsafe{
        let system=&mut *(system as *mut S);

        let objects=&mut *(objects as *mut S::Objects);

        let shared=&mut *(shared as *mut S::SharedData);

        system.handle(objects,event,window,shared,manager)
    }
}

fn system_destroy_wrapper<'s,'a,S:System<'s,'a>>(data:*mut (),objects:*mut (),shared:*mut (),graphics:*mut Graphics){
    unsafe{
        let system=&mut *(data as *mut S);
        let shared=&mut *(shared as *mut S::SharedData);

        if !graphics.is_null(){
            system.destroy(shared,&mut *graphics);
        }

        // Вызываем ленивый деконструктор для данных системы
        drop(Box::from_raw(data as *mut S));

        // Вызываем ленивый деконструктор для ссылок на объекты
        drop(Box::from_raw(objects as *mut S::Objects));
    }
}