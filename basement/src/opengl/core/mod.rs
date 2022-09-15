//! This module contains slightly wrapped OpenGL functions.
#[cfg(target_os="windows")]
use crate::winapi::OpenGraphicsLibrary;

pub mod blend;
use blend::Blend;

pub mod buffer;
use buffer::Buffer;

pub mod constants;
use constants::*;

pub mod depth;
use depth::Depth;

pub mod drawing;
use drawing::Drawing;

pub mod parameter_info;
use parameter_info::CoreParameterInfo;

pub mod primitive_parameters;
use primitive_parameters::PrimitiveParameters;

pub mod program;
use program::Program;

pub mod shader;
use shader::Shader;

pub mod stencil;
use stencil::Stencil;

pub mod texture;
use texture::Texture;

pub mod uniform;
use uniform::Uniform;

pub mod vertex_array;
use vertex_array::VertexArray;

use core::{
    mem::transmute,
    ptr::NonNull,
};

mod types{
    use std::{
        marker::{
            PhantomData,
            PhantomPinned
        },
        os::raw::{
            c_uchar,
            c_char,
            c_short,
            c_ushort,
            c_uint,
            c_int,
            c_float,
            c_double,
            c_void
        }
    };

    pub struct __GLsync{
        _data:[u8;0],
        _marker:PhantomData<(*mut u8,PhantomPinned)>,
    }

    pub type GLboolean=c_uchar;
    pub type GLbyte=c_char;
    pub type GLubyte=c_uchar;
    pub type GLchar=c_char;
    pub type GLshort=c_short;
    pub type GLushort=c_ushort;
    pub type GLint=c_int;
    pub type GLuint=c_uint;
    pub type GLfixed=c_uint;
    pub type GLint64=i64;
    pub type GLuint64=u64;
    pub type GLsizei=c_int;
    pub type GLenum=c_uint;
    pub type GLintptr=isize;
    pub type GLsizeiptr=isize;
    pub type GLsync=*const __GLsync;
    pub type GLbitfield=c_int;
    pub type GLhalf=c_ushort;
    pub type GLfloat=c_float;
    pub type GLclampf=c_float;
    pub type GLdouble=c_double;
    pub type GLclampd=c_double;
    pub type GLvoid=c_void;
}
use types::*;

#[cfg(target_os="linux")]
extern "system"{
    fn glEnable(capability:GLenum);
    fn glEnablei(capability:GLenum,index:GLuint);

    fn glDisable(capability:GLenum);
    fn glDisablei(capability:GLenum,index:GLuint);

    fn glIsEnabled(capability:GLenum)->GLboolean;
    fn glIsEnabledi(capability:GLenum,index:GLuint)->GLboolean;

    fn glPixelStoref(parameter:GLenum,value:GLfloat);
    fn glPixelStorei(parameter:GLenum,value:GLint);

    fn glClampColor(target:GLenum,clamp:GLenum);

    fn glViewport(x:GLint,y:GLint,width:GLint,height:GLint);

    fn glClearColor(red:GLclampf,green:GLclampf,blue:GLclampf,alpha:GLclampf);
    fn glClearDepth(depth:GLclampd);
    fn glClearStencil(stencil:GLint);

    fn glClear(mask:GLbitfield);

    fn glFlush();
    fn glFinish();

    fn glGetError()->GLenum;

    fn glGetString(name:GLenum)->*const GLubyte;
    fn glGetStringi(name:GLenum,index:GLuint)->*const GLubyte;

    fn glGetMultisamplefv(parameter:GLenum,index:GLuint,values:*mut GLfloat);
}

#[cfg(target_os="windows")]
mod gl{
    pub static mut glEnable:usize=0;
    pub static mut glEnablei:usize=0;

    pub static mut glDisable:usize=0;
    pub static mut glDisablei:usize=0;

    pub static mut glIsEnabled:usize=0;
    pub static mut glIsEnabledi:usize=0;

    pub static mut glPixelStoref:usize=0;
    pub static mut glPixelStorei:usize=0;

    pub static mut glClampColor:usize=0;

    pub static mut glViewport:usize=0;

    pub static mut glClearColor:usize=0;
    pub static mut glClearDepth:usize=0;
    pub static mut glClearStencil:usize=0;

    pub static mut glClear:usize=0;

    pub static mut glFlush:usize=0;
    pub static mut glFinish:usize=0;

