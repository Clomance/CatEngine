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
#[derive(Debug)]
pub enum GLError{
    NO_ERROR=NO_ERROR,
    INVALID_ENUM=INVALID_ENUM,
    INVALID_VALUE=INVALID_VALUE,
    INVALID_OPERATION=INVALID_OPERATION,
    INVALID_FRAMEBUFFER_OPERATION=INVALID_FRAMEBUFFER_OPERATION,
    OUT_OF_MEMORY=OUT_OF_MEMORY,
}

impl GLError{
    pub (crate) fn get_error()->GLError{
        unsafe{
            transmute(GetError())
        }
    }
}