use crate::windows::{
    WinCore,
    core::icon::IconHandle,
};

use winapi::{
    shared::{
        windef::{
            HICON,
        }
    },

    um::{
        winuser::{
            CreateIconIndirect,
            DestroyIcon,
            ICONINFO,
        },
    }
};

use image::{
    ImageBuffer,
    Bgra
};

pub struct Pixel{
    red:u8,
    green:u8,
    blue:u8,
    alpha:u8
}

pub struct Icon{
    handle:IconHandle,
}

impl Icon{
    pub fn from_bgra(icon:bool,position:[u32;2],image:&ImageBuffer<Bgra<u8>,Vec<u8>>)->Icon{
        unsafe{
            let (width,height)=image.dimensions();
            let pixel_count=image.len()/4;
            let mut and_mask=Vec::with_capacity(pixel_count);
            let pixels=std::slice::from_raw_parts_mut(image.as_ptr() as *mut Pixel,pixel_count);
            for pixel in pixels{
                and_mask.push(pixel.alpha.wrapping_sub(std::u8::MAX)); // invert alpha channel
            }

            let mut iconinfo=ICONINFO{
                fIcon:if icon{1i32}else{0i32},
                xHotspot:position[0],
                yHotspot:position[1],
                hbmMask:WinCore.bitmap.create(
                    [width as i32,height as i32],
                    1,
                    4,
                    Some(&*and_mask.as_ptr())
                ).unwrap().as_raw(),
                hbmColor:WinCore.bitmap.create(
                    [width as i32,height as i32],
                    1,
                    32,
                    Some(&*image.as_ptr())
                ).unwrap().as_raw(),
            };

            Self{
                handle:IconHandle::from_raw(CreateIconIndirect(&mut iconinfo)).unwrap()
            }
        }
    }

    #[inline(always)]
    pub fn handle(&self)->IconHandle{
        self.handle
    }

    #[inline(always)]
    pub fn destroy(self){
        unsafe{
            DestroyIcon(self.handle.as_raw());
        }
    }
}