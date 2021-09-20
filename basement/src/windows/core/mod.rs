pub mod bitmap;
use bitmap::Bitmap;

pub mod cursor;
use cursor::Cursor;

pub mod device_context;
use device_context::DeviceContext;

pub mod render_context;
use render_context::RenderContext;

pub mod window_class;
use window_class::WindowClass;

pub mod window;
use window::Window;

use std::mem::{
    transmute,
    transmute_copy
};

use winapi::um::errhandlingapi::GetLastError;

#[macro_export]
macro_rules! implement_handle_wrapper{
    ($wrapper:ty,$raw_handle:ty) => {
        impl $wrapper{
            #[inline(always)]
            pub fn from_raw(raw_handle:$raw_handle)->Option<$wrapper>{
                unsafe{
                    transmute(raw_handle)
                }
            }

            #[inline(always)]
            pub unsafe fn from_raw_unchecked(raw_handle:$raw_handle)->$wrapper{
                transmute(raw_handle)
            }

            #[inline(always)]
            pub fn to_raw(handle:Option<$wrapper>)->$raw_handle{
                unsafe{
                    transmute(handle)
                }
            }

            #[inline(always)]
            pub fn as_raw(&self)->$raw_handle{
                unsafe{
                    transmute_copy(self)
                }
            }
        }

        unsafe impl Sync for $wrapper{}
        unsafe impl Send for $wrapper{}
    };
}

pub struct WindowsCore{
    pub bitmap:Bitmap,
    pub cursor:Cursor,
    pub device_context:DeviceContext,
    pub render_context:RenderContext,
    pub window_class:WindowClass,
    pub window:Window,
}

impl WindowsCore{
    pub const fn new()->WindowsCore{
        Self{
            bitmap:Bitmap::new(),
            cursor:Cursor::new(),
            device_context:DeviceContext::new(),
            render_context:RenderContext::new(),
            window_class:WindowClass::new(),
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