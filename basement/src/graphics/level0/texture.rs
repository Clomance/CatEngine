use super::GLError;

use gl::{
    // consts
    MAX_TEXTURE_SIZE,
    TEXTURE_MIN_FILTER,
    TEXTURE_MAG_FILTER,

    REPEAT,
    MIRRORED_REPEAT,
    CLAMP_TO_EDGE,
    CLAMP_TO_BORDER,

    RED,
    RG,
    RGB,
    BGR,
    RGBA,
    BGRA,
    RED_INTEGER,
    RG_INTEGER,
    RGB_INTEGER,
    BGR_INTEGER,
    RGBA_INTEGER,
    BGRA_INTEGER,
    STENCIL_INDEX,
    DEPTH_COMPONENT,
    DEPTH_STENCIL,

    UNSIGNED_BYTE,
    BYTE,
    UNSIGNED_SHORT,
    SHORT,
    UNSIGNED_INT,
    INT,
    HALF_FLOAT,
    FLOAT,
    UNSIGNED_BYTE_3_3_2,
    UNSIGNED_BYTE_2_3_3_REV,
    UNSIGNED_SHORT_5_6_5,
    UNSIGNED_SHORT_5_6_5_REV,
    UNSIGNED_SHORT_4_4_4_4,
    UNSIGNED_SHORT_4_4_4_4_REV,
    UNSIGNED_SHORT_5_5_5_1,
    UNSIGNED_SHORT_1_5_5_5_REV,
    UNSIGNED_INT_8_8_8_8,
    UNSIGNED_INT_8_8_8_8_REV,
    UNSIGNED_INT_10_10_10_2,
    UNSIGNED_INT_2_10_10_10_REV,

    // Sized Internal Format
    R8,
    R8_SNORM,
    R16,
    R16_SNORM,
    RG8,
    RG8_SNORM,
    RG16,
    RG16_SNORM,
    R3_G3_B2,
    RGB4,RGB5,
    RGB8,
    RGB8_SNORM,
    RGB10,
    RGB12,
    RGB16,
    RGB16_SNORM,
    RGBA2,
    RGBA4,
    RGB5_A1,
    RGBA8,
    RGBA8_SNORM,
    RGB10_A2,
    RGB10_A2UI,
    RGBA12,
    RGBA16,
    SRGB8,
    SRGB8_ALPHA8,
    R16F,
    RG16F,
    RGB16F,
    RGBA16F,
    R32F,
    RG32F,
    RGB32F,
    RGBA32F,
    R11F_G11F_B10F,
    RGB9_E5,
    R8I,
    R8UI,
    R16I,
    R16UI,
    R32I,
    R32UI,
    RG8I,
    RG8UI,
    RG16I,
    RG16UI,
    RG32I,
    RG32UI,
    RGB8I,
    RGB8UI,
    RGB16I,
    RGB16UI,
    RGB32I,
    RGB32UI,
    RGBA8I,
    RGBA8UI,
    RGBA16I,
    RGBA16UI,
    RGBA32I,
    RGBA32UI,

    // 1D Texture targerts
    TEXTURE_1D,
    PROXY_TEXTURE_1D,

    // 2D Texture targets
    TEXTURE_2D,
    PROXY_TEXTURE_2D,
    TEXTURE_1D_ARRAY,
    PROXY_TEXTURE_1D_ARRAY,
    TEXTURE_RECTANGLE,
    PROXY_TEXTURE_RECTANGLE,
    TEXTURE_CUBE_MAP_POSITIVE_X,
    TEXTURE_CUBE_MAP_NEGATIVE_X,
    TEXTURE_CUBE_MAP_POSITIVE_Y,
    TEXTURE_CUBE_MAP_NEGATIVE_Y,
    TEXTURE_CUBE_MAP_POSITIVE_Z,
    TEXTURE_CUBE_MAP_NEGATIVE_Z,
    PROXY_TEXTURE_CUBE_MAP,

    // Texture filter
    NEAREST,
    LINEAR,
    NEAREST_MIPMAP_NEAREST,
    LINEAR_MIPMAP_NEAREST,
    NEAREST_MIPMAP_LINEAR,
    LINEAR_MIPMAP_LINEAR,

    // functions
    GetError,
    GenTextures,
    BindTexture,
    TexParameteri,
    TexImage1D,
    TexImage2D,
    TexImage3D,
    TexSubImage2D,
    GetTexImage,
    DeleteTextures,
    CopyTexSubImage2D,
};

