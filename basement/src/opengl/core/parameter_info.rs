#[cfg(target_os="windows")]
use crate::winapi::OpenGraphicsLibrary;

use super::{
    types::*,
    constants::*
};

use core::mem::transmute;

#[cfg(target_os="linux")]
extern "system"{
    fn glGetBooleanv(name:GLenum,values:*mut GLboolean);

    fn glGetDoublev(name:GLenum,values:*mut GLdouble);

    fn glGetFloatv(name:GLenum,values:*mut GLfloat);

    fn glGetIntegerv(name:GLenum,values:*mut GLint);

    fn glGetInteger64v(name:GLenum,values:*mut GLint64);

    fn glGetBooleani_v(name:GLenum,index:GLuint,values:*mut GLboolean);

    fn glGetIntegeri_v(name:GLenum,index:GLuint,values:*mut GLint);

    fn glGetInteger64i_v(name:GLenum,index:GLuint,values:*mut GLint64);
}

#[cfg(target_os="windows")]
mod gl{
    pub static mut glGetBooleanv:usize=0;

    pub static mut glGetDoublev:usize=0;

    pub static mut glGetFloatv:usize=0;

    pub static mut glGetIntegerv:usize=0;

    pub static mut glGetInteger64v:usize=0;

    pub static mut glGetBooleani_v:usize=0;

    pub static mut glGetIntegeri_v:usize=0;

    pub static mut glGetInteger64i_v:usize=0;
}

#[cfg(target_os="windows")]
mod gl_functions{
    use super::*;

    pub unsafe fn glGetBooleanv(name:GLenum,values:*mut GLboolean){
        transmute::<usize,extern "system" fn(GLenum,*mut GLboolean)>(gl::glGetBooleanv)(name,values)
    }

    pub unsafe fn glGetDoublev(name:GLenum,values:*mut GLdouble){
        transmute::<usize,extern "system" fn(GLenum,*mut GLdouble)>(gl::glGetDoublev)(name,values)
    }

    pub unsafe fn glGetFloatv(name:GLenum,values:*mut GLfloat){
        transmute::<usize,extern "system" fn(GLenum,*mut GLfloat)>(gl::glGetFloatv)(name,values)
    }

    pub unsafe fn glGetIntegerv(name:GLenum,values:*mut GLint){
        transmute::<usize,extern "system" fn(GLenum,*mut GLint)>(gl::glGetIntegerv)(name,values)
    }

    pub unsafe fn glGetInteger64v(name:GLenum,values:*mut GLint64){
        transmute::<usize,extern "system" fn(GLenum,*mut GLint64)>(gl::glGetInteger64v)(name,values)
    }

    pub unsafe fn glGetBooleani_v(name:GLenum,index:GLuint,values:*mut GLboolean){
        transmute::<usize,extern "system" fn(GLenum,GLuint,*mut GLboolean)>(gl::glGetBooleani_v)(name,index,values)
    }

    pub unsafe fn glGetIntegeri_v(name:GLenum,index:GLuint,values:*mut GLint){
        transmute::<usize,extern "system" fn(GLenum,GLuint,*mut GLint)>(gl::glGetIntegeri_v)(name,index,values)
    }

    pub unsafe fn glGetInteger64i_v(name:GLenum,index:GLuint,values:*mut GLint64){
        transmute::<usize,extern "system" fn(GLenum,GLuint,*mut GLint64)>(gl::glGetInteger64i_v)(name,index,values)
    }
}

#[cfg(target_os="windows")]
use gl_functions::*;

/// Returns the value or values of a selected parameter.
/// 
/// `name` specifies the parameter value to be returned.
/// 
/// `values` returns the value or values of the specified parameter.
/// 
/// `index` specifies the index of the particular element being queried.
/// 
/// These commands return values for simple state variables in GL.
/// `name` is a symbolic constant indicating the state variable to be returned,
/// and `value` is a pointer to an array of the indicated type in which to place the returned data.
/// 
/// Type conversion is performed if `values` has a different type than the state variable value being requested.
/// If `CoreParameterInfo::get_boolean` is called,
/// a floating-point (or integer) value is converted to `false`
/// if and only if it is 0.0 (or 0).
/// Otherwise, it is converted to `false`.
/// If `CoreParameterInfo::get_integer` is called,
/// boolean values are returned as `true` or `false`,
/// and most floating-point values are rounded to the nearest integer value.
/// Floating-point colors and normals, however, are returned with a linear mapping
/// that maps 1.0 to the most positive representable integer value
/// and −1.0 to the most negative representable integer value.
/// If `CoreParameterInfo::get_float` or `CoreParameterInfo::get_double` is called, boolean values are returned
/// as `true` or `false`, and integer values are converted to floating-point values.
/// 
/// `Error::InvalidEnum` is generated if pname is not an accepted value.
/// 
/// `Error::InvalidValue` is generated on any of `CoreParameterInfo::get_booleani`,
/// `CoreParameterInfo::get_integeri`, or `CoreParameterInfo::integer64i`
/// if index is outside of the valid range for the indexed state target.
pub struct CoreParameterInfo;

impl CoreParameterInfo{
    #[cfg(target_os="windows")]
    pub fn load(library:&OpenGraphicsLibrary){
        unsafe{
            use gl::*;

            glGetBooleanv=transmute(library.get_proc_address("glGetBooleanv\0"));

            glGetDoublev=transmute(library.get_proc_address("glGetDoublev\0"));

            glGetFloatv=transmute(library.get_proc_address("glGetFloatv\0"));

            glGetIntegerv=transmute(library.get_proc_address("glGetIntegerv\0"));

            glGetInteger64v=transmute(library.get_proc_address("glGetInteger64v\0"));

            glGetBooleani_v=transmute(library.get_proc_address("glGetBooleani_v\0"));

            glGetIntegeri_v=transmute(library.get_proc_address("glGetIntegeri_v\0"));

            glGetInteger64i_v=transmute(library.get_proc_address("glGetInteger64i_v\0"));
        }
    }
}

