use crate::windows::{
    WinCore,
    WinError,
    core::bitmap::BitmapHandle,
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

    pub fn from_bgra(image:&ImageBuffer<Bgra<u8>,Vec<u8>>)->Result<Bitmap,WinError>{
        unsafe{
            let (width,height)=image.dimensions();
            let data=&*image.as_ptr();

            if let Some(bitmap)=WinCore.bitmap.create([width as i32,height as i32],1,32,Some(data)){
                Ok(
                    Self{
                        handle:bitmap,
                    }
                )
            }
            else{
                Err(
                    WinError::get_last_error()
                )
            }
        }
    }

    pub const fn handle(&self)->BitmapHandle{
        self.handle
    }

    #[inline(always)]
    pub fn destroy(self)->bool{
        unsafe{
            WinCore.bitmap.destroy(self.handle)
        }
    }
}