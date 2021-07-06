use winapi::{
    shared::windef::HBITMAP,

    um::wingdi::{
        CreateBitmap,
        DeleteObject,
    },
};

use image::{
    ImageBuffer,
    Bgra,
};

use std::{
    mem::transmute,
};

pub struct Bitmap{
    handle:HBITMAP,
}

impl Bitmap{
    /// A BGRA8 image.
    pub fn raw(width:i32,height:i32,data:&[u8])->Bitmap{
        unsafe{
            Self{
                handle:CreateBitmap(width as i32,height as i32,1,32,data.as_ptr() as *mut _),
            }
        }
    }

    pub fn from_bgra(image:&ImageBuffer<Bgra<u8>,Vec<u8>>)->Bitmap{
        unsafe{
            let (width,height)=image.dimensions();

            Self{
                handle:CreateBitmap(width as i32,height as i32,1,32,image.as_ptr() as *mut _),
            }
        }
    }

    #[inline(always)]
    pub fn handle(&self)->HBITMAP{
        self.handle
    }

    #[inline(always)]
    pub fn destroy(self){
        unsafe{
            let _=DeleteObject(transmute(self.handle));
        }
    }
}