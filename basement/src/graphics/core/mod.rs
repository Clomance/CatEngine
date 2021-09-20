#[cfg(target_os="windows")]
use crate::windows::OpenGraphicsLibrary;

pub mod blending;
use blending::Blending;

pub mod buffer;
use buffer::Buffer;

pub mod drawing;
use drawing::Drawing;

pub mod framebuffer;
use framebuffer::Framebuffer;

pub mod program;
use program::Program;

pub mod shader;
use shader::Shader;

pub mod texture;
use texture::Texture;

pub mod uniform;
use uniform::Uniform;

pub mod vertex_array;
use vertex_array::VertexArray;

pub mod viewport;
use viewport::Viewport;

use std::{
    ffi::CStr,
    mem::transmute,
};

const MAJOR_VERSION:u32=0x821B;
const MINOR_VERSION:u32=0x821C;
const VERSION:u32=0x1F02;

// Capabilities
const BLEND:u32=0x0BE2;
// pub const CLIP_DISTANCE0:u32=0x3000;
// pub const CLIP_DISTANCE1:u32=0x3001;
// pub const CLIP_DISTANCE2:u32=0x3002;
// pub const CLIP_DISTANCE3:u32=0x3003;
// pub const CLIP_DISTANCE4:u32=0x3004;
// pub const CLIP_DISTANCE5:u32=0x3005;
// pub const CLIP_DISTANCE6:u32=0x3006;
// pub const CLIP_DISTANCE7:u32=0x3007;
// pub const COLOR_LOGIC_OP:u32=0x0BF2;
// pub const CULL_FACE:u32=0x0B44;

// Clear mask bits
const COLOR_BUFFER_BIT:u32=0x00004000;
const DEPTH_BUFFER_BIT:u32=0x00000100;
const STENCIL_BUFFER_BIT:u32=0x00000400;

// Errors
const NO_ERROR:u32=0;
const INVALID_ENUM:u32=0x0500;
const INVALID_VALUE:u32=0x0501;
const INVALID_OPERATION:u32=0x0502;
const INVALID_FRAMEBUFFER_OPERATION:u32=0x0506;
const OUT_OF_MEMORY:u32=0x0505;

// Pixel storage parameters
pub const UNPACK_ALIGNMENT:u32=0x0CF5;

// Draw buffer modes
const NONE:u32=0;
const FRONT_LEFT:u32=0x0400;
const FRONT_RIGHT:u32=0x0401;
const BACK_LEFT:u32=0x0402;
const BACK_RIGHT:u32=0x0403;
const FRONT:u32=0x0404;
const BACK:u32=0x0405;
const LEFT:u32=0x0406;
const RIGHT:u32=0x0407;
const FRONT_AND_BACK:u32=0x0408;

