mod layer;
use layer::{
    Layer,
    Layers,
    LayerType,
    LayerInfo,
};

mod object;
pub use object::{
    ObjectReference,
    ArrayReference,
    ObjectEvent,
    SystemObjectManager,
    TextObject,
    SimpleObject,
    TextureObject,
    Vertices,
    Indices,
    TextRenderDataInterface,
    SimpleRenderDataInterface,
    TextureRenderDataInterface,
};
pub (crate) use object::{
    Objects,
    SystemObjectStorage
};

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



pub struct GraphicsCoreManager<'m>{
    pub camera:&'m mut Camera,
    pub parameters:&'m mut GraphicsParameters,

    pub simple:&'m mut SimpleGraphics,
    pub texture:&'m mut TextureGraphics,
    pub text:&'m mut TextGraphics,

    pub layers:&'m mut Layers,
    pub objects:&'m mut Objects
}



pub struct GraphicsManager<'m>{
    pub camera:&'m mut Camera,
    pub parameters:&'m mut GraphicsParameters,

    pub simple:&'m mut SimpleGraphics,
    pub texture:&'m mut TextureGraphics,
    pub text:&'m mut TextGraphics,

    pub (crate) layers:&'m mut Layers,
}

impl<'m> GraphicsManager<'m>{
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



pub struct GraphicsCore{
    pub (crate) render_context:OpenGLRenderContext,

    pub camera:Camera,

    pub simple:SimpleGraphics,
    pub texture:TextureGraphics,
    pub text:TextGraphics,

    pub parameters:GraphicsParameters,

    pub (crate) layers:Layers,

    pub (crate) objects:Objects,
}

impl GraphicsCore{
    pub (crate) fn new(
        viewport:[f32;2],
        view_space_size:[f32;3],
        attribites:&GraphicsAttributes,
        render_context:OpenGLRenderContext,
        module:OpenGraphicsLibrary
    )->GraphicsCore{
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

            objects:Objects::new()
        }
    }

    pub fn manager(&mut self)->GraphicsCoreManager{
        GraphicsCoreManager{
            camera:&mut self.camera,
            parameters:&mut self.parameters,
            simple:&mut self.simple,
            texture:&mut self.texture,
            text:&mut self.text,
            layers:&mut self.layers,
            objects:&mut self.objects,
        }
    }

    pub (crate) fn draw(&mut self){
        for layer in self.layers.layers(){
            layer.draw();
        }
    }
}