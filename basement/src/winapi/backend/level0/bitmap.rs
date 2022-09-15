use crate::winapi::{
    WinCore,
    core::{
        Colour,
        brush::{
            
        },
    },
};

pub use crate::winapi::core::bitmap::{
    BitmapHandle,
};

use super::error::Error;

#[repr(transparent)]
pub struct Bitmap{
    inner:BitmapHandle,
}

impl Bitmap{
    pub const fn handle(&self)->BitmapHandle{
        self.inner
    }
}