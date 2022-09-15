use super::{
    ElementIndexType,
    RenderData,
    Vertex,
    BufferedMesh,
    Layer,
    MeshAttributes,
    MeshError, ObjectAttributes,
};

use crate::{object::TextureObject, texture::Texture2D};

use cat_engine_basement::{
    opengl::{
        core::{
            vertex_array::{
                VertexArray as VertexArrayFunctions,
                VertexComponents,
                DataType,
            },
            program::{
                Program as ProgramFunctions,
            },
            uniform::Uniform,
        },
        shader::{
            Shader,
            ShaderType,
        },
        program::{
            Program,
        },
        texture::{
            TextureTarget
        },
    },
    support::{
        storage::StaticStorage,
        math::matrix::Matrix
    },
};

const vertex_shader_source:&'static str=include_str!("shaders/texture/vertex.glsl");
const fragment_shader_source:&'static str=include_str!("shaders/texture/fragment.glsl");

struct TextureLayer{
    program:u32,
    draw_parameters:Matrix,
    parameters_location:i32,

    texture:*mut Texture2D,
    mesh:BufferedMesh<TexturedVertex,ElementIndexType>,
}

impl TextureLayer{
    pub fn new(
        program:u32,
        parameters_location:i32,
        texture:*mut Texture2D,
        mesh:BufferedMesh<TexturedVertex,ElementIndexType>
    )->TextureLayer{
        Self{
            program,
            draw_parameters:Matrix::new(),
            parameters_location,
            texture,
            mesh
        }
    }
}

impl Layer for TextureLayer{
    fn draw(&mut self){
        unsafe{
            ProgramFunctions::bind(self.program);
            Uniform::set_matrix_4_f32(self.parameters_location,1,false,self.draw_parameters.ptr());
        }

        self.mesh.vertex_array.bind();
        self.mesh.flush_vertices();
        self.mesh.flush_indices();

        unsafe{&*self.texture}.inner.bind(TextureTarget::Texture2D);

        self.mesh.draw();

        unsafe{
            VertexArrayFunctions::bind(0);
        }
    }
}

#[derive(Debug,Clone,Copy)]
pub struct TexturedVertex{
    pub position:[f32;4],
    pub colour:[f32;4],
    pub texture_coordinates:[f32;2]
}

impl TexturedVertex{
    pub fn new(position:[f32;4],colour:[f32;4],texture_coordinates:[f32;2])->TexturedVertex{
        Self{
            position,
            colour,
            texture_coordinates
        }
    }
}

impl Vertex for TexturedVertex{
    fn attribute_pointer(){
        unsafe{
            VertexArrayFunctions::attribute_pointer(0,VertexComponents::Four,DataType::F32,false,40,0 as *const _);
            VertexArrayFunctions::enable_attribute(0);
            VertexArrayFunctions::attribute_pointer(1,VertexComponents::Four,DataType::F32,false,40,16 as *const _);
            VertexArrayFunctions::enable_attribute(1);
            VertexArrayFunctions::attribute_pointer(2,VertexComponents::Two,DataType::F32,false,40,32 as *const _);
            VertexArrayFunctions::enable_attribute(2);
        }
    }
}

pub struct TextureGraphicsAttributes{
    pub texture_limit:usize,
    pub layer_limit:usize,
}

impl TextureGraphicsAttributes{
    pub const fn new()->TextureGraphicsAttributes{
        Self{
            texture_limit:10,
            layer_limit:10
        }
    }
}

pub struct TextureGraphics{
    texture_usage:Vec<u8>,
    texture_storage:StaticStorage<Texture2D>,

    program:Program,
    layer_draw_parameters_location:i32,

    valid:Vec<u8>,
    texture_attached:Vec<usize>,
    layers:StaticStorage<TextureLayer>
}

impl TextureGraphics{
    pub (crate) fn new(attributes:&TextureGraphicsAttributes)->TextureGraphics{
        let vertex_shader=Shader::new(vertex_shader_source,ShaderType::VertexShader).unwrap();
        let fragment_shader=Shader::new(fragment_shader_source,ShaderType::FragmentShader).unwrap();

        let program=Program::new();
        program.attach_shader(&vertex_shader);
        program.attach_shader(&fragment_shader);
        program.link().unwrap();
        program.bind();

        program.bind_uniform_block("DrawParameters\0",0);

        let layer_draw_parameters_location=program.get_uniform_location("LayerDrawParameters\0").unwrap();

        Self{
            texture_usage:vec![0u8;attributes.texture_limit],
            texture_storage:StaticStorage::new(attributes.texture_limit),

            program,
            layer_draw_parameters_location,

            valid:vec![0u8;attributes.layer_limit],
            texture_attached:vec![0usize;attributes.layer_limit],
            layers:StaticStorage::new(attributes.layer_limit),
        }
    }

