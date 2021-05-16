use gl::{
    // constants
    NO_ERROR,
    INVALID_ENUM,
    INVALID_VALUE,
    INVALID_OPERATION,
    INVALID_FRAMEBUFFER_OPERATION,
    OUT_OF_MEMORY,

    // functions
    GetError,
};

use std::mem::transmute;

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum GLError{
    NoError=NO_ERROR,
    InvalidEnum=INVALID_ENUM,
    InvalidValue=INVALID_VALUE,
    InvalidOperation=INVALID_OPERATION,
    InvalidFramebufferOperation=INVALID_FRAMEBUFFER_OPERATION,
    OutOfMemory=OUT_OF_MEMORY,
}

impl GLError{
    pub (crate) fn get_error()->GLError{
        unsafe{
            transmute(GetError())
        }
    }
}