#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum GLCapability{
    /// If enabled, blend the computed fragment colour values with the values in the colour buffers.
    /// See glBlendFunc.
    Blend=BLEND,

    // /// If enabled, clip geometry against user-defined half space i.
    // ClipDistance0=CLIP_DISTANCE0,
    // ClipDistance1=CLIP_DISTANCE1,
    // ClipDistance2=CLIP_DISTANCE2,
    // ClipDistance3=CLIP_DISTANCE3,
    // ClipDistance4=CLIP_DISTANCE4,
    // ClipDistance5=CLIP_DISTANCE5,
    // ClipDistance6=CLIP_DISTANCE6,
    // ClipDistance7=CLIP_DISTANCE7,

    // /// If enabled, apply the currently selected logical operation to the computed fragment colour and colour buffer values.
    // /// See glLogicOp.
    // ColourLogicOperation=COLOR_LOGIC_OP,

    // /// If enabled, cull polygons based on their winding in window coordinates.
    // /// See glCullFace.
    // CullFace=CULL_FACE,

    // /// If enabled, the −wc≤zc≤wc plane equation is ignored by view volume clipping (effectively, there is no near or far plane clipping).
    // /// See glDepthRange.
    // GL_DEPTH_CLAMP,

    // /// If enabled, do depth comparisons and update the depth buffer.
    // /// Note that even if the depth buffer exists and the depth mask is non-zero,
    // /// the depth buffer is not updated if the depth test is disabled.
    // /// See glDepthFunc and glDepthRange.
    // GL_DEPTH_TEST,

    // /// If enabled, dither colour components or indices before they are written to the colour buffer.
    // GL_DITHER,

    // /// If enabled and the value of GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING for the framebuffer attachment
    // /// corresponding to the destination buffer is GL_SRGB, the R, G, and B destination colour values (after conversion from fixed-point to floating-point)
    // /// are considered to be encoded for the sRGB colour space and hence are linearized prior to their use in blending.,
    // GL_FRAMEBUFFER_SRGB,
    
    // /// If enabled, draw lines with correct filtering.
    // /// Otherwise, draw aliased lines.
    // /// See glLineWidth.
    // GL_LINE_SMOOTH,

    // /// If enabled, use multiple fragment samples in computing the final colour of a pixel.
    // /// See glSampleCoverage.
    // GL_MULTISAMPLE,

    // /// If enabled, and if the polygon is rendered in GL_FILL mode,
    // /// an offset is added to depth values of a polygon's fragments before the depth comparison is performed.
    // /// See glPolygonOffset.
    // GL_POLYGON_OFFSET_FILL,

    // /// If enabled, and if the polygon is rendered in GL_LINE mode,
    // /// an offset is added to depth values of a polygon's fragments before the depth comparison is performed.
    // /// See glPolygonOffset.
    // GL_POLYGON_OFFSET_LINE,

    // /// If enabled, an offset is added to depth values of a polygon's fragments before the depth comparison is performed,
    // /// if the polygon is rendered in GL_POINT mode.
    // /// See glPolygonOffset.,
    // GL_POLYGON_OFFSET_POINT,

    // /// If enabled, draw polygons with proper filtering.
    // /// Otherwise, draw aliased polygons.
    // /// For correct antialiased polygons, an alpha buffer is needed and the polygons must be sorted front to back.
    // GL_POLYGON_SMOOTH,

    // /// Enables primitive restarting.
    // /// If enabled, any one of the draw commands which transfers a set of generic attribute array elements to the GL will restart the primitive
    // /// when the index of the vertex is equal to the primitive restart index.
    // /// See glPrimitiveRestartIndex.
    // /// 
    // ///GL version is 3.1 or greater.
    // GL_PRIMITIVE_RESTART,

    // /// If enabled, compute a temporary coverage value where each bit is determined by the alpha value at the corresponding sample location.
    // /// The temporary coverage value is then ANDed with the fragment coverage value.
    // GL_SAMPLE_ALPHA_TO_COVERAGE,

    // /// If enabled, each sample alpha value is replaced by the maximum representable alpha value.
    // GL_SAMPLE_ALPHA_TO_ONE,

    // /// If enabled, the fragment's coverage is ANDed with the temporary coverage value.
    // /// If GL_SAMPLE_COVERAGE_INVERT is set to GL_TRUE, invert the coverage value.
    // /// See glSampleCoverage.,
    // GL_SAMPLE_COVERAGE,

    // /// If enabled, discard fragments that are outside the scissor rectangle.
    // /// See glScissor.
    // GL_SCISSOR_TEST,

    // /// If enabled, do stencil testing and update the stencil buffer.
    // /// See glStencilFunc and glStencilOp.
    // GL_STENCIL_TEST,

    // /// If enabled, modifies the way sampling is performed on cube map textures.
    // /// See the spec for more information.
    // GL_TEXTURE_CUBE_MAP_SEAMLESS,

    // /// If enabled and a vertex or geometry shader is active,
    // /// then the derived point size is taken from the (potentially clipped) shader builtin gl_PointSize
    // /// and clamped to the implementation-dependent point size range.
    // GL_PROGRAM_POINT_SIZE,
}

/// Mask that indicate the buffers to be cleared.
#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum ClearMask{
    Colour=COLOR_BUFFER_BIT,
    Depth=DEPTH_BUFFER_BIT,
    Stencil=STENCIL_BUFFER_BIT,
    ColourDepth=COLOR_BUFFER_BIT|DEPTH_BUFFER_BIT,
    ColourStencil=COLOR_BUFFER_BIT|STENCIL_BUFFER_BIT,
    DepthStencil=DEPTH_BUFFER_BIT|STENCIL_BUFFER_BIT,
    ColourDepthStencil=COLOR_BUFFER_BIT|DEPTH_BUFFER_BIT|STENCIL_BUFFER_BIT,
}


#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum GLError{
    /// No error has been recorded.
    /// The value of this symbolic constant is guaranteed to be 0.
    NoError=NO_ERROR,

    /// An unacceptable value is specified for an enumerated argument.
    /// The offending command is ignored and has no other side effect than to set the error flag.
    InvalidEnum=INVALID_ENUM,

    /// A numeric argument is out of range.
    /// The offending command is ignored and has no other side effect than to set the error flag.
    InvalidValue=INVALID_VALUE,

    /// The specified operation is not allowed in the current state.
    /// The offending command is ignored and has no other side effect than to set the error flag.
    InvalidOperation=INVALID_OPERATION,

    /// The framebuffer object is not complete.
    /// The offending command is ignored and has no other side effect than to set the error flag.
    InvalidFramebufferOperation=INVALID_FRAMEBUFFER_OPERATION,

    /// There is not enough memory left to execute the command.
    /// The state of the GL is undefined, except for the state of the error flags, after this error is recorded.
    OutOfMemory=OUT_OF_MEMORY,
}

