use crate::graphics::Colour;

use gl::{
    // constants
    BLEND,
    ZERO,
    ONE,
    SRC_COLOR,
    ONE_MINUS_SRC_COLOR,
    DST_COLOR,
    ONE_MINUS_DST_COLOR,
    SRC_ALPHA,
    ONE_MINUS_SRC_ALPHA,
    DST_ALPHA,
    ONE_MINUS_DST_ALPHA,
    CONSTANT_COLOR,
    ONE_MINUS_CONSTANT_COLOR,
    CONSTANT_ALPHA,
    ONE_MINUS_CONSTANT_ALPHA,
    SRC_ALPHA_SATURATE,
    SRC1_COLOR,
    ONE_MINUS_SRC1_COLOR,
    SRC1_ALPHA,
    ONE_MINUS_SRC1_ALPHA,

    FUNC_ADD,
    FUNC_SUBTRACT,
    FUNC_REVERSE_SUBTRACT,
    MIN,
    MAX,

    BLEND_COLOR,
    BLEND_SRC_RGB,
    BLEND_SRC_ALPHA,
    BLEND_DST_RGB,
    BLEND_DST_ALPHA,
    BLEND_EQUATION_RGB,
    BLEND_EQUATION_ALPHA,
    // functions
    Enable,
    Disable,
    IsEnabled,
    GetFloatv,
    GetIntegerv,
    BlendFunc,
    BlendFuncSeparate,
    BlendColor,
    BlendEquation,
    BlendEquationSeparate,
};

use std::mem::transmute;

#[repr(u32)]
/// The `BlendingEquation::MIN` and `BlendingEquation::MAX` equations
/// do not use the source or destination factors,
/// only the source and destination colors.
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
    /// Multiply the component by the corresponding value in the blending constant colour.
    /// 
    /// Result = Colour * ConstantColour.
    ConstantColour=CONSTANT_COLOR,
    /// Equivalent to `1 - ConstantColour`.
    /// 
    /// Result = Colour * (1 - ConstantColour).
    OneMinusConstantColour=ONE_MINUS_CONSTANT_COLOR,
    /// Multiply the component by the alpha value of the blending constant colour.
    /// 
    /// Result = Colour * ConstantAlpha.
    ConstantAlpha=CONSTANT_ALPHA,
    /// Equivalent to `1 - ConstantAlpha`.
    /// 
    /// Result = Colour * (1 - ConstantAlpha).
    OneMinusConstantAlpha=ONE_MINUS_CONSTANT_ALPHA,
    /// Multiply the component by the smallest value of `SourceAlpha` and `1 - DestinationAlpha`.
    SourceAlphaSaturate=SRC_ALPHA_SATURATE,
    // /// Not fully supported yet.
    // Source1Colour=SRC1_COLOR,
    // /// Not fully supported yet.
    // OneMinusSource1Colour=ONE_MINUS_SRC1_COLOR,
    // /// Not fully supported yet.
    // Source1Alpha=SRC1_ALPHA,
    // /// Not fully supported yet.
    // OneMinusSourse1Alpha=ONE_MINUS_SRC1_ALPHA,
}

/// A wrapper for blending functions.
/// 
/// Blending is disabled by default.
/// 
/// The default blending constant colour is `[0f32;4]`.
/// 
/// The default blending functions for `Source` are `BlendingFunction::One`.
/// 
/// The default blending functions for `Destination` are `BlendingFunction::Zero`.
/// 
/// The default blending equations are `BlendingEquation::Addition`.
pub struct Blending;

impl Blending{
    /// Enables blending.
    #[inline(always)]
    pub fn enable(&self){
        unsafe{
            Enable(BLEND)
        }
    }

    /// Disables blending.
    #[inline(always)]
    pub fn disable(&self){
        unsafe{
            Disable(BLEND);
        }
    }

    /// Checks whether blending is enabled.
    #[inline(always)]
    pub fn is_enabled(&self)->bool{
        unsafe{
            transmute(IsEnabled(BLEND))
        }
    }
}

impl Blending{
    /// Sets the blending constant colour.
    #[inline(always)]
    pub fn set_blending_colour(&self,[r,g,b,a]:Colour){
        unsafe{
            BlendColor(r,g,b,a)
        }
    }

    /// Returns the blending constant colour.
    #[inline(always)]
    pub fn get_blending_colour(&self)->Colour{
        unsafe{
            let mut colour=[0f32;4];
            GetFloatv(BLEND_COLOR,colour.get_unchecked_mut(0));
            colour
        }
    }

    /// Writes the blending constant colour to `colour`.
    #[inline(always)]
    pub fn write_blending_colour(&self,colour:&mut Colour){
        unsafe{
            GetFloatv(BLEND_COLOR,colour.get_unchecked_mut(0));
        }
    }
}

