use cat_engine_basement::graphics::{
    gl::BindFramebuffer,
    level0::{
        FrameBufferTarget,
        Texture2DTarget,
        TextureFilter,
        TextureInternalFormat,
        ImageDataFormat,
    },
    level1::{
        Texture2D,
        BoundTexture2D,
    },
};

mod image_base;
pub use image_base::ImageBase;

mod image_object;
pub use image_object::ImageObject;

use image::{
    GenericImageView,
    RgbaImage,
};

use std::{
    path::Path,
};

pub struct Texture{
    texture:Texture2D,
}

impl Texture{
    pub fn new(size:[u32;2],data:&[u8])->Option<Texture>{
        if data.len()!=(size[0]*size[1]) as usize{
            return None
        }

        let texture=Texture2D::new(
            TextureInternalFormat::RGBA8,
            TextureFilter::Linear,
            TextureFilter::Linear,
            size,
            ImageDataFormat::RGBA_U8,
            data
        );

        Some(
            Self{
                texture,
            }
        )
    }

    pub fn empty(size:[u32;2])->Texture{
        let texture=Texture2D::empty(
            TextureInternalFormat::RGBA8,
            TextureFilter::Linear,
            TextureFilter::Linear,
            size,
        );

        Self{
            texture,
        }
    }

    /// Flips verticaly.
    pub fn from_path<P:AsRef<Path>>(path:P)->Option<Texture>{
        if let Ok(image)=image::open(path){
            let (w,h)=image.dimensions();
            let texture=Texture2D::new(
                TextureInternalFormat::RGBA8,
                TextureFilter::Linear,
                TextureFilter::Linear,
                [w,h],
                ImageDataFormat::RGBA_U8,
                &image.flipv().to_rgba8()
            );

            Some(
                Self{
                    texture,
                }
            )
        }
        else{
            None
        }
    }

    pub fn from_image(image:&RgbaImage)->Texture{
        let (w,h)=image.dimensions();
        let texture=Texture2D::new(
            TextureInternalFormat::RGBA8,
            TextureFilter::Linear,
            TextureFilter::Linear,
            [w,h],
            ImageDataFormat::RGBA_U8,
            image.as_ref()
        );

        Self{
            texture,
        }
    }

    pub fn texture_2d(&self)->&Texture2D{
        &self.texture
    }

    pub fn into_texture_2d(self)->Texture2D{
        self.texture
    }
}

impl Texture{
    pub fn write(&self,offset:[u32;2],image_data_format:ImageDataFormat,size:[u32;2],data:&[u8]){
        self.texture.bind().write_image(offset,size,image_data_format,data)
    }

    pub fn write_rbga(&self,offset:[u32;2],image:&RgbaImage){
        let (w,h)=image.dimensions();
        self.texture.bind().write_image(offset,[w,h],ImageDataFormat::RGBA_U8,image)
    }

    pub fn write_image(&self,image:&RgbaImage){
        let (w,h)=image.dimensions();
        self.texture.bind().write_image([0;2],[w,h],ImageDataFormat::RGBA_U8,image)
    }

    pub fn write_screen_buffer(&self,screen_offset:[u32;2],texture_offset:[u32;2],size:[u32;2]){
        unsafe{BindFramebuffer(FrameBufferTarget::Read as u32,0)}
        self.texture.texture().bind(Texture2DTarget::Texture2D as u32).write_read_framebuffer(
            screen_offset,
            texture_offset,
            size,
        );
    }

    pub fn rewrite_image(&self,texture_internal_format:TextureInternalFormat,image:&RgbaImage){
        let (w,h)=image.dimensions();
        self.texture.bind().rewrite_image(texture_internal_format,[w,h],ImageDataFormat::RGBA_U8,image)
    }
}