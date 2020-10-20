use super::{
    OutlineCurveBuilder,
    RawGlyph,
    OutlinedGlyph,
    Scale,
    OutlineCurve,
    GlyphCache,
    RawGlyphCache,
    ScaledGlyph,
};

use glium::{
    Display,
    texture::{
        Texture2d,
        UncompressedFloatFormat,
        MipmapsOption,
        RawImage2d,
        ClientFormat,
    },
    Rect,
};

use ttf_parser::{
    Face,
    GlyphId,
};

use std::{
    collections::HashMap,
    borrow::Cow,
    ops::Range,
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
    face:Option<Face<'static>>,
}

impl OwnedFont{
    fn load<P:AsRef<Path>>(path:P)->Option<OwnedFont>{
        let data=match read(path){
            Ok(data)=>data,
            Err(_)=>return None,
        };

        let mut font=Self{
            data,
            face:None,
        };

        let slice:&'static [u8]=unsafe{
            std::slice::from_raw_parts(font.data.as_ptr(),font.data.len())
        };

        let face=match Face::from_slice(slice,0){
            Ok(face)=>face,
            Err(_)=>return None,
        };

        font.face=Some(face);

        Some(font)
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
        self.font.as_ref().face.as_ref().unwrap()
    }

    pub fn face_wrapper<'a>(&'a self)->FaceWrapper<'a>{
        FaceWrapper(self.font.as_ref().face.clone().unwrap())
    }
}





// Ссылка на данные шрифта
// /ᐠ｡ꞈ｡ᐟ\
/// Обёртка позволяющая работать со шрифтом.
/// A wrapper that provides methods to work with fonts.
pub struct FaceWrapper<'a>(pub Face<'a>);

impl<'a> Font for FaceWrapper<'a>{
    fn scale_for_height(&self,height:f32)->Scale{
        let k=height/self.0.global_bounding_box().height() as f32;
        Scale::new(k,k)
    }

    fn build_raw_glyph(&self,character:char)->Option<RawGlyph<Vec<OutlineCurve>>>{
        // Поиск глифа
        if let Some(glyph_id)=self.0.glyph_index(character){
            let mut outline_builder=OutlineCurveBuilder::default();
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
                    outline_builder.outline,
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
        else{
            None
        }
    }

    fn build_raw_undefined_glyph(&self)->RawGlyph<Vec<OutlineCurve>>{
        let glyph_id=GlyphId(0);

        let mut outline_builder=OutlineCurveBuilder::default();

        // Получение точек для построения глифа
        let bounding_box=self.0.outline_glyph(glyph_id,&mut outline_builder).expect("No undefined glyph");
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

        RawGlyph::<Vec<OutlineCurve>>::raw(
            outline_builder.outline,
            glyph_size,
            glyph_offset,
            advance_width,
        )
    }

    fn whitespace_advance_width(&self,horizontal_scale:f32)->f32{
        // 3 - whitespace
        self.0.glyph_hor_advance(GlyphId(3)).expect("No whitespace glyph") as f32*horizontal_scale
    }
}


/// Шрифт с хранилищем глифов.
/// A font with glyph cache.
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

    pub fn new_alphabet(font:FontOwner,alphabet:&str,scale:Scale,display:&Display)->CachedFont{
        let face=font.face();
        let cache=GlyphCache::new_alphabet(face,alphabet,scale,display);

        Self{
            font,
            cache
        }
    }

    pub fn scaled_undefined_glyph(&self,scale:Scale)->ScaledGlyph<Texture2d>{
        self.cache.raw_undefined_glyph().scale(scale)
    }
}

impl Font for CachedFont{
    fn scale_for_height(&self,height:f32)->Scale{
        self.font.face_wrapper().scale_for_height(height)
    }

    fn build_raw_glyph<'a>(&'a self,character:char)->Option<RawGlyph<Vec<OutlineCurve>>>{
        self.font.face_wrapper().build_raw_glyph(character)
    }

    fn build_raw_undefined_glyph(&self)->RawGlyph<Vec<OutlineCurve>>{
        self.font.face_wrapper().build_raw_undefined_glyph()
    }

    fn whitespace_advance_width(&self,horizontal_scale:f32)->f32{
        self.font.face_wrapper().whitespace_advance_width(horizontal_scale)
    }
}

impl RawGlyphCache for CachedFont{
    fn whitespace_advance_width(&self,horizontal_scale:f32)->f32{
        self.cache.whitespace_advance_width()*horizontal_scale
    }

    fn raw_glyph(&self,character:char)->Option<&RawGlyph<Texture2d>>{
        if let Some(glyph)=self.cache.raw_glyph(character){
            Some(glyph)
        }
        else{
            None
        }
    }

    fn raw_undefined_glyph(&self)->&RawGlyph<Texture2d>{
        self.cache.raw_undefined_glyph()
    }
}


/// Типаж для определения шрифтов.
/// A trait for defining fonts.
pub trait Font{
    /// Возвращает масштабированную ширину пробела.
    /// 
    /// Returns whitespace's scaled advance width.
    fn whitespace_advance_width(&self,horizontal_scale:f32)->f32;

    /// Возвращает масштаб, при котором
    /// все глифы шрифта подходят под данную высоту.
    /// 
    /// Returns the scale for the font's glyphs
    /// that makes them fit this height.
    fn scale_for_height(&self,height:f32)->Scale;
    /// Строит глиф для данного символа.
    /// 
    /// Builds a glyph for the given character.
    fn build_raw_glyph<'a>(&'a self,character:char)->Option<RawGlyph<Vec<OutlineCurve>>>;

    /// Строит глиф для неопределённого символа.
    /// 
    /// Builds a glyph for the undefined character.
    fn build_raw_undefined_glyph(&self)->RawGlyph<Vec<OutlineCurve>>;
}