//! # Графические основы. Graphics bases.
//! 
//! Графический движок разделен на три части:
//! 1. Простая графика - одноцветные объекты состоящие из `Vertex2D`.
//! 2. Текстуры (изображения)
//! 3. Текст
//! 
//! #
//! 
//! The graphics engine is divided into three parts:
//! 1. Simple graphics - plain objects composed of `Vertex2D`.
//! 2. Textures (images)
//! 3. Text

#[cfg(feature="colour_filter")]
use crate::Colour;

// #[cfg(feature="2D")]
pub (crate) mod two_dimensions;

pub use two_dimensions::{
    Graphics2D,
    TexturedVertex2D,
    Vertex2D
};


mod base;
pub use base::Graphics;

mod objects;
pub use objects::DependentObject;

#[cfg(feature="colour_filter")]
mod colour_filter;
#[cfg(feature="colour_filter")]
pub use colour_filter::{
    FilteringFunction,
    ColourFilter,
};

#[derive(Clone,Debug)]
pub enum ObjectType{
    Simple,
    Textured,
    Text,
}

#[derive(Clone,Debug)]
pub enum DrawType{
    Common,
    Shifting([f32;2]),
    Rotating((f32,[f32;2])),
}

#[derive(Clone,Debug)]
pub struct InnerGraphicsSettings{
    /// The capacity of the vertex buffer.
    /// 
    /// The default is 128.
    pub vertex_buffer_size:usize,

    /// The [0..offset] range is for common drawing,
    /// [offset..] is for saving objects.
    /// 
    /// The default is 64.
    pub vertex_buffer_offset:usize,

    /// The capacity of the index buffer.
    /// 
    /// The default is 128.
    pub index_buffer_size:usize,

    /// The [0..offset] range is for common drawing,
    /// [offset..] is for saving objects.
    /// 
    /// The default is 64.
    pub index_buffer_offset:usize,

    /// The capacity of the object buffer.
    /// 
    /// The default is 16.
    pub object_buffer_size:usize,
}

impl InnerGraphicsSettings{
    pub const fn new()->InnerGraphicsSettings{
        Self{
            vertex_buffer_size:128,
            vertex_buffer_offset:64,

            index_buffer_size:128,
            index_buffer_offset:64,

            object_buffer_size:16,
        }
    }
}

#[derive(Clone,Debug)]
pub struct TextGraphicsSettings{
    /// The size for dynamic glyph rendering.
    /// 
    /// The default is [256;2]
    pub glyph_texture_size:[u32;2],

}

impl TextGraphicsSettings{
    pub const fn new()->TextGraphicsSettings{
        Self{
            glyph_texture_size:[256;2]
        }
    }
}


/// Настройки графических основ.
/// Settings for graphics bases.
#[derive(Clone,Debug)]
pub struct GraphicsSettings{
    /// feature = "texture_graphics"
    #[cfg(feature="texture_graphics")]
    pub texture:InnerGraphicsSettings,

    /// feature = "simple_graphics"
    #[cfg(feature="simple_graphics")]
    pub simple:InnerGraphicsSettings,

    /// feature = "text_graphics"
    #[cfg(feature="text_graphics")]
    pub text:TextGraphicsSettings,
}

impl GraphicsSettings{
    pub const fn new()->GraphicsSettings{
        Self{
            #[cfg(feature="texture_graphics")]
            texture:InnerGraphicsSettings::new(),

            #[cfg(feature="simple_graphics")]
            simple:InnerGraphicsSettings::new(),

            #[cfg(feature="text_graphics")]
            text:TextGraphicsSettings::new(),
        }
    }
}