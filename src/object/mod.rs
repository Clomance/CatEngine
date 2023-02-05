mod audio;

mod graphics;

use cat_audio::OutputObject;

pub use self::audio::AudioObjectManager;
pub use self::graphics::GraphicsObjectManager;

use std::ops::{
    DerefMut,
    Deref
};



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
    index:usize,
    inner:Vec<ObjectReference<'a,O>>
}

impl<'a,O:OutputObject> ArrayReference<'a,O>{
    pub fn len(&self)->usize{
        self.inner.len()
    }

    pub fn push(&mut self,object:O,manager:&mut ObjectManager){
        manager.audio.push_to_array(self,object)
    }
}

impl<'a,O> Deref for ArrayReference<'a,O>{
    type Target=[ObjectReference<'a,O>];

    fn deref(&self)->&[ObjectReference<'a,O>]{
        self.inner.as_ref()
    }
}



pub struct ObjectManager<'m>{
    pub audio:AudioObjectManager<'m>,
    pub graphics:GraphicsObjectManager<'m>
}

impl<'m> ObjectManager<'m>{
    pub (crate) fn new(audio:AudioObjectManager<'m>,graphics:GraphicsObjectManager<'m>)->ObjectManager<'m>{
        ObjectManager{
            audio,
            graphics
        }
    }
}
