#[cfg(target_os="windows")]
use crate::winapi::OpenGraphicsLibrary;

use crate::opengl::Colour;

use super::{
    types::*,
    constants::*
};

use core::mem::transmute;

#[cfg(target_os="linux")]
extern "system"{
    fn glStencilFunc(function:GLenum,reference:GLint,mask:GLuint);
    fn glStencilFuncSeparate(face:GLenum,function:GLenum,reference:GLint,mask:GLuint);

    fn glStencilMask(mask:GLuint);
    fn glStencilMaskSeparate(face:GLenum,mask:GLuint);

    fn glStencilOp(stencil_fail:GLenum,depth_fail:GLenum,both_pass:GLenum);
    fn glStencilOpSeparate(face:GLenum,stencil_fail:GLenum,depth_fail:GLenum,both_pass:GLenum);
}

#[cfg(target_os="windows")]
mod gl{
    pub static mut glStencilFunc:usize=0;
    pub static mut glStencilFuncSeparate:usize=0;

    pub static mut glStencilMask:usize=0;
    pub static mut glStencilMaskSeparate:usize=0;

    pub static mut glStencilOp:usize=0;
    pub static mut glStencilOpSeparate:usize=0;
}

#[cfg(target_os="windows")]
mod functions{
    use super::*;

    pub unsafe fn glStencilFunc(function:GLenum,reference:GLint,mask:GLuint){
        transmute::<usize,extern "system" fn(GLenum,GLint,GLuint)>(gl::glStencilFunc)(
            function,reference,mask
        )
    }

    pub unsafe fn glStencilFuncSeparate(face:GLenum,function:GLenum,reference:GLint,mask:GLuint){
        transmute::<usize,extern "system" fn(GLenum,GLenum,GLint,GLuint)>(gl::glStencilFuncSeparate)(
            face,function,reference,mask
        )
    }

    pub unsafe fn glStencilMask(mask:GLuint){
        transmute::<usize,extern "system" fn(GLuint)>(gl::glStencilMask)(mask)
    }

    pub unsafe fn glStencilMaskSeparate(face:GLenum,mask:GLuint){
        transmute::<usize,extern "system" fn(GLenum,GLuint)>(gl::glStencilMaskSeparate)(face,mask)
    }

    pub unsafe fn glStencilOp(stencil_fail:GLenum,depth_fail:GLenum,both_pass:GLenum){
        transmute::<usize,extern "system" fn(GLenum,GLenum,GLenum)>(gl::glStencilOp)(
            stencil_fail,depth_fail,both_pass
        )
    }

    pub unsafe fn glStencilOpSeparate(
        face:GLenum,
        stencil_fail:GLenum,
        depth_fail:GLenum,
        both_pass:GLenum
    ){
        transmute::<usize,extern "system" fn(
            GLenum,GLenum,GLenum,GLenum
        )>(gl::glStencilOpSeparate)(
            face,stencil_fail,depth_fail,both_pass
        )
    }
}

#[cfg(target_os="windows")]
use functions::*;

pub struct Stencil;

impl Stencil{
    #[cfg(target_os="windows")]
    pub fn load(library:&OpenGraphicsLibrary){
        unsafe{
            use gl::*;

            glStencilFunc=transmute(library.get_proc_address("glStencilFunc\0"));
            glStencilFuncSeparate=transmute(library.get_proc_address("glStencilFuncSeparate\0"));

            glStencilMask=transmute(library.get_proc_address("glStencilMask\0"));
            glStencilMaskSeparate=transmute(library.get_proc_address("glStencilMaskSeparate\0"));

            glStencilOp=transmute(library.get_proc_address("glStencilOp\0"));
            glStencilOpSeparate=transmute(library.get_proc_address("glStencilOpSeparate\0"));
        }
    }
}

#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum StencilFunction{
    /// Always fails.
    Never=NEVER,

    /// Passes if ( `reference` & `mask` ) < ( `stencil` & `mask` ).
    Less=LESS,

    /// Passes if ( `reference` & `mask` ) <= ( `stencil` & `mask` ).
    LessEqual=LEQUAL,

    /// Passes if ( `reference` & `mask` ) > ( `stencil` & `mask` ).
    Greater=GREATER,

    /// Passes if ( `reference` & `mask` ) >= ( `stencil` & `mask` ).
    GreaterEqual=GEQUAL,

    /// Passes if ( `reference` & `mask` ) == ( `stencil` & `mask` ).
    Equal=EQUAL,

    /// Passes if ( `reference` & `mask` ) != ( `stencil` & `mask` ).
    NotEqual=NOTEQUAL,

    /// Always passes.
    Always=ALWAYS
}

