mod manager;

pub use manager::ObjectManager;

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
        ElementIndexType,
    },
    text::GlyphCache,
    texture::Texture2D, object::storage::ObjectData
};

pub use self::storage::{ObjectReference, ObjectArray};

pub trait SimpleObject:Sized{
    fn event(&mut self,event:ObjectEvent);

    fn get_render_data<'o,'d>(&'o self)->SimpleRenderData<'d>{
        unsafe{
            let ptr=self as *const Self as *const usize;

            let table=&*(ptr.offset(-3) as *const ObjectData<Self>);
            let graphics=&mut *table.graphics;

            let render_data=graphics.simple.get_render_data(table.layer,table.object_id).unwrap();

            SimpleRenderData{
                render:render_data
            }
        }
    }
}

pub trait TextureObject:Sized{
    fn event(&mut self,event:ObjectEvent);

    fn get_render_data<'o,'d>(&'o self)->TextureRenderData<'d>{
        unsafe{
            let ptr=self as *const Self as *const usize;

            let table=&*(ptr.offset(-3) as *const ObjectData<Self>);
            let graphics=&mut *table.graphics;

            let texture=graphics.texture.get_layer_texture_raw(table.layer);
            let render_data=graphics.texture.get_render_data(table.layer,table.object_id);

            TextureRenderData{
                render:render_data,
                texture:&*texture
            }
        }
    }
}

pub trait TextObject:Sized{
    fn event(&mut self,event:ObjectEvent);

    fn get_render_data<'o,'d>(&'o self)->TextRenderData<'d>{
        unsafe{
            let ptr=self as *const Self as *const usize;

            let table=&*(ptr.offset(-3) as *const ObjectData<Self>);
            let graphics=&mut *table.graphics;

            let glyph_cache=graphics.text.get_layer_font_raw(table.layer);
            let render_data=graphics.text.get_render_data(table.layer,table.object_id);

            TextRenderData{
                render:render_data,
                glyph_cache:&*glyph_cache
            }
        }
    }
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

    pub fn handle(&mut self,storage:usize,event:ObjectEvent){
        for objects in &mut self.objects.get_mut(storage).unwrap().data{
            for object in objects{
                object.handle(event);
            }
        }
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