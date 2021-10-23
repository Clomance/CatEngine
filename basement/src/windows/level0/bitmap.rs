use crate::windows::{
    WinCore,
    core::{
        Colour,
        brush::{
            
        },
    },
};

pub use crate::windows::core::bitmap::{
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