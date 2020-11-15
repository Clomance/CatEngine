use super::{
    OutlineCurveBuilder,
    RawGlyph,
    ScaledGlyph,
    OutlinedGlyph,
    Scale,
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
};

// ᶠᵉᵉᵈ ᵐᵉ /ᐠ-ⱉ-ᐟ\ﾉ
/// Хранилище глифов.
/// A glyph cache.
pub struct GlyphCache{
    glyphs:HashMap<char,RawGlyph<Texture2d>>,
    undefined_glyph:RawGlyph<Texture2d>,
    // Немасштабированная ширина пробела
    whitespace_advance:f32,
    bounding_size:[f32;2],
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
    /// Ignores undefined characters.
    /// 
    /// range = `None` - takes all characters of the font.
    pub fn new(font:&Face,range:Option<Range<u16>>,scale:Scale,display:&Display)->GlyphCache{
        let global_box=font.global_bounding_box();

        let bounding_size=[
            global_box.width() as f32*scale.horizontal,
            global_box.height() as f32*scale.vertical,
        ];

        // Неопределённый символ
        let not_defined_id=GlyphId(0);
        let undefined_glyph=build_glyph(not_defined_id,scale,&font,display).unwrap();

        // Ширина пробела
        let whitespace_advance_id=GlyphId(3);
        let whitespace_advance=font.glyph_hor_advance(whitespace_advance_id).unwrap() as f32*scale.horizontal;

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
            whitespace_advance,
            bounding_size,
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
        let global_box=font.global_bounding_box();

        let bounding_size=[
            global_box.width() as f32*scale.horizontal,
            global_box.height() as f32*scale.vertical,
        ];

        let mut glyphs=HashMap::with_capacity(alphabet.len());

        // Неопределённый символ
        let not_defined_id=GlyphId(0);
        let undefined_glyph=build_glyph(not_defined_id,scale,&font,display).unwrap();

        // Ширина пробела
        let whitespace_advance_id=GlyphId(3);
        let whitespace_advance=font.glyph_hor_advance(whitespace_advance_id).unwrap() as f32*scale.horizontal;

        for character in alphabet.chars(){
            let id=font.glyph_index(character).unwrap();

            if let Some(glyph)=build_glyph(id,scale,&font,display){
                glyphs.insert(character,glyph);
            }
        }

        Self{
            glyphs,
            undefined_glyph,
            whitespace_advance,
            bounding_size
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
    pub fn bounding_size(&self)->[f32; 2]{
        self.bounding_size
    }
}

impl RawGlyphCache for GlyphCache{
    #[inline(always)]
    fn whitespace_advance_width(&self,horizontal_scale:f32)->f32{
        self.whitespace_advance*horizontal_scale
    }

    #[inline(always)]
    fn raw_glyph(&self,character:char)->Option<&RawGlyph<Texture2d>>{
        self.glyphs.get(&character)
    }

    #[inline(always)]
    fn raw_undefined_glyph(&self)->&RawGlyph<Texture2d>{
        &self.undefined_glyph
    }

    fn scale_for_height(&self, height:f32)->Scale{
        let height0 = self.bounding_size[1];
        let k = height/height0;
        Scale::new(k, k)
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

        let glyph=OutlinedGlyph::raw(outline_builder.outline,offset,size,scale);

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
    /// Returns an unscaled undefined character glyph.
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

    fn text_width(&self,text:&str,scale:Scale)->f32{
        let mut width=0f32;
        for character in text.chars(){
            width+=if let Some(glyph)=self.scaled_glyph(character,scale){
                let advance_width=glyph.advance_width();
                advance_width
            }
            else{
                if character==' '{
                    self.whitespace_advance_width(scale.horizontal)
                }
                else{
                    self.scaled_undefined_glyph(scale).advance_width()
                }
            }
        }
        width
    }

    fn text_size(&self,text:&str,scale:Scale)->[f32;2]{
        let mut size=[0f32;2];
        for character in text.chars(){
            if let Some(glyph)=self.scaled_glyph(character,scale){
                let glyph_size=glyph.size();
                if glyph_size[1] as f32>size[1]{
                    size[1]=glyph_size[1] as f32;
                }
                size[0]+=glyph.advance_width();
            }
            else{
                if character==' '{
                    size[0]+=self.whitespace_advance_width(scale.horizontal);
                }
                else{
                    let glyph=self.scaled_undefined_glyph(scale);
                    let glyph_size=glyph.size();
                    if glyph_size[1] as f32>size[1]{
                        size[1]=glyph_size[1] as f32;
                    }
                    size[0]+=glyph.advance_width();
                }
            }
        }

        size
    }

    /// Возращает масштабированную ширину пробела.
    /// 
    /// Returns whitespace's scaled width.
    fn whitespace_advance_width(&self,horizontal_scale:f32)->f32;


    fn scale_for_height(&self,height:f32)->Scale;
}