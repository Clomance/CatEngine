use crate::graphics::Colour;

#[cfg(target_os="windows")]
use crate::windows::OpenGraphicsLibrary;

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

    // not supported
    // Source1Colour=SRC1_COLOR,
    // 
    // OneMinusSource1Colour=ONE_MINUS_SRC1_COLOR,
    // 
    // Source1Alpha=SRC1_ALPHA,
    // 
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
pub struct Blend{
    glBlendColor:usize,
    glBlendFunc:usize,
    glBlendFuncSeparate:usize,
    glBlendEquation:usize,
    glBlendEquationSeparate:usize,
}

impl Blend{
    pub const fn new()->Blend{
        Self{
            glBlendColor:0,
            glBlendFunc:0,
            glBlendFuncSeparate:0,
            glBlendEquation:0,
            glBlendEquationSeparate:0,
        }
    }

    #[cfg(target_os="windows")]
    pub fn load(&mut self,library:&OpenGraphicsLibrary){
        unsafe{
            self.glBlendColor=transmute(library.get_proc_address("glBlendColor\0"));
            self.glBlendFunc=transmute(library.get_proc_address("glBlendFunc\0"));
            self.glBlendFuncSeparate=transmute(library.get_proc_address("glBlendFuncSeparate\0"));
            self.glBlendEquation=transmute(library.get_proc_address("glBlendEquation\0"));
            self.glBlendEquationSeparate=transmute(library.get_proc_address("glBlendEquationSeparate\0"));
        }
    }
}

impl Blend{
    /// Enables blending.
    #[inline(always)]
    pub fn enable(&self){
        unsafe{
            GLCore.enable(GLCapability::Blend)
        }
    }

    /// Disables blending.
    #[inline(always)]
    pub fn disable(&self){
        unsafe{
            GLCore.disable(GLCapability::Blend)
        }
    }

    /// Checks whether blending is enabled.
    #[inline(always)]
    pub fn is_enabled(&self)->bool{
        unsafe{
            GLCore.is_enabled(GLCapability::Blend)
        }
    }
}

impl Blend{
    /// Sets the blending constant colour.
    #[inline(always)]
    pub fn set_blending_colour(&self,[red,greed,blue,alpha]:Colour){
        unsafe{
            transmute::<usize,fn(f32,f32,f32,f32)>(self.glBlendColor)(red,greed,blue,alpha)
        }
    }

    /// Returns the blending constant colour.
    #[inline(always)]
    pub fn get_blending_colour(&self)->Colour{
        unsafe{
            let mut colour=[0f32;4];
            GLCore.get_float_v(BLEND_COLOR,colour.get_unchecked_mut(0));
            colour
        }
    }

    /// Writes the blending constant colour to `colour`.
    #[inline(always)]
    pub fn write_blending_colour(&self,colour:&mut Colour){
        unsafe{
            GLCore.get_float_v(BLEND_COLOR,colour.get_unchecked_mut(0))
        }
    }
}

impl Blend{
    /// Sets the blending functions.
    #[inline(always)]
    pub fn set_function(&self,sourse_factor:BlendingFunction,destination_factor:BlendingFunction){
        unsafe{
            transmute::<usize,fn(BlendingFunction,BlendingFunction)>(self.glBlendFunc)(sourse_factor,destination_factor)
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
            transmute::<usize,fn(BlendingFunction,BlendingFunction,BlendingFunction,BlendingFunction)>(self.glBlendFuncSeparate)(
                sourse_factor_rgb,
                destination_factor_rgb,
                sourse_factor_alpha,
                destination_factor_alpha
            )
        }
    }

    /// Returns the source blending function for the RBG colour components.
    #[inline(always)]
    pub fn get_function_src_rgb(&self)->BlendingFunction{
        unsafe{
            let mut function=BlendingFunction::One;
            GLCore.get_integer_v(BLEND_SRC_RGB,transmute(&mut function));
            function
        }
    }

