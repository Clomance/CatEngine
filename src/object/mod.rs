use crate::graphics::{
    Graphics,
    ElementIndexType,
    MeshError,
    ObjectAttributes,
    SimpleVertex,
    TexturedVertex,
    TextVertex
};

mod interface;

use interface::{
    drop_simple_object,
    drop_texture_object,
    drop_text_object,

    SimpleWrapper,
    TextureWrapper,
    TextWrapper,

    ObjectInterface,
    RenderDataInterface,
};

pub use interface::{
    SimpleObject,
    TextureObject,
    TextObject,

    SimpleRenderDataInterface,
    TextureRenderDataInterface,
    TextRenderDataInterface,
};

use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    mem::transmute
};



#[derive(Debug,Clone,Copy)]
pub enum ObjectEvent{
    Update,
    Prerender
}



/// Данные и функции объекта.
pub struct ObjectTable{
    object_data:Box<dyn ObjectInterface>,
    render_data:RenderDataInterface,
    drop:fn(render_data:RenderDataInterface)
}

impl ObjectTable{
    pub fn simple<O:SimpleObject>(object:O,render_data:RenderDataInterface)->ObjectTable{
        let boxed_object:Box<dyn ObjectInterface>=Box::new(SimpleWrapper(object));

        Self{
            object_data:boxed_object,
            render_data,
            drop:drop_simple_object
        }
    }

    pub fn texture<O:TextureObject>(object:O,render_data:RenderDataInterface)->ObjectTable{
        let boxed_object:Box<dyn ObjectInterface>=Box::new(TextureWrapper(object));

        Self{
            object_data:boxed_object,
            render_data,
            drop:drop_texture_object
        }
    }

    pub fn text<O:TextObject>(object:O,render_data:RenderDataInterface)->ObjectTable{
        let boxed_object:Box<dyn ObjectInterface>=Box::new(TextWrapper(object));

        Self{
            object_data:boxed_object,
            render_data,
            drop:drop_text_object
        }
    }

    pub fn object_data(&mut self)->*mut (){
        unsafe{
            let (data,_):(*mut (),*mut ())=transmute(self.object_data.as_mut());
            data
        }
    }

    pub fn event(&mut self,event:ObjectEvent){
        self.object_data.event(event,self.render_data.clone())
    }

    pub fn drop(&mut self){
        (self.drop)(self.render_data.clone())
    }
}


/// Представление одного типа объекта.
pub struct ObjectUnit{
    objects:Vec<ObjectTable>,
}

impl ObjectUnit{
    pub fn single(object_table:ObjectTable)->ObjectUnit{
        let mut objects=Vec::with_capacity(1);
        objects.push(object_table);

        Self{
            objects
        }
    }

    pub fn array(capacity:usize)->ObjectUnit{
        Self{
            objects:Vec::with_capacity(capacity)
        }
    }

    pub fn len(&self)->usize{
        self.objects.len()
    }

    pub fn push(&mut self,object:ObjectTable){
        self.objects.push(object)
    }

    pub fn remove(&mut self,index:usize)->ObjectTable{
        self.objects.remove(index)
    }
}



/// Хранилище объектов системы.
pub struct SystemObjectStorage{
    units:Vec<ObjectUnit>,
}

impl SystemObjectStorage{
    pub fn new()->SystemObjectStorage{
        Self{
            units:Vec::new()
        }
    }

    pub fn len(&self)->usize{
        self.units.len()
    }

    pub fn push(&mut self,unit:ObjectUnit){
        self.units.push(unit)
    }

    pub fn get_unit(&mut self,index:usize)->&mut ObjectUnit{
        &mut self.units[index]
    }
}



/// Массив хранилищ объектов.
pub struct Objects{
    /// Ссылки на данные системы, чтобы быстро менять местами хранилища.
    /// 
    /// см. функцию remove
    systems:Vec<*mut usize>,

    /// Массив хранилищ объектов.
    /// 
    /// Системы ссылаются на этих хранилища.
    storages:Vec<SystemObjectStorage>,
}

impl Objects{
    pub fn new()->Objects{
        Self{
            systems:Vec::new(),
            storages:Vec::new()
        }
    }

    pub fn len(&self)->usize{
        self.systems.len()
    }

    pub fn push(&mut self,system_storage_id_ptr:*mut usize,storage:SystemObjectStorage){
        self.systems.push(system_storage_id_ptr);
        self.storages.push(storage)
    }

    pub fn push_new(&mut self,system_storage_index_ptr:&mut usize){
        *system_storage_index_ptr=self.systems.len();

        let storage=SystemObjectStorage::new();

        self.systems.push(system_storage_index_ptr);
        self.storages.push(storage);
    }

    pub fn get_storage(&mut self,index:usize)->&mut SystemObjectStorage{
        &mut self.storages[index]
    }

    pub fn remove(&mut self,index:usize){
        let last_object=self.storages.pop().unwrap();
        let last_system=self.systems.pop().unwrap();

        if !self.storages.is_empty(){
            unsafe{
                *last_system=index
            }

            for unit in &mut self.storages[index].units{
                for object in &mut unit.objects{
                    object.drop()
                }
            }

            self.storages[index]=last_object;
            self.systems[index]=last_system;
        }
    }

    pub fn event(&mut self,event:ObjectEvent){
        for object_storage in &mut self.storages{
            for object_unit in &mut object_storage.units{
                for object in &mut object_unit.objects{
                    object.event(event)
                }
            }
        }
    }
}



