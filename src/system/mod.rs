use std::{
    mem::{
        transmute,
        MaybeUninit,
        transmute_copy
    }
};

use cat_engine_basement::winapi::{
    VirtualKeyCode,
    MouseButton,
    window::Window,
    backend::core::message::Message
};

use crate::{
    graphics::{
        GraphicsCore,
        GraphicsCoreManager,
        GraphicsManager,
        SystemObjectStorage,
        SystemObjectManager,
    },
    object::{
        ObjectManager,
        AudioObjectManager, GraphicsObjectManager
    },
};

use cat_audio::{
    ResourceManager as AudioResourceManager,
    AudioCoreManager,
    OutputStream, AudioManager, ObjectStorage,
};



#[derive(Debug,Clone,Copy)]
pub enum SystemStatus{
    /// Runs the next system.
    Next,

    /// Stops the current system and destroys it.
    Stop,

    /// Closes the app immediatelly.
    Exit
}



#[derive(Debug,Clone,Copy)]
pub enum SystemEvent{
    Update,
    AudioRender,
    Keyboard{
        state:bool,
        key:VirtualKeyCode
    },
    CharacterInput(char),
    MouseMove([u16;2]),
    MouseButton{
        state:bool,
        position:[u16;2],
        button:MouseButton
    },
    Resize([u16;2]),
    Move([i16;2]),
    WindowDestroy
}



pub struct ComponentManager<'m>{
    pub window:&'m Window,

    pub graphics:GraphicsManager<'m>,

    pub audio_stream:&'m mut OutputStream,
}

impl<'m> ComponentManager<'m>{
    pub fn new(
        window:&'m Window,
        graphics:GraphicsManager<'m>,
        audio_stream:&'m mut OutputStream
    )->ComponentManager<'m>{
        Self{
            window,
            graphics,
            audio_stream
        }
    }
}



pub struct ResourceManager<'m>{
    pub audio:AudioResourceManager<'m>
}

impl<'m> ResourceManager<'m>{
    pub (crate) fn new(audio:AudioResourceManager<'m>)->ResourceManager<'m>{
        Self{
            audio
        }
    }
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
        objects:ObjectManager<'a>,
        resources:ResourceManager<'a>,
        components:ComponentManager<'a>
    )->Self::Objects;

    /// Processes the system events.
    fn handle(
        &'s mut self,
        event:SystemEvent,
        objects:&mut Self::Objects,
        shared:&mut Self::SharedData,
        manager:SystemManager<'a>
    )->SystemStatus;

