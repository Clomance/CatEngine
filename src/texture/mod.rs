use cat_engine_basement::graphics::{
    GCore,
    core::texture::{
        TextureBindTarget,
        Texture2DRewriteTarget,
        Texture2DWriteTarget,
        TextureMagFilter,
        TextureMinFilter,
        Texture2DInternalFormat,
        ImageDataFormat,
        ImageDataType,
    },
    level1::Texture2D,
};

mod image_base;
pub use image_base::ImageBase;

mod image_object;
pub use image_object::ImageObject;

use cat_engine_basement::image::{
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
        if data.len()!=(size[0]*size[1]*4) as usize{
            return None
        }

        let texture=Texture2D::new(
            Texture2DInternalFormat::RGBA8,
            TextureMagFilter::Linear,
            TextureMinFilter::Linear,
            size,
            ImageDataFormat::RGBA,
            ImageDataType::U8,
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
            Texture2DInternalFormat::RGBA8,
            TextureMagFilter::Linear,
            TextureMinFilter::Linear,
            size,
        );

        Self{
            texture,
        }
    }

    /// Flips verticaly.
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
    pub fn write(&self,[x,y,width,height]:[i32;4],image_data_format:ImageDataFormat,image_data_type:ImageDataType,data:&[u8]){
        self.texture.bind();
        unsafe{
            GCore.texture.write_image_2d(Texture2DWriteTarget::Texture2D,0,[x,y,width,height],image_data_format,image_data_type,&data[0])
        }
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
            GCore.texture.write_image_2d(Texture2DWriteTarget::Texture2D,0,frame,ImageDataFormat::RGBA,ImageDataType::U8,image)
        }
    }

    pub fn write_image(&self,image:&RgbaImage){
        let (w,h)=image.dimensions();
        self.texture.bind();
        unsafe{
            GCore.texture.write_image_2d(Texture2DWriteTarget::Texture2D,0,[0,0,w as i32,h as i32],ImageDataFormat::RGBA,ImageDataType::U8,image)
        }
    }

    // pub fn write_screen_buffer(&self,screen_offset:[u32;2],texture_offset:[u32;2],size:[u32;2]){
    //     unsafe{BindFramebuffer(FrameBufferTarget::Read as u32,0)}
    //     self.texture.texture().bind(Texture2DTarget::Texture2D as u32).write_read_framebuffer(
    //         screen_offset,
    //         texture_offset,
    //         size,
    //     );
    // }

    pub fn rewrite_image(&self,texture_internal_format:Texture2DInternalFormat,image:&RgbaImage){
        let (w,h)=image.dimensions();
        self.texture.bind();
        unsafe{
            GCore.texture.rewrite_image_2d(Texture2DRewriteTarget::Texture2D,0,texture_internal_format,[w as i32,h as i32],ImageDataFormat::RGBA,ImageDataType::U8,image)
        }
    }
}