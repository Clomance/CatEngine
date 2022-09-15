#[cfg(target_os="windows")]
use crate::winapi::OpenGraphicsLibrary;

use super::{
    types::*,
    constants::*
};

use core::mem::transmute;

#[cfg(target_os="linux")]
extern "system"{
    fn glGenTextures(number:GLsizei,textures:*mut GLuint);
    fn glDeleteTextures(number:GLsizei,textures:*const GLuint);
    fn glIsTexture(texture:GLuint)->GLboolean;

    fn glActiveTexture(texture:GLenum);
    fn glBindTexture(target:GLenum,texture:GLuint);

    fn glTexImage2D(
        target:GLenum,
        level:GLint,
        internal_format:GLint,
        width:GLsizei,
        height:GLsizei,
        border:GLint,
        pixel_format:GLenum,
        pixel_type:GLenum,
        data:*const GLvoid
    );
    fn glTexImage3D(
        target:GLenum,
        level:GLint,
        internal_format:GLint,
        width:GLsizei,
        height:GLsizei,
        depth:GLsizei,
        border:GLint,
        pixel_format:GLenum,
        pixel_type:GLenum,
        data:*const GLvoid
    );

    fn glTexSubImage2D(
        target:GLenum,
        level:GLint,
        xoffset:GLint,
        yoffset:GLint,
        width:GLsizei,
        height:GLsizei,
        pixel_format:GLenum,
        pixel_type:GLenum,
        data:*const GLvoid
    );
    fn glTexSubImage3D(
        target:GLenum,
        level:GLint,
        xoffset:GLint,
        yoffset:GLint,
        zoffset:GLint,
        width:GLsizei,
        height:GLsizei,
        depth:GLsizei,
        pixel_format:GLenum,
        pixel_type:GLenum,
        data:*const GLvoid
    );

    fn glTexParameterf(target:GLenum,parameter:GLenum,value:GLfloat);
    fn glTexParameteri(target:GLenum,parameter:GLenum,value:GLint);
    fn glTexParameterfv(target:GLenum,parameter:GLenum,values:*const GLfloat);
    fn glTexParameteriv(target:GLenum,parameter:GLenum,values:*const GLint);
    fn glTexParameterIiv(target:GLenum,parameter:GLenum,values:*const GLint);
    fn glTexParameterIuiv(target:GLenum,parameter:GLenum,values:*const GLuint);
}

#[cfg(target_os="windows")]
mod gl{
    pub static mut glGenTextures:usize=0;
    pub static mut glDeleteTextures:usize=0;
    pub static mut glIsTexture:usize=0;

    pub static mut glActiveTexture:usize=0;
    pub static mut glBindTexture:usize=0;

    pub static mut glTexImage2D:usize=0;
    pub static mut glTexImage3D:usize=0;

    pub static mut glTexSubImage2D:usize=0;
    pub static mut glTexSubImage3D:usize=0;

    pub static mut glTexParameterf:usize=0;
    pub static mut glTexParameteri:usize=0;
    pub static mut glTexParameterfv:usize=0;
    pub static mut glTexParameteriv:usize=0;
    pub static mut glTexParameterIiv:usize=0;
    pub static mut glTexParameterIuiv:usize=0;
}

#[cfg(target_os="windows")]
mod gl_functions{
    use super::*;

    #[inline(always)]
    pub unsafe fn glGenTextures(number:GLsizei,textures:*mut GLuint){
        transmute::<usize,extern "system" fn(GLsizei,*mut GLuint)>(gl::glGenTextures)(number,textures)
    }

    #[inline(always)]
    pub unsafe fn glDeleteTextures(number:GLsizei,textures:*const GLuint){
        transmute::<usize,extern "system" fn(GLsizei,*const GLuint)>(gl::glDeleteTextures)(number,textures)
    }

    #[inline(always)]
    pub unsafe fn glIsTexture(texture:GLuint)->GLboolean{
        transmute::<usize,extern "system" fn(GLuint)->GLboolean>(gl::glIsTexture)(texture)
    }

    #[inline(always)]
    pub unsafe fn glActiveTexture(texture:GLenum){
        transmute::<usize,extern "system" fn(GLenum)>(gl::glActiveTexture)(texture)
    }

    #[inline(always)]
    pub unsafe fn glBindTexture(target:GLenum,texture:GLuint){
        transmute::<usize,extern "system" fn(GLenum,GLuint)>(gl::glBindTexture)(target,texture)
    }

    #[inline(always)]
    pub unsafe fn glTexImage2D(
        target:GLenum,
        level:GLint,
        internal_format:GLint,
        width:GLsizei,
        height:GLsizei,
        border:GLint,
        pixel_format:GLenum,
        pixel_type:GLenum,
        data:*const GLvoid
    ){
        transmute::<usize,extern "system" fn(
            GLenum,
            GLint,
            GLint,
            GLsizei,
            GLsizei,
            GLint,
            GLenum,
            GLenum,
            *const GLvoid
        )>(gl::glTexImage2D)(
            target,
            level,
            internal_format,
            width,
            height,
            border,
            pixel_format,
            pixel_type,
            data,
        )
    }

    #[inline(always)]
    pub unsafe fn glTexImage3D(
        target:GLenum,
        level:GLint,
        internal_format:GLint,
        width:GLsizei,
        height:GLsizei,
        depth:GLsizei,
        border:GLint,
        pixel_format:GLenum,
        pixel_type:GLenum,
        data:*const GLvoid
    ){
        transmute::<usize,extern "system" fn(
            GLenum,
            GLint,
            GLint,
            GLsizei,
            GLsizei,
            GLsizei,
            GLint,
            GLenum,
            GLenum,
            *const GLvoid
        )>(gl::glTexImage3D)(
            target,
            level,
            internal_format,
            width,
            height,
            depth,
            border,
            pixel_format,
            pixel_type,
            data,
        )
    }

    #[inline(always)]
    pub unsafe fn glTexSubImage2D(
        target:GLenum,
        level:GLint,
        xoffset:GLint,
        yoffset:GLint,
        width:GLsizei,
        height:GLsizei,
        pixel_format:GLenum,
        pixel_type:GLenum,
        data:*const GLvoid
    ){
        transmute::<usize,extern "system" fn(
            GLenum,
            GLint,
            GLint,
            GLint,
            GLsizei,
            GLsizei,
            GLenum,
            GLenum,
            *const GLvoid
        )>(gl::glTexSubImage2D)(
            target,
            level,
            xoffset,
            yoffset,
            width,
            height,
            pixel_format,
            pixel_type,
            data
        )
    }
    #[inline(always)]
    pub unsafe fn glTexSubImage3D(
        target:GLenum,
        level:GLint,
        xoffset:GLint,
        yoffset:GLint,
        zoffset:GLint,
        width:GLsizei,
        height:GLsizei,
        depth:GLsizei,
        pixel_format:GLenum,
        pixel_type:GLenum,
        data:*const GLvoid
    ){
        transmute::<usize,extern "system" fn(
            GLenum,
            GLint,
            GLint,
            GLint,
            GLint,
            GLsizei,
            GLsizei,
            GLsizei,
            GLenum,
            GLenum,
            *const GLvoid
        )>(gl::glTexSubImage3D)(
            target,
            level,
            xoffset,
            yoffset,
            zoffset,
            width,
            height,
            depth,
            pixel_format,
            pixel_type,
            data
        )
    }


