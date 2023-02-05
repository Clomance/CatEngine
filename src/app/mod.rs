mod app_system;
pub use app_system::AppSystem;
use cat_audio::{
    AudioSystem,
    AudioClientError,
    OutputStream,
    AudioSystemStatus,
    AudioCore
};

use crate::{
    window::WinError,
    system::{
        Systems,
        SystemEvent,
        StartSystem
    },

    graphics::{
        GraphicsCore,
        GraphicsAttributes,
        ObjectEvent,
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
            WindowResizeType
        },
        Error,
        WindowEvent,
        Event,
        ProcessEvent, backend::core::message::Message,
    }
};

use std::marker::PhantomData;



pub struct AppAudioSystem;

impl AudioSystem for AppAudioSystem{
    fn error(&mut self,error:AudioClientError,stream:&mut OutputStream)->AudioSystemStatus{
        let AudioClientError::None=error else{
            *stream=OutputStream::new().unwrap();
            return AudioSystemStatus::Processed
        };

        AudioSystemStatus::Processed
    }
}



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
        Self{
            class:WindowClassAttributes::new(window_title),
            window:WindowAttributes::new(window_title),
            context:OpenGLRenderContextAttributes::new(),
            graphics:GraphicsAttributes::new(),
            event_loop:EventLoopAttributes::new()
        }
    }
}



pub (crate) struct AppData<S>{
    pub audio:AudioCore<AppAudioSystem>,
    pub graphics:GraphicsCore,
    pub systems:Systems<S>
}

impl<S> AppData<S>{
    pub fn new(
        audio:AudioCore<AppAudioSystem>,
        graphics:GraphicsCore,
        systems:Systems<S>
    )->AppData<S>{
        
        Self{
            audio,
            graphics,
            systems
        }
    }
}



pub struct App{
    _window_class:WindowClass,
    window:Window,
    event_loop:EventLoop,
}

impl App{
    pub fn new<'a,'s:'a,S:StartSystem<'a,'s>+'s+'a>(attributes:AppAttributes,create_parameters:&'s mut S::CreateParameters)->Result<App,WinError>{
        let _window_class=match WindowClass::new(attributes.class){
            Ok(class)=>class,
            Err(e)=>return Err(e)
        };

        let mut app_create_parameters=AppCreateParameters{
            context:attributes.context,
            graphics:attributes.graphics,
            create_parameters
        };

        let window=match Window::new::<WinProc<'a,'s,S>>(&_window_class,attributes.window,&mut app_create_parameters){
            Ok(window)=>window,
            Err(e)=>return Err(e)
        };

        let event_loop=EventLoop::new(attributes.event_loop);

        Ok(
            Self{
                _window_class,
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



pub (crate) struct WinProc<'s,'a,S:StartSystem<'s,'a>+'s>{
    marker1:PhantomData<&'s S>,
    marker2:PhantomData<&'a S>
}

impl<'s,'a,S:StartSystem<'s,'a>+'s> WindowProcedure for WinProc<'s,'a,S>{
    type CreateParameters=AppCreateParameters<&'s mut S::CreateParameters>;
    type Data=AppData<S::SharedData>;

    fn create(window:&Window,create_parameters:&mut Self::CreateParameters)->Result<Self::Data,Error>{
        match OpenGLRenderContext::new(window,create_parameters.context.clone()){
            Ok(render_context)=>{
                let opengl_module=OpenGraphicsLibrary::new();

                let [w,h]=window.client_size();

                let [w,h,d]=[
                    w as f32,
                    h as f32,
                    10f32
                ];

                let graphics=GraphicsCore::new(
                    [w,h],
                    [w,h,d],
                    &create_parameters.graphics,
                    render_context,
                    opengl_module,
                );

                let shared_data=S::create_shared_data(create_parameters.create_parameters);

                let audio=AudioCore::default(AppAudioSystem);

                Ok(
                    AppData::new(
                        audio,
                        graphics,
                        Systems::new(shared_data)
                    )
                )
            },
            Err(e)=>{
                Err(e)
            }
        }
    }

    fn data_packed(window:&Window,create_parameters:&mut Self::CreateParameters,data:&mut Self::Data){
        let start_system=S::create(
            &mut create_parameters.create_parameters,
            window,
            data.systems.shared_data(),
        );

        data.systems.push(start_system,window,&mut data.graphics,data.audio.core_manager());
    }

    fn close(window:&Window,_:&mut Self::Data){
        let _=window.destroy();
    }

    fn destroy(window:&Window,data:&mut Self::Data){
        data.systems.handle(
            SystemEvent::WindowDestroy,
            window,
            &mut data.graphics,
            data.audio.core_manager()
        );
        Message::post_quit(0)
    }

    fn paint(window:&Window,data:&mut Self::Data){
        unsafe{
            GLCore::flush()
        }
        data.graphics.render_context.swap_buffers().unwrap();

        data.graphics.objects.event(ObjectEvent::Prerender);

        data.graphics.camera.uniform_buffer.write(&data.graphics.camera.matrix).unwrap();
        data.graphics.camera.uniform_buffer.bind_base(0);

        window.draw(||{
            unsafe{
                GLCore::clear(data.graphics.parameters.clear_mask);

                data.graphics.draw();
            }
        });

        window.redraw();
    }

    fn resized(size:[u16;2],_:WindowResizeType,window:&Window,data:&mut Self::Data){
        unsafe{
            data.graphics.camera.set_viewport([size[0] as f32,size[1] as f32]);
            // app.graphics.camera.uniform_buffer.write(&app.graphics.camera.matrix).unwrap();
            GLCore::set_viewport(0,0,size[0] as i32,size[1] as i32);
        }

        data.systems.handle(
            SystemEvent::Resize(size),
            window,
            &mut data.graphics,
            data.audio.core_manager()
        )
    }

    fn moved(position:[i16;2],window:&Window,data:&mut Self::Data){
        data.systems.handle(
            SystemEvent::Move(position),
            window,
            &mut data.graphics,
            data.audio.core_manager()
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
                    &mut data.graphics,
                    data.audio.core_manager()
                )
            }
            WindowEvent::KeyRelease(key)=>{
                data.systems.handle(
                    SystemEvent::Keyboard{
                        state:false,
                        key,
                    },
                    window,
                    &mut data.graphics,
                    data.audio.core_manager()
                )
            }
            WindowEvent::CharacterInput(character)=>{
                data.systems.handle(
                    SystemEvent::CharacterInput(character),
                    window,
                    &mut data.graphics,
                    data.audio.core_manager()
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
                    &mut data.graphics,
                    data.audio.core_manager()
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
                    &mut data.graphics,
                    data.audio.core_manager()
                )
            }
            WindowEvent::MouseMove(cursor_position)=>{
                data.systems.handle(
                    SystemEvent::MouseMove(cursor_position),
                    window,
                    &mut data.graphics,
                    data.audio.core_manager()
                )
            }
            _=>{}
        }
    }

    fn user_event(_w_param:usize,_l_param:isize,window:&Window,data:&mut Self::Data){
        data.audio.render();
        data.systems.handle(
            SystemEvent::AudioRender,
            window,
            &mut data.graphics,
            data.audio.core_manager()
        );

        data.graphics.objects.event(ObjectEvent::Update);
        data.systems.handle(
            SystemEvent::Update,
            &window,
            &mut data.graphics,
            data.audio.core_manager()
        )
    }

    fn catch_panic(_window:&Window,_data:Option<&mut Self::Data>,_error:Box<dyn std::any::Any+Send>){
        Message::post_quit(0)
    }
}