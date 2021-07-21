use crate::event::{
    WindowEvent,
    MouseButton,
};

use super::{
    // structs
    Monitor,
    Bitmap,
    Icon,
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
    Fullscreen,
    Window,
    CreateParameters,
    WindowAttributes,
};


/// Defines window's behavior.
pub trait WindowProcedure<A:Sized>{
    fn handle(event:WindowEvent,window:&Window,args:&mut A);
}

/// Indicates to the event loop that it's thread has made a request to close.
pub fn quit(){
    unsafe{
        winapi::um::winuser::PostQuitMessage(0);
    }
}