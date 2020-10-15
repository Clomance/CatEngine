use super::{
    OutlineCurveBuilder,
    Glyph,
    OutlinedGlyph,
    Scale,
    RawGlyph,
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

use ab_glyph_rasterizer::point;

use std::{
    collections::HashMap,
    borrow::Cow,
    ops::Range,
    path::Path,
    fs::read,
};

/// Хранит данные шрифта.
/// Должен быть несдвигаемым в памяти,
/// чтобы не сломать ссылку
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

        let face=Face::from_slice(slice,0).unwrap();

        font.face=Some(face);

        Some(font)
    }
}





/// Хранилище для шрифта.
/// 
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

impl<'a> FaceWrapper<'a>{
    /// Строит глиф для данного символа.
    /// 
    /// Builds a glyph for the given character.
    pub fn glyph(&self,character:char)->Option<Glyph>{
        // Поиск глифа
        if let Some(glyph_id)=self.0.glyph_index(character){
            let mut outline_builder=OutlineCurveBuilder::default();
            // Получение точек для построения глифа
            if let Some(bounding_box)=self.0.outline_glyph(glyph_id,&mut outline_builder){
                // Высота для выравнивания
                let global_height=self.0.ascender() as f32;

                let glyph_size=[
                    bounding_box.width() as f32,
                    bounding_box.height() as f32,
                ];

                let glyph_offset=[
                    bounding_box.x_min as f32,
                    bounding_box.y_min as f32,
                ];

                // Мастабирование под размер шрифта (относительно самого высокого символа)
                let scale=glyph_size[1]/global_height;

                // Ширина до следующего символа
                let advance_width=self.0.glyph_hor_advance(glyph_id).unwrap() as f32;

                let glyph=Glyph::new(
                    glyph_offset,
                    glyph_size,
                    advance_width,
                    scale,
                    outline_builder.outline
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

    /// Строит глиф для неопределённого символа.
    /// 
    /// Builds a glyph for the undefined character.
    pub fn undefined_glyph(&self)->Glyph{
        let glyph_id=GlyphId(0);

        let mut outline_builder=OutlineCurveBuilder::default();

        if let Some(bounding_box)=self.0.outline_glyph(glyph_id,&mut outline_builder){
            // Высота для выравнивания
            let global_height=self.0.ascender() as f32;

            let glyph_size=[
                bounding_box.width() as f32,
                bounding_box.height() as f32,
            ];

            let glyph_offset=[
                bounding_box.x_min as f32,
                bounding_box.y_min as f32,
            ];

            // Мастабирование под размер шрифта (относительно самого высокого символа)
            let scale=glyph_size[1]/global_height;

            // Ширина до следующего символа
            let advance_width=self.0.glyph_hor_advance(glyph_id).unwrap() as f32;

            Glyph::new(
                glyph_offset,
                glyph_size,
                advance_width,
                scale,
                outline_builder.outline
            )
        }
        else{
            // unreachable... maybe :)
            panic!("No undefined glyph");
        }
    }

    /// Возвращает ширину пробела.
    /// 
    /// Returns the whitespace advance.
    pub fn whitespace_advance(&self,font_size:f32)->f32{
        // TODO: доделать
        font_size 
    }
}





/// Хранилище глифов.
/// 
/// A glyph cache.
// ᶠᵉᵉᵈ ᵐᵉ /ᐠ-ⱉ-ᐟ\ﾉ
pub struct GlyphCache{
    glyphs:HashMap<char,RawGlyph>,
    undefined_glyph:RawGlyph,
    // Коэффициент размера пробела - whitespace_advance = self.whitespace_advance * font_size
    whitespace_advance:f32,
}

impl GlyphCache{
    /// Создаёт новое хранилище глифов для данной области номеров символов.
    /// 
    /// Игнорирует неопределённые символы.
    /// 
    /// range = None - использует все символы шрифта.
    /// 
    /// Creates a new glyph cache with the given range of characters ids.
    /// 
    /// Ignors undefined characters.
    /// 
    /// range = None - takes all characters of the font.
    pub fn new(font:&Face,range:Option<Range<u16>>,scale:Scale,display:&Display)->GlyphCache{
        let len=font.number_of_glyphs();

        let space=font.glyph_hor_advance(GlyphId(3)).unwrap() as f32;

        let mut glyphs=HashMap::with_capacity(len as usize);

        let global_height=font.ascender() as f32*scale.vertical;

        // Неопределённый символ
        let not_defined_id=GlyphId(0);
        let undefined_glyph=build_glyph(not_defined_id,global_height,scale,&font,display).unwrap();

        for g in 1..len{
            let id=GlyphId(g);

            if let Some(glyph)=build_glyph(id,global_height,scale,&font,display){
                let character=unsafe{std::char::from_u32_unchecked(g as u32)};
                glyphs.insert(character,glyph);
            }
        }

        Self{
            glyphs,
            undefined_glyph,
            whitespace_advance:space,
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

        let space=font.glyph_hor_advance(GlyphId(3)).unwrap() as f32;

        let global_width=font.global_bounding_box().width() as f32;

        let space_advance=space/global_width;

        // The maximal height of all glyphs.
        let global_height=font.ascender() as f32*scale.vertical;

        // Неопределённый символ
        let not_defined_id=GlyphId(0);
        let undefined_glyph=build_glyph(not_defined_id,global_height,scale,&font,display).unwrap();


        for character in alphabet.chars(){
            let id=font.glyph_index(character).unwrap();

            if let Some(glyph)=build_glyph(id,global_height,scale,&font,display){
                glyphs.insert(character,glyph);
            }
        }

        Self{
            glyphs,
            undefined_glyph,
            whitespace_advance:space_advance,
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
            let global_height=font.ascender() as f32*scale.vertical;

            if let Some(glyph)=build_glyph(id,global_height,scale,font,display){
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

    /// Возращает немасштабированный глиф.
    /// 
    /// Returns an unscaled glyph.
    #[inline(always)]
    pub fn glyph(&self,character:char)->Option<&RawGlyph>{
        self.glyphs.get(&character)
    }

    /// Возращает немасштабированный глиф для данного или неопределённого символа.
    /// 
    /// Returns an unscaled glyph of the given character or of the undefined one.
    pub fn glyph_or_undefined(&self,character:char)->&RawGlyph{
        if let Some(glyph)=self.glyphs.get(&character){
            glyph
        }
        else{
            &self.undefined_glyph
        }
    }

    /// Возращает немасштабированный глиф неопределённого символа.
    /// 
    /// Returns an unscaled glyph of the undefined character.
    #[inline(always)]
    pub fn undefined_glyph(&self)->&RawGlyph{
        &self.undefined_glyph
    }

    pub fn text_width(&self,text:&str,font_size:f32)->f32{
        let mut width=0f32;
        for character in text.chars(){
            if let Some(glyph)=self.glyph(character){
                let glyph_size=glyph.height_advance(font_size);
                width+=glyph_size[0];
            }
            else{
                if character==' '{
                    width+=self.whitespace_advance(font_size);
                    continue
                }
                let glyph_size=self.undefined_glyph.height_advance(font_size);
                width+=glyph_size[0];
            }
        }
        width
    }

    #[inline(always)]
    pub fn whitespace_advance(&self,font_size:f32)->f32{
        self.whitespace_advance*font_size
    }

    pub fn text_size(&self,text:&str,font_size:f32)->[f32;2]{
        let mut size=[0f32;2];
        for character in text.chars(){
            if let Some(glyph)=self.glyph(character){
                let glyph_size=glyph.height_advance(font_size);
                if glyph_size[1]>size[1]{
                    size[1]=glyph_size[1];
                }
                size[0]+=glyph_size[0];
            }
            else{
                size[0]+=self.whitespace_advance(font_size);
            }
        }

        size
    }
}

fn build_glyph(id:GlyphId,global_height:f32,scale:Scale,face:&Face,display:&Display)->Option<RawGlyph>{
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

        let mut len=width*height as usize;
        let mut image=Vec::with_capacity(len);

        glyph.draw(|c,a|{
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

        // Собственное мастабирование для символов
        let glyph_scale=size[1]/global_height;

        let advance=face.glyph_hor_advance(id).unwrap() as f32*scale.horizontal;

        let glyph=RawGlyph::raw(
            texture,
            size,
            offset,
            advance,
            glyph_scale
        );

        Some(glyph)
    }
    else{
        None
    }
}