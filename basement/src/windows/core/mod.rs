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

pub mod bitmap;
use bitmap::{
    Bitmap,
    BitmapHandle,
};

pub mod brush;
use brush::{
    Brush,
    BrushHandle,
};

pub mod cursor;
use cursor::Cursor;

pub mod device_context;
use device_context::DeviceContext;

pub mod icon;
use icon::Icon;

pub mod menu;
use menu::Menu;

// pub mod monitor;
// use monitor::Monitor;

pub mod render_context;
use render_context::RenderContext;

pub mod window_class;
use window_class::WindowClass;

pub mod window;
use window::Window;

use core::{
    mem::{
        transmute,
        transmute_copy
    },
    ptr::NonNull,
};

use winapi::{
    shared::minwindef::HINSTANCE,
    um::{
        errhandlingapi::GetLastError,
        wingdi::{
            CLR_INVALID,
        },
    },
};

#[derive(Clone,Copy)]
#[repr(transparent)]
pub struct InstanceHandle{
    inner:NonNull<HINSTANCE>,
}
implement_handle_wrapper!(InstanceHandle,HINSTANCE);

/// Represents the Windows colour.
/// 
/// Colour is defined as a combination of three primary colors red, green, and blue.
/// The system identifies a colour by giving it a colour value (sometimes called an RGB triplet),
/// which consists of three 8-bit values specifying the intensities of its colour components.
/// Black has the minimum intensity for red, green, and blue, so the colour value for black is (0, 0, 0).
/// White has the maximum intensity for red, green, and blue, so its colour value is (255, 255, 255).
#[derive(Clone,Copy)]
#[repr(transparent)]
pub struct Colour{
    /// red, green, blue, empty
    inner:[u8;4]
}

impl Colour{
    pub const fn new([red,green,blue]:[u8;3])->Colour{
        Self{
            inner:[red,green,blue,0u8]
        }
    }

    pub const fn red(&self)->u8{
        self.inner[0]
    }

    pub const fn green(&self)->u8{
        self.inner[1]
    }

    pub const fn blue(&self)->u8{
        self.inner[2]
    }

    #[inline(always)]
    pub fn as_raw(self)->u32{
        unsafe{
            transmute(self)
        }
    }
}

#[repr(transparent)]
pub struct ColourResult{
    inner:Colour,
}

impl ColourResult{
    pub fn get_colour(self)->Option<Colour>{
        if self.inner.as_raw()==CLR_INVALID{
            None
        }
        else{
            Some(self.inner)
        }
    }

    #[inline(always)]
    pub fn get_colour_unchecked(self)->Colour{
        self.inner
    }
}

pub struct WindowsCore{
    pub bitmap:Bitmap,
    pub brush:Brush,
    pub cursor:Cursor,
    pub device_context:DeviceContext,
    pub icon:Icon,
    pub menu:Menu,
    pub render_context:RenderContext,
    pub window_class:WindowClass,
    pub window:Window,
}

impl WindowsCore{
    pub const fn new()->WindowsCore{
        Self{
            bitmap:Bitmap::new(),
            brush:Brush::new(),
            cursor:Cursor::new(),
            device_context:DeviceContext::new(),
            icon:Icon::new(),
            menu:Menu::new(),
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