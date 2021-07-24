use crate::graphics::{
    GCore,
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
    pub fn initiate()->VertexArray<V>{
        unsafe{
            let mut id=MaybeUninit::uninit().assume_init();
            GCore.vertex_array.generate_one(&mut id);
            Self{
                id,
                marker:PhantomData,
            }
        }
    }

    pub fn new(vertex_buffer:&Buffer<V>)->VertexArray<V>{
        let vertex_array=VertexArray::initiate();
        vertex_array.bind().unwrap();
        vertex_buffer.bind(BufferTarget::ArrayBuffer).unwrap();
        Vertex::bind_for_vertex_array(&vertex_array);
        vertex_array.unbind();
        vertex_array
    }

    #[inline(always)]
    pub fn bind(&self)->GLError{
        unsafe{
            GCore.vertex_array.bind(self.id);
            GCore.get_error()
        }
    }

    #[inline(always)]
    pub fn unbind(&self){
        unsafe{
            GCore.vertex_array.bind(0)
        }
    }
}

impl<V:Vertex> Drop for VertexArray<V>{
    fn drop(&mut self){
        unsafe{
            GCore.vertex_array.delete_one(&self.id);
        }
    }
}