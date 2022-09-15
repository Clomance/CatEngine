use crate::{
    window::WinError,
    system::{
        System,
        Systems,
        SystemEvent,
        StartSystem,
    },

    object::{
        Objects,
        ObjectEvent,
        ObjectManager,
        ObjectStorage,
    },

    graphics::{
        Graphics,
        GraphicsAttributes,
    },
};

use cat_engine_basement::{
    opengl::core::Core as GLCore,
    winapi::{
        OpenGraphicsLibrary,
        OpenGLRenderContext,
        OpenGLRenderContextAttributes,
        EventLoop,
        EventLoopAttributes,
        window::{
            WindowProcedure,
            WindowClassAttributes,
            WindowClass,
            WindowAttributes,
            Window,
            WindowResizeType,
            quit,
        },
        Error,
        WindowEvent,
        Event,
        ProcessEvent,
    }
};

use std::marker::PhantomData;

pub (crate) struct AppCreateParameters<P>{
    pub context:OpenGLRenderContextAttributes,
    pub graphics:GraphicsAttributes,
    pub create_parameters:P
}

pub struct AppAttributes{
    pub class:WindowClassAttributes,
    pub window:WindowAttributes,
    pub context:OpenGLRenderContextAttributes,
    pub graphics:GraphicsAttributes,
    pub event_loop:EventLoopAttributes
}

impl AppAttributes{
    pub fn new(window_title:&str)->AppAttributes{
        Self {
            class:WindowClassAttributes::new(window_title),
            window:WindowAttributes::new(window_title),
            context:OpenGLRenderContextAttributes::new(),
            graphics:GraphicsAttributes::new(),
            event_loop:EventLoopAttributes::new()
        }
    }
}

pub struct App{
    window_class:WindowClass,
    window:Window,
    event_loop:EventLoop,
}

impl App{
    pub fn new<'s,S:StartSystem<'s>+'s>(attributes:AppAttributes,create_parameters:&'s mut S::CreateParameters)->Result<App,WinError>{
        let window_class=match WindowClass::new(attributes.class){
            Ok(class)=>class,
            Err(e)=>return Err(e)
        };

        let mut app_create_parameters=AppCreateParameters{
            context:attributes.context,
            graphics:attributes.graphics,
            create_parameters
        };

        let window=match Window::new::<WinProc<'s,S>>(&window_class,attributes.window,&mut app_create_parameters){
            Ok(window)=>window,
            Err(e)=>return Err(e)
        };

        let event_loop=EventLoop::new(attributes.event_loop);

        Ok(
            Self{
                window_class,
                window,
                event_loop,
            }
        )
    }

    pub fn run(&mut self){
        self.event_loop.run(|event,_manager|{
            match event{
                Event::Window(_event)=>{
                    
                }
                Event::Process(event)=>{
                    match event{
                        ProcessEvent::Update(_)=>{
                            self.window.send_app_message(0,0);
                        }
                        _=>{},
                    }
                }
            }
        })
    }
}

pub (crate) struct WinProc<'s,S:StartSystem<'s>>{
    marker:PhantomData<&'s S>
}

