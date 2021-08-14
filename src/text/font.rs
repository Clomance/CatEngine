use crate::graphics::Graphics2D;

use super::{
    TexturedGlyph,
    Scale,
    GlyphCache,
};

use ttf_parser::{
    Face,
    GlyphId,
    FaceParsingError
};

use std::{
    path::Path,
    fs::read,
    io::Error,
};

#[derive(Debug)]
pub enum FontError{
    IOError(Error),
    ParseError(FaceParsingError)
}

/// A font owner.
/// 
/// Contains font's data.
pub struct FontOwner{
    /// Данные
    data:Vec<u8>,
    /// Ссылка на данные,
    /// которая предоставляет методы для работы с ними
    face:Face<'static>,
}

impl FontOwner{
    /// Loads data from the path and then parses it.
    pub fn load<P:AsRef<Path>>(path:P)->Result<FontOwner,FontError>{
        let data=match read(path){
            Ok(data)=>data,
            Err(e)=>return Err(FontError::IOError(e)),
        };

        let slice:&'static [u8]=unsafe{
            std::slice::from_raw_parts(data.as_ptr(),data.len())
        };

        let face=match Face::from_slice(slice,0){
            Ok(face)=>face,
            Err(e)=>return Err(FontError::ParseError(e)),
        };

        Ok(Self{
            data,
            face,
        })
    }

    /// Takes data and then parses it.
    pub fn parse(data:Vec<u8>)->Result<FontOwner,FaceParsingError>{
        let slice:&'static [u8]=unsafe{
            std::slice::from_raw_parts(data.as_ptr(),data.len())
        };

        let face=match Face::from_slice(slice,0){
            Ok(face)=>face,
            Err(e)=>return Err(e),
        };

        Ok(Self{
            data,
            face,
        })
    }

    /// Copies data and then parses it.
    pub fn parse_copy(data:&Vec<u8>)->Result<FontOwner,FaceParsingError>{
        let data_copy=data.clone();

        let slice:&'static [u8]=unsafe{
            std::slice::from_raw_parts(data_copy.as_ptr(),data_copy.len())
        };

        let face=match Face::from_slice(slice,0){
            Ok(face)=>face,
            Err(e)=>return Err(e),
        };

        Ok(Self{
            data:data_copy,
            face,
        })
    }

    /// Checks wheather data is correct.
    pub fn parse_check(data:&Vec<u8>)->Result<(),FaceParsingError>{
        match Face::from_slice(data,0){
            Ok(_)=>Ok(()),
            Err(e)=>return Err(e),
        }
    }

    pub fn face(&self)->&Face{
        &self.face
    }

    pub fn face_wrapper<'a>(&'a self)->FaceWrapper<'a>{
        FaceWrapper(self.face.clone())
    }
}



// /ᐠ｡ꞈ｡ᐟ\
/// A wrapper that provides methods to work with fonts.
/// 
/// Обёртка, позволяющая работать со шрифтом.
pub struct FaceWrapper<'a>(pub Face<'a>);

impl<'a> FaceWrapper<'a>{
    pub fn glyph_id(&self,character:char)->Option<GlyphId>{
        self.0.glyph_index(character)
    }

    pub fn scale_for_height(&self,height:f32)->Scale{
        let k=height/self.0.global_bounding_box().height() as f32;
        Scale::new(k,k)
    }
}


/// A font with a glyph cache.
/// 
/// Шрифт с хранилищем глифов.
pub struct CachedFont{
    font:FontOwner,
    cache:GlyphCache,
}

impl CachedFont{
    pub fn raw(font:FontOwner,glyph_cache:GlyphCache)->CachedFont{
        Self{
            font,
            cache:glyph_cache,
        }
    }

    pub fn new_alphabet(font:FontOwner,alphabet:&str,scale:Scale,graphics:&Graphics2D)->CachedFont{
        let face=font.face();
        let cache=GlyphCache::new_alphabet(face,alphabet,scale,graphics);

        Self{
            font,
            cache
        }
    }

    pub fn scale_for_height(&self,height:f32)->Scale{
        self.font.face_wrapper().scale_for_height(height)
    }

    pub fn glyph_id(&self,character:char)->Option<GlyphId>{
        self.font.face_wrapper().glyph_id(character)
    }

    pub fn text_width(&self,text:&str,scale:Scale)->f32{
        let mut text_width=0f32;
        let glyph_cache_scale=scale/self.glyph_cache().scale();

        for character in text.chars(){
            let glyph_id=if let Some(glyph_id)=self.glyph_id(character){
                glyph_id
            }
            else{
                GlyphId(0u16)
            };

            if let Some(cached_glyph)=self.cached_glyph(glyph_id){
                let width=cached_glyph.advance_width(glyph_cache_scale.horizontal);

                text_width+=width;
            }
            else if let Some(advance_width)=self.font.face.glyph_hor_advance(glyph_id){
                text_width+=advance_width as f32*scale.horizontal
            }
        }

        text_width
    }

    pub fn text_size(&self,text:&str,scale:Scale)->[f32;2]{
        let mut size=[0f32;2];
        let glyph_cache_scale=scale/self.glyph_cache().scale();

        for character in text.chars(){
            let glyph_id=if let Some(glyph_id)=self.glyph_id(character){
                glyph_id
            }
            else{
                GlyphId(0u16)
            };

            if let Some(cached_glyph)=self.cached_glyph(glyph_id){
                let width=cached_glyph.advance_width(glyph_cache_scale.horizontal);
                let height=cached_glyph.height(glyph_cache_scale.vertical);

                size[0]+=width;
                if height>size[1]{
                    size[1]=height
                }
            }
            else if let Some(bounding_box)=self.font.face.glyph_bounding_box(glyph_id){
                let width=self.font.face.glyph_hor_advance(glyph_id).unwrap() as f32*scale.horizontal;

                let height=bounding_box.height() as f32*scale.vertical;

                size[0]+=width;
                if height>size[1]{
                    size[1]=height
                }
            }
            else if let Some(advance_width)=self.font.face.glyph_hor_advance(glyph_id){
                size[0]+=advance_width as f32*scale.horizontal
            }
        }

        size
    }
}

impl CachedFont{
    pub fn font(&self)->&FontOwner{
        &self.font
    }

    pub fn glyph_cache(&self)->&GlyphCache{
        &self.cache
    }

    pub fn cached_glyph(&self,id:GlyphId)->Option<&TexturedGlyph>{
        self.cache.glyph(id)
    }
}