#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum StencilFace{
    Front=FRONT,
    Back=BACK,
    FrontAndBack=FRONT_AND_BACK
}

#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum StencilAction{
    /// Keeps the current value.
    Keep=KEEP,

    /// Sets the stencil buffer value to 0.
    Zero=ZERO,

    /// Sets the stencil buffer value to `reference`,
    /// as specified by `Stencil::set_function`.
    Replace=REPLACE,

    /// Increments the current stencil buffer value.
    /// Clamps to the maximum representable unsigned value.
    Increment=INCR,

    /// Increments the current stencil buffer value.
    /// Wraps stencil buffer value to zero
    /// when incrementing the maximum representable unsigned value.
    IncrementWrap=INCR_WRAP,

    /// Decrements the current stencil buffer value.
    /// Clamps to 0.
    Decrement=DECR,

    /// Decrements the current stencil buffer value.
    /// Wraps stencil buffer value to the maximum representable unsigned value
    /// when decrementing a stencil buffer value of zero.
    DecrementWrap=DECR_WRAP,

    /// Bitwise inverts the current stencil buffer value.
    Invert=INVERT,
}

impl Stencil{
    /// Sets front and back function and reference value for stencil testing.
    /// 
    /// `function` specifies the test function.
    /// The initial value is `StencilFunction::Always`.
    /// 
    /// `reference` specifies the reference value for the stencil test.
    /// `reference` is clamped to the range `[0,2^n−1]`,
    /// where `n` is the number of bitplanes in the stencil buffer.
    /// The initial value is 0.
    /// 
    /// `mask` specifies a mask that is ANDed with both the reference value
    /// and the stored stencil value when the test is done.
    /// The initial value is all `1`'s.
    /// 
    /// Stenciling, like depth-buffering, enables and disables drawing on a per-pixel basis.
    /// Stencil planes are first drawn into using GL drawing primitives,
    /// then geometry and images are rendered using the stencil planes to mask out portions of the screen.
    /// Stenciling is typically used in multipass rendering algorithms to achieve special effects,
    /// such as decals, outlining, and constructive solid geometry rendering.
    /// 
    /// The stencil test conditionally eliminates a pixel based on the outcome of a comparison
    /// between the reference value and the value in the stencil buffer.
    /// To enable and disable the test, call `Core::enable` and `Core::disable` with argument `STENCIL_TEST`.
    /// To specify actions based on the outcome of the stencil test,
    /// call `Stencil::set_action` or `Stencil::set_action_separate`.
    /// 
    /// There can be two separate sets of `function`, `reference`, and `mask` parameters;
    /// one affects back-facing polygons, and the other affects front-facing polygons as well as other non-polygon primitives.
    /// `Stencil::set_function` sets both front and back stencil state to the same values.
    /// Use `Stencil::set_function_separate` to set front and back stencil state to different values.
    /// 
    /// `function` is a symbolic constant that determines the stencil comparison function.
    /// It accepts one of eight values, shown in the following list.
    /// `reference` is an integer reference value that is used in the stencil comparison.
    /// It is clamped to the range `[0,2^n−1]`, where `n` is the number of bitplanes in the stencil buffer.
    /// `mask` is bitwise ANDed with both the reference value and the stored stencil value,
    /// with the ANDed values participating in the comparison.
    /// 
    /// If stencil represents the value stored in the corresponding stencil buffer location,
    /// the following list shows the effect of each comparison function that can be specified by `function`.
    /// Only if the comparison succeeds is the pixel passed
    /// through to the next stage in the rasterization process (see `Stencil::set_action`).
    /// All tests treat `stencil` values as unsigned integers in the range `[0,2^n−1]`,
    /// where `n` is the number of bitplanes in the stencil buffer.
    /// 
    /// Initially, the stencil test is disabled.
    /// If there is no stencil buffer,
    /// no stencil modification can occur
    /// and it is as if the stencil test always passes.
    /// 
    /// `Stencil::set_function` is the same as calling `Stencil::set_function_separate` with face set to `StencilFace::FrontAndBack`.
    /// 
    /// `Error::InvalidEnum` is generated if `function` is not one of the eight accepted values.
    #[inline(always)]
    pub fn set_function(function:StencilFunction,reference:i32,mask:u32){
        unsafe{
            glStencilFunc(function as GLenum,reference,mask)
        }
    }

