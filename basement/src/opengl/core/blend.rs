use crate::opengl::Colour;

#[cfg(target_os="windows")]
use crate::winapi::OpenGraphicsLibrary;

use super::{
    types::*,
    constants::*
};

use core::mem::transmute;

// #[cfg_attr(windows,link(name="opengl32"))]
#[cfg(target_os="linux")]
extern "system"{
    fn glBlendColor(red:GLfloat,greed:GLfloat,blue:GLfloat,alpha:GLfloat);
    fn glBlendFunc(sourse_factor:GLenum,destination_factor:GLenum);
    fn glBlendFuncSeparate(
        sourse_factor_rgb:GLenum,
        destination_factor_rgb:GLenum,
        sourse_factor_alpha:GLenum,
        destination_factor_alpha:GLenum
    );
    fn glBlendEquation(equation:GLenum);
    fn glBlendEquationSeparate(equation_rgb:GLenum,equation_alpha:GLenum);
}

#[cfg(target_os="windows")]
mod gl{
    pub static mut glBlendColor:usize=0;
    pub static mut glBlendFunc:usize=0;
    pub static mut glBlendFuncSeparate:usize=0;
    pub static mut glBlendEquation:usize=0;
    pub static mut glBlendEquationSeparate:usize=0;
}

#[cfg(target_os="windows")]
mod gl_functions{
    use super::*;

    pub unsafe fn glBlendColor(red:GLfloat,greed:GLfloat,blue:GLfloat,alpha:GLfloat){
        transmute::<usize,extern "system" fn(GLfloat,GLfloat,GLfloat,GLfloat)>(gl::glBlendColor)(
            red,greed,blue,alpha
        )
    }
    pub unsafe fn glBlendFunc(sourse_factor:GLenum,destination_factor:GLenum){
        transmute::<usize,extern "system" fn(GLenum,GLenum)>(gl::glBlendFunc)(
            sourse_factor,
            destination_factor
        )
    }
    pub unsafe fn glBlendFuncSeparate(
        sourse_factor_rgb:GLenum,
        destination_factor_rgb:GLenum,
        sourse_factor_alpha:GLenum,
        destination_factor_alpha:GLenum
    ){
        transmute::<usize,extern "system" fn(
            GLenum,
            GLenum,
            GLenum,
            GLenum
        )>(gl::glBlendFuncSeparate)(
            sourse_factor_rgb,
            destination_factor_rgb,
            sourse_factor_alpha,
            destination_factor_alpha
        )
    }
    pub unsafe fn glBlendEquation(equation:GLenum){
        transmute::<usize,extern "system" fn(GLenum)>(gl::glBlendEquation)(equation)
    }
    pub unsafe fn glBlendEquationSeparate(
        equation_rgb:GLenum,
        equation_alpha:GLenum
    ){
        transmute::<usize,extern "system" fn(GLenum,GLenum)>(gl::glBlendEquationSeparate)(
            equation_rgb,
            equation_alpha
        )
    }
}

#[cfg(target_os="windows")]
use gl_functions::*;

/// The `BlendingEquation::MIN` and `BlendingEquation::MAX` equations
/// do not use the source or destination factors,
/// only the source and destination colors.
#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum BlendingEquation{
    /// The default, adds both colours to each other.
    /// 
    /// Result = source + destination.
    Addition=FUNC_ADD,

    /// Subtracts both colours from each other.
    /// 
    /// Result = source - destination.
    Subtraction=FUNC_SUBTRACT,

    /// Subtracts both colours, but reverses order.
    /// 
    /// Result = destination - source.
    ReverseSubtraction=FUNC_REVERSE_SUBTRACT,

    /// Takes the component-wise minimum of both colours.
    /// 
    /// Result = min(source, destination).
    Minimum=MIN,

    /// Takes the component-wise maximum of both colours.
    /// 
    /// Result = max(source, destination).
    Maximum=MAX
}

