use crate::{
    text::{GlyphCacheUnitReference, GlyphCacheManager, Glyphs},
};

use super::{
    ElementIndexType,
    RenderData,
    Vertex,
    BufferedMesh,
    Layer,
    MeshAttributes,
    MeshError,
    ObjectAttributes,
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
    utility::{
        storage::StaticStorage,
        math::matrix::Matrix
    }
};

use std::{ptr::null_mut};

const VERTEX_SHADER_SOURCE:&'static str=include_str!("shaders/text/vertex.glsl");
const FRAGMENT_SHADER_SOURCE:&'static str=include_str!("shaders/text/fragment.glsl");

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

    glyph_cache:Option<GlyphCacheUnitReference>,
    mesh:BufferedMesh<TextVertex,ElementIndexType>,
}

impl TextLayer{
    pub fn new(
        program:u32,
        parameters_location:i32,
        glyph_cache:Option<GlyphCacheUnitReference>,
        mesh:BufferedMesh<TextVertex,ElementIndexType>
    )->TextLayer{
        Self{
            program,
            draw_parameters:Matrix::new(),
            parameters_location,
            glyph_cache,
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

        if let Some(glyph_cache)=&mut self.glyph_cache{
            glyph_cache.cache().bind()
        }

        self.mesh.draw();

        unsafe{
            VertexArrayFunctions::bind(0);
        }
    }
}



pub struct TextGraphicsManager<'m>{
    pub glyphs:GlyphCacheManager<'m>
}



pub struct TextGraphicsAttributes{
    pub fonts_limit:usize,
    pub layers_limit:usize,
}

impl TextGraphicsAttributes{
    pub const fn new()->TextGraphicsAttributes{
        Self{
            fonts_limit:10,
            layers_limit:10
        }
    }
}



pub struct TextGraphics{
    glyphs:Glyphs,

    program:Program,
    layer_draw_parameters_location:i32,

    valid:Vec<u8>,
    layers:StaticStorage<TextLayer>
}

impl TextGraphics{
    pub (crate) fn new(attributes:&TextGraphicsAttributes)->TextGraphics{
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
            glyphs:Glyphs::new(attributes.fonts_limit),

            program,
            layer_draw_parameters_location,

            valid:vec![0u8;attributes.layers_limit],
            layers:StaticStorage::new(attributes.layers_limit),
        }
    }

    pub fn manager(&mut self)->TextGraphicsManager{
        TextGraphicsManager{
            glyphs:self.glyphs.manager()
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
                None,
                BufferedMesh::new(attributes)
            )
        )
    }

    pub (crate) fn attach_layer(&mut self,layer:usize,font:usize)->Option<&'static mut dyn Layer>{
        unsafe{
            if let Some(mesh)=self.layers.get_mut(layer){
                if let Some(reference)=self.glyphs.get_reference(font){
                    mesh.glyph_cache=Some(reference);

                    self.valid[layer]+=1;

                    return Some(std::mem::transmute(mesh as &mut dyn Layer))
                }
            }

            None
        }
    }

    /// Косвенно открепляет слой от рендеринга и шрифта.
    /// 
    /// Используется только внутри движка: layer - всегда существует.
    pub (crate) fn detach_layer(&mut self,id:usize){
        unsafe{
            let layer=self.layers.get_unchecked_mut(id);

            layer.glyph_cache.take();

            *self.valid.get_unchecked_mut(id)-=1;
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

    pub fn get_layer_font(&mut self,layer:usize)->Option<&GlyphCacheUnitReference>{
        if let Some(layer)=self.layers.get(layer){
            layer.glyph_cache.as_ref()
        }
        else{
            None
        }
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