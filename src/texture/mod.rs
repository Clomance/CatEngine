use std::io::Read;

use cat_engine_basement::opengl::{
    core::texture::{Texture as TextureFunctions, Texture2DWriteTarget},
    texture::{
        Texture as RawTexture,
        Texture2DAllocateTarget,
        TextureInternalFormat,
        TextureParameter,
        TextureParameterTarget,
        TextureTarget,
        PixelFormat,
        PixelType,
    }
};

pub use cat_engine_basement::opengl::{
    texture::{
        TextureMagFilter,
        TextureMinFilter,
    }
};

pub struct Texture2D{
    pub (crate) inner:RawTexture
}

impl Texture2D{
    pub fn new(
        min_filter:TextureMinFilter,
        mag_filter:TextureMagFilter,
        [width,height]:[u32;2],
        data:&[u8]
    )->Texture2D{
        let texture=RawTexture::new();

        texture.bind(TextureTarget::Texture2D);

        RawTexture::set_parameteri(
            TextureParameterTarget::Texture2D,
            TextureParameter::MinFilter,
            min_filter as i32
        );

        RawTexture::set_parameteri(
            TextureParameterTarget::Texture2D,
            TextureParameter::MagFilter,
            mag_filter as i32
        );

        RawTexture::allocate_2d(
            Texture2DAllocateTarget::Texture2D,
            0,
            TextureInternalFormat::RGBA8,
            width as i32,
            height as i32,
            PixelFormat::RGBA,
            PixelType::U8,
            unsafe{data.get_unchecked(0)}
        );

        Self{
            inner:texture
        }
    }

    pub fn empty(
        min_filter:TextureMinFilter,
        mag_filter:TextureMagFilter,
        [width,height]:[u32;2]
    )->Texture2D{
        let texture=RawTexture::new();

        texture.bind(TextureTarget::Texture2D);

        RawTexture::set_parameteri(
            TextureParameterTarget::Texture2D,
            TextureParameter::MinFilter,
            min_filter as i32
        );

        RawTexture::set_parameteri(
            TextureParameterTarget::Texture2D,
            TextureParameter::MagFilter,
            mag_filter as i32
        );

        RawTexture::allocate_2d::<u8>(
            Texture2DAllocateTarget::Texture2D,
            0,
            TextureInternalFormat::RGBA8,
            width as i32,
            height as i32,
            PixelFormat::RGBA,
            PixelType::U8,
            std::ptr::null()
        );

        Self{
            inner:texture
        }
    }

    pub fn write(&self,[width,height]:[u32;2],data:&[u8]){
        self.inner.bind(TextureTarget::Texture2D);

        RawTexture::write_2d(
            Texture2DWriteTarget::Texture2D,
            0,
            0,
            0,
            width as i32,
            height as i32,
            PixelFormat::RGBA,
            PixelType::U8,
            unsafe{data.get_unchecked(0)}
        )
    }
}