use super::{
    // enums
    MouseButton,
    KeyboardButton,
    // structs
    Window,
};

use glium::glutin::event::{MouseScrollDelta,ModifiersState};

use std::path::PathBuf;

/// Типаж для создания страниц окна.
/// A trait for implementing window pages.
pub trait WindowPage<'a>{
    /// The type of output when
    /// the 'page' (for the `PagedWindow`)/ window (for the `DynamicWindow`) is closed.
    type Output;

    /// Called when the window has been requested to close.
    fn on_window_close_requested(&mut self,window:&mut Window);

    /// feature != "lazy"
    #[cfg(not(feature="lazy"))]
    fn on_update_requested(&mut self,window:&mut Window);

    fn on_redraw_requested(&mut self,window:&Window);

    fn on_mouse_pressed(&mut self,window:&mut Window,button:MouseButton);
    fn on_mouse_released(&mut self,window:&mut Window,button:MouseButton);
    fn on_mouse_scrolled(&mut self,window:&mut Window,scroll:MouseScrollDelta);
    fn on_mouse_moved(&mut self,window:&mut Window,position:[f32;2]);

    fn on_keyboard_pressed(&mut self,window:&mut Window,button:KeyboardButton);
    fn on_keyboard_released(&mut self,window:&mut Window,button:KeyboardButton);
    fn on_character_recieved(&mut self,window:&mut Window,character:char);

    fn on_window_resized(&mut self,window:&mut Window,new_size:[u32;2]);
    fn on_window_moved(&mut self,window:&mut Window,position:[i32;2]);

    /// Called when the window loses or gains focus.
    fn on_window_focused(&mut self,window:&mut Window,focused:bool);

    fn on_suspended(&mut self,window:&mut Window);
    fn on_resumed(&mut self,window:&mut Window);

    fn on_modifiers_changed(&mut self,window:&mut Window,modifiers:ModifiersState);

    /// feature = "file_drop"
    #[cfg(feature="file_drop")]
    fn on_file_dropped(&mut self,window:&mut Window,path:PathBuf);
    /// feature = "file_drop"
    #[cfg(feature="file_drop")]
    fn on_file_hovered(&mut self,window:&mut Window,path:PathBuf);
    /// feature = "file_drop"
    #[cfg(feature="file_drop")]
    fn on_file_hovered_canceled(&mut self,window:&mut Window);

    /// Called when the event loop has been stopped.
    /// 
    /// The 'page' (for the `PagedWindow`)/ window (for the `DynamicWindow`)
    /// is closed after this function is called.
    fn on_event_loop_closed(&mut self,window:&mut Window)->Self::Output;
}