use super::{
    // structs
    Monitor,
    Bitmap,
    Icon,
    // enums
    WindowEvent,
};

mod window_procedure;
use window_procedure::{
    default_window_procedure,
    window_procedure,
    window_settings_auto_redraw,
};

mod window_class;
pub use window_class::{
    CursorIcon,
    Background,
    WindowClass,
    WindowClassAttributes,
    WindowClassStyle,
};

mod window;
pub use window::{
    Window,
    WindowStyle,
    WindowStyles,
    ExtendedWindowStyle,
    ExtendedWindowStyles,
    Fullscreen,
    CreateParameters,
    WindowAttributes,
};


/// Defines window's behavior.
pub trait WindowProcedure<A:Sized>{
    fn render(window:&Window,args:&mut A);

    fn handle(event:WindowEvent,window:&Window,args:&mut A);
}

/// Indicates to the event loop that it's thread has made a request to close.
pub fn quit(){
    unsafe{
        winapi::um::winuser::PostQuitMessage(0);
    }
}