use std::mem::transmute;

use super::{ObjectReference, ArrayReference};

use cat_audio::{SystemObjectManager, OutputObject};



pub struct AudioObjectManager<'m>{
    manager:SystemObjectManager<'m>
}

impl<'m> AudioObjectManager<'m>{
    pub (crate) fn new(manager:SystemObjectManager)->AudioObjectManager<'m>{
        unsafe{transmute(manager)}
    }

    pub fn push<'a,O:OutputObject>(&mut self,object:O)->ObjectReference<'a,O>{
        unsafe{transmute(self.manager.push(object))}
    }

    pub fn push_array<'a,O:OutputObject>(&mut self)->ArrayReference<'a,O>{
        unsafe{transmute(self.manager.push_array::<O>())}
    }

    pub fn push_to_array<'a,O:OutputObject>(&mut self,array:&mut ArrayReference<'a,O>,object:O){
        unsafe{self.manager.push_to_array(transmute(array),object)}
    }

    pub fn insert_into_array<'a,O:OutputObject>(&mut self,array:&mut ArrayReference<'a,O>,index:usize,object:O){
        unsafe{self.manager.insert_into_array(transmute(array),index,object)}
    }

    pub fn remove_from_array<'a,O:OutputObject>(&mut self,array:&mut ArrayReference<'a,O>,index:usize){
        unsafe{self.manager.remove_from_array::<O>(transmute(array),index)}
    }
}