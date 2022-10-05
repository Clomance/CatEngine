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

use super::{
    SimpleObject,
    Vertices,
    Indices,
    ObjectManager,
    TextObject,
    TextureObject,
    ObjectEvent,
    ObjectType
};

use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

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

        let layer=object.get_layer();
        let object_id=object.get_object_id();

        match object.object_type{
            ObjectType::Simple=>{
                object_manager.graphics.simple.remove_object(layer,object_id)
            }

            ObjectType::Textured=>{
                object_manager.graphics.texture.remove_object(layer,object_id)
            }

            ObjectType::Text=>{
                object_manager.graphics.text.remove_object(layer,object_id)
            }
        }

        (object.drop)(object.data);

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

        let table=Box::new(
            ObjectData{
                object,
                graphics:object_manager.graphics,
                layer,
                object_id
            }
        );

        let table_ptr=Box::leak(table);
        let object_ptr=&mut table_ptr.object;

        let data=ObjectTable{
            data:object_ptr as *mut O as *mut (),
            handle:simple_object_handle_wrapper::<O>,
            drop:drop_wrapper::<O>,
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

        let table=Box::new(
            ObjectData{
                object,
                graphics:object_manager.graphics,
                layer,
                object_id
            }
        );

        let table_ptr=Box::leak(table);
        let object_ptr=&mut table_ptr.object;

        let data=ObjectTable{
            data:object_ptr as *mut O as *mut (),
            handle:texture_object_handle_wrapper::<O>,
            drop:drop_wrapper::<O>,
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

        let table=Box::new(
            ObjectData{
                object,
                graphics:object_manager.graphics,
                layer,
                object_id
            }
        );

        let table_ptr=Box::leak(table);
        let object_ptr=&mut table_ptr.object;

        let data=ObjectTable{
            data:object_ptr as *mut O as *mut (),
            handle:text_object_handle_wrapper::<O>,
            drop:drop_wrapper::<O>,
            object_type:ObjectType::Text,
        };

        object_manager.object_storage.data[self.index].push(data);

        self.objects.push(ObjectReference::new(object_ptr));
    }
}

pub struct ObjectTable{
    pub data:*mut (),
    pub handle:fn(*mut (),ObjectEvent),
    pub drop:fn(*mut ()),
    pub object_type:ObjectType,
}

impl ObjectTable{
    pub fn get_layer(&self)->usize{
        unsafe{
            let table=&*(self.data.offset(-3) as *const ObjectData<Self>);

            table.layer
        }
    }

    pub fn get_object_id(&self)->usize{
        unsafe{
            let table=&*(self.data.offset(-3) as *const ObjectData<Self>);

            table.object_id
        }
    }

    pub fn handle(&self,event:ObjectEvent){
        (self.handle)(self.data,event)
    }

    pub fn drop(&self){
        (self.drop)(self.data)
    }
}

pub struct ObjectData<O>{
    pub graphics:*mut Graphics,
    pub layer:usize,
    pub object_id:usize,
    pub object:O,
}

pub struct ObjectStorage{
    pub data:Vec<Vec<ObjectTable>>
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
        graphics:&mut Graphics
    )->ObjectReference<'a,O>{
        let table=Box::new(
            ObjectData{
                object,
                graphics,
                layer,
                object_id
            }
        );

        let table_ptr=Box::leak(table);
        let object_ptr=&mut table_ptr.object;

        let data=ObjectTable{
            data:object_ptr as *mut O as *mut (),
            handle:simple_object_handle_wrapper::<O>,
            drop:drop_wrapper::<O>,
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
        graphics:&mut Graphics
    )->ObjectReference<'a,O>{
        let table=Box::new(
            ObjectData{
                object,
                graphics,
                layer,
                object_id
            }
        );

        let table_ptr=Box::leak(table);
        let object_ptr=&mut table_ptr.object;

        let data=ObjectTable{
            data:object_ptr as *mut O as *mut (),
            handle:texture_object_handle_wrapper::<O>,
            drop:drop_wrapper::<O>,
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
        graphics:&mut Graphics
    )->ObjectReference<'a,O>{
        let table=Box::new(
            ObjectData{
                object,
                graphics,
                layer,
                object_id
            }
        );

        let table_ptr=Box::leak(table);
        let object_ptr=&mut table_ptr.object;

        let data=ObjectTable{
            data:object_ptr as *mut O as *mut (),
            handle:text_object_handle_wrapper::<O>,
            drop:drop_wrapper::<O>,
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
                let layer=object.get_layer();
                let object_id=object.get_object_id();

                match object.object_type{
                    ObjectType::Simple=>{
                        graphics.simple.remove_object(layer,object_id);
                    }

                    ObjectType::Textured=>{
                        graphics.texture.remove_object(layer,object_id);
                    }

                    ObjectType::Text=>{
                        graphics.text.remove_object(layer,object_id);
                    }
                }
                object.drop()
            }
        }
    }
}

fn simple_object_handle_wrapper<O:SimpleObject>(object:*mut (),event:ObjectEvent){
    unsafe{
        let object=&mut *(object as *mut O);

        object.event(event)
    }
}

fn texture_object_handle_wrapper<O:TextureObject>(object:*mut (),event:ObjectEvent){
    unsafe{
        let object=&mut *(object as *mut O);

        object.event(event)
    }
}

fn text_object_handle_wrapper<O:TextObject>(object:*mut (),event:ObjectEvent){
    unsafe{
        let object=&mut *(object as *mut O);

        object.event(event)
    }
}

fn drop_wrapper<O>(object:*mut ()){
    unsafe{
        let table=object.offset(-3) as *mut ObjectData<O>;
        drop(Box::from_raw(table))
    }
}
