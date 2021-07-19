use crate::graphics::{
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
    pub fn initiate()->UniformBuffer<U>{
        Self{
            buffer:Buffer::initiate(),
        }
    }

    #[inline(always)]
    pub fn new(uniform:&U,usage:BufferUsage)->UniformBuffer<U>{
        unsafe{
            Self{
                buffer:Buffer::new_raw(BufferTarget::UniformBuffer,size_of::<U>() as isize,uniform,usage),
            }
        }
    }

    #[inline(always)]
    pub fn empty(usage:BufferUsage)->UniformBuffer<U>{
        unsafe{
            Self{
                buffer:Buffer::empty(BufferTarget::UniformBuffer,1,usage),
            }
        }
    }

    #[inline(always)]
    pub fn raw(&self)->&Buffer<U>{
        &self.buffer
    }

    #[inline(always)]
    pub fn into_raw(self)->Buffer<U>{
        self.buffer
    }

    #[inline(always)]
    pub fn bind(&self){
        self.buffer.bind(BufferTarget::UniformBuffer).unwrap()
    }

    #[inline(always)]
    pub fn bind_base(&self,binding_index:u32){
        self.buffer.bind_base(BufferIndexedTarget::UniformBuffer,binding_index).unwrap()
    }

    #[inline(always)]
    pub fn bind_range(&self,binding_index:u32,offset:isize,size:isize){
        self.buffer.bind_range(BufferIndexedTarget::UniformBuffer,binding_index,offset,size).unwrap()
    }

    #[inline(always)]
    pub fn write(&self,uniform:&U){
        self.buffer.write_raw(BufferTarget::UniformBuffer,0,size_of::<U>() as isize,uniform).unwrap()
    }

    #[inline(always)]
    pub fn rewrite(&self,uniform:&U,usage:BufferUsage){
        self.buffer.rewrite_raw(BufferTarget::UniformBuffer,size_of::<U>() as isize,uniform,usage).unwrap()
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