    pub static mut glGetError:usize=0;

    pub static mut glGetString:usize=0;
    pub static mut glGetStringi:usize=0;

    pub static mut glGetMultisamplefv:usize=0;
}

#[cfg(target_os="windows")]
mod gl_functions{
    use super::*;

    pub unsafe fn glEnable(capability:GLenum){
        transmute::<usize,extern "system" fn(GLenum)>(gl::glEnable)(capability)
    }
    pub unsafe fn glEnablei(capability:GLenum,index:GLuint){
        transmute::<usize,extern "system" fn(GLenum,GLuint)>(gl::glEnablei)(capability,index)
    }

    pub unsafe fn glDisable(capability:GLenum){
        transmute::<usize,extern "system" fn(GLenum)>(gl::glDisable)(capability)
    }
    pub unsafe fn glDisablei(capability:GLenum,index:GLuint){
        transmute::<usize,extern "system" fn(GLenum,GLuint)>(gl::glDisablei)(capability,index)
    }

    pub unsafe fn glIsEnabled(capability:GLenum)->GLboolean{
        transmute::<usize,extern "system" fn(GLenum)->GLboolean>(gl::glIsEnabled)(capability)
    }
    pub unsafe fn glIsEnabledi(capability:GLenum,index:GLuint)->GLboolean{
        transmute::<usize,extern "system" fn(GLenum,GLuint)->GLboolean>(gl::glIsEnabledi)(capability,index)
    }


    pub unsafe fn glPixelStoref(parameter:GLenum,value:GLfloat){
        transmute::<usize,extern "system" fn(GLenum,GLfloat)>(gl::glPixelStoref)(parameter,value)
    }
    pub unsafe fn glPixelStorei(parameter:GLenum,value:GLint){
        transmute::<usize,extern "system" fn(GLenum,GLint)>(gl::glPixelStorei)(parameter,value)
    }


    pub unsafe fn glClampColor(target:GLenum,clamp:GLenum){
        transmute::<usize,extern "system" fn(GLenum,GLenum)>(gl::glClampColor)(target,clamp)
    }

    pub unsafe fn glViewport(x:GLint,y:GLint,width:GLint,height:GLint){
        transmute::<usize,extern "system" fn(GLint,GLint,GLint,GLint)>(gl::glViewport)(x,y,width,height)
    }

    pub unsafe fn glClearColor(red:GLclampf,green:GLclampf,blue:GLclampf,alpha:GLclampf){
        transmute::<usize,extern "system" fn(GLclampf,GLclampf,GLclampf,GLclampf)>(gl::glClearColor)(red,green,blue,alpha)
    }
    pub unsafe fn glClearDepth(depth:GLclampd){
        transmute::<usize,extern "system" fn(GLclampd)>(gl::glClearDepth)(depth)
    }
    pub unsafe fn glClearStencil(stencil:GLint){
        transmute::<usize,extern "system" fn(GLint)>(gl::glClearStencil)(stencil)
    }
    pub unsafe fn glClear(mask:GLbitfield){
        transmute::<usize,extern "system" fn(GLbitfield)>(gl::glClear)(mask)
    }


    pub unsafe fn glFlush(){
        transmute::<usize,extern "system" fn()>(gl::glFlush)()
    }

    pub unsafe fn glFinish(){
        transmute::<usize,extern "system" fn()>(gl::glFinish)()
    }

    pub unsafe fn glGetError()->GLenum{
        transmute::<usize,extern "system" fn()->GLenum>(gl::glGetError)()
    }

    pub unsafe fn glGetString(name:GLenum)->*const GLubyte{
        transmute::<usize,extern "system" fn(GLenum)->*const GLubyte>(gl::glGetString)(name)
    }
    pub unsafe fn glGetStringi(name:GLenum,index:GLuint)->*const GLubyte{
        transmute::<usize,extern "system" fn(GLenum,GLuint)->*const GLubyte>(gl::glGetStringi)(name,index)
    }

    pub unsafe fn glGetMultisamplefv(parameter:GLenum,index:GLuint,values:*mut GLfloat){
        transmute::<usize,extern "system" fn(GLenum,GLuint,*mut GLfloat)>(gl::glGetMultisamplefv)(parameter,index,values)
    }
}

#[cfg(target_os="windows")]
use gl_functions::*;

