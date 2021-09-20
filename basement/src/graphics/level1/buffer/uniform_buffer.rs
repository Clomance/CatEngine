use crate::graphics::{
    core::GLError,
    core::buffer::{
        BufferTarget,
        BufferIndexedTarget,
        BufferUsage,
    },
    level0::Buffer,
};

use core::mem::size_of;

pub struct UniformBuffer<U:Sized>{
    buffer:Buffer<U>,
}

impl<U:Sized> UniformBuffer<U>{
    #[inline(always)]
    pub fn generate()->UniformBuffer<U>{
        Self{
            buffer:Buffer::generate(),
        }
    }

    #[inline(always)]
    pub unsafe fn raw(buffer:Buffer<U>)->UniformBuffer<U>{
        Self{
            buffer,
        }
    }

    pub fn new(uniform:&U,usage:BufferUsage)->Result<UniformBuffer<U>,GLError>{
        let buffer=UniformBuffer::generate();
        let result=buffer.rewrite(uniform,usage);
        if result.is_error(){
            Err(result)
        }
        else{
            Ok(buffer)
        }
    }

    pub fn empty(usage:BufferUsage)->Result<UniformBuffer<U>,GLError>{
        let buffer=UniformBuffer::generate();
        let result=buffer.rewrite_empty(usage);
        if result.is_error(){
            Err(result)
        }
        else{
            Ok(buffer)
        }
    }

    #[inline(always)]
    pub fn as_raw(&self)->&Buffer<U>{
        &self.buffer
    }

    #[inline(always)]
    pub fn into_raw(self)->Buffer<U>{
        self.buffer
    }

    #[inline(always)]
    pub fn bind(&self)->GLError{
        self.buffer.bind(BufferTarget::UniformBuffer)
    }

    #[inline(always)]
    pub fn bind_base(&self,binding_index:u32)->GLError{
        self.buffer.bind_base(BufferIndexedTarget::UniformBuffer,binding_index)
    }

    #[inline(always)]
    pub fn bind_range(&self,binding_index:u32,offset:isize,size:isize)->GLError{
        self.buffer.bind_range(BufferIndexedTarget::UniformBuffer,binding_index,offset,size)
    }

    pub fn write(&self,uniform:&U)->GLError{
        let result=self.bind();
        if result.is_error(){
            result
        }
        else{
            Buffer::write_raw(BufferTarget::UniformBuffer,0,size_of::<U>() as isize,uniform)
        }
    }

    pub fn rewrite(&self,uniform:&U,usage:BufferUsage)->GLError{
        let result=self.bind();
        if result.is_error(){
            result
        }
        else{
            Buffer::rewrite_raw(BufferTarget::UniformBuffer,size_of::<U>() as isize,Some(uniform),usage)
        }
    }

    pub fn rewrite_empty(&self,usage:BufferUsage)->GLError{
        let result=self.bind();
        if result.is_error(){
            result
        }
        else{
            Buffer::<U>::rewrite_empty(BufferTarget::UniformBuffer,1,usage)
        }
    }
}

// Unicorn - why not?
// |
//  \\\\                           ||
//   \\\\\                         ||
//     \\\\                        ||
//      \\\\           \\\\\       ||
//        \\\         \\\\\\\\\    ||
// __    __\\\__    \\\\\\\|\ \\\\ ||
// /  |  /       \  \\\\\\\\|\\ ___||_
// / | | /         \  \\\\\\\\_ \     o \______
//  __/ /| |/      \    \\\\\\\\\/               \
// |__L/ |________  \    \\\\\\\/     \____/-----/
//  __/ /\     \    \\\\\/        /
//  |__L/  \ \   \    \\\/      / /
//          \ \   \_   \/      / /
//           \ \       /      / /
//            \ \/           / /
//             \ |          / /
//              \|           /
//               |    |     /
//               \    |____/|
//                \   |  \  |
//                 \  |   \ \
//                  \ \    \ \
//                   \ \    \ \
//                    \ \    \ \
//                     \ \    \_\
//                      \_\    | \
//                       | \   \\|
//                       \\|    \|
//                        \|