#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum BlendingFunction{
    /// Multiply the component by zero.
    /// 
    /// Result = Colour * 0.
    Zero=ZERO,

    /// Multiply the component by one.
    /// 
    /// Result = Colour * 1.
    One=ONE,

    /// Multiply the component by its corresponding value in the source.
    /// 
    /// Result = Colour * SourceColour.
    SourceColour=SRC_COLOR,

    /// Equivalent to `1 - SourceColour`.
    /// 
    /// Result = Result = Colour * (1 - SourceColour).
    OneMinusSourceColour=ONE_MINUS_SRC_COLOR,

    /// Multiply the component by its corresponding value in the destination.
    /// 
    /// Result = Colour * DestinationColour
    DestinationColour=DST_COLOR,

    /// Equivalent to `1 - DestinationColour`.
    /// 
    /// Result = Result = Colour * (1 - DestinationColour).
    OneMinusDestinationColour=ONE_MINUS_DST_COLOR,

    /// Multiply the component by the alpha value of the source.
    /// 
    /// Result = Colour * SourceAlpha.
    SourceAlpha=SRC_ALPHA,

    /// Equivalent to `1 - SourceAlpha`.
    /// 
    /// Result = Colour * (1 - SourceAlpha).
    OneMinusSourceAlpha=ONE_MINUS_SRC_ALPHA,

    /// Multiply the component by the alpha value of the destination.
    /// 
    /// Result = Colour * DestinationAlpha.
    DestinationAlpha=DST_ALPHA,

    /// Equivalent to `1 - DestinationAlpha`.
    /// 
    /// Result = Colour * (1 - DestinationAlpha).
    OneMinusDestinationAlpha=ONE_MINUS_DST_ALPHA,

    /// Multiply the component by the corresponding value in the blend constant colour.
    /// 
    /// Result = Colour * ConstantColour.
    ConstantColour=CONSTANT_COLOR,

    /// Equivalent to `1 - ConstantColour`.
    /// 
    /// Result = Colour * (1 - ConstantColour).
    OneMinusConstantColour=ONE_MINUS_CONSTANT_COLOR,

    /// Multiply the component by the alpha value of the blend constant colour.
    /// 
    /// Result = Colour * ConstantAlpha.
    ConstantAlpha=CONSTANT_ALPHA,

    /// Equivalent to `1 - ConstantAlpha`.
    /// 
    /// Result = Colour * (1 - ConstantAlpha).
    OneMinusConstantAlpha=ONE_MINUS_CONSTANT_ALPHA,

    /// Multiply the component by the smallest value of `SourceAlpha` and `1 - DestinationAlpha`.
    SourceAlphaSaturate=SRC_ALPHA_SATURATE,

    // Source1Colour=SRC1_COLOR,
    
    // OneMinusSource1Colour=ONE_MINUS_SRC1_COLOR,
    
    // Source1Alpha=SRC1_ALPHA,
    
    // OneMinusSourse1Alpha=ONE_MINUS_SRC1_ALPHA,
}

/// A wrapper for blending functions.
/// 
/// Blend is disabled by default.
/// 
/// The default blending constant colour is `[0f32;4]`.
/// 
/// The default blending functions for `Source` are `BlendingFunction::One`.
/// 
/// The default blending functions for `Destination` are `BlendingFunction::Zero`.
/// 
/// The default blending equations are `BlendingEquation::Addition`.
pub struct Blend;

impl Blend{
    #[cfg(target_os="windows")]
    pub fn load(library:&OpenGraphicsLibrary){
        unsafe{
            use gl::*;

            glBlendColor=transmute(library.get_proc_address("glBlendColor\0"));
            glBlendFunc=transmute(library.get_proc_address("glBlendFunc\0"));
            glBlendFuncSeparate=transmute(library.get_proc_address("glBlendFuncSeparate\0"));
            glBlendEquation=transmute(library.get_proc_address("glBlendEquation\0"));
            glBlendEquationSeparate=transmute(library.get_proc_address("glBlendEquationSeparate\0"));
        }
    }
}

impl Blend{
    /// Sets the blend color
    #[inline(always)]
    pub fn set_blending_colour([red,greed,blue,alpha]:Colour){
        unsafe{
            glBlendColor(red,greed,blue,alpha)
        }
    }

    #[inline(always)]
    pub fn set_function(sourse_factor:BlendingFunction,destination_factor:BlendingFunction){
        unsafe{
            glBlendFunc(sourse_factor as GLenum,destination_factor as GLenum)
        }
    }

    #[inline(always)]
    pub fn set_function_separate(
        sourse_factor_rgb:BlendingFunction,
        destination_factor_rgb:BlendingFunction,
        sourse_factor_alpha:BlendingFunction,
        destination_factor_alpha:BlendingFunction,
    ){
        unsafe{
            glBlendFuncSeparate(
                sourse_factor_rgb as GLenum,
                destination_factor_rgb as GLenum,
                sourse_factor_alpha as GLenum,
                destination_factor_alpha as GLenum
            )
        }
    }

    #[inline(always)]
    pub fn set_equation(equation:BlendingEquation){
        unsafe{
            glBlendEquation(equation as GLenum)
        }
    }

    #[inline(always)]
    pub fn set_equation_separate(equation_rgb:BlendingEquation,equation_alpha:BlendingEquation){
        unsafe{
            glBlendEquationSeparate(equation_rgb as GLenum,equation_alpha as GLenum)
        }
    }
}