    #[inline(always)]
    pub unsafe fn glTexParameterf(target:GLenum,parameter:GLenum,value:GLfloat){
        transmute::<usize,extern "system" fn(GLenum,GLenum,GLfloat)>(gl::glTexParameterf)(target,parameter,value)
    }

    #[inline(always)]
    pub unsafe fn glTexParameteri(target:GLenum,parameter:GLenum,value:GLint){
        transmute::<usize,extern "system" fn(GLenum,GLenum,GLint)>(gl::glTexParameteri)(target,parameter,value)
    }

    #[inline(always)]
    pub unsafe fn glTexParameterfv(target:GLenum,parameter:GLenum,values:*const GLfloat){
        transmute::<usize,extern "system" fn(GLenum,GLenum,*const GLfloat)>(gl::glTexParameterfv)(target,parameter,values)
    }

    #[inline(always)]
    pub unsafe fn glTexParameteriv(target:GLenum,parameter:GLenum,values:*const GLint){
        transmute::<usize,extern "system" fn(GLenum,GLenum,*const GLint)>(gl::glTexParameteriv)(target,parameter,values)
    }

    #[inline(always)]
    pub unsafe fn glTexParameterIiv(target:GLenum,parameter:GLenum,values:*const GLint){
        transmute::<usize,extern "system" fn(GLenum,GLenum,*const GLint)>(gl::glTexParameterIiv)(target,parameter,values)
    }

    #[inline(always)]
    pub unsafe fn glTexParameterIuiv(target:GLenum,parameter:GLenum,values:*const GLuint){
        transmute::<usize,extern "system" fn(GLenum,GLenum,*const GLuint)>(gl::glTexParameterIuiv)(target,parameter,values)
    }
}

#[cfg(target_os="windows")]
use gl_functions::*;

pub struct Texture;

impl Texture{
    #[cfg(target_os="windows")]
    pub fn load(library:&OpenGraphicsLibrary){
        use gl::*;

        unsafe{
            glGenTextures=transmute(library.get_proc_address("glGenTextures\0"));
            glDeleteTextures=transmute(library.get_proc_address("glDeleteTextures\0"));
            glIsTexture=transmute(library.get_proc_address("glIsTexture\0"));

            glActiveTexture=transmute(library.get_proc_address("glActiveTexture\0"));
            glBindTexture=transmute(library.get_proc_address("glBindTexture\0"));

            glTexImage2D=transmute(library.get_proc_address("glTexImage2D\0"));
            glTexImage3D=transmute(library.get_proc_address("glTexImage3D\0"));

            glTexSubImage2D=transmute(library.get_proc_address("glTexSubImage2D\0"));
            glTexSubImage3D=transmute(library.get_proc_address("glTexSubImage3D\0"));
            // glCopyTexSubImage2D=transmute(library.get_proc_address("glCopyTexSubImage2D\0"));

            glTexParameterf=transmute(library.get_proc_address("glTexParameterf\0"));
            glTexParameteri=transmute(library.get_proc_address("glTexParameteri\0"));
            glTexParameterfv=transmute(library.get_proc_address("glTexParameterfv\0"));
            glTexParameteriv=transmute(library.get_proc_address("glTexParameteriv\0"));
            glTexParameterIiv=transmute(library.get_proc_address("glTexParameterIiv\0"));
            glTexParameterIuiv=transmute(library.get_proc_address("glTexParameterIuiv\0"));
        }
    }
}

impl Texture{
    /// Generates texture names.
    /// 
    /// `number` specifies the number of texture names to be generated.
    /// 
    /// `textures` specifies an array in which the generated texture names are stored.
    /// 
    /// Returns `number` texture names in `textures`.
    /// There is no guarantee that the names form a contiguous set of integers;
    /// however, it is guaranteed that none of the returned names was in use immediately before the call to `Texture::generate`.
    /// 
    /// The generated textures have no dimensionality;
    /// they assume the dimensionality of the texture target to which they are first bound (see `Texture::bind`).
    /// 
    /// Texture names returned by a call to `Texture::generate` are not returned by subsequent calls,
    /// unless they are first deleted with `Texture::delete`.
    /// 
    /// `Error::InvalidValue` is generated if n is negative.
    #[inline(always)]
    pub unsafe fn generate(number:i32,textures:*mut u32){
        glGenTextures(number,textures)
    }

    /// Deletes named textures.
    /// 
    /// `number` specifies the number of textures to be deleted.
    /// 
    /// `textures` specifies an array of textures to be deleted.
    /// 
    /// Deletes `number` textures named by the elements of the array `textures`.
    /// After a texture is deleted, it has no contents or dimensionality, and its name is free for reuse (for example by `Texture::generate`).
    /// If a texture that is currently bound is deleted, the binding reverts to 0 (the default texture).
    /// 
    /// Silently ignores 0's and names that do not correspond to existing textures.
    /// 
    /// `Error::InvalidValue` is generated if n is negative.
    #[inline(always)]
    pub unsafe fn delete(number:i32,textures:*const u32){
        glDeleteTextures(number,textures)
    }

    /// Determines if a name corresponds to a texture.
    /// 
    /// Returns `true` if `texture` is currently the name of a texture.
    /// If `texture` is zero, or is a non-zero value that is not currently the name of a texture, or if an error occurs,
    /// Returns `false`.
    /// 
    /// A name returned by `Texture::generate`, but not yet associated with a texture by calling `Texture::bind`, is not the name of a texture.
    #[inline(always)]
    pub unsafe fn is_texture(buffer:u32)->bool{
        transmute(glIsTexture(buffer))
    }
}

pub struct TextureUnit{
    inner:u32
}

