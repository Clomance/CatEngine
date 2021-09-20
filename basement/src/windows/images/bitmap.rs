use crate::windows::{
    WinCore,
    core::bitmap::BitmapHandle,
};

use std::{
    mem::transmute,
};

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



pub struct Bitmap{
    handle:BitmapHandle,
}

impl Bitmap{
    /// A BGRA8 image.
    pub fn raw(size:[i32;2],data:&[u8])->Bitmap{
        unsafe{
            Self{
                handle:WinCore.bitmap.create(size,1,32,Some(&data[0])).unwrap(),
            }
        }
    }

    pub fn from_bgra(image:&ImageBuffer<Bgra<u8>,Vec<u8>>)->Bitmap{
        unsafe{
            let (width,height)=image.dimensions();
            let data=&*image.as_ptr();

            Self{
                handle:WinCore.bitmap.create([width as i32,height as i32],1,32,Some(data)).unwrap(),
            }
        }
    }

    #[inline(always)]
    pub fn handle(&self)->BitmapHandle{
        self.handle
    }

    #[inline(always)]
    pub fn destroy(self)->bool{
        unsafe{
            WinCore.bitmap.destroy(self.handle)
        }
    }
}