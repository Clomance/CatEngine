use super::{
    // structs
    Monitor,
    Bitmap,
    Icon,
    WinError,
    // enums
    WindowEvent,
};

mod messages;
use messages::{
    WindowMessage,
};

pub use messages::WindowResizeType;

mod window_procedure;
use window_procedure::{
    setup_window_procedure,
    window_procedure,
    window_settings_auto_redraw,
};

mod window_class;
pub use window_class::{
    CursorIcon,
    SystemCursor,
    Background,
    WindowClass,
    WindowClassAttributes,
    WindowClassStyle,
    WindowBackgroundSystemColour,
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

use std::any::Any;

/// Defines window's behavior.
pub trait WindowProcedure{
    type CreateParameters;
    type Data:Copy;

    /// Called when an application requests that a window be created.
    fn create(window:&Window,parameters:&mut Self::CreateParameters)->Result<Self::Data,WinError>;

    /// Called as a signal that a window or an application should terminate.
    fn close_request(window:&Window,data:Self::Data);

    /// Called when a window is being destroyed,
    /// after the window is removed from the screen.
    fn destroy(window:&Window,data:Self::Data);

    /// Called when the system or another application
    /// makes a request to paint a portion of an application's window.
    fn paint(window:&Window,data:Self::Data);

    /// Called if the mouse causes the cursor to move
    /// within a window and mouse input is not captured.
    /// 
    /// Note that you have to set up cursor manually each time the function is called.
    #[cfg(feature="set_cursor_event")]
    fn set_cursor(window:&Window,data:Self::Data);

    /// Called after window's size has changed.
    /// 
    /// `client_size` specifies the new width of the client area.
    fn resized(client_size:[u16;2],resize_type:WindowResizeType,window:&Window,data:Self::Data);

    /// Called after a window has been moved.
    /// 
    /// `client_position` contains coordinates of the upper-left corner of the client area of the window.
    fn moved(client_position:[i16;2],window:&Window,data:Self::Data);

    /// Called in other cases.
    fn handle(event:WindowEvent,window:&Window,data:Self::Data);

    /// Called when one of the functions above panics
    /// and `catch_unwind` catches the panic in `WndProc`.
    /// 
    /// You have to handle this panic manually.
    /// The engine just continue processing events with the default function (`DefWindowProcW`).
    /// 
    /// This capturing is needed because all of the `WindowProcedure` functions
    /// are called by the system beyond the Rust rules.
    /// 
    /// You can remove panic capturing with disabling the `wnd_proc_catch_panic` feature.
    /// In that case any panic will cause aborting the process.
    /// 
    /// Note that this function may not catch all panics in Rust.
    /// A panic in Rust is not always implemented via unwinding,
    /// but can be implemented by aborting the process as well.
    /// This function only catches unwinding panics, not those that abort the process.
    #[cfg(feature="wnd_proc_catch_panic")]
    fn catch_panic(window:&Window,data:Self::Data,error:Box<dyn Any+Send>);
}

/// Indicates to the event loop that it's thread has made a request to close.
pub fn quit(exit_code:i32){
    unsafe{
        winapi::um::winuser::PostQuitMessage(exit_code);
    }
}