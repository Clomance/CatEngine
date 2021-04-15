use crate::{
    Colour,
};

use super::PrimitiveType;

use cat_engine_basement::graphics::{
    level0::{
        Vertex,
        VertexArray,
    },
    gl::{
        FLOAT,FALSE,
        VertexAttribPointer,
        EnableVertexAttribArray,
    },
};

use std::mem::size_of;

#[derive(Clone,Debug)]
pub struct SimpleVertex2D{
    pub position:[f32;2],
    pub colour:Colour
}

impl SimpleVertex2D{
    pub const fn new(position:[f32;2],colour:Colour)->SimpleVertex2D{
        Self{
            position,
            colour,
        }
    }
}

impl Vertex for SimpleVertex2D{
    fn bind_for_vertex_array(vertex_array:&VertexArray<Self>){
        vertex_array.bind();

        unsafe{
            // layout = 0 - position
            VertexAttribPointer(0,2,FLOAT,FALSE,size_of::<SimpleVertex2D>() as i32,core::ptr::null());
            EnableVertexAttribArray(0);
            // layout = 1 - colour
            let offset=2*4;
            VertexAttribPointer(1,4,FLOAT,FALSE,size_of::<SimpleVertex2D>() as i32,offset as *const _);
            EnableVertexAttribArray(1);
        }
    }
}

#[derive(Clone,Debug)]
pub struct TexturedVertex2D{
    pub position:[f32;2],
    pub tex_coords:[f32;2],
    pub colour:Colour
}

impl TexturedVertex2D{
    pub const fn new(position:[f32;2],tex_coords:[f32;2],colour:Colour)->TexturedVertex2D{
        Self{
            position,
            tex_coords,
            colour,
        }
    }
}

impl Vertex for TexturedVertex2D{
    fn bind_for_vertex_array(_vertex_array:&VertexArray<Self>){
        unsafe{
            // layout = 0 - position
            VertexAttribPointer(0,2,FLOAT,FALSE,size_of::<TexturedVertex2D>() as i32,core::ptr::null());
            EnableVertexAttribArray(0);
            // layout = 1 - texture_coords
            let offset=2*4;
            VertexAttribPointer(1,2,FLOAT,FALSE,size_of::<TexturedVertex2D>() as i32,offset as *const _);
            EnableVertexAttribArray(1);
            // layout = 2 - colour_filter
            let offset=4*4;
            VertexAttribPointer(2,4,FLOAT,FALSE,size_of::<TexturedVertex2D>() as i32,offset as *const _);
            EnableVertexAttribArray(2);
        }
    }
}


#[derive(Copy,Clone)]
pub struct TextVertex2D{
    pub position:[f32;2],
    pub tex_coords:[f32;2],
}

impl TextVertex2D{
    pub const fn new(position:[f32;2],tex_coords:[f32;2])->TextVertex2D{
        Self{
            position,
            tex_coords,
        }
    }
}

impl Vertex for TextVertex2D{
    fn bind_for_vertex_array(_vertex_array:&VertexArray<Self>){
        unsafe{
            // layout = 0 - position
            VertexAttribPointer(0,2,FLOAT,FALSE,size_of::<TextVertex2D>() as i32,core::ptr::null());
            EnableVertexAttribArray(0);
            // layout = 1 - texture_coords
            let offset=2*4;
            VertexAttribPointer(1,2,FLOAT,FALSE,size_of::<TextVertex2D>() as i32,offset as *const _);
            EnableVertexAttribArray(1);
        }
    }
}

/// Типаж для создания объектов, которые зависят от буферов вершин и индексов.
/// A trait for creating objects depend on vertex and index buffers.
/// 
/// The crate's graphics engine uses `TexturedVertex2D` and `Vertex2D` for vertices
/// and `u8` for indices, but you can add your own
/// and draw objects with `window.draw()`.
pub trait DependentObject<V:Vertex,I:Sized>{
    type Vertices:AsRef<[V]>;
    type Indices:AsRef<[I]>;

    /// Вершины объекта.
    /// 
    /// Object's vertices.
    /// 
    /// The crate's graphics uses the window coordinate system.
    fn vertices(&self)->Self::Vertices;

    /// Индексы для построения объекта.
    /// 
    /// Object's indices.
    fn indices(&self)->Self::Indices;

    fn primitive_type(&self)->PrimitiveType;
}