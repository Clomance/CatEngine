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

// #[cfg(feature="2D")]
pub (crate) mod two_dimensions;

#[cfg(feature="simple_graphics")]
pub use two_dimensions::{
    SimpleObject,
    Vertex2D,
};

pub use two_dimensions::{
    Graphics2D,
    InnerGraphicsSettings
};

#[cfg(feature="3D")]
pub (crate) mod three_dimensions;


mod base;
pub use base::Graphics;

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

    /// The default is 2000.
    /// 
    /// feature = "text_graphics"
    #[cfg(feature="text_graphics")]
    pub text_vertex_buffer_size:usize,
}

impl GraphicsSettings{
    pub const fn new()->GraphicsSettings{
        Self{
            #[cfg(feature="texture_graphics")]
            texture:InnerGraphicsSettings::new(),

            #[cfg(feature="simple_graphics")]
            simple:InnerGraphicsSettings::new(),

            #[cfg(feature="text_graphics")]
            text_vertex_buffer_size:2000usize,
        }
    }
}