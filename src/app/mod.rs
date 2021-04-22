use crate::graphics::{
    Graphics,
    Graphics2DAttributes,
};

#[cfg(target_os="windows")]
use cat_engine_basement::windows::{
    Window,
    WindowClass,
    EventLoop,
    OpenGLRenderContext,
    OpenGraphicsLibrary,
    WindowSubclassArguments,
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
        OpenGLRenderContextAttributes,
        UpdateInterval,
    },
};

#[cfg(feature="file_drop")]
use std::path::PathBuf;

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

struct WindowStorage{
    free_ids:Vec<usize>,
    window_subclass_args:Vec<Option<WindowSubclassArguments>>,
    windows:Vec<Option<Window>>,
    render_contexts:Vec<Option<OpenGLRenderContext>>,
    graphics:Vec<Option<Graphics>>,
}

impl WindowStorage{
    pub fn empty(capacity:usize)->WindowStorage{
        let mut free_ids=Vec::with_capacity(capacity);
        let mut window_subclass_args=Vec::with_capacity(capacity);
        let mut windows=Vec::with_capacity(capacity);
        let mut render_contexts=Vec::with_capacity(capacity);
        let mut graphics=Vec::with_capacity(capacity);

        for id in (0..capacity).rev(){
            free_ids.push(id);
            window_subclass_args.push(None);
            windows.push(None);
            render_contexts.push(None);
            graphics.push(None);
        }

        Self{
            free_ids,
            window_subclass_args,
            windows,
            render_contexts,
            graphics,
        }
    }

    pub fn add_window(
        &mut self,
        event_loop:&EventLoop,
        window_class:&WindowClass,
        window_attributes:WindowAttributes,
        render_context_attributes:OpenGLRenderContextAttributes,
        graphics_library:&mut Option<OpenGraphicsLibrary>,
        graphics_attributes:Graphics2DAttributes,
    )->bool{
        if let Some(id)=self.free_ids.pop(){
            let window_subclass_args=WindowSubclassArguments::new(event_loop.get_handler(),id as u64);
            self.window_subclass_args[id]=Some(window_subclass_args);
            let reference=self.window_subclass_args[id].as_ref().unwrap();

            let window=Window::new(window_class,window_attributes,reference).unwrap();

            let render_context=OpenGLRenderContext::new(
                window.context(),
                render_context_attributes
            ).unwrap();

            if graphics_library.is_none(){
                *graphics_library=Some(OpenGraphicsLibrary::new())
            }

            let graphics=Graphics::new(graphics_attributes);

            self.windows[id]=Some(window);
            self.render_contexts[id]=Some(render_context);

            self.graphics[id]=Some(graphics);

            true
        }
        else{
            false
        }
    }

    pub fn get_any_window(&self)->Option<&Window>{
        for window in &self.windows{
            if let Some(window)=window{
                return Some(window)
            }
        }

        None
    }

    pub fn get_window(&self,id:usize)->Option<&Window>{
        if let Some(window)=self.windows.get(id){
            window.as_ref()
        }
        else{
            None
        }
    }

    pub fn get_window_unchecked(&self,id:usize)->&Window{
        unsafe{
            let maybe_window=self.windows.get_unchecked(id).as_ref();
            std::mem::transmute(maybe_window)
        }
    }

    pub fn get_render_context(&self,id:usize)->Option<&OpenGLRenderContext>{
        if let Some(render_context)=self.render_contexts.get(id){
            render_context.as_ref()
        }
        else{
            None
        }
    }

    pub fn get_render_context_unchecked(&self,id:usize)->&OpenGLRenderContext{
        unsafe{
            let maybe_render_context=self.render_contexts.get_unchecked(id).as_ref();
            std::mem::transmute(maybe_render_context)
        }
    }

    pub fn get_graphics(&self,id:usize)->Option<&Graphics>{
        if let Some(graphics)=self.graphics.get(id){
            graphics.as_ref()
        }
        else{
            None
        }
    }

    pub fn get_graphics_unchecked(&self,id:usize)->&Graphics{
        unsafe{
            let maybe_graphics=self.graphics.get_unchecked(id).as_ref();
            std::mem::transmute(maybe_graphics)
        }
    }

    pub fn get_graphics_mut(&mut self,id:usize)->Option<&mut Graphics>{
        if let Some(graphics)=self.graphics.get_mut(id){
            graphics.as_mut()
        }
        else{
            None
        }
    }

    pub fn get_graphics_unchecked_mut(&mut self,id:usize)->&mut Graphics{
        unsafe{
            let maybe_graphics=self.graphics.get_unchecked_mut(id).as_mut();
            std::mem::transmute(maybe_graphics)
        }
    }
}

#[cfg(target_os="windows")]
pub struct App{
    graphics_library:Option<OpenGraphicsLibrary>,
    event_loop:EventLoop,
    window_class:WindowClass,
    window_storage:WindowStorage,
}