impl Blending{
    /// Sets the blending functions.
    #[inline(always)]
    pub fn set_function(&self,sourse_factor:BlendingFunction,destination_factor:BlendingFunction){
        unsafe{
            BlendFunc(sourse_factor as u32,destination_factor as u32)
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
            BlendFuncSeparate(
                sourse_factor_rgb as u32,
                destination_factor_rgb as u32,
                sourse_factor_alpha as u32,
                destination_factor_alpha as u32
            )
        }
    }

    /// Returns the source blending function for the RBG colour components.
    #[inline(always)]
    pub fn get_function_src_rgb(&self)->BlendingFunction{
        unsafe{
            let mut function=BlendingFunction::One;
            GetIntegerv(BLEND_SRC_RGB,transmute(&mut function));
            function
        }
    }

    /// Writes the source blending function for the RBG colour components to `function`.
    #[inline(always)]
    pub fn write_function_src_rgb(&self,function:&mut BlendingFunction){
        unsafe{
            GetIntegerv(BLEND_SRC_RGB,transmute(function));
        }
    }

    /// Returns the source blending function for the Alpha colour component.
    #[inline(always)]
    pub fn get_function_src_alpha(&self)->BlendingFunction{
        unsafe{
            let mut function=BlendingFunction::One;
            GetIntegerv(BLEND_SRC_ALPHA,transmute(&mut function));
            function
        }
    }

    /// Writes the souse blending function for the Alpha colour component to the `function`.
    #[inline(always)]
    pub fn write_function_src_alpha(&self,function:&mut BlendingFunction){
        unsafe{
            GetIntegerv(BLEND_SRC_ALPHA,transmute(function));
        }
    }

    /// Returns the destination blending function for the RBG colour components.
    #[inline(always)]
    pub fn get_function_dst_rgb(&self)->BlendingFunction{
        unsafe{
            let mut function=BlendingFunction::Zero;
            GetIntegerv(BLEND_DST_RGB,transmute(&mut function));
            function
        }
    }

    /// Writes the destination blending function for the RBG colour components to `function`.
    #[inline(always)]
    pub fn write_function_dst_rgb(&self,function:&mut BlendingFunction){
        unsafe{
            GetIntegerv(BLEND_DST_RGB,transmute(function));
        }
    }

    /// Returns the destination blending function for the Alpha colour component.
    #[inline(always)]
    pub fn get_function_dst_alpha(&self)->BlendingFunction{
        unsafe{
            let mut function=BlendingFunction::Zero;
            GetIntegerv(BLEND_DST_RGB,transmute(&mut function));
            function
        }
    }

    /// Writes the blending function for the Aplha colour component to `function`.
    #[inline(always)]
    pub fn write_function_dst_alpha(&self,function:&mut BlendingFunction){
        unsafe{
            GetIntegerv(BLEND_DST_RGB,transmute(function));
        }
    }
}

impl Blending{
    /// Sets the equation used for both the RGB blending equation and the Alpha blend equation.
    #[inline(always)]
    pub fn set_equation(&self,equation:BlendingEquation){
        unsafe{
            BlendEquation(equation as u32)
        }
    }

    /// Sets the equation for the RGB blending equation and the Alpha blend equation sepatately.
    #[inline(always)]
    pub fn set_equation_separate(&self,equation_rgb:BlendingEquation,equation_alpha:BlendingEquation){
        unsafe{
            BlendEquationSeparate(equation_rgb as u32,equation_alpha as u32)
        }
    }

    /// Returns the RGB blending equation.
    #[inline(always)]
    pub fn get_equation_rbg(&self)->BlendingEquation{
        unsafe{
            let mut equation=BlendingEquation::Addition;
            GetIntegerv(BLEND_EQUATION_RGB,transmute(&mut equation));
            equation
        }
    }

    /// Writes the RGB blending equation to `equation`.
    #[inline(always)]
    pub fn write_equation_rbg(&self,equation:&mut BlendingEquation){
        unsafe{
            GetIntegerv(BLEND_EQUATION_RGB,transmute(equation));
        }
    }

    /// Returns the Alpha blending equation.
    #[inline(always)]
    pub fn get_equation_alpha(&self)->BlendingEquation{
        unsafe{
            let mut equation=BlendingEquation::Addition;
            GetIntegerv(BLEND_EQUATION_ALPHA,transmute(&mut equation));
            equation
        }
    }

    /// Writes the Alpha blending equation to `equation`.
    #[inline(always)]
    pub fn write_equation_alpha(&self,equation:&mut BlendingEquation){
        unsafe{
            GetIntegerv(BLEND_EQUATION_ALPHA,transmute(equation));
        }
    }
}