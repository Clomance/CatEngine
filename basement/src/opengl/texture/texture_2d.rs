use super::{
    Texture,
    TextureTarget,
    Texture2DAllocateTarget,
    TextureInternalFormat,
    PixelFormat,
    PixelType,
    TextureParameterTarget,
    TextureParameter,
    TextureMinFilter,
    TextureMagFilter,
    TextureUnit,
};

pub struct Texture2D{
    inner:Texture,
}

impl Texture2D{
    pub fn new()->Texture2D{
        let texture=Texture::new();

        texture.bind(TextureTarget::Texture2D);

        Self{
            inner:texture,
        }
    }
}