impl GLError{
    pub fn no_error(&self)->bool{
        *self==GLError::NoError
    }

    pub fn is_error(&self)->bool{
        *self!=GLError::NoError
    }

    pub fn unwrap(&self){
        if self.is_error(){
            panic!("{:?}",self)
        }
    }

    pub fn expect(&self,message:&str){
        if self.is_error(){
            panic!("{:?}, Message: {}",self,message)
        }
    }
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum DrawBufferMode{
    /// No color buffers are written.
    None=NONE,

    /// Only the front left color buffer is written.
    FrontLeft=FRONT_LEFT,

    /// Only the front right color buffer is written.
    FrontRight=FRONT_RIGHT,

    /// Only the back left color buffer is written.
    BackLeft=BACK_LEFT,

    /// Only the back right color buffer is written.
    BackRight=BACK_RIGHT,

    /// Only the front left and front right color buffers are written.
    /// If there is no front right color buffer, only the front left color buffer is written.
    Front=FRONT,

    /// Only the back left and back right color buffers are written.
    /// If there is no back right color buffer, only the back left color buffer is written.
    Back=BACK,

    /// Only the front left and back left color buffers are written.
    /// If there is no back left color buffer, only the front left color buffer is written.
    Left=LEFT,

    /// Only the front right and back right color buffers are written.
    /// If there is no back right color buffer, only the front right color buffer is written.
    Right=RIGHT,

    /// All the front and back color buffers (front left, front right, back left, back right) are written.
    /// If there are no back color buffers, only the front left and front right color buffers are written.
    /// If there are no right color buffers, only the front left and back left color buffers are written.
    /// If there are no right or back color buffers, only the front left color buffer is written.
    FrontBack=FRONT_AND_BACK
}

pub struct GraphicsCore{
    pub blending:Blending,
    pub buffer:Buffer,
    pub drawing:Drawing,
    pub framebuffer:Framebuffer,
    pub program:Program,
    pub shader:Shader,
    pub texture:Texture,
    pub uniform:Uniform,
    pub vertex_array:VertexArray,
    pub viewport:Viewport,

    glEnable:usize,
    glDisable:usize,
    glIsEnabled:usize,

    glGetIntegerv:usize,
    glGetFloatv:usize,
    glGetString:usize,

    glClearColor:usize,
    glClear:usize,

    glGetError:usize,

    glPixelStoref:usize,
    glPixelStorei:usize,

    glDrawBuffer:usize,

    glFinish:usize,
}

impl GraphicsCore{
    pub const fn new()->GraphicsCore{
        Self{
            blending:Blending::new(),
            buffer:Buffer::new(),
            drawing:Drawing::new(),
            framebuffer:Framebuffer::new(),
            program:Program::new(),
            shader:Shader::new(),
            texture:Texture::new(),
            uniform:Uniform::new(),
            vertex_array:VertexArray::new(),
            viewport:Viewport::new(),

            glEnable:0,
            glDisable:0,
            glIsEnabled:0,

            glGetIntegerv:0,
            glGetFloatv:0,
            glGetString:0,

            glClearColor:0,
            glClear:0,

            glGetError:0,

            glPixelStoref:0,
            glPixelStorei:0,

            glDrawBuffer:0,

            glFinish:0,
        }
    }

    #[cfg(target_os="windows")]
    pub fn load_functions(&mut self,library:&OpenGraphicsLibrary){
        self.blending.load(library);
        self.buffer.load(library);
        self.drawing.load(library);
        self.framebuffer.load(library);
        self.program.load(library);
        self.shader.load(library);
        self.texture.load(library);
        self.uniform.load(library);
        self.vertex_array.load(library);
        self.viewport.load(library);

        unsafe{
            self.glEnable=transmute(library.get_proc_address("glEnable\0"));
            self.glDisable=transmute(library.get_proc_address("glDisable\0"));
            self.glIsEnabled=transmute(library.get_proc_address("glIsEnabled\0"));

            self.glGetIntegerv=transmute(library.get_proc_address("glGetIntegerv\0"));
            self.glGetFloatv=transmute(library.get_proc_address("glGetFloatv\0"));
            self.glGetString=transmute(library.get_proc_address("glGetString\0"));

            self.glClearColor=transmute(library.get_proc_address("glClearColor\0"));
            self.glClear=transmute(library.get_proc_address("glClear\0"));

            self.glGetError=transmute(library.get_proc_address("glGetError\0"));

            self.glPixelStoref=transmute(library.get_proc_address("glPixelStoref\0"));
            self.glPixelStorei=transmute(library.get_proc_address("glPixelStorei\0"));

            self.glDrawBuffer=transmute(library.get_proc_address("glDrawBuffer\0"));

            self.glFinish=transmute(library.get_proc_address("glFinish\0"));
        }
    }
}

impl GraphicsCore{
    /// Enables server-side GL capabilities.
    #[inline(always)]
    pub fn enable(&self,capability:GLCapability){
        unsafe{
            transmute::<usize,fn(GLCapability)>(self.glEnable)(capability)
        }
    }

