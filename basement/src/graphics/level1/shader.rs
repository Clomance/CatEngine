use crate::graphics::{
    core::shader::ShaderType,
    level0::Shader,
};

pub struct VertexShader{
    shader:Shader,
}

impl VertexShader{
    /// Creates and compiles a shader.
    /// 
    /// Создаёт и компилирует шейдер.
    pub fn new(source:&str)->Result<VertexShader,String>{
        match Shader::new(source,ShaderType::VertexShader){
            Ok(shader)=>Ok(Self{shader}),
            Err(error)=>Err(error)
        }
    }

    pub fn id(&self)->u32{
        self.shader.id()
    }
}

pub struct FragmentShader{
    shader:Shader,
}

impl FragmentShader{
    /// Creates and compiles a shader.
    /// 
    /// Создаёт и компилирует шейдер.
    pub fn new(source:&str)->Result<FragmentShader,String>{
        match Shader::new(source,ShaderType::FragmentShader){
            Ok(shader)=>Ok(Self{shader}),
            Err(error)=>Err(error)
        }
    }

    pub fn id(&self)->u32{
        self.shader.id()
    }
}