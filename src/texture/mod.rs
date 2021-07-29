use cat_engine_basement::graphics::{
    GCore,
    core::texture::{
        Texture2DRewriteTarget,
        Texture2DWriteTarget,
        TextureMagFilter,
        TextureMinFilter,
        Texture2DInternalFormat,
        ImageDataFormat,
    },
    level1::Texture2D,
};

mod image_base;
pub use image_base::ImageBase;

mod image_object;
pub use image_object::ImageObject;

use cat_engine_basement::image::RgbaImage;

use std::{
    path::Path,
};

/// A 8-bit RGBA 2D texture.
pub struct Texture{
    texture:Texture2D,
}

impl Texture{
    /// Creates a texture.
    /// 
    /// If the size of `data` is not equal to `size`,
    /// returns None.
    pub fn new(size:[u32;2],data:&[u8])->Option<Texture>{
        if data.len()!=(size[0]*size[1]*4) as usize{
            return None
        }

        let texture=Texture2D::new(
            Texture2DInternalFormat::RGBA8,
            TextureMagFilter::Linear,
            TextureMinFilter::Linear,
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

    /// Creates a texture with no data loaded.
    pub fn empty(size:[u32;2])->Texture{
        let texture=Texture2D::empty(
            Texture2DInternalFormat::RGBA8,
            TextureMagFilter::Linear,
            TextureMinFilter::Linear,
            size,
        );

        Self{
            texture,
        }
    }

    /// Loads an image from the path, flips it verticaly,
    /// converts to 8-bit RGBA and creates a texture.
    pub fn from_path<P:AsRef<Path>>(path:P)->Option<Texture>{
        if let Ok(image)=cat_engine_basement::image::open(path){
            let image=image.flipv().to_rgba8();
            let (w,h)=image.dimensions();
            Self::new([w,h],image.as_ref())
        }
        else{
            None
        }
    }

    /// Creates a texture with the given image.
    pub fn from_image(image:&RgbaImage)->Texture{
        let (w,h)=image.dimensions();
        Self::new([w,h],image.as_ref()).unwrap()
    }

    pub fn texture_2d(&self)->&Texture2D{
        &self.texture
    }

    pub fn into_texture_2d(self)->Texture2D{
        self.texture
    }
}

impl Texture{
    pub fn write(
        &self,
        [x,y,width,height]:[i32;4],
        image_data_format:ImageDataFormat,
        data:&[u8]
    ){
        self.texture.bind();
        self.texture.write_image(
            [x,y,width,height],
            image_data_format,
            data
        )
    }

    pub fn write_rbga(&self,[x,y]:[i32;2],image:&RgbaImage){
        let (w,h)=image.dimensions();
        let frame=[
            x,
            y,
            w as i32,
            h as i32,
        ];
        self.texture.bind();
        unsafe{
            GCore.texture.write_image_2d(
                Texture2DWriteTarget::Texture2D,
                0,
                frame,
                ImageDataFormat::RGBA_U8,
                image
            )
        }
    }

    pub fn write_image(&self,image:&RgbaImage){
        let (w,h)=image.dimensions();
        self.write(
            [0,0,w as i32,h as i32],
            ImageDataFormat::RGBA_U8,
            image.as_ref()
        );
    }

    pub fn rewrite_image(&self,texture_internal_format:Texture2DInternalFormat,image:&RgbaImage){
        let (w,h)=image.dimensions();
        self.texture.bind();
        unsafe{
            GCore.texture.rewrite_image_2d(
                Texture2DRewriteTarget::Texture2D,
                0,
                texture_internal_format,
                [w as i32,h as i32],
                ImageDataFormat::RGBA_U8,
                image.as_ref() as *const [u8] as *const u8
            )
        }
    }
}