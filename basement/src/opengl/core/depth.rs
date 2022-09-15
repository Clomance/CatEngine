#[cfg(target_os="windows")]
use crate::winapi::OpenGraphicsLibrary;

use super::{
    types::*,
    constants::*
};

use core::mem::transmute;

/// Specifies the conditions under which the pixel will be drawn.
#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum DepthFunction{
    /// Never passes.
    Never=NEVER,

    /// Passes if the incoming depth value is less than the stored depth value.
    Less=LESS,

    /// Passes if the incoming depth value is equal to the stored depth value.
    Equal=EQUAL,

    /// Passes if the incoming depth value is less than or equal to the stored depth value.
    LessEqual=LEQUAL,

    /// Passes if the incoming depth value is greater than the stored depth value.
    Greater=GREATER,

    /// Passes if the incoming depth value is greater than or equal to the stored depth value.
    GreaterEqual=GEQUAL,

    /// Passes if the incoming depth value is not equal to the stored depth value.
    NotEqual=NOTEQUAL,

    /// Always passes.
    Always=ALWAYS
}

#[cfg(target_os="linux")]
extern "system"{
    fn glDepthFunc(function:GLenum);
    fn glDepthMask(flag:GLboolean);
    fn glDepthRange(near:GLclampd,far:GLclampd);
}

#[cfg(target_os="windows")]
mod gl{
    pub static mut glDepthFunc:usize=0;
    pub static mut glDepthMask:usize=0;
    pub static mut glDepthRange:usize=0;
}

#[cfg(target_os="windows")]
mod gl_functions{
    use super::*;

    pub unsafe fn glDepthFunc(function:GLenum){
        transmute::<usize,extern "system" fn(GLenum)>(gl::glDepthFunc)(function)
    }
    pub unsafe fn glDepthMask(flag:GLboolean){
        transmute::<usize,extern "system" fn(GLboolean)>(gl::glDepthMask)(flag)
    }
    pub unsafe fn glDepthRange(near:GLclampd,far:GLclampd){
        transmute::<usize,extern "system" fn(GLclampd,GLclampd)>(gl::glDepthRange)(near,far)
    }
}

#[cfg(target_os="windows")]
use gl_functions::*;

pub struct Depth;

impl Depth{
    #[cfg(target_os="windows")]
    pub fn load(library:&OpenGraphicsLibrary){
        unsafe{
            use gl::*;

            glDepthFunc=transmute(library.get_proc_address("glDepthFunc\0"));
            glDepthMask=transmute(library.get_proc_address("glDepthMask\0"));
            glDepthRange=transmute(library.get_proc_address("glDepthRange\0"));
        }
    }
}

impl Depth{
    /// Specifies the value used for depth buffer comparisons.
    /// 
    /// Specifies the function used to compare each incoming pixel depth value with the depth value present in the depth buffer.
    /// The comparison is performed only if depth testing is enabled (See `Core::enable` and `Core::disable` of `DEPTH_TEST`).
    /// 
    /// The initial value of `function` is `DepthFunction::Less`.
    /// Initially, depth testing is disabled.
    /// If depth testing is disabled or if no depth buffer exists,
    /// it is as if the depth test always passes.
    /// 
    /// Even if the depth buffer exists and the depth mask is non-zero,
    /// the depth buffer is not updated if the depth test is disabled.
    /// In order to unconditionally write to the depth buffer,
    /// the depth test should be enabled and set to `DepthFunction::Always`.
    /// 
    /// `Error::InvalidEnum` is generated if `function` is not an accepted value.
    pub unsafe fn set_function(function:DepthFunction){
        glDepthFunc(function as GLenum)
    }

    /// Enables or disables writing into the depth buffer.
    /// 
    /// Specifies whether the depth buffer is enabled for writing.
    /// If `flag` is `false`, depth buffer writing is disabled.
    /// Otherwise, it is enabled.
    /// Initially, depth buffer writing is enabled.
    /// 
    /// Even if the depth buffer exists and the depth mask is non-zero,
    /// the depth buffer is not updated if the depth test is disabled.
    /// In order to unconditionally write to the depth buffer,
    /// the depth test should be enabled and set to `DepthFunction::Always`.
    pub unsafe fn set_mask(flag:bool){
        glDepthMask(flag as GLboolean)
    }

    /// Specifies mapping of depth values from normalized device coordinates to window coordinates.
    /// 
    /// `near` specifies the mapping of the near clipping plane to window coordinates.
    /// The initial value is `0`.
    /// 
    /// `far` specifies the mapping of the far clipping plane to window coordinates.
    /// The initial value is `1`.
    /// 
    /// After clipping and division by `w`, depth coordinates range from −1 to 1, corresponding to the near and far clipping planes.
    /// `Depth::set_range` specifies a linear mapping of the normalized depth coordinates in this range to window depth coordinates.
    /// Regardless of the actual depth buffer implementation, window coordinate depth values are treated as though they range from 0 through 1 (like color components).
    /// Thus, the values accepted by `Depth::set_range` are both clamped to this range before they are accepted.
    /// 
    /// The setting of (0,1) maps the near plane to 0 and the far plane to 1.
    /// With this mapping, the depth buffer range is fully utilized.
    /// 
    /// It is not necessary that `near` be less than `far`.
    /// Reverse mappings such as `near` = `1`, and `far` = `0` are acceptable.
    pub unsafe fn set_range(near:f64,far:f64){
        glDepthRange(near,far)
    }
}