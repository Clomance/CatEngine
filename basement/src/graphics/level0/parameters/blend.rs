use crate::{
    graphics::{
        Colour,
        GLCore,
        core::parameters::GLCapability,
        core::parameters::blend::{
            BlendingEquation,
            BlendingFunction,
        },
    },
};

use core::mem::transmute;

const BLEND_COLOR:u32=0x8005;
const BLEND_DST_ALPHA:u32=0x80CA;
const BLEND_DST_RGB:u32=0x80C8;
const BLEND_EQUATION:u32=0x8009;
const BLEND_EQUATION_ALPHA:u32=0x883D;
const BLEND_EQUATION_RGB:u32=0x8009;
const BLEND_SRC_ALPHA:u32=0x80CB;
const BLEND_SRC_RGB:u32=0x80C9;

/// A wrapper for blending functions.
/// 
/// Blending is disabled by default.
/// 
/// The default blend constant colour is `[0f32;4]`.
/// 
/// The default blending functions for `Source` are `BlendingFunction::One`.
/// 
/// The default blending functions for `Destination` are `BlendingFunction::Zero`.
/// 
/// The default blending equations are `BlendingEquation::Addition`.
pub struct Blend{

}

impl Blend{
    pub const fn new()->Blend{
        Self{}
    }
}

impl Blend{
    /// Enables blending.
    #[inline(always)]
    pub fn enable(&self){
        unsafe{
            GLCore.parameters.enable(GLCapability::Blend)
        }
    }

    /// Disables blending.
    #[inline(always)]
    pub fn disable(&self){
        unsafe{
            GLCore.parameters.disable(GLCapability::Blend)
        }
    }

    /// Checks whether blending is enabled.
    #[inline(always)]
    pub fn is_enabled(&self)->bool{
        unsafe{
            GLCore.parameters.is_enabled(GLCapability::Blend)
        }
    }
}

impl Blend{
    /// Sets the blend constant colour.
    /// 
    /// The colour components are clamped to the range [0,1] before being stored.
    /// 
    /// Initially the colour is set to `[0f32;4]`.
    #[inline(always)]
    pub fn set_blend_colour(&self,colour:Colour){
        unsafe{
            GLCore.parameters.blend.set_blending_colour(colour);
        }
    }

    /// Returns the blend constant colour.
    #[inline(always)]
    pub fn get_blend_colour(&self)->Colour{
        unsafe{
            let mut colour=[0f32;4];
            GLCore.get_float_v(BLEND_COLOR,colour.get_unchecked_mut(0));
            colour
        }
    }

    /// Writes the blend constant colour to `colour`.
    #[inline(always)]
    pub fn write_blend_colour(&self,colour:&mut Colour){
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
            GLCore.parameters.blend.set_function(sourse_factor,destination_factor)
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
            GLCore.parameters.blend.set_function_separate(
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
            GLCore.parameters.blend.set_equation(equation)
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
            GLCore.parameters.blend.set_equation_separate(equation_rgb,equation_alpha)
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