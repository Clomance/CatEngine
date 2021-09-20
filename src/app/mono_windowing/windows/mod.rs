use crate::graphics::{
    Graphics,
    Graphics2DAttributes,
};

use cat_engine_basement::windows::{
    WindowClass,
    OpenGraphicsLibrary,
};

pub use cat_engine_basement::{
    windows::{
        EventLoop,
        Window,
        CursorIcon,
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
        WindowProcedure,
        ProcessEvent,
        Event,
        WindowEvent,
        quit,
    },
};

use std::{
    cell::UnsafeCell,
    mem::MaybeUninit
};

pub struct EmptyHandler;

impl<S> WindowProcedure<WindowInner<S>> for EmptyHandler{
    fn render(_:&Window,_:&mut WindowInner<S>){}
    fn handle(_:WindowEvent,_:&Window,_:&mut WindowInner<S>){}
}

pub struct WindowInner<S>{
    graphics:Graphics,
    context:OpenGLRenderContext,
    storage:*mut S,
}

impl<S> WindowInner<S>{
    pub fn graphics_ref(&self)->&Graphics{
        &self.graphics
    }

    pub fn graphics(&mut self)->&mut Graphics{
        &mut self.graphics
    }

    pub fn context(&self)->&OpenGLRenderContext{
        &self.context
    }

    pub fn storage(&mut self)->&mut S{
        unsafe{
            &mut *self.storage
        }
    }

    pub fn storage_ref(&self)->&S{
        unsafe{
            &*self.storage
        }
    }

    pub fn draw<F:FnMut(&Window,&mut Graphics,&S)>(&mut self,window:&Window,mut f:F)->Result<(),WinError>{
        self.context.make_current(true)?;

        let [width,height]=window.client_size();
        unsafe{
            self.graphics.core().viewport.set([0,0,width as i32,height as i32]);
        }
        self.graphics.draw_parameters().set_viewport([0f32,0f32,width as f32,height as f32]);

        f(window,&mut self.graphics,unsafe{&*self.storage});

        self.graphics.core().finish();
        self.context.swap_buffers()?;
        Ok(())
    }
}

/// A structure to easily create a windowed application.
/// 
/// Loads everything needed for drawing.
pub struct App<S:Sized+'static>{
    pub event_loop:EventLoop,
    window_class:WindowClass,
    pub window:Window,
    window_inner:UnsafeCell<Box<WindowInner<S>>>,
    storage:UnsafeCell<Box<S>>,
}

impl<S:Sized+'static> App<S>{
    /// Creates an application with the given attributes.
    /// 
    /// `W` is the type
    /// that implements the `WindowProcedure` trait
    /// that defines window's behavior.
    /// 
    /// `WindowInner` stores graphics and context structures and `S`.
    /// 
    /// `S` is user defined type for anything (e.g. for storing objects for rendering).
    pub fn new<W:WindowProcedure<WindowInner<S>>>(attributes:AppAttributes,storage:S)->App<S>{
        let event_loop=EventLoop::new(attributes.event_loop);

        let class=WindowClass::new(attributes.class).unwrap();

        let inner=MaybeUninit::<WindowInner<S>>::zeroed();
        let inner=unsafe{inner.assume_init()};
        let mut window_inner:Box<WindowInner<S>>=Box::new(inner);

        let window=Window::new::<EmptyHandler,WindowInner<S>>(
            &class,
            attributes.window,
            window_inner.as_mut()
        ).unwrap();

        let context=OpenGLRenderContext::new(
            &window,
            attributes.render_context
        ).unwrap();

        let library=OpenGraphicsLibrary::new();
        library.load_functions();

        let graphics=Graphics::new(attributes.graphics);

        let mut app_storage=Box::new(storage);

        unsafe{
            (window_inner.as_mut() as *mut WindowInner<S>).write(WindowInner{
                graphics,
                context,
                storage:app_storage.as_mut()
            });
        }

        unsafe{window.set_window_handle::<W,WindowInner<S>>()}

        Self{
            event_loop,
            window_class:class,
            window,
            window_inner:UnsafeCell::new(window_inner),
            storage:UnsafeCell::new(app_storage),
        }
    }

    /// Replaces the window procedure with functions defined by `W`.
    pub fn set_window_handle<W:WindowProcedure<WindowInner<S>>>(&self){
        unsafe{
            self.window.set_window_handle::<W,WindowInner<S>>()
        }
    }
}

impl<S:Sized+'static> App<S>{
    pub fn window_inner(&self)->&mut WindowInner<S>{
        unsafe{
            (&mut*self.window_inner.get()).as_mut()
        }
    }

    pub fn graphics(&self)->&mut Graphics{
        &mut self.window_inner().graphics
    }

    pub fn context(&self)->&OpenGLRenderContext{
        &self.window_inner().context
    }

    pub fn storage(&self)->&mut S{
        unsafe{
            (&mut*self.storage.get()).as_mut()
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