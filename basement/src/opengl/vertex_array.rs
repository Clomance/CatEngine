use crate::opengl::core::{
    Core,
    vertex_array::VertexArray as VertexArrayFunctions,
    Error,
};

use core::{
    mem::{
        size_of,
        MaybeUninit,
    },
    marker::PhantomData,
    ptr::null,
};

pub struct VertexArray{
    id:u32
}

impl VertexArray{
    pub fn new()->VertexArray{
        unsafe{
            let mut id:u32=MaybeUninit::uninit().assume_init();

            VertexArrayFunctions::generate(1,&mut id);

            Self{
                id
            }
        }
    }

    pub fn bind(&self){
        unsafe{
            VertexArrayFunctions::bind(self.id);
        }
    }

    pub fn id(&self)->u32{
        self.id
    }
}

impl Drop for VertexArray{
    fn drop(&mut self){
        unsafe{
            VertexArrayFunctions::delete(1,&self.id)
        }
    }
}