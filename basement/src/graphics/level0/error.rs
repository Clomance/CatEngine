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
pub enum Error{
    NO_ERROR=NO_ERROR,
    INVALID_ENUM=INVALID_ENUM,
    INVALID_VALUE=INVALID_VALUE,
    INVALID_OPERATION=INVALID_OPERATION,
    INVALID_FRAMEBUFFER_OPERATION=INVALID_FRAMEBUFFER_OPERATION,
    OUT_OF_MEMORY=OUT_OF_MEMORY,
}

pub fn get_error()->Error{
    unsafe{
        transmute(GetError())
    }
}