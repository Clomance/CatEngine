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
    KeyboardButton,
    VirtualKeyCode,
};

mod window_procedure;
use window_procedure::window_subclass_procedure;

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
    WindowAttributes,
    WindowStyles,
    WindowSubclassArguments,
};

use winapi::{
    shared::{
        ntdef::{LPSTR,LPCWSTR},
        windef::{
            HWND,
            HDC,
            HGLRC,
            RECT,
        }
    },

    um::{
        processthreadsapi::GetCurrentThreadId,
        wingdi::SwapBuffers,
        winuser::{
            // ShowWindow,
            // SetFocus,
            // SetForegroundWindow,
            // SetCapture,
            DestroyWindow,
            GetDC,
            GetWindowRect,
            GetClientRect,
            UpdateWindow,
        },
        errhandlingapi::{GetLastError},
    }
};