use super::{
    Error,
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
};

mod class;
pub use class::{
    CursorIcon,
    SystemCursor,
    Background,
    WindowClass,
    WindowClassAttributes,
    WindowClassStyle,
    WindowClassStyles,
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

/// Defines window's behavior.
pub trait WindowProcedure{
    type CreateParameters;

    /// Data that is shared between all window functions.
    /// 
    /// Boxed after the `create` function is called.
    /// 
    /// Dropped after the window has been destroyed (after the `destroy` function).
    type Data;

    /// Called when an application requests that the window be created.
    fn create(window:&Window,parameters:&mut Self::CreateParameters)->Result<Self::Data,Error>;

    /// Called when the data returned from `create` is boxed.
    /// 
    /// Now the data may safety be accessed with raw pointers.
    fn data_packed(window:&Window,parameters:&mut Self::CreateParameters,data:&mut Self::Data);

    /// Called as a signal that the window or application should terminate.
    fn close(window:&Window,data:&mut Self::Data);

    /// Called when the window is being destroyed,
    /// after the window is removed from the screen.
    fn destroy(window:&Window,data:&mut Self::Data);

    /// Called when the system or another application
    /// makes a request to paint a portion of an application's window.
    fn paint(window:&Window,data:&mut Self::Data);

    /// Called if the mouse causes the cursor to move
    /// within the window and mouse input is not captured.
    /// 
    /// Note that you have to set up cursor manually each time the function is called.
    #[cfg(feature="set_cursor_event")]
    fn set_cursor(window:&Window,data:&mut Self::Data);

    /// Called after window's size has changed.
    /// 
    /// `client_size` specifies the new width of the client area.
    fn resized(client_size:[u16;2],resize_type:WindowResizeType,window:&Window,data:&mut Self::Data);

    /// Called after the window has been moved.
    /// 
    /// `client_position` contains coordinates of the upper-left corner of the client area of the window.
    fn moved(client_position:[i16;2],window:&Window,data:&mut Self::Data);

    /// Called in other cases.
    fn handle(event:WindowEvent,window:&Window,data:&mut Self::Data);

    /// For custom events.
    fn user_event(w_param:usize,l_param:isize,window:&Window,data:&mut Self::Data);

    /// Called when one of the functions above panics
    /// and `catch_unwind` catches the panic.
    /// 
    /// You have to handle this panic manually.
    /// The window just will continue processing events with the default function (`DefWindowProcW`).
    /// 
    /// If the window panics upon creation, `data` is `None`,
    /// otherwise it always points to the shared data (destoyed after the `destroy` function has been called).
    /// 
    /// This capturing is needed because all of the `WindowProcedure` functions
    /// are called by the operation system beyond the Rust rules.
    /// 
    /// You can remove panic capturing with disabling the `wnd_proc_catch_panic` feature.
    /// In that case any panic will abort the process.
    /// 
    /// Note that this function may not catch all panics in Rust.
    /// A panic in Rust is not always implemented via unwinding,
    /// but can be implemented by aborting the process as well.
    /// This function only catches unwinding panics, not those that abort the process.
    #[cfg(all(feature="wnd_proc_catch_panic",not(feature="wnd_proc_catch_panic_default")))]
    fn catch_panic(window:&Window,data:Option<&mut Self::Data>,error:Box<dyn std::any::Any+Send>);

    #[cfg(all(feature="wnd_proc_catch_panic",feature="wnd_proc_catch_panic_default"))]
    fn catch_panic(window:&Window,data:Option<&mut Self::Data>,error:Box<dyn std::any::Any+Send>){
        println!("{:?}",error);
        if let Some(_)=data{
            println!("Destroying the window.");
            let _=window.destroy();
        }
    }
}

/// Indicates to the event loop that it's thread has made a request to close.
pub fn quit(exit_code:i32){
    unsafe{
        winapi::um::winuser::PostQuitMessage(exit_code);
    }
}