    /// Sets front and/or back function and reference value for stencil testing.
    /// 
    /// `face` specifies whether front and/or back stencil state is updated.
    /// 
    /// `function` specifies the test function.
    /// The initial value is `StencilFunction::Always`.
    /// 
    /// `reference` specifies the reference value for the stencil test.
    /// `reference` is clamped to the range `[0,2^n−1]`,
    /// where `n` is the number of bitplanes in the stencil buffer.
    /// The initial value is `0`.
    /// 
    /// `mask` specifies a mask that is ANDed with both the reference value
    /// and the stored stencil value when the test is done.
    /// The initial value is all `1`'s.
    /// 
    /// Stenciling, like depth-buffering, enables and disables drawing on a per-pixel basis.
    /// Stencil planes are first drawn into using GL drawing primitives,
    /// then geometry and images are rendered using the stencil planes to mask out portions of the screen.
    /// Stenciling is typically used in multipass rendering algorithms to achieve special effects,
    /// such as decals, outlining, and constructive solid geometry rendering.
    /// 
    /// The stencil test conditionally eliminates a pixel based on the outcome of a comparison
    /// between the reference value and the value in the stencil buffer.
    /// To enable and disable the test, call `Core::enable` and `Core::disable` with argument `STENCIL_TEST`.
    /// To specify actions based on the outcome of the stencil test,
    /// call `Stencil::set_action` or `Stencil::set_action_separate`.
    /// 
    /// There can be two separate sets of `function`, `reference`, and `mask` parameters;
    /// one affects back-facing polygons, and the other affects front-facing polygons as well as other non-polygon primitives.
    /// `Stencil::set_function` sets both front and back stencil state to the same values.
    /// Use `Stencil::set_function_separate` to set front and back stencil state to different values.
    /// 
    /// `function` is a symbolic constant that determines the stencil comparison function.
    /// It accepts one of eight values, shown in the following list.
    /// `reference` is an integer reference value that is used in the stencil comparison.
    /// It is clamped to the range `[0,2^n−1]`, where `n` is the number of bitplanes in the stencil buffer.
    /// `mask` is bitwise ANDed with both the reference value and the stored stencil value,
    /// with the ANDed values participating in the comparison.
    /// 
    /// If stencil represents the value stored in the corresponding stencil buffer location,
    /// the following list shows the effect of each comparison function that can be specified by `function`.
    /// Only if the comparison succeeds is the pixel passed
    /// through to the next stage in the rasterization process (see `Stencil::set_action`).
    /// All tests treat `stencil` values as unsigned integers in the range `[0,2^n−1]`,
    /// where `n` is the number of bitplanes in the stencil buffer.
    /// 
    /// Initially, the stencil test is disabled.
    /// If there is no stencil buffer,
    /// no stencil modification can occur
    /// and it is as if the stencil test always passes.
    /// 
    /// `Error::InvalidEnum` is generated
    /// if `function` is not one of the eight accepted values,
    /// if `face` is not one of the accepted tokens.
    #[inline(always)]
    pub fn set_function_separate(face:StencilFace,function:StencilFunction,reference:i32,mask:u32){
        unsafe{
            glStencilFuncSeparate(face as GLenum,function as GLenum,reference,mask)
        }
    }

    /// Controls the writing of individual bits in the stencil planes.
    /// 
    /// `mask` specifies a bit mask to enable and disable writing of individual bits in the stencil planes.
    /// Initially, the mask is all `1`'s.
    /// 
    /// The least significant `n` bits of `mask`, where `n` is the number of bits in the stencil buffer, specify a mask.
    /// Where a 1 appears in the mask, it's possible to write to the corresponding bit in the stencil buffer.
    /// Where a 0 appears, the corresponding bit is write-protected.
    /// Initially, all bits are enabled for writing.
    /// 
    /// There can be two separate `mask` writemasks;
    /// one affects back-facing polygons, and the other affects front-facing polygons as well as other non-polygon primitives.
    /// `Stencil::set_mask` sets both front and back stencil writemasks to the same values.
    /// Use `Stencil::set_mask_separate` to set front and back stencil writemasks to different values.
    /// 
    /// `Stencil::set_mask` is the same as calling `Stencil::set_mask_separate` with `face` set to `StencilFace::FrontAndBack`.
    #[inline(always)]
    pub fn set_mask(mask:u32){
        unsafe{
            glStencilMask(mask)
        }
    }

