#[cfg(any(windows))]
use crate::windows::OpenGraphicsLibrary;

use core::mem::transmute;

// Texture targets
const TEXTURE_1D:u32=0x0DE0;
const TEXTURE_2D:u32=0x0DE1;
const PROXY_TEXTURE_1D:u32=0x8063;
const PROXY_TEXTURE_2D:u32=0x8064;
const TEXTURE_3D:u32=0x806F;
const TEXTURE_RECTANGLE:u32=0x84F5;
const TEXTURE_CUBE_MAP:u32=0x8513;
const TEXTURE_1D_ARRAY:u32=0x8C18;
const TEXTURE_2D_ARRAY:u32=0x8C1A;
const TEXTURE_BUFFER:u32=0x8C2A;
const TEXTURE_2D_MULTISAMPLE:u32=0x9100;
const TEXTURE_2D_MULTISAMPLE_ARRAY:u32=0x9102;

// Texture internal formats
const R8:u32=0x8229;
const R16:u32=0x822A;
const R16F:u32=0x822D;
const R16I:u32=0x8233;
const R16UI:u32=0x8234;
const R16_SNORM:u32=0x8F98;
const R32F:u32=0x822E;
const R32I:u32=0x8235;
const R32UI:u32=0x8236;
const R3_G3_B2:u32=0x2A10;
const R8I:u32=0x8231;
const R8UI:u32=0x8232;
const R8_SNORM:u32=0x8F94;
const RG:u32=0x8227;
const RG16:u32=0x822C;
const RG16F:u32=0x822F;
const RG16I:u32=0x8239;
const RG16UI:u32=0x823A;
const RG16_SNORM:u32=0x8F99;
const RG32F:u32=0x8230;
const RG32I:u32=0x823B;
const RG32UI:u32=0x823C;
const RG8:u32=0x822B;
const RG8I:u32=0x8237;
const RG8UI:u32=0x8238;
const RG8_SNORM:u32=0x8F95;
const RGB:u32=0x1907;
const RGB10:u32=0x8052;
const RGB10_A2:u32=0x8059;
const RGB10_A2UI:u32=0x906F;
const RGB12:u32=0x8053;
const RGB16:u32=0x8054;
const RGB16F:u32=0x881B;
const RGB16I:u32=0x8D89;
const RGB16UI:u32=0x8D77;
const RGB16_SNORM:u32=0x8F9A;
const RGB32F:u32=0x8815;
const RGB32I:u32=0x8D83;
const RGB32UI:u32=0x8D71;
const RGB4:u32=0x804F;
const RGB5:u32=0x8050;
const RGB565:u32=0x8D62;
const RGB5_A1:u32=0x8057;
const RGB8:u32=0x8051;
const RGB8I:u32=0x8D8F;
const RGB8UI:u32=0x8D7D;
const RGB8_SNORM:u32=0x8F96;
const RGB9_E5:u32=0x8C3D;
const RGBA12:u32=0x805A;
const RGBA16:u32=0x805B;
const RGBA16F:u32=0x881A;
const RGBA16I:u32=0x8D88;
const RGBA16UI:u32=0x8D76;
const RGBA16_SNORM:u32=0x8F9B;
const RGBA2:u32=0x8055;
const RGBA32F:u32=0x8814;
const RGBA32I:u32=0x8D82;
const RGBA32UI:u32=0x8D70;
const RGBA4:u32=0x8056;
const RGBA8I:u32=0x8D8E;
const RGBA8UI:u32=0x8D7C;
const RGBA8_SNORM:u32=0x8F97;
const SRGB:u32=0x8C40;
const SRGB8:u32=0x8C41;
const SRGB8_ALPHA8:u32=0x8C43;
const RGBA8:u32=0x8058;
const R11F_G11F_B10F:u32=0x8C3A;

// Image data formats
const RED:u32=0x1903;
const BGR:u32=0x80E0;
const RGBA:u32=0x1908;
const BGRA:u32=0x80E1;

