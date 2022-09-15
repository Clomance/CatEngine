use std::{marker::PhantomData, ops::{Deref, DerefMut}};

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
enum ObjectType{
    Simple,
    Textured,
    Text
}

struct ObjectData{
    ptr:*mut (),
    handle:fn(*mut (),ObjectEvent,*mut ()),
    drop:fn(*mut ()),
    layer:usize,
    object_id:usize,
    object_type:ObjectType,
}

pub struct ObjectRef<O>{
    ptr:*mut O,
}

impl<O> ObjectRef<O>{
    pub (crate) fn new(ptr:*mut O)->ObjectRef<O>{
        Self{
            ptr
        }
    }
}

impl<O> Deref for ObjectRef<O>{
    type Target=O;

    fn deref(&self)->&O{
        unsafe{&*self.ptr}
    }
}

impl<O> DerefMut for ObjectRef<O>{
    fn deref_mut(&mut self)->&mut O{
        unsafe{&mut *self.ptr}
    }
}

pub struct ObjectArray<O>{
    index:usize,
    objects:Vec<ObjectRef<O>>
}

impl<O> ObjectArray<O>{
    pub fn len(&self)->usize{
        self.objects.len()
    }

    pub fn get(&mut self,index:usize)->Option<&mut ObjectRef<O>>{
        self.objects.get_mut(index)
    }

    pub fn remove(&mut self,index:usize,object_manager:&mut ObjectManager){
        let object=object_manager.object_storage.data[self.index].remove(index);

        match object.object_type{
            ObjectType::Simple=>{
                object_manager.graphics.simple.remove_object(object.layer,object.object_id)
            }

            ObjectType::Textured=>{
                object_manager.graphics.textured.remove_object(object.layer,object.object_id)
            }

            ObjectType::Text=>{
                object_manager.graphics.text.remove_object(object.layer,object.object_id)
            }
        }

        (object.drop)(object.ptr);

        self.objects.remove(index);
    }
}

impl<O:SimpleObject> ObjectArray<O>{
    pub fn push_simple(
        &mut self,
        object:O,
        vertices:Vertices<SimpleVertex>,
        indices:Indices<ElementIndexType>,
        layer:usize,
        object_manager:&mut ObjectManager
    ){
        let attributes=ObjectAttributes::new(
            vertices.vertices,
            vertices.allocate,
            indices.indices,
            indices.allocate,
            indices.range
        );
        let object_id=object_manager.graphics.simple.push_object(attributes,layer).unwrap();

        let object=Box::new(object);

        let object_ptr=Box::leak(object);

        let data=ObjectData{
            ptr:object_ptr as *mut O as *mut (),
            handle:simple_object_handle_wrapper::<O>,
            drop:drop_wrapper::<O>,
            layer,
            object_id,
            object_type:ObjectType::Simple,
        };

        object_manager.object_storage.data[self.index].push(data);

        self.objects.push(ObjectRef::new(object_ptr));
    }
}

impl<O:TextureObject> ObjectArray<O>{
    pub fn push_textured(
        &mut self,
        object:O,
        vertices:Vertices<TexturedVertex>,
        indices:Indices<ElementIndexType>,
        layer:usize,
        object_manager:&mut ObjectManager
    ){
        let attributes=ObjectAttributes::new(
            vertices.vertices,
            vertices.allocate,
            indices.indices,
            indices.allocate,
            indices.range
        );
        let object_id=object_manager.graphics.textured.push_object(attributes,layer).unwrap();

        let object=Box::new(object);

        let object_ptr=Box::leak(object);

        let data=ObjectData{
            ptr:object_ptr as *mut O as *mut (),
            handle:texture_object_handle_wrapper::<O>,
            drop:drop_wrapper::<O>,
            layer,
            object_id,
            object_type:ObjectType::Textured,
        };

        object_manager.object_storage.data[self.index].push(data);

        self.objects.push(ObjectRef::new(object_ptr));
    }
}

impl<O:TextObject> ObjectArray<O>{
    pub fn push_text(
        &mut self,
        object:O,
        vertices:Vertices<TextVertex>,
        indices:Indices<ElementIndexType>,
        layer:usize,
        object_manager:&mut ObjectManager
    ){
        let attributes=ObjectAttributes::new(
            vertices.vertices,
            vertices.allocate,
            indices.indices,
            indices.allocate,
            indices.range
        );
        let object_id=object_manager.graphics.text.push_object(attributes,layer).unwrap();

        let object=Box::new(object);

        let object_ptr=Box::leak(object);

        let data=ObjectData{
            ptr:object_ptr as *mut O as *mut (),
            handle:text_object_handle_wrapper::<O>,
            drop:drop_wrapper::<O>,
            layer,
            object_id,
            object_type:ObjectType::Text,
        };

        object_manager.object_storage.data[self.index].push(data);

        self.objects.push(ObjectRef::new(object_ptr));
    }
}

pub struct ObjectStorage{
    references:*mut (),
    data:Vec<Vec<ObjectData>>
}

impl ObjectStorage{
    pub (crate) fn new()->ObjectStorage{
        Self{
            references:std::ptr::null_mut(),
            data:Vec::new(),
        }
    }

    pub (crate) fn set_references(&mut self,references:*mut ()){
        self.references=references
    }

    pub (crate) fn get_references(&mut self)->*mut (){
        self.references
    }