#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum CoreCapability{
    /// If enabled, blend the computed fragment colour values with the values in the colour buffers.
    /// 
    /// See `Blend::set_function`.
    Blend=BLEND,

    /// If enabled, apply the currently selected logical operation to the computed fragment colour and colour buffer values.
    /// See `glLogicOp`.
    ColourLogicOperation=COLOR_LOGIC_OP,

    /// If enabled, cull polygons based on their winding in window coordinates.
    /// See `glCullFace`.
    CullFace=CULL_FACE,

    /// If enabled, the `−wc≤zc≤wc` plane equation is ignored by view volume clipping
    /// (effectively, there is no near or far plane clipping).
    /// See glDepthRange.
    DepthClamp=DEPTH_CLAMP,

    /// If enabled, do depth comparisons and update the depth buffer.
    /// Note that even if the depth buffer exists and the depth mask is non-zero,
    /// the depth buffer is not updated if the depth test is disabled.
    /// See glDepthFunc and glDepthRange.
    DepthTest=DEPTH_TEST,

    /// If enabled, dither colour components or indices before they are written to the colour buffer.
    Dither=DITHER,

    /// If enabled, discard fragments that are outside the scissor rectangle.
    /// See glScissor.
    ScissorTest=SCISSOR_TEST,

    /// If enabled, do stencil testing and update the stencil buffer.
    /// See glStencilFunc and glStencilOp.
    StencilTest=STENCIL_TEST,

    /// If enabled and the value of GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING for the framebuffer attachment
    /// corresponding to the destination buffer is GL_SRGB, the R, G, and B destination colour values (after conversion from fixed-point to floating-point)
    /// are considered to be encoded for the sRGB colour space and hence are linearized prior to their use in blending.,
    FramebufferSRGB=FRAMEBUFFER_SRGB,

    /// If enabled, draw lines with correct filtering.
    /// Otherwise, draw aliased lines.
    /// See `GraphicsParameters::glLineWidth`.
    LineSmooth=LINE_SMOOTH,

    /// If enabled, use multiple fragment samples in computing the final colour of a pixel.
    /// See glSampleCoverage.
    Multisample=MULTISAMPLE,

    /// If enabled, and if the polygon is rendered in GL_FILL mode,
    /// an offset is added to depth values of a polygon's fragments before the depth comparison is performed.
    /// See glPolygonOffset.
    PolygonOffsetFill=POLYGON_OFFSET_FILL,

    /// If enabled, and if the polygon is rendered in GL_LINE mode,
    /// an offset is added to depth values of a polygon's fragments before the depth comparison is performed.
    /// See glPolygonOffset.
    PolygonOffsetLine=POLYGON_OFFSET_LINE,

    /// If enabled, an offset is added to depth values of a polygon's fragments before the depth comparison is performed,
    /// if the polygon is rendered in GL_POINT mode.
    /// See glPolygonOffset.
    PolygonOffsetPoint=POLYGON_OFFSET_POINT,

    /// If enabled, draw polygons with proper filtering.
    /// Otherwise, draw aliased polygons.
    /// For correct antialiased polygons, an alpha buffer is needed and the polygons must be sorted front to back.
    PolygonSmooth=POLYGON_SMOOTH,

    /// Enables primitive restarting.
    /// If enabled, any one of the draw commands which transfers a set of generic attribute array elements to the GL will restart the primitive
    /// when the index of the vertex is equal to the primitive restart index.
    /// See glPrimitiveRestartIndex.
    /// 
    /// GL version is 3.1 or greater.
    PrimiviteRestart=PRIMITIVE_RESTART,

    /// If enabled, compute a temporary coverage value where each bit is determined by the alpha value at the corresponding sample location.
    /// The temporary coverage value is then ANDed with the fragment coverage value.
    SampleAlphaToCoverage=SAMPLE_ALPHA_TO_COVERAGE,

    /// If enabled, each sample alpha value is replaced by the maximum representable alpha value.
    SampleAlphaToOne=SAMPLE_ALPHA_TO_ONE,

    /// If enabled, the fragment's coverage is ANDed with the temporary coverage value.
    /// If GL_SAMPLE_COVERAGE_INVERT is set to GL_TRUE, invert the coverage value.
    /// See glSampleCoverage.,
    SampleCoverage=SAMPLE_COVERAGE,

    /// If enabled, modifies the way sampling is performed on cube map textures.
    /// See the spec for more information.
    TextureCubeMapSeamless=TEXTURE_CUBE_MAP_SEAMLESS,

    /// If enabled and a vertex or geometry shader is active,
    /// then the derived point size is taken from the (potentially clipped) shader builtin gl_PointSize
    /// and clamped to the implementation-dependent point size range.
    ProgramPointSize=PROGRAM_POINT_SIZE,
}

