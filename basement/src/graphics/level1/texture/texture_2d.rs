use crate::graphics::{
    core::GLError,
    core::texture::{
        TextureBindTarget,
        Texture2DRewriteTarget,
        Texture2DWriteTarget,
        TextureMagFilter,
        TextureMinFilter,
        Texture2DInternalFormat,
        ImageDataFormat,
        TextureParameterTarget,
    },
    level0::Texture,
};

pub struct Texture2D{
    texture:Texture,
}

impl Texture2D{
    pub fn create()->Result<Texture2D,GLError>{
        match Texture::create(TextureBindTarget::Texture2D){
            Result::Ok(texture)=>Ok(Self{texture}),
            Result::Err(e)=>Err(e),
        }
    }

    /// Creates a texture.
    pub fn new(
        texture_internal_format:Texture2DInternalFormat,
        mag_filter:TextureMagFilter,
        min_filter:TextureMinFilter,
        size:[u32;2],
        image_data_format:ImageDataFormat,
        data:&[u8]
    )->Result<Texture2D,GLError>{
        let texture=Texture2D::create()?;

        Texture::set_min_filter(TextureParameterTarget::Texture2D,min_filter);
        Texture::set_mag_filter(TextureParameterTarget::Texture2D,mag_filter);

        Texture::rewrite_image_2d(
            Texture2DRewriteTarget::Texture2D,
            0,
            texture_internal_format,
            [size[0] as i32,size[1] as i32],
            image_data_format,
            data
        );

        Ok(texture)
    }

    pub fn empty(
        texture_internal_format:Texture2DInternalFormat,
        mag:TextureMagFilter,
        min:TextureMinFilter,
        size:[u32;2]
    )->Result<Texture2D,GLError>{
        Texture2D::new(
            texture_internal_format,
            mag,
            min,
            size,
            ImageDataFormat::R_U8,
            &[]
        )
    }

    pub fn raw(texture:Texture)->Texture2D{
        Self{
            texture,
        }
    }

    pub fn as_raw(&self)->&Texture{
        &self.texture
    }

    pub fn into_raw(self)->Texture{
        self.texture
    }

    pub fn bind(&self)->GLError{
        self.texture.bind(TextureBindTarget::Texture2D)
    }
}

impl Texture2D{
    pub fn rewrite_image(
        &self,
        texture_internal_format:Texture2DInternalFormat,
        size:[u32;2],
        image_data_format:ImageDataFormat,
        data:&[u8]
    )->GLError{
        let result=self.bind();
        if result.is_error(){
            result
        }
        else{
            Texture::rewrite_image_2d(
                Texture2DRewriteTarget::Texture2D,
                0,
                texture_internal_format,
                [size[0] as i32,size[1] as i32],
                image_data_format,
                data
            )
        }
    }

    pub fn write_image(
        &self,
        [x,y,width,height]:[u32;4],
        image_data_format:ImageDataFormat,
        data:&[u8]
    )->GLError{
        let result=self.bind();
        if result.is_error(){
            result
        }
        else{
            Texture::write_image_2d(
                Texture2DWriteTarget::Texture2D,
                0,
                [x as i32,y as i32,width as i32,height as i32],
                image_data_format,
                data
            )
        }
    }
}