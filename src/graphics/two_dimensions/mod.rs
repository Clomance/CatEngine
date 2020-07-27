use super::GraphicsSettings;

mod objects_2d;
pub (crate) use objects_2d::SimpleObject2D;
#[cfg(feature="simple_graphics")]
pub use objects_2d::Vertex2D;
#[cfg(feature="texture_graphics")]
pub (crate) use objects_2d::{TexturedObject2D,TexturedVertex2D};


#[cfg(feature="simple_graphics")]
mod simple_graphics;
#[cfg(feature="simple_graphics")]
pub (crate) use simple_graphics::SimpleGraphics;
#[cfg(feature="simple_graphics")]
pub use simple_graphics::SimpleObject;


#[cfg(feature="texture_graphics")]
mod texture_graphics;
#[cfg(feature="texture_graphics")]
pub (crate) use texture_graphics::TextureGraphics;


#[cfg(feature="text_graphics")]
mod text_graphics;
#[cfg(feature="text_graphics")]
pub (crate) use text_graphics::TextGraphics;


mod graphics_2d;
pub use graphics_2d::Graphics2D;


#[derive(Clone,Debug)]
pub struct InnerGraphicsSettings{
    /// The capacity of the vertex buffer.
    /// 
    /// The default is 8.
    pub vertex_buffer_size:usize,

    /// The range from 0 to the `offset` is for common drawing.
    /// The range from `offset` to the end is for saving objects.
    /// 
    /// The default is 4.
    pub vertex_buffer_offset:usize,

    /// The capacity of the index buffer.
    /// 
    /// The default is 8.
    pub index_buffer_size:usize,

    /// The range from 0 to the `offset` is for common drawing.
    /// The range from `offset` to the end is for saving objects.
    /// 
    /// The default is 4.
    pub index_buffer_offset:usize,

    /// The capacity of the object buffer.
    /// 
    /// The default is 2.
    pub object_buffer_size:usize,
}

impl InnerGraphicsSettings{
    pub const fn new()->InnerGraphicsSettings{
        Self{
            vertex_buffer_size:8,
            vertex_buffer_offset:4,

            index_buffer_size:8,
            index_buffer_offset:4,

            object_buffer_size:2,
        }
    }
}