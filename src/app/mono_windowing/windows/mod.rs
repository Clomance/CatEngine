use crate::graphics::{
    Graphics,
    Graphics2DAttributes,
};

use cat_engine_basement::windows::{
    WindowClass,
    OpenGraphicsLibrary,
    WindowProcedure,
};

pub use cat_engine_basement::{
    windows::{
        EventLoop,
        Window,
        CursorIcon,
        SystemCursor,
        Background,
        Fullscreen,
        Monitor,
        MouseButton,
        OpenGLRenderContext,
        WindowAttributes,
        WindowClassAttributes,
        VirtualKeyCode,
        LoopControl,
        EventLoopAttributes,
        OpenGLRenderContextAttributes,
        EventInterval,
        WinError,
        ProcessEvent,
        Event,
        WindowEvent,
        WindowResizeType,
        quit,
    },
};

use std::{
    cell::UnsafeCell,
    mem::replace,
    marker::PhantomData,
};

pub enum AppCreateParamters<S,C>{
    None,
    Get(OpenGLRenderContextAttributes,Graphics2DAttributes,C),
    Return(*mut OpenGLRenderContext,*mut Graphics,*mut S),
}

impl<S,C> AppCreateParamters<S,C>{
    pub fn take(&mut self)->AppCreateParamters<S,C>{
        replace(self,AppCreateParamters::None)
    }
}

/// Defines app window's behavior.
pub trait AppWindowProcedure<S,C>{
    /// Called when an application requests that a window be created.
    fn create(window:&Window,data:(&mut OpenGLRenderContext,&mut Graphics,C))->S;

    /// Called as a signal that a window or an application should terminate.
    fn close_request(window:&Window,data:(&mut OpenGLRenderContext,&mut Graphics,&mut S));

    /// Called when a window is being destroyed,
    /// after the window is removed from the screen.
    fn destroy(window:&Window,data:(&mut OpenGLRenderContext,&mut Graphics,&mut S));

    /// Called when the system or another application
    /// makes a request to paint a portion of an application's window.
    fn paint(window:&Window,data:(&mut OpenGLRenderContext,&mut Graphics,&mut S));

    /// Called if the mouse causes the cursor to move
    /// within a window and mouse input is not captured.
    #[cfg(feature="set_cursor_event")]
    fn set_cursor(window:&Window,data:(&mut OpenGLRenderContext,&mut Graphics,&mut S));

    /// Called after window's size has changed.
    /// 
    /// `client_size` specifies the new width of the client area.
    fn resized(client_size:[u16;2],resize_type:WindowResizeType,window:&Window,data:(&mut OpenGLRenderContext,&mut Graphics,&mut S));

    /// Called after a window has been moved.
    /// 
    /// `client_position` contains coordinates of the upper-left corner of the client area of the window.
    fn moved(client_position:[i16;2],window:&Window,data:(&mut OpenGLRenderContext,&mut Graphics,&mut S));

    fn handle(event:WindowEvent,window:&Window,data:(&mut OpenGLRenderContext,&mut Graphics,&mut S));

    #[cfg(feature="wnd_proc_catch_panic")]
    fn catch_panic(window:&Window,data:(*mut OpenGLRenderContext,*mut Graphics,*mut S),error:Box<dyn std::any::Any+Send>);
}

pub struct AppWindowHandler<P:AppWindowProcedure<S,C>,S,C>{
    procedure:PhantomData<P>,
    create_parameter:PhantomData<C>,
    storage:PhantomData<S>,
}

impl<P:AppWindowProcedure<S,C>,S,C> WindowProcedure for AppWindowHandler<P,S,C>{
    type CreateParameters=AppCreateParamters<S,C>;
    type Data=(*mut OpenGLRenderContext,*mut Graphics,*mut S);

    fn create(
        window:&Window,
        create_paramters:&mut AppCreateParamters<S,C>
    )->Result<Self::Data,WinError>{
        if let AppCreateParamters::Get(rca,ga,storage)=create_paramters.take(){
            // create a render context
            match OpenGLRenderContext::new(window,rca){
                Ok(render_context)=>{
                    // load opengl functions
                    let opengl_library=OpenGraphicsLibrary::new();
                    opengl_library.load_functions();

                    let graphics=Graphics::new(ga);

                    let empty_storage:S=unsafe{std::mem::MaybeUninit::zeroed().assume_init()};
                    let window_data=Box::leak(Box::new((render_context,graphics,empty_storage)));

                    // call user function
                    let storage=P::create(window,(&mut window_data.0,&mut window_data.1,storage));

                    // write with raw pointer to avoid dropping zeroed `S`
                    unsafe{((&mut window_data.2) as *mut S).write(storage)}

                    *create_paramters=AppCreateParamters::Return(
                        &mut window_data.0,
                        &mut window_data.1,
                        &mut window_data.2
                    );

                    Ok((&mut window_data.0,&mut window_data.1,&mut window_data.2))
                }

                Err(error)=>Err(error),
            }
        }
        else{
            unreachable!()
        }
    }