impl TextureUnit{
    pub fn new(unit:u32)->TextureUnit{
        Self{
            inner:TEXTURE0+unit
        }
    }
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum TextureTarget{
    Texture1D=TEXTURE_1D,
    Texture2D=TEXTURE_2D,
    Texture3D=TEXTURE_3D,
    TextureRectable=TEXTURE_RECTANGLE,
    TextureCubeMap=TEXTURE_CUBE_MAP,
    Texture1DArray=TEXTURE_1D_ARRAY,
    Texture2DArray=TEXTURE_2D_ARRAY,
    TextureBuffer=TEXTURE_BUFFER,

    /// Available only if the GL version is 3.2 or higher.
    Texture2DMultisample=TEXTURE_2D_MULTISAMPLE,

    /// Available only if the GL version is 3.2 or higher.
    Texture2DMultisampleArray=TEXTURE_2D_MULTISAMPLE_ARRAY,
}

impl Texture{
    /// Selects active texture unit.
    /// 
    /// `texture` Specifies which texture unit to make active.
    /// The number of texture units is implementation dependent, but must be at least 48.
    /// `texture` must be one of GL_TEXTUREi, where i ranges from 0 (GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS - 1).
    /// The initial value is GL_TEXTURE0.
    /// 
    /// Selects which texture unit subsequent texture state calls will affect.
    /// The number of texture units an implementation supports is implementation dependent, but must be at least 48.
    /// 
    /// `Error::InvalidEnum` is generated if `texture` is not one of `GL_TEXTUREi`, where i ranges from 0 to (GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS - 1).
    #[inline(always)]
    pub unsafe fn active(texture:TextureUnit){
        glActiveTexture(texture.inner)
    }

