use crate::graphics::{
    GLCore,
    core::GLError,
    core::parameters::{
        GLCapability,
        DrawBufferMode,
    },
};

mod blend;
pub use blend::Blend;

mod viewport;
pub use viewport::Viewport;

pub struct GraphicsParameters{
    pub blend:Blend,
}

impl GraphicsParameters{
    pub const fn new()->GraphicsParameters{
        Self{
            blend:Blend::new(),
        }
    }
}

impl GraphicsParameters{
    /// Enables server-side GL capabilities.
    #[inline(always)]
    pub fn enable(&self,capability:GLCapability){
        unsafe{
            GLCore.parameters.enable(capability)
        }
    }

    /// Disables server-side GL capabilities.
    #[inline(always)]
    pub fn disable(&self,capability:GLCapability){
        unsafe{
            GLCore.parameters.disable(capability)
        }
    }

    /// Tests whether a capability is enabled
    #[inline(always)]
    pub fn is_enabled(&self,capability:GLCapability)->bool{
        unsafe{
            GLCore.parameters.is_enabled(capability)
        }
    }
}

impl GraphicsParameters{
    #[inline(always)]
    pub fn set_line_width(&self,width:f32)->GLError{
        unsafe{
            GLCore.parameters.set_line_width(width);
            GLCore.get_error()
        }
    }
}

impl GraphicsParameters{
    #[inline(always)]
    pub fn set_clear_colour(&self,colour:[f32;4]){
        unsafe{
            GLCore.parameters.set_clear_colour(colour)
        }
    }
}

impl GraphicsParameters{
    #[inline(always)]
    pub fn draw_buffer(&self,mode:DrawBufferMode){
        unsafe{
            GLCore.parameters.draw_buffer(mode)
        }
    }
}