    pub (crate) fn get_render_data(&mut self,layer:usize,object:usize)->RenderData<TexturedVertex,ElementIndexType>{
        let layer=self.layers.get_mut(layer).unwrap();
        let info=layer.mesh.get_render_data(object).unwrap();

        let vertex_start=info.vertex_buffer_start;
        let vertex_count=info.vertex_buffer_count;

        let index_start=info.index_buffer_start;
        let index_count=info.index_buffer_count;

        RenderData::new(
            &mut layer.mesh,
            object,
            vertex_start as usize,
            vertex_count as usize,
            index_start as usize,
            index_count as usize
        )
    }

    pub fn create_layer(&mut self,attributes:MeshAttributes)->Option<usize>{
        self.layers.add(
            TextureLayer::new(
                self.program.id(),
                self.layer_draw_parameters_location,
                0 as *mut _,
                BufferedMesh::new(attributes)
            )
        )
    }

    pub (crate) fn attach_layer(&mut self,layer:usize,texture:usize)->Option<&'static mut dyn Layer>{
        unsafe{
            let texture_index=texture;
            let texture=self.texture_storage.get_mut(texture).unwrap();
            if let Some(mesh)=self.layers.get_mut(layer){
                mesh.texture=texture;

                self.texture_usage[texture_index]+=1;

                self.texture_attached[layer]=texture_index;

                self.valid[layer]+=1;

                Some(std::mem::transmute(mesh as &mut dyn Layer))
            }
            else{
                None
            }
        }
    }

    /// Косвенно открепляет слой от рендеринга и текстуры.
    /// 
    /// Используется только внутри движка: layer - всегда существует.
    pub (crate) fn detach_layer(&mut self,layer:usize){
        unsafe{
            *self.valid.get_unchecked_mut(layer)-=1;

            let texture_index=self.texture_attached.get_unchecked(layer).clone();

            *self.texture_usage.get_unchecked_mut(texture_index)-=1;
        }
    }

    pub fn remove_layer(&mut self,layer:usize)->bool{
        if let Some(mesh)=self.layers.get(layer){
            if mesh.mesh.is_empty() && self.valid[layer]==0{
                self.layers.remove(layer);
                return true
            }
        }
        false
    }

    pub fn layer_transform_matrix(&mut self,layer:usize)->Option<&mut Matrix>{
        if let Some(layer)=self.layers.get_mut(layer){
            Some(&mut layer.draw_parameters)
        }
        else{
            None
        }
    }
}



impl TextureGraphics{
    pub fn push_texture(&mut self,texture:Texture2D)->Option<usize>{
        self.texture_storage.add(texture)
    }

    pub fn get_texture(&mut self,texture:usize)->Option<&mut Texture2D>{
        self.texture_storage.get_mut(texture)
    }

    pub fn get_layer_texture(&mut self,layer:usize)->Option<&mut Texture2D>{
        unsafe{
            if let Some(layer)=self.layers.get(layer){
                Some(&mut *layer.texture)
            }
            else{
                None
            }
        }
    }

    pub (crate) fn get_layer_texture_raw(&mut self,layer:usize)->*mut Texture2D{
        self.layers.get(layer).unwrap().texture
    }

    pub fn remove_texture(&mut self,texture:usize)->Option<Texture2D>{
        if let Some(usage)=self.texture_usage.get(texture){
            if *usage==0{
                return self.texture_storage.remove(texture)
            }
        }
        None
    }
}



impl TextureGraphics{
    pub (crate) fn push_object(
        &mut self,
        attributes:ObjectAttributes<TexturedVertex,u16>,
        layer:usize
    )->Result<usize,MeshError>{
        let layer=self.layers.get_mut(layer).unwrap();

        layer.mesh.push_object(attributes)
    }

    pub (crate) fn remove_object(&mut self,layer:usize,object:usize){
        let layer=self.layers.get_mut(layer).unwrap();
        layer.mesh.remove_object(object)
    }
}