use crate::graphics::{
    GCore,
    core::GLError,
    core::texture::{
        TextureBindTarget,
        Texture2DRewriteTarget,
        Texture2DWriteTarget,
        Texture2DInternalFormat,
        ImageDataFormat,
        TextureParameterTarget,
        TextureMinFilter,
        TextureMagFilter,
        TextureCompareFunction,
        TextureCompareMode,
        TextureWrap,
    },
};

use core::mem::MaybeUninit;

pub struct Texture{
    id:u32,
}

impl Texture{
    /// Generates a texture.
    pub fn generate()->Texture{
        unsafe{
            let mut id=MaybeUninit::uninit().assume_init();
            GCore.texture.generate_one(&mut id);

            Self{
                id,
            }
        }
    }

    /// Generates a texture with the given target.
    /// 
    /// Returns `GLError::InvalidValue` if there's no current context.
    pub fn create(target:TextureBindTarget)->Result<Texture,GLError>{
        let texture=Texture::generate();
        let error=texture.bind(target);
        if error.is_error(){
            Err(error)
        }
        else{
            Ok(texture)
        }
    }

    /// Binds a texture to a texturing target.
    /// 
    /// When a texture is bound to a target,
    /// the previous binding for that target is automatically broken.
    /// 
    /// Returns `GLError::NoError` if no error has accured.
    /// 
    /// Returns `GLError::InvalidValue` if there's no current context.
    /// 
    /// Returns `GLError::InvalidOperation`
    /// if texture was previously created with a target that doesn't match that of target.
    pub fn bind(&self,target:TextureBindTarget)->GLError{
        unsafe{
            GCore.texture.bind(target,self.id);
            GCore.get_error()
        }
    }

    /// Binds the zero-named texture to a texturing target.
    /// 
    /// When a texture is bound to a target,
    /// the previous binding for that target is automatically broken.
    /// 
    /// Returns `GLError::NoError` if no error has accured.
    /// 
    /// Returns `GLError::InvalidValue` if there's no current context.
    pub fn unbind(target:TextureBindTarget)->GLError{
        unsafe{
            GCore.texture.bind(target,0);
            GCore.get_error()
        }
    }

    #[inline(always)]
    pub fn id(&self)->u32{
        self.id
    }
}

impl Texture{
    /// Specifies the index of the lowest defined mipmap level.
    /// 
    /// The initial value is 0.
    #[inline(always)]
    pub fn set_base_level(target:TextureParameterTarget,level:i32){
        unsafe{
            GCore.texture.set_base_level(target,level)
        }
    }

    /// Specifies the comparison operator.
    /// 
    /// The comparison operator is used when `TEXTURE_COMPARE_MODE` is set to `COMPARE_REF_TO_TEXTURE`.
    #[inline(always)]
    pub fn set_compare_function(&self,target:TextureParameterTarget,function:TextureCompareFunction){
        unsafe{
            GCore.texture.set_compare_function(target,function)
        }
    }

    /// Specifies the texture comparison mode for currently bound depth textures (the iternal format = `DEPTH_COMPONENT`).
    #[inline(always)]
    pub fn set_compare_mode(&self,target:TextureParameterTarget,mode:TextureCompareMode){
        unsafe{
            GCore.texture.set_compare_mode(target,mode)
        }
    }

    /// Specifies the texture magnification function.
    /// 
    /// The texture magnification function is used whenever the level-of-detail function used
    /// when sampling from the texture determines that the texture should be magified.
    /// 
    /// Initially, it is set to `TextureMagFilter::Linear`.
    #[inline(always)]
    pub fn set_mag_filter(target:TextureParameterTarget,filter:TextureMagFilter){
        unsafe{
            GCore.texture.set_mag_filter(target,filter)
        }
    }

    /// Specifies the texture minifying function.
    /// 
    /// The texture minifying function is used whenever the level-of-detail function used
    /// when sampling from the texture determines that the texture should be minified.
    /// 
    /// Initially, it is set to `TextureMinFilter::LinearMipmapLinear`.
    #[inline(always)]
    pub fn set_min_filter(target:TextureParameterTarget,filter:TextureMinFilter){
        unsafe{
            GCore.texture.set_min_filter(target,filter);
        }
    }

    /// Sets the wrap parameter for texture coordinate `s`.
    /// 
    /// Initially, it is set to `TextureWrap::Repeat`.
    #[inline(always)]
    pub fn set_wrap_s(target:TextureParameterTarget,value:TextureWrap){
        unsafe{
            GCore.texture.set_wrap_s(target,value);
        }
    }

    /// Sets the wrap parameter for texture coordinate `t`.
    /// 
    /// Initially, it is set to `TextureWrap::Repeat`.
    #[inline(always)]
    pub fn set_wrap_t(target:TextureParameterTarget,value:TextureWrap){
        unsafe{
            GCore.texture.set_wrap_t(target,value);
        }
    }

    /// Sets the wrap parameter for texture coordinate `r`.
    /// 
    /// Initially, it is set to `TextureWrap::Repeat`.
    #[inline(always)]
    pub fn set_wrap_r(target:TextureParameterTarget,value:TextureWrap){
        unsafe{
            GCore.texture.set_wrap_r(target,value);
        }
    }
}

impl Texture{
    pub fn rewrite_image_2d(
        target:Texture2DRewriteTarget,
        mipmap_level:i32,
        texture_internal_format:Texture2DInternalFormat,
        size:[i32;2],
        image_data_format:ImageDataFormat,
        data:&[u8]
    )->GLError{
        unsafe{
            let data_ref=if data.len()!=0{
                (data as *const [u8]) as *const core::ffi::c_void
            }
            else{
                0 as *const core::ffi::c_void
            };
            GCore.texture.rewrite_image_2d(
                target,
                mipmap_level,
                texture_internal_format,
                size,
                image_data_format,
                data_ref
            );
            GCore.get_error()
        }
    }

    pub fn write_image_2d(
        target:Texture2DWriteTarget,
        mipmap_level:i32,
        [x,y,width,height]:[i32;4],
        image_data_format:ImageDataFormat,
        data:&[u8]
    )->GLError{
        unsafe{
            let data_ref=if data.len()!=0{
                (data as *const [u8]) as *const core::ffi::c_void
            }
            else{
                0 as *const core::ffi::c_void
            };
            GCore.texture.write_image_2d(
                target,
                mipmap_level,
                [x,y,width,height],
                image_data_format,
                data_ref
            );
            GCore.get_error()
        }
    }
}

impl Drop for Texture{
    fn drop(&mut self){
        unsafe{
            GCore.texture.delete_one(&self.id)
        }
    }
}