    /// Disables server-side GL capabilities.
    #[inline(always)]
    pub fn disable(&self,capability:GLCapability){
        unsafe{
            transmute::<usize,fn(GLCapability)>(self.glDisable)(capability)
        }
    }

    /// Tests whether a capability is enabled
    #[inline(always)]
    pub fn is_enabled(&self,capability:GLCapability)->bool{
        unsafe{
            transmute::<usize,fn(GLCapability)->bool>(self.glIsEnabled)(capability)
        }
    }

    /// Specifies clear values for the colour buffers.
    #[inline(always)]
    pub fn set_clear_colour(&self,[red,greed,blue,alpha]:[f32;4]){
        unsafe{
            transmute::<usize,fn(f32,f32,f32,f32)>(self.glClearColor)(red,greed,blue,alpha)
        }
    }

    #[inline(always)]
    pub fn clear(&self,mask:ClearMask){
        unsafe{
            transmute::<usize,fn(ClearMask)>(self.glClear)(mask)
        }
    }

    #[inline(always)]
    pub fn get_error(&self)->GLError{
        unsafe{
            transmute::<usize,fn()->GLError>(self.glGetError)()
        }
    }

    /// Specify which color buffers are to be drawn into.
    /// 
    /// When colors are written to the frame buffer,
    /// they are written into the color buffers specified by `GraphicsCore::draw_buffer`.
    /// 
    /// If more than one color buffer is selected for drawing,
    /// then blending or logical operations are computed
    /// and applied independently for each color bufferand can produce different results in each buffer.
    /// 
    /// Monoscopic contexts include only left buffers,
    /// and stereoscopic contexts include both left and right buffers.
    /// Likewise, single-buffered contexts include only front buffers,
    /// and double-buffered contexts include both front and back buffers.
    /// The context is selected at GL initialization.
    /// 
    /// The initial value is `DrawBufferMode::Front` for single-buffered contexts,
    /// and GL_BACK for double-buffered contexts.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if none of the buffers indicated by `mode` exists.
    #[inline(always)]
    pub fn draw_buffer(&self,mode:DrawBufferMode){
        unsafe{
            transmute::<usize,fn(DrawBufferMode)>(self.glDrawBuffer)(mode)
        }
    }

    /// Blocks the current thread until all GL execution is complete.
    #[inline(always)]
    pub fn finish(&self){
        unsafe{
            transmute::<usize,fn()>(self.glFinish)()
        }
    }
}

impl GraphicsCore{
    #[inline(always)]
    pub unsafe fn set_pixel_storage_modei(&self,parameter:u32,value:i32){
        transmute::<usize,fn(u32,i32)>(self.glPixelStorei)(parameter,value)
    }

    #[inline(always)]
    pub unsafe fn set_pixel_storage_modef(&self,parameter:u32,value:f32){
        transmute::<usize,fn(u32,f32)>(self.glPixelStoref)(parameter,value)
    }
}

impl GraphicsCore{
    #[inline(always)]
    pub unsafe fn get_integer_v(&self,parameter:u32,value:&mut i32){
        transmute::<usize,fn(u32,&mut i32)>(self.glGetIntegerv)(parameter,value)
    }

    #[inline(always)]
    pub unsafe fn get_float_v(&self,parameter:u32,value:&mut f32){
        transmute::<usize,fn(u32,&mut f32)>(self.glGetFloatv)(parameter,value)
    }

    // GL_VENDOR, GL_RENDERER, GL_VERSION, or GL_SHADING_LANGUAGE_VERSION,
    #[inline(always)]
    pub unsafe fn get_string(&self,connection:u32)->&CStr{
        CStr::from_ptr(transmute::<usize,fn(u32)->*const i8>(self.glGetString)(connection))
    }
}

/// Versions.
impl GraphicsCore{
    #[inline(always)]
    pub fn get_major_version(&self)->i32{
        unsafe{
            let mut version=0i32;
            self.get_integer_v(MAJOR_VERSION,&mut version);
            version
        }
    }

    #[inline(always)]
    pub fn get_minor_version(&self)->i32{
        unsafe{
            let mut version=0i32;
            self.get_integer_v(MINOR_VERSION,&mut version);
            version
        }
    }

    #[inline(always)]
    pub fn get_version_string(&self)->&CStr{
        unsafe{
            self.get_string(VERSION)
        }
    }
}