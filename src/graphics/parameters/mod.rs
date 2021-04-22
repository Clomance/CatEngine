use crate::{
    Colour,
};

mod blending;
pub use blending::{
    BlendingEquation,
    BlendingFunction,
    Blending,
};

mod drawing;
pub use drawing::{
    DrawMode,
    DrawParameters,
};

use cat_engine_basement::graphics::gl::{
    // constants
    // functions
    Enable,
    Disable,
    ClearColor,
    ClearStencil,
    ClearDepth,
    Viewport,
};

/// Graphics library drawing parameters.
pub struct GraphicsParameters{
    blending:Blending
}

impl GraphicsParameters{
    pub fn new()->GraphicsParameters{
        Self{
            blending:Blending,
        }
    }
}

impl GraphicsParameters{
    /// Enables server-side GL capabilities.
    pub unsafe fn enable(&self,capability:u32){
        Enable(capability)
    }

    /// Disables server-side GL capabilities.
    pub unsafe fn disable(&self,capability:u32){
        Disable(capability)
    }
}

impl GraphicsParameters{
    /// Specifies clear values for the colour buffers.
    pub fn set_clear_colour(&self,[red,greed,blue,alpha]:Colour){
        unsafe{
            ClearColor(red,greed,blue,alpha);
        }
    }

    /// Specifies the clear value for the stencil buffer.
    pub fn set_clear_stencil(&self,stencil:i32){
        unsafe{
            ClearStencil(stencil)
        }
    }

    /// Specify the clear value for the depth buffer.
    pub fn set_clear_depth(&self,depth:f64){
        unsafe{
            ClearDepth(depth)
        }
    }
}

impl GraphicsParameters{
    pub fn blending(&self)->&Blending{
        &self.blending
    }
}