impl CoreParameterInfo{
    #[inline(always)]
    pub unsafe fn get_boolean(name:CoreParameter,values:*mut bool){
        glGetBooleanv(name as GLenum,transmute(values))
    }

    #[inline(always)]
    pub unsafe fn get_double(name:CoreParameter,values:*mut f64){
        glGetDoublev(name as GLenum,values)
    }

    #[inline(always)]
    pub unsafe fn get_float(name:CoreParameter,values:*mut f32){
        glGetFloatv(name as GLenum,values)
    }

    #[inline(always)]
    pub unsafe fn get_integer(name:CoreParameter,values:*mut i32){
        glGetIntegerv(name as GLenum,values)
    }

    #[inline(always)]
    pub unsafe fn get_interger64(name:CoreParameter,values:*mut i64){
        glGetInteger64v(name as GLenum,values)
    }

    #[inline(always)]
    pub unsafe fn get_booleani(name:IndexedCoreParameter,index:u32,values:*mut bool){
        glGetBooleani_v(name as GLenum,index,transmute(values))
    }

    #[inline(always)]
    pub unsafe fn get_integeri(name:IndexedCoreParameter,index:u32,values:*mut i32){
        glGetIntegeri_v(name as GLenum,index,values)
    }

    #[inline(always)]
    pub unsafe fn get_integer64i(name:IndexedCoreParameter,index:u32,values:*mut i64){
        glGetInteger64i_v(name as GLenum,index,values)
    }
}

