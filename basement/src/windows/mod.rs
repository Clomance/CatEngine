mod images;
pub use images::{
    Bitmap,
    Icon,
};

mod opengl;
pub use opengl::{
    GraphicsLibrary,
    RenderContext,
    RenderContextAttributes,
};

mod monitor;
use monitor::Monitor;

mod window;
pub use window::{
    Fullscreen,
    Window,
    WindowReference,
    WindowAttributes,
    CursorIcon,
    Background,
    WindowClass,
    WindowClassAttributes
};

mod event_loop;
pub use event_loop::{
    Ticks,
    EventLoop,
    LoopControl,
    UpdateInterval,
    EventLoopAttributes,
    EventHandler,

    Event,
    WindowEvent,
    MouseButton,
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