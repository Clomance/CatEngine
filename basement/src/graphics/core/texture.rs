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
const RGBA8:u32=0x8058;
const R8:u32=0x8229;

// Image data formats
const RED:u32=0x1903;
const RGBA:u32=0x1908;

// Image data type
pub const UNSIGNED_BYTE:u32=0x1401;

// Parameters
const TEXTURE_MAG_FILTER:u32=0x2800;
const TEXTURE_MIN_FILTER:u32=0x2801;
const TEXTURE_BASE_LEVEL:u32=0x813C;
const TEXTURE_COMPARE_MODE:u32=0x884C;
const TEXTURE_COMPARE_FUNC:u32=0x884D;

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
    /// with bit ordering determined by GL_UNPACK_LSB_FIRST (see glPixelStore).
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
    // GL_R8I,
    // GL_R8UI,
    // GL_R16I,
    // GL_R16UI,
    // GL_R16F,
    // GL_R32I,
    // GL_R32UI,
    // GL_R32F,
    RGBA8=RGBA8,
    // GL_RGBA32F,
    // GL_RGBA32I,
    // GL_RGBA32UI,
    // GL_RGBA16,
    // GL_RGBA16F,
    // GL_RGBA16I,
    // GL_RGBA16UI,
    // GL_RGBA8UI,
    // GL_SRGB8_ALPHA8,
    // GL_RGB10_A2,
    // GL_RGB10_A2UI,
    // GL_R11F_G11F_B10F,
    // GL_RG32F,
    // GL_RG32I,
    // GL_RG32UI,
    // GL_RG16,
    // GL_RG16F,
    // GL_RG8,
    // GL_RG8I,
    // GL_RG8UI,
    // GL_RGBA16_SNORM,
    // GL_RGBA8_SNORM,
    // GL_RGB32F,
    // GL_RGB32I,
    // GL_RGB32UI,
    // GL_RGB16_SNORM,
    // GL_RGB16F,
    // GL_RGB16I,
    // GL_RGB16UI,
    // GL_RGB16,
    // GL_RGB8_SNORM,
    // GL_RGB8,
    // GL_RGB8I,
    // GL_RGB8UI,
    // GL_SRGB8,
    // GL_RGB9_E5,
    // GL_RG16_SNORM, GL_RG8_SNORM, GL_COMPRESSED_RG_RGTC2, GL_COMPRESSED_SIGNED_RG_RGTC2, GL_R16_SNORM, GL_R8_SNORM, GL_COMPRESSED_RED_RGTC1, GL_COMPRESSED_SIGNED_RED_RGTC1, GL_DEPTH_COMPONENT32F, GL_DEPTH_COMPONENT24, GL_DEPTH_COMPONENT16, GL_DEPTH32F_STENCIL8,
    // GL_DEPTH24_STENCIL8
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum ImageDataFormat{
    Red=RED,
    // GL_RG,
    // GL_RGB,
    // GL_BGR,
    RGBA=RGBA,
    // GL_BGRA,
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum ImageDataType{
    U8=UNSIGNED_BYTE,
    // GL_BYTE,
    // GL_UNSIGNED_SHORT,
    // GL_SHORT,
    // GL_UNSIGNED_INT,
    // GL_INT,
    // GL_FLOAT,
    // GL_UNSIGNED_BYTE_3_3_2,
    // GL_UNSIGNED_BYTE_2_3_3_REV,
    // GL_UNSIGNED_SHORT_5_6_5,
    // GL_UNSIGNED_SHORT_5_6_5_REV,
    // GL_UNSIGNED_SHORT_4_4_4_4,
    // GL_UNSIGNED_SHORT_4_4_4_4_REV,
    // GL_UNSIGNED_SHORT_5_5_5_1,
    // GL_UNSIGNED_SHORT_1_5_5_5_REV,
    // GL_UNSIGNED_INT_8_8_8_8,
    // GL_UNSIGNED_INT_8_8_8_8_REV,
    // GL_UNSIGNED_INT_10_10_10_2,
    // GL_UNSIGNED_INT_2_10_10_10_REV
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
    Nearest=NEAREST,
    Linear=LINEAR,
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum TextureMinFilter{
    Nearest=NEAREST,
    Linear=LINEAR,
    NearestMipmapNearest=NEAREST_MIPMAP_NEAREST,
    LinearMipmapNearest=LINEAR_MIPMAP_NEAREST,
    NearestMipmapLinear=NEAREST_MIPMAP_LINEAR,
    LinearMipmapLinear=LINEAR_MIPMAP_LINEAR,
}

// #[repr(u32)]
// #[derive(Clone,Copy,Debug)]
// pub enum TextureWrap{
//     Repeat=REPEAT,
//     MirroredRepeat=MIRRORED_REPEAT,
//     ClampToEdge=CLAMP_TO_EDGE,
//     ClampToBorder=CLAMP_TO_BORDER,
// }


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
    #[inline(always)]
    pub fn generate_one(&self,texture:&mut u32){
        unsafe{
            transmute::<usize,fn(i32,&mut u32)>(self.glGenTextures)(1,texture)
        }
    }

    #[inline(always)]
    pub unsafe fn delete_one(&self,texture:&u32){
        transmute::<usize,fn(i32,&u32)>(self.glDeleteTextures)(1,texture)
    }

    #[inline(always)]
    pub unsafe fn generate(&self,textures:&mut [u32]){
        transmute::<usize,fn(i32,&mut u32)>(self.glGenTextures)(textures.len() as i32,&mut textures[0])
    }

    #[inline(always)]
    pub fn delete(&self,textures:&[u32]){
        unsafe{
            transmute::<usize,fn(i32,&u32)>(self.glDeleteTextures)(textures.len() as i32,&textures[0])
        }
    }

    /// Binds a texture to a texturing target.
    #[inline(always)]
    pub unsafe fn bind(&self,target:TextureBindTarget,texture_id:u32){
        transmute::<usize,fn(TextureBindTarget,u32)>(self.glBindTexture)(target,texture_id)
    }
}

    // GL_TEXTURE_LOD_BIAS,
    // GL_TEXTURE_MIN_LOD,
    // GL_TEXTURE_MAX_LOD,
    // GL_TEXTURE_MAX_LEVEL,
    // GL_TEXTURE_SWIZZLE_R,
    // GL_TEXTURE_SWIZZLE_G,
    // GL_TEXTURE_SWIZZLE_B,
    // GL_TEXTURE_SWIZZLE_A,
    // GL_TEXTURE_WRAP_S,
    // GL_TEXTURE_WRAP_T,
    // GL_TEXTURE_WRAP_R

/// Texture parameters.
impl Texture{
    /// Specifies the index of the lowest defined mipmap level.
    /// 
    /// The initial value is 0.
    #[inline(always)]
    pub unsafe fn set_base_level(&self,target:TextureParameterTarget,value:i32){
        transmute::<usize,fn(TextureParameterTarget,u32,i32)>(self.glTexParameteri)(target,TEXTURE_BASE_LEVEL,value)
    }

    /// Specifies the comparison operator used when GL_TEXTURE_COMPARE_MODE is set to GL_COMPARE_REF_TO_TEXTURE.
    #[inline(always)]
    pub unsafe fn set_compare_function(&self,target:TextureParameterTarget,value:TextureCompareFunction){
        transmute::<usize,fn(TextureParameterTarget,u32,TextureCompareFunction)>(self.glTexParameteri)(target,TEXTURE_COMPARE_FUNC,value)
    }

    /// Specifies the texture comparison mode for currently bound depth textures (iternal format = DEPTH_COMPONENT).
    #[inline(always)]
    pub unsafe fn set_compare_mode(&self,target:TextureParameterTarget,value:TextureCompareMode){
        transmute::<usize,fn(TextureParameterTarget,u32,TextureCompareMode)>(self.glTexParameteri)(target,TEXTURE_COMPARE_MODE,value)
    }

    #[inline(always)]
    pub unsafe fn set_mag_filter(&self,target:TextureParameterTarget,value:TextureMagFilter){
        transmute::<usize,fn(TextureParameterTarget,u32,TextureMagFilter)>(self.glTexParameteri)(target,TEXTURE_MAG_FILTER,value)
    }

    #[inline(always)]
    pub unsafe fn set_min_filter(&self,target:TextureParameterTarget,value:TextureMinFilter){
        transmute::<usize,fn(TextureParameterTarget,u32,TextureMinFilter)>(self.glTexParameteri)(target,TEXTURE_MIN_FILTER,value)
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
    #[inline(always)]
    pub unsafe fn rewrite_image_2d<I:Sized>(
        &self,
        target:Texture2DRewriteTarget,
        mipmap_level:i32,
        internal_format:Texture2DInternalFormat,
        [width,height]:[i32;2],
        image_format:ImageDataFormat,
        image_data_type:ImageDataType,
        data:*const I,
    ){
        transmute::<usize,fn(
            Texture2DRewriteTarget,
            i32,
            Texture2DInternalFormat,
            i32,
            i32,
            i32,
            ImageDataFormat,
            ImageDataType,
            *const I
        )>(self.glTexImage2D)(
            target,
            mipmap_level,
            internal_format,
            width,
            height,
            0,
            image_format,
            image_data_type,
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
        image_format:ImageDataFormat,
        image_data_type:ImageDataType,
        data:*const I
    ){
        transmute::<usize,fn(
            Texture2DWriteTarget,
            i32,
            i32,
            i32,
            i32,
            i32,
            ImageDataFormat,
            ImageDataType,
            *const I
        )>(self.glTexSubImage2D)(
            target,
            mipmap_level,
            x,
            y,
            width,
            height,
            image_format,
            image_data_type,
            data,
        )
    }

    /// Copies a two-dimensional texture subimage.
    /// 
    /// Replaces a rectangular portion of a two-dimensional texture image or cube-map texture image with pixels from the current `GL_READ_BUFFER`.
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
    /// if `mipmap_level>log2(max)`, where `max` is the returned value of `GL_MAX_TEXTURE_SIZE`,
    /// if `write_x<0`, `(write_x+width)>w`, `write_y<0`, or `(write_y+height)>h`,
    /// where `w` is the `GL_TEXTURE_WIDTH`, and `h` is the `GL_TEXTURE_HEIGHT` of the texture image being modified.
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