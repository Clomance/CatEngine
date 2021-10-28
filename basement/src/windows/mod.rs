pub mod core;
use self::core::{
    Colour,
    ColourResult,
    WindowsCore,
};

// pub mod level0;

mod error;
pub use error::WinError;

mod images;
pub use images::{
    Bitmap,
    Icon,
};

#[cfg(feature="opengl")]
mod opengl;
#[cfg(feature="opengl")]
pub use opengl::{
    OpenGraphicsLibrary,
    OpenGLRenderContext,
    OpenGLRenderContextAttributes,
};

mod event;
pub use event::{
    Event,
    ProcessEvent,
    WindowEvent,
    MouseButton,
    VirtualKeyCode,
};

mod monitor;
pub use monitor::Monitor;

mod window;
pub use window::{
    WindowProcedure,
    Fullscreen,
    Window,
    WindowAttributes,
    CursorIcon,
    SystemCursor,
    Background,
    WindowClass,
    WindowClassAttributes,
    WindowResizeType,
    WindowBackgroundSystemColour,
    // functions
    quit
};

mod event_loop;
pub use event_loop::{
    Ticks,
    EventLoop,
    LoopControl,
    EventInterval,
    EventLoopAttributes,
};

pub use winapi;

pub type WinColour=Colour;
pub type WinColourResult=ColourResult;

/// Gives an access to raw Windows functions.
pub static mut WinCore:WindowsCore=WindowsCore::new();