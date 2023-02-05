use crate::{
    OutputBuffer,
};

use std::ops::{
    Deref,
    DerefMut
};



pub trait OutputObject{
    fn render(&mut self,output:&mut OutputBuffer);
}



pub struct ObjectReference<'a,O>{
    inner:&'a mut O
}

impl<'a,O> Deref for ObjectReference<'a,O>{
    type Target=O;

    fn deref(&self)->&O{
        self.inner
    }
}

impl<'a,O> DerefMut for ObjectReference<'a,O>{
    fn deref_mut(&mut self)->&mut O{
        self.inner
    }
}



pub struct ArrayReference<'a,O>{
    /// The unit index in the object storage.
    index:usize,
    inner:Vec<ObjectReference<'a,O>>
}

impl<'a,O> Deref for ArrayReference<'a,O>{
    type Target=Vec<ObjectReference<'a,O>>;

    fn deref(&self)->&Vec<ObjectReference<'a,O>>{
        &self.inner
    }
}



pub struct ObjectTable{
    data:*mut (),
    render:fn(*mut (),&mut OutputBuffer),
    drop:fn(*mut ())
}

impl ObjectTable{
    pub fn new<O:OutputObject>(object:O)->ObjectTable{
        let object=Box::new(object);

        let data=Box::leak(object);

        Self{
            data:data as *mut O as *mut (),
            render:Self::render_wrapper::<O>,
            drop:Self::drop_wrapper::<O>
        }
    }

    pub fn render(&mut self,output:&mut OutputBuffer){
        (self.render)(self.data,output)
    }

    fn render_wrapper<O:OutputObject>(data:*mut (),output:&mut OutputBuffer){
        let object=unsafe{&mut *(data as *mut O)};
        object.render(output)
    }

    fn drop_wrapper<O:OutputObject>(data:*mut ()){
        unsafe{drop(Box::from_raw(data as *mut O));}
    }
}

impl Drop for ObjectTable{
    fn drop(&mut self){
        (self.drop)(self.data)
    }
}



pub struct ObjectUnit{
    objects:Vec<ObjectTable>
}

impl ObjectUnit{
    pub fn new()->ObjectUnit{
        ObjectUnit{
            objects:Vec::new()
        }
    }

    pub fn push<O:OutputObject>(&mut self,object:O)->*mut O{
        let table=ObjectTable::new(object);
        let object_data=table.data;
        self.objects.push(table);
        object_data as _
    }

    pub fn insert<O:OutputObject>(&mut self,index:usize,object:O)->*mut O{
        let table=ObjectTable::new(object);
        let object_data=table.data;
        self.objects.insert(index,table);
        object_data as _
    }

    pub fn remove(&mut self,index:usize){
        self.objects.remove(index);
    }
}



pub struct ObjectStorage{
    units:Vec<ObjectUnit>
}

impl ObjectStorage{
    pub fn new()->ObjectStorage{
        ObjectStorage{
            units:Vec::new()
        }
    }

    pub fn push<'a,O:OutputObject>(&mut self,object:O)->ObjectReference<'a,O>{
        let mut unit=ObjectUnit::new();
        let object_data=unit.push(object);
        self.units.push(unit);

        ObjectReference{
            inner:unsafe{&mut *object_data}
        }
    }

    pub fn push_array<'a,O:OutputObject>(&mut self)->ArrayReference<'a,O>{
        let index=self.units.len();
        let unit=ObjectUnit::new();
        self.units.push(unit);

        ArrayReference{
            index,
            inner:Vec::new()
        }
    }

