pub use super::level0::{
    Texture,
    BoundTexture,
    TextureFilter,
    TextureInternalFormat,
    ImageDataFormat,
};

use gl::{
    // consts
    NO_ERROR,
    TEXTURE_2D,
    RGBA,
    RGBA8,
    UNSIGNED_BYTE,
    TEXTURE_MIN_FILTER,
    TEXTURE_MAG_FILTER,
    NEAREST,
    LINEAR,

    // functions
    GetError,
    GenTextures,
    BindTexture,
    TexParameteri,
    TexImage2D,
    TexSubImage2D,
    GetTexImage,
    DeleteTextures,
};

pub struct Texture2D{
    texture:Texture,
}

impl Texture2D{
    pub fn initialize()->Texture2D{
        Self{
            texture:Texture::initialize(),
        }
    }

    /// mag for scaling upwards, min for scaling downwards
    pub fn new(
        texture_internal_format:TextureInternalFormat,
        mag:TextureFilter,
        min:TextureFilter,
        size:[u32;2],
        image_data_format:ImageDataFormat,
        data:&[u8]
    )->Texture2D{
        let texture=Texture::new_2d(
            texture_internal_format,
            mag,
            min,
            size,
            image_data_format,
            data
        );

        Self{
            texture,
        }
    }

    pub fn empty(
        texture_internal_format:TextureInternalFormat,
        mag:TextureFilter,
        min:TextureFilter,
        size:[u32;2]
    )->Texture2D{
        let texture=Texture::new_2d(
            texture_internal_format,
            mag,
            min,
            size,
            ImageDataFormat::R_U8,
            &[]
        );

        Self{
            texture,
        }
    }

    pub fn raw(texture:Texture)->Texture2D{
        Self{
            texture,
        }
    }

    pub fn texture(&self)->&Texture{
        &self.texture
    }

    pub fn bind<'a>(&'a self)->BoundTexture2D<'a>{
        BoundTexture2D::raw(self.texture.bind(TEXTURE_2D))
    }
}


pub struct BoundTexture2D<'a>{
    marker:BoundTexture<'a>,
}

impl<'a> BoundTexture2D<'a>{
    pub fn raw(bound_texture:BoundTexture<'a>)->BoundTexture2D<'a>{
        Self{
            marker:bound_texture
        }
    }

    pub fn rewrite_image(
        &self,
        texture_internal_format:TextureInternalFormat,
        size:[u32;2],
        image_data_format:ImageDataFormat,
        data:&[u8]
    ){
        self.marker.rewrite_image_2d(
            texture_internal_format,
            size,
            image_data_format,
            data
        )
    }

    pub fn write_image(&self,offset:[u32;2],size:[u32;2],image_data_format:ImageDataFormat,data:&[u8]){
        unsafe{
            let data_ref=if data.len()!=0{
                (data as *const [u8]) as *const core::ffi::c_void
            }
            else{
                0 as *const core::ffi::c_void
            };
            let [image_type,image_format]=image_data_format.as_gl_enums();
            TexSubImage2D(
                self.marker.target(),
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
}
