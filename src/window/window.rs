use crate::graphics::{
    Graphics,
    GraphicsSettings,
    two_dimensions::Graphics2D
};

use super::{
    // enums
    MouseButton,
    KeyboardButton,
    InnerWindowEvent,
    // structs
    WindowBase,
    GeneralSettings,
    WindowSettings,
};

use glium::{
    Display,
    draw_parameters::DrawParameters,
    backend::glutin::DisplayCreationError,
    SwapBuffersError
};

use glium::glutin::{
    ContextBuilder,
    NotCurrent,
    monitor::MonitorHandle,
    event_loop::EventLoop,
    event::{MouseScrollDelta,ModifiersState},
    window::WindowBuilder,
};

use image::DynamicImage;

use std::path::{Path,PathBuf};

/// Типаж для создания страниц окна.
/// A trait for implementing window pages.
pub trait WindowPage<'a>{
    type Window:Window+'a;

    /// The type of output when
    /// the 'page' (for the `PagedWindow`)/ window (for the `DynamicWindow`) is closed.
    type Output;

    /// Called when the window has been requested to close.
    fn on_window_close_requested(&mut self,window:&mut Self::Window);

    /// feature != "lazy"
    #[cfg(not(feature="lazy"))]
    fn on_update_requested(&mut self,window:&mut Self::Window);

    fn on_redraw_requested(&mut self,window:&mut Self::Window);

    fn on_mouse_pressed(&mut self,window:&mut Self::Window,button:MouseButton);
    fn on_mouse_released(&mut self,window:&mut Self::Window,button:MouseButton);
    fn on_mouse_scrolled(&mut self,window:&mut Self::Window,scroll:MouseScrollDelta);
    fn on_mouse_moved(&mut self,window:&mut Self::Window,position:[f32;2]);

    fn on_keyboard_pressed(&mut self,window:&mut Self::Window,button:KeyboardButton);
    fn on_keyboard_released(&mut self,window:&mut Self::Window,button:KeyboardButton);
    fn on_character_recieved(&mut self,window:&mut Self::Window,character:char);

    fn on_window_resized(&mut self,window:&mut Self::Window,new_size:[u32;2]);
    fn on_window_moved(&mut self,window:&mut Self::Window,position:[i32;2]);

    /// Called when the window loses or gains focus.
    fn on_window_focused(&mut self,window:&mut Self::Window,focused:bool);

    fn on_suspended(&mut self,window:&mut Self::Window);
    fn on_resumed(&mut self,window:&mut Self::Window);

    fn on_modifiers_changed(&mut self,window:&mut Self::Window,modifiers:ModifiersState);

    /// feature = "file_drop"
    #[cfg(feature="file_drop")]
    fn on_file_dropped(&mut self,window:&mut Self::Window,path:PathBuf);
    /// feature = "file_drop"
    #[cfg(feature="file_drop")]
    fn on_file_hovered(&mut self,window:&mut Self::Window,path:PathBuf);
    /// feature = "file_drop"
    #[cfg(feature="file_drop")]
    fn on_file_hovered_canceled(&mut self,window:&mut Self::Window);

    /// Called when the event loop has been stopped.
    /// 
    /// The 'page' (for the `PagedWindow`)/ window (for the `DynamicWindow`)
    /// is closed after this function is called.
    fn on_event_loop_closed(&mut self,window:&mut Self::Window)->Self::Output;
}



/// Типаж, помогающий создать более сложное окно на базе `WindowBase`.
/// A trait that helps to create a more complex window based on a `WindowBase`.
pub trait Window:Sized{
    fn window_base(&self)->&WindowBase;
    fn window_base_mut(&mut self)->&mut WindowBase;

    fn raw(
        window_builder:WindowBuilder,
        context_builder:ContextBuilder<NotCurrent>,
        graphics_settings:GraphicsSettings,
        event_loop:EventLoop<InnerWindowEvent>,
        general_settings:GeneralSettings,
    )->Result<(Self,Graphics2D),DisplayCreationError>;

    fn new<F>(setting:F)->Result<(Self,Graphics2D),DisplayCreationError>
            where F:FnOnce(Vec<MonitorHandle>,&mut WindowSettings){
        let event_loop=EventLoop::<InnerWindowEvent>::with_user_event();
        let monitors=event_loop.available_monitors().collect();

        let mut window_settings=WindowSettings::new();


        // Настройка
        setting(monitors,&mut window_settings);


        let mut window_builder=WindowBuilder::default();
        window_builder.window=window_settings.window_attributes;

        let mut context_builder=ContextBuilder::new();
        context_builder.gl_attr.vsync=window_settings.vsync;
        context_builder.gl_attr.debug=window_settings.debug;

        context_builder.pf_reqs=window_settings.pixel_fmt_req;

        Self::raw(
            window_builder,
            context_builder,
            window_settings.graphics_base_settings,
            event_loop,
            window_settings.general,
        )
    }

    #[inline(always)]
    fn display(&self)->&Display{
        &self.window_base().display
    }

    /// Выполняет замыкание
    /// и рисует пользовательский курсор мыши,
    /// если установлен.
    /// 
    /// Executes the closure
    /// and draws user's mouse cursor if set.
    #[inline(always)]
    fn draw<F:FnOnce(&mut DrawParameters,&mut Graphics)>(&mut self,graphics_base:&mut Graphics2D,f:F)->Result<(),SwapBuffersError>{
        self.window_base_mut().draw(graphics_base,f)
    }


    /// Возвращает скриншот.
    /// 
    /// Returns a screenshot.
    #[inline(always)]
    fn screenshot(&self)->Option<DynamicImage>{
        self.window_base().screenshot()
    }

    /// Сохраняет скриншот в формате png.
    /// 
    /// Saves a screenshot in png format.
    #[inline(always)]
    fn save_screenshot<P:AsRef<Path>>(&self,path:P){
        self.window_base().save_screenshot(path)
    }
}