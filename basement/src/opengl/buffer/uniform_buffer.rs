use super::{
    Core,
    Buffer,
    BufferTarget,
    BufferIndexedTarget,
    BufferUsage,
    Error,
};

use core::{
    mem::size_of,
    marker::PhantomData,
    ptr::null,
};

pub struct UniformBuffer<T>{
    buffer:Buffer<T>
}

impl<T> UniformBuffer<T>{
    pub fn new(data:&T,usage:BufferUsage)->UniformBuffer<T>{
        unsafe{
            let buffer=Buffer::new();

            buffer.bind(BufferTarget::UniformBuffer);

            Buffer::allocate(BufferTarget::UniformBuffer,size_of::<T>() as isize,data,usage);

            Self{
                buffer
            }
        }
    }

    pub fn empty(usage:BufferUsage)->UniformBuffer<T>{
        unsafe{
            let buffer=Buffer::new();

            buffer.bind(BufferTarget::UniformBuffer);

            Buffer::<T>::allocate(BufferTarget::UniformBuffer,size_of::<T>() as isize,null(),usage);

            Self{
                buffer
            }
        }
    }

    pub fn bind(&self){
        unsafe{
            self.buffer.bind(BufferTarget::UniformBuffer);
        }
    }

    pub fn bind_base(&self,index:u32){
        unsafe{
            self.buffer.bind_base(BufferIndexedTarget::UniformBuffer,index);
        }
    }

    pub fn bind_range(&self,index:u32,offset:isize,size:isize){
        unsafe{
            self.buffer.bind_range(BufferIndexedTarget::UniformBuffer,index,offset,size);
        }
    }

    pub fn id(&self)->u32{
        self.buffer.id()
    }
}

impl<T> UniformBuffer<T>{
    pub fn reallocate(&self,data:&T,usage:BufferUsage)->Result<(),Error>{
        unsafe{
            self.buffer.bind(BufferTarget::UniformBuffer);
            Buffer::allocate(BufferTarget::UniformBuffer,size_of::<T>() as isize,data,usage);
            let error=Core::get_error();
            if let Error::None=error{
                Result::Ok(())
            }
            else{
                Result::Err(error)
            }
        }
    }

    pub fn write(&self,data:&T)->Result<(),Error>{
        unsafe{
            self.buffer.bind(BufferTarget::UniformBuffer);
            Buffer::write(BufferTarget::UniformBuffer,0,size_of::<T>() as isize,data);
            let error=Core::get_error();
            if let Error::None=error{
                Result::Ok(())
            }
            else{
                Result::Err(error)
            }
        }
    }
}