use crate::graphics::{
    GLCore,
    core::GLError,
};

use core::mem::transmute;

const VIEWPORT:u32=0x0BA2;
const MAX_VIEWPORT_DIMS:u32=0x0D3A;

pub struct Viewport{}

impl Viewport{
    pub const fn new()->Viewport{
        Self{

        }
    }
}

impl Viewport{
    /// Sets the viewport.
    /// 
    /// Specifies the affine transformation of `x` and `y` from normalized device coordinates to window coordinates.
    /// 
    /// Let (Xnd, Ynd) be normalized device coordinates. Then the window coordinates (Xw, Yw) are computed as follows:
    /// ```
    ///     Xw = (Xnd + 1)(width / 2) + x
    ///     Yw = (Ynd + 1)(height / 2) + y
    /// ```
    /// 
    /// `x`, `y` - Specify the lower left corner of the viewport rectangle, in pixels.
    /// The initial value is `(0i32, 0i32)`.
    /// 
    /// `widht`, `height` - Specify the width and height of the viewport.
    /// When a GL context is first attached to a window,
    /// `width` and `height` are set to the dimensions of that window.
    /// 
    /// Viewport width and height are silently clamped to a range that depends on the implementation.
    /// To query this range, call `Viewport::get_max_dimensions` or `Viewport::write_max_dimensions`.
    /// 
    /// `GLError::InvalidValue` is generated if either `width` or `height` is negative.
    #[inline(always)]
    pub fn set(&self,[x,y,widht,height]:[i32;4])->GLError{
        unsafe{
            GLCore.parameters.viewport.set([x,y,widht,height]);
            GLCore.get_error()
        }
    }

    #[inline(always)]
    pub fn get(&self)->[i32;4]{
        unsafe{
            let mut viewport=[0i32;4];
            GLCore.get_integer_v(VIEWPORT,viewport.get_unchecked_mut(0));
            viewport
        }
    }

    #[inline(always)]
    pub fn write(&self,viewport:&mut [i32;4]){
        unsafe{
            GLCore.get_integer_v(VIEWPORT,viewport.get_unchecked_mut(0));
        }
    }

    #[inline(always)]
    pub fn get_max_dimensions(&self)->[i32;2]{
        unsafe{
            let mut dimensions=[0i32;2];
            GLCore.get_integer_v(MAX_VIEWPORT_DIMS,dimensions.get_unchecked_mut(0));
            dimensions
        }
    }

    #[inline(always)]
    pub fn write_max_dimensions(&self,dimensions:&mut [i32;2]){
        unsafe{
            GLCore.get_integer_v(MAX_VIEWPORT_DIMS,dimensions.get_unchecked_mut(0));
        }
    }
}