// Image data type
const BYTE:u32=0x1400;
const UNSIGNED_BYTE:u32=0x1401;
const UNSIGNED_BYTE_2_3_3_REV:u32=0x8362;
const UNSIGNED_BYTE_3_3_2:u32=0x8032;
const SHORT:u32=0x1402;
const UNSIGNED_SHORT:u32=0x1403;
const UNSIGNED_SHORT_1_5_5_5_REV:u32=0x8366;
const UNSIGNED_SHORT_4_4_4_4:u32=0x8033;
const UNSIGNED_SHORT_4_4_4_4_REV:u32=0x8365;
const UNSIGNED_SHORT_5_5_5_1:u32=0x8034;
const UNSIGNED_SHORT_5_6_5:u32=0x8363;
const UNSIGNED_SHORT_5_6_5_REV:u32=0x8364;
const INT:u32=0x1404;
const FLOAT:u32=0x1406;
const UNSIGNED_INT:u32=0x1405;
const UNSIGNED_INT_10F_11F_11F_REV:u32=0x8C3B;
const UNSIGNED_INT_24_8:u32=0x84FA;
const UNSIGNED_INT_5_9_9_9_REV:u32=0x8C3E;
const UNSIGNED_INT_8_8_8_8:u32=0x8035;
const UNSIGNED_INT_8_8_8_8_REV:u32=0x8367;
const UNSIGNED_INT_2_10_10_10_REV:u32=0x8368;
const UNSIGNED_INT_10_10_10_2:u32=0x8036;

// Parameters
const TEXTURE_MAG_FILTER:u32=0x2800;
const TEXTURE_MIN_FILTER:u32=0x2801;
const TEXTURE_BASE_LEVEL:u32=0x813C;
const TEXTURE_COMPARE_MODE:u32=0x884C;
const TEXTURE_COMPARE_FUNC:u32=0x884D;
const TEXTURE_WRAP_R:u32=0x8072;
const TEXTURE_WRAP_S:u32=0x2802;
const TEXTURE_WRAP_T:u32=0x2803;

// Compare functions
const NEVER:u32=0x0200;
const LESS:u32=0x0201;
const EQUAL:u32=0x0202;
const LEQUAL:u32=0x0203;
const GREATER:u32=0x0204;
const NOTEQUAL:u32=0x0205;
const GEQUAL:u32=0x0206;
const ALWAYS:u32=0x0207;

// Compare modes
const NONE:u32=0;
const COMPARE_REF_TO_TEXTURE:u32=0x884E;

// Texture filters
const NEAREST:u32=0x2600;
const NEAREST_MIPMAP_LINEAR:u32=0x2702;
const NEAREST_MIPMAP_NEAREST:u32=0x2700;
const LINEAR:u32=0x2601;
const LINEAR_MIPMAP_LINEAR:u32=0x2703;
const LINEAR_MIPMAP_NEAREST:u32=0x2701;

