//! # Графические основы. Graphics basics.
//! 
//! Графический движок разделен на три части:
//! 1. Простая графика - одноцветные объекты состоящие из `Vertex2D`.
//! 2. Текстуры (изображения)
//! 3. Текст
//! 
//! Обычные функции рисования используют VertexBuffer с начала.
//! 
//! Также есть возможность сохранять и использовать координаты объектов
//! (пример ниже).
//! 
//! #
//! 
//! The graphics engine is divided into three parts:
//! 1. Simple graphics - plain objects composed of `Vertex2D`.
//! 2. Textures (images)
//! 3. Text
//! 
//! Common functions use VertexBuffer from the beginning.
//! 
//! Also it's possible to save and use vertices of objects.
//! 
//! #
//! 
//! ```
//! let image_base=ImageBase::new(White,unsafe{[
//!     (window_width-400f32)/2f32,
//!     (window_height-400f32)/2f32,
//!     400f32,
//!     400f32
//! ]});
//! 
//! let range=window.graphics().bind_image(4..8usize,image_base).unwrap();
//! 
//! let logo=Texture::from_path("./resources/images/logo.png",window.display()).unwrap();
//! 
//! window.draw(|parameters,graphics|{
//!     graphics.clear_colour(White);
//!     graphics.draw_range_image(
//!         range,
//!         &logo,
//!         White,
//!         parameters
//!     );
//! });
//! 
//! window.graphics().unbind_texture(range);
//! ```

// #[cfg(feature="2D")]
pub (crate) mod two_dimensions;

pub use two_dimensions::Graphics2D;

#[cfg(feature="simple_graphics")]
pub use two_dimensions::{
    SimpleObject,
    Vertex2D,
    SimpleGraphicsSettings
};


#[cfg(feature="3D")]
pub (crate) mod three_dimensions;


mod base;
pub use base::Graphics;



/// Настройки графических основ.
/// Settings for graphics basics.
#[derive(Clone,Debug)]
pub struct GraphicsSettings{
    /// The default is 8.
    /// 
    /// feature = "texture_graphics"
    #[cfg(feature="texture_graphics")]
    pub texture_vertex_buffer_size:usize,

    /// feature = "simple_graphics"
    #[cfg(feature="simple_graphics")]
    pub simple:SimpleGraphicsSettings,

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
            texture_vertex_buffer_size:8usize,

            #[cfg(feature="simple_graphics")]
            simple:SimpleGraphicsSettings::new(),

            #[cfg(feature="text_graphics")]
            text_vertex_buffer_size:2000usize,
        }
    }
}