use super::core::{
    Core,
    Error,
    buffer::Buffer as BufferFunctions,
};

pub use super::core::{
    buffer::{
        BufferUsage,
        BufferTarget,
        BufferIndexedTarget,
    }
};

mod vertex_buffer;
pub use vertex_buffer::VertexBuffer;

mod index_buffer;
pub use index_buffer::IndexBuffer;

mod uniform_buffer;
pub use uniform_buffer::UniformBuffer;

use core::{
    mem::{
        size_of,
        MaybeUninit
    },
    marker::PhantomData,
    ptr::null,
};

pub struct Buffer<T>{
    id:u32,
    marker:PhantomData<T>
}

impl<T> Buffer<T>{
    pub fn new()->Buffer<T>{
        unsafe{
            let mut id:u32=MaybeUninit::uninit().assume_init();

            BufferFunctions::generate(1,&mut id);

            Self{
                id,
                marker:PhantomData
            }
        }
    }

    pub fn id(&self)->u32{
        self.id
    }

    pub unsafe fn bind(&self,target:BufferTarget){
        BufferFunctions::bind(target,self.id)
    }

    pub unsafe fn bind_base(&self,target:BufferIndexedTarget,index:u32){
        BufferFunctions::bind_base(target,index,self.id)
    }

    pub unsafe fn bind_range(&self,target:BufferIndexedTarget,index:u32,offset:isize,size:isize){
        BufferFunctions::bind_range(target,index,self.id,offset,size)
    }
}

impl Buffer<()>{
    pub unsafe fn unbind(target:BufferTarget){
        BufferFunctions::bind(target,0)
    }

    pub unsafe fn unbind_base(target:BufferIndexedTarget,index:u32){
        BufferFunctions::bind_base(target,index,0)
    }
}

impl<T> Buffer<T>{
    pub unsafe fn allocate(target:BufferTarget,size:isize,data:*const T,usage:BufferUsage){
        BufferFunctions::allocate(target,size,data,usage)
    }

    pub unsafe fn write(target:BufferTarget,offset:isize,size:isize,data:*const T){
        BufferFunctions::write(target,offset,size,data)
    }

    pub unsafe fn read(target:BufferTarget,offset:isize,size:isize,data:*mut T){
        BufferFunctions::read(target,offset,size,data)
    }
}

impl<T> Drop for Buffer<T>{
    fn drop(&mut self){
        unsafe{
            BufferFunctions::delete(1,&self.id)
        }
    }
}

pub struct BufferSlice<'a,T>{
    buffer:&'a Buffer<T>,
    start:isize,
    end:isize,
}