use std::{
    marker::PhantomData,
    mem::MaybeUninit
};

#[repr(u64)]
#[derive(Clone,Copy,Debug)]
pub enum ImageDataFormat{
    /// Red = u8
    R_U8=RED as u64 | ((UNSIGNED_BYTE as u64) << 32),
    /// Red = i8
    R_I8=RED as u64 | ((BYTE as u64) << 32),
    /// Red = u16
    R_U16=RED as u64 | ((UNSIGNED_SHORT as u64) << 32),
    /// Red = u16
    R_I16=RED as u64 | ((SHORT as u64) << 32),
    /// Red = u32
    R_U32=RED as u64 | ((UNSIGNED_INT as u64) << 32),
    /// Red = i32
    R_I32=RED as u64 | ((INT as u64) << 32),
    /// Red = f16
    /// 
    /// Since opengl 3.0
    R_F16=RED as u64 | ((HALF_FLOAT as u64) << 32),
    /// Red = f32
    R_F32=RED as u64 | ((FLOAT as u64) << 32),

    /// Red =  u8, Green = u8
    RG_U8=RG as u64 | ((UNSIGNED_BYTE as u64) << 32),
    /// Red =  i8, Green = i8
    RG_I8=RG as u64 | ((BYTE as u64) << 32),
    /// Red =  u16, Green = u16
    RG_U16=RG as u64 | ((UNSIGNED_SHORT as u64) << 32),
    //// Red =  i16, Green = i16
    RG_I16=RG as u64 | ((SHORT as u64) << 32),
    /// Red =  u32, Green = u32
    RG_U32=RG as u64 | ((UNSIGNED_INT as u64) << 32),
    //// Red =  i32, Green = i32
    RG_I32=RG as u64 | ((INT as u64) << 32),
    //// Red =  f16, Green = f16
    /// 
    /// Since opengl 3.0
    RG_F16=RG as u64 | ((HALF_FLOAT as u64) << 32),
    /// Red =  f32, Green = f32
    RG_F32=RG as u64 | ((FLOAT as u64) << 32),