#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum IndexedCoreCapability{
    /// If enabled, clip geometry against user-defined half space `index`.
    ClipDistance=CLIP_DISTANCE0,
}

/// The mask that indicates the buffers to be cleared.
#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum ClearMask{
    None=0,

    /// Indicates the buffers currently enabled for colour writing.
    Colour=COLOR_BUFFER_BIT,

    /// Indicates the depth buffer.
    Depth=DEPTH_BUFFER_BIT,

    /// Indicates the stencil buffer.
    Stencil=STENCIL_BUFFER_BIT,

    ColourDepth=COLOR_BUFFER_BIT|DEPTH_BUFFER_BIT,
    ColourStencil=COLOR_BUFFER_BIT|STENCIL_BUFFER_BIT,
    DepthStencil=DEPTH_BUFFER_BIT|STENCIL_BUFFER_BIT,
    ColourDepthStencil=COLOR_BUFFER_BIT|DEPTH_BUFFER_BIT|STENCIL_BUFFER_BIT,
}


impl ClearMask{
    pub fn add(self,mask:ClearMask)->ClearMask{
        unsafe{transmute(self as u32|mask as u32)}
    }

    pub fn remove(self,mask:ClearMask)->ClearMask{
        unsafe{transmute(self as u32&!(mask as u32))}
    }
}


/// Represents the defined errors.
#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Error{
    None=0,

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

pub struct Core;

impl Core{
    #[cfg(target_os="windows")]
    pub fn load_functions(library:&OpenGraphicsLibrary){
        Blend::load(library);
        Buffer::load(library);
        Depth::load(library);
        Drawing::load(library);
        CoreParameterInfo::load(library);
        PrimitiveParameters::load(library);
        Program::load(library);
        Shader::load(library);
        Stencil::load(library);
        Texture::load(library);
        Uniform::load(library);
        VertexArray::load(library);

        unsafe{
            use gl::*;

            glEnable=transmute(library.get_proc_address("glEnable\0"));
            glEnablei=transmute(library.get_proc_address("glEnablei\0"));

            glDisable=transmute(library.get_proc_address("glDisable\0"));
            glDisablei=transmute(library.get_proc_address("glDisablei\0"));

            glIsEnabled=transmute(library.get_proc_address("glIsEnabled\0"));
            glIsEnabledi=transmute(library.get_proc_address("glIsEnabledi\0"));

            glPixelStoref=transmute(library.get_proc_address("glPixelStoref\0"));
            glPixelStorei=transmute(library.get_proc_address("glPixelStorei\0"));

            glClampColor=transmute(library.get_proc_address("glClampColor\0"));

            glViewport=transmute(library.get_proc_address("glViewport\0"));

            glClearColor=transmute(library.get_proc_address("glClearColor\0"));
            glClearDepth=transmute(library.get_proc_address("glClearDepth\0"));
            glClearStencil=transmute(library.get_proc_address("glClearStencil\0"));

            glClear=transmute(library.get_proc_address("glClear\0"));

            glFlush=transmute(library.get_proc_address("glFlush\0"));
            glFinish=transmute(library.get_proc_address("glFinish\0"));

            glGetError=transmute(library.get_proc_address("glGetError\0"));

            glGetString=transmute(library.get_proc_address("glGetString\0"));
            glGetStringi=transmute(library.get_proc_address("glGetStringi\0"));

            glGetMultisamplefv=transmute(library.get_proc_address("glGetMultisamplefv\0"));
        }
    }
}

/// The core capabilities.
/// 
/// `Core::enable` and `Core::disable` enable and disable various capabilities.
/// Use `Core::is_enabled` or `CoreParameterInfo::get*` to determine the current setting of any capability.
/// The initial value for each capability
/// with the exception of `CoreCapability::Dither` and `CoreCapability::Multisample` is `false`.
/// The initial value for `CoreCapability::Dither` and `CoreCapability::Multisample` is `true`.
/// 
/// Any token accepted by `Core::enable` or `Core::disable` is also accepted by `Core::enablei` and `Core::disablei`,
/// but if the capability is not indexed, the maximum value that index may take is zero.
/// 
/// In general, passing an indexed capability to `Core::enable` or `Core::disable`
/// will enable or disable that capability for all indices, resepectively.
impl Core{
    /// Enables server-side GL capabilities.
    /// 
    /// `capability` specifies a symbolic constant indicating a GL capability.
    /// 
    /// `Error::InvalidEnum` is generated if cap is not an accepted value.
    pub unsafe fn enable(capability:CoreCapability){
        glEnable(capability as GLenum)
    }

