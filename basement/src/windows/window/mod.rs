use crate::event::{
    Event,
    WindowEvent,
    MouseButton,
};

use super::{
    // structs
    Monitor,
    Bitmap,
    Icon,
    // enums
    LoopControl,
    VirtualKeyCode,
};

mod window_procedure;
use window_procedure::{
    default_window_procedure,
    window_procedure
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
    WindowStyles,
};


/// Defines window's behavior.
pub trait WindowProcedure<A:Sized>{
    fn handle(window:&Window,args:&mut A,event:WindowEvent);
}

/// Indicates to the event loop that it's thread has made a request to close.
pub fn quit(){
    unsafe{
        winapi::um::winuser::PostQuitMessage(0);
    }
}