    /// Red = u3, Green = u3, Blue = u2
    R3_G3_B2=RGB as u64 | ((UNSIGNED_BYTE_3_3_2 as u64) << 32),
    /// Blue = u2, Green = u3, Red = u3
    B2_G3_R3=RGB as u64 | ((UNSIGNED_BYTE_2_3_3_REV as u64) << 32),
    /// Red = u8, Green = u8, Blue = u8
    RGB_U8=RGB as u64 | ((UNSIGNED_BYTE as u64) << 32),
    /// Red = i8, Green = i8, Blue = i8
    RGB_I8=RGB as u64 | ((BYTE as u64) << 32),
    /// Blue = u8, Green = u8, Red = u8
    BGR_U8=BGR as u64 | ((UNSIGNED_BYTE as u64) << 32),
    /// Blue = i8, Green = i8, Red = i8
    BGR_I8=BGR as u64 | ((BYTE as u64) << 32),
    /// Red = u5, Green = u6, Blue = u5
    R5_G6_B5=RGB as u64 | ((UNSIGNED_SHORT_5_6_5 as u64) << 32),
    /// Blue = u5, Green = u6, Red = u5
    B5_G6_R5=RGB as u64 | ((UNSIGNED_SHORT_5_6_5_REV as u64) << 32),
    /// Red = u16, Green = u16, Blue = u16
    RGB_U16=RGB as u64 | ((UNSIGNED_SHORT as u64) << 32),
    /// Red = i16, Green = i16, Blue = i16
    RGB_I16=RGB as u64 | ((SHORT as u64) << 32),
    /// Blue = u16, Green = u16, Red = u16
    BGR_U16=BGR as u64 | ((UNSIGNED_SHORT as u64) << 32),
    /// Blue = i16, Green = i16, Red = i16
    BGR_I16=BGR as u64 | ((SHORT as u64) << 32),
    /// Red = f16, Green = f16, Blue = f16
    /// 
    /// Since OpenGL 3.0.
    RGB_F16=RGB as u64 | ((HALF_FLOAT as u64) << 32),
    /// Blue = f16, Green = f16, Red = f16
    /// 
    /// Since OpenGL 3.0.
    BGR_F16=BGR as u64 | ((HALF_FLOAT as u64) << 32),
    /// Red = u32, Green = u32, Blue = u32
    RGB_U32=RGB as u64 | ((UNSIGNED_INT as u64) << 32),
    /// Red = i32, Green = i32, Blue = i32
    RGB_I32=RGB as u64 | ((INT as u64) << 32),
    /// Blue = u32, Green = u32, Red = u32
    BGR_U32=BGR as u64 | ((UNSIGNED_INT as u64) << 32),
    /// Blue = i32, Green = i32, Red = i32
    BGR_I32=BGR as u64 | ((INT as u64) << 32),
    /// Red = f32, Green = f32, Blue = f32
    RGB_F32=RGB as u64 | ((FLOAT as u64) << 32),
    /// Blue = f32, Green = f32, Red = f32
    BGR_F32=BGR as u64 | ((FLOAT as u64) << 32),

    /// Red = u8, Green = u8, Blue = u8, Alpha = u8
    RGBA_U8=RGBA as u64 | ((UNSIGNED_BYTE as u64) << 32),
    /// Red = i8, Green = i8, Blue = i8, Alpha = i8
    RGBA_I8=RGBA as u64 | ((BYTE as u64) << 32),
    /// Blue = u8, Green = u8, Red = u8, Alpha = u8
    BGRA_U8=BGRA as u64 | ((UNSIGNED_BYTE as u64) << 32),
    /// Blue = i8, Green = i8, Red = i8, Alpha = i8
    BGRA_I8=BGRA as u64 | ((BYTE as u64) << 32),
    /// Red = u16, Green = u16, Blue = u16, Alpha = u16
    RGBA_U16=RGBA as u64 | ((UNSIGNED_SHORT as u64) << 32),
    /// Red = i16, Green = i16, Blue = i16, Alpha = i16
    RGBA_I16=RGBA as u64 | ((SHORT as u64) << 32),
    /// Blue = u16, Green = u16, Red = u16, Alpha = u16
    BGRA_U16=BGRA as u64 | ((UNSIGNED_SHORT as u64) << 32),
    /// Blue = i16, Green = i16, Red = i16, Alpha = i16
    BGRA_I16=BGRA as u64 | ((SHORT as u64) << 32),
    /// Red = F16, Green = F16, Blue = F16, Alpha = F16
    /// 
    /// Since OpenGL 3.0
    RGBA_F16=RGBA as u64 | ((HALF_FLOAT as u64) << 32),
    /// Blue = F16, Green = F16, Red = F16, Alpha = F16
    /// 
    /// Since OpenGL 3.0
    BGRA_F16=BGRA as u64 | ((HALF_FLOAT as u64) << 32),

    // StencilIndex=STENCIL_INDEX as u64,
    // DepthComponent=DEPTH_COMPONENT as u64,
    // DepthStencil=DEPTH_STENCIL as u64,
}

impl ImageDataFormat{
    /// [the format of the pixel data, the format of colour component]
    pub (crate) fn as_gl_enums(self)->[u32;2]{
        unsafe{
            std::mem::transmute::<ImageDataFormat,[u32;2]>(self)
        }
    }
}

