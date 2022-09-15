use crate::{
    text::GlyphCache,
    object::TextObject
};

use super::{
    ElementIndexType,
    RenderData,
    Vertex,
    BufferedMesh,
    Layer,
    MeshAttributes,
    MeshError, ObjectAttributes,
};

use cat_engine_basement::{
    opengl::{
        core::{
            program::{
                Program as ProgramFunctions,
            },
            vertex_array::{
                VertexArray as VertexArrayFunctions,
                VertexComponents,
                DataType,
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
    support::{
        storage::StaticStorage,
        math::matrix::Matrix
    },
};

use std::ptr::null_mut;

const vertex_shader_source:&'static str=include_str!("shaders/text/vertex.glsl");
const fragment_shader_source:&'static str=include_str!("shaders/text/fragment.glsl");

#[derive(Clone,Copy,Debug)]
pub struct TextVertex{
    pub position:[f32;4],
    pub colour:[f32;4],
    pub texture_coordinates:[f32;3],
}

impl TextVertex{
    pub fn new(position:[f32;4],colour:[f32;4],texture_coordinates:[f32;3])->TextVertex{
        Self{
            position,
            colour,
            texture_coordinates,
        }
    }
}

impl Default for TextVertex{
    fn default()->TextVertex{
        Self{
            position:[0f32;4],
            colour:[1f32;4],
            texture_coordinates:[0f32;3],
        }
    }
}


impl Vertex for TextVertex{
    fn attribute_pointer(){
        unsafe{
            VertexArrayFunctions::attribute_pointer(0,VertexComponents::Four,DataType::F32,false,44,null_mut());
            VertexArrayFunctions::enable_attribute(0);
            VertexArrayFunctions::attribute_pointer(1,VertexComponents::Four,DataType::F32,false,44,16 as *const _);
            VertexArrayFunctions::enable_attribute(1);
            VertexArrayFunctions::attribute_pointer(2,VertexComponents::Three,DataType::F32,false,44,32 as *const _);
            VertexArrayFunctions::enable_attribute(2);
        }
    }
}

struct TextLayer{
    program:u32,
    draw_parameters:Matrix,
    parameters_location:i32,

    font:*mut GlyphCache,
    mesh:BufferedMesh<TextVertex,ElementIndexType>,
}

impl TextLayer{
    pub fn new(
        program:u32,
        parameters_location:i32,
        font:*mut GlyphCache,
        mesh:BufferedMesh<TextVertex,ElementIndexType>
    )->TextLayer{
        Self{
            program,
            draw_parameters:Matrix::new(),
            parameters_location,
            font,
            mesh
        }
    }
}

impl Layer for TextLayer{
    fn draw(&mut self){
        unsafe{
            ProgramFunctions::bind(self.program);
            Uniform::set_matrix_4_f32(self.parameters_location,1,false,self.draw_parameters.ptr());
        }
        self.mesh.vertex_array.bind();
        self.mesh.flush_vertices();
        self.mesh.flush_indices();

        unsafe{&*self.font}.bind();

        self.mesh.draw();

        unsafe{
            VertexArrayFunctions::bind(0);
        }
    }
}

pub struct TextGraphicsAttributes{
    pub font_limit:usize,
    pub layer_limit:usize,
}

impl TextGraphicsAttributes{
    pub const fn new()->TextGraphicsAttributes{
        Self{
            font_limit:10,
            layer_limit:10
        }
    }
}

pub struct TextGraphics{
    font_usage:Vec<u8>,
    font_storage:StaticStorage<GlyphCache>,

    program:Program,
    layer_draw_parameters_location:i32,

    valid:Vec<u8>,
    font_attached:Vec<usize>,
    layers:StaticStorage<TextLayer>
}

impl TextGraphics{
    pub (crate) fn new(attributes:&TextGraphicsAttributes)->TextGraphics{
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
            font_usage:vec![0u8;attributes.font_limit],
            font_storage:StaticStorage::new(attributes.font_limit),

            program,
            layer_draw_parameters_location,

            valid:vec![0u8;attributes.layer_limit],
            font_attached:vec![0usize;attributes.layer_limit],
            layers:StaticStorage::new(attributes.layer_limit),
        }
    }

    pub (crate) fn get_render_data(&mut self,layer:usize,object:usize)->RenderData<TextVertex,ElementIndexType>{
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
            TextLayer::new(
                self.program.id(),
                self.layer_draw_parameters_location,
                null_mut(),
                BufferedMesh::new(attributes)
            )
        )
    }

    pub (crate) fn attach_layer(&mut self,layer:usize,font:usize)->Option<&'static mut dyn Layer>{
        unsafe{
            if let Some(mesh)=self.layers.get_mut(layer){
                if let Some(glyph_cache)=self.font_storage.get_mut(font){
                    self.font_usage[font]+=1;
                    mesh.font=glyph_cache;

                    self.valid[layer]+=1;
                    self.font_attached[layer]=font;

                    return Some(std::mem::transmute(mesh as &mut dyn Layer))
                }
            }

            None
        }
    }

    /// Косвенно открепляет слой от рендеринга и шрифта.
    /// 
    /// Используется только внутри движка: layer - всегда существует.
    pub (crate) fn detach_layer(&mut self,layer:usize){
        unsafe{
            let font=self.font_attached.get_unchecked(layer).clone();

            *self.font_usage.get_unchecked_mut(font)-=1;

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



impl TextGraphics{
    pub fn push_font(&mut self,font:GlyphCache)->Option<usize>{
        self.font_storage.add(font)
    }

    pub fn get_font(&mut self,font:usize)->Option<&mut GlyphCache>{
        self.font_storage.get_mut(font)
    }

    pub fn get_layer_font(&mut self,layer:usize)->Option<&mut GlyphCache>{
        if let Some(mesh)=self.layers.get(layer){
            Some(unsafe{&mut *mesh.font})
        }
        else{
            None
        }
    }

    pub (crate) fn get_layer_font_raw(&mut self,layer:usize)->*mut GlyphCache{
        self.layers.get(layer).unwrap().font
    }

    pub fn remove_font(&mut self,font:usize)->Option<GlyphCache>{
        if let Some(&usage)=self.font_usage.get(font){
            if usage==0{
                return self.font_storage.remove(font)
            }
        }
        None
    }
}



impl TextGraphics{
    pub (crate) fn push_object(&mut self,attributes:ObjectAttributes<TextVertex,u16>,layer:usize)->Result<usize,MeshError>{
        if let Some(layer)=self.layers.get_mut(layer){
            return layer.mesh.push_object(attributes)
        }

        panic!()
    }

    pub (crate) fn remove_object(&mut self,layer:usize,object:usize){
        if let Some(layer)=self.layers.get_mut(layer){
            layer.mesh.remove_object(object)
        }
    }
}