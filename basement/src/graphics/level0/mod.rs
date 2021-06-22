mod blending;
pub use blending::{
    Blending,
    BlendingEquation,
    BlendingFunction,
};

mod error;
pub use error::{
    GLError,
};

mod buffer;
pub use buffer::{
    Buffer,
    BoundBuffer,
    BufferTarget,
    BufferUsage,
};

mod frame_buffer;
pub use frame_buffer::{
    FrameBuffer,
    FrameBufferTarget,
    FrameBufferAttachment,
};

mod vertex_array;
pub use vertex_array::{
    Vertex,
    VertexArray
};

mod shader;
pub use shader::{
    Shader,
    ShaderType,
};

mod texture;
pub use texture::{
    Texture,
    BoundTexture,
    TextureFilter,
    TextureInternalFormat,
    ImageDataFormat,
    Texture2DTarget,
};

use gl::{
    // constants
    VIEWPORT,
    MAX_VIEWPORT_DIMS,
    // functions
    Enable,
    Disable,
    IsEnabled,
    ClearColor,
    ClearStencil,
    ClearDepth,
    GetIntegerv,
};

use std::mem::transmute;

pub struct GraphicsCore;

impl GraphicsCore{
    /// Enables server-side GL capabilities.
    #[inline(always)]
    pub unsafe fn enable(&self,capability:u32){
        Enable(capability)
    }

    /// Disables server-side GL capabilities.
    #[inline(always)]
    pub unsafe fn disable(&self,capability:u32){
        Disable(capability)
    }

    #[inline(always)]
    /// Checks whether capability is enabled.
    pub unsafe fn is_enabled(&self,capability:u32)->bool{
        transmute(IsEnabled(capability))
    }
}

impl GraphicsCore{
    #[inline(always)]
    pub fn viewport(&self)->Viewport{
        Viewport{}
    }

    #[inline(always)]
    pub fn blending(&self)->Blending{
        Blending{}
    }
}

impl GraphicsCore{
    /// Specifies clear values for the colour buffers.
    #[inline(always)]
    pub fn set_clear_colour(&self,[red,greed,blue,alpha]:[f32;4]){
        unsafe{
            ClearColor(red,greed,blue,alpha);
        }
    }

    /// Specifies the clear value for the stencil buffer.
    #[inline(always)]
    pub fn set_clear_stencil(&self,stencil:i32){
        unsafe{
            ClearStencil(stencil)
        }
    }

    /// Specify the clear value for the depth buffer.
    #[inline(always)]
    pub fn set_clear_depth(&self,depth:f64){
        unsafe{
            ClearDepth(depth)
        }
    }
}

pub struct Viewport{}

impl Viewport{
    #[inline(always)]
    pub unsafe fn set(&self,[x,y,widht,height]:[i32;4]){
        gl::Viewport(x,y,widht,height);
    }

    #[inline(always)]
    pub fn get(&self)->[i32;4]{
        unsafe{
            let mut viewport=[0i32;4];
            GetIntegerv(VIEWPORT,viewport.get_unchecked_mut(0));
            viewport
        }
    }

    #[inline(always)]
    pub fn get_ref(&self,destination:&mut [i32;4]){
        unsafe{
            GetIntegerv(VIEWPORT,destination.get_unchecked_mut(0));
        }
    }

    #[inline(always)]
    pub fn get_max_dimensions(&self)->[i32;2]{
        unsafe{
            let mut dimensions=[0i32;2];
            GetIntegerv(MAX_VIEWPORT_DIMS,dimensions.get_unchecked_mut(0));
            dimensions
        }
    }

    #[inline(always)]
    pub fn write_max_dimensions(&self,dimensions:&mut [i32;2]){
        unsafe{
            GetIntegerv(MAX_VIEWPORT_DIMS,dimensions.get_unchecked_mut(0));
        }
    }
}