    fn destroy(
        &'s mut self,
        shared:&mut Self::SharedData,
        graphics:GraphicsManager<'a>
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
    graphics_object_storage:usize,
    audio_object_storage:usize,
    graphics:GraphicsManager<'static>,
}

impl<S,O> ExtendedSystemData<S,O>{
    pub fn new(
        system:S,
        object_references:O,
        graphics_object_storage:usize,
        audio_object_storage:usize,
        graphics:GraphicsManager
    )->ExtendedSystemData<S,O>{
        Self{
            system,
            object_references,
            graphics_object_storage,
            audio_object_storage,
            graphics:unsafe{transmute(graphics)}
        }
    }
}



/// Данные и функции системы.
#[derive(Clone)]
pub (crate) struct SystemTable{
    /// Ссылка на данные самой системы.
    system_data:*mut (),
    /// Ссылка на расширенные данные системы.
    extended_system_data:*mut (),
    /// Ссылки на индексы хранилищ объектов.
    graphics_object_storage:*mut usize,
    audio_object_storage:*mut usize,
    /// Ссылка на сохранённые ссылки на объекты.
    object_references:*mut (),
    handle:fn(
        system_data:*mut (),
        object_references:*mut (),
        event:SystemEvent,
        shared:*mut (),
        manager:SystemManager
    )->SystemStatus,
    destroy:fn(extended_system_data:*mut (),shared:*mut ())
}

impl SystemTable{
    pub fn new<'s,'a,S:System<'s,'a>>(system:S,graphics:GraphicsManager)->SystemTable{
        // Делаем затычку для сохранённых ссылок на объекты
        let empty_object_references:S::Objects=unsafe{MaybeUninit::uninit().assume_init()};
        // Упаковываем данные
        let boxed_data=Box::new(ExtendedSystemData::new(system,empty_object_references,0usize,0usize,graphics));

        let extended_system_data_reference=Box::leak(boxed_data);

        Self{
            system_data:&mut extended_system_data_reference.system as *mut _ as *mut (),
            extended_system_data:extended_system_data_reference as *mut _ as *mut (),

            graphics_object_storage:&mut extended_system_data_reference.graphics_object_storage as *mut usize,
            audio_object_storage:&mut extended_system_data_reference.audio_object_storage as *mut usize,

            object_references:&mut extended_system_data_reference.object_references as *mut _ as *mut (),

            // Подгоняем lifetime
            handle:unsafe{transmute(Self::handle_wrapper::<S> as usize)},
            destroy:Self::destroy_wrapper::<S>,
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

    pub fn graphics_object_storage_index(&mut self)->&mut usize{
        unsafe{
            &mut *self.graphics_object_storage
        }
    }

    pub fn audio_object_storage_index(&mut self)->&mut usize{
        unsafe{
            &mut *self.audio_object_storage
        }
    }

    pub fn handle(&mut self,event:SystemEvent,shared:*mut (),manager:SystemManager)->SystemStatus{
        (self.handle)(self.system_data,self.object_references,event,shared,manager)
    }

    pub fn destroy(&mut self,shared:*mut ()){
        (self.destroy)(self.extended_system_data,shared)
    }

    fn handle_wrapper<'s,'a,S:System<'s,'a>>(
        system_data:*mut (),
        object_references:*mut (),
        event:SystemEvent,
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

        system.handle(event,object_references,shared,manager)
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

            let graphics=unsafe{
                transmute_copy(&extended_system_data.graphics)
            };

            extended_system_data.system.destroy(shared,graphics);
        }

        // Вызываем ленивый деконструктор
        drop(unsafe{Box::from_raw(extended_system_data)});
    }
}



pub struct ActiveSystems{
    systems:Vec<SystemTable>
}

impl ActiveSystems{
    pub fn new()->ActiveSystems{
        Self{
            systems:Vec::new()
        }
    }


    pub fn push<'s,'a,S:System<'s,'a>>(
        &mut self,
        system:S,
        shared:&mut S::SharedData,
        window:&Window,
        graphics:GraphicsCoreManager,
        mut audio:AudioCoreManager
    ){
        let graphics_manager=GraphicsManager{
            camera:graphics.camera,
            parameters:graphics.parameters,
            simple:graphics.simple,
            texture:graphics.texture,
            text:graphics.text,
            layers:graphics.layers,
        };

        let mut system_table=SystemTable::new(system,graphics_manager);

        graphics.objects.push(system_table.graphics_object_storage_index());

        let system=system_table.system_data::<S>();

        let graphics_object_manager=GraphicsObjectManager::new(graphics.objects.manager(*system_table.graphics_object_storage_index()));

        audio.objects.push(system_table.audio_object_storage_index());
        let audio_object_manager=AudioObjectManager::new(
            audio.objects.system_manager(*system_table.audio_object_storage_index())
        );

        let object_manager=unsafe{
            transmute(ObjectManager::new(audio_object_manager,graphics_object_manager))
        };

        let resource_manager=unsafe{
            transmute(ResourceManager::new(audio.sources))
        };

        let graphics_manager=GraphicsManager{
            camera:graphics.camera,
            parameters:graphics.parameters,
            simple:graphics.simple,
            texture:graphics.texture,
            text:graphics.text,
            layers:graphics.layers,
        };

        let component_manager=unsafe{
            transmute(ComponentManager::new(
                window,
                graphics_manager,
                audio.stream
            ))
        };

        let object_references=system.set_up(shared,object_manager,resource_manager,component_manager);

        system_table.set_object_references(object_references);

        self.systems.push(system_table)
    }