    pub fn push_simple_object<O:SimpleObject>(
        &mut self,
        object:O,
        layer:usize,
        object_id:usize,
    )->ObjectRef<O>{
        let object=Box::new(object);

        let object_ptr=Box::leak(object);

        let data=ObjectData{
            ptr:object_ptr as *mut O as *mut (),
            handle:simple_object_handle_wrapper::<O>,
            drop:drop_wrapper::<O>,
            layer,
            object_id,
            object_type:ObjectType::Simple,
        };

        self.data.push(vec![data]);

        ObjectRef{
            ptr:object_ptr
        }
    }

    pub fn push_textured_object<O:TextureObject>(
        &mut self,
        object:O,
        layer:usize,
        object_id:usize,
    )->ObjectRef<O>{
        let object=Box::new(object);

        let object_ptr=Box::leak(object);

        let data=ObjectData{
            ptr:object_ptr as *mut O as *mut (),
            handle:texture_object_handle_wrapper::<O>,
            drop:drop_wrapper::<O>,
            layer,
            object_id,
            object_type:ObjectType::Textured,
        };

        self.data.push(vec![data]);

        ObjectRef{
            ptr:object_ptr
        }
    }

    pub fn push_text_object<O:TextObject>(
        &mut self,
        object:O,
        layer:usize,
        object_id:usize,
    )->ObjectRef<O>{
        let object=Box::new(object);

        let object_ptr=Box::leak(object);

        let data=ObjectData{
            ptr:object_ptr as *mut O as *mut (),
            handle:text_object_handle_wrapper::<O>,
            drop:drop_wrapper::<O>,
            layer,
            object_id,
            object_type:ObjectType::Text,
        };

        self.data.push(vec![data]);

        ObjectRef{
            ptr:object_ptr
        }
    }

    pub fn push_simple_object_array<O:SimpleObject>(&mut self)->ObjectArray<O>{
        let index=self.data.len();
        self.data.push(Vec::new());

        ObjectArray{
            index,
            objects:Vec::new(),
        }
    }

    pub fn push_texture_object_array<O:TextureObject>(&mut self)->ObjectArray<O>{
        let index=self.data.len();
        self.data.push(Vec::new());

        ObjectArray{
            index,
            objects:Vec::new(),
        }
    }

    pub fn push_text_object_array<O:TextObject>(&mut self)->ObjectArray<O>{
        let index=self.data.len();
        self.data.push(Vec::new());

        ObjectArray{
            index,
            objects:Vec::new(),
        }
    }

    pub (crate) fn clear_storage(&mut self,graphics:&mut Graphics){
        while let Some(objects)=self.data.pop(){
            for object in objects{
                match object.object_type{
                    ObjectType::Simple=>{
                        graphics.simple.remove_object(object.layer,object.object_id);
                    }

                    ObjectType::Textured=>{
                        graphics.textured.remove_object(object.layer,object.object_id);
                    }

                    ObjectType::Text=>{
                        graphics.text.remove_object(object.layer,object.object_id);
                    }
                }
                (object.drop)(object.ptr)
            }
        }
    }
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
                        let texture=graphics.textured.get_layer_texture_raw(object.layer);
                        let render_data=graphics.textured.get_render_data(object.layer,object.object_id);

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

pub struct ObjectManager<'m>{
    object_storage:&'m mut ObjectStorage,
    graphics:&'m mut Graphics,
}

impl<'m> ObjectManager<'m>{
    pub (crate) fn new(
        object_storage:&'m mut ObjectStorage,
        graphics:&'m mut Graphics
    )->ObjectManager<'m>{
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
    )->Result<ObjectRef<O>,MeshError>{
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
    )->Result<ObjectRef<O>,MeshError>{
        let attributes=ObjectAttributes::new(
            vertices.vertices,
            vertices.allocate,
            indices.indices,
            indices.allocate,
            indices.range
        );
        match self.graphics.textured.push_object(attributes,layer){
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
    )->Result<ObjectRef<O>,MeshError>{
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


    pub fn push_simple_object_array<O:SimpleObject>(&mut self)->ObjectArray<O>{
        self.object_storage.push_simple_object_array()
    }

    pub fn push_texture_object_array<O:TextureObject>(&mut self)->ObjectArray<O>{
        self.object_storage.push_texture_object_array()
    }

    pub fn push_text_object_array<O:TextObject>(&mut self)->ObjectArray<O>{
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

fn simple_object_handle_wrapper<O:SimpleObject>(object:*mut (),event:ObjectEvent,render_data:*mut ()){
    unsafe{
        let object=&mut *(object as *mut O);

        let render_data=&mut *(render_data as *mut SimpleRenderData);

        object.event(event,render_data)
    }
}

fn texture_object_handle_wrapper<O:TextureObject>(object:*mut (),event:ObjectEvent,render_data:*mut ()){
    unsafe{
        let object=&mut *(object as *mut O);

        let render_data=&mut *(render_data as *mut TextureRenderData);

        object.event(event,render_data)
    }
}

fn text_object_handle_wrapper<O:TextObject>(object:*mut (),event:ObjectEvent,render_data:*mut ()){
    unsafe{
        let object=&mut *(object as *mut O);

        let render_data=&mut *(render_data as *mut TextRenderData);

        object.event(event,render_data)
    }
}

fn drop_wrapper<O>(object:*mut ()){
    unsafe{
        Box::from_raw(object as *mut O);
    }
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