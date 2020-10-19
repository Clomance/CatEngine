use super::{
    OutlineCurveBuilder,
    RawGlyph,
    ScaledGlyph,
    OutlinedGlyph,
    Scale,
    OutlineCurve,
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

/// Хранилище глифов.
/// 
/// A glyph cache.

// ᶠᵉᵉᵈ ᵐᵉ /ᐠ-ⱉ-ᐟ\ﾉ
pub struct GlyphCache{
    glyphs:HashMap<char,RawGlyph<Texture2d>>,
    undefined_glyph:RawGlyph<Texture2d>,
}

impl GlyphCache{
    /// Создаёт новое хранилище глифов для данной области номеров символов.
    /// 
    /// Игнорирует неопределённые символы.
    /// 
    /// range = `None` - использует все символы шрифта.
    /// 
    /// Creates a new glyph cache with the given range of characters ids.
    /// 
    /// Ignors undefined characters.
    /// 
    /// range = `None` - takes all characters of the font.
    pub fn new(font:&Face,range:Option<Range<u16>>,scale:Scale,display:&Display)->GlyphCache{
        // Неопределённый символ
        let not_defined_id=GlyphId(0);
        let undefined_glyph=build_glyph(not_defined_id,scale,&font,display).unwrap();

        let range=if let Some(range)=range{
            range
        }
        else{
            1u16..font.number_of_glyphs()
        };

        let mut glyphs=HashMap::with_capacity(range.len());

        for g in range{
            let id=GlyphId(g);

            if let Some(glyph)=build_glyph(id,scale,&font,display){
                let character=unsafe{std::char::from_u32_unchecked(g as u32)};
                glyphs.insert(character,glyph);
            }
        }

        Self{
            glyphs,
            undefined_glyph,
        }
    }

    /// Создаёт новое хранилище глифов для данного алфавита.
    /// 
    /// Игнорирует неопределённые символы.
    /// 
    /// Creates a new glyph cache for the given alphabet.
    /// 
    /// Ignors undefined characters.
    pub fn new_alphabet(font:&Face,alphabet:&str,scale:Scale,display:&Display)->GlyphCache{
        let mut glyphs=HashMap::with_capacity(alphabet.len());

        // The maximal height of all glyphs.
        let global_height=font.ascender() as f32*scale.vertical;

        // Неопределённый символ
        let not_defined_id=GlyphId(0);
        let undefined_glyph=build_glyph(not_defined_id,scale,&font,display).unwrap();


        for character in alphabet.chars(){
            let id=font.glyph_index(character).unwrap();

            if let Some(glyph)=build_glyph(id,scale,&font,display){
                glyphs.insert(character,glyph);
            }
        }

        Self{
            glyphs,
            undefined_glyph,
        }
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
    pub fn insert_char(&mut self,character:char,font:&Face,scale:Scale,display:&Display){
        if let Some(id)=font.glyph_index(character){
            if let Some(glyph)=build_glyph(id,scale,font,display){
                self.glyphs.insert(character,glyph);
            }
        }
    }

    /// Создаёт и вставляет новые глифы для данный символов.
    /// 
    /// Creates and inserts new glyphs for the given characters.
    /// 
    /// ```
    /// for character in alphabet.chars(){
    ///     self.insert_char(character,font,scale,display);
    /// }
    /// ```
    pub fn insert_str(&mut self,font:&Face,alphabet:&str,scale:Scale,display:&Display){
        for character in alphabet.chars(){
            self.insert_char(character,font,scale,display);
        }
    }

    
    

    // pub fn text_width(&self,text:&str,scale:Scale)->f32{
    //     let mut width=0f32;
    //     for character in text.chars(){
    //         if let Some(glyph)=self.glyph(character){
    //             let height=glyph.height(scale.vertical);
    //             let advance_width=glyph.
    //             let glyph_size=glyph.height_and_advance(font_size);
    //             width+=glyph_size[0];
    //         }
    //         else{
    //             if character==' '{
    //                 width+=self.whitespace_advance(scale.horizontal);
    //                 continue
    //             }
    //             let glyph_size=self.undefined_glyph.height_and_advance(font_size);
    //             width+=glyph_size[0];
    //         }
    //     }
    //     width
    // }

    // pub fn text_size(&self,text:&str,font_size:f32)->[f32;2]{
    //     let mut size=[0f32;2];
    //     for character in text.chars(){
    //         if let Some(glyph)=self.glyph(character){
    //             let glyph_size=glyph.height_and_advance(font_size);
    //             if glyph_size[1]>size[1]{
    //                 size[1]=glyph_size[1];
    //             }
    //             size[0]+=glyph_size[0];
    //         }
    //         else{
    //             size[0]+=self.whitespace_advance(font_size);
    //         }
    //     }

    //     size
    // }
}

impl RawGlyphCache for GlyphCache{
    #[inline(always)]
    fn raw_glyph(&self,character:char)->Option<&RawGlyph<Texture2d>>{
        self.glyphs.get(&character)
    }

    #[inline(always)]
    fn raw_undefined_glyph(&self)->&RawGlyph<Texture2d>{
        &self.undefined_glyph
    }
}

fn build_glyph(id:GlyphId,scale:Scale,face:&Face,display:&Display)->Option<RawGlyph<Texture2d>>{
    let mut outline_builder=OutlineCurveBuilder::default();

    if let Some(bounds)=face.outline_glyph(id,&mut outline_builder){
        // Сдвиг символа относительно глобальной рамки (global bounding box)
        let offset=[
            (bounds.x_min as f32*scale.horizontal),
            (bounds.y_min as f32*scale.vertical),
        ];

        // Размер символа, изображения
        let size=[
            (bounds.width() as f32*scale.horizontal).ceil(),
            (bounds.height() as f32*scale.vertical).ceil(),
        ];

        let rect=[
            offset[0],
            offset[1],
            size[0],
            size[1]
        ];

        let glyph=OutlinedGlyph::new(outline_builder.outline,rect,scale);

        let width=size[0] as usize;
        let height=size[1] as u32;

        let len=width*height as usize;
        let mut image=Vec::with_capacity(len);

        glyph.draw(|_,a|{
            let gray=255f32*a;
            let byte=gray.round() as u8;
            image.push(byte);
        });

        let texture=Texture2d::empty_with_format(
            display,
            UncompressedFloatFormat::U8,
            MipmapsOption::NoMipmap,
            width as u32,
            height
        ).unwrap();

        let rect=Rect{
            left:0,
            bottom:0,
            width:width as u32,
            height:height,
        };

        let raw_image=RawImage2d{
            data:Cow::Borrowed(&image),
            width:width as u32,
            height:height,
            format:ClientFormat::U8,
        };

        texture.write(rect,raw_image);

        let advance_width=face.glyph_hor_advance(id).unwrap() as f32*scale.horizontal;

        let glyph=RawGlyph::<Texture2d>::raw(
            texture,
            size,
            offset,
            advance_width,
        );

        Some(glyph)
    }
    else{
        None
    }
}


/// Типаж для определения хранилищ глифов.
/// 
/// A trait for defining glyph cache.
pub trait RawGlyphCache{
    /// Возращает немасштабированный глиф.
    /// 
    /// Returns an unscaled glyph.
    fn raw_glyph(&self,character:char)->Option<&RawGlyph<Texture2d>>;

    /// Возращает немасштабированный глиф неопределённого символа.
    /// 
    /// Returns an unscaled glyph of the undefined character.
    fn raw_undefined_glyph(&self)->&RawGlyph<Texture2d>;

    /// Возращает немасштабированный глиф для данного или неопределённого символа.
    /// 
    /// Returns an unscaled glyph of the given character or of the undefined one.
    fn raw_glyph_or_undefined(&self,character:char)->&RawGlyph<Texture2d>{
        if let Some(glyph)=self.raw_glyph(character){
            glyph
        }
        else{
            self.raw_undefined_glyph()
        }
    }

    /// Возращает масштабированный глиф.
    /// 
    /// Returns a scaled glyph.
    fn scaled_glyph(&self,character:char,scale:Scale)->Option<ScaledGlyph<Texture2d>>{
        if let Some(glyph)=self.raw_glyph(character){
            Some(glyph.scale(scale))
        }
        else{
            None
        }
    }

    /// Возращает масштабированный глиф неопределённого символа.
    /// 
    /// Returns the scaled glyph of the undefined character.
    fn scaled_undefined_glyph(&self,scale:Scale)->ScaledGlyph<Texture2d>{
        self.raw_undefined_glyph().scale(scale)
    }

    /// Возращает масштабированный глиф для данного или неопределённого символа.
    /// 
    /// Returns the scaled glyph of the given character or of the undefined one.
    fn scaled_glyph_or_undefined(&self,character:char,scale:Scale)->ScaledGlyph<Texture2d>{
        if let Some(glyph)=self.raw_glyph(character){
            glyph.scale(scale)
        }
        else{
            self.raw_undefined_glyph().scale(scale)
        }
    }
}