    /// Writes the source blending function for the RBG colour components to `function`.
    #[inline(always)]
    pub fn write_function_src_rgb(&self,function:&mut BlendingFunction){
        unsafe{
            GLCore.get_integer_v(BLEND_SRC_RGB,transmute(function))
        }
    }

    /// Returns the source blending function for the Alpha colour component.
    #[inline(always)]
    pub fn get_function_src_alpha(&self)->BlendingFunction{
        unsafe{
            let mut function=BlendingFunction::One;
            GLCore.get_integer_v(BLEND_SRC_ALPHA,transmute(&mut function));
            function
        }
    }

    /// Writes the souse blending function for the Alpha colour component to the `function`.
    #[inline(always)]
    pub fn write_function_src_alpha(&self,function:&mut BlendingFunction){
        unsafe{
            GLCore.get_integer_v(BLEND_SRC_ALPHA,transmute(function))
        }
    }

    /// Returns the destination blending function for the RBG colour components.
    #[inline(always)]
    pub fn get_function_dst_rgb(&self)->BlendingFunction{
        unsafe{
            let mut function=BlendingFunction::Zero;
            GLCore.get_integer_v(BLEND_DST_RGB,transmute(&mut function));
            function
        }
    }

    /// Writes the destination blending function for the RBG colour components to `function`.
    #[inline(always)]
    pub fn write_function_dst_rgb(&self,function:&mut BlendingFunction){
        unsafe{
            GLCore.get_integer_v(BLEND_DST_RGB,transmute(function))
        }
    }

    /// Returns the destination blending function for the Alpha colour component.
    #[inline(always)]
    pub fn get_function_dst_alpha(&self)->BlendingFunction{
        unsafe{
            let mut function=BlendingFunction::Zero;
            GLCore.get_integer_v(BLEND_DST_ALPHA,transmute(&mut function));
            function
        }
    }

    /// Writes the blending function for the Alpha colour component to `function`.
    #[inline(always)]
    pub fn write_function_dst_alpha(&self,function:&mut BlendingFunction){
        unsafe{
            GLCore.get_integer_v(BLEND_DST_ALPHA,transmute(function))
        }
    }
}

impl Blend{
    /// Sets the equation used for both the RGB blending equation and the Alpha blend equation.
    #[inline(always)]
    pub fn set_equation(&self,equation:BlendingEquation){
        unsafe{
            transmute::<usize,fn(BlendingEquation)>(self.glBlendEquation)(equation)
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
            transmute::<usize,fn(BlendingEquation,BlendingEquation)>(self.glBlendEquationSeparate)(equation_rgb,equation_alpha)
        }
    }

    /// Returns the RGB blending equation.
    #[inline(always)]
    pub fn get_equation_rbg(&self)->BlendingEquation{
        unsafe{
            let mut equation=BlendingEquation::Addition;
            GLCore.get_integer_v(BLEND_EQUATION_RGB,transmute(&mut equation));
            equation
        }
    }

    /// Writes the RGB blending equation to `equation`.
    #[inline(always)]
    pub fn write_equation_rbg(&self,equation:&mut BlendingEquation){
        unsafe{
            GLCore.get_integer_v(BLEND_EQUATION_RGB,transmute(equation))
        }
    }

    /// Returns the Alpha blending equation.
    #[inline(always)]
    pub fn get_equation_alpha(&self)->BlendingEquation{
        unsafe{
            let mut equation=BlendingEquation::Addition;
            GLCore.get_integer_v(BLEND_EQUATION_ALPHA,transmute(&mut equation));
            equation
        }
    }

    /// Writes the Alpha blending equation to `equation`.
    #[inline(always)]
    pub fn write_equation_alpha(&self,equation:&mut BlendingEquation){
        unsafe{
            GLCore.get_integer_v(BLEND_EQUATION_ALPHA,transmute(equation));
        }
    }
}