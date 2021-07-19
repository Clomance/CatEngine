use crate::graphics::{
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
    level0::{
        Texture,
        // BoundTexture,
    },
};

pub struct Texture2D{
    texture:Texture,
}

impl Texture2D{
    pub fn initiate()->Texture2D{
        Self{
            texture:Texture::initiate(),
        }
    }

    /// mag for scaling upwards, min for scaling downwards
    pub fn new(
        texture_internal_format:Texture2DInternalFormat,
        mag:TextureMagFilter,
        min:TextureMinFilter,
        size:[u32;2],
        image_data_format:ImageDataFormat,
        image_data_type:ImageDataType,
        data:&[u8]
    )->Texture2D{
        let texture=Texture::new_2d(
            texture_internal_format,
            mag,
            min,
            size,
            image_data_format,
            image_data_type,
            data
        );

        Self{
            texture,
        }
    }

    pub fn empty(
        texture_internal_format:Texture2DInternalFormat,
        mag:TextureMagFilter,
        min:TextureMinFilter,
        size:[u32;2]
    )->Texture2D{
        let texture=Texture::new_2d::<()>(
            texture_internal_format,
            mag,
            min,
            size,
            ImageDataFormat::Red,
            ImageDataType::U8,
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

    pub fn as_raw(&self)->&Texture{
        &self.texture
    }

    pub fn bind(&self){
        self.texture.bind(TextureBindTarget::Texture2D)
    }

    pub fn rewrite_image(
        &self,
        texture_internal_format:Texture2DInternalFormat,
        size:[u32;2],
        image_data_format:ImageDataFormat,
        image_data_type:ImageDataType,
        data:&[u8]
    ){
        self.texture.rewrite_image_2d(
            Texture2DRewriteTarget::Texture2D,
            texture_internal_format,
            size,
            image_data_format,
            image_data_type,
            data
        )
    }


    pub fn write_image(
        &self,
        [x,y,width,height]:[i32;4],
        image_data_format:ImageDataFormat,
        image_data_type:ImageDataType,
        data:&[u8]
    ){
        self.texture.write_image_2d(
            Texture2DWriteTarget::Texture2D,
            [x,y,width,height],
            image_data_format,
            image_data_type,
            data
        )
    }
}