// Texture wrap
const REPEAT:u32=0x2901;
const MIRRORED_REPEAT:u32=0x8370;
const CLAMP_TO_EDGE:u32=0x812F;
const CLAMP_TO_BORDER:u32=0x812D;

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum TextureBindTarget{
    Texture1D=TEXTURE_1D,
    Texture2D=TEXTURE_2D,
    Texture3D=TEXTURE_3D,
    TextureRectable=TEXTURE_RECTANGLE,
    TextureCubeMap=TEXTURE_CUBE_MAP,
    Texture1DArray=TEXTURE_1D_ARRAY,
    Texture2DArray=TEXTURE_2D_ARRAY,
    TextureBuffer=TEXTURE_BUFFER,
    Texture2DMultisample=TEXTURE_2D_MULTISAMPLE,
    Texture2DMultisampleArray=TEXTURE_2D_MULTISAMPLE_ARRAY,
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
pub enum Texture1DRewriteTarget{
    /// Data is read from data as a sequence of signed or unsigned bytes,
    /// shorts, or longs, or single-precision floating-point values, depending on type.
    /// These values are grouped into sets of one,
    /// two, three, or four values, depending on format, to form elements.
    /// Each data byte is treated as eight 1-bit elements,
    /// with bit ordering determined by UNPACK_LSB_FIRST (see glPixelStore).
    Texture1D=TEXTURE_1D,

    /// No data is read from data,
    /// but all of the texture image state is recalculated,
    /// checked for consistency,
    /// and checked against the implementation's capabilities.
    /// If the implementation cannot handle a texture of the requested texture size,
    /// it sets all of the image state to 0,
    /// but does not generate an error.
    /// To query for an entire mipmap array,
    /// use an image array level greater than or equal to 1.
    ProxyTexture1D=PROXY_TEXTURE_1D
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum Texture2DRewriteTarget{
    Texture2D=TEXTURE_2D,
    ProxyTexture2D=PROXY_TEXTURE_2D,
    // TEXTURE_1D_ARRAY,
    // PROXY_TEXTURE_1D_ARRAY,
    // TEXTURE_RECTANGLE,
    // PROXY_TEXTURE_RECTANGLE,
    // TEXTURE_CUBE_MAP_POSITIVE_X,
    // TEXTURE_CUBE_MAP_NEGATIVE_X,
    // TEXTURE_CUBE_MAP_POSITIVE_Y,
    // TEXTURE_CUBE_MAP_NEGATIVE_Y,
    // TEXTURE_CUBE_MAP_POSITIVE_Z,
    // TEXTURE_CUBE_MAP_NEGATIVE_Z,
    // PROXY_TEXTURE_CUBE_MAP
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum Texture2DWriteTarget{
    Texture2D=TEXTURE_2D,
    // TEXTURE_CUBE_MAP_POSITIVE_X,
    // TEXTURE_CUBE_MAP_NEGATIVE_X,
    // TEXTURE_CUBE_MAP_POSITIVE_Y,
    // TEXTURE_CUBE_MAP_NEGATIVE_Y,
    // TEXTURE_CUBE_MAP_POSITIVE_Z,
    // TEXTURE_CUBE_MAP_NEGATIVE_Z,
    // TEXTURE_1D_ARRAY
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum Texture2DCopyTarget{
    Texture2D=TEXTURE_2D,
    // TEXTURE_CUBE_MAP_POSITIVE_X,
    // TEXTURE_CUBE_MAP_NEGATIVE_X,
    // TEXTURE_CUBE_MAP_POSITIVE_Y,
    // TEXTURE_CUBE_MAP_NEGATIVE_Y,
    // TEXTURE_CUBE_MAP_POSITIVE_Z,
    // TEXTURE_CUBE_MAP_NEGATIVE_Z,
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum Texture2DInternalFormat{
    R8=R8,
    // RedI8=R8I,
    // RedU8=R8UI,
    // R8_SNorm=R8_SNORM,
    R16=R16,
    // R16_SNorm=R16_SNORM,
    // RedI16=R16I,
    // RedU16=R16UI,
    // RedF16=R16F,
    // RedI32=R32I,
    // RedU32=R32UI,
    // RedF32=R32F,

    RG8=RG8,
    // RG8_SNorm=RG8_SNORM,
    RG16=RG16,
    // RG16_SNorm=RG16_SNORM,
    // RG16F,

    RGB8=RGB8,
    // SRGB8=SRGB8,
    // RGB8_SNorm=RGB8_SNORM,
    // SRGB8_ALPHA8=SRGB8_ALPHA8,
    RGB16=RGB16,
    // RGB16_SNorm=RGB16_SNORM,

    RGBA8=RGBA8,
    // RGBA8_SNorm=RGBA8_SNORM,

    // RGBA32F,
    // RGBA32I,
    // RGBA32UI,
    RGBA16=RGBA16,
    // RGBA16_SNorm=RGBA16_SNORM,
    // RGBA16F,
    // RGBA16I,
    // RGBA16UI,
    // RGBA8UI,
    // RGB10_A2,
    // RGB10_A2UI,
    // R11F_G11F_B10F,
    // RG32F,
    // RG32I,
    // RG32UI,
    // RG8I,
    // RG8UI,
    // RGB32F,
    // RGB32I,
    // RGB32UI,
    // RGB16F,
    // RGB16I,
    // RGB16UI,
    // RGB8I,
    // RGB8UI,
    // RGB9_E5,
    // COMPRESSED_RG_RGTC2,
    // COMPRESSED_SIGNED_RG_RGTC2,
    // COMPRESSED_RED_RGTC1,
    // COMPRESSED_SIGNED_RED_RGTC1,
    // DEPTH_COMPONENT24,
    // DEPTH_COMPONENT16,
    // DEPTH_COMPONENT32F,
    // DEPTH24_STENCIL8,
    // DEPTH32F_STENCIL8,
}

#[repr(u64)]
#[derive(Clone,Copy,Debug)]
pub enum ImageDataFormat{
    R_I8=RED as u64 | (BYTE as u64)<<32,
    R_U8=RED as u64 | (UNSIGNED_BYTE as u64)<<32,
    R_I16=RED as u64 | (SHORT as u64)<<32,
    R_U16=RED as u64 | (UNSIGNED_SHORT as u64)<<32,
    R_I32=RED as u64 | (INT as u64)<<32,
    R_U32=RED as u64 | (UNSIGNED_INT as u64)<<32,
    R_F32=RED as u64 | (FLOAT as u64)<<32,

    RG_I8=RG as u64 | (BYTE as u64)<<32,
    RG_U8=RG as u64 | (UNSIGNED_BYTE as u64)<<32,
    RG_I16=RG as u64 | (SHORT as u64)<<32,
    RG_U16=RG as u64 | (UNSIGNED_SHORT as u64)<<32,
    RG_I32=RG as u64 | (INT as u64)<<32,
    RG_U32=RG as u64 | (UNSIGNED_INT as u64)<<32,
    RG_F32=RG as u64 | (FLOAT as u64)<<32,

    RGB_I8=RGB as u64 | (BYTE as u64)<<32,
    RGB_U8=RGB as u64 | (UNSIGNED_BYTE as u64)<<32,
    RGB_I16=RGB as u64 | (SHORT as u64)<<32,
    RGB_U16=RGB as u64 | (UNSIGNED_SHORT as u64)<<32,
    RGB_I32=RGB as u64 | (INT as u64)<<32,
    RGB_U32=RGB as u64 | (UNSIGNED_INT as u64)<<32,
    RGB_F32=RGB as u64 | (FLOAT as u64)<<32,
    // R3_G3_B2=RGB as u64 | (UNSIGNED_BYTE_3_3_2 as u64)<<32,
    // B2_G3_R3=RGB as u64 | (UNSIGNED_BYTE_2_3_3_REV as u64)<<32,
    // R5_G6_B5=RGB as u64 | (UNSIGNED_SHORT_5_6_5 as u64)<<32,
    // B5_G6_R5=RGB as u64 | (UNSIGNED_SHORT_5_6_5_REV as u64)<<32,
    // RF10_GF11_BF11=RGB as u64 | (UNSIGNED_INT_10F_11F_11F_REV as u64)<<32,

    BGR_I8=BGR as u64 | (BYTE as u64)<<32,
    BGR_U8=BGR as u64 | (UNSIGNED_BYTE as u64)<<32,
    BGR_I16=BGR as u64 | (SHORT as u64)<<32,
    BGR_U16=BGR as u64 | (UNSIGNED_SHORT as u64)<<32,
    BGR_I32=BGR as u64 | (INT as u64)<<32,
    BGR_U32=BGR as u64 | (UNSIGNED_INT as u64)<<32,
    BGR_F32=BGR as u64 | (FLOAT as u64)<<32,

    RGBA_I8=RGBA as u64 | (BYTE as u64)<<32,
    RGBA_U8=RGBA as u64 | (UNSIGNED_BYTE as u64)<<32,
    RGBA_I16=RGBA as u64 | (SHORT as u64)<<32,
    RGBA_U16=RGBA as u64 | (UNSIGNED_SHORT as u64)<<32,
    RGBA_I32=RGBA as u64 | (INT as u64)<<32,
    RGBA_U32=RGBA as u64 | (UNSIGNED_INT as u64)<<32,
    RGBA_F32=RGBA as u64 | (FLOAT as u64)<<32,
    // RGBA_U4=RGBA as u64 | (UNSIGNED_SHORT_4_4_4_4 as u64)<<32,
    // RGBA_U4_REV=RGBA as u64 | (UNSIGNED_SHORT_4_4_4_4_REV as u64)<<32,
    // R5_G5_B5_A1=RGBA as u64 | (UNSIGNED_SHORT_5_5_5_1 as u64)<<32,
    // R5_G5_B5_A1_REV=RGBA as u64 | (UNSIGNED_SHORT_1_5_5_5_REV as u64)<<32,
    // R8_G8_B8_A8=RGBA as u64 | (UNSIGNED_INT_8_8_8_8 as u64)<<32,
    // R8_G8_B8_A8_REV=RGBA as u64 | (UNSIGNED_INT_8_8_8_8_REV as u64)<<32,
    // R10_G10_B10_A2=RGBA as u64 | (UNSIGNED_INT_10_10_10_2 as u64)<<32,
    // R10_G10_B10_A2_REV=RGBA as u64 | (UNSIGNED_INT_2_10_10_10_REV as u64)<<32,

    BGRA_I8=BGRA as u64 | (BYTE as u64)<<32,
    BGRA_U8=BGRA as u64 | (UNSIGNED_BYTE as u64)<<32,
    BGRA_I16=BGRA as u64 | (SHORT as u64)<<32,
    BGRA_U16=BGRA as u64 | (UNSIGNED_SHORT as u64)<<32,
    BGRA_I32=BGRA as u64 | (INT as u64)<<32,
    BGRA_U32=BGRA as u64 | (UNSIGNED_INT as u64)<<32,
    BGRA_F32=BGRA as u64 | (FLOAT as u64)<<32,
    BGRA_U4=BGRA as u64 | (UNSIGNED_SHORT_4_4_4_4 as u64)<<32,
    // BGRA_U4_REV=BGRA as u64 | (UNSIGNED_SHORT_4_4_4_4_REV as u64)<<32,
    // B5_G5_R5_A1=BGRA as u64 | (UNSIGNED_SHORT_5_5_5_1 as u64)<<32,
    // B5_G5_R5_A1_REV=BGRA as u64 | (UNSIGNED_SHORT_1_5_5_5_REV as u64)<<32,
    // B8_G8_R8_A8=BGRA as u64 | (UNSIGNED_INT_8_8_8_8 as u64)<<32,
    // B8_G8_R8_A8_REV=BGRA as u64 | (UNSIGNED_INT_8_8_8_8_REV as u64)<<32,
    // B10_G10_R10_A2=BGRA as u64 | (UNSIGNED_INT_10_10_10_2 as u64)<<32,
    // B10_G10_R10_A2_REV=BGRA as u64 | (UNSIGNED_INT_2_10_10_10_REV as u64)<<32,
}

impl ImageDataFormat{
    /// [image data format,image data type]
    fn to_gl_enums(self)->[u32;2]{
        unsafe{
            transmute(self)
        }
    }
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
pub enum TextureWrap{
    Repeat=REPEAT,
    MirroredRepeat=MIRRORED_REPEAT,
    ClampToEdge=CLAMP_TO_EDGE,
    ClampToBorder=CLAMP_TO_BORDER,
}


pub struct Texture{
    glGenTextures:usize,
    glDeleteTextures:usize,

    glBindTexture:usize,

    glTexImage2D:usize,
    glTexSubImage2D:usize,
    glCopyTexSubImage2D:usize,

    glTexParameteri:usize,
}

impl Texture{
    pub const fn new()->Texture{
        Self{
            glGenTextures:0,
            glDeleteTextures:0,

            glBindTexture:0,

            glTexImage2D:0,
            glTexSubImage2D:0,
            glCopyTexSubImage2D:0,

            glTexParameteri:0,
        }
    }

    #[cfg(any(windows))]
    pub fn load(&mut self,library:&OpenGraphicsLibrary){
        unsafe{
            self.glGenTextures=transmute(library.get_proc_address("glGenTextures\0"));
            self.glDeleteTextures=transmute(library.get_proc_address("glDeleteTextures\0"));

            self.glBindTexture=transmute(library.get_proc_address("glBindTexture\0"));

            self.glTexImage2D=transmute(library.get_proc_address("glTexImage2D\0"));
            self.glTexSubImage2D=transmute(library.get_proc_address("glTexSubImage2D\0"));
            self.glCopyTexSubImage2D=transmute(library.get_proc_address("glCopyTexSubImage2D\0"));

            self.glTexParameteri=transmute(library.get_proc_address("glTexParameteri\0"));
        }
    }
}

impl Texture{
    /// Generates a texture name.
    #[inline(always)]
    pub fn generate_one(&self,texture:&mut u32){
        unsafe{
            transmute::<usize,fn(i32,&mut u32)>(self.glGenTextures)(1,texture)
        }
    }

    /// Deletes a named texture.
    /// 
    /// Silently ignores 0's and names that do not correspond to existing textures.
    /// 
    /// If a texture that is currently bound is deleted,
    /// the binding reverts to 0 (the default texture).
    #[inline(always)]
    pub unsafe fn delete_one(&self,texture:&u32){
        transmute::<usize,fn(i32,&u32)>(self.glDeleteTextures)(1,texture)
    }

    /// Generates texture names.
    #[inline(always)]
    pub unsafe fn generate(&self,textures:&mut [u32]){
        transmute::<usize,fn(i32,&mut u32)>(self.glGenTextures)(textures.len() as i32,&mut textures[0])
    }

    /// Deletes named textures.
    /// 
    /// Silently ignores 0's and names that do not correspond to existing textures.
    /// 
    /// If a texture that is currently bound is deleted,
    /// the binding reverts to 0 (the default texture).
    #[inline(always)]
    pub fn delete(&self,textures:&[u32]){
        unsafe{
            transmute::<usize,fn(i32,&u32)>(self.glDeleteTextures)(textures.len() as i32,&textures[0])
        }
    }

    /// Binds a texture to a texturing target.
    /// 
    /// When a texture is bound to a target,
    /// the previous binding for that target is automatically broken.
    /// 
    /// When a texture is first bound, it assumes the specified target:
    /// A texture first bound to `GL_TEXTURE_1D` becomes one-dimensional texture,
    /// a texture first bound to `GL_TEXTURE_2D` becomes two-dimensional texture,
    /// and so on.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if target is not a name returned from a previous call to glGenTextures.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if texture was previously created with a target that doesn't match that of target.
    #[inline(always)]
    pub unsafe fn bind(&self,target:TextureBindTarget,texture_id:u32){
        transmute::<usize,fn(TextureBindTarget,u32)>(self.glBindTexture)(target,texture_id)
    }
}

    // TEXTURE_LOD_BIAS,
    // TEXTURE_MIN_LOD,
    // TEXTURE_MAX_LOD,
    // TEXTURE_MAX_LEVEL,
    // TEXTURE_SWIZZLE_R,
    // TEXTURE_SWIZZLE_G,
    // TEXTURE_SWIZZLE_B,
    // TEXTURE_SWIZZLE_A,

/// Texture parameters.
impl Texture{
    /// Specifies the index of the lowest defined mipmap level.
    /// 
    /// The initial value is 0.
    #[inline(always)]
    pub fn set_base_level(&self,target:TextureParameterTarget,level:i32){
        unsafe{
            transmute::<usize,fn(TextureParameterTarget,u32,i32)>(self.glTexParameteri)(target,TEXTURE_BASE_LEVEL,level)
        }
    }

    /// Specifies the comparison operator.
    /// 
    /// The comparison operator is used when `TextureCompareMode::CompareRefToTexture` is set (see `Texture::set_compare_mode`).
    #[inline(always)]
    pub fn set_compare_function(&self,target:TextureParameterTarget,function:TextureCompareFunction){
        unsafe{
            transmute::<usize,fn(TextureParameterTarget,u32,TextureCompareFunction)>(self.glTexParameteri)(target,TEXTURE_COMPARE_FUNC,function)
        }
    }

    /// Specifies the texture comparison mode for currently bound depth textures (the iternal format = `DEPTH_COMPONENT`).
    #[inline(always)]
    pub fn set_compare_mode(&self,target:TextureParameterTarget,mode:TextureCompareMode){
        unsafe{
            transmute::<usize,fn(TextureParameterTarget,u32,TextureCompareMode)>(self.glTexParameteri)(target,TEXTURE_COMPARE_MODE,mode)
        }
    }

    /// Specifies the texture magnification function.
    /// 
    /// The texture magnification function is used whenever the level-of-detail function used
    /// when sampling from the texture determines that the texture should be magified.
    /// 
    /// Initially, it is set to `TextureMagFilter::Linear`.
    #[inline(always)]
    pub fn set_mag_filter(&self,target:TextureParameterTarget,filter:TextureMagFilter){
        unsafe{
            transmute::<usize,fn(TextureParameterTarget,u32,TextureMagFilter)>(self.glTexParameteri)(target,TEXTURE_MAG_FILTER,filter)
        }
    }

    /// Specifies the texture minifying function.
    /// 
    /// The texture minifying function is used whenever the level-of-detail function used
    /// when sampling from the texture determines that the texture should be minified.
    /// 
    /// Initially, it is set to `TextureMinFilter::LinearMipmapLinear`.
    #[inline(always)]
    pub fn set_min_filter(&self,target:TextureParameterTarget,filter:TextureMinFilter){
        unsafe{
            transmute::<usize,fn(TextureParameterTarget,u32,TextureMinFilter)>(self.glTexParameteri)(target,TEXTURE_MIN_FILTER,filter)
        }
    }

    /// Sets the wrap parameter for texture coordinate `s`.
    /// 
    /// Initially, it is set to `TextureWrap::Repeat`.
    #[inline(always)]
    pub fn set_wrap_s(&self,target:TextureParameterTarget,value:TextureWrap){
        unsafe{
            transmute::<usize,fn(TextureParameterTarget,u32,TextureWrap)>(self.glTexParameteri)(target,TEXTURE_WRAP_S,value)
        }
    }

    /// Sets the wrap parameter for texture coordinate `t`.
    /// 
    /// Initially, it is set to `TextureWrap::Repeat`.
    #[inline(always)]
    pub fn set_wrap_t(&self,target:TextureParameterTarget,value:TextureWrap){
        unsafe{
            transmute::<usize,fn(TextureParameterTarget,u32,TextureWrap)>(self.glTexParameteri)(target,TEXTURE_WRAP_T,value)
        }
    }

    /// Sets the wrap parameter for texture coordinate `r`.
    /// 
    /// Initially, it is set to `TextureWrap::Repeat`.
    #[inline(always)]
    pub fn set_wrap_r(&self,target:TextureParameterTarget,value:TextureWrap){
        unsafe{
            transmute::<usize,fn(TextureParameterTarget,u32,TextureWrap)>(self.glTexParameteri)(target,TEXTURE_WRAP_R,value)
        }
    }
}

/// Texture 2D data.
impl Texture{
    /// Specify a two-dimensional texture image.
    /// 
    /// `target` - Specifies the target texture.
    /// 
    /// `mipmap_level` - Specifies the level-of-detail number.
    /// Level 0 is the base image level.
    /// Level n is the nth mipmap reduction image.
    /// If target is `Texture2DRewriteTarget::TextureRectangle`
    /// or `Texture2DRewriteTarget::ProxyTextureRectangle`,
    /// level must be 0.
    /// 
    /// `internal_format` - Specifies the number of color components in the texture.
    /// 
    /// `width` - Specifies the width of the texture image.
    /// All implementations support texture images that are at least 1024 texels wide.
    /// 
    /// `height` - Specifies the height of the texture image, or the number of layers in a texture array,
    /// in the case of the `Texture1DArray` and `ProxyTexture1DArray` targets.
    /// All implementations support 2D texture images that are at least 1024 texels high,
    /// and texture arrays that are at least 256 layers deep.
    /// 
    /// `image_format` - Specifies the format of the pixel data.
    /// 
    /// `image_data_type` - Specifies the data type of the pixel data.
    /// 
    /// `data` - Specifies a pointer to the image data in memory.
    /// 
    /// `GLError::InvalidEnum` is generated
    /// if `target` is one of the six cube map 2D image targets
    /// and the width and height parameters are not equal.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if `width` is less than `0` or greater than `GL_MAX_TEXTURE_SIZE`,
    /// if `target` is not `GL_TEXTURE_1D_ARRAY` or `GL_PROXY_TEXTURE_1D_ARRAY`
    /// and `height` is less than `0` or greater than `GL_MAX_TEXTURE_SIZE`,
    /// if `target` is `GL_TEXTURE_1D_ARRAY` or `GL_PROXY_TEXTURE_1D_ARRAY`
    /// and `height` is less than `0` or greater than `GL_MAX_ARRAY_TEXTURE_LAYERS`,
    /// if `mipmap_level` is less than 0,
    /// if `mipmap_level` is greater than log2(max),
    /// where `max` is the returned value of `GL_MAX_TEXTURE_SIZE`,
    /// if `width` or `height` is less than 0 or greater than `GL_MAX_TEXTURE_SIZE,
    /// if non-power-of-two textures are not supported
    /// and the width or height cannot be represented
    /// as `2^k` for some integer value of `k`,
    /// if target is `GL_TEXTURE_RECTANGLE` or `GL_PROXY_TEXTURE_RECTANGLE` and `mipmap_level` is not `0`.
    #[inline(always)]
    pub unsafe fn rewrite_image_2d<I:Sized>(
        &self,
        target:Texture2DRewriteTarget,
        mipmap_level:i32,
        internal_format:Texture2DInternalFormat,
        [width,height]:[i32;2],
        image_data_format:ImageDataFormat,
        data:*const I,
    ){
        let [image_format,image_type]=image_data_format.to_gl_enums();
        transmute::<usize,fn(
            Texture2DRewriteTarget,
            i32,
            Texture2DInternalFormat,
            i32,
            i32,
            i32,
            u32,
            u32,
            *const I
        )>(self.glTexImage2D)(
            target,
            mipmap_level,
            internal_format,
            width,
            height,
            0,
            image_format,
            image_type,
            data
        )
    }

    /// Specify a two-dimensional texture subimage.
    /// 
    /// `target` - Specifies the target texture.
    /// 
    /// `mipmap_level` - Specifies the level-of-detail number.
    /// Level 0 is the base image level.
    /// Level n is the nth mipmap reduction image.
    /// 
    /// `x` - Specifies a texel offset in the x direction within the texture array.
    /// 
    /// `y` - Specifies a texel offset in the y direction within the texture array.
    /// 
    /// `width` - Specifies the width of the texture subimage.
    /// 
    /// `height` - Specifies the height of the texture subimage.
    /// 
    /// `image_format` - Specifies the format of the pixel data.
    /// 
    /// `image_data_type` - Specifies the data type of the pixel data.
    /// 
    /// `data` - Specifies a pointer to the image data in memory.
    #[inline(always)]
    pub unsafe fn write_image_2d<I:Sized>(
        &self,
        target:Texture2DWriteTarget,
        mipmap_level:i32,
        [x,y,width,height]:[i32;4],
        image_data_format:ImageDataFormat,
        data:*const I
    ){
        let [image_format,image_type]=image_data_format.to_gl_enums();
        transmute::<usize,fn(
            Texture2DWriteTarget,
            i32,
            i32,
            i32,
            i32,
            i32,
            u32,
            u32,
            *const I
        )>(self.glTexSubImage2D)(
            target,
            mipmap_level,
            x,
            y,
            width,
            height,
            image_format,
            image_type,
            data,
        )
    }

    /// Copies a two-dimensional texture subimage.
    /// 
    /// Replaces a rectangular portion of a two-dimensional texture image or cube-map texture image with pixels from the current `READ_BUFFER`.
    /// 
    /// `target` - Specifies the target texture.
    /// 
    /// `mipmap_level` - Specifies the level-of-detail number.
    /// Level 0 is the base image level.
    /// Level n is the nth mipmap reduction image.
    /// 
    /// `read_x`, `read_y` - Specify the window coordinates of the lower left corner
    /// of the rectangular region of pixels to be copied.
    /// 
    /// `write_x` - Specifies a texel offset in the x direction within the texture array.
    /// 
    /// `write_y` - Specifies a texel offset in the y direction within the texture array.
    /// 
    /// `width` - Specifies the width of the texture subimage.
    /// 
    /// `height` - Specifies the height of the texture subimage.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if the texture array has not been defined
    /// by a previous `Texture::rewrite_image_2d()` or `Texture::copy_image_2d()` operation.
    /// 
    /// `GLError::InvalidValue` is generated if `mipmap_level` is less than 0,
    /// if `mipmap_level>log2(max)`, where `max` is the returned value of `MAX_TEXTURE_SIZE`,
    /// if `write_x<0`, `(write_x+width)>w`, `write_y<0`, or `(write_y+height)>h`,
    /// where `w` is the `TEXTURE_WIDTH`, and `h` is the `TEXTURE_HEIGHT` of the texture image being modified.
    #[inline(always)]
    pub unsafe fn copy_image_2d(
        &self,
        target:Texture2DCopyTarget,
        mipmap_level:i32,
        [read_x,read_y]:[i32;2],
        [write_x,write_y]:[i32;2],
        [width,height]:[i32;2]
    ){
        transmute::<usize,fn(Texture2DCopyTarget,i32,i32,i32,i32,i32,i32,i32)>(self.glCopyTexSubImage2D)(
            target,
            mipmap_level,
            write_x,write_y,
            read_x,read_y,width,height
        )
    }
}