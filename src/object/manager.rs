use super::{ObjectStorage, SimpleObject, ObjectArray, TextObject, ObjectReference, Vertices, Indices, TextureObject};

use crate::{
    graphics::{
        Graphics,
        SimpleVertex,
        TexturedVertex,
        TextVertex,
        MeshError,
        ElementIndexType,
        ObjectAttributes,
    },
};

pub struct ObjectManager<'a>{
    pub (crate) object_storage:&'a mut ObjectStorage,
    pub (crate) graphics:&'a mut Graphics,
}

impl<'a> ObjectManager<'a>{
    pub (crate) fn new(
        object_storage:&'a mut ObjectStorage,
        graphics:&'a mut Graphics
    )->ObjectManager<'a>{
        Self{
            object_storage,
            graphics
        }
    }

    pub fn push_simple_object<O:SimpleObject>(
        &mut self,
        object:O,
        vertices:Vertices<SimpleVertex>,
        indices:Indices<ElementIndexType>,
        layer:usize
    )->Result<ObjectReference<'a,O>,MeshError>{
        let attributes=ObjectAttributes::new(
            vertices.vertices,
            vertices.allocate,
            indices.indices,
            indices.allocate,
            indices.range
        );
        match self.graphics.simple.push_object(attributes,layer){
            Ok(object_id)=>Ok(self.object_storage.push_simple_object(object,layer,object_id,self.graphics)),
            Err(e)=>Err(e)
        }
    }

    pub fn push_texture_object<O:TextureObject>(
        &mut self,
        object:O,
        vertices:Vertices<TexturedVertex>,
        indices:Indices<ElementIndexType>,
        layer:usize
    )->Result<ObjectReference<'a,O>,MeshError>{
        let attributes=ObjectAttributes::new(
            vertices.vertices,
            vertices.allocate,
            indices.indices,
            indices.allocate,
            indices.range
        );
        match self.graphics.texture.push_object(attributes,layer){
            Ok(object_id)=>Ok(self.object_storage.push_textured_object(object,layer,object_id,self.graphics)),
            Err(e)=>Err(e)
        }
    }

    pub fn push_text_object<O:TextObject>(
        &mut self,
        object:O,
        vertices:Vertices<TextVertex>,
        indices:Indices<ElementIndexType>,
        layer:usize
    )->Result<ObjectReference<'a,O>,MeshError>{
        let attributes=ObjectAttributes::new(
            vertices.vertices,
            vertices.allocate,
            indices.indices,
            indices.allocate,
            indices.range
        );
        match self.graphics.text.push_object(attributes,layer){
            Ok(object_id)=>Ok(self.object_storage.push_text_object(object,layer,object_id,self.graphics)),
            Err(e)=>Err(e)
        }
    }

    pub fn push_simple_object_array<O:SimpleObject>(&mut self)->ObjectArray<'a,O>{
        self.object_storage.push_simple_object_array()
    }

    pub fn push_texture_object_array<O:TextureObject>(&mut self)->ObjectArray<'a,O>{
        self.object_storage.push_texture_object_array()
    }

    pub fn push_text_object_array<O:TextObject>(&mut self)->ObjectArray<'a,O>{
        self.object_storage.push_text_object_array()
    }

    pub fn graphics(&mut self)->&mut Graphics{
        self.graphics
    }
}