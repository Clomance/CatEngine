mod storage;
pub (crate) use storage::ObjectStorage;

use cat_engine_basement::{
    support::storage::DynamicStorage
};

use crate::{
    graphics::{
        Graphics,
        RenderData,
        SimpleVertex,
        TexturedVertex,
        TextVertex,
        MeshError,
        ElementIndexType,
        ObjectAttributes,
    },
    text::GlyphCache, texture::Texture2D
};

pub use self::storage::{ObjectReference, ObjectArray};

pub trait SimpleObject{
    fn event(&mut self,event:ObjectEvent,render:&mut SimpleRenderData);
}

pub trait TextureObject{
    fn event(&mut self,event:ObjectEvent,render:&mut TextureRenderData);
}

pub trait TextObject{
    fn event(&mut self,event:ObjectEvent,render:&mut TextRenderData);
}

#[derive(Clone,Copy)]
pub enum ObjectEvent{
    Update,
    Prerender
}

#[derive(Debug)]
pub enum ObjectType{
    Simple,
    Textured,
    Text
}

pub (crate) struct Objects{
    objects:DynamicStorage<ObjectStorage>,
}

impl Objects{
    pub fn new()->Objects{
        Self{
            objects:DynamicStorage::new(),
        }
    }

    pub fn create_storage(&mut self)->usize{
        self.objects.add(ObjectStorage::new())
    }

    pub fn get_storage(&mut self,storage:usize)->&mut ObjectStorage{
        self.objects.get_mut(storage).unwrap()
    }

    pub fn remove_storage(&mut self,storage:usize,graphics:&mut Graphics){
        let mut storage=self.objects.remove(storage).unwrap();

        storage.clear_storage(graphics)
    }

    pub fn handle(&mut self,storage:usize,event:ObjectEvent,graphics:&mut Graphics){
        for objects in &mut self.objects.get_mut(storage).unwrap().data{
            for object in objects{
                match object.object_type{
                    ObjectType::Simple=>unsafe{
                        let render_data=graphics.simple.get_render_data(object.layer,object.object_id);
                        let mut simple_render_data=SimpleRenderData{
                            render:render_data
                        };
                        (object.handle)(object.ptr,event,std::mem::transmute(&mut simple_render_data))
                    }

                    ObjectType::Textured=>unsafe{
                        let texture=graphics.texture.get_layer_texture_raw(object.layer);
                        let render_data=graphics.texture.get_render_data(object.layer,object.object_id);

                        let mut texture_render_data=TextureRenderData{
                            render:render_data,
                            texture:&*texture,
                        };

                        (object.handle)(object.ptr,event,std::mem::transmute(&mut texture_render_data))
                    }

                    ObjectType::Text=>unsafe{
                        let glyph_cache=graphics.text.get_layer_font_raw(object.layer);
                        let render_data=graphics.text.get_render_data(object.layer,object.object_id);
    
                        let mut tex_render_data=TextRenderData{
                            render:render_data,
                            glyph_cache:&*glyph_cache,
                        };

                        (object.handle)(object.ptr,event,std::mem::transmute(&mut tex_render_data))
                    }
                }
            }
        }
    }
}

pub struct ObjectManager<'a>{
    object_storage:&'a mut ObjectStorage,
    graphics:&'a mut Graphics,
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
            Ok(object_id)=>Ok(self.object_storage.push_simple_object(object,layer,object_id)),
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
            Ok(object_id)=>Ok(self.object_storage.push_textured_object(object,layer,object_id)),
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
            Ok(object_id)=>Ok(self.object_storage.push_text_object(object,layer,object_id)),
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

pub struct SimpleRenderData<'a>{
    pub render:RenderData<'a,SimpleVertex,ElementIndexType>,
}

pub struct TextureRenderData<'a>{
    pub render:RenderData<'a,TexturedVertex,ElementIndexType>,
    pub texture:&'a Texture2D,
}

pub struct TextRenderData<'a>{
    pub render:RenderData<'a,TextVertex,ElementIndexType>,
    pub glyph_cache:&'a GlyphCache,
}


/// Note that if you convert from `Vec<V>`,
/// it's capacity attribute is used to specify the size of memory space for vertices.
pub struct Vertices<'v,V>{
    vertices:&'v [V],
    allocate:usize,
}

impl<'v,V> Vertices<'v,V>{
    pub const fn new(vertices:&'v [V])->Vertices<'v,V>{
        Self{
            vertices,
            allocate:vertices.len()
        }
    }

    pub const fn empty(allocate:usize)->Vertices<'v,V>{
        Self{
            vertices:&[],
            allocate,
        }
    }

    pub const fn raw(vertices:&'v [V],allocate:usize)->Vertices<'v,V>{
        Self{
            vertices,
            allocate
        }
    }
}

impl<'v,V> From<&'v [V]> for Vertices<'v,V>{
    fn from(from:&'v [V])->Vertices<'v,V>{
        Self{
            vertices:from,
            allocate:from.len(),
        }
    }
}

impl<'v,V> From<&'v Vec<V>> for Vertices<'v,V>{
    fn from(from:&'v Vec<V>)->Vertices<'v,V>{
        Self{
            vertices:from.as_slice(),
            allocate:from.capacity(),
        }
    }
}

/// Note that if you convert from `Vec<I>`,
/// it's capacity attribute is used to specify the size of memory space for indices.
pub struct Indices<'i,I>{
    indices:&'i [I],
    allocate:usize,
    range:[usize;2]
}

impl<'i,I> Indices<'i,I>{
    pub const fn new(indices:&'i [I])->Indices<'i,I>{
        Self{
            indices,
            allocate:indices.len(),
            range:[0,indices.len()]
        }
    }

    pub const fn empty(allocate:usize)->Indices<'i,I>{
        Self{
            indices:&[],
            allocate,
            range:[0,0]
        }
    }

    pub const fn none()->Indices<'i,I>{
        Self{
            indices:&[],
            allocate:0,
            range:[0,0]
        }
    }

    pub const fn raw(indices:&'i [I],allocate:usize,range:[usize;2])->Indices<'i,I>{
        Self{
            indices,
            allocate,
            range
        }
    }
}

impl<'i,I> From<&'i [I]> for Indices<'i,I>{
    fn from(from:&'i [I])->Indices<'i,I>{
        Self{
            indices:from,
            allocate:from.len(),
            range:[0,from.len()]
        }
    }
}

impl<'i,I> From<&'i Vec<I>> for Indices<'i,I>{
    fn from(from:&'i Vec<I>)->Indices<'i,I>{
        Self{
            indices:from.as_slice(),
            allocate:from.capacity(),
            range:[0,from.len()]
        }
    }
}