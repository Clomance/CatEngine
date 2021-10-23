use crate::windows::{
    WinCore,
    core::{
        Colour,
        brush::{
            
        },
    },
};

pub use crate::windows::core::brush::{
    BrushHandle,
    Hatch,
    ColourFormat,
    BrushLog,
};

use super::{
    bitmap::Bitmap,
    error::Error,
};

#[repr(transparent)]
pub struct Brush{
    handle:BrushHandle
}

impl Brush{
    pub fn create_indirect(brush_log:&BrushLog)->Result<Brush,Error>{
        unsafe{
            if let Some(brush)=WinCore.brush.create_indirect(brush_log){
                Ok(Self{handle:brush})
            }
            else{
                Err(Error::get_last_error())
            }
        }
    }

    // pub fn create_dib_pattern()->Option<Brush>{
    //     WinCore.brush.create_indirect()
    // }

    pub fn create_hatch(hatch:Hatch,colour:Colour)->Result<Brush,Error>{
        unsafe{
            if let Some(brush)=WinCore.brush.create_hatch(hatch,colour){
                Ok(Self{handle:brush})
            }
            else{
                Err(Error::get_last_error())
            }
        }
    }

    pub fn create_pattern(bitmap:&Bitmap)->Result<Brush,Error>{
        unsafe{
            if let Some(brush)=WinCore.brush.create_pattern(bitmap.handle()){
                Ok(Self{handle:brush})
            }
            else{
                Err(Error::get_last_error())
            }
        }
    }

    pub fn create_solid(colour:Colour)->Result<Brush,Error>{
        unsafe{
            if let Some(brush)=WinCore.brush.create_solid(colour){
                Ok(Self{handle:brush})
            }
            else{
                Err(Error::get_last_error())
            }
        }
    }

    pub fn get_system(index:i32)->Result<Brush,Error>{
        unsafe{
            if let Some(brush)=WinCore.brush.get_system(index){
                Ok(Self{handle:brush})
            }
            else{
                Err(Error::get_last_error())
            }
        }
    }

    pub const fn handle(&self)->BrushHandle{
        self.handle
    }
}

impl Drop for Brush{
    fn drop(&mut self){
        unsafe{
            WinCore.brush.destroy(self.handle);
        }
    }
}