    /// Enables server-side GL capabilities.
    /// 
    /// `capability` specifies a symbolic constant indicating a GL capability.
    /// 
    /// `index` specifies the index of the capability.
    /// 
    /// `Error::InvalidEnum` is generated if cap is not an accepted value.
    /// 
    /// `Error::InvalidValue` is generated
    /// if `index` is greater than or equal to the number of indexed capabilities for cap.
    pub unsafe fn enablei(capability:IndexedCoreCapability,index:u32){
        glEnablei(capability as GLenum,index)
    }

    /// Disables server-side GL capabilities.
    /// 
    /// `capability` specifies a symbolic constant indicating a GL capability.
    /// 
    /// `Error::InvalidEnum` is generated if cap is not an accepted value.
    pub unsafe fn disable(capability:CoreCapability){
        glDisable(capability as GLenum)
    }

    /// Disables server-side GL capabilities.
    /// 
    /// `capability` specifies a symbolic constant indicating a GL capability.
    /// 
    /// `index` specifies the index of the capability.
    /// 
    /// `Error::InvalidEnum` is generated if cap is not an accepted value.
    /// 
    /// `Error::InvalidValue` is generated
    /// if `index` is greater than or equal to the number of indexed capabilities for cap.
    pub unsafe fn disablei(capability:IndexedCoreCapability,index:u32){
        glDisablei(capability as GLenum,index)
    }

    /// Tests whether a capability is enabled.
    /// 
    /// `capability` specifies a symbolic constant indicating a GL capability.
    /// 
    /// Returns `true` if cap is an enabled capability and returns `false` otherwise.
    /// Boolean states that are indexed may be tested with `Core::is_enablei`.
    /// 
    /// If an error is generated, `Core::is_enable` and `Core::is_enablei` return `false`.
    /// 
    /// `Error::InvalidEnum` is generated if cap is not an accepted value.
    pub unsafe fn is_enabled(capability:CoreCapability)->bool{
        transmute(glIsEnabled(capability as GLenum))
    }

    /// Tests whether a capability is enabled.
    /// 
    /// `capability` specifies a symbolic constant indicating a GL capability.
    /// 
    /// `index` specifies the index of the capability to test.
    /// `index` must be between zero and the count of indexed capabilities for cap.
    /// 
    /// Returns `true` if cap is an enabled capability and returns `false` otherwise.
    /// 
    /// If an error is generated, `Core::is_enable` and `Core::is_enablei` return `false`.
    /// 
    /// `Error::InvalidEnum` is generated if cap is not an accepted value.
    /// 
    /// `Error::InvalidValue` is generated
    /// if `index` is greater than or equal to the number of indexed capabilities for cap.
    pub unsafe fn is_enabledi(capability:IndexedCoreCapability,index:u32)->bool{
        transmute(glIsEnabledi(capability as GLenum,index))
    }
}

#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum PixelStoreParameter{
    PACK_SWAP_BYTES=PACK_SWAP_BYTES,
    PACK_LSB_FIRST=PACK_LSB_FIRST,
    PACK_ROW_LENGTH=PACK_ROW_LENGTH,
    PACK_IMAGE_HEIGHT=PACK_IMAGE_HEIGHT,
    PACK_SKIP_PIXELS=PACK_SKIP_PIXELS,
    PACK_SKIP_ROWS=PACK_SKIP_ROWS,
    PACK_SKIP_IMAGES=PACK_SKIP_IMAGES,
    PACK_ALIGNMENT=PACK_ALIGNMENT,
    UNPACK_SWAP_BYTES=UNPACK_SWAP_BYTES,
    UNPACK_LSB_FIRST=UNPACK_LSB_FIRST,
    UNPACK_ROW_LENGTH=UNPACK_ROW_LENGTH,
    UNPACK_IMAGE_HEIGHT=UNPACK_IMAGE_HEIGHT,
    UNPACK_SKIP_PIXELS=UNPACK_SKIP_PIXELS,
    UNPACK_SKIP_ROWS=UNPACK_SKIP_ROWS,
    UNPACK_SKIP_IMAGES=UNPACK_SKIP_IMAGES,
    UNPACK_ALIGNMENT=UNPACK_ALIGNMENT
}

