use crate::winapi::{
    Error,
    backend::core::bitmap::{
        BitmapHandle,
        Bitmap as BitmapFunctions,
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
                handle:BitmapFunctions::create(size,1,32,Some(&data[0])).unwrap(),
            }
        }
    }

    pub fn from_bgra(image:&ImageBuffer<Bgra<u8>,Vec<u8>>)->Result<Bitmap,Error>{
        unsafe{
            let (width,height)=image.dimensions();
            let data=&*image.as_ptr();

            if let Some(bitmap)=BitmapFunctions::create([width as i32,height as i32],1,32,Some(data)){
                Ok(
                    Self{
                        handle:bitmap,
                    }
                )
            }
            else{
                Err(
                    Error::get_last_error()
                )
            }
        }
    }

    pub const fn handle(&self)->BitmapHandle{
        self.handle
    }

    #[inline(always)]
    pub fn destroy(self)->bool{
        BitmapFunctions::destroy(self.handle)
    }
}