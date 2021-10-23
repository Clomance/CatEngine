use cat_engine_basement::graphics::{
    GLCore,
    core::GLError,
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

use cat_engine_basement::image::{
    RgbaImage,
    ImageError,
    open,
};

use std::path::Path;

#[derive(Debug)]
pub enum TexureCreationError{
    GLError(GLError),
    ImageError(ImageError),
}

/// A 8-bit RGBA 2D texture.
pub struct Texture{
    texture:Texture2D,
}

impl Texture{
    /// Creates a texture.
    pub fn new(size:[u32;2],data:&[u8])->Result<Texture,GLError>{
        match Texture2D::new(
            Texture2DInternalFormat::RGBA8,
            TextureMagFilter::Linear,
            TextureMinFilter::Linear,
            size,
            ImageDataFormat::RGBA_U8,
            data
        ){
            Ok(texture)=>Ok(
                Self{
                    texture,
                }
            ),
            Err(e)=>Err(e),
        }
    }

    /// Creates a texture with no data loaded.
    pub fn empty(size:[u32;2])->Result<Texture,GLError>{
        match Texture2D::empty(
            Texture2DInternalFormat::RGBA8,
            TextureMagFilter::Linear,
            TextureMinFilter::Linear,
            size,
        ){
            Ok(texture)=>Ok(
                Self{
                    texture,
                }
            ),
            Err(e)=>Err(e),
        }
    }

    /// Loads an image from the path, flips it verticaly,
    /// converts to 8-bit RGBA and creates a texture.
    pub fn from_path<P:AsRef<Path>>(path:P)->Result<Texture,TexureCreationError>{
        match open(path){
            Ok(image)=>{
                let image=image.flipv().to_rgba8();
                let (w,h)=image.dimensions();
                match Self::new([w,h],image.as_ref()){
                    Ok(texture)=>Ok(texture),
                    Err(e)=>Err(TexureCreationError::GLError(e)),
                }
            }
            Err(e)=>Err(TexureCreationError::ImageError(e)),
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
        [x,y,width,height]:[u32;4],
        image_data_format:ImageDataFormat,
        data:&[u8]
    )->GLError{
        self.texture.write_image(
            [x,y,width,height],
            image_data_format,
            data
        )
    }

    pub fn write_rbga(&self,[x,y]:[u32;2],image:&RgbaImage)->GLError{
        let (w,h)=image.dimensions();
        let frame=[x,y,w,h];
        self.texture.write_image(
            frame,
            ImageDataFormat::RGBA_U8,
            image
        )
    }

    pub fn write_image(&self,image:&RgbaImage)->GLError{
        let (w,h)=image.dimensions();
        self.write(
            [0,0,w,h],
            ImageDataFormat::RGBA_U8,
            image.as_ref()
        )
    }

    pub fn rewrite_image(
        &self,
        texture_internal_format:Texture2DInternalFormat,
        image:&RgbaImage
    )->GLError{
        let (w,h)=image.dimensions();
        self.texture.bind();
        self.texture.rewrite_image(
            texture_internal_format,
            [w,h],
            ImageDataFormat::RGBA_U8,
            image.as_ref()
        )
    }
}