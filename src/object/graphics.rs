use crate::graphics::{
    SystemObjectManager,
    SimpleObject,
    MeshError,
    SimpleVertex,
    Vertices,
    Indices,
    ElementIndexType,
    SimpleGraphics,
    TextObject,
    TextGraphics,
    TextVertex,
    TextureObject,
    TexturedVertex,
    TextureGraphics,
};

use super::{
    ObjectReference,
    ArrayReference
};

use std::mem::transmute;



pub struct GraphicsObjectManager<'m>{
    manager:SystemObjectManager<'m>
}

impl<'m> GraphicsObjectManager<'m>{
    pub (crate) fn new(manager:SystemObjectManager)->GraphicsObjectManager<'m>{
        GraphicsObjectManager{
            manager:unsafe{transmute(manager)}
        }
    }
}

impl<'m> GraphicsObjectManager<'m>{
    pub fn push_simple_object<'a,O:SimpleObject>(
        &mut self,
        object:O,
        vertices:Vertices<SimpleVertex>,
        indices:Indices<ElementIndexType>,
        layer:usize,
        graphics:&mut SimpleGraphics
    )->Result<ObjectReference<'a,O>,MeshError>{
        unsafe{transmute(self.manager.push_simple_object(object,vertices,indices,layer,graphics))}
    }

    pub fn push_simple_array<'a,O:SimpleObject>(&mut self)->ArrayReference<'a,O>{
        unsafe{transmute(self.manager.push_simple_object_array::<O>())}
    }
}

impl<'m> GraphicsObjectManager<'m>{
    pub fn push_texture_object<'a,O:TextureObject>(
        &mut self,
        object:O,
        vertices:Vertices<TexturedVertex>,
        indices:Indices<ElementIndexType>,
        layer:usize,
        graphics:&mut TextureGraphics
    )->Result<ObjectReference<'a,O>,MeshError>{
        unsafe{transmute(self.manager.push_texture_object(object,vertices,indices,layer,graphics))}
    }

    pub fn push_texture_array<'a,O:TextureObject>(&mut self)->ArrayReference<'a,O>{
        unsafe{transmute(self.manager.push_texture_object_array::<O>())}
    }
}

impl<'m> GraphicsObjectManager<'m>{
    pub fn push_text_object<'a,O:TextObject>(
        &mut self,
        object:O,
        vertices:Vertices<TextVertex>,
        indices:Indices<ElementIndexType>,
        layer:usize,
        graphics:&mut TextGraphics
    )->Result<ObjectReference<'a,O>,MeshError>{
        unsafe{transmute(self.manager.push_text_object(object,vertices,indices,layer,graphics))}
    }

    pub fn push_text_array<'a,O:TextObject>(&mut self)->ArrayReference<'a,O>{
        unsafe{transmute(self.manager.push_text_object_array::<O>())}
    }
}