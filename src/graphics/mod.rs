mod mesh;
pub (crate) use mesh::{
    BufferedMesh,
    ObjectAttributes
};

pub use mesh::{
    Vertex,
    MeshError,
    RenderData,
    MeshAttributes,
};

mod camera;
pub use camera::Camera;

mod simple;
pub use simple::{
    SimpleVertex,
    SimpleGraphics,
    SimpleGraphicsAttributes,
};

mod text;
pub use text::{
    TextVertex,
    TextGraphics,
    TextGraphicsAttributes
};

mod textured;
pub use textured::{
    TexturedVertex,
    TextureGraphics,
    TextureGraphicsAttributes
};

use cat_engine_basement::{
    opengl::{
        core::{
            Core as GLCore,
            CoreCapability,

            blend::{
                Blend,
                BlendingFunction,
            },
            ClearMask,
        },
    },
    winapi::{
        OpenGraphicsLibrary,
        OpenGLRenderContext,
    },
};

pub use cat_engine_basement::opengl::{
    Colour,

    core::{
        drawing::PrimitiveType,
    }
};

pub type ElementIndexType=u16;

pub (crate) trait Layer{
    fn draw(&mut self);
}

pub (crate) enum LayerType{
    Simple,
    Textured,
    Text,
}

pub (crate) struct LayerInfo{
    layer_type:LayerType,
    index:usize,
}

pub (crate) struct Layers{
    layer_info:Vec<LayerInfo>,
    layers:Vec<&'static mut dyn Layer>
}

impl Layers{
    pub fn new()->Layers{
        Self{
            layer_info:Vec::new(),
            layers:Vec::new(),
        }
    }

    pub fn len(&self)->usize{
        self.layers.len()
    }

    pub fn push(&mut self,layer:&'static mut dyn Layer,info:LayerInfo){
        self.layer_info.push(info);
        self.layers.push(layer);
    }

    pub fn insert(&mut self,location:usize,layer:&mut dyn Layer,info:LayerInfo){
        self.layer_info.insert(location,info);
        self.layers.insert(location,unsafe{std::mem::transmute(layer)});
    }

    pub fn remove(&mut self,layer:usize)->LayerInfo{
        self.layers.remove(layer);
        self.layer_info.remove(layer)
    }

    pub fn pop(&mut self)->LayerInfo{
        self.layers.pop();
        self.layer_info.pop().unwrap()
    }

    pub fn layers(&mut self)->&mut Vec<&'static mut dyn Layer>{
        &mut self.layers
    }
}



pub struct GraphicsParameters{
    pub (crate) clear_mask:ClearMask,
}

impl GraphicsParameters{
    pub (crate) fn new()->GraphicsParameters{
        Self{
            clear_mask:ClearMask::None
        }
    }

    pub fn set_clear_colour(&mut self,colour:Option<Colour>){
        if let Some(colour)=colour{
            unsafe{
                GLCore::set_clear_colour(colour[0],colour[1],colour[2],colour[3])
            }

            self.clear_mask=self.clear_mask.add(ClearMask::Colour);
        }
        else{
            self.clear_mask=self.clear_mask.remove(ClearMask::Colour);
        }
    }
}



pub struct GraphicsAttributes{
    pub simple:SimpleGraphicsAttributes,
    pub texture:TextureGraphicsAttributes,
    pub text:TextGraphicsAttributes,
}

impl GraphicsAttributes{
    pub fn new()->GraphicsAttributes{
        Self{
            simple:SimpleGraphicsAttributes::new(),
            texture:TextureGraphicsAttributes::new(),
            text:TextGraphicsAttributes::new(),
        }
    }
}




pub struct Graphics{
    pub (crate) render_context:OpenGLRenderContext,

    pub camera:Camera,

    pub simple:SimpleGraphics,
    pub texture:TextureGraphics,
    pub text:TextGraphics,

    pub parameters:GraphicsParameters,

    pub (crate) layers:Layers,
}