/// Specifies .
// #[derive(Clone,Copy,Debug)]
// pub enum ImageDataFormat{
//     RGBA=RGBA as u64,
//     BGRA=BGRA as u64,
//     RedInteger=RED_INTEGER as u64,
//     RGInterger=RG_INTEGER as u64,
//     RGBInteger=RGB_INTEGER as u64,
//     BGRInteger=BGR_INTEGER as u64,
//     RGBAInteger=RGBA_INTEGER as u64,
//     BGRAInteger=BGRA_INTEGER as u64,
//     StencilIndex=STENCIL_INDEX as u64,
//     DepthComponent=DEPTH_COMPONENT as u64,
//     DepthStencil=DEPTH_STENCIL as u64,
// }

// #[derive(Clone,Copy,Debug)]
// pub enum ImageDataType{
//     U8=UNSIGNED_BYTE as u64,
//     I8=BYTE as u64,
//     U16=UNSIGNED_SHORT as u64,
//     I16=SHORT as u64,
//     U32=UNSIGNED_INT as u64,
//     I32=INT as u64,
//     /// Since OpenGL 3.0.
//     F16=HALF_FLOAT as u64,
//     F32=FLOAT as u64,

//     U16_4_4_4_4=UNSIGNED_SHORT_4_4_4_4 as u64,
//     U16_4_4_4_4_Rev=UNSIGNED_SHORT_4_4_4_4_REV as u64,
//     U16_5_5_5_1=UNSIGNED_SHORT_5_5_5_1 as u64,
//     U16_1_5_5_5_Rev=UNSIGNED_SHORT_1_5_5_5_REV as u64,
//     U32_8_8_8_8=UNSIGNED_INT_8_8_8_8 as u64,
//     U32_8_8_8_8_Rev=UNSIGNED_INT_8_8_8_8_REV as u64,
//     U32_10_10_10_2=UNSIGNED_INT_10_10_10_2 as u64,
//     U32_2_10_10_10_Rev=UNSIGNED_INT_2_10_10_10_REV as u64
// }