pub struct ObjectManager<'a>{
    pub (crate) system_object_storage:&'a mut SystemObjectStorage,
    pub (crate) graphics:&'a mut Graphics,
}

impl<'a> ObjectManager<'a>{
    pub (crate) fn new(
        system_object_storage:&'a mut SystemObjectStorage,
        graphics:&'a mut Graphics
    )->ObjectManager<'a>{
        Self{
            system_object_storage,
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
            Ok(render_object_id)=>Ok({
                let render_data=RenderDataInterface::new(self.graphics,layer,render_object_id);

                let mut object_table=ObjectTable::simple(object,render_data);
                let object_data=object_table.object_data() as *mut O;
                let object_unit=ObjectUnit::single(object_table);

                self.system_object_storage.push(object_unit);

                ObjectReference::new(object_data)
            }),
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
            Ok(render_object_id)=>Ok({
                let render_data=RenderDataInterface::new(self.graphics,layer,render_object_id);

                let mut object_table=ObjectTable::texture(object,render_data);
                let object_data=object_table.object_data() as *mut O;
                let object_unit=ObjectUnit::single(object_table);

                self.system_object_storage.push(object_unit);

                ObjectReference::new(object_data)
            }),
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
            Ok(render_object_id)=>Ok({
                let render_data=RenderDataInterface::new(self.graphics,layer,render_object_id);

                let mut object_table=ObjectTable::text(object,render_data);
                let object_data=object_table.object_data() as *mut O;
                let object_unit=ObjectUnit::single(object_table);

                self.system_object_storage.push(object_unit);

                ObjectReference::new(object_data)
            }),
            Err(e)=>Err(e)
        }
    }

    pub fn push_simple_object_array<O:SimpleObject>(&mut self)->ObjectArray<'a,O>{
        let index=self.system_object_storage.len();

        let unit=ObjectUnit::array(8);
        self.system_object_storage.push(unit);

        ObjectArray::new(index,Vec::with_capacity(8))
    }

    pub fn push_texture_object_array<O:TextureObject>(&mut self)->ObjectArray<'a,O>{
        let index=self.system_object_storage.len();

        let unit=ObjectUnit::array(8);
        self.system_object_storage.push(unit);

        ObjectArray::new(index,Vec::with_capacity(8))
    }

    pub fn push_text_object_array<O:TextObject>(&mut self)->ObjectArray<'a,O>{
        let index=self.system_object_storage.len();

        let unit=ObjectUnit::array(8);
        self.system_object_storage.push(unit);

        ObjectArray::new(index,Vec::with_capacity(8))
    }

    pub fn graphics(&mut self)->&mut Graphics{
        self.graphics
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



pub struct ObjectReference<'a,O>{
    object_data:*mut O,
    marker:PhantomData<&'a O>
}

impl<'a,O> ObjectReference<'a,O>{
    pub (crate) fn new(object_data:*mut O)->ObjectReference<'a,O>{
        Self{
            object_data,
            marker:PhantomData
        }
    }
}

impl<'a,O> Deref for ObjectReference<'a,O>{
    type Target=O;

    fn deref(&self)->&O{
        unsafe{&*self.object_data}
    }
}

impl<'a,O> DerefMut for ObjectReference<'a,O>{
    fn deref_mut(&mut self)->&mut O{
        unsafe{&mut *self.object_data}
    }
}

pub struct ObjectArray<'a,O>{
    index:usize,
    /// Массив ссылок на объекты.
    objects:Vec<ObjectReference<'a,O>>
}

impl<'a,O> ObjectArray<'a,O>{
    pub (crate) fn new(index:usize,objects:Vec<ObjectReference<'a,O>>)->ObjectArray<'a,O>{
        Self{
            index,
            objects
        }
    }

    pub fn len(&self)->usize{
        self.objects.len()
    }

    pub fn get(&mut self,index:usize)->Option<ObjectReference<'a,O>>{
        if let Some(reference)=self.objects.get_mut(index){
            Some(unsafe{(reference as *mut ObjectReference<'a,O>).read()})
        }
        else{
            None
        }
    }

    pub fn remove(&mut self,index:usize,object_manager:&mut ObjectManager){
        let mut object_table=object_manager.system_object_storage.get_unit(self.index).remove(index);
        object_table.drop();

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
        let render_object_id=object_manager.graphics.simple.push_object(attributes,layer).unwrap();

        let render_data=RenderDataInterface::new(object_manager.graphics,layer,render_object_id);

        let mut object_table=ObjectTable::simple(object,render_data);
        let object_data=object_table.object_data() as *mut O;

        object_manager.system_object_storage.get_unit(self.index).push(object_table);

        self.objects.push(ObjectReference::new(object_data));
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
        let render_object_id=object_manager.graphics.texture.push_object(attributes,layer).unwrap();

        let render_data=RenderDataInterface::new(object_manager.graphics,layer,render_object_id);

        let mut object_table=ObjectTable::texture(object,render_data);
        let object_data=object_table.object_data() as *mut O;

        object_manager.system_object_storage.get_unit(self.index).push(object_table);

        self.objects.push(ObjectReference::new(object_data));
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
        let render_object_id=object_manager.graphics.text.push_object(attributes,layer).unwrap();

        let render_data=RenderDataInterface::new(object_manager.graphics,layer,render_object_id);

        let mut object_table=ObjectTable::text(object,render_data);
        let object_data=object_table.object_data() as *mut O;

        object_manager.system_object_storage.get_unit(self.index).push(object_table);

        self.objects.push(ObjectReference::new(object_data));
    }
}