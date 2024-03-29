use crate::graphics::Graphics2D;

use super::{
    TexturedGlyph,
    Scale,
};

use cat_engine_basement::graphics::{
    GLCore,
    core::parameters::UNPACK_ALIGNMENT,
    core::texture::{
        Texture2DInternalFormat,
        ImageDataFormat,
        TextureMagFilter,
        TextureMinFilter,
    },
    level1::Texture2D,
};

use ttf_parser::{
    Face,
    GlyphId,
};

use std::{
    collections::HashMap,
    ops::Range,
};

// ᶠᵉᵉᵈ ᵐᵉ /ᐠ-ⱉ-ᐟ\ﾉ
/// A glyph cache.
/// Хранилище глифов.
pub struct GlyphCache{
    // Глифы
    glyphs:HashMap<u16,TexturedGlyph>,
    scale:Scale,
}

impl GlyphCache{
    /// Creates a new glyph cache with the given range of characters ids.
    /// 
    /// Ignores characters that is not contained by the font.
    /// 
    /// If `range` is `None`, takes all the characters of the font.
    /// 
    /// Создаёт новое хранилище глифов для данной области номеров символов.
    /// 
    /// Игнорирует символы, которых нет в шрифте.
    /// 
    /// Если `range` равно `None`, то используются все символы шрифта.
    pub fn new(font:&Face,range:Option<Range<u16>>,scale:Scale,graphics:&Graphics2D)->GlyphCache{
        let range=if let Some(range)=range{
            range
        }
        else{
            1u16..font.number_of_glyphs()
        };

        let mut cache=Self{
            glyphs:HashMap::with_capacity(range.len()),
            scale
        };

        for g in range{
            let id=GlyphId(g);

            cache.insert_glyph(id,font,graphics)
        }

        cache
    }

    /// Creates a new glyph cache for the given alphabet.
    /// 
    /// Ignors undefined characters.
    /// 
    /// Создаёт новое хранилище глифов для данного алфавита.
    /// 
    /// Игнорирует неопределённые символы.
    pub fn new_alphabet(font:&Face,alphabet:&str,scale:Scale,graphics:&Graphics2D)->GlyphCache{
        let mut cache=Self{
            glyphs:HashMap::with_capacity(alphabet.len()),
            scale,
        };

        for character in alphabet.chars(){
            if let Some(id)=font.glyph_index(character){
                cache.insert_glyph(id,font,graphics)
            }
        }

        cache
    }

    /// Создаёт и добавляет новый глиф для данного символа.
    /// 
    /// Игнорирует неопределённые символы.
    /// Заменяет старый глиф для этого символа, если такой есть.
    /// 
    /// Creates and inserts a new glyph for the given character.
    /// 
    /// Ignors undefined characters.
    /// Replaces the old glyph for this character if there is one.
    pub fn insert_glyph(&mut self,id:GlyphId,font:&Face,graphics:&Graphics2D){
        if let Some(glyph)=build_glyph(id,self.scale,font,graphics){
            self.glyphs.insert(id.0,glyph);
        }
    }

    pub fn insert_char(&mut self,character:char,font:&Face,graphics:&Graphics2D){
        if let Some(id)=font.glyph_index(character){
            self.insert_glyph(id,font,graphics)
        }
    }

    /// Создаёт и вставляет новые глифы для данный символов.
    /// 
    /// Creates and inserts new glyphs for the given characters.
    /// 
    /// ```
    /// for character in alphabet.chars(){
    ///     self.insert_char(character,font,scale);
    /// }
    /// ```
    pub fn insert_str(&mut self,font:&Face,alphabet:&str,graphics:&Graphics2D){
        for character in alphabet.chars(){
            self.insert_char(character,font,graphics);
        }
    }

    pub fn scale(&self)->Scale{
        self.scale
    }

    pub fn glyph(&self,id:GlyphId)->Option<&TexturedGlyph>{
        self.glyphs.get(&id.0)
    }
}


fn build_glyph(id:GlyphId,scale:Scale,face:&Face,graphics:&Graphics2D)->Option<TexturedGlyph>{
    if let Some((
        [
            offset_x,
            offset_y,
            width,
            height,
        ],
        image,
    ))=graphics.build_glyph_image(id,scale,face){
        let size=[width as u32,height as u32];

        unsafe{GLCore.parameters.set_pixel_storage_modei(UNPACK_ALIGNMENT,1)}

        let texture_2d=Texture2D::new(
            Texture2DInternalFormat::R8,
            TextureMagFilter::Linear,
            TextureMinFilter::Linear,
            size,
            ImageDataFormat::R_U8,
            &image
        ).unwrap();

        unsafe{GLCore.parameters.set_pixel_storage_modei(UNPACK_ALIGNMENT,4)}

        let advance_width=face.glyph_hor_advance(id).unwrap() as f32*scale.horizontal;

        let glyph=TexturedGlyph::raw(
            texture_2d,
            [width,height],
            [offset_x,offset_y],
            advance_width,
        );

        Some(glyph)
    }
    else{
        None
    }
}