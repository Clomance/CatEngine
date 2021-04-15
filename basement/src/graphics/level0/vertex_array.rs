use gl::{
    // constants

    // functions
    VertexAttribPointer,
    GenVertexArrays,
    BindVertexArray,
    EnableVertexAttribArray,
};

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
    pub fn initialize()->VertexArray<V>{
        unsafe{
            let mut id=MaybeUninit::uninit().assume_init();
            GenVertexArrays(1,&mut id);
            Self{
                id,
                marker:PhantomData,
            }
        }
    }

    pub fn new()->VertexArray<V>{
        let vertex_array=VertexArray::initialize();
        vertex_array.bind();
        Vertex::bind_for_vertex_array(&vertex_array);
        VertexArray::<V>::unbind();
        vertex_array
    }

    #[inline(always)]
    pub fn bind(&self){
        unsafe{
            BindVertexArray(self.id)
        }
    }

    #[inline(always)]
    pub fn unbind(){
        unsafe{
            BindVertexArray(0)
        }
    }
}