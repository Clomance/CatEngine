use crate::Colour;

#[cfg(feature="texture_graphics")]
use crate::image::Texture;

use glium::{
    implement_vertex,
    index::{
        PrimitiveType,
        IndicesSource,
        IndexType,
    },
    vertex::{
        VerticesSource,
        VertexFormat,
    },
    buffer::{
        Buffer,
        Content,
    },
};

use core::ops::Range;

implement_vertex!(Vertex2D,position);
/// Вершина для простых объектов.
/// A vertex for simple objects.
#[derive(Copy,Clone)]
pub struct Vertex2D{
    pub position:[f32;2],
}

impl Vertex2D{
    pub const fn new(x:f32,y:f32)->Vertex2D{
        Self{
            position:[x,y]
        }
    }
}

pub struct SimpleObject2D{
    pub (crate) vertex_buffer_range:Range<usize>,
    pub (crate) index_buffer_range:Range<usize>,
    pub (crate) primitive_type:PrimitiveType,
    pub (crate) colour:Colour,
}

impl SimpleObject2D{
    /// Returns the vertices source of the object.
    pub fn vertices_source<'a,T:Copy>(
        &self,
        vertices:&'a Buffer<[T]>,
        vertex_format:&'a VertexFormat
    )->VerticesSource<'a>
        where [T]:Content
    {
        let slice=vertices.slice(self.vertex_buffer_range.clone()).unwrap();

        VerticesSource::VertexBuffer(
            slice.as_slice_any(),
            &vertex_format,
            false
        )
    }

    /// Returns the indices source of the object.
    pub fn indices_source<'a>(
        &self,
        indices:&'a Buffer<[u8]>,
    )->IndicesSource<'a>{
        if self.index_buffer_range.len()!=0{
            let slice=indices.slice(self.index_buffer_range.clone()).unwrap();
            IndicesSource::IndexBuffer{
                buffer:slice.as_slice_any(),
                data_type:IndexType::U8,
                primitives:self.primitive_type,
            }
        }
        else{
            IndicesSource::NoIndices{primitives:self.primitive_type}
        }
    }
}


implement_vertex!(TexturedVertex2D,position,tex_coords);
#[derive(Copy,Clone)]
pub struct TexturedVertex2D{
    position:[f32;2],
    tex_coords:[f32;2],
}

impl TexturedVertex2D{
    pub const fn new(position:[f32;2],tex_coords:[f32;2])->TexturedVertex2D{
        Self{
            position,
            tex_coords,
        }
    }
}

#[cfg(feature="texture_graphics")]
pub struct TexturedObject2D{
    pub (crate) base:SimpleObject2D,
    pub (crate) texture:Texture,
}

#[cfg(feature="texture_graphics")]
impl TexturedObject2D{
    /// Returns the vertices source of the object.
    pub fn vertices_source<'a,T:Copy>(
        &self,
        vertices:&'a Buffer<[T]>,
        vertex_format:&'a VertexFormat
    )->VerticesSource<'a>
        where [T]:Content
    {
        self.base.vertices_source(vertices,vertex_format)
    }

    /// Returns the indices source of the object.
    pub fn indices_source<'a>(
        &self,
        indices:&'a Buffer<[u8]>,
    )->IndicesSource<'a>{
        self.base.indices_source(indices)
    }
}
