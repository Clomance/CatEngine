use crate::{
    Colour,
};

use super::PrimitiveType;

use cat_engine_basement::graphics::{
    GCore,
    core::vertex_array::DataType,
    level0::{
        Vertex,
        VertexArray,
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
    fn bind_for_vertex_array(_vertex_array:&VertexArray<Self>){
        unsafe{
            // layout = 0 - position
            GCore.vertex_array.attribute_pointer(0,0,2,size_of::<SimpleVertex2D>() as i32,DataType::F32,false);
            GCore.vertex_array.enable_attribute(0);
            // layout = 1 - colour
            GCore.vertex_array.attribute_pointer(1,8,4,size_of::<SimpleVertex2D>() as i32,DataType::F32,false);
            GCore.vertex_array.enable_attribute(1);
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
            GCore.vertex_array.attribute_pointer(0,0,2,size_of::<TexturedVertex2D>() as i32,DataType::F32,false);
            GCore.vertex_array.enable_attribute(0);
            // layout = 1 - texture_coords
            GCore.vertex_array.attribute_pointer(1,8,2,size_of::<TexturedVertex2D>() as i32,DataType::F32,false);
            GCore.vertex_array.enable_attribute(1);
            // layout = 2 - colour_filter
            GCore.vertex_array.attribute_pointer(2,16,4,size_of::<TexturedVertex2D>() as i32,DataType::F32,false);
            GCore.vertex_array.enable_attribute(2);
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
            GCore.vertex_array.attribute_pointer(0,0,2,size_of::<TextVertex2D>() as i32,DataType::F32,false);
            GCore.vertex_array.enable_attribute(0);
            // layout = 1 - texture_coords
            GCore.vertex_array.attribute_pointer(1,8,2,size_of::<TextVertex2D>() as i32,DataType::F32,false);
            GCore.vertex_array.enable_attribute(1);
        }
    }
}

pub trait ShapeObject<V:Vertex,I:Sized>{
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