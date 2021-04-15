use crate::Colour;

use cat_engine_basement::graphics::gl::{
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
    // functions
    Enable,
    Disable,
    BlendFunc,
    BlendFuncSeparate,
    BlendColor,
    BlendEquation,
};

pub enum BlendingEquation{
    /// The default, adds both colours to each other.
    /// 
    /// Result = source + destination.
    Addition=FUNC_ADD as isize,

    /// Subtracts both colours from each other.
    /// 
    /// Result = source - destination.
    Subtraction=FUNC_SUBTRACT as isize,

    /// Subtracts both colours, but reverses order.
    /// 
    /// Result = destination - source.
    ReverseSubtraction=FUNC_REVERSE_SUBTRACT as isize,

    /// Takes the component-wise minimum of both colours.
    /// 
    /// Result = min(source, destination).
    Minimum=MIN as isize,

    /// Takes the component-wise maximum of both colours.
    /// 
    /// Result = max(source, destination).
    Maximum=MAX as isize
}

impl BlendingEquation{
    pub fn as_gl_enum(self)->u32{
        self as u32
    }
}

#[derive(Clone,Copy,Debug)]
pub enum BlendingFunction{
    /// Multiply the component by zero.
    /// 
    /// Result = Colour * 0.
    Zero=ZERO as isize,
    /// Multiply the component by one.
    /// 
    /// Result = Colour * 1.
    One=ONE as isize,
    /// Multiply the component by its corresponding value in the source.
    /// 
    /// Result = Colour * SourceColour.
    SourceColour=SRC_COLOR as isize,
    /// Equivalent to `1 - SourceColour`.
    /// 
    /// Result = Result = Colour * (1 - SourceColour).
    OneMinusSourceColour=ONE_MINUS_SRC_COLOR as isize,
    /// Multiply the component by its corresponding value in the destination.
    /// 
    /// Result = Colour * DestinationColour
    DestinationColour=DST_COLOR as isize,
    /// Equivalent to `1 - DestinationColour`.
    /// 
    /// Result = Result = Colour * (1 - DestinationColour).
    OneMinusDestinationColour=ONE_MINUS_DST_COLOR as isize,
    /// Multiply the component by the alpha value of the source.
    /// 
    /// Result = Colour * SourceAlpha.
    SourceAlpha=SRC_ALPHA as isize,
    /// Equivalent to `1 - SourceAlpha`.
    /// 
    /// Result = Colour * (1 - SourceAlpha).
    OneMinusSourceAlpha=ONE_MINUS_SRC_ALPHA as isize,
    /// Multiply the component by the alpha value of the destination.
    /// 
    /// Result = Colour * DestinationAlpha.
    DestinationAlpha=DST_ALPHA as isize,
    /// Equivalent to `1 - DestinationAlpha`.
    /// 
    /// Result = Colour * (1 - DestinationAlpha).
    OneMinusDestinationAlpha=ONE_MINUS_DST_ALPHA as isize,
    /// Multiply the component by the corresponding value in the blending constant colour.
    /// 
    /// Result = Colour * ConstantColour.
    ConstantColour=CONSTANT_COLOR as isize,
    /// Equivalent to `1 - ConstantColour`.
    /// 
    /// Result = Colour * (1 - ConstantColour).
    OneMinusConstantColour=ONE_MINUS_CONSTANT_COLOR as isize,
    /// Multiply the component by the alpha value of the blending constant colour.
    /// 
    /// Result = Colour * ConstantAlpha.
    ConstantAlpha=CONSTANT_ALPHA as isize,
    /// Equivalent to `1 - ConstantAlpha`.
    /// 
    /// Result = Colour * (1 - ConstantAlpha).
    OneMinusConstantAlpha=ONE_MINUS_CONSTANT_ALPHA as isize,
    /// Multiply the component by the smallest value of `SourceAlpha` and `1 - DestinationAlpha`.
    /// 
    /// 
    SourceAlphaSaturate=SRC_ALPHA_SATURATE as isize,
    /// Not fully supported yet.
    Source1Colour=SRC1_COLOR as isize,
    /// Not fully supported yet.
    OneMinusSource1Colour=ONE_MINUS_SRC1_COLOR as isize,
    /// Not fully supported yet.
    Source1Alpha=SRC1_ALPHA as isize,
    /// Not fully supported yet.
    OneMinusSourse1Alpha=ONE_MINUS_SRC1_ALPHA as isize,
}

impl BlendingFunction{
    pub fn as_gl_enum(self)->u32{
        self as u32
    }
}

/// A struct for wrapping
/// blending functions.
pub struct Blending;

impl Blending{
    /// Enables blending.
    pub fn enable(&self){
        unsafe{
            Enable(BLEND)
        }
    }

    /// Disables blending.
    pub fn disable(&self){
        unsafe{
            Disable(BLEND);
        }
    }

    /// Sets the blending constant colour.
    pub fn set_blending_colour(&self,[r,g,b,a]:Colour){
        unsafe{
            BlendColor(r,g,b,a)
        }
    }

    /// Sets the blending functions.
    pub fn set_blending_function(&self,sourse_factor:BlendingFunction,destination_factor:BlendingFunction){
        unsafe{
            BlendFunc(sourse_factor.as_gl_enum(),destination_factor.as_gl_enum())
        }
    }

    /// Sets the blending functions for the RBG and Alpha colour components separately.
    pub fn set_blending_function_separate(
        &self,
        sourse_factor_rgb:BlendingFunction,
        destination_factor_rgb:BlendingFunction,
        sourse_factor_alpha:BlendingFunction,
        destination_factor_alpha:BlendingFunction,
    ){
        unsafe{
            BlendFuncSeparate(
                sourse_factor_rgb.as_gl_enum(),
                destination_factor_rgb.as_gl_enum(),
                sourse_factor_alpha.as_gl_enum(),
                destination_factor_alpha.as_gl_enum()
            )
        }
    }

    /// Sets the equation used for both the RGB blend equation and the Alpha blend equation.
    pub fn set_blend_equation(blending_equation:BlendingEquation){
        unsafe{
            BlendEquation(blending_equation.as_gl_enum())
        }
    }
}