mod gl_core;
pub use gl_core::{
    GraphicsCore,
    Blending,
    BlendingEquation,
    BlendingFunction,
    Viewport,
    Drawing,
    IndexType,
    PrimitiveType,
};

pub mod level0;

pub mod level1;

pub mod level2;

pub use gl;

pub type ColourComponent=f32;
pub type Colour=[ColourComponent;4];

use gl::{
    // Errors
    NO_ERROR,
    INVALID_ENUM,
    INVALID_VALUE,
    INVALID_OPERATION,
    INVALID_FRAMEBUFFER_OPERATION,
    OUT_OF_MEMORY,

    // functions
    GetError
};

use std::mem::transmute;

#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum GLError{
    NoError=NO_ERROR,
    InvalidEnum=INVALID_ENUM,
    InvalidValue=INVALID_VALUE,
    InvalidOperation=INVALID_OPERATION,
    InvalidFramebufferOperation=INVALID_FRAMEBUFFER_OPERATION,
    OutOfMemory=OUT_OF_MEMORY,
}

impl GLError{
    /// Returns a error.
    pub fn get_error()->GLError{
        unsafe{
            transmute(GetError())
        }
    }
}