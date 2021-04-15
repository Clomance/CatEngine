use super::{
    // structs
    Monitor,
    Bitmap,
    Icon,
    EventHandler,
    // enums
    Event,
    WindowEvent,
    LoopControl,
    KeyboardButton,
    VirtualKeyCode,
    MouseButton,
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
        //errhandlingapi::{GetLastError},
    }
};

#[derive(Clone,Copy)]
pub struct WindowReference{
    handle:HWND,
    context:HDC,
}

impl WindowReference{
    pub fn new(window:&Window)->WindowReference{
        Self{
            handle:window.handle(),
            context:window.context(),
        }
    }

    pub fn swap_buffers(&self){
        unsafe{
            let _result=SwapBuffers(self.context);
        }
    }

    pub fn request_redraw(&self){
        unsafe{
            let _result=UpdateWindow(self.handle);
        }
    }
}