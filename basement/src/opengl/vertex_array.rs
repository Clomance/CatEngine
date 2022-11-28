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
            let mut id=MaybeUninit::uninit();

            VertexArrayFunctions::generate(1,id.as_mut_ptr());

            Self{
                id:id.assume_init()
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