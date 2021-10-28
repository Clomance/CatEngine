#[cfg(any(windows))]
use crate::windows::OpenGraphicsLibrary;

use core::mem::transmute;

pub struct Viewport{
    glViewport:usize,
}

impl Viewport{
    pub const fn new()->Viewport{
        Self{
            glViewport:0,
        }
    }

    #[cfg(any(windows))]
    pub fn load(&mut self,library:&OpenGraphicsLibrary){
        unsafe{
            self.glViewport=transmute(library.get_proc_address("glViewport\0"))
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
    /// To query this range, call `glGet` with argument `GL_MAX_VIEWPORT_DIMS`.
    /// 
    /// `GLError::InvalidValue` is generated if either `width` or `height` is negative.
    #[inline(always)]
    pub unsafe fn set(&self,[x,y,widht,height]:[i32;4]){
        transmute::<usize,fn(i32,i32,i32,i32)>(self.glViewport)(x,y,widht,height)
    }
}