impl Graphics{
    pub (crate) fn new(
        viewport:[f32;2],
        view_space_size:[f32;3],
        attribites:&GraphicsAttributes,
        render_context:OpenGLRenderContext,
        module:OpenGraphicsLibrary
    )->Graphics{
        unsafe{
            GLCore::load_functions(&module);

            GLCore::enable(CoreCapability::Blend);
            Blend::set_function(
                BlendingFunction::SourceAlpha,
                BlendingFunction::OneMinusSourceAlpha
            );

            // GLCore::enable(CoreCapability::DepthTest);
            // GLCore::set_clear_depth(0f64);
            // Depth::set_function(DepthFunction::GreaterEqual);
            // Depth::set_mask(true);
        }

        let simple=SimpleGraphics::new(&attribites.simple);

        let texture=TextureGraphics::new(&attribites.texture);

        let text=TextGraphics::new(&attribites.text);

        Self{
            render_context,

            camera:Camera::new(viewport,view_space_size),

            simple,
            texture,
            text,

            parameters:GraphicsParameters::new(),

            layers:Layers::new(),
        }
    }

    pub (crate) fn draw(&mut self){
        for layer in self.layers.layers(){
            layer.draw();
        }
    }
}

impl Graphics{
    pub fn layer_len(&self)->usize{
        self.layers.len()
    }

    pub fn push_simple_layer(&mut self,layer:usize)->bool{
        if let Some(mesh)=self.simple.attach_layer(layer){
            let info=LayerInfo{
                layer_type:LayerType::Simple,
                index:layer,
            };
            self.layers.push(mesh,info);

            true
        }
        else{
            false
        }
    }

    pub fn insert_simple_layer(&mut self,location:usize,layer:usize)->bool{
        if let Some(mesh)=self.simple.attach_layer(layer){
            let info=LayerInfo{
                layer_type:LayerType::Simple,
                index:layer,
            };
            self.layers.insert(location,mesh,info);

            true
        }
        else{
            false
        }
    }

    pub fn push_text_layer(&mut self,layer:usize,font:usize)->bool{
        if let Some(mesh)=self.text.attach_layer(layer,font){
            let info=LayerInfo{
                layer_type:LayerType::Text,
                index:layer,
            };
            self.layers.push(mesh,info);

            true
        }
        else{
            false
        }
    }

    pub fn insert_text_layer(&mut self,location:usize,layer:usize,font:usize)->bool{
        if let Some(mesh)=self.text.attach_layer(layer,font){
            let info=LayerInfo{
                layer_type:LayerType::Text,
                index:layer,
            };
            self.layers.insert(location,mesh,info);

            true
        }
        else{
            false
        }
    }

    pub fn push_texture_layer(&mut self,layer:usize,texture:usize)->bool{
        if let Some(mesh)=self.texture.attach_layer(layer,texture){
            let info=LayerInfo{
                layer_type:LayerType::Textured,
                index:layer,
            };
            self.layers.push(mesh,info);

            true
        }
        else{
            false
        }
    }

    pub fn insert_texture_layer(&mut self,location:usize,layer:usize,texture:usize)->bool{
        if let Some(mesh)=self.texture.attach_layer(layer,texture){
            let info=LayerInfo{
                layer_type:LayerType::Textured,
                index:layer,
            };
            self.layers.insert(location,mesh,info);

            true
        }
        else{
            false
        }
    }

    pub fn remove_layer(&mut self,layer:usize){
        let info=self.layers.remove(layer);

        match info.layer_type{
            LayerType::Simple=>{
                self.simple.detach_layer(info.index)
            }

            LayerType::Textured=>{
                self.texture.detach_layer(info.index)
            }

            LayerType::Text=>{
                self.text.detach_layer(info.index)
            }
        }
    }

    pub fn pop_layer(&mut self){
        let info=self.layers.pop();

        match info.layer_type{
            LayerType::Simple=>{
                self.simple.detach_layer(info.index)
            }

            LayerType::Textured=>{
                self.texture.detach_layer(info.index)
            }

            LayerType::Text=>{
                self.text.detach_layer(info.index)
            }
        }
    }
}