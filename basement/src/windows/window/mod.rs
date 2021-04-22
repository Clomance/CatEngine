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

    /// Returns window's client area size.
    /// 
    /// Возвращает размеры клиентской области окна.
    /// 
    /// [width, height]
    pub fn client_size(&self)->[u32;2]{
        let mut client_rectangle=[0i32;4];
        unsafe{
            let ptr=&mut client_rectangle as *mut [i32;4];
            GetClientRect(self.handle,ptr as usize as *mut RECT);
        }
        let [_,_,width,height]=client_rectangle;
        [
            width as u32,
            height as u32,
        ]
    }
}

/// Requests and sending events.
impl WindowReference{
    pub fn request_redraw(&self){
        unsafe{
            UpdateWindow(self.handle);
        }
    }

    pub fn destroy(&self){
        unsafe{
            DestroyWindow(self.handle);
        }
    }
}


impl std::fmt::Debug for WindowReference{
    fn fmt(&self,formatter:&mut std::fmt::Formatter)->std::fmt::Result{
        formatter.debug_struct("WindowReference").finish()
    }
}