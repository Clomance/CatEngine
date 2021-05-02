pub use cat_engine_basement::graphics::{
    level0::{
        BoundTexture,
        TextureFilter,
        TextureInternalFormat,
        ImageDataFormat,
    },
    level1::texture::{
        texture_2d::{
            Texture2D,
            BoundTexture2D,
        },
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

    // flips verticaly
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