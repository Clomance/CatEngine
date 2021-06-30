use crate::graphics::{
    Graphics,
    Graphics2DAttributes,
};

mod windows_thread;
use windows_thread::{
    CommandSender,
    CommandResult,
    command_channel,
    create_windows_thread,
};

use cat_engine_basement::windows::{
    Window,
    WindowClass,
    EventLoop,
    OpenGLRenderContext,
    OpenGraphicsLibrary,
    WindowSubclassArguments,
};

pub use cat_engine_basement::{
    windows::{
        WinError,
        CursorIcon,
        Background,
        Fullscreen,
        Monitor,
        WindowAttributes,
        WindowClassAttributes,
        VirtualKeyCode,
        LoopControl,
        EventLoopAttributes,
        OpenGLRenderContextAttributes,
        UpdateInterval,
        winapi::um::processthreadsapi::GetCurrentThreadId,
    },
    event::{
        Event,
        WindowEvent,
    },
};

use std::thread::JoinHandle;

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
        command_sender:&CommandSender,
        window_class:&WindowClass,
        window_attributes:WindowAttributes,
        render_context_attributes:OpenGLRenderContextAttributes,
        graphics_library:&mut Option<OpenGraphicsLibrary>,
        graphics_attributes:Graphics2DAttributes,
    )->Result<bool,WinError>{
        if let Some(id)=self.free_ids.pop(){
            let main_thread_id=unsafe{GetCurrentThreadId()};
            let window_subclass_args=WindowSubclassArguments::new(main_thread_id as usize,id);
            self.window_subclass_args[id]=Some(window_subclass_args);
            let window_subclass_args_reference=self.window_subclass_args[id].as_ref().unwrap();

            match command_sender.create_window(
                window_class,
                window_subclass_args_reference,
                window_attributes
            ){
                CommandResult::Window{window}=>{
                    let render_context=OpenGLRenderContext::new(
                        window.context(),
                        render_context_attributes
                    ).unwrap();

                    if graphics_library.is_none(){
                        let library=OpenGraphicsLibrary::new();
                        library.load_functions();
                        *graphics_library=Some(library)
                    }

                    let graphics=Graphics::new(graphics_attributes);

                    self.windows[id]=Some(window);
                    self.render_contexts[id]=Some(render_context);

                    self.graphics[id]=Some(graphics);
                    Ok(true)
                }
                CommandResult::None=>Ok(false)
            }
        }
        else{
            Ok(false)
        }
    }

    pub fn remove_window(&mut self,id:usize)->Option<Window>{
        if let Some(maybe_window)=self.windows.get_mut(id){
            if let Some(window)=maybe_window.take(){
                unsafe{
                    let _render_contexts=self.render_contexts.get_unchecked_mut(id).take().unwrap();
                    let _graphics=self.graphics.get_unchecked_mut(id).take().unwrap();
                }
                self.free_ids.push(id);
                Some(window)
            }
            else{
                None
            }
        }
        else{
            None
        }
    }

    pub fn is_any_window(&self)->bool{
        self.free_ids.len()<self.free_ids.capacity()
    }

    pub fn get_any_window(&self)->Option<&Window>{
        for maybe_window in &self.windows{
            if let Some(window)=maybe_window{
                return Some(window)
            }
        }

        None
    }

    pub fn get_window(&self,id:usize)->Option<&Window>{
        if let Some(maybe_window)=self.windows.get(id){
            maybe_window.as_ref()
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

pub struct App{
    graphics_library:Option<OpenGraphicsLibrary>,
    event_loop:EventLoop,
    window_class:WindowClass,
    window_storage:WindowStorage,
    command_sender:CommandSender,
    windows_thread:JoinHandle<()>,
}

impl App{
    pub fn new(attributes:AppAttributes)->App{
        let mut graphics_library=None;

        let mut event_loop=EventLoop::new(attributes.event_loop);

        let class=WindowClass::new(attributes.class).unwrap();

        let mut window_storage=WindowStorage::empty(attributes.windows_limit as usize);

        let (command_sender,command_receiver)=command_channel();
        let windows_thread=create_windows_thread(command_receiver);

        window_storage.add_window(
            &command_sender,
            &class,
            attributes.window,
            attributes.render_context,
            &mut graphics_library,
            attributes.graphics,
        );

        Self{
            window_class:class,
            window_storage,
            graphics_library,
            event_loop,
            command_sender,
            windows_thread
        }
    }

    // pub fn get_proc_address(&self,name:&str)->*const std::ffi::c_void{
    //     self.graphics_library.get_proc_address(name) as *const std::ffi::c_void
    // }
}

impl App{
    pub fn create_window(
        &mut self,
        window_attributes:WindowAttributes,
        render_context_attributes:OpenGLRenderContextAttributes,
        graphics_attributes:Graphics2DAttributes,
    )->Result<bool,WinError>{
        self.window_storage.add_window(
            &self.command_sender,
            &self.window_class,
            window_attributes,
            render_context_attributes,
            &mut self.graphics_library,
            graphics_attributes,
        )
    }

    pub fn remove_window(&mut self,id:usize)->Option<Window>{
        self.window_storage.remove_window(id)
    }
}

impl App{
    pub fn is_any_window(&self)->bool{
        self.window_storage.is_any_window()
    }

    pub fn get_window(&self,id:usize)->Option<&Window>{
        self.window_storage.get_window(id)
    }

    pub fn get_window_unchecked(&self,id:usize)->&Window{
        self.window_storage.get_window_unchecked(id)
    }

    pub fn get_render_context(&self,id:usize)->Option<&OpenGLRenderContext>{
        self.window_storage.get_render_context(id)
    }

    pub fn get_render_context_unchecked(&self,id:usize)->&OpenGLRenderContext{
        self.window_storage.get_render_context_unchecked(id)
    }

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

impl App{
    pub fn run<F:FnMut(Event,&mut AppControl)>(&mut self,mut event_handler:F){
        let event_loop:&'static mut EventLoop=unsafe{std::mem::transmute(&mut self.event_loop)};

        let mut exit=false;

        event_loop.run(|event,loop_control|{
            let mut app_control=AppControl::new(self,loop_control);

            match &event{
                Event::WindowEvent{window_event,window_id}=>match window_event{
                    WindowEvent::Destroy=>{
                        let _window=app_control.app.window_storage.remove_window(*window_id);
                    }

                    _=>{}
                }
                _=>{},
            }

            event_handler(event,&mut app_control);
            if let LoopControl::Destroy=app_control.loop_control{
                exit=true;
                event_handler(Event::EventLoopDestroy,&mut app_control);
            }
        });

        if exit{
            self.command_sender.call_break();
        }
    }
}

impl Drop for App{
    fn drop(&mut self){
        self.command_sender.call_break();
    }
}

pub struct AppAttributes{
    pub windows_limit:u8,
    pub event_loop:EventLoopAttributes,
    pub class:WindowClassAttributes,
    pub window:WindowAttributes,
    pub render_context:OpenGLRenderContextAttributes,
    pub graphics:Graphics2DAttributes,
}

impl AppAttributes{
    pub fn new()->AppAttributes{
        Self{
            windows_limit:1u8,
            event_loop:EventLoopAttributes::new(),
            class:WindowClassAttributes::new("NewWindowClass"),
            window:WindowAttributes::new("NewWindow"),
            render_context:OpenGLRenderContextAttributes::new(),
            graphics:Graphics2DAttributes::new(),
        }
    }
}

pub struct AppControl{
    app:&'static mut App,
    loop_control:&'static mut LoopControl,
}

impl AppControl{
    pub fn new(app:&mut App,loop_control:&mut LoopControl)->AppControl{
        unsafe{
            Self{
                app:std::mem::transmute(app),
                loop_control:std::mem::transmute(loop_control)
            }
        }
    }

    pub fn create_window(
        &mut self,
        window_attributes:WindowAttributes,
        render_context_attributes:OpenGLRenderContextAttributes,
        graphics_attributes:Graphics2DAttributes
    )->Result<bool,WinError>{
        self.app.create_window(
            window_attributes,
            render_context_attributes,
            graphics_attributes
        )
    }

    /// Destroyes app's event loop and windows.
    pub fn exit(&mut self){
        *self.loop_control=LoopControl::Destroy;
    }

    /// Break app's event loop.
    pub fn break_loop(&mut self){
        *self.loop_control=LoopControl::Break;
    }

    /// Sets the 'lazy' mode flag.
    pub fn lazy(&mut self,lazy:bool){
        if lazy{
            *self.loop_control=LoopControl::Lazy;
        }
        else{
            *self.loop_control=LoopControl::Run;
        }
    }
}

impl AppControl{
    /// Checks whether an app has any windows.
    pub fn is_any_window(&self)->bool{
        self.app.window_storage.is_any_window()
    }

    pub fn get_window(&self,id:usize)->Option<&Window>{
        self.app.window_storage.get_window(id)
    }

    pub fn get_window_unchecked(&self,id:usize)->&Window{
        self.app.window_storage.get_window_unchecked(id)
    }

    pub fn get_render_context(&self,id:usize)->Option<&OpenGLRenderContext>{
        self.app.window_storage.get_render_context(id)
    }

    pub fn get_render_context_unchecked(&self,id:usize)->&OpenGLRenderContext{
        self.app.window_storage.get_render_context_unchecked(id)
    }

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

    pub fn draw<F:FnMut(&Window,&mut Graphics)>(&mut self,id:usize,mut f:F)->Result<(),WinError>{
        if let Some(window)=self.app.window_storage.get_window(id){
            let window:&'static Window=unsafe{std::mem::transmute(window)};
            // Указатель на графические функции (чтобы не ругался)
            let graphics:&'static mut Graphics=unsafe{std::mem::transmute(
                self.app.window_storage.get_graphics_unchecked_mut(id) as *mut Graphics
            )};

            let render_context=self.app.window_storage.get_render_context_unchecked(id);
            render_context.make_current(true)?;

            let [width,height]=window.client_size();
            graphics.core().viewport().set([0,0,width as i32,height as i32]);
            graphics.draw_parameters().change_viewport([0f32,0f32,width as f32,height as f32]);

            f(window,graphics);

            render_context.swap_buffers()?;
        }
        Ok(())
    }
}