#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum CoreParameter{
    /// Returns a single value indicating the active multitexture unit. The initial value is GL_TEXTURE0.
    /// See glActiveTexture.
    ACTIVE_TEXTURE=ACTIVE_TEXTURE,

    /// Returns a pair of values indicating the range of widths supported for aliased lines.
    /// See glLineWidth.
    ALIASED_LINE_WIDTH_RANGE=ALIASED_LINE_WIDTH_RANGE,

    /// Returns a pair of values indicating the range of widths supported for smooth (antialiased) lines.
    /// See glLineWidth.
    SMOOTH_LINE_WIDTH_RANGE=SMOOTH_LINE_WIDTH_RANGE,

    /// Returns a single value indicating the level of quantization applied to smooth line width parameters.
    SMOOTH_LINE_WIDTH_GRANULARITY=SMOOTH_LINE_WIDTH_GRANULARITY,

    /// Returns a single value, the name of the buffer object currently bound to the target `BufferTarget::ArrayBuffer`.
    /// If no buffer object is bound to this target, 0 is returned.
    /// The initial value is 0.
    /// See `Buffer::bind`.
    ArrayBufferBinding=ARRAY_BUFFER_BINDING,

    /// Returns a single boolean value indicating whether blending is enabled.
    /// The initial value is `false`.
    /// See `Blend::set_function`.
    Blend=BLEND,

    /// Returns four values, the red, green, blue, and alpha values which are the components of the blend color.
    /// See glBlendColor.
    BLEND_COLOR=BLEND_COLOR,

    /// Returns one value, the symbolic constant identifying the alpha destination blend function. The initial value is GL_ZERO. See glBlendFunc and glBlendFuncSeparate.
    BLEND_DST_ALPHA=BLEND_DST_ALPHA,

    /// Returns one value, the symbolic constant identifying the RGB destination blend function. The initial value is GL_ZERO. See glBlendFunc and glBlendFuncSeparate.
    BLEND_DST_RGB=BLEND_DST_RGB,

    /// Returns one value, a symbolic constant indicating whether the RGB blend equation is GL_FUNC_ADD, GL_FUNC_SUBTRACT, GL_FUNC_REVERSE_SUBTRACT, GL_MIN or GL_MAX. See glBlendEquationSeparate.
    BLEND_EQUATION_RGB=BLEND_EQUATION_RGB,

    /// Returns one value, a symbolic constant indicating whether the Alpha blend equation is GL_FUNC_ADD, GL_FUNC_SUBTRACT, GL_FUNC_REVERSE_SUBTRACT, GL_MIN or GL_MAX. See glBlendEquationSeparate.
    BLEND_EQUATION_ALPHA=BLEND_EQUATION_ALPHA,

    /// Returns one value, the symbolic constant identifying the alpha source blend function. The initial value is GL_ONE. See glBlendFunc and glBlendFuncSeparate.
    BLEND_SRC_ALPHA=BLEND_SRC_ALPHA,

    /// Returns one value, the symbolic constant identifying the RGB source blend function. The initial value is GL_ONE. See glBlendFunc and glBlendFuncSeparate.
    BLEND_SRC_RGB=BLEND_SRC_RGB,

    /// Returns four values: the red, green, blue, and alpha values used to clear the color buffers. Integer values, if requested, are linearly mapped from the internal floating-point representation such that 1.0 returns the most positive representable integer value, and −1.0 returns the most negative representable integer value. The initial value is (0, 0, 0, 0). See glClearColor.
    COLOR_CLEAR_VALUE=COLOR_CLEAR_VALUE,

    /// Returns a single boolean value indicating whether a fragment's RGBA color values are merged into the framebuffer using a logical operation. The initial value is GL_FALSE. See glLogicOp.
    COLOR_LOGIC_OP=COLOR_LOGIC_OP,

    /// Returns four boolean values: the red, green, blue, and alpha write enables for the color buffers. The initial value is (GL_TRUE, GL_TRUE, GL_TRUE, GL_TRUE). See glColorMask.
    COLOR_WRITEMASK=COLOR_WRITEMASK,

    /// Returns a list of symbolic constants of length GL_NUM_COMPRESSED_TEXTURE_FORMATS indicating which compressed texture formats are available. See glCompressedTexImage2D.
    COMPRESSED_TEXTURE_FORMATS=COMPRESSED_TEXTURE_FORMATS,

    /// Returns a single boolean value indicating whether polygon culling is enabled. The initial value is GL_FALSE. See glCullFace.
    CULL_FACE=CULL_FACE,

    /// Returns one value, the name of the program object that is currently active, or 0 if no program object is active. See glUseProgram.
    CURRENT_PROGRAM=CURRENT_PROGRAM,

    /// Returns one value, the value that is used to clear the depth buffer. Integer values, if requested, are linearly mapped from the internal floating-point representation such that 1.0 returns the most positive representable integer value, and −1.0 returns the most negative representable integer value. The initial value is 1. See glClearDepth.
    DEPTH_CLEAR_VALUE=DEPTH_CLEAR_VALUE,

    /// Returns one value, the symbolic constant that indicates the depth comparison function. The initial value is GL_LESS. See glDepthFunc.
    DEPTH_FUNC=DEPTH_FUNC,

    /// Returns two values: the near and far mapping limits for the depth buffer. Integer values, if requested, are linearly mapped from the internal floating-point representation such that 1.0 returns the most positive representable integer value, and −1.0 returns the most negative representable integer value. The initial value is (0, 1). See glDepthRange.
    DEPTH_RANGE=DEPTH_RANGE,

    /// Returns a single boolean value indicating whether depth testing of fragments is enabled. The initial value is GL_FALSE. See glDepthFunc and glDepthRange.
    DEPTH_TEST=DEPTH_TEST,

    /// Returns a single boolean value indicating if the depth buffer is enabled for writing. The initial value is GL_TRUE. See glDepthMask.
    DEPTH_WRITEMASK=DEPTH_WRITEMASK,

    /// Returns a single boolean value indicating whether dithering of fragment colors and indices is enabled.
    /// The initial value is `true`.
    Dither=DITHER,

    /// Returns a single boolean value indicating whether double buffering is supported.
    Doublebuffer=DOUBLEBUFFER,

    /// Returns one value, a symbolic constant indicating which buffers are being drawn to.
    /// See glDrawBuffer.
    /// The initial value is GL_BACK if there are back buffers, otherwise it is GL_FRONT.
    DRAW_BUFFER=DRAW_BUFFER,

    /// Returns one value, the name of the framebuffer object currently bound to the GL_DRAW_FRAMEBUFFER target. If the default framebuffer is bound, this value will be zero. The initial value is zero. See glBindFramebuffer.
    DRAW_FRAMEBUFFER_BINDING=DRAW_FRAMEBUFFER_BINDING,

    /// Returns one value, the name of the framebuffer object currently bound to the GL_READ_FRAMEBUFFER target. If the default framebuffer is bound, this value will be zero. The initial value is zero. See glBindFramebuffer.
    READ_FRAMEBUFFER_BINDING=READ_FRAMEBUFFER_BINDING,

    /// Returns a single value, the name of the buffer object currently bound to the target GL_ELEMENT_ARRAY_BUFFER. If no buffer object is bound to this target, 0 is returned. The initial value is 0. See glBindBuffer.
    ELEMENT_ARRAY_BUFFER_BINDING=ELEMENT_ARRAY_BUFFER_BINDING,

    /// Returns a single value, the name of the renderbuffer object currently bound to the target GL_RENDERBUFFER. If no renderbuffer object is bound to this target, 0 is returned. The initial value is 0. See glBindRenderbuffer.
    RENDERBUFFER_BINDING=RENDERBUFFER_BINDING,

    /// Returns one value, a symbolic constant indicating the mode of the derivative accuracy hint for fragment shaders. The initial value is GL_DONT_CARE. See glHint.
    FRAGMENT_SHADER_DERIVATIVE_HINT=FRAGMENT_SHADER_DERIVATIVE_HINT,

    /// Returns a single boolean value indicating whether antialiasing of lines is enabled. The initial value is GL_FALSE. See glLineWidth.
    LINE_SMOOTH=LINE_SMOOTH,

    /// Returns one value, a symbolic constant indicating the mode of the line antialiasing hint. The initial value is GL_DONT_CARE. See glHint.
    LINE_SMOOTH_HINT=LINE_SMOOTH_HINT,

    /// Returns one value, the line width as specified with glLineWidth. The initial value is 1.
    LINE_WIDTH=LINE_WIDTH,

    /// Returns one value, a symbolic constant indicating the selected logic operation mode. The initial value is GL_COPY. See glLogicOp.
    LOGIC_OP_MODE=LOGIC_OP_MODE,

    /// Returns one value, a rough estimate of the largest 3D texture that the GL can handle. The value must be at least 64. Use GL_PROXY_TEXTURE_3D to determine if a texture is too large. See glTexImage3D.
    MAX_3D_TEXTURE_SIZE=MAX_3D_TEXTURE_SIZE,

    /// Returns one value, the maximum number of application-defined clipping distances. The value must be at least 8.
    MAX_CLIP_DISTANCES=MAX_CLIP_DISTANCES,

    /// Returns one value, the number of words for fragment shader uniform variables in all uniform blocks (including default). The value must be at least 1. See glUniform.
    MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS=MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS,

    /// Returns one value, the maximum supported texture image units that can be used to access texture maps from the vertex shader and the fragment processor combined. If both the vertex shader and the fragment processing stage access the same texture image unit, then that counts as using two texture image units against this limit. The value must be at least 48. See glActiveTexture.
    MAX_COMBINED_TEXTURE_IMAGE_UNITS=MAX_COMBINED_TEXTURE_IMAGE_UNITS,

    /// Returns one value, the number of words for vertex shader uniform variables in all uniform blocks (including default). The value must be at least 1. See glUniform.
    MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS=MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS,

    /// Returns one value, the number of words for geometry shader uniform variables in all uniform blocks (including default). The value must be at least 1. See glUniform.
    MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS=MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS,

    /// Returns one value, the maximum number of uniform blocks per program. The value must be at least 36. See glUniformBlockBinding.
    MAX_COMBINED_UNIFORM_BLOCKS=MAX_COMBINED_UNIFORM_BLOCKS,

    /// Returns one value. The value gives a rough estimate of the largest cube-map texture that the GL can handle. The value must be at least 1024. Use GL_PROXY_TEXTURE_CUBE_MAP to determine if a texture is too large. See glTexImage2D.
    MAX_CUBE_MAP_TEXTURE_SIZE=MAX_CUBE_MAP_TEXTURE_SIZE,

    /// Returns one value, the maximum number of simultaneous outputs that may be written in a fragment shader. The value must be at least 8. See glDrawBuffers.
    MAX_DRAW_BUFFERS=MAX_DRAW_BUFFERS,

    /// Returns one value, the maximum number of active draw buffers when using dual-source blending. The value must be at least 1. See glBlendFunc and glBlendFuncSeparate.
    MAX_DUAL_SOURCE_DRAW_BUFFERS=MAX_DUAL_SOURCE_DRAW_BUFFERS,

    /// Returns one value, the recommended maximum number of vertex array indices. See glDrawRangeElements.
    MAX_ELEMENTS_INDICES=MAX_ELEMENTS_INDICES,

    /// Returns one value, the recommended maximum number of vertex array vertices. See glDrawRangeElements.
    MAX_ELEMENTS_VERTICES=MAX_ELEMENTS_VERTICES,

    /// Returns one value, the maximum number of individual floating-point, integer, or boolean values that can be held in uniform variable storage for a fragment shader. The value must be at least 1024. See glUniform.
    MAX_FRAGMENT_UNIFORM_COMPONENTS=MAX_FRAGMENT_UNIFORM_COMPONENTS,

    /// Returns one value, the maximum number of uniform blocks per fragment shader. The value must be at least 12. See glUniformBlockBinding.
    MAX_FRAGMENT_UNIFORM_BLOCKS=MAX_FRAGMENT_UNIFORM_BLOCKS,

    /// Returns one value, the maximum number of components of the inputs read by the fragment shader, which must be at least 128.
    MAX_FRAGMENT_INPUT_COMPONENTS=MAX_FRAGMENT_INPUT_COMPONENTS,

    /// Returns one value, the minimum texel offset allowed in a texture lookup, which must be at most -8.
    MIN_PROGRAM_TEXEL_OFFSET=MIN_PROGRAM_TEXEL_OFFSET,

    /// Returns one value, the maximum texel offset allowed in a texture lookup, which must be at least 7.
    MAX_PROGRAM_TEXEL_OFFSET=MAX_PROGRAM_TEXEL_OFFSET,

    /// Returns one value. The value gives a rough estimate of the largest rectangular texture that the GL can handle. The value must be at least 1024. Use GL_PROXY_TEXTURE_RECTANGLE to determine if a texture is too large. See glTexImage2D.
    MAX_RECTANGLE_TEXTURE_SIZE=MAX_RECTANGLE_TEXTURE_SIZE,

    /// Returns one value, the maximum supported texture image units that can be used to access texture maps from the fragment shader. The value must be at least 16. See glActiveTexture.
    MAX_TEXTURE_IMAGE_UNITS=MAX_TEXTURE_IMAGE_UNITS,

    /// Returns one value, the maximum, absolute value of the texture level-of-detail bias. The value must be at least 2.0.
    MAX_TEXTURE_LOD_BIAS=MAX_TEXTURE_LOD_BIAS,

    /// Returns one value. The value gives a rough estimate of the largest texture that the GL can handle. The value must be at least 1024. Use a proxy texture target such as GL_PROXY_TEXTURE_1D or GL_PROXY_TEXTURE_2D to determine if a texture is too large. See glTexImage1D and glTexImage2D.
    MAX_TEXTURE_SIZE=MAX_TEXTURE_SIZE,

    /// Returns one value. The value indicates the maximum supported size for renderbuffers. See glFramebufferRenderbuffer.
    MAX_RENDERBUFFER_SIZE=MAX_RENDERBUFFER_SIZE,

    /// Returns one value. The value indicates the maximum number of layers allowed in an array texture, and must be at least 256. See glTexImage2D.
    MAX_ARRAY_TEXTURE_LAYERS=MAX_ARRAY_TEXTURE_LAYERS,

    /// Returns one value. The value gives the maximum number of texels allowed in the texel array of a texture buffer object. Value must be at least 65536.
    MAX_TEXTURE_BUFFER_SIZE=MAX_TEXTURE_BUFFER_SIZE,

    /// Returns one value, the maximum size in basic machine units of a uniform block. The value must be at least 16384. See glUniformBlockBinding.
    MAX_UNIFORM_BLOCK_SIZE=MAX_UNIFORM_BLOCK_SIZE,

    /// Returns one value,
    /// the maximum number of interpolators available for processing varying variables
    /// used by vertex and fragment shaders.
    /// This value represents the number of individual floating-point values that can be interpolated;
    /// varying variables declared as vectors, matrices, and arrays will all consume multiple interpolators.
    /// The value must be at least 32.
    MAX_VARYING_FLOATS=MAX_VARYING_FLOATS,

    /// Returns one value, the maximum number of 4-component generic vertex attributes accessible to a vertex shader. The value must be at least 16. See glVertexAttrib.
    MAX_VERTEX_ATTRIBS=MAX_VERTEX_ATTRIBS,

    /// Returns one value, the maximum supported texture image units that can be used to access texture maps from the vertex shader. The value may be at least 16. See glActiveTexture.
    MAX_VERTEX_TEXTURE_IMAGE_UNITS=MAX_VERTEX_TEXTURE_IMAGE_UNITS,

    /// Returns one value, the maximum supported texture image units that can be used to access texture maps from the geometry shader. The value must be at least 16. See glActiveTexture.
    MAX_GEOMETRY_TEXTURE_IMAGE_UNITS=MAX_GEOMETRY_TEXTURE_IMAGE_UNITS,

    /// Returns one value, the maximum number of individual floating-point, integer, or boolean values that can be held in uniform variable storage for a vertex shader. The value must be at least 1024. See glUniform.
    MAX_VERTEX_UNIFORM_COMPONENTS=MAX_VERTEX_UNIFORM_COMPONENTS,

    /// Returns one value, the maximum number of components of output written by a vertex shader, which must be at least 64.
    MAX_VERTEX_OUTPUT_COMPONENTS=MAX_VERTEX_OUTPUT_COMPONENTS,

    /// Returns one value, the maximum number of individual floating-point, integer, or boolean values that can be held in uniform variable storage for a geometry shader. The value must be at least 1024. See glUniform.
    MAX_GEOMETRY_UNIFORM_COMPONENTS=MAX_GEOMETRY_UNIFORM_COMPONENTS,

    /// Returns one value, the maximum number of sample mask words.
    MAX_SAMPLE_MASK_WORDS=MAX_SAMPLE_MASK_WORDS,

    /// Returns one value, the maximum number of samples in a color multisample texture.
    MAX_COLOR_TEXTURE_SAMPLES=MAX_COLOR_TEXTURE_SAMPLES,

    /// Returns one value, the maximum number of samples in a multisample depth or depth-stencil texture.
    MAX_DEPTH_TEXTURE_SAMPLES=MAX_DEPTH_TEXTURE_SAMPLES,

    /// Returns one value, the maximum number of samples supported in integer format multisample buffers.
    MAX_INTEGER_SAMPLES=MAX_INTEGER_SAMPLES,

    /// Returns one value, the maximum glWaitSync timeout interval.
    MAX_SERVER_WAIT_TIMEOUT=MAX_SERVER_WAIT_TIMEOUT,

    /// Returns one value, the maximum number of uniform buffer binding points on the context, which must be at least 36.
    MAX_UNIFORM_BUFFER_BINDINGS=MAX_UNIFORM_BUFFER_BINDINGS,

    /// Returns one value, the maximum number of uniform blocks per vertex shader. The value must be at least 12. See glUniformBlockBinding.
    MAX_VERTEX_UNIFORM_BLOCKS=MAX_VERTEX_UNIFORM_BLOCKS,

    /// Returns one value, the maximum number of uniform blocks per geometry shader. The value must be at least 12. See glUniformBlockBinding.
    MAX_GEOMETRY_UNIFORM_BLOCKS=MAX_GEOMETRY_UNIFORM_BLOCKS,

    /// Returns one value, the maximum number of components of inputs read by a geometry shader, which must be at least 64.
    MAX_GEOMETRY_INPUT_COMPONENTS=MAX_GEOMETRY_INPUT_COMPONENTS,

    /// Returns one value, the maximum number of components of outputs written by a geometry shader, which must be at least 128.
    MAX_GEOMETRY_OUTPUT_COMPONENTS=MAX_GEOMETRY_OUTPUT_COMPONENTS,

    /// Returns two values: the maximum supported width and height of the viewport. These must be at least as large as the visible dimensions of the display being rendered to. See glViewport.
    MAX_VIEWPORT_DIMS=MAX_VIEWPORT_DIMS,

    /// Returns a single integer value indicating the number of available compressed texture formats. The minimum value is 4. See glCompressedTexImage2D.
    NUM_COMPRESSED_TEXTURE_FORMATS=NUM_COMPRESSED_TEXTURE_FORMATS,

    /// Returns one value, the byte alignment used for writing pixel data to memory. The initial value is 4. See glPixelStore.
    PACK_ALIGNMENT=PACK_ALIGNMENT,

    /// Returns one value, the image height used for writing pixel data to memory. The initial value is 0. See glPixelStore.
    PACK_IMAGE_HEIGHT=PACK_IMAGE_HEIGHT,

    /// Returns a single boolean value indicating whether single-bit pixels being written to memory are written first to the least significant bit of each unsigned byte. The initial value is GL_FALSE. See glPixelStore.
    PACK_LSB_FIRST=PACK_LSB_FIRST,

    /// Returns one value, the row length used for writing pixel data to memory. The initial value is 0. See glPixelStore.
    PACK_ROW_LENGTH=PACK_ROW_LENGTH,

    /// Returns one value, the number of pixel images skipped before the first pixel is written into memory. The initial value is 0. See glPixelStore.
    PACK_SKIP_IMAGES=PACK_SKIP_IMAGES,

    /// Returns one value, the number of pixel locations skipped before the first pixel is written into memory. The initial value is 0. See glPixelStore.
    PACK_SKIP_PIXELS=PACK_SKIP_PIXELS,

    /// Returns one value, the number of rows of pixel locations skipped before the first pixel is written into memory. The initial value is 0. See glPixelStore.
    PACK_SKIP_ROWS=PACK_SKIP_ROWS,

    /// Returns a single boolean value indicating whether the bytes of two-byte and four-byte pixel indices and components are swapped before being written to memory. The initial value is GL_FALSE. See glPixelStore.
    PACK_SWAP_BYTES=PACK_SWAP_BYTES,

    /// Returns a single value, the name of the buffer object currently bound to the target GL_PIXEL_PACK_BUFFER. If no buffer object is bound to this target, 0 is returned. The initial value is 0. See glBindBuffer.
    PIXEL_PACK_BUFFER_BINDING=PIXEL_PACK_BUFFER_BINDING,

    /// Returns a single value, the name of the buffer object currently bound to the target GL_PIXEL_UNPACK_BUFFER. If no buffer object is bound to this target, 0 is returned. The initial value is 0. See glBindBuffer.
    PIXEL_UNPACK_BUFFER_BINDING=PIXEL_UNPACK_BUFFER_BINDING,

    /// Returns one value, the point size threshold for determining the point size. See glPointParameter.
    POINT_FADE_THRESHOLD_SIZE=POINT_FADE_THRESHOLD_SIZE,

    /// Returns one value, the current primitive restart index. The initial value is 0. See glPrimitiveRestartIndex.
    PRIMITIVE_RESTART_INDEX=PRIMITIVE_RESTART_INDEX,

    /// Returns a single boolean value indicating whether vertex program point size mode is enabled. If enabled, then the point size is taken from the shader built-in gl_PointSize. If disabled, then the point size is taken from the point state as specified by glPointSize. The initial value is GL_FALSE.
    PROGRAM_POINT_SIZE=PROGRAM_POINT_SIZE,

    /// Returns one value, the currently selected provoking vertex convention. The initial value is GL_LAST_VERTEX_CONVENTION. See glProvokingVertex.
    PROVOKING_VERTEX=PROVOKING_VERTEX,

    /// Returns one value, the point size as specified by glPointSize. The initial value is 1.
    POINT_SIZE=POINT_SIZE,

    /// Returns one value, the size difference between adjacent supported sizes for antialiased points. See glPointSize.
    POINT_SIZE_GRANULARITY=POINT_SIZE_GRANULARITY,

    /// Returns two values: the smallest and largest supported sizes for antialiased points. The smallest size must be at most 1, and the largest size must be at least 1. See glPointSize.
    POINT_SIZE_RANGE=POINT_SIZE_RANGE,

    /// Returns one value, the scaling factor used to determine the variable offset that is added to the depth value of each fragment generated when a polygon is rasterized. The initial value is 0. See glPolygonOffset.
    POLYGON_OFFSET_FACTOR=POLYGON_OFFSET_FACTOR,

    /// Returns one value. This value is multiplied by an implementation-specific value and then added to the depth value of each fragment generated when a polygon is rasterized. The initial value is 0. See glPolygonOffset.
    POLYGON_OFFSET_UNITS=POLYGON_OFFSET_UNITS,

    /// Returns a single boolean value indicating whether polygon offset is enabled for polygons in fill mode. The initial value is GL_FALSE. See glPolygonOffset.
    POLYGON_OFFSET_FILL=POLYGON_OFFSET_FILL,

    /// Returns a single boolean value indicating whether polygon offset is enabled for polygons in line mode. The initial value is GL_FALSE. See glPolygonOffset.
    POLYGON_OFFSET_LINE=POLYGON_OFFSET_LINE,

    /// Returns a single boolean value indicating whether polygon offset is enabled for polygons in point mode. The initial value is GL_FALSE. See glPolygonOffset.
    POLYGON_OFFSET_POINT=POLYGON_OFFSET_POINT,

    /// Returns a single boolean value indicating whether antialiasing of polygons is enabled. The initial value is GL_FALSE. See glPolygonMode.
    POLYGON_SMOOTH=POLYGON_SMOOTH,

    /// Returns one value, a symbolic constant indicating the mode of the polygon antialiasing hint. The initial value is GL_DONT_CARE. See glHint.
    POLYGON_SMOOTH_HINT=POLYGON_SMOOTH_HINT,

    /// Returns one value, a symbolic constant indicating which color buffer is selected for reading. The initial value is GL_BACK if there is a back buffer, otherwise it is GL_FRONT. See glReadPixels.
    READ_BUFFER=READ_BUFFER,

    /// Returns a single integer value indicating the number of sample buffers associated with the framebuffer. See glSampleCoverage.
    SAMPLE_BUFFERS=SAMPLE_BUFFERS,

    /// Returns a single positive floating-point value indicating the current sample coverage value. See glSampleCoverage.
    SAMPLE_COVERAGE_VALUE=SAMPLE_COVERAGE_VALUE,

    /// Returns a single boolean value indicating if the temporary coverage value should be inverted. See glSampleCoverage.
    SAMPLE_COVERAGE_INVERT=SAMPLE_COVERAGE_INVERT,

    /// Returns a single value, the name of the sampler object currently bound to the active texture unit. The initial value is 0. See glBindSampler.
    SAMPLER_BINDING=SAMPLER_BINDING,

    /// Returns a single integer value indicating the coverage mask size. See glSampleCoverage.
    SAMPLES=SAMPLES,

    /// Returns four values: the x and y window coordinates of the scissor box, followed by its width and height. Initially the x and y window coordinates are both 0 and the width and height are set to the size of the window. See glScissor.
    SCISSOR_BOX=SCISSOR_BOX,

    /// Returns a single boolean value indicating whether scissoring is enabled. The initial value is GL_FALSE. See glScissor.
    SCISSOR_TEST=SCISSOR_TEST,

    /// Returns one value, a symbolic constant indicating what action is taken for back-facing polygons when the stencil test fails. The initial value is GL_KEEP. See glStencilOpSeparate.
    STENCIL_BACK_FAIL=STENCIL_BACK_FAIL,

    /// Returns one value, a symbolic constant indicating what function is used for back-facing polygons to compare the stencil reference value with the stencil buffer value. The initial value is GL_ALWAYS. See glStencilFuncSeparate.
    STENCIL_BACK_FUNC=STENCIL_BACK_FUNC,

    /// Returns one value, a symbolic constant indicating what action is taken for back-facing polygons when the stencil test passes, but the depth test fails. The initial value is GL_KEEP. See glStencilOpSeparate.
    STENCIL_BACK_PASS_DEPTH_FAIL=STENCIL_BACK_PASS_DEPTH_FAIL,

    /// Returns one value, a symbolic constant indicating what action is taken for back-facing polygons when the stencil test passes and the depth test passes. The initial value is GL_KEEP. See glStencilOpSeparate.
    STENCIL_BACK_PASS_DEPTH_PASS=STENCIL_BACK_PASS_DEPTH_PASS,

    /// Returns one value, the reference value that is compared with the contents of the stencil buffer for back-facing polygons. The initial value is 0. See glStencilFuncSeparate.
    STENCIL_BACK_REF=STENCIL_BACK_REF,

    /// Returns one value, the mask that is used for back-facing polygons to mask both the stencil reference value and the stencil buffer value before they are compared. The initial value is all 1's. See glStencilFuncSeparate.
    STENCIL_BACK_VALUE_MASK=STENCIL_BACK_VALUE_MASK,

    /// Returns one value, the mask that controls writing of the stencil bitplanes for back-facing polygons. The initial value is all 1's. See glStencilMaskSeparate.
    STENCIL_BACK_WRITEMASK=STENCIL_BACK_WRITEMASK,

    /// Returns one value, the index to which the stencil bitplanes are cleared. The initial value is 0. See glClearStencil.
    STENCIL_CLEAR_VALUE=STENCIL_CLEAR_VALUE,

    /// Returns one value, a symbolic constant indicating what action is taken when the stencil test fails. The initial value is GL_KEEP. See glStencilOp. This stencil state only affects non-polygons and front-facing polygons. Back-facing polygons use separate stencil state. See glStencilOpSeparate.
    STENCIL_FAIL=STENCIL_FAIL,

    /// Returns one value, a symbolic constant indicating what function is used to compare the stencil reference value with the stencil buffer value. The initial value is GL_ALWAYS. See glStencilFunc. This stencil state only affects non-polygons and front-facing polygons. Back-facing polygons use separate stencil state. See glStencilFuncSeparate.
    STENCIL_FUNC=STENCIL_FUNC,

    /// Returns one value, a symbolic constant indicating what action is taken when the stencil test passes, but the depth test fails. The initial value is GL_KEEP. See glStencilOp. This stencil state only affects non-polygons and front-facing polygons. Back-facing polygons use separate stencil state. See glStencilOpSeparate.
    STENCIL_PASS_DEPTH_FAIL=STENCIL_PASS_DEPTH_FAIL,

    /// Returns one value, a symbolic constant indicating what action is taken when the stencil test passes and the depth test passes. The initial value is GL_KEEP. See glStencilOp. This stencil state only affects non-polygons and front-facing polygons. Back-facing polygons use separate stencil state. See glStencilOpSeparate.
    STENCIL_PASS_DEPTH_PASS=STENCIL_PASS_DEPTH_PASS,

    /// Returns one value, the reference value that is compared with the contents of the stencil buffer. The initial value is 0. See glStencilFunc. This stencil state only affects non-polygons and front-facing polygons. Back-facing polygons use separate stencil state. See glStencilFuncSeparate.
    STENCIL_REF=STENCIL_REF,

    /// Returns a single boolean value indicating whether stencil testing of fragments is enabled. The initial value is GL_FALSE. See glStencilFunc and glStencilOp.
    STENCIL_TEST=STENCIL_TEST,

    /// Returns one value, the mask that is used to mask both the stencil reference value and the stencil buffer value before they are compared. The initial value is all 1's. See glStencilFunc. This stencil state only affects non-polygons and front-facing polygons. Back-facing polygons use separate stencil state. See glStencilFuncSeparate.
    STENCIL_VALUE_MASK=STENCIL_VALUE_MASK,

    /// Returns one value, the mask that controls writing of the stencil bitplanes. The initial value is all 1's. See glStencilMask. This stencil state only affects non-polygons and front-facing polygons. Back-facing polygons use separate stencil state. See glStencilMaskSeparate.
    STENCIL_WRITEMASK=STENCIL_WRITEMASK,

    /// Returns a single boolean value indicating whether stereo buffers (left and right) are supported.
    STEREO=STEREO,

    /// Returns one value, an estimate of the number of bits of subpixel resolution that are used to position rasterized geometry in window coordinates. The value must be at least 4.
    SUBPIXEL_BITS=SUBPIXEL_BITS,

    /// Returns a single value, the name of the texture currently bound to the target GL_TEXTURE_1D. The initial value is 0. See glBindTexture.
    TEXTURE_BINDING_1D=TEXTURE_BINDING_1D,

    /// Returns a single value, the name of the texture currently bound to the target GL_TEXTURE_1D_ARRAY. The initial value is 0. See glBindTexture.
    TEXTURE_BINDING_1D_ARRAY=TEXTURE_BINDING_1D_ARRAY,

    /// Returns a single value, the name of the texture currently bound to the target GL_TEXTURE_2D. The initial value is 0. See glBindTexture.
    TEXTURE_BINDING_2D=TEXTURE_BINDING_2D,

    /// Returns a single value, the name of the texture currently bound to the target GL_TEXTURE_2D_ARRAY. The initial value is 0. See glBindTexture.
    TEXTURE_BINDING_2D_ARRAY=TEXTURE_BINDING_2D_ARRAY,

    /// Returns a single value, the name of the texture currently bound to the target GL_TEXTURE_2D_MULTISAMPLE. The initial value is 0. See glBindTexture.
    TEXTURE_BINDING_2D_MULTISAMPLE=TEXTURE_BINDING_2D_MULTISAMPLE,

    /// Returns a single value, the name of the texture currently bound to the target GL_TEXTURE_2D_MULTISAMPLE_ARRAY. The initial value is 0. See glBindTexture.
    TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY=TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY,

    /// Returns a single value, the name of the texture currently bound to the target GL_TEXTURE_3D. The initial value is 0. See glBindTexture.
    TEXTURE_BINDING_3D=TEXTURE_BINDING_3D,

    /// Returns a single value, the name of the texture currently bound to the target GL_TEXTURE_BUFFER. The initial value is 0. See glBindTexture.
    TEXTURE_BINDING_BUFFER=TEXTURE_BINDING_BUFFER,

    /// Returns a single value, the name of the texture currently bound to the target GL_TEXTURE_CUBE_MAP. The initial value is 0. See glBindTexture.
    TEXTURE_BINDING_CUBE_MAP=TEXTURE_BINDING_CUBE_MAP,

    /// Returns a single value, the name of the texture currently bound to the target GL_TEXTURE_RECTANGLE. The initial value is 0. See glBindTexture.
    TEXTURE_BINDING_RECTANGLE=TEXTURE_BINDING_RECTANGLE,

    /// Returns a single value indicating the mode of the texture compression hint. The initial value is GL_DONT_CARE.
    TEXTURE_COMPRESSION_HINT=TEXTURE_COMPRESSION_HINT,

    /// Returns a single value, the 64-bit value of the current GL time. See glQueryCounter.
    TIMESTAMP=TIMESTAMP,

    /// When used with non-indexed variants of glGet (such as glGetIntegerv), returns a single value, the name of the buffer object currently bound to the target GL_TRANSFORM_FEEDBACK_BUFFER. If no buffer object is bound to this target, 0 is returned. When used with indexed variants of glGet (such as glGetIntegeri_v), returns a single value, the name of the buffer object bound to the indexed transform feedback attribute stream. The initial value is 0 for all targets. See glBindBuffer, glBindBufferBase, and glBindBufferRange.
    TRANSFORM_FEEDBACK_BUFFER_BINDING=TRANSFORM_FEEDBACK_BUFFER_BINDING,

    /// When used with indexed variants of glGet (such as glGetInteger64i_v), returns a single value, the start offset of the binding range for each transform feedback attribute stream. The initial value is 0 for all streams. See glBindBufferRange.
    TRANSFORM_FEEDBACK_BUFFER_START=TRANSFORM_FEEDBACK_BUFFER_START,

    /// When used with indexed variants of glGet (such as glGetInteger64i_v), returns a single value, the size of the binding range for each transform feedback attribute stream. The initial value is 0 for all streams. See glBindBufferRange.
    TRANSFORM_FEEDBACK_BUFFER_SIZE=TRANSFORM_FEEDBACK_BUFFER_SIZE,

    /// When used with non-indexed variants of glGet (such as glGetIntegerv), returns a single value, the name of the buffer object currently bound to the target GL_UNIFORM_BUFFER. If no buffer object is bound to this target, 0 is returned. When used with indexed variants of glGet (such as glGetIntegeri_v), returns a single value, the name of the buffer object bound to the indexed uniform buffer binding point. The initial value is 0 for all targets. See glBindBuffer, glBindBufferBase, and glBindBufferRange.
    UNIFORM_BUFFER_BINDING=UNIFORM_BUFFER_BINDING,

    /// When used with indexed variants of glGet (such as glGetInteger64i_v), returns a single value, the start offset of the binding range for each indexed uniform buffer binding. The initial value is 0 for all bindings. See glBindBufferRange.
    UNIFORM_BUFFER_START=UNIFORM_BUFFER_START,

    /// When used with indexed variants of glGet (such as glGetInteger64i_v), returns a single value, the size of the binding range for each indexed uniform buffer binding. The initial value is 0 for all bindings. See glBindBufferRange.
    UNIFORM_BUFFER_SIZE=UNIFORM_BUFFER_SIZE,

    /// Returns a single value, the minimum required alignment for uniform buffer sizes and offset. The initial value is 1. See glUniformBlockBinding.
    UNIFORM_BUFFER_OFFSET_ALIGNMENT=UNIFORM_BUFFER_OFFSET_ALIGNMENT,

    /// Returns one value, the byte alignment used for reading pixel data from memory. The initial value is 4. See glPixelStore.
    UNPACK_ALIGNMENT=UNPACK_ALIGNMENT,

    /// Returns one value, the image height used for reading pixel data from memory. The initial is 0. See glPixelStore.
    UNPACK_IMAGE_HEIGHT=UNPACK_IMAGE_HEIGHT,

    /// Returns a single boolean value indicating whether single-bit pixels being read from memory are read first from the least significant bit of each unsigned byte. The initial value is GL_FALSE. See glPixelStore.
    UNPACK_LSB_FIRST=UNPACK_LSB_FIRST,

    /// Returns one value, the row length used for reading pixel data from memory. The initial value is 0. See glPixelStore.
    UNPACK_ROW_LENGTH=UNPACK_ROW_LENGTH,

    /// Returns one value, the number of pixel images skipped before the first pixel is read from memory. The initial value is 0. See glPixelStore.
    UNPACK_SKIP_IMAGES=UNPACK_SKIP_IMAGES,

    /// Returns one value, the number of pixel locations skipped before the first pixel is read from memory. The initial value is 0. See glPixelStore.
    UNPACK_SKIP_PIXELS=UNPACK_SKIP_PIXELS,

    /// Returns one value, the number of rows of pixel locations skipped before the first pixel is read from memory. The initial value is 0. See glPixelStore.
    UNPACK_SKIP_ROWS=UNPACK_SKIP_ROWS,

    /// Returns a single boolean value indicating whether the bytes of two-byte and four-byte pixel indices and components are swapped after being read from memory. The initial value is GL_FALSE. See glPixelStore.
    UNPACK_SWAP_BYTES=UNPACK_SWAP_BYTES,

    /// Returns one value, the number of extensions supported by the GL implementation for the current context. See glGetString.
    NUM_EXTENSIONS=NUM_EXTENSIONS,

    /// Returns one value, the major version number of the OpenGL API supported by the current context.
    MAJOR_VERSION=MAJOR_VERSION,

    /// Returns one value, the minor version number of the OpenGL API supported by the current context.
    MINOR_VERSION=MINOR_VERSION,

    /// Returns one value, the flags with which the context was created (such as debugging functionality).
    CONTEXT_FLAGS=CONTEXT_FLAGS,

    /// Returns four values: the x and y window coordinates of the viewport, followed by its width and height.
    /// Initially the x and y window coordinates are both set to 0, and the width and height are set to the width and height of the window into which the GL will do its rendering. See glViewport.
    VIEWPORT=VIEWPORT,
}

#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum IndexedCoreParameter{
    /// Returns one value, a symbolic constant indicating which buffers are being drawn to by the corresponding output color.
    /// See glDrawBuffers.
    /// The initial value of GL_DRAW_BUFFER0 is GL_BACK if there are back buffers, otherwise it is GL_FRONT.
    /// The initial values of draw buffers for all other output colors is GL_NONE.
    DrawBuffer=DRAW_BUFFER
}