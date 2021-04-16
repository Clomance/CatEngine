use cat_engine_basement::{
    graphics::gl::{
        Viewport,
    },
};

#[cfg(target_os="windows")]
use cat_engine_basement::windows::{
    Window,
    WindowClass,
    EventLoop,
    RenderContext,
    GraphicsLibrary
};

#[cfg(target_os="windows")]
pub use cat_engine_basement::{
    windows::{
        CursorIcon,
        Background,
        Fullscreen,
        WindowReference,
        WindowAttributes,
        WindowClassAttributes,
        Event,
        WindowEvent,
        LoopControl,
        EventLoopAttributes,
        RenderContextAttributes,
        UpdateInterval,
    },
};

#[cfg(feature="file_drop")]
use std::path::PathBuf;


/// Ширина окна. The window width.
pub static mut window_width:f32=0f32;
/// Высота окна. The window height.
pub static mut window_height:f32=0f32;
/// Центр окна. The window center. [x, y]
pub static mut window_center:[f32;2]=[0f32;2];

/// Счётчик кадров в секунду. A frame per seconds counter. `feature = "fps_counter"`
/// 
/// Обновляется раз в секунду. Updates once a second.
#[cfg(feature="fps_counter")]
pub static mut fps:u32=0;

/// Счётчик обновлений в секунду. An update per seconds counter. `feature = "ups_counter"`
/// 
/// Обновляется раз в секунду. Updates once a second.
#[cfg(feature="ups_counter")]
pub static mut ups:u32=0;

#[cfg(target_os="windows")]
pub struct App{
    window_class:WindowClass,
    window:Window,
    render_context:RenderContext,
    graphics_library:GraphicsLibrary,
    event_loop:EventLoop,
}

#[cfg(target_os="windows")]
impl App{
    pub fn raw(
        window_class:WindowClass,
        window:Window,
        render_context:RenderContext,
        graphics_library:GraphicsLibrary,
        event_loop:EventLoop
    )->App{
        let [width,height]=window.client_size();
        unsafe{
            window_width=width as f32;
            window_height=height as f32;
            window_center=[(width/2u32) as f32,(height/2u32) as f32];
        }
        Self{
            window_class,
            window,
            render_context,
            graphics_library,
            event_loop,
        }
    }

    pub fn new(attributes:AppAttributes)->App{
        let mut event_loop=EventLoop::new(attributes.event_loop);

        let class=WindowClass::new(attributes.class).unwrap();

        let window=Window::new(&class,attributes.window,event_loop.get_handler()).unwrap();

        event_loop.set_main_window(Some(&window));

        let [width,height]=window.client_size();
        unsafe{
            window_width=width as f32;
            window_height=height as f32;
            window_center=[(width/2u32) as f32,(height/2u32) as f32];
        }

        let render_context=RenderContext::opengl(window.context(),attributes.render_context).unwrap();

        let graphics_library=GraphicsLibrary::opengl();

        Self{
            window_class:class,
            window:window,
            render_context,
            graphics_library,
            event_loop,
        }
    }

    pub fn get_proc_address(&self,name:&str)->*const std::ffi::c_void{
        self.graphics_library.get_proc_address(name) as *const std::ffi::c_void
    }
}

#[cfg(target_os="windows")]
impl App{
    pub fn position(&self)->[i32;2]{
        self.window.position()
    }

    pub fn size(&self)->[u32;2]{
        self.window.size()
    }

    pub fn client_size(&self)->[u32;2]{
        self.window.client_size()
    }
}

#[cfg(target_os="windows")]
impl App{
    pub fn run<F:FnMut(Event,&AppControl,WindowReference,&mut LoopControl)>(&mut self,mut event_handler:F){
        let event_loop=unsafe{&mut *(&mut self.event_loop as *mut EventLoop)};

        let window=WindowReference::new(&self.window);

        let app_control=AppControl::new(self);

        event_loop.run(|event,control|{
            match &event{
                Event::WindowEvent(window_event)=>match window_event{
                    WindowEvent::Redraw=>{
                        window.request_redraw();
                        event_handler(event,&app_control,window,control);
                        window.swap_buffers();
                        return
                    }

                    WindowEvent::Resize([width,height])=>unsafe{
                        Viewport(0i32,0i32,*width as i32,*height as i32);
                        window_width=*width as f32;
                        window_height=*height as f32;
                        window_center=[(width/2u16) as f32,(height/2u16) as f32];
                    }

                    _=>{}
                }

                _=>{},
            }

            event_handler(event,&app_control,window,control)
        });
    }
}

#[cfg(target_os="windows")]
pub struct AppAttributes{
    pub class:WindowClassAttributes,
    pub window:WindowAttributes,
    pub render_context:RenderContextAttributes,
    pub event_loop:EventLoopAttributes,
}

#[cfg(target_os="windows")]
impl AppAttributes{
    pub fn new()->AppAttributes{
        Self{
            class:WindowClassAttributes::new("NewWindowClass"),
            window:WindowAttributes::new("NewWindow"),
            render_context:RenderContextAttributes::new(),
            event_loop:EventLoopAttributes::new(),
        }
    }
}

#[cfg(target_os="windows")]
pub struct AppControl{
    app:&'static App,
}

#[cfg(target_os="windows")]
impl AppControl{
    pub fn new(app:&App)->AppControl{
        unsafe{
            Self{
                app:std::mem::transmute(app),
            }
        }
    }
}