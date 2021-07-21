pub mod window;
use window::Window;

use core::ptr::null_mut;

use winapi::{
    shared::ntdef::{
        MAKELANGID,
        LANG_NEUTRAL,
        SUBLANG_DEFAULT,
    },
    um::{
        errhandlingapi::GetLastError,
        winbase::{
            FORMAT_MESSAGE_FROM_SYSTEM,
            FORMAT_MESSAGE_IGNORE_INSERTS,
            FORMAT_MESSAGE_ALLOCATE_BUFFER,

            FormatMessageW,
        },
    }
};

pub struct WindowsCore{
    pub window:Window,
}

impl WindowsCore{
    pub const fn new()->WindowsCore{
        Self{
            window:Window::new(),
        }
    }
}

impl WindowsCore{
    pub fn get_last_error(&self)->u32{
        unsafe{
            GetLastError()
        }
    }
}