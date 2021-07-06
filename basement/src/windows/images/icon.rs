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

        wingdi::{
            CreateBitmap,
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
    handle:HICON,
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
                hbmMask:CreateBitmap(width as i32,height as i32,1,4,and_mask.as_ptr() as *mut _),
                hbmColor:CreateBitmap(width as i32,height as i32,1,32,image.as_ptr() as *mut _),
            };

            Self{
                handle:CreateIconIndirect(&mut iconinfo),
            }
        }
    }

    #[inline(always)]
    pub fn handle(&self)->HICON{
        self.handle
    }

    #[inline(always)]
    pub fn destroy(self){
        unsafe{
            DestroyIcon(self.handle);
        }
    }
}