    pub fn push_to_array<'a,O:OutputObject>(&mut self,array:&mut ArrayReference<'a,O>,object:O){
        unsafe{
            let object_data=self.units.get_unchecked_mut(array.index).push(object);

            let object_reference=ObjectReference{
                inner:&mut *object_data
            };

            array.inner.push(object_reference)
        }
    }

    pub fn insert_into_array<'a,O:OutputObject>(&mut self,array:&mut ArrayReference<'a,O>,index:usize,object:O){
        unsafe{
            let object_data=self.units.get_unchecked_mut(array.index).insert(index,object);

            let object_reference=ObjectReference{
                inner:&mut *object_data
            };

            array.inner.insert(index,object_reference)
        }
    }

    pub fn remove_from_array<'a,O:OutputObject>(&mut self,array:&mut ArrayReference<'a,O>,index:usize){
        unsafe{
            self.units.get_unchecked_mut(array.index).remove(index);
            array.inner.remove(index);
        }
    }

    pub fn render(&mut self,output:&mut OutputBuffer){
        for unit in &mut self.units{
            for object in &mut unit.objects{
                object.render(output)
            }
        }
    }
}



pub struct Objects{
    /// Used to remove system and it's objects safely.
    system_storage_index_pointer:Vec<*mut usize>,
    pub (crate) storages:Vec<ObjectStorage>
}

impl Objects{
    pub (crate) fn new()->Objects{
        Self{
            system_storage_index_pointer:Vec::new(),
            storages:Vec::new()
        }
    }

    pub fn manager(&mut self)->ObjectManager{
        ObjectManager{
            objects:self
        }
    }

    pub fn push(&mut self,system_storage_index_pointer:*mut usize){
        let index=self.system_storage_index_pointer.len();
        unsafe{*system_storage_index_pointer=index}
        self.system_storage_index_pointer.push(system_storage_index_pointer);
        self.storages.push(ObjectStorage::new())
    }

    pub fn push_new(&mut self,system_storage_index_pointer:*mut usize,storage:ObjectStorage){
        let index=self.system_storage_index_pointer.len();
        unsafe{*system_storage_index_pointer=index}
        self.system_storage_index_pointer.push(system_storage_index_pointer);
        self.storages.push(storage)
    }

    pub fn get(&mut self,index:usize)->&mut ObjectStorage{
        &mut self.storages[index]
    }

    pub fn remove(&mut self,index:usize){
        if let Some(object_storage)=self.storages.pop(){
            let index_ptr=self.system_storage_index_pointer.pop().unwrap();

            if self.storages.is_empty() || index==self.storages.len(){
                return
            }

            unsafe{*index_ptr=index}

            self.system_storage_index_pointer[index]=index_ptr;
            self.storages[index]=object_storage
        }
    }
}



pub struct ObjectManager<'m>{
    objects:&'m mut Objects
}

impl<'m> ObjectManager<'m>{
    pub fn system_manager(&mut self,index:usize)->SystemObjectManager{
        SystemObjectManager{
            objects:self.objects.get(index)
        }
    }

    pub fn push(&mut self,system_storage_index_pointer:*mut usize){
        self.objects.push(system_storage_index_pointer)
    }

    pub fn push_new(&mut self,system_storage_index_pointer:*mut usize,storage:ObjectStorage){
        self.objects.push_new(system_storage_index_pointer,storage);
    }

    pub fn get(&mut self,index:usize)->&mut ObjectStorage{
        self.objects.get(index)
    }

    pub fn remove(&mut self,index:usize){
        self.objects.remove(index)
    }
}



pub struct SystemObjectManager<'m>{
    objects:&'m mut ObjectStorage
}

impl<'m> SystemObjectManager<'m>{
    pub fn new(objects:&'m mut ObjectStorage)->SystemObjectManager<'m>{
        Self{
            objects
        }
    }

    pub fn push<'a,O:OutputObject>(&mut self,object:O)->ObjectReference<'a,O>{
        self.objects.push(object)
    }

    pub fn push_array<'a,O:OutputObject>(&mut self)->ArrayReference<'a,O>{
        self.objects.push_array()
    }

    pub fn push_to_array<'a,O:OutputObject>(&mut self,array:&mut ArrayReference<'a,O>,object:O){
        self.objects.push_to_array(array,object)
    }

    pub fn insert_into_array<'a,O:OutputObject>(&mut self,array:&mut ArrayReference<'a,O>,index:usize,object:O){
        self.objects.insert_into_array(array,index,object)
    }

    pub fn remove_from_array<'a,O:OutputObject>(&mut self,array:&mut ArrayReference<'a,O>,index:usize){
        self.objects.remove_from_array(array,index)
    }
}