#[cfg(all(target_os="windows",feature="windows"))]
use crate::windows::OpenGraphicsLibrary;

use crate::graphics::core::types::*;

use core::mem::transmute;

// #[cfg_attr(windows,link(name="opengl32"))]
#[cfg(all(target_os="linux",feature="linux"))]
extern "system"{
    fn glViewport(x:GLint,y:GLint,width:GLint,height:GLint)->();
}

#[cfg(all(target_os="windows",feature="windows"))]
mod gl{
    pub static mut glViewport:usize=0;
}

#[cfg(all(target_os="windows",feature="windows"))]
mod gl_functions{
    use super::*;

    pub unsafe extern "system" fn glViewport(x:GLint,y:GLint,width:GLint,height:GLint){
        transmute::<usize,fn(GLint,GLint,GLint,GLint)>(gl::glViewport)(
            x,y,width,height
        )
    }
}

#[cfg(all(target_os="windows",feature="windows"))]
use gl_functions::*;

pub struct Viewport;

impl Viewport{
    pub const fn new()->Viewport{
        Self
    }

    #[cfg(all(target_os="windows",feature="windows"))]
    pub fn load(&mut self,library:&OpenGraphicsLibrary){
        unsafe{
            use gl::*;
            glViewport=transmute(library.get_proc_address("glViewport\0"));
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
    pub unsafe fn set(&self,[x,y,width,height]:[i32;4]){
        glViewport(x,y,width,height);
    }
}