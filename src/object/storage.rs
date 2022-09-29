use crate::{
    graphics::{
        Graphics,
        SimpleVertex,
        TexturedVertex,
        TextVertex,
        ElementIndexType,
        ObjectAttributes,
    },
};

use std::{marker::PhantomData, ops::{Deref, DerefMut}};

use super::{SimpleObject, Vertices, Indices, ObjectManager, TextObject, TextureObject, ObjectEvent, TextureRenderData, SimpleRenderData, TextRenderData, ObjectType};

pub struct ObjectReference<'a,O>{
    ptr:*mut O,
    marker:PhantomData<&'a O>
}

impl<'a,O> ObjectReference<'a,O>{
    pub (crate) fn new(ptr:*mut O)->ObjectReference<'a,O>{
        Self{
            ptr,
            marker:PhantomData
        }
    }
}

impl<'a,O> Deref for ObjectReference<'a,O>{
    type Target=O;

    fn deref(&self)->&O{
        unsafe{&*self.ptr}
    }
}

impl<'a,O> DerefMut for ObjectReference<'a,O>{
    fn deref_mut(&mut self)->&mut O{
        unsafe{&mut *self.ptr}
    }
}

pub struct ObjectArray<'a,O>{
    index:usize,
    objects:Vec<ObjectReference<'a,O>>
}

impl<'a,O> ObjectArray<'a,O>{
    pub fn len(&self)->usize{
        self.objects.len()
    }

    pub fn get(&mut self,index:usize)->Option<&mut ObjectReference<'a,O>>{
        self.objects.get_mut(index)
    }

    pub fn remove(&mut self,index:usize,object_manager:&mut ObjectManager){
        let object=object_manager.object_storage.data[self.index].remove(index);

        match object.object_type{
            ObjectType::Simple=>{
                object_manager.graphics.simple.remove_object(object.layer,object.object_id)
            }

            ObjectType::Textured=>{
                object_manager.graphics.texture.remove_object(object.layer,object.object_id)
            }

            ObjectType::Text=>{
                object_manager.graphics.text.remove_object(object.layer,object.object_id)
            }
        }

        (object.drop)(object.ptr);

        self.objects.remove(index);
    }
}

impl<'a,O:SimpleObject> ObjectArray<'a,O>{
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

        self.objects.push(ObjectReference::new(object_ptr));
    }
}

impl<'a,O:TextureObject> ObjectArray<'a,O>{
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
        let object_id=object_manager.graphics.texture.push_object(attributes,layer).unwrap();

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

        self.objects.push(ObjectReference::new(object_ptr));
    }
}

impl<'a,O:TextObject> ObjectArray<'a,O>{
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

        self.objects.push(ObjectReference::new(object_ptr));
    }
}

pub struct ObjectData{
    pub ptr:*mut (),
    pub handle:fn(*mut (),ObjectEvent,*mut ()),
    pub drop:fn(*mut ()),
    pub layer:usize,
    pub object_id:usize,
    pub object_type:ObjectType,
}

pub struct ObjectStorage{
    pub data:Vec<Vec<ObjectData>>
}

impl ObjectStorage{
    pub (crate) fn new()->ObjectStorage{
        Self{
            data:Vec::new(),
        }
    }

    pub fn push_simple_object<'a,O:SimpleObject>(
        &mut self,
        object:O,
        layer:usize,
        object_id:usize,
    )->ObjectReference<'a,O>{
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

        ObjectReference::new(object_ptr)
    }

    pub fn push_textured_object<'a,O:TextureObject>(
        &mut self,
        object:O,
        layer:usize,
        object_id:usize,
    )->ObjectReference<'a,O>{
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

        ObjectReference::new(object_ptr)
    }

    pub fn push_text_object<'a,O:TextObject>(
        &mut self,
        object:O,
        layer:usize,
        object_id:usize,
    )->ObjectReference<'a,O>{
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

        ObjectReference::new(object_ptr)
    }

    pub fn push_simple_object_array<'a,O:SimpleObject>(&mut self)->ObjectArray<'a,O>{
        let index=self.data.len();
        self.data.push(Vec::new());

        ObjectArray{
            index,
            objects:Vec::new(),
        }
    }

    pub fn push_texture_object_array<'a,O:TextureObject>(&mut self)->ObjectArray<'a,O>{
        let index=self.data.len();
        self.data.push(Vec::new());

        ObjectArray{
            index,
            objects:Vec::new(),
        }
    }

    pub fn push_text_object_array<'a,O:TextObject>(&mut self)->ObjectArray<'a,O>{
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
                        graphics.texture.remove_object(object.layer,object.object_id);
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
