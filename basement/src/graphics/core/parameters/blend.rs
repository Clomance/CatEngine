#[cfg(target_os="windows")]
use crate::windows::OpenGraphicsLibrary;

use crate::graphics::Colour;

use core::mem::transmute;

// Blend functions
const ZERO:u32=0;
const ONE:u32=1;
const SRC_COLOR:u32=0x0300;
const ONE_MINUS_SRC_COLOR:u32=0x0301;
const SRC_ALPHA:u32=0x0302;
const ONE_MINUS_SRC_ALPHA:u32=0x0303;
const DST_ALPHA:u32=0x0304;
const ONE_MINUS_DST_ALPHA:u32=0x0305;
const DST_COLOR:u32=0x0306;
const ONE_MINUS_DST_COLOR:u32=0x0307;
const SRC_ALPHA_SATURATE:u32=0x0308;
const CONSTANT_COLOR:u32=0x8001;
const ONE_MINUS_CONSTANT_COLOR:u32=0x8002;
const CONSTANT_ALPHA:u32=0x8003;
const ONE_MINUS_CONSTANT_ALPHA:u32=0x8004;
const SRC1_ALPHA:u32=0x8589;
const SRC1_COLOR:u32=0x88F9;
const ONE_MINUS_SRC1_COLOR:u32=0x88FA;
const ONE_MINUS_SRC1_ALPHA:u32=0x88FB;

// Blend equation
const FUNC_ADD:u32=0x8006;
const FUNC_SUBTRACT:u32=0x800A;
const FUNC_REVERSE_SUBTRACT:u32=0x800B;
const MIN:u32=0x8007;
const MAX:u32=0x8008;

// Blend parameters
const BLEND_COLOR:u32=0x8005;
const BLEND_DST_ALPHA:u32=0x80CA;
const BLEND_DST_RGB:u32=0x80C8;
const BLEND_EQUATION:u32=0x8009;
const BLEND_EQUATION_ALPHA:u32=0x883D;
const BLEND_EQUATION_RGB:u32=0x8009;
const BLEND_SRC_ALPHA:u32=0x80CB;
const BLEND_SRC_RGB:u32=0x80C9;

/// The `BlendingEquation::MIN` and `BlendingEquation::MAX` equations
/// do not use the source or destination factors,
/// only the source and destination colors.
#[repr(u32)]
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
#[derive(Clone,Copy,Debug)]
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

#[cfg_attr(windows,link(name="opengl32"))]
extern "system"{
    fn glBlendColor(red:f32,greed:f32,blue:f32,alpha:f32)->();
    fn glBlendFunc(sourse_factor:BlendingFunction,destination_factor:BlendingFunction)->();
    fn glBlendFuncSeparate(
        sourse_factor_rgb:BlendingFunction,
        destination_factor_rgb:BlendingFunction,
        sourse_factor_alpha:BlendingFunction,
        destination_factor_alpha:BlendingFunction
    )->();
    fn glBlendEquation(equation:BlendingEquation)->();
    fn glBlendEquationSeparate(
        equation_rgb:BlendingEquation,
        equation_alpha:BlendingEquation
    )->();
}

mod gl{
    pub static mut glBlendColor:usize=0;
    pub static mut glBlendFunc:usize=0;
    pub static mut glBlendFuncSeparate:usize=0;
    pub static mut glBlendEquation:usize=0;
    pub static mut glBlendEquationSeparate:usize=0;
}

mod gl_function{
    use super::*;

    pub unsafe extern "system" fn glBlendColor(red:f32,greed:f32,blue:f32,alpha:f32){
        transmute::<usize,fn(f32,f32,f32,f32)>(gl::glBlendColor)(red,greed,blue,alpha)
    }
    pub unsafe extern "system" fn glBlendFunc(sourse_factor:BlendingFunction,destination_factor:BlendingFunction){
        transmute::<usize,fn(
            BlendingFunction,
            BlendingFunction
        )>(gl::glBlendFunc)
        (
            sourse_factor,
            destination_factor
        )
    }
    pub unsafe extern "system" fn glBlendFuncSeparate(
        sourse_factor_rgb:BlendingFunction,
        destination_factor_rgb:BlendingFunction,
        sourse_factor_alpha:BlendingFunction,
        destination_factor_alpha:BlendingFunction
    ){
        transmute::<usize,fn(
            BlendingFunction,
            BlendingFunction,
            BlendingFunction,
            BlendingFunction
        )>(gl::glBlendFuncSeparate)(
            sourse_factor_rgb,
            destination_factor_rgb,
            sourse_factor_alpha,
            destination_factor_alpha
        )
    }
    pub unsafe extern "system" fn glBlendEquation(equation:BlendingEquation){
        transmute::<usize,fn(BlendingEquation)>(gl::glBlendEquation)(equation)
    }
    pub unsafe extern "system" fn glBlendEquationSeparate(
        equation_rgb:BlendingEquation,
        equation_alpha:BlendingEquation
    ){
        transmute::<usize,fn(
            BlendingEquation,
            BlendingEquation
        )>(gl::glBlendEquationSeparate)(
            equation_rgb,
            equation_alpha
        )
    }
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
    pub const fn new()->Blend{
        Self
    }

    #[cfg(target_os="windows")]
    pub fn load(&mut self,library:&OpenGraphicsLibrary){
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
    /// Sets the blend constant colour.
    /// 
    /// The colour components are clamped to the range \[0,1\] before being stored.
    /// 
    /// Initially the colour is set to `[0f32;4]`.
    #[inline(always)]
    pub fn set_blending_colour(&self,[red,greed,blue,alpha]:Colour){
        unsafe{
            glBlendColor(red,greed,blue,alpha)
        }
    }

    /// Specify pixel arithmetic.
    /// 
    /// Specifies how the red, green, blue, and alpha source blending factors are computed.
    /// 
    /// The initial value is `BlendingFunction::One`.
    /// 
    /// Pixels can be drawn using a function
    /// that blends the incoming (source) RGBA values with the RGBA values
    /// that are already in the frame buffer (the destination values).
    /// Blending is initially disabled.
    #[inline(always)]
    pub fn set_function(
        &self,
        sourse_factor:BlendingFunction,
        destination_factor:BlendingFunction
    ){
        unsafe{
            glBlendFunc(sourse_factor,destination_factor)
        }
    }

    /// Sets the blending functions for the RBG and Alpha colour components separately.
    #[inline(always)]
    pub fn set_function_separate(
        &self,
        sourse_factor_rgb:BlendingFunction,
        destination_factor_rgb:BlendingFunction,
        sourse_factor_alpha:BlendingFunction,
        destination_factor_alpha:BlendingFunction,
    ){
        unsafe{
            glBlendFuncSeparate(
                sourse_factor_rgb,
                destination_factor_rgb,
                sourse_factor_alpha,
                destination_factor_alpha
            )
        }
    }

    /// Sets the equation used for both the RGB blending equation and the Alpha blend equation.
    #[inline(always)]
    pub fn set_equation(&self,equation:BlendingEquation){
        unsafe{
            glBlendEquation(equation)
        }
    }

    /// Sets the equation for the RGB blending equation and the Alpha blend equation sepatately.
    #[inline(always)]
    pub fn set_equation_separate(
        &self,
        equation_rgb:BlendingEquation,
        equation_alpha:BlendingEquation
    ){
        unsafe{
            glBlendEquationSeparate(
                equation_rgb,
                equation_alpha
            )
        }
    }
}