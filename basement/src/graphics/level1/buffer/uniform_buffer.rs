use super::level0::{
    Buffer,
    BoundBuffer,
    BufferTarget,
    BufferUsage
};

pub struct BoundUniformBuffer<'a,U:Sized>{
    marker:BoundBuffer<'a,U>
}

/// Since 3.1.
/// 
/// Not supported yet.
pub struct UniformBuffer<U:Sized>{
    buffer:Buffer<U>,
}

impl<U:Sized> UniformBuffer<U>{
    pub fn initialize()->UniformBuffer<U>{
        Self{
            buffer:Buffer::initialize(),
        }
    }

    pub fn new(uniform:&U,usage:BufferUsage)->UniformBuffer<U>{
        unsafe{
            Self{
                buffer:Buffer::new_value(BufferTarget::UniformBuffer,uniform,usage),
            }
        }
    }

    /// The size is the amount of Iertices.
    pub fn empty(size:usize,usage:BufferUsage)->UniformBuffer<U>{
        unsafe{
            Self{
                buffer:Buffer::empty(BufferTarget::UniformBuffer,size,usage),
            }
        }
    }
}

impl<U:Sized> UniformBuffer<U>{
    pub fn bind(&self)->BoundUniformBuffer<U>{
        unsafe{
            BoundUniformBuffer{
                marker:self.buffer.bind(BufferTarget::UniformBuffer)
            }
        }
    }
}

pub trait UniformBlock{
    
}

pub trait UniformValue{

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