    /// Controls the front and/or back writing of individual bits in the stencil planes.
    /// 
    /// `face` specifies whether the front and/or back stencil writemask is updated.
    /// 
    /// `mask` specifies a bit mask to enable and disable writing of individual bits in the stencil planes.
    /// Initially, the mask is all `1`'s.
    /// 
    /// The least significant `n` bits of `mask`, where `n` is the number of bits in the stencil buffer, specify a mask.
    /// Where a 1 appears in the mask, it's possible to write to the corresponding bit in the stencil buffer.
    /// Where a 0 appears, the corresponding bit is write-protected.
    /// Initially, all bits are enabled for writing.
    /// 
    /// There can be two separate `mask` writemasks;
    /// one affects back-facing polygons, and the other affects front-facing polygons as well as other non-polygon primitives.
    /// `Stencil::set_mask` sets both front and back stencil writemasks to the same values,
    /// as if `Stencil::set_mask_separate` were called with face set to `StencilFace::FrontAndBack`.
    /// 
    /// `Error::InvalidEnum` is generated if `face` is not one of the accepted tokens.
    #[inline(always)]
    pub fn set_mask_separate(face:StencilFace,mask:GLuint){
        unsafe{
            glStencilMaskSeparate(face as GLenum,mask)
        }
    }

    /// Sets front and back stencil test actions.
    /// 
    /// `stencil_fail` specifies the action to take when the stencil test fails.
    /// 
    /// `depth_fail` specifies the stencil action when the stencil test passes, but the depth test fails.
    /// 
    /// `both_pass` specifies the stencil action when both the stencil test and the depth test pass,
    /// or when the stencil test passes and either there is no depth buffer or depth testing is not enabled.
    /// 
    /// Stenciling, like depth-buffering, enables and disables drawing on a per-pixel basis.
    /// You draw into the stencil planes using GL drawing primitives,
    /// then render geometry and images, using the stencil planes to mask out portions of the screen.
    /// Stenciling is typically used in multipass rendering algorithms to achieve special effects,
    /// such as decals, outlining, and constructive solid geometry rendering.
    /// 
    /// The stencil test conditionally eliminates a pixel based on the outcome of a comparison
    /// between the value in the stencil buffer and a reference value.
    /// To enable and disable the test, call `Core::enable` and `Core::disable` with argument `STENCIL_TEST`;
    /// to control it, call `Stencil::set_function` or `Stencil::set_function_separate`.
    /// 
    /// There can be two separate sets of `stencil_fail`, `depth_fail`, and `both_pass` parameters;
    /// one affects back-facing polygons, and the other affects front-facing polygons as well as other non-polygon primitives.
    /// `Stencil::set_action` sets both front and back stencil state to the same values.
    /// Use `Stencil::set_action_separate` to set front and back stencil state to different values.
    /// 
    /// `Stencil::set_action` takes three arguments that indicate
    /// what happens to the stored stencil value while stenciling is enabled.
    /// If the stencil test fails, no change is made to the pixel's color or depth buffers,
    /// and `stencil_fail` specifies what happens to the stencil buffer contents.
    /// 
    /// Stencil buffer values are treated as unsigned integers.
    /// When incremented and decremented, values are clamped to `0` and `2^n−1`,
    /// where `n` is the value returned by querying `STENCIL_BITS`.
    /// 
    /// The other two arguments to `Stencil::set_action` specify stencil buffer actions
    /// that depend on whether subsequent depth buffer tests succeed (`stencil_fail`)
    /// or fail (`depth_fail`) (see `Depth::set_function`).
    /// The actions are specified using the same eight symbolic constants as `stencil_fail`.
    /// Note that `depth_fail` is ignored when there is no depth buffer, or when the depth buffer is not enabled.
    /// In these cases, `stencil_fail` and `both_pass` specify stencil action
    /// when the stencil test fails and passes, respectively.
    /// 
    /// Initially the stencil test is disabled.
    /// If there is no stencil buffer, no stencil modification can occur
    /// and it is as if the stencil tests always pass, regardless of any call to `Stencil::set_action`.
    /// 
    /// `Stencil::set_action` is the same as calling `Stencil::set_action_separate` with face set to `StencilFace::FrontAndBack`.
    /// 
    /// `Error::InvalidEnum` is generated if `stencil_fail`, `depth_fail`,
    /// or `both_pass` is any value other than the defined constant values.
    #[inline(always)]
    pub fn set_action(
        stencil_fail:StencilAction,
        depth_fail:StencilAction,
        both_pass:StencilAction
    ){
        unsafe{
            glStencilOp(
                stencil_fail as GLenum,
                depth_fail as GLenum,
                both_pass as GLenum
            )
        }
    }