impl<'s,S:StartSystem<'s>> WindowProcedure for WinProc<'s,S>{
    type CreateParameters=AppCreateParameters<&'s mut S::CreateParameters>;
    type Data=AppSystem<S::SharedData>;

    fn create(window:&Window,create_parameters:&mut Self::CreateParameters)->Result<Self::Data,Error>{
        match OpenGLRenderContext::new(window,create_parameters.context.clone()){
            Ok(render_context)=>unsafe{
                let opengl_module=OpenGraphicsLibrary::new();

                let [w,h]=window.client_size();

                let [w,h,d]=[
                    w as f32,
                    h as f32,
                    1f32
                ];

                let graphics=Graphics::new(
                    [w,h],
                    [w,h,d],
                    &create_parameters.graphics,
                    render_context,
                    opengl_module,
                );

                let shared_data=S::create_shared_data(create_parameters.create_parameters);

                let mut app_system=AppSystem::new(graphics,Systems::new(shared_data),Objects::new());
                let start_system=S::create(
                    &mut create_parameters.create_parameters,
                    window,
                    app_system.systems.shared_data(),
                );

                let shared_data=std::mem::transmute(app_system.systems.shared_data());

                let object_storage_id=app_system.objects.create_storage();
                let object_storage=app_system.objects.get_storage(object_storage_id);
                let start_system=app_system.systems.push(start_system,object_storage_id);

                let object_manager=ObjectManager::new(std::mem::transmute(object_storage as *mut ObjectStorage),std::mem::transmute(&mut app_system.graphics));
                let references=start_system.set_objects(shared_data,object_manager);
                let object_storage_references=Box::new(references);
                object_storage.set_references(Box::leak(object_storage_references) as *mut S::Objects as *mut ());

                Ok(app_system)
            },
            Err(e)=>{
                Err(e)
            }
        }
    }

    fn close(window:&Window,_:&mut Self::Data){
        let _=window.destroy();
    }

    fn destroy(window:&Window,data:&mut Self::Data){
        data.systems.handle(
            SystemEvent::Destroy,
            window,
            &mut data.objects,
            &mut data.graphics
        );
        quit(0);
    }

    fn paint(window:&Window,app:&mut Self::Data){
        unsafe{
            GLCore::flush()
        }
        app.graphics.render_context.swap_buffers().unwrap();

        app.systems.object_handle(ObjectEvent::Prerender,&mut app.objects,&mut app.graphics);

        app.graphics.camera.uniform_buffer.write(&app.graphics.camera.matrix).unwrap();
        app.graphics.camera.uniform_buffer.bind_base(0);

        window.draw(||{
            unsafe{
                GLCore::clear(app.graphics.parameters.clear_mask);

                app.graphics.draw();
            }
        });

        window.redraw();
    }

    fn resized(size:[u16;2],_:WindowResizeType,window:&Window,app:&mut Self::Data){
        unsafe{
            app.graphics.camera.set_viewport([size[0] as f32,size[1] as f32]);
            // app.graphics.camera.uniform_buffer.write(&app.graphics.camera.matrix).unwrap();
            GLCore::set_viewport(0,0,size[0] as i32,size[1] as i32);
        }

        app.systems.handle(
            SystemEvent::Resize(size),
            window,
            &mut app.objects,
            &mut app.graphics
        )
    }

    fn moved(position:[i16;2],window:&Window,data:&mut Self::Data){
        data.systems.handle(
            SystemEvent::Move(position),
            window,
            &mut data.objects,
            &mut data.graphics
        )
    }

    fn handle(event:WindowEvent,window:&Window,data:&mut Self::Data){
        match event{
            WindowEvent::KeyPress(key)=>{
                data.systems.handle(
                    SystemEvent::Keyboard{
                        state:true,
                        key,
                    },
                    window,
                    &mut data.objects,
                    &mut data.graphics
                )
            }
            WindowEvent::KeyRelease(key)=>{
                data.systems.handle(
                    SystemEvent::Keyboard{
                        state:false,
                        key,
                    },
                    window,
                    &mut data.objects,
                    &mut data.graphics
                )
            }
            WindowEvent::CharacterInput(character)=>{
                data.systems.handle(
                    SystemEvent::CharacterInput(character),
                    window,
                    &mut data.objects,
                    &mut data.graphics
                )
            }
            WindowEvent::MousePress{cursor_position,button}=>{
                data.systems.handle(
                    SystemEvent::MouseButton{
                        state:true,
                        position:cursor_position,
                        button,
                    },
                    window,
                    &mut data.objects,
                    &mut data.graphics
                )
            }
            WindowEvent::MouseRelease{cursor_position,button}=>{
                data.systems.handle(
                    SystemEvent::MouseButton{
                        state:false,
                        position:cursor_position,
                        button,
                    },
                    window,
                    &mut data.objects,
                    &mut data.graphics
                )
            }
            WindowEvent::MouseMove(cursor_position)=>{
                data.systems.handle(
                    SystemEvent::MouseMove(cursor_position),
                    window,
                    &mut data.objects,
                    &mut data.graphics
                )
            }
            _=>{}
        }
    }

    fn user_event(_w_param:usize,_l_param:isize,window:&Window,data:&mut Self::Data){
        data.systems.object_handle(ObjectEvent::Update,&mut data.objects,&mut data.graphics);
        data.systems.handle(
            SystemEvent::Update,
            &window,
            &mut data.objects,
            &mut data.graphics
        )
    }

    fn catch_panic(_window:&Window,_data:Option<&mut Self::Data>,_error:Box<dyn std::any::Any+Send>){
        quit(0)
    }
}

pub (crate) struct AppSystem<S>{
    pub systems:Systems<S>,
    pub objects:Objects,
    pub graphics:Graphics,
}

impl<S> AppSystem<S>{
    pub fn new(graphics:Graphics,systems:Systems<S>,objects:Objects)->AppSystem<S>{
        Self{
            graphics,
            systems,
            objects,
        }
    }
}