    fn close_request(window:&Window,data:Self::Data){
        let data=unsafe{(&mut*data.0,&mut*data.1,&mut*data.2)};
        P::close_request(window,data)
    }

    fn destroy(window:&Window,data:Self::Data){
        P::destroy(window,unsafe{(&mut*data.0,&mut*data.1,&mut*data.2)});
        // thats the way we drop OpenGLRenderContext
        unsafe{
            Box::<(OpenGLRenderContext,Graphics,S)>::from_raw(std::mem::transmute(data.0));
        }
    }

    fn paint(window:&Window,data:Self::Data){
        let (render_context,graphics,storage)=unsafe{(&mut*data.0,&mut*data.1,&mut*data.2)};

        render_context.make_current(true).unwrap();

        let [width,height]=window.client_size();

        unsafe{
            graphics.core().parameters.viewport.set([0,0,width as i32,height as i32]);
        }
        graphics.graphics_2d.draw_parameters().set_viewport([0f32,0f32,width as f32,height as f32]);

        P::paint(window,(render_context,graphics,storage));

        unsafe{
            graphics.core().finish()
        }

        render_context.swap_buffers().unwrap();
    }

    #[cfg(feature="set_cursor_event")]
    fn set_cursor(window:&Window,data:Self::Data){
        let data=unsafe{(&mut*data.0,&mut*data.1,&mut*data.2)};
        P::set_cursor(window,data)
    }

    fn resized(client_size:[u16;2],resize_type:WindowResizeType,window:&Window,data:Self::Data){
        let data=unsafe{(&mut*data.0,&mut*data.1,&mut*data.2)};
        P::resized(client_size,resize_type,window,data)
    }

    fn moved(client_position:[i16;2],window:&Window,data:Self::Data){
        let data=unsafe{(&mut*data.0,&mut*data.1,&mut*data.2)};
        P::moved(client_position,window,data)
    }

    fn handle(event:WindowEvent,window:&Window,data:Self::Data){
        let data=unsafe{(&mut*data.0,&mut*data.1,&mut*data.2)};
        P::handle(event,window,data)
    }

    #[cfg(feature="wnd_proc_catch_panic")]
    fn catch_panic(window:&Window,data:Self::Data,error:Box<dyn std::any::Any+Send>){
        P::catch_panic(window,data,error)
    }
}

/// A structure to easily create a windowed application.
/// 
/// Loads everything needed for drawing.
pub struct App<S:Sized+'static>{
    pub event_loop:EventLoop,
    window_class:WindowClass,
    pub window:Window,
    data:(*mut OpenGLRenderContext,*mut Graphics,*mut S),
}

impl<S:Sized+'static> App<S>{
    /// Creates an application with the given attributes.
    pub fn new<P:AppWindowProcedure<S,C>,C>(
        attributes:AppAttributes,
        storage_parameters:C
    )->Result<App<S>,WinError>{
        let event_loop=EventLoop::new(attributes.event_loop);

        let class=WindowClass::new(attributes.class)?;

        let mut paramenters=AppCreateParamters::Get(
            attributes.render_context,
            attributes.graphics,
            storage_parameters
        );
        let window=Window::new::<AppWindowHandler<P,S,C>>(&class,attributes.window,&mut paramenters)?;
        if let AppCreateParamters::Return(c,g,s)=paramenters.take(){
            Ok(
                Self{
                    event_loop,
                    window_class:class,
                    window,
                    data:(c,g,s),
                }
            )
        }
        else{
            unreachable!()
        }
    }

    /// Replaces the window procedure.
    pub fn set_window_handle<P:AppWindowProcedure<S,C>,C>(&self){
        unsafe{
            self.window.set_window_handle::<AppWindowHandler<P,S,C>>()
        }
    }
}

impl<S:Sized+'static> App<S>{
    pub fn context(&self)->&mut OpenGLRenderContext{
        unsafe{
            &mut*self.data.0
        }
    }

    pub fn graphics(&self)->&mut Graphics{
        unsafe{
            &mut*self.data.1
        }
    }

    pub fn storage(&self)->&mut S{
        unsafe{
            &mut*self.data.2
        }
    }
}

pub struct AppAttributes{
    pub event_loop:EventLoopAttributes,
    pub class:WindowClassAttributes,
    pub window:WindowAttributes,
    pub render_context:OpenGLRenderContextAttributes,
    pub graphics:Graphics2DAttributes,
}

impl AppAttributes{
    pub fn new()->AppAttributes{
        Self{
            event_loop:EventLoopAttributes::new(),
            class:WindowClassAttributes::new("CatEngineWindowClass"),
            window:WindowAttributes::new("CatEngineWindow"),
            render_context:OpenGLRenderContextAttributes::new(),
            graphics:Graphics2DAttributes::new(),
        }
    }
}