    /// Sets front and back stencil test actions.
    /// 
    /// `stencil_fail` specifies the action to take when the stencil test fails.
    /// 
    /// `depth_fail` specifies the stencil action when the stencil test passes, but the depth test fails.
    /// 
    /// `both_pass` specifies the stencil action when both the stencil test and the depth test pass,
    /// or when the stencil test passes and either there is no depth buffer or depth testing is not enabled.
    /// 
    /// Stenciling, like depth-buffering, enables and disables drawing on a per-pixel basis.
    /// You draw into the stencil planes using GL drawing primitives,
    /// then render geometry and images, using the stencil planes to mask out portions of the screen.
    /// Stenciling is typically used in multipass rendering algorithms to achieve special effects,
    /// such as decals, outlining, and constructive solid geometry rendering.
    /// 
    /// The stencil test conditionally eliminates a pixel based on the outcome of a comparison
    /// between the value in the stencil buffer and a reference value.
    /// To enable and disable the test, call `Core::enable` and `Core::disable` with argument `STENCIL_TEST`;
    /// to control it, call `Stencil::set_function` or `Stencil::set_function_separate`.
    /// 
    /// There can be two separate sets of `stencil_fail`, `depth_fail`, and `both_pass` parameters;
    /// one affects back-facing polygons, and the other affects front-facing polygons as well as other non-polygon primitives.
    /// `Stencil::set_action` sets both front and back stencil state to the same values.
    /// Use `Stencil::set_action_separate` to set front and back stencil state to different values.
    /// 
    /// `Stencil::set_action` takes three arguments that indicate
    /// what happens to the stored stencil value while stenciling is enabled.
    /// If the stencil test fails, no change is made to the pixel's color or depth buffers,
    /// and `stencil_fail` specifies what happens to the stencil buffer contents.
    /// 
    /// Stencil buffer values are treated as unsigned integers.
    /// When incremented and decremented, values are clamped to `0` and `2^n−1`,
    /// where `n` is the value returned by querying `STENCIL_BITS`.
    /// 
    /// The other two arguments to `Stencil::set_action` specify stencil buffer actions
    /// that depend on whether subsequent depth buffer tests succeed (`stencil_fail`)
    /// or fail (`depth_fail`) (see `Depth::set_function`).
    /// The actions are specified using the same eight symbolic constants as `stencil_fail`.
    /// Note that `depth_fail` is ignored when there is no depth buffer, or when the depth buffer is not enabled.
    /// In these cases, `stencil_fail` and `both_pass` specify stencil action
    /// when the stencil test fails and passes, respectively.
    /// 
    /// Initially the stencil test is disabled.
    /// If there is no stencil buffer, no stencil modification can occur
    /// and it is as if the stencil tests always pass, regardless of any call to `Stencil::set_action`.
    /// 
    /// `Error::InvalidEnum` is generated
    /// if `stencil_fail`, `depth_fail`,
    /// or `both_pass` is any value other than the defined constant values,
    /// if `face` is not one of the accepted tokens.
    #[inline(always)]
    pub fn set_action_separate(
        face:StencilFace,
        stencil_fail:StencilAction,
        depth_fail:StencilAction,
        both_pass:StencilAction
    ){
        unsafe{
            glStencilOpSeparate(
                face as GLenum,
                stencil_fail as GLenum,
                depth_fail as GLenum,
                both_pass as GLenum
            )
        }
    }
}