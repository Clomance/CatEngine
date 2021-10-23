use crate::graphics::{
    GLCore,
    core::GLError,
    core::buffer::BufferTarget,
};

use super::Buffer;

use std::{
    marker::PhantomData,
    mem::{
        MaybeUninit
    },
};

pub trait Vertex:Sized{
    fn bind_for_vertex_array(vertex_array:&VertexArray<Self>);
}

pub struct VertexArray<V:Vertex>{
    id:u32,
    marker:PhantomData<V>
}

impl<V:Vertex> VertexArray<V>{
    pub fn generate()->VertexArray<V>{
        unsafe{
            let mut id=MaybeUninit::uninit().assume_init();
            GLCore.vertex_array.generate_one(&mut id);
            Self{
                id,
                marker:PhantomData,
            }
        }
    }

    pub fn new(vertex_buffer:&Buffer<V>)->VertexArray<V>{
        let vertex_array=VertexArray::generate();
        vertex_array.bind().unwrap();
        vertex_buffer.bind(BufferTarget::ArrayBuffer).unwrap();
        Vertex::bind_for_vertex_array(&vertex_array);
        vertex_array.unbind();
        vertex_array
    }

    #[inline(always)]
    pub fn bind(&self)->GLError{
        unsafe{
            GLCore.vertex_array.bind(self.id);
            GLCore.get_error()
        }
    }

    #[inline(always)]
    pub fn unbind(&self){
        unsafe{
            GLCore.vertex_array.bind(0)
        }
    }
}

impl<V:Vertex> Drop for VertexArray<V>{
    fn drop(&mut self){
        unsafe{
            GLCore.vertex_array.delete_one(&self.id);
        }
    }
}