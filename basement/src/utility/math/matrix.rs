pub use glm;
use glm::{Mat4, Vec3};

pub struct Matrix{
    matrix:Mat4
}

impl Matrix{
    pub fn new()->Matrix{
        Self{
            matrix:glm::mat4(
                1f32,0f32,0f32,0f32,
                0f32,1f32,0f32,0f32,
                0f32,0f32,1f32,0f32,
                0f32,0f32,0f32,1f32
            ),
        }
    }

    pub fn reset(&mut self){
        self.matrix=glm::mat4(
            1f32,0f32,0f32,0f32,
            0f32,1f32,0f32,0f32,
            0f32,0f32,1f32,0f32,
            0f32,0f32,0f32,1f32
        );
    }

    pub fn rotate(&mut self,angle:f32,[x,y,z]:[f32;3]){
        self.matrix=glm::ext::rotate(&self.matrix,angle,Vec3::new(x,y,z))
    }

    pub fn translate(&mut self,[x,y,z]:[f32;3]){
        self.matrix=glm::ext::translate(&self.matrix,Vec3::new(x,y,z))
    }

    pub fn scale(&mut self,[x,y,z]:[f32;3]){
        self.matrix=glm::ext::scale(&self.matrix,Vec3::new(x,y,z))
    }

    pub fn ptr(&self)->*const f32{
        unsafe{
            std::mem::transmute(self.matrix.as_array())
        }
    }
}