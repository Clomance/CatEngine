macro_rules! implement_handle_wrapper{
    ($wrapper:ty,$raw_handle:ty) => {
        impl $wrapper{
            #[inline(always)]
            pub fn raw(raw:isize)->Option<$wrapper>{
                unsafe{
                    transmute(raw)
                }
            }

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

pub mod device_context;

pub mod window;

pub mod bitmap;

pub mod brush;

pub mod cursor;

pub mod icon;

pub mod menu;

pub mod message;

// pub mod monitor;
// use monitor::Monitor;

pub mod render_context;

pub mod timer;

pub mod window_class;

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
pub struct InstanceHandle(NonNull<()>);
implement_handle_wrapper!(InstanceHandle,HINSTANCE);



/// Represents the Windows colour.
/// 
/// Colour is defined as a combination of three primary colors red, green, and blue.
/// The system identifies a colour by giving it a colour value (sometimes called an RGB triplet),
/// which consists of three 8-bit values specifying the intensities of its colour components.
/// Black has the minimum intensity for red, green, and blue, so the colour value for black is (0, 0, 0).
/// White has the maximum intensity for red, green, and blue, so its colour value is (255, 255, 255).
#[derive(Clone,Copy)]
#[repr(C)]
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



/// Wrapper for some winapi functions.
pub struct WindowsCore;

impl WindowsCore{
    pub fn get_last_error()->u32{
        unsafe{
            GetLastError()
        }
    }
}