/// Specifies the number of colour components in the texture.
#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum TextureInternalFormat{
    R16=R16,
    R16_SNorm=R16_SNORM,
    /// Since OpenGL 3.0
    R_F16=R16F,
    R_F32=R32F,

    R_I8=R8I,
    R_U8=R8,
    R8_SNorm=R8_SNORM,
    R_I16=R16I,
    R_U16=R16UI,
    R_I32=R32I,
    R_U32=R32UI,

    RG8=RG8,
    RG8_SNorm=RG8_SNORM,
    RG16=RG16,
    RG16_SNorm=RG16_SNORM,

    RGB4=RGB4,
    RGB5=RGB5,
    RGB8=RGB8,
    RGB8_SNorm=RGB8_SNORM,
    R3G3B2=R3_G3_B2,
    RGB10=RGB10,
    RGB12=RGB12,
    RGB16=RGB16,
    BGR=BGR,
    BGRA=BGRA,

    RGBA8=RGBA8,
    RGBA8_SNorm=RGBA8_SNORM,
    RGBA16=RGBA16,

    RGB10A2=RGB10_A2,

    DepthComponent=DEPTH_COMPONENT,
    DepthStencil=DEPTH_STENCIL,
    // RGB16_SNORM,RGBA2,RGBA4,RGB5_A1,
    // RGB10_A2UI,RGBA12,RGBA16,SRGB8,
    // SRGB8_ALPHA8RG16F,RGB16F,RGBA16F,RG32F,RGB32F,RGBA32F,R11F_G11F_B10F,RGB9_E5,
    // RG8I,RG8UI,RG16I,RG16UI,RG32I,RG32UI,RGB8I,RGB8UI,RGB16I,RGB16UI,RGB32I,
    // RGB32UI,RGBA8I,RGBA8UI,RGBA16I,RGBA16UI,RGBA32I,RGBA32UI,
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum TextureFilter{
    Nearest=NEAREST,
    Linear=LINEAR,
    NearestMipmapNearest=NEAREST_MIPMAP_NEAREST,
    LinearMipmapNearest=LINEAR_MIPMAP_NEAREST,
    NearestMipmapLinear=NEAREST_MIPMAP_LINEAR,
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

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum Texture2DTarget{
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
    PROXY_TEXTURE_CUBE_MAP=PROXY_TEXTURE_CUBE_MAP,
}

pub struct Texture{
    id:u32,
}

impl Texture{
    pub fn initialize()->Texture{
        unsafe{
            let mut id=MaybeUninit::uninit().assume_init();

            GenTextures(1,&mut id as *mut _);

            Self{
                id,
            }
        }
    }

    pub fn new_1d(
        texture_internal_format:TextureInternalFormat,
        mag_filter:TextureFilter,
        min_filter:TextureFilter,
        size:u32,
        image_data_format:ImageDataFormat,
        data:&[u8],
    )->Texture{
        unsafe{
            let texture=Texture::initialize();
            texture.bind(TEXTURE_1D).rewrite_image_1d(texture_internal_format,size,image_data_format,data);

            TexParameteri(TEXTURE_1D,TEXTURE_MAG_FILTER,mag_filter as i32);

            TexParameteri(TEXTURE_1D,TEXTURE_MIN_FILTER,min_filter as i32);

            texture
        }
    }

    pub fn new_2d(
        texture_internal_format:TextureInternalFormat,
        mag_filter:TextureFilter,
        min_filter:TextureFilter,
        size:[u32;2],
        image_data_format:ImageDataFormat,
        data:&[u8],
    )->Texture{
        unsafe{
            let texture=Texture::initialize();
            texture.bind(TEXTURE_2D).rewrite_image_2d(texture_internal_format,size,image_data_format,data);

            TexParameteri(TEXTURE_2D,TEXTURE_MAG_FILTER,mag_filter as i32);
           
            TexParameteri(TEXTURE_2D,TEXTURE_MIN_FILTER,min_filter as i32);

            

            texture
        }
    }

    #[inline(always)]
    pub fn id(&self)->u32{
        self.id
    }

    pub fn bind<'a>(&'a self,target:u32)->BoundTexture<'a>{
        unsafe{
            BindTexture(target,self.id);
            BoundTexture{
                target,
                marker:PhantomData
            }
        }
    }
}

impl Drop for Texture{
    fn drop(&mut self){
        unsafe{
            DeleteTextures(1,&self.id as *const _);
        }
    }
}

pub struct BoundTexture<'a>{
    target:u32,
    marker:PhantomData<&'a Texture>
}

impl<'a> BoundTexture<'a>{
    pub fn target(&self)->u32{
        self.target
    }

    pub fn rewrite_image_1d(
        &self,
        texture_internal_format:TextureInternalFormat,
        size:u32,
        image_data_format:ImageDataFormat,
        data:&[u8]
    ){
        unsafe{
            // Arguments:
            // 1 - target
            // 2 - mipmap level - if we want to set mipmap textures manually
            // 3 - colour format of the texture
            // 4 - size
            // 5 - 
            // 6 - colour format of the image
            // 7 - byte format
            // 8 - data
            let [pixel_format,component_format]=image_data_format.as_gl_enums();
            let data_ref=if data.len()!=0{
                (data as *const [u8]) as *const core::ffi::c_void
            }
            else{
                0 as *const core::ffi::c_void
            };
            TexImage1D(
                self.target,
                0,
                texture_internal_format as i32,
                size as i32,
                0,
                pixel_format,
                component_format,
                data_ref
            );
        }
    }

