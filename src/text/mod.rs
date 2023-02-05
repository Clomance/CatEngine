mod glyphs;
pub use glyphs::{
    Scale,
    Glyph,
    FontOwner,
    GlyphCache,
    GlyphCacheBuilder,
    UnsafeGlyphCacheBuilder
};

use cat_engine_basement::utility::storage::StaticStorage;



pub (crate) struct GlyphCacheUnit{
    pub cache:GlyphCache,
    pub reference_count:usize
}

impl GlyphCacheUnit{
    pub fn new(glyph_cache:GlyphCache)->GlyphCacheUnit{
        Self{
            cache:glyph_cache,
            reference_count:0
        }
    }

    pub fn inc(&mut self){
        self.reference_count+=1
    }

    pub fn dec(&mut self){
        self.reference_count-=1
    }
}



pub struct GlyphCacheUnitReference{
    reference:&'static mut GlyphCacheUnit
}

impl GlyphCacheUnitReference{
    pub (crate) fn new(glyph_cache:&mut GlyphCacheUnit)->GlyphCacheUnitReference{
        Self{
            reference:unsafe{std::mem::transmute(glyph_cache)}
        }
    }

    pub fn cache(&self)->&GlyphCache{
        &self.reference.cache
    }
}

impl Drop for GlyphCacheUnitReference{
    fn drop(&mut self){
        self.reference.dec()
    }
}



pub (crate) struct Glyphs{
    glyphs:StaticStorage<GlyphCacheUnit>,
}

impl Glyphs{
    pub fn new(capacity:usize)->Glyphs{
        Self{
            glyphs:StaticStorage::new(capacity)
        }
    }

    pub fn manager(&mut self)->GlyphCacheManager{
        GlyphCacheManager{
            glyphs:self
        }
    }

    pub fn add(&mut self,glyph_cache:GlyphCache)->Option<usize>{
        self.glyphs.add(GlyphCacheUnit::new(glyph_cache))
    }

    pub fn get(&mut self,id:usize)->Option<&GlyphCache>{
        if let Some(unit)=self.glyphs.get(id){
            Some(&unit.cache)
        }
        else{
            None
        }
    }

    pub fn get_reference(&mut self,id:usize)->Option<GlyphCacheUnitReference>{
        if let Some(unit)=self.glyphs.get_mut(id){
            unit.inc();
            Some(GlyphCacheUnitReference::new(unit))
        }
        else{
            None
        }
    }

    pub fn remove(&mut self,id:usize)->Option<GlyphCache>{
        if let Some(GlyphCacheUnit{reference_count,..})=self.glyphs.get(id){
            if *reference_count==0{
                Some(self.glyphs.remove(id).unwrap().cache)
            }
            else{
                None
            }
        }
        else{
            None
        }
    }
}



pub struct GlyphCacheManager<'m>{
    glyphs:&'m mut Glyphs
}

impl<'m> GlyphCacheManager<'m>{
    pub fn push_glyphs(&mut self,font:GlyphCache)->Option<usize>{
        self.glyphs.add(font)
    }

    pub fn get_glyphs<'a>(&'a mut self,font:usize)->Option<&'a GlyphCache>{
        self.glyphs.get(font)
    }

    pub fn remove_glyphs(&mut self,id:usize)->Option<GlyphCache>{
        self.glyphs.remove(id)
    }
}