    pub fn handle(&mut self,create:&mut CreateSystems,event:SystemEvent,shared:*mut (),window:&Window,graphics:GraphicsCoreManager,mut audio:AudioCoreManager){
        let mut c=0;
        unsafe{
            while c<self.systems.len(){
                let mut system=self.systems.get_unchecked_mut(c).clone();

                let graphics_object_manager=GraphicsObjectManager::new(graphics.objects.manager(*system.graphics_object_storage_index()));

                let audio_object_manager=AudioObjectManager::new(
                    audio.objects.system_manager(*system.audio_object_storage_index())
                );

                let object_manager={
                    transmute(ObjectManager::new(audio_object_manager,graphics_object_manager))
                };

                let resource_manager={
                    transmute(ResourceManager::new(transmute_copy(&audio.sources)))
                };

                let graphics_manager=GraphicsManager{
                    camera:graphics.camera,
                    parameters:graphics.parameters,
                    simple:graphics.simple,
                    texture:graphics.texture,
                    text:graphics.text,
                    layers:graphics.layers,
                };

                let component_manager={
                    transmute(ComponentManager::new(
                        window,
                        graphics_manager,
                        audio.stream
                    ))
                };


                let system_map=SystemMap::new(self,create);

                let manager=SystemManager::new(
                    system_map,
                    window,
                    object_manager,
                    resource_manager,
                    component_manager
                );

                let status=system.handle(
                    event,
                    shared,
                    transmute_copy(&manager)
                );

                match status{
                    SystemStatus::Stop=>{
                        let mut system=self.systems.remove(c);
                        graphics.objects.remove(*system.graphics_object_storage);
                        audio.objects.remove(*system.audio_object_storage);

                        system.destroy(shared);
                    }

                    SystemStatus::Exit=>{
                        Message::post_quit(0);
                        break
                    }

                    SystemStatus::Next=>c+=1,
                }
            }
        }
    }
}



pub struct CreateSystem{
    system:SystemTable,
    graphics_objects:SystemObjectStorage,
    audio_objects:ObjectStorage
}



pub struct CreateSystems{
    systems:Vec<CreateSystem>
}

impl CreateSystems{
    pub fn new()->CreateSystems{
        Self{
            systems:Vec::new()
        }
    }

    pub fn push<'s,'a,S:System<'s,'a>>(
        &mut self,
        system:S,
        shared:&mut S::SharedData,
        window:&Window,
        graphics:GraphicsManager,
        audio:AudioManager
    ){
        let graphics_manager=unsafe{
            transmute_copy(&graphics)
        };

        let mut system_table=SystemTable::new(system,graphics_manager);

        let mut graphics_object_storage=SystemObjectStorage::new();
        let mut audio_object_storage=ObjectStorage::new();

        let system=system_table.system_data::<S>();

        let graphics_object_manager=GraphicsObjectManager::new(SystemObjectManager::new(&mut graphics_object_storage));

        let audio_object_manager=AudioObjectManager::new(cat_audio::SystemObjectManager::new(&mut audio_object_storage));

        let object_manager=unsafe{
            transmute(ObjectManager::new(audio_object_manager,graphics_object_manager))
        };

        let resource_manager=unsafe{
            transmute(ResourceManager::new(audio.sources))
        };

        let graphics_manager=GraphicsManager{
            camera:graphics.camera,
            parameters:graphics.parameters,
            simple:graphics.simple,
            texture:graphics.texture,
            text:graphics.text,
            layers:graphics.layers,
        };

        let component_manager=unsafe{
            transmute(ComponentManager::new(
                window,
                graphics_manager,
                audio.stream
            ))
        };

        let object_references=system.set_up(shared,object_manager,resource_manager,component_manager);

        system_table.set_object_references(object_references);

        let create_system=CreateSystem{
            system:system_table,
            graphics_objects:graphics_object_storage,
            audio_objects:audio_object_storage
        };

        self.systems.push(create_system)
    }
}



pub struct Systems<D>{
    /// Общие данные.
    shared:D,

