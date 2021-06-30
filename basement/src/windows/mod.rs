mod error;
pub use error::WinError;

mod images;
pub use images::{
    Bitmap,
    Icon,
};

mod opengl;
pub use opengl::{
    OpenGraphicsLibrary,
    OpenGLRenderContext,
    OpenGLRenderContextAttributes,
};

mod monitor;
pub use monitor::Monitor;

mod window;
pub use window::{
    Fullscreen,
    Window,
    WindowAttributes,
    CursorIcon,
    Background,
    WindowClass,
    WindowClassAttributes,
    WindowSubclassArguments
};

mod event_loop;
pub use event_loop::{
    Ticks,
    EventLoop,
    LoopControl,
    UpdateInterval,
    EventLoopAttributes,
    KeyboardButton,
    VirtualKeyCode,
};

use winapi::{
    shared::{
        minwindef::{
            PROC,
            HMODULE,
        },
        windef::{
            HGLRC,
            HDC,
            HWND,
        },
    },
};

pub use winapi;