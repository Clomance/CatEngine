use crate::graphics::Graphics2D;

use super::{
    OutlineCurveBuilder,
    RawGlyph,
    Scale,
    OutlineCurve,
    GlyphCache,
};

use cat_engine_basement::graphics::level1::Texture2D;

use ttf_parser::{
    Face,
    GlyphId,
};

use std::{
    path::Path,
    fs::read,
};

/// Хранит данные шрифта.
/// Должен быть несдвигаемым в памяти,
/// чтобы не сломать ссылку.
/// 
/// Contains the font data.
struct OwnedFont{
    // Данные
    data:Vec<u8>,
    // Ссылка на данные, которая предоставляет методы для работы с ними
    face:Face<'static>,
}

impl OwnedFont{
    fn load<P:AsRef<Path>>(path:P)->Option<OwnedFont>{
        let data=match read(path){
            Ok(data)=>data,
            Err(_)=>return None,
        };

        let slice:&'static [u8]=unsafe{
            std::slice::from_raw_parts(data.as_ptr(),data.len())
        };

        let face=match Face::from_slice(slice,0){
            Ok(face)=>face,
            Err(_)=>return None,
        };

        Some(Self{
            data,
            face,
        })
    }
}





/// Хранилище для шрифта.
/// A font owner.
pub struct FontOwner{
    font:Box<OwnedFont>,
}

impl FontOwner{
    pub fn load<P:AsRef<Path>>(path:P)->Option<FontOwner>{
        let font=OwnedFont::load(path)?;

        Some(Self{
            font:Box::new(font),
        })
    }

    pub fn face(&self)->&Face{
        &self.font.as_ref().face
    }

    pub fn face_wrapper<'a>(&'a self)->FaceWrapper<'a>{
        FaceWrapper(self.font.as_ref().face.clone())
    }
}





// Ссылка на данные шрифта
// /ᐠ｡ꞈ｡ᐟ\
/// Обёртка, позволяющая работать со шрифтом.
/// A wrapper that provides methods to work with fonts.
pub struct FaceWrapper<'a>(pub Face<'a>);

impl<'a> FaceWrapper<'a>{
    pub fn glyph_id(&self,character:char)->Option<GlyphId>{
        self.0.glyph_index(character)
    }

    pub fn scale_for_height(&self,height:f32)->Scale{
        let k=height/self.0.global_bounding_box().height() as f32;
        Scale::new(k,k)
    }

    pub fn build_raw_glyph(&self,glyph_id:GlyphId)->Option<RawGlyph<Vec<OutlineCurve>>>{
        // Поиск глифа
        let mut outline_builder=OutlineCurveBuilder::new();
        // Получение точек для построения глифа
        if let Some(bounding_box)=self.0.outline_glyph(glyph_id,&mut outline_builder){
            let glyph_size=[
                bounding_box.width() as f32,
                bounding_box.height() as f32,
            ];

            let glyph_offset=[
                bounding_box.x_min as f32,
                bounding_box.y_min as f32,
            ];

            // Горизонтальное расстояние до следующего символа
            let advance_width=self.0.glyph_hor_advance(glyph_id).unwrap() as f32;

            let glyph=RawGlyph::<Vec<OutlineCurve>>::raw(
                outline_builder.outline_curves(),
                glyph_size,
                glyph_offset,
                advance_width,
            );

            Some(glyph)
        }
        else{
            None
        }
    }
}


/// Шрифт с хранилищем глифов.
/// A font with a glyph cache.
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
        for character in text.chars(){
            let glyph_id=if let Some(glyph_id)=self.glyph_id(character){
                glyph_id
            }
            else{
                GlyphId(0u16)
            };

            if let Some(cached_glyph)=self.cached_glyph(glyph_id){
                let width=cached_glyph.width(scale.horizontal);

                text_width+=width;
            }
            else if let Some(glyph)=self.build_glyph(glyph_id){
                let width=glyph.width(scale.horizontal);

                text_width+=width;
            }
        }

        text_width
    }

    pub fn text_size(&self,text:&str,scale:Scale)->[f32;2]{
        let mut size=[0f32;2];
        for character in text.chars(){
            let glyph_id=if let Some(glyph_id)=self.glyph_id(character){
                glyph_id
            }
            else{
                GlyphId(0u16)
            };

            if let Some(cached_glyph)=self.cached_glyph(glyph_id){
                let width=cached_glyph.width(scale.horizontal);
                let height=cached_glyph.height(scale.vertical);

                size[0]+=width;
                if height>size[1]{
                    size[1]=height
                }
            }
            else if let Some(glyph)=self.build_glyph(glyph_id){
                let width=glyph.width(scale.horizontal);
                let height=glyph.height(scale.vertical);

                size[0]+=width;
                if height>size[1]{
                    size[1]=height
                }
            }
        }

        size
    }
}

impl CachedFont{
    pub fn font(&self)->&FontOwner{
        &self.font
    }

    pub fn build_glyph<'a>(&'a self,glyph_id:GlyphId)->Option<RawGlyph<Vec<OutlineCurve>>>{
        self.font.face_wrapper().build_raw_glyph(glyph_id)
    }
}

impl CachedFont{
    pub fn glyph_cache(&self)->&GlyphCache{
        &self.cache
    }

    pub fn cached_glyph(&self,id:GlyphId)->Option<&RawGlyph<Texture2D>>{
        self.cache.glyph(id)
    }
}