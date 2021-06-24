use super::level0::{
    Buffer,
    BoundBuffer,
    BufferTarget,
    BufferUsage
};

use gl::{
    MAX_UNIFORM_BUFFER_BINDINGS
};

/// Since 3.1.
/// 
/// Not fully supported yet.
pub struct UniformBuffer<U:Sized>{
    buffer:Buffer<U>,
}

impl<U:Sized> UniformBuffer<U>{
    #[inline(always)]
    pub fn initialize()->UniformBuffer<U>{
        Self{
            buffer:Buffer::initialize(),
        }
    }

    #[inline(always)]
    pub fn new(uniform:&U,usage:BufferUsage)->UniformBuffer<U>{
        unsafe{
            Self{
                buffer:Buffer::new(BufferTarget::UniformBuffer,uniform,usage),
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
}

impl<U:Sized> UniformBuffer<U>{
    #[inline(always)]
    pub fn bind(&self)->BoundUniformBuffer<U>{
        unsafe{
            BoundUniformBuffer{
                marker:self.buffer.bind(BufferTarget::UniformBuffer)
            }
        }
    }

    pub fn bind_base(&self,binding_index:u32){
        unsafe{
            if binding_index<MAX_UNIFORM_BUFFER_BINDINGS{
                self.buffer.bind_base(BufferTarget::UniformBuffer,binding_index)
            }
        }
    }

    // offset, size - bytes
    pub fn bind_range(&self,binding_index:u32,offset:isize,size:isize){
        unsafe{
            if binding_index<MAX_UNIFORM_BUFFER_BINDINGS{
                self.buffer.bind_range(BufferTarget::UniformBuffer,binding_index,offset,size)
            }
        }
    }
}

pub struct BoundUniformBuffer<'a,U:Sized>{
    marker:BoundBuffer<'a,U>
}

impl<'a,U:Sized> BoundUniformBuffer<'a,U>{
    #[inline(always)]
    pub fn write(&self,uniform:&U){
        unsafe{
            self.marker.write(0,uniform)
        }
    }

    #[inline(always)]
    pub fn rewrite(&self,uniform:&U,usage:BufferUsage){
        unsafe{
            self.marker.rewrite(uniform,usage)
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