    /// Binds a named texture to a texturing target.
    /// 
    /// `target` specifies the target to which the texture is bound.
    /// 
    /// `texture` specifies the name of a texture.
    /// 
    /// Lets you create or use a named texture.
    /// Calling with `target` set to `TextureTarget` and `texture` set to the name of the new texture binds the texture name to the target.
    /// When a texture is bound to a target, the previous binding for that target is automatically broken.
    /// 
    /// Texture names are unsigned integers. The value zero is reserved to represent the default texture for each texture target.
    /// Texture names and the corresponding texture contents are local to the shared object space of the current GL rendering context;
    /// two rendering contexts share texture names only if they explicitly enable sharing between contexts through the appropriate GL windows interfaces functions.
    /// 
    /// You must use `Texture::generate` to generate a set of new texture names.
    /// 
    /// When a texture is first bound, it assumes the specified target:
    /// A texture first bound to GL_TEXTURE_1D becomes one-dimensional texture,
    /// a texture first bound to GL_TEXTURE_2D becomes two-dimensional texture,
    /// a texture first bound to GL_TEXTURE_3D becomes three-dimensional texture,
    /// a texture first bound to GL_TEXTURE_1D_ARRAY becomes one-dimensional array texture,
    /// a texture first bound to GL_TEXTURE_2D_ARRAY becomes two-dimensional arary texture,
    /// a texture first bound to GL_TEXTURE_RECTANGLE becomes rectangle texture,
    /// a, texture first bound to GL_TEXTURE_CUBE_MAP becomes a cube-mapped texture,
    /// a texture first bound to GL_TEXTURE_BUFFER becomes a buffer texture,
    /// a texture first bound to GL_TEXTURE_2D_MULTISAMPLE becomes a two-dimensional multisampled texture,
    /// and a texture first bound to GL_TEXTURE_2D_MULTISAMPLE_ARRAY becomes a two-dimensional multisampled array texture.
    /// The state of a one-dimensional texture immediately after it is first bound is equivalent to the state of the default GL_TEXTURE_1D at GL initialization,
    /// and similarly for the other texture types.
    /// 
    /// While a texture is bound, GL operations on the target to which it is bound affect the bound texture,
    /// and queries of the target to which it is bound return state from the bound texture.
    /// In effect, the texture targets become aliases for the textures currently bound to them,
    /// and the texture name zero refers to the default textures that were bound to them at initialization.
    /// 
    /// A texture binding created with `Texture::bind` remains active until a different texture is bound to the same target, or until the bound texture is deleted with `Texture::delete`.
    /// 
    /// Once created, a named texture may be re-bound to its same original target as often as needed.
    /// It is usually much faster to use `Texture::bind` to bind an existing named texture to one of the texture targets
    /// than it is to reload the texture image using glTexImage1D, glTexImage2D, glTexImage3D or another similar function.
    /// 
    /// `Error::InvalidEnum` is generated if `target` is not one of the allowable values.
    /// 
    /// `Error::InvalidValue` is generated
    /// if `target` is not a name returned from a previous call to `Texture::generate`.
    /// 
    /// `Error::InvalidOperation` is generated
    /// if `texture` was previously created with a target that doesn't match that of `target`.
    #[inline(always)]
    pub unsafe fn bind(target:TextureTarget,texture:u32){
        glBindTexture(target as GLenum,texture)
    }
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum Texture2DAllocateTarget{
    Texture2D=TEXTURE_2D,
    ProxyTexture2D=PROXY_TEXTURE_2D,
    Texture1DArray=TEXTURE_1D_ARRAY,
    PROXY_TEXTURE_1D_ARRAY=PROXY_TEXTURE_1D_ARRAY,
    TEXTURE_RECTANGLE=TEXTURE_RECTANGLE,
    PROXY_TEXTURE_RECTANGLE=PROXY_TEXTURE_RECTANGLE,
    TEXTURE_CUBE_MAP_POSITIVE_X=TEXTURE_CUBE_MAP_POSITIVE_X,
    TEXTURE_CUBE_MAP_NEGATIVE_X=TEXTURE_CUBE_MAP_NEGATIVE_X,
    TEXTURE_CUBE_MAP_POSITIVE_Y=TEXTURE_CUBE_MAP_POSITIVE_Y,
    TEXTURE_CUBE_MAP_NEGATIVE_Y=TEXTURE_CUBE_MAP_NEGATIVE_Y,
    TEXTURE_CUBE_MAP_POSITIVE_Z=TEXTURE_CUBE_MAP_POSITIVE_Z,
    TEXTURE_CUBE_MAP_NEGATIVE_Z=TEXTURE_CUBE_MAP_NEGATIVE_Z,
    PROXY_TEXTURE_CUBE_MAP=PROXY_TEXTURE_CUBE_MAP
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum Texture3DAllocateTarget{
    Texture3D=TEXTURE_3D,
    ProxyTexure3D=PROXY_TEXTURE_3D,
    Texture2DArray=TEXTURE_2D_ARRAY,
    ProxyTexture2DArray=PROXY_TEXTURE_2D_ARRAY
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum TextureInternalFormat{
    R8=R8,
    R8I=R8I,
    R8UI=R8UI,
    R8_SNORM=R8_SNORM,
    R16=R16,
    R16_SNORM=R16_SNORM,
    R16I=R16I,
    R16UI=R16UI,
    R16F=R16F,
    R32I=R32I,
    R32UI=R32UI,
    R32F=R32F,
    RG8=RG8,
    RG8_SNORM=RG8_SNORM,
    RG16=RG16,
    RG16_SNORM=RG16_SNORM,
    RG16F=RG16F,
    RGB8=RGB8,
    SRGB8=SRGB8,
    RGB8_SNORM=RGB8_SNORM,
    SRGB8_ALPHA8=SRGB8_ALPHA8,
    RGB16=RGB16,
    RGB16_SNorm=RGB16_SNORM,
    RGBA8=RGBA8,
    RGBA8_SNORM=RGBA8_SNORM,
    RGBA32F=RGBA32F,
    RGBA32I=RGBA32I,
    RGBA32UI=RGBA32UI,
    RGBA16=RGBA16,
    RGBA16_SNORM=RGBA16_SNORM,
    RGBA16F=RGBA16F,
    RGBA16I=RGBA16I,
    RGBA16UI=RGBA16UI,
    RGBA8UI=RGBA8UI,
    RGB10_A2=RGB10_A2,
    RGB10_A2UI=RGB10_A2UI,
    R11F_G11F_B10F=R11F_G11F_B10F,
    RG32F=RG32F,
    RG32I=RG32I,
    RG32UI=RG32UI,
    RG8I=RG8I,
    RG8UI=RG8UI,
    RGB32F=RGB32F,
    RGB32I=RGB32I,
    RGB32UI=RGB32UI,
    RGB16F=RGB16F,
    RGB16I=RGB16I,
    RGB16UI=RGB16UI,
    RGB8I=RGB8I,
    RGB8UI=RGB8UI,
    RGB9_E5=RGB9_E5,
    COMPRESSED_RG_RGTC2=COMPRESSED_RG_RGTC2,
    COMPRESSED_SIGNED_RG_RGTC2=COMPRESSED_SIGNED_RG_RGTC2,
    COMPRESSED_RED_RGTC1=COMPRESSED_RED_RGTC1,
    COMPRESSED_SIGNED_RED_RGTC1=COMPRESSED_SIGNED_RED_RGTC1,
    DEPTH_COMPONENT24=DEPTH_COMPONENT24,
    DEPTH_COMPONENT16=DEPTH_COMPONENT16,
    DEPTH_COMPONENT32F=DEPTH_COMPONENT32F,
    DEPTH24_STENCIL8=DEPTH24_STENCIL8,
    DEPTH32F_STENCIL8=DEPTH32F_STENCIL8,
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum PixelFormat{
    /// Each element is a single red component.
    /// The GL converts it to floating point and assembles it into an RGBA element by attaching 0 for green and blue, and 1 for alpha.
    /// Each component is clamped to the range [0,1].
    RED=RED,

    /// Each element is a red/green double.
    /// The GL converts it to floating point and assembles it into an RGBA element by attaching 0 for blue, and 1 for alpha.
    /// Each component is clamped to the range [0,1].
    RG=RG,

    /// Each element is an RGB triple.
    /// The GL converts it to floating point and assembles it into an RGBA element by attaching 1 for alpha.
    /// Each component clamped to the range [0,1].
    RGB=RGB,

    /// Each element is an RGB triple.
    /// The GL converts it to floating point and assembles it into an RGBA element by attaching 1 for alpha.
    /// Each component clamped to the range [0,1].
    BGR=BGR,

    /// Each element contains all four components.
    /// Each component is clamped to the range [0,1].
    RGBA=RGBA,

    /// Each element contains all four components.
    /// Each component is clamped to the range [0,1].
    BGRA=BGRA,

    /// Each element is a single depth value.
    /// The GL converts it to floating point, and clamps to the range [0,1].
    DEPTH_COMPONENT=DEPTH_COMPONENT,

    /// Each element is a pair of depth and stencil values.
    /// The depth component of the pair is interpreted as in GL_DEPTH_COMPONENT.
    /// The stencil component is interpreted based on specified the depth + stencil internal format.
    DEPTH_STENCIL=DEPTH_STENCIL
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum PixelType{
    U8=UNSIGNED_BYTE,
    I8=BYTE,
    U16=UNSIGNED_SHORT,
    I16=SHORT,
    U32=UNSIGNED_INT,
    I32=INT,
    F32=FLOAT,
    // U8_3_3_2=UNSIGNED_BYTE_3_3_2,
    // U8_2_3_3_REV=UNSIGNED_BYTE_2_3_3_REV,
    // UNSIGNED_SHORT_5_6_5=UNSIGNED_SHORT_5_6_5,
    // UNSIGNED_SHORT_5_6_5_REV=UNSIGNED_SHORT_5_6_5_REV,
    // UNSIGNED_SHORT_4_4_4_4=UNSIGNED_SHORT_4_4_4_4,
    // UNSIGNED_SHORT_4_4_4_4_REV=UNSIGNED_SHORT_4_4_4_4_REV,
    // UNSIGNED_SHORT_5_5_5_1=UNSIGNED_SHORT_5_5_5_1,
    // UNSIGNED_SHORT_1_5_5_5_REV=UNSIGNED_SHORT_1_5_5_5_REV,
    // UNSIGNED_INT_8_8_8_8=UNSIGNED_INT_8_8_8_8,
    // UNSIGNED_INT_8_8_8_8_REV=UNSIGNED_INT_8_8_8_8_REV,
    // UNSIGNED_INT_10_10_10_2=UNSIGNED_INT_10_10_10_2,
    // UNSIGNED_INT_2_10_10_10_REV=UNSIGNED_INT_2_10_10_10_REV
}

impl Texture{
    /// Specifies a two-dimensional texture image.
    /// 
    /// `target` specifies the target texture.
    /// 
    /// `level` specifies the level-of-detail number.
    /// Level 0 is the base image level.
    /// Level n is the nth mipmap reduction image.
    /// If `target` is GL_TEXTURE_RECTANGLE or GL_PROXY_TEXTURE_RECTANGLE, level must be 0.
    /// 
    /// `internalFormat` specifies the number of color components in the texture.
    /// 
    /// `width` specifies the width of the texture image.
    /// All implementations support texture images that are at least 1024 texels wide.
    /// 
    /// `height` specifies the height of the texture image, or the number of layers in a texture array,
    /// in the case of the GL_TEXTURE_1D_ARRAY and GL_PROXY_TEXTURE_1D_ARRAY targets.
    /// All implementations support 2D texture images that are at least 1024 texels high,
    /// and texture arrays that are at least 256 layers deep.
    /// 
    /// `border` - This value must be 0.
    /// 
    /// `pixel_format` specifies the format of the pixel data.
    /// 
    /// `pixel_type` specifies the data type of the pixel data.
    /// 
    /// `data` specifies a pointer to the image data in memory.
    /// 
    /// Texturing allows elements of an image array to be read by shaders.
    /// 
    /// To define texture images, call `Texture::allocate`.
    /// The arguments describe the parameters of the texture image, such as height, width, width of the border, level-of-detail number (see glTexParameter), and number of color components provided.
    /// The last three arguments describe how the image is represented in memory.
    /// 
    /// If `target` is GL_PROXY_TEXTURE_2D, GL_PROXY_TEXTURE_1D_ARRAY, GL_PROXY_TEXTURE_CUBE_MAP, or GL_PROXY_TEXTURE_RECTANGLE, no data is read from data, but all of the texture image state is recalculated, checked for consistency, and checked against the implementation's capabilities. If the implementation cannot handle a texture of the requested texture size, it sets all of the image state to 0, but does not generate an error (see glGetError).
    /// To query for an entire mipmap array, use an image array level greater than or equal to 1.
    /// 
    /// If target is GL_TEXTURE_2D, GL_TEXTURE_RECTANGLE or one of the GL_TEXTURE_CUBE_MAP targets, data is read from data as a sequence of signed or unsigned bytes, shorts, or longs, or single-precision floating-point values, depending on type. These values are grouped into sets of one, two, three, or four values, depending on format, to form elements.
    /// Each data byte is treated as eight 1-bit elements, with bit ordering determined by GL_UNPACK_LSB_FIRST (see glPixelStore).
    /// 
    /// If `target` is GL_TEXTURE_1D_ARRAY, data is interpreted as an array of one-dimensional images.
    /// 
    /// If a non-zero named buffer object is bound to the GL_PIXEL_UNPACK_BUFFER target (see glBindBuffer) while a texture image is specified, data is treated as a byte offset into the buffer object's data store.
    /// 
    /// The first element corresponds to the lower left corner of the texture image.
    /// Subsequent elements progress left-to-right through the remaining texels in the lowest row of the texture image, and then in successively higher rows of the texture image.
    /// The final element corresponds to the upper right corner of the texture image.
    /// 
    /// `format` determines the composition of each element in data.
    /// 
    /// If an application wants to store the texture at a certain resolution or in a certain format, it can request the resolution and format with internalFormat.
    /// The GL will choose an internal representation that closely approximates that requested by internalFormat, but it may not match exactly.
    /// (The representations specified by GL_RED, GL_RG, GL_RGB, and GL_RGBA must match exactly.)
    /// 
    /// If the internalFormat parameter is one of the generic compressed formats, GL_COMPRESSED_RED, GL_COMPRESSED_RG, GL_COMPRESSED_RGB, or GL_COMPRESSED_RGBA, the GL will replace the internal format with the symbolic constant for a specific internal format and compress the texture before storage. If no corresponding internal format is available, or the GL can not compress that image for any reason, the internal format is instead replaced with a corresponding base internal format.
    /// 
    /// If the internalFormat parameter is GL_SRGB, GL_SRGB8, GL_SRGB_ALPHA, or GL_SRGB8_ALPHA8, the texture is treated as if the red, green, or blue components are encoded in the sRGB color space. Any alpha component is left unchanged. 
    /// 
    /// Use the GL_PROXY_TEXTURE_2D, GL_PROXY_TEXTURE_1D_ARRAY, GL_PROXY_TEXTURE_RECTANGLE, or GL_PROXY_TEXTURE_CUBE_MAP target to try out a resolution and format.
    /// The implementation will update and recompute its best match for the requested storage resolution and format.
    /// To then query this state, call glGetTexLevelParameter.
    /// If the texture cannot be accommodated, texture state is set to 0.
    /// 
    /// A one-component texture image uses only the red component of the RGBA color extracted from data.
    /// A two-component image uses the R and G values. A three-component image uses the R, G, and B values.
    /// A four-component image uses all of the RGBA components.
    /// 
    /// Image-based shadowing can be enabled by comparing texture r coordinates to depth texture values to generate a boolean result.
    /// See glTexParameter for details on texture comparison.
    /// 
    /// The glPixelStore mode affects texture images.
    /// 
    /// data may be a null pointer.
    /// In this case, texture memory is allocated to accommodate a texture of width width and height height.
    /// You can then download subtextures to initialize this texture memory.
    /// The image is undefined if the user tries to apply an uninitialized portion of the texture image to a primitive.
    /// 
    /// `Texture::allocate` specifies the two-dimensional texture for the current texture unit, specified with `Texture::active`.
    /// 
    /// `GLError::InvalidEnum` is generated
    /// if `target` is not one of the allowable values,
    /// if `pixel_type` is not a type constant,
    /// 
    /// `GLError::InvalidValue` is generated
    /// if `width` is less than 0 or greater than GL_MAX_TEXTURE_SIZE,
    /// if `target` is not GL_TEXTURE_1D_ARRAY or GL_PROXY_TEXTURE_1D_ARRAY and height is less than 0 or greater than GL_MAX_TEXTURE_SIZE,
    /// if `target` is GL_TEXTURE_1D_ARRAY or GL_PROXY_TEXTURE_1D_ARRAY and height is less than 0 or greater than GL_MAX_ARRAY_TEXTURE_LAYERS,
    /// if `level` is less than 0.
    /// if `level` is greater than log2(max), where max is the returned value of GL_MAX_TEXTURE_SIZE,
    /// if `internal_format` is not one of the accepted resolution and format symbolic constants,
    /// if `width` or `height` is less than 0 or greater than GL_MAX_TEXTURE_SIZE,
    /// if non-power-of-two textures are not supported and the `width` or `height` cannot be represented as 2^k+2(border) for some integer value of k,
    /// if `border` is not 0,
    /// if `target` is GL_TEXTURE_RECTANGLE or GL_PROXY_TEXTURE_RECTANGLE and `level` is not 0.
    /// 
    /// `GLError::InvalidOperation`
    /// if `pixel_type` is one of GL_UNSIGNED_BYTE_3_3_2, GL_UNSIGNED_BYTE_2_3_3_REV, GL_UNSIGNED_SHORT_5_6_5, GL_UNSIGNED_SHORT_5_6_5_REV, or GL_UNSIGNED_INT_10F_11F_11F_REV, and format is not GL_RGB,
    /// if `pixel_type` is one of GL_UNSIGNED_SHORT_4_4_4_4, GL_UNSIGNED_SHORT_4_4_4_4_REV, GL_UNSIGNED_SHORT_5_5_5_1, GL_UNSIGNED_SHORT_1_5_5_5_REV, GL_UNSIGNED_INT_8_8_8_8, GL_UNSIGNED_INT_8_8_8_8_REV, GL_UNSIGNED_INT_10_10_10_2, GL_UNSIGNED_INT_2_10_10_10_REV, or GL_UNSIGNED_INT_5_9_9_9_REV, and format is neither GL_RGBA nor GL_BGRA,
    /// if `target` is not GL_TEXTURE_2D, GL_PROXY_TEXTURE_2D, GL_TEXTURE_RECTANGLE, or GL_PROXY_TEXTURE_RECTANGLE, and internalFormat is GL_DEPTH_COMPONENT, GL_DEPTH_COMPONENT16, GL_DEPTH_COMPONENT24, or GL_DEPTH_COMPONENT32F,
    /// if `pixel_format` is GL_DEPTH_COMPONENT and internalFormat is not GL_DEPTH_COMPONENT, GL_DEPTH_COMPONENT16, GL_DEPTH_COMPONENT24, or GL_DEPTH_COMPONENT32F,
    /// if `internal_format` is GL_DEPTH_COMPONENT, GL_DEPTH_COMPONENT16, GL_DEPTH_COMPONENT24, or GL_DEPTH_COMPONENT32F, and format is not GL_DEPTH_COMPONENT,
    /// if a non-zero buffer object name is bound to the GL_PIXEL_UNPACK_BUFFER target and the buffer object's data store is currently mapped,
    /// if a non-zero buffer object name is bound to the GL_PIXEL_UNPACK_BUFFER target and the data would be unpacked from the buffer object such that the memory reads required would exceed the data store size,
    /// if a non-zero buffer object name is bound to the GL_PIXEL_UNPACK_BUFFER target and data is not evenly divisible into the number of bytes needed to store in memory a datum indicated by `type`.
    pub unsafe fn allocate_2d(
        target:Texture2DAllocateTarget,
        level:i32,
        internal_format:TextureInternalFormat,
        width:i32,
        height:i32,
        border:i32,
        pixel_format:PixelFormat,
        pixel_type:PixelType,
        data:*const GLvoid
    ){
        glTexImage2D(
            target as GLenum,
            level,
            internal_format as GLint,
            width,
            height,
            border,
            pixel_format as GLenum,
            pixel_type as GLenum,
            data,
        )
    }

    /// Specifies a three-dimensional texture image.
    /// 
    /// `target` specifies the target texture.
    /// 
    /// `level` pecifies the level-of-detail number.
    /// Level 0 is the base image level.
    /// Level n is the nth mipmap reduction image.
    /// 
    /// `internal_format` specifies the number of color components in the texture.
    /// 
    /// `width` specifies the width of the texture image.
    /// All implementations support 3D texture images that are at least 16 texels wide.
    /// 
    /// `height` specifies the height of the texture image.
    /// All implementations support 3D texture images that are at least 256 texels high.
    /// 
    /// `depth` specifies the depth of the texture image, or the number of layers in a texture array.
    /// All implementations support 3D texture images that are at least 256 texels deep, and texture arrays that are at least 256 layers deep.
    /// 
    /// `border` must be 0.
    /// 
    /// `pixel_format` specifies the format of the pixel data.
    /// 
    /// `pixel_type` specifies the data type of the pixel data.
    /// 
    /// `data` specifies a pointer to the image data in memory.
    /// 
    /// Texturing maps a portion of a specified texture image onto each graphical primitive for which texturing is enabled.
    /// To enable and disable three-dimensional texturing, call glEnable and glDisable with argument GL_TEXTURE_3D.
    /// 
    /// To define texture images, call glTexImage3D.
    /// The arguments describe the parameters of the texture image, such as height, width, depth, width of the border, level-of-detail number (see glTexParameter), and number of color components provided.
    /// The last three arguments describe how the image is represented in memory.
    /// 
    /// If target is GL_PROXY_TEXTURE_3D, no data is read from data, but all of the texture image state is recalculated, checked for consistency, and checked against the implementation's capabilities. If the implementation cannot handle a texture of the requested texture size, it sets all of the image state to 0, but does not generate an error (see glGetError).
    /// To query for an entire mipmap array, use an image array level greater than or equal to 1.
    /// 
    /// If target is GL_TEXTURE_3D, data is read from data as a sequence of signed or unsigned bytes, shorts, or longs, or single-precision floating-point values, depending on type. These values are grouped into sets of one, two, three, or four values, depending on format, to form elements.
    /// Each data byte is treated as eight 1-bit elements, with bit ordering determined by GL_UNPACK_LSB_FIRST (see glPixelStore).
    /// 
    /// If a non-zero named buffer object is bound to the GL_PIXEL_UNPACK_BUFFER target (see glBindBuffer) while a texture image is specified, data is treated as a byte offset into the buffer object's data store.
    /// 
    /// The first element corresponds to the lower left corner of the texture image.
    /// Subsequent elements progress left-to-right through the remaining texels in the lowest row of the texture image, and then in successively higher rows of the texture image.
    /// The final element corresponds to the upper right corner of the texture image.
    /// 
    /// `pixel_format` determines the composition of each element in `data`.
    /// 
    /// If an application wants to store the texture at a certain resolution or in a certain format, it can request the resolution and format with `internal_format`.
    /// The GL will choose an internal representation that closely approximates that requested by internalFormat, but it may not match exactly.
    /// 
    /// The glPixelStore mode affects texture images.
    /// 
    /// data may be a null pointer.
    /// In this case texture memory is allocated to accommodate a texture of width width, height height, and depth depth.
    /// You can then download subtextures to initialize this texture memory.
    /// The image is undefined if the user tries to apply an uninitialized portion of the texture image to a primitive.
    /// 
    /// glTexImage3D specifies the three-dimensional texture for the current texture unit, specified with glActiveTexture.
    /// 
    /// GL_INVALID_ENUM is generated if target is not GL_TEXTURE_3D or GL_PROXY_TEXTURE_3D.
    /// 
    /// GL_INVALID_ENUM is generated if format is not an accepted format constant. Format constants other than GL_STENCIL_INDEX and GL_DEPTH_COMPONENT are accepted.
    /// 
    /// GL_INVALID_ENUM is generated if type is not a type constant.
    /// 
    /// GL_INVALID_VALUE is generated if level is less than 0.
    /// 
    /// GL_INVALID_VALUE may be generated if level is greater than log2(max), where max is the returned value of GL_MAX_TEXTURE_SIZE.
    /// 
    /// GL_INVALID_VALUE is generated if internalFormat is not one of the accepted resolution and format symbolic constants.
    /// 
    /// GL_INVALID_VALUE is generated if width, height, or depth is less than 0 or greater than GL_MAX_TEXTURE_SIZE.
    /// 
    /// GL_INVALID_VALUE is generated if non-power-of-two textures are not supported and the width, height, or depth cannot be represented as 2k+2(border) for some integer value of k.
    /// 
    /// GL_INVALID_VALUE is generated if border is not 0 or 1.
    /// 
    /// `Error::InvalidOperation` is generated
    /// if `pixel_type` is one of GL_UNSIGNED_BYTE_3_3_2, GL_UNSIGNED_BYTE_2_3_3_REV, GL_UNSIGNED_SHORT_5_6_5, or GL_UNSIGNED_SHORT_5_6_5_REV and format is not GL_RGB,
    /// if type is one of GL_UNSIGNED_SHORT_4_4_4_4, GL_UNSIGNED_SHORT_4_4_4_4_REV, GL_UNSIGNED_SHORT_5_5_5_1, GL_UNSIGNED_SHORT_1_5_5_5_REV, GL_UNSIGNED_INT_8_8_8_8, GL_UNSIGNED_INT_8_8_8_8_REV, GL_UNSIGNED_INT_10_10_10_2, or GL_UNSIGNED_INT_2_10_10_10_REV and format is neither GL_RGBA nor GL_BGRA,
    /// if format or internalFormat is GL_DEPTH_COMPONENT, GL_DEPTH_COMPONENT16, GL_DEPTH_COMPONENT24, or GL_DEPTH_COMPONENT32,
    /// if a non-zero buffer object name is bound to the GL_PIXEL_UNPACK_BUFFER target and the buffer object's data store is currently mapped,
    /// if a non-zero buffer object name is bound to the GL_PIXEL_UNPACK_BUFFER target and the data would be unpacked from the buffer object such that the memory reads required would exceed the data store size,
    /// if a non-zero buffer object name is bound to the GL_PIXEL_UNPACK_BUFFER target and data is not evenly divisible into the number of bytes needed to store in memory a datum indicated by type.
    pub unsafe fn allocate_3d(
        target:Texture3DAllocateTarget,
        level:i32,
        internal_format:TextureInternalFormat,
        width:i32,
        height:i32,
        depth:i32,
        border:i32,
        pixel_format:PixelFormat,
        pixel_type:PixelType,
        data:*const GLvoid
    ){
        glTexImage3D(
            target as GLenum,
            level,
            internal_format as GLint,
            width,
            height,
            depth,
            border,
            pixel_format as GLenum,
            pixel_type as GLenum,
            data,
        )
    }
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum Texture2DWriteTarget{
    Texture2D=TEXTURE_2D,
    TEXTURE_CUBE_MAP_POSITIVE_X=TEXTURE_CUBE_MAP_POSITIVE_X,
    TEXTURE_CUBE_MAP_NEGATIVE_X=TEXTURE_CUBE_MAP_NEGATIVE_X,
    TEXTURE_CUBE_MAP_POSITIVE_Y=TEXTURE_CUBE_MAP_POSITIVE_Y,
    TEXTURE_CUBE_MAP_NEGATIVE_Y=TEXTURE_CUBE_MAP_NEGATIVE_Y,
    TEXTURE_CUBE_MAP_POSITIVE_Z=TEXTURE_CUBE_MAP_POSITIVE_Z,
    TEXTURE_CUBE_MAP_NEGATIVE_Z=TEXTURE_CUBE_MAP_NEGATIVE_Z,
    Texture1DArray=TEXTURE_1D_ARRAY,
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum Texture3DWriteTarget{
    Texture3D=TEXTURE_3D,
    Texture2DArray=TEXTURE_2D_ARRAY,
}

impl Texture{
    pub unsafe fn write_2d(
        target:Texture2DWriteTarget,
        level:GLint,
        xoffset:GLint,
        yoffset:GLint,
        width:GLsizei,
        height:GLsizei,
        pixel_format:PixelFormat,
        pixel_type:PixelType,
        data:*const GLvoid
    ){
        glTexSubImage2D(
            target as GLenum,
            level,
            xoffset,
            yoffset,
            width,
            height,
            pixel_format  as GLenum,
            pixel_type as GLenum,
            data
        )
    }

    pub unsafe fn write_3d(
        target:Texture3DWriteTarget,
        level:GLint,
        xoffset:GLint,
        yoffset:GLint,
        zoffset:GLint,
        width:GLsizei,
        height:GLsizei,
        depth:GLsizei,
        pixel_format:PixelFormat,
        pixel_type:PixelType,
        data:*const GLvoid
    ){
        glTexSubImage3D(
            target as GLenum,
            level,
            xoffset,
            yoffset,
            zoffset,
            width,
            height,
            depth,
            pixel_format  as GLenum,
            pixel_type as GLenum,
            data
        )
    }
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum TextureParameterTarget{
    Texture1D=TEXTURE_1D,
    Texture2D=TEXTURE_2D,
    Texture3D=TEXTURE_3D,
    TextureRectable=TEXTURE_RECTANGLE,
    TextureCubeMap=TEXTURE_CUBE_MAP,
    Texture1DArray=TEXTURE_1D_ARRAY,
    Texture2DArray=TEXTURE_2D_ARRAY,
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum TextureParameter{
    /// Specifies the index of the lowest defined mipmap level.
    /// This is an integer value.
    /// The initial value is 0.
    BASE_LEVEL=TEXTURE_BASE_LEVEL,
    COMPARE_FUNC=TEXTURE_COMPARE_FUNC,
    COMPARE_MODE=TEXTURE_COMPARE_MODE,
    LOD_BIAS=TEXTURE_LOD_BIAS,
    MinFilter=TEXTURE_MIN_FILTER,
    MagFilter=TEXTURE_MAG_FILTER,

    /// Sets the minimum level-of-detail parameter.
    /// This floating-point value limits the selection of highest resolution mipmap (lowest mipmap level).
    /// The initial value is -1000.
    MIN_LOD=TEXTURE_MIN_LOD,

    /// Sets the maximum level-of-detail parameter.
    /// This floating-point value limits the selection of the lowest resolution mipmap (highest mipmap level).
    /// The initial value is 1000.
    MAX_LOD=TEXTURE_MAX_LOD,

    /// Sets the index of the highest defined mipmap level.
    /// This is an integer value.
    /// The initial value is 1000.
    MAX_LEVEL=TEXTURE_MAX_LEVEL,
    SWIZZLE_R=TEXTURE_SWIZZLE_R,
    SWIZZLE_G=TEXTURE_SWIZZLE_G,
    SWIZZLE_B=TEXTURE_SWIZZLE_B,
    SWIZZLE_A=TEXTURE_SWIZZLE_A,

    /// Sets the wrap parameter for texture coordinate `s`.
    /// 
    /// Initialy, is set to `TextureWrap::Repeat`
    WRAP_S=TEXTURE_WRAP_S,

    /// Sets the wrap parameter for texture coordinate `t`.
    /// 
    /// Initialy, is set to `TextureWrap::Repeat`
    WRAP_T=TEXTURE_WRAP_T,

    /// Sets the wrap parameter for texture coordinate `r`.
    /// 
    /// Initialy, is set to `TextureWrap::Repeat`
    WRAP_R=TEXTURE_WRAP_R
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum TextureParameterv{
    /// Specifies the index of the lowest defined mipmap level.
    /// This is an integer value.
    /// The initial value is 0.
    BaseLevel=TEXTURE_BASE_LEVEL,
    BorderColor=TEXTURE_BORDER_COLOR,
    COMPARE_FUNC=TEXTURE_COMPARE_FUNC,
    COMPARE_MODE=TEXTURE_COMPARE_MODE,
    LOD_BIAS=TEXTURE_LOD_BIAS,
    MinFilter=TEXTURE_MIN_FILTER,
    MagFilter=TEXTURE_MAG_FILTER,

    /// Sets the minimum level-of-detail parameter.
    /// This floating-point value limits the selection of highest resolution mipmap (lowest mipmap level).
    /// The initial value is -1000.
    MinLod=TEXTURE_MIN_LOD,

    /// Sets the maximum level-of-detail parameter.
    /// This floating-point value limits the selection of the lowest resolution mipmap (highest mipmap level).
    /// The initial value is 1000.
    MaxLod=TEXTURE_MAX_LOD,

    /// Sets the index of the highest defined mipmap level.
    /// This is an integer value.
    /// The initial value is 1000.
    MaxLevel=TEXTURE_MAX_LEVEL,
    SWIZZLE_R=TEXTURE_SWIZZLE_R,
    SWIZZLE_G=TEXTURE_SWIZZLE_G,
    SWIZZLE_B=TEXTURE_SWIZZLE_B,
    SWIZZLE_A=TEXTURE_SWIZZLE_A,
    SWIZZLE_RGBA=TEXTURE_SWIZZLE_RGBA,

    /// Sets the wrap parameter for texture coordinate `s`.
    /// 
    /// Initialy, is set to `TextureWrap::Repeat`
    WrapS=TEXTURE_WRAP_S,

    /// Sets the wrap parameter for texture coordinate `t`.
    /// 
    /// Initialy, is set to `TextureWrap::Repeat`
    WrapT=TEXTURE_WRAP_T,

    /// Sets the wrap parameter for texture coordinate `r`.
    /// 
    /// Initialy, is set to `TextureWrap::Repeat`
    WrapR=TEXTURE_WRAP_R
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum TextureMinFilter{
    /// Returns the value of the texture element
    /// that is nearest (in Manhattan distance) to the specified texture coordinates.
    Nearest=NEAREST,

    /// Returns the weighted average of the four texture elements that are closest to the specified texture coordinates.
    /// These can include items wrapped or repeated from other parts of a texture,
    /// depending on the values of `GL_TEXTURE_WRAP_S` and `GL_TEXTURE_WRAP_T`,and on the exact mapping.
    Linear=LINEAR,

    /// Chooses the mipmap that most closely matches the size of the pixel being textured
    /// and uses the `GL_NEAREST` criterion (the texture element closest to the specified texture coordinates)
    /// to produce a texture value.
    NearestMipmapNearest=NEAREST_MIPMAP_NEAREST,

    /// Chooses the mipmap that most closely matches the size of the pixel being textured
    /// and uses the `GL_LINEAR` criterion (a weighted average of the four texture elements that are closest to the specified texture coordinates)
    /// to produce a texture value.
    LinearMipmapNearest=LINEAR_MIPMAP_NEAREST,

    /// Chooses the two mipmaps that most closely match the size of the pixel being textured
    /// and uses the GL_NEAREST criterion (the texture element closest to the specified texture coordinates )
    /// to produce a texture value from each mipmap.
    /// The final texture value is a weighted average of those two values.
    NearestMipmapLinear=NEAREST_MIPMAP_LINEAR,

    /// Chooses the two mipmaps that most closely match the size of the pixel being textured
    /// and uses the GL_LINEAR criterion (a weighted average of the texture elements that are closest to the specified texture coordinates)
    /// to produce a texture value from each mipmap.
    /// The final texture value is a weighted average of those two values.
    LinearMipmapLinear=LINEAR_MIPMAP_LINEAR,
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum TextureMagFilter{
    /// Returns the value of the texture element
    /// that is nearest (in Manhattan distance) to the specified texture coordinates.
    Nearest=NEAREST,

    /// Returns the weighted average of the four texture elements that are closest to the specified texture coordinates.
    /// These can include items wrapped or repeated from other parts of a texture,
    /// depending on the values of `GL_TEXTURE_WRAP_S` and `GL_TEXTURE_WRAP_T`,and on the exact mapping.
    Linear=LINEAR,
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum TextureCompareMode{
    /// Specifies that the interpolated and clamped `r` texture coordinate
    /// should be compared to the value in the currently bound depth texture.
    /// 
    /// The result of the comparison is assigned to the red channel.
    CompareRefToTexture=COMPARE_REF_TO_TEXTURE,

    /// Specifies that the red channelshould be assigned
    /// the appropriate value from the currently bound depth texture.
    None=NONE,
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum TextureWrap{
    Repeat=REPEAT,
    MirroredRepeat=MIRRORED_REPEAT,
    ClampToEdge=CLAMP_TO_EDGE,
    ClampToBorder=CLAMP_TO_BORDER,
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum TextureCompareFunction{
    LessEqual=LEQUAL,
    GreaterEqual=GEQUAL,
    Less=LESS,
    Greater=GREATER,
    Equal=EQUAL,
    NotEqual=NOTEQUAL,
    Always=ALWAYS,
    Never=NEVER,
}

/// Set texture parameters
/// 
/// `target` specifies the target texture.
/// 
/// `parameter` specifies the symbolic name of a single-valued texture parameter.
/// 
/// `values` specifies the values of `parameter`.
/// 
/// Assign the value or values in `value` to the texture parameter specified as `parameter`.
/// 
/// Suppose that a program attempts to sample from a texture and has set GL_TEXTURE_MIN_FILTER to one of the functions that requires a mipmap.
/// If either the dimensions of the texture images currently defined (with previous calls to glTexImage1D, glTexImage2D, glTexImage3D, glCopyTexImage1D, or glCopyTexImage2D) do not follow the proper sequence for mipmaps (described above), or there are fewer texture images defined than are needed, or the set of texture images have differing numbers of texture components, then the texture is considered incomplete.
/// 
/// Linear filtering accesses the four nearest texture elements only in 2D textures.
/// In 1D textures, linear filtering accesses the two nearest texture elements.
/// In 3D textures, linear filtering accesses the eight nearest texture elements.
/// 
/// glTexParameter specifies the texture parameters for the active texture unit, specified by calling glActiveTexture.
/// 
/// `Error::InvalidEnum` is generated if `target` or `parameter` is not one of the accepted defined values,
/// if `values` should have a defined constant value (based on the value of `parameter) and does not.
impl Texture{
    #[inline(always)]
    pub unsafe fn set_parameterf(target:TextureParameterTarget,parameter:TextureParameter,value:f32){
        glTexParameterf(target as GLenum,parameter as GLenum,value)
    }

    #[inline(always)]
    pub unsafe fn set_parameteri(target:TextureParameterTarget,parameter:TextureParameter,value:i32){
        glTexParameteri(target as GLenum,parameter as GLenum,value)
    }

    #[inline(always)]
    pub unsafe fn set_parameterfv(target:TextureParameterTarget,parameter:TextureParameterv,values:*const f32){
        glTexParameterfv(target as GLenum,parameter as GLenum,values)
    }

    #[inline(always)]
    pub unsafe fn set_parameteriv(target:TextureParameterTarget,parameter:TextureParameterv,values:*const i32){
        glTexParameteriv(target as GLenum,parameter as GLenum,values)
    }

    #[inline(always)]
    pub unsafe fn set_parameterIiv(target:TextureParameterTarget,parameter:TextureParameterv,values:*const i32){
        glTexParameterIiv(target as GLenum,parameter as GLenum,values)
    }

    #[inline(always)]
    pub unsafe fn set_parameterIuiv(target:TextureParameterTarget,parameter:TextureParameterv,values:*const u32){
        glTexParameterIuiv(target as GLenum,parameter as GLenum,values)
    }
}