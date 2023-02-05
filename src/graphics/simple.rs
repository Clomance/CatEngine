use super::{
    ElementIndexType,
    Vertex,
    BufferedMesh,
    RenderData,
    Layer,
    mesh::MeshAttributes,
    MeshError,
    ObjectAttributes
};

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
    },
    utility::storage::StaticStorage,
    utility::math::matrix::Matrix
};

const VERTEX_SHADER_SOURCE:&'static str=include_str!("shaders/simple/vertex.glsl");
const FRAGMENT_SHADER_SOURCE:&'static str=include_str!("shaders/simple/fragment.glsl");

#[derive(Debug,Clone,Copy)]
pub struct SimpleVertex{
    pub position:[f32;4],
    pub colour:[f32;4]
}

impl SimpleVertex{
    pub fn new(position:[f32;4],colour:[f32;4])->SimpleVertex{
        Self{
            position,
            colour
        }
    }
}

impl Vertex for SimpleVertex{
    fn attribute_pointer(){
        unsafe{
            VertexArrayFunctions::attribute_pointer(0,VertexComponents::Four,DataType::F32,false,32,0 as *const _);
            VertexArrayFunctions::enable_attribute(0);
            VertexArrayFunctions::attribute_pointer(1,VertexComponents::Four,DataType::F32,false,32,16 as *const _);
            VertexArrayFunctions::enable_attribute(1);
        }
    }
}

pub struct SimpleLayer{
    program:u32,
    draw_parameters:Matrix,
    parameters_location:i32,
    mesh:BufferedMesh<SimpleVertex,ElementIndexType>,
}

impl SimpleLayer{
    pub (crate) fn new(
        program:u32,
        parameters_location:i32,
        mesh:BufferedMesh<SimpleVertex,ElementIndexType>
    )->SimpleLayer{
        Self{
            program,
            draw_parameters:Matrix::new(),
            parameters_location,
            mesh
        }
    }
}

impl Layer for SimpleLayer{
    fn draw(&mut self){
        unsafe{
            ProgramFunctions::bind(self.program);
            Uniform::set_matrix_4_f32(self.parameters_location,1,false,self.draw_parameters.ptr());
        }
        self.mesh.vertex_array.bind();
        self.mesh.flush_vertices();
        self.mesh.flush_indices();

        self.mesh.draw();

        unsafe{
            VertexArrayFunctions::bind(0);
        }
    }
}

pub struct SimpleGraphicsAttributes{
    pub layers_limit:usize,
}

impl SimpleGraphicsAttributes{
    pub const fn new()->SimpleGraphicsAttributes{
        Self{
            layers_limit:10
        }
    }
}

pub struct SimpleGraphics{
    program:Program,
    layer_draw_parameters_location:i32,

    valid:Vec<u8>,
    layers:StaticStorage<SimpleLayer>
}

impl SimpleGraphics{
    pub (crate) fn new(attributes:&SimpleGraphicsAttributes)->SimpleGraphics{
        let vertex_shader=Shader::new(VERTEX_SHADER_SOURCE,ShaderType::VertexShader).unwrap();
        let fragment_shader=Shader::new(FRAGMENT_SHADER_SOURCE,ShaderType::FragmentShader).unwrap();

        let program=Program::new();
        program.attach_shader(&vertex_shader);
        program.attach_shader(&fragment_shader);
        program.link().unwrap();
        program.bind();

        program.bind_uniform_block("DrawParameters\0",0);

        let layer_draw_parameters_location=program.get_uniform_location("LayerDrawParameters\0").unwrap();

        Self{
            program,
            layer_draw_parameters_location,

            valid:vec![0u8;attributes.layers_limit],
            layers:StaticStorage::new(attributes.layers_limit),
        }
    }

    pub (crate) fn get_render_data(&mut self,layer:usize,object:usize)->Option<RenderData<SimpleVertex,ElementIndexType>>{
        if let Some(layer)=self.layers.get_mut(layer){
            let info=layer.mesh.get_render_data(object).unwrap();

            let vertex_start=info.vertex_buffer_start;
            let vertex_count=info.vertex_buffer_count;
    
            let index_start=info.index_buffer_start;
            let index_count=info.index_buffer_count;

            Some(
                RenderData::new(
                    &mut layer.mesh,
                    object,
                    vertex_start as usize,
                    vertex_count as usize,
                    index_start as usize,
                    index_count as usize
                )
            )
        }
        else{
            None
        }
    }

    pub fn create_layer(&mut self,attributes:MeshAttributes)->Option<usize>{
        self.layers.add(
            SimpleLayer::new(
                self.program.id(),
                self.layer_draw_parameters_location,
                BufferedMesh::new(attributes)
            )
        )
    }

    pub (crate) fn attach_layer(&mut self,layer:usize)->Option<&'static mut dyn Layer>{
        unsafe{
            if let Some(mesh)=self.layers.get_mut(layer){
                self.valid[layer]+=1;
                Some(std::mem::transmute(mesh as &mut dyn Layer))
            }
            else{
                None
            }
        }
    }

    /// Косвенно открепляет слой от рендеринга.
    /// 
    /// Используется только внутри движка: layer - всегда существует.
    pub (crate) fn detach_layer(&mut self,layer:usize){
        unsafe{
            self.layers.get(layer).unwrap();

            *self.valid.get_unchecked_mut(layer)-=1;
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

impl SimpleGraphics{
    pub (crate) fn push_object(
        &mut self,
        attributes:ObjectAttributes<SimpleVertex,u16>,
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