/// Sets pixel storage modes.
/// 
/// `parameter` specifies the symbolic name of the parameter to be set.
/// 
/// `value` specifies the value that `parameter` is set to.
/// 
/// Sets pixel storage modes that affect the operation of subsequent glReadPixels as well as the unpacking of texture patterns
/// (see glTexImage1D, glTexImage2D, glTexImage3D, glTexSubImage1D, glTexSubImage2D, glTexSubImage3D).
/// 
/// `parameter` is a symbolic constant indicating the parameter to be set, and `value` is the new value.
/// Six of the twelve storage parameters affect how pixel data is returned to client memory.
/// 
/// 
impl Core{
    pub unsafe fn set_pixel_storef(parameter:PixelStoreParameter,value:f32){
        glPixelStoref(parameter as GLenum,value)
    }

    pub unsafe fn set_pixel_storei(parameter:PixelStoreParameter,value:i32){
        glPixelStorei(parameter as GLenum,value)
    }
}

#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum ClampTarget{
    ReadColour=CLAMP_READ_COLOR
}

#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum ClampOption{
    Enabled=TRUE as u32,
    Disabled=FALSE as u32,
    FixedOnly=FIXED_ONLY
}

impl Core{
    /// Specifies whether data read via `glReadPixels` should be clamped.
    /// 
    /// `target` is target for colour clamping.
    /// 
    /// `clamp` specifies whether to apply color clamping.
    /// 
    /// Controls colour clamping that is performed during `glReadPixels`.
    /// `target` must be `ClampTarget::ReadColour`.
    /// If `clamp` is `ClampOption::Enabled`, read colour clamping is enabled;
    /// if `clamp` is `ClampOption::Disabled`, read colour clamping is disabled.
    /// If `clamp` is `ClampOption::FixedOnly`, read colour clamping is enabled
    /// only if the selected read buffer has fixed point components and disabled otherwise.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if `target` is not `ClampTarget::ReadColour`.
    /// if `clamp` is not `ClampOption`.
    pub unsafe fn set_colour_clamping(target:ClampTarget,clamp:ClampOption){
        glClampColor(target as GLenum,clamp as GLenum)
    }

    /// Sets the viewport.
    /// 
    /// Specifies the affine transformation of `x` and `y` from normalized device coordinates to window coordinates.
    /// 
    /// Let (Xnd, Ynd) be normalized device coordinates. Then the window coordinates (Xw, Yw) are computed as follows:
    /// ```
    ///     Xw = (Xnd + 1)(width / 2) + x
    ///     Yw = (Ynd + 1)(height / 2) + y
    /// ```
    /// 
    /// `x`, `y` - Specify the lower left corner of the viewport rectangle, in pixels.
    /// The initial value is `(0i32, 0i32)`.
    /// 
    /// `widht`, `height` - Specify the width and height of the viewport.
    /// When a GL context is first attached to a window,
    /// `width` and `height` are set to the dimensions of that window.
    /// 
    /// Viewport width and height are silently clamped to a range that depends on the implementation.
    /// To query this range, call `glGet` with argument `GL_MAX_VIEWPORT_DIMS`.
    /// 
    /// `GLError::InvalidValue` is generated if either `width` or `height` is negative.
    #[inline(always)]
    pub unsafe fn set_viewport(x:i32,y:i32,width:i32,height:i32){
        glViewport(x,y,width,height)
    }
}

impl Core{
    /// Clears buffers to preset values.
    /// 
    /// `mask` bitwise OR of masks that indicate the buffers to be cleared.
    /// 
    /// Sets the bitplane area of the window to values previously
    /// selected by `Core::set_clear_colour`, `Core::set_clear_depth`, and `Core::set_clear_stencil`.
    /// Multiple colour buffers can be cleared simultaneously
    /// by selecting more than one buffer at a time using glDrawBuffer.
    /// 
    /// The pixel ownership test, the scissor test, dithering,
    /// and the buffer writemasks affect the operation of `Core::clear`.
    /// The scissor box bounds the cleared region.
    /// Alpha function, blend function, logical operation, stenciling,
    /// texture mapping, and depth-buffering are ignored by `Core::clear`.
    /// 
    /// If a buffer is not present, then a `Core::clear` directed at that buffer has no effect.
    /// 
    /// `Error::InvalidValue` is generated if any mask other than the defined ones is set in `mask`.
    #[inline(always)]
    pub unsafe fn clear(mask:ClearMask){
        glClear(mask as GLbitfield)
    }

