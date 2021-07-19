use crate::{
    windows::OpenGraphicsLibrary,
    graphics::GCore,
};

use core::mem::transmute;

const VIEWPORT:u32=0x0BA2;
const MAX_VIEWPORT_DIMS:u32=0x0D3A;

pub struct Viewport{
    glViewport:usize,
}

impl Viewport{
    pub const fn new()->Viewport{
        Self{
            glViewport:0,
        }
    }

    pub fn load(&mut self,library:&OpenGraphicsLibrary){
        unsafe{
            self.glViewport=transmute(library.get_proc_address("glViewport\0"))
        }
    }
}

impl Viewport{
    #[inline(always)]
    pub unsafe fn set(&self,[x,y,widht,height]:[i32;4]){
        transmute::<usize,fn(i32,i32,i32,i32)>(self.glViewport)(x,y,widht,height)
    }
}

impl Viewport{
    #[inline(always)]
    pub fn get(&self)->[i32;4]{
        unsafe{
            let mut viewport=[0i32;4];
            GCore.get_integer_v(VIEWPORT,viewport.get_unchecked_mut(0));
            viewport
        }
    }

    #[inline(always)]
    pub fn write(&self,viewport:&mut [i32;4]){
        unsafe{
            GCore.get_integer_v(VIEWPORT,viewport.get_unchecked_mut(0));
        }
    }

    #[inline(always)]
    pub fn get_max_dimensions(&self)->[i32;2]{
        unsafe{
            let mut dimensions=[0i32;2];
            GCore.get_integer_v(MAX_VIEWPORT_DIMS,dimensions.get_unchecked_mut(0));
            dimensions
        }
    }

    #[inline(always)]
    pub fn write_max_dimensions(&self,dimensions:&mut [i32;2]){
        unsafe{
            GCore.get_integer_v(MAX_VIEWPORT_DIMS,dimensions.get_unchecked_mut(0));
        }
    }
}