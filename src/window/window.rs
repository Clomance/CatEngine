use crate::graphics::{Graphics2D,Graphics,GraphicsSettings};

#[cfg(feature="mouse_cursor_icon")]
use super::MouseCursorIconSettings;

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
    Frame,
    draw_parameters::DrawParameters,
    backend::glutin::DisplayCreationError,
    SwapBuffersError
};

use glium::glutin::{
    ContextBuilder,
    NotCurrent,
    monitor::MonitorHandle,
    event_loop::EventLoop,
    event::MouseScrollDelta,
    window::WindowBuilder,
};

use image::DynamicImage;

use std::path::{Path,PathBuf};

/// Типаж для создания страниц окна.
/// A trait for implementing window pages.
pub trait WindowPage<'a>{
    type Window:Window+'a;

    /// The return type of output when the loop is closed.
    type Output;

    /// Called when the window has been requested to close.
    fn on_close_requested(&mut self,window:&mut Self::Window);

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

    fn on_window_focused(&mut self,window:&mut Self::Window,focused:bool);

    fn on_suspended(&mut self,window:&mut Self::Window);
    fn on_resumed(&mut self,window:&mut Self::Window);

    fn on_file_dropped(&mut self,window:&mut Self::Window,path:PathBuf);
    fn on_file_hovered(&mut self,window:&mut Self::Window,path:PathBuf);
    fn on_file_hovered_canceled(&mut self,window:&mut Self::Window);

    /// Event loop has been stopped.
    /// 
    /// The page closes after this function is called.
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

        #[cfg(feature="mouse_cursor_icon")]
        mouse_cursor_icon_settings:MouseCursorIconSettings<PathBuf>,
    )->Result<Self,DisplayCreationError>;

    fn new<F>(setting:F)->Result<Self,DisplayCreationError>
            where F:FnOnce(Vec<MonitorHandle>,&mut WindowSettings){
        let event_loop=EventLoop::<InnerWindowEvent>::with_user_event();
        let monitors=event_loop.available_monitors().collect();

        let mut window_settings=WindowSettings::new();


        // Настройка
        setting(monitors,&mut window_settings);

        #[cfg(feature="mouse_cursor_icon")]
        let (window_builder,context_builder,graphics_settings,general_settings,mouse_cursor_icon_settings)
                =window_settings.devide::<PathBuf>();

        #[cfg(not(feature="mouse_cursor_icon"))]
        let (window_builder,context_builder,graphics_settings,general_settings)
                =window_settings.devide();


        #[cfg(feature="mouse_cursor_icon")]
        let window=Self::raw(
            window_builder,
            context_builder,
            graphics_settings,
            event_loop,
            general_settings,
            mouse_cursor_icon_settings,
        );

        #[cfg(not(feature="mouse_cursor_icon"))]
        let window=Self::raw(
            window_builder,
            context_builder,
            graphics_settings,
            event_loop,
            general_settings,
        );

        window
    }

    #[inline(always)]
    fn display(&self)->&Display{
        &self.window_base().display
    }

    /// Возвращает графическую основу.
    /// 
    /// Returns the graphics base.
    #[inline(always)]
    fn graphics(&mut self)->&mut Graphics2D{
        &mut self.window_base_mut().graphics
    }

    /// Даёт кадр для рисования.
    /// 
    /// Gives a frame for drawing.
    #[inline(always)]
    fn draw_raw<F:FnOnce(&mut DrawParameters,&mut Frame)>(&self,f:F)->Result<(),SwapBuffersError>{
        self.window_base().draw_raw(f)
    }

    /// Выполняет замыкание.
    /// 
    /// Executes the closure.
    #[inline(always)]
    fn draw<F:FnOnce(&mut DrawParameters,&mut Graphics)>(&self,f:F)->Result<(),SwapBuffersError>{
        self.window_base().draw(f)
    }


    /// Sets alpha channel for drawing with a changing alpha channel.
    #[cfg(feature="alpha_smoothing")]
    #[inline(always)]
    fn set_alpha(&mut self,alpha:f32){
        self.window_base_mut().set_alpha(alpha)
    }

    /// Sets smooth for drawing with a changing alpha channel.
    #[cfg(feature="alpha_smoothing")]
    #[inline(always)]
    fn set_smooth(&mut self,smooth:f32){
        self.window_base_mut().set_smooth(smooth)
    }

    /// Sets smooth and zeroes alpha channel for drawing with a changing alpha channel.
    #[cfg(feature="alpha_smoothing")]
    #[inline(always)]
    fn set_new_smooth(&mut self,smooth:f32){
        self.window_base_mut().set_new_smooth(smooth)
    }

    /// Выдаёт альфа-канал, возвращает его следующее значение.
    /// 
    /// Нужна для рисования с изменяющимся альфа-канала.
    /// 
    /// Gives an alpha channel, returns it's next value.
    /// 
    /// Needed for drawing with a changing alpha channel.
    /// 
    /// feature = "alpha_smoothing"
    #[cfg(feature="alpha_smoothing")]
    fn draw_smooth<F:FnOnce(f32,&mut DrawParameters,&mut Graphics)>(&mut self,f:F)->Result<f32,SwapBuffersError>{
        self.window_base_mut().draw_smooth(f)
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




    /// feature = "mouse_cursor_icon
    #[cfg(feature="mouse_cursor_icon")]
    #[inline(always)]
    fn set_user_cursor_visible(&mut self,visible:bool){
        self.window_base_mut().mouse_icon.set_visible(visible)
    }
}