#[cfg(target_os="windows")]
impl App{
    pub fn new(attributes:AppAttributes)->App{
        let mut graphics_library=None;

        let mut event_loop=EventLoop::new(attributes.event_loop);

        let class=WindowClass::new(attributes.class).unwrap();

        let mut window_storage=WindowStorage::empty(attributes.windows_limit as usize);

        window_storage.add_window(
            &event_loop,
            &class,
            attributes.window,
            attributes.render_context,
            &mut graphics_library,
            attributes.graphics,
        );

        event_loop.set_main_window(window_storage.get_any_window());

        Self{
            window_class:class,
            window_storage,
            graphics_library,
            event_loop,
        }
    }

    // pub fn get_proc_address(&self,name:&str)->*const std::ffi::c_void{
    //     self.graphics_library.get_proc_address(name) as *const std::ffi::c_void
    // }
}

#[cfg(target_os="windows")]
impl App{
    pub fn create_window(
        &mut self,
        window_attributes:WindowAttributes,
        render_context_attributes:OpenGLRenderContextAttributes,
        graphics_attributes:Graphics2DAttributes,
    )->bool{
        self.window_storage.add_window(
            &self.event_loop,
            &self.window_class,
            window_attributes,
            render_context_attributes,
            &mut self.graphics_library,
            graphics_attributes,
        )
    }
}

#[cfg(target_os="windows")]
impl App{
    pub fn get_graphics(&self,id:usize)->Option<&Graphics>{
        self.window_storage.get_graphics(id)
    }

    pub fn get_graphics_unchecked(&self,id:usize)->&Graphics{
        self.window_storage.get_graphics_unchecked(id)
    }

    pub fn get_graphics_mut(&mut self,id:usize)->Option<&mut Graphics>{
        self.window_storage.get_graphics_mut(id)
    }

    pub fn get_graphics_unchecked_mut(&mut self,id:usize)->&mut Graphics{
        self.window_storage.get_graphics_unchecked_mut(id)
    }
}

#[cfg(target_os="windows")]
impl App{
    pub fn run<F:FnMut(Event,&mut AppControl,&mut LoopControl)>(&mut self,mut event_handler:F){
        let event_loop=unsafe{&mut *(&mut self.event_loop as *mut EventLoop)};

        let mut app_control=AppControl::new(self);

        event_loop.run(|event,control|{
            match &event{
                Event::WindowEvent{window_reference,window_event,argument}=>match window_event{
                    WindowEvent::Redraw=>{
                        let window_render_context=self.window_storage.get_render_context_unchecked(*argument as usize);
                        window_reference.request_redraw();
                        window_render_context.make_current(true);
                        event_handler(event.clone(),&mut app_control,control);
                        window_render_context.swap_buffers();
                        return
                    }

                    WindowEvent::CloseRequest=>{
                        //event_loop.set_main_window(app_control.app.window_storage.get_any_window());

                        window_reference.destroy()
                    }

                    _=>{}
                }
                _=>{},
            }

            event_handler(event,&mut app_control,control)
        });
    }
}

#[cfg(target_os="windows")]
pub struct AppAttributes{
    pub event_loop:EventLoopAttributes,
    pub windows_limit:u8,
    pub class:WindowClassAttributes,
    pub window:WindowAttributes,
    pub render_context:OpenGLRenderContextAttributes,
    pub graphics:Graphics2DAttributes,
}

#[cfg(target_os="windows")]
impl AppAttributes{
    pub fn new()->AppAttributes{
        Self{
            event_loop:EventLoopAttributes::new(),
            windows_limit:1u8,
            class:WindowClassAttributes::new("NewWindowClass"),
            window:WindowAttributes::new("NewWindow"),
            render_context:OpenGLRenderContextAttributes::new(),
            graphics:Graphics2DAttributes::new(),
        }
    }
}

#[cfg(target_os="windows")]
pub struct AppControl{
    app:&'static mut App,
}

#[cfg(target_os="windows")]
impl AppControl{
    pub fn new(app:&mut App)->AppControl{
        unsafe{
            Self{
                app:std::mem::transmute(app),
            }
        }
    }

    pub fn close(&self){
        
    }
}

#[cfg(target_os="windows")]
impl AppControl{
    pub fn get_graphics(&self,id:usize)->Option<&Graphics>{
        self.app.window_storage.get_graphics(id)
    }

    pub fn get_graphics_unchecked(&self,id:usize)->&Graphics{
        self.app.window_storage.get_graphics_unchecked(id)
    }

    pub fn get_graphics_mut(&mut self,id:usize)->Option<&mut Graphics>{
        self.app.window_storage.get_graphics_mut(id)
    }

    pub fn get_graphics_unchecked_mut(&mut self,id:usize)->&mut Graphics{
        self.app.window_storage.get_graphics_unchecked_mut(id)
    }
}