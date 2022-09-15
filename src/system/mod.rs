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

pub trait System<'s>{
    type Objects;
    type SharedData;
    type CreateParameters;

    /// Creates the system.
    /// 
    /// See `SystemManager::push`.
    fn create(
        create_parameters:&mut Self::CreateParameters,
        window:&Window,
        shared:&mut Self::SharedData,
    )->Self;

    /// Sets up system's layers, objects, etc.
    /// 
    /// Called after the `System::create` function.
    fn set_objects(
        &mut self,
        shared:&mut Self::SharedData,
        object_manager:ObjectManager,
    )->Self::Objects;

    /// Processes the system events.
    fn handle(
        &mut self,
        objects:&mut Self::Objects,
        event:SystemEvent,
        window:&Window,
        shared:&mut Self::SharedData,
        manager:SystemManager
    )->SystemStatus;

    fn destroy(&mut self,shared:&mut Self::SharedData,graphics:&mut Graphics);
}

pub trait StartSystem<'s>:System<'s>{
    fn create_shared_data(create_parameters:&mut Self::CreateParameters)->Self::SharedData;
}

pub struct SystemManager<'m>{
    current_system:usize,
    active_systems:&'m mut Vec<SystemStructure>,
    paused_systems:&'m mut Vec<SystemStructure>,

    objects:&'m mut Objects,
    graphics:&'m mut Graphics,
}

impl<'m> SystemManager<'m>{
    pub (crate) fn new(
        current_system:usize,
        active_systems:&'m mut Vec<SystemStructure>,
        paused_systems:&'m mut Vec<SystemStructure>,
        objects:&'m mut Objects,
        graphics:&'m mut Graphics
    )->SystemManager<'m>{
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

    pub fn object_manager(&mut self)->ObjectManager<'m>{
        let system=&self.active_systems[self.current_system];
        let object_storage=self.objects.get_storage(system.object_storage);
        unsafe{
            ObjectManager::new(
                std::mem::transmute(object_storage),
                &mut *(self.graphics as *mut Graphics)
            )
        }
    }

    pub fn push<'s,S:System<'s>>(
        &mut self,
        window:&Window,
        shared:&mut S::SharedData,
        create_parameters:&mut S::CreateParameters
    ){
        let system=S::create(
            create_parameters,
            window,
            shared,
        );
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
        object_storage.set_references(Box::leak(object_storage_references) as *mut S::Objects as *mut ());

        let data=Box::leak(boxed_system);

        let system=SystemStructure{
            system:data as *mut _ as *mut (),
            object_storage:object_storage_id,
            handle:unsafe{std::mem::transmute(system_handle_wrapper::<S> as usize)},
            drop:system_drop_wrapper::<S>
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

pub (crate) struct SystemStructure{
    system:*mut (),
    object_storage:usize,
    handle:fn(*mut (),*mut (),SystemEvent,&Window,*mut (),SystemManager)->SystemStatus,
    drop:fn(*mut (),*mut (),*mut (),&mut Graphics),
}

impl SystemStructure{
    #[inline(always)]
    fn handle(
        &self,
        object_references:*mut (),
        event:SystemEvent,
        window:&Window,
        shared:*mut (),
        system_manager:SystemManager
    )->SystemStatus{
        (self.handle)(
            self.system,
            object_references,
            event,
            window,
            shared,
            system_manager
        )
    }
}

pub (crate) struct Systems<D>{
    /// Общие данные.
    shared:D,
    /// Активные системы.
    active:Vec<SystemStructure>,
    /// Неактивные системы.
    paused:Vec<SystemStructure>
}

impl<D> Systems<D>{
    pub fn new(shared:D)->Systems<D>{
        Self{
            shared,
            active:Vec::new(),
            paused:Vec::new(),
        }
    }

    pub fn push<'s,S:System<'s>>(&mut self,system:S,object_storage:usize)->&mut S{
        let boxed_system=Box::new(system);

        let data=Box::leak(boxed_system);

        let system=SystemStructure{
            system:data as *mut _ as *mut (),
            object_storage,
            handle:unsafe{std::mem::transmute(system_handle_wrapper::<S> as usize)},
            drop:system_drop_wrapper::<S>
        };

        self.active.push(system);

        data
    }

    pub fn object_handle(&mut self,event:ObjectEvent,objects:&mut Objects,graphics:&mut Graphics){
        for system in &self.active{
            objects.handle(system.object_storage,event.clone(),graphics)
        }
    }

    pub fn handle(&mut self,event:SystemEvent,window:&Window,objects:&mut Objects,graphics:&mut Graphics){
        let mut c=0;
        unsafe{
            while c<self.active.len(){
                let active_systems:&'static mut Vec<SystemStructure>=std::mem::transmute(&mut self.active);
                let pause_systems:&'static mut Vec<SystemStructure>=std::mem::transmute(&mut self.paused);

                let system=self.active.get_unchecked(c);
                let object_references=objects.get_storage(system.object_storage).get_references();
                let manager=SystemManager::new(c,active_systems,pause_systems,objects,graphics);

                let status=system.handle(
                    object_references,
                    event,
                    window,
                    &mut self.shared as *mut D as *mut (),
                    std::mem::transmute_copy(&manager)
                );

                match status{
                    SystemStatus::Stop=>{
                        let system=self.active.remove(c);
                        objects.remove_storage(system.object_storage,graphics);
                        (system.drop)(system.system,object_references,&mut self.shared as *mut D as *mut (),graphics);
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

fn system_handle_wrapper<'s,S:System<'s>>(
    system:*mut (),
    objects:*mut (),
    event:SystemEvent,
    window:&Window,
    shared:*mut (),
    manager:SystemManager
)->SystemStatus{
    unsafe{
        let system=&mut *(system as *mut S);

        let objects=&mut *(objects as *mut S::Objects);

        let shared=&mut *(shared as *mut S::SharedData);

        system.handle(objects,event,window,shared,manager)
    }
}

fn system_drop_wrapper<'s,S:System<'s>>(data:*mut (),objects:*mut (),shared:*mut (),graphics:&mut Graphics){
    unsafe{
        let system=&mut *(data as *mut S);
        let shared=&mut *(shared as *mut S::SharedData);
        system.destroy(shared,graphics);

        // Вызываем ленивый деконструктор для данных системы
        Box::from_raw(data as *mut S);

        // Вызываем ленивый деконструктор для ссылок на объекты
        Box::from_raw(objects as *mut S::Objects);
    }
}