    /// Активные системы.
    active:ActiveSystems,

    /// Системы, ожидающие инициализации.
    create:CreateSystems
}

impl<D> Systems<D>{
    pub fn new(shared:D)->Systems<D>{
        Self{
            shared,
            active:ActiveSystems::new(),
            create:CreateSystems::new()
        }
    }

    pub fn push<'s,'a,S:System<'s,'a>>(&mut self,system:S,window:&Window,graphics:&mut GraphicsCore,audio:AudioCoreManager){
        let shared=unsafe{transmute(&mut self.shared)};
        self.active.push(system,shared,window,graphics.manager(),audio)
    }

    pub fn shared_data(&mut self)->&mut D{
        &mut self.shared
    }

    pub fn handle(&mut self,event:SystemEvent,window:&Window,graphics:&mut GraphicsCore,audio:AudioCoreManager){
        let mut audio_manager:AudioCoreManager=unsafe{
            transmute_copy(&audio)
        };

        let shared=unsafe{transmute(&mut self.shared)};
        self.active.handle(&mut self.create,event,shared,window,graphics.manager(),audio);

        while !self.create.systems.is_empty(){
            let mut system=self.create.systems.remove(0);

            graphics.manager().objects.push_new(system.system.graphics_object_storage_index(),system.graphics_objects);
            audio_manager.objects.push_new(system.system.audio_object_storage,system.audio_objects);

            self.active.systems.push(system.system);
        }
    }
}



pub struct SystemMap<'a>{
    active:&'a mut ActiveSystems,
    create:&'a mut CreateSystems
}

impl<'a> SystemMap<'a>{
    pub (crate) fn new(active:&'a mut ActiveSystems,create:&'a mut CreateSystems)->SystemMap<'a>{
        Self{
            active,
            create
        }
    }

    pub fn push<'s,S:System<'s,'a>>(&mut self,system:S,shared:&mut S::SharedData,window:&Window,graphics:GraphicsCoreManager,audio:AudioCoreManager){
        self.active.push(system,shared,window,graphics,audio)
    }

    pub fn create<'s,S:System<'s,'a>>(&mut self,system:S,shared:&mut S::SharedData,window:&Window,graphics:GraphicsManager,audio:AudioManager){
        self.create.push(system,shared,window,graphics,audio)
    }
}

pub struct SystemManager<'a>{
    systems:SystemMap<'a>,

    pub window:&'a Window,

    pub objects:ObjectManager<'a>,

    pub resources:ResourceManager<'a>,

    pub components:ComponentManager<'a>
}

impl<'a> SystemManager<'a>{
    pub (crate) fn new(
        systems:SystemMap<'a>,
        window:&'a Window,
        objects:ObjectManager<'a>,
        resources:ResourceManager<'a>,
        components:ComponentManager<'a>
    )->SystemManager<'a>{
        Self{
            systems,
            window,
            objects,
            resources,
            components
        }
    }

    pub fn push<'s,S:System<'s,'a>>(
        &mut self,
        shared:&mut S::SharedData,
        system:S,
    ){
        let graphics=GraphicsManager{
            camera:self.components.graphics.camera,
            parameters:self.components.graphics.parameters,
            simple:self.components.graphics.simple,
            texture:self.components.graphics.texture,
            text:self.components.graphics.text,
            layers:self.components.graphics.layers,
        };

        let audio=unsafe{
            transmute_copy(&self.resources.audio)
        };

        let audio=AudioManager{
            stream:self.components.audio_stream,
            sources:audio
        };

        self.systems.create(system,shared,self.window,graphics,audio)
    }
}