    /// Specifies clear values for the color buffers.
    /// 
    /// `red`, `green`, `blue`, `alpha` specifies the red, green, blue, and alpha values used when the color buffers are cleared.
    /// The initial values are all 0.
    /// 
    /// Specifies the red, green, blue, and alpha values used by `Core::clear` to clear the color buffers.
    /// Values specified by `Core::clear_colour` are clamped to the range [0,1].
    #[inline(always)]
    pub unsafe fn set_clear_colour(red:f32,green:f32,blue:f32,alpha:f32){
        glClearColor(red,green,blue,alpha)
    }

    /// Specifies the clear value for the depth buffer.
    /// 
    /// `depth` specifies the depth value used when the depth buffer is cleared.
    /// The initial value is 1.
    /// 
    /// Specifies the depth value used by `Core::clear` to clear the depth buffer.
    /// Values specified by `Core::clear_depth` are clamped to the range [0,1].
    #[inline(always)]
    pub unsafe fn set_clear_depth(depth:f64){
        glClearDepth(depth)
    }

    /// Specifies the clear value for the stencil buffer.
    /// 
    /// `stencil` specifies the index used when the stencil buffer is cleared.
    /// The initial value is 0.
    /// 
    /// Specifies the index used by `Core::clear` to clear the stencil buffer.
    /// `stencil` is masked with `2^m − 1`, where `m` is the number of bits in the stencil buffer.
    #[inline(always)]
    pub unsafe fn set_clear_stencil(stencil:i32){
        glClearStencil(stencil)
    }
}

impl Core{
    /// Forces execution of GL commands in finite time.
    /// 
    /// Different GL implementations buffer commands in several different locations,
    /// including network buffers and the graphics accelerator itself.
    /// `Core::flush` empties all of these buffers,
    /// causing all issued commands to be executed as quickly
    /// as they are accepted by the actual rendering engine.
    /// Though this execution may not be completed in any particular time period,
    /// it does complete in finite time.
    /// 
    /// Because any GL program might be executed over a network,
    /// or on an accelerator that buffers commands, all programs should call `Core::flush`
    /// whenever they count on having all of their previously issued commands completed.
    /// For example, call `Core::flush` before waiting for user input that depends on the generated image.
    /// 
    /// `Core::flush` can return at any time.
    /// It does not wait until the execution of all previously issued GL commands is complete.
    #[inline(always)]
    pub unsafe fn flush(){
        glFlush()
    }

    /// Blocks the current thread until all GL execution is complete.
    /// 
    /// Does not return until the effects of all previously called GL commands are complete.
    /// Such effects include all changes to GL state,
    /// all changes to connection state,
    /// and all changes to the frame buffer contents.
    /// 
    /// Requires a round trip to the server.
    #[inline(always)]
    pub unsafe fn finish(){
        glFinish()
    }
}

#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum CoreInfo{
    /// Returns the company responsible for this GL implementation.
    /// This name does not change from release to release.
    Vendor=VENDOR,

    /// Returns the name of the renderer.
    /// This name is typically specific to a particular configuration of a hardware platform.
    /// It does not change from release to release.
    Renderer=RENDERER,

    /// Returns a version or release number.
    Version=VERSION,

    /// Returns a version or release number for the shading language.
    ShadingLanguageVersion=SHADING_LANGUAGE_VERSION,
}

#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum IndexedCoreInfo{
    /// Returns the extension string supported by the implementation at `index`.
    Extensions=EXTENSIONS
}

#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum SampleParameter{
    SamplePosition=SAMPLE_POSITION,
}