    pub fn rewrite_image_2d(
        &self,
        texture_internal_format:TextureInternalFormat,
        size:[u32;2],
        image_data_format:ImageDataFormat,
        data:&[u8]
    ){
        unsafe{
            // Arguments:
            // 1 - target
            // 2 - mipmap level - if we want to set mipmap textures manually
            // 3 - colour format of the texture
            // 4, 5 - size (width, height)
            // 6 - 
            // 7 - colour format of the image
            // 8 - byte format
            // 9 - data
            let [pixel_format,component_format]=image_data_format.as_gl_enums();
            let data_ref=if data.len()!=0{
                (data as *const [u8]) as *const core::ffi::c_void
            }
            else{
                0 as *const core::ffi::c_void
            };
            TexImage2D(
                self.target,
                0,
                texture_internal_format as i32,
                size[0] as i32,
                size[1] as i32,
                0,
                pixel_format,
                component_format,
                data_ref
            );
        }
    }

    pub fn rewrite_image_3d(
        &self,
        texture_internal_format:TextureInternalFormat,
        size:[u32;3],
        image_data_format:ImageDataFormat,
        data:&[u8]
    ){
        unsafe{
            // Arguments:
            // 1 - target
            // 2 - mipmap level - if we want to set mipmap textures manually
            // 3 - colour format of the texture
            // 4, 5, 6 - size (width, height, depth)
            // 7 - 
            // 8 - colour format of the image
            // 9 - byte format
            // 10 - data
            let [pixel_format,component_format]=image_data_format.as_gl_enums();
            let data_ref=if data.len()!=0{
                (data as *const [u8]) as *const core::ffi::c_void
            }
            else{
                0 as *const core::ffi::c_void
            };
            TexImage3D(
                self.target,
                0,
                texture_internal_format as i32,
                size[0] as i32,
                size[1] as i32,
                size[2] as i32,
                0,
                pixel_format,
                component_format,
                data_ref
            );
        }
    }

    pub fn write_image_2d(&self,offset:[u32;2],size:[u32;2],image_data_format:ImageDataFormat,data:&[u8]){
        unsafe{
            let data_ref=if data.len()!=0{
                (data as *const [u8]) as *const core::ffi::c_void
            }
            else{
                0 as *const core::ffi::c_void
            };
            let [image_type,image_format]=image_data_format.as_gl_enums();
            TexSubImage2D(
                self.target,
                0,
                offset[0] as i32,
                offset[1] as i32,
                size[0] as i32,
                size[1] as i32,
                image_type,
                image_format,
                data_ref
            );
        }
    }

    /// The target must be GL_TEXTURE_2D, GL_TEXTURE_CUBE_MAP_POSITIVE_X,
    /// GL_TEXTURE_CUBE_MAP_NEGATIVE_X, GL_TEXTURE_CUBE_MAP_POSITIVE_Y,
    /// GL_TEXTURE_CUBE_MAP_NEGATIVE_Y, GL_TEXTURE_CUBE_MAP_POSITIVE_Z, or GL_TEXTURE_CUBE_MAP_NEGATIVE_Z.
    pub fn write_read_framebuffer(&self,buffer_offset:[u32;2],texture_offset:[u32;2],size:[u32;2]){
        unsafe{
            CopyTexSubImage2D(
                self.target,
                0,
                texture_offset[0] as i32,
                texture_offset[1] as i32,
                buffer_offset[0] as i32,
                buffer_offset[1] as i32,
                size[0] as i32,
                size[1] as i32,
            );
        }
    }
}

/// Settings parameters.
impl<'a> BoundTexture<'a>{
    #[inline(always)]
    pub unsafe fn set_parameters(&self,parameter:u32,value:i32){
        TexParameteri(self.target,parameter,value);
    }

    #[inline(always)]
    pub fn set_mag_filter(&self,filter:TextureFilter){
        unsafe{
            TexParameteri(self.target,TEXTURE_MAG_FILTER,filter as i32);
        }
    }

    #[inline(always)]
    pub fn set_min_filter(&self,filter:TextureFilter){
        unsafe{
            TexParameteri(self.target,TEXTURE_MIN_FILTER,filter as i32);
        }
    }
}