impl Core{
    /// Returns the value of the error flag or `None` if no error has been recorded.
    /// 
    /// Each detectable error is assigned a numeric code and symbolic name.
    /// When an error occurs, the error flag is set to the appropriate error code value.
    /// No other errors are recorded until `Error::get_error` is called,
    /// the error code is returned, and the flag is reset to `None`.
    /// If a call to `Error::get_error` returns `None`,
    /// there has been no detectable error since the last call to `Error::get_error`,
    /// or since the GL was initialized.
    /// 
    /// To allow for distributed implementations, there may be several error flags.
    /// If any single error flag has recorded an error, the value of that flag is returned
    /// and that flag is reset to `None` when `Error::get_error` is called.
    /// If more than one flag has recorded an error,
    /// `Error::get_error` returns and clears an arbitrary error flag value.
    /// Thus, `Error::get_error` should always be called in a loop,
    /// until it returns `None`, if all error flags are to be reset.
    /// 
    /// When an error flag is set, results of a GL operation are undefined
    /// only if `Error::OutOfMemory` has occurred.
    /// In all other cases, the command generating the error is ignored
    /// and has no effect on the GL state or frame buffer contents.
    /// If the generating command returns a value, it returns `None`.
    /// If `Error::get_error` itself generates an error, it returns `None`.
    /// 
    /// Initially, all error flags are set to `None`.
    #[inline(always)]
    pub unsafe fn get_error()->Error{
        transmute(glGetError())
    }

    /// Returns a string describing the current GL connection.
    /// 
    /// `name` specifies a symbolic constant.
    /// 
    /// Returns a pointer to a static string describing some aspect of the current GL connection.
    /// 
    /// Strings `CoreInfo::Vendor` and `CoreInfo::Renderer` together uniquely specify a platform.
    /// They do not change from release to release and should be used by platform-recognition algorithms.
    /// 
    /// The `CoreInfo::Version` and `CoreInfo::ShadingLanguageVersion` strings begin with a version number.
    /// The version number uses one of these forms:
    /// `major_number.minor_number major_number.minor_number.release_number`
    /// 
    /// Vendor-specific information may follow the version number.
    /// Its format depends on the implementation,
    /// but a space always separates the version number
    /// and the vendor-specific information.
    /// 
    /// All strings are null-terminated.
    /// 
    /// If an error is generated, `Core::get_string` returns 0.
    /// 
    /// The client and server may support different versions.
    /// `Core::get_string` always returns a compatible version number.
    /// The release number always describes the server.
    /// 
    /// `Error::InvalidEnum` is generated if `name` is not an accepted value.
    pub unsafe fn get_string(name:CoreInfo)->Option<NonNull<u8>>{
        transmute(glGetString(name as GLenum))
    }

    /// Returns a string describing the current GL connection.
    /// 
    /// `name` specifies a symbolic constant.
    /// 
    /// `index` specifies the index of the string to return.
    /// 
    /// Returns a pointer to a static string indexed by `index`.
    /// 
    /// All strings are null-terminated.
    /// 
    /// If an error is generated, `Core::get_stringi` returns 0.
    /// 
    /// `Error::InvalidEnum` is generated if `name` is not an accepted value.
    /// 
    /// `Error::InvalidValue` is generated if `index` is outside the valid range for indexed state name.
    pub unsafe fn get_stringi(name:IndexedCoreInfo,index:u32)->Option<NonNull<u8>>{
        transmute(glGetStringi(name as GLenum,index))
    }

    /// Retrieves the location of a sample.
    /// 
    /// `parameter` specifies the sample parameter name.
    /// 
    /// `index` specifies the index of the sample whose position to query.
    /// 
    /// `values` specifies the address of an array to receive the position of the sample.
    /// 
    /// Queries the location of a given sample.
    /// `name` specifies the sample parameter to retrieve and must be `SampleParameter::SamplePosition`.
    /// `index` corresponds to the sample for which the location should be returned.
    /// The sample location is returned as two floating-point values in val[0] and val[1],
    /// each between 0 and 1, corresponding to the x and y locations respectively in the GL pixel space of that sample.
    /// (0.5, 0.5) this corresponds to the pixel center.
    /// `index` must be between zero and the value of `GL_SAMPLES - 1`.
    /// 
    /// If the multisample mode does not have fixed sample locations,
    /// the returned values may only reflect the locations of samples within some pixels.
    /// 
    /// `Error::InvalidEnum` is generated if `name` is not one `SampleParameter::SamplePosition`.
    /// 
    /// `Error::InvalidValue` is generated if `index` is greater than or equal to the value of `GL_SAMPLES`.
    pub unsafe fn get_multisamplefv(parameter:SampleParameter,index:u32,values:*mut f32){
        glGetMultisamplefv(parameter as GLenum,index,values)
    }
}