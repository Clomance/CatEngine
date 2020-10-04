use super::{
    OutlineCurveBuilder,
    OutlinedGlyph,
    Scale,
    outline::Rect as TRect,
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
    GlyphId,
    Face,
};

use ab_glyph_rasterizer::point;

use std::{
    fs::read,
    path::Path,
    collections::HashMap,
    borrow::Cow,
    ops::Range,
};


// Определения \\

// Глиф (glyph) - здесь изображение (текстура) символа.

// Ascender (выносной элемент) - в типографике часть строчной буквы,
// выходящая за пределы линии строчных знаков или базовой линии шрифта.
// Здесь расстояние от строки до верхней границы этой части.
// Примеры: загагулина у буквы f, палочка у букв h и b, крышка у буквы А.

// Размер шрифта (font size) - здесь высота,
// под которую выравниваются все текстуры символов.
// Таким будет размер самого большого глифа при рендеринге.
// Но определить точный размер этого глифа не всегда удобно, поэтому
// чаще всего все символы будут чуть меньше. Используйте функцию `text_size`
// для точного определения точных размеров текста.

// Хранилище \\

// Все символы хранятся в вместе с глифами в хранилище (`GlyphCache`).
// Для каждого символа создаётся текстура и в неё загружается глиф.
// Поиск глифов по символам выполняется с помощью функций `HashMap`.

// Построение глифа \\

// Сначала определяется максимальный размер символа у шрифта -
// под него выравниваются все остальные символы.

// Размер глифа при рендеринге определяется по следующей формуле.
// ```
// glyph_render_size = glyph_size / (glyph_global_height) * font_size
// ```

// В используется значение `ascender`.


/// Хранилище глифов.
/// 
/// A glyph cache.
pub struct GlyphCache{
    glyphs:HashMap<char,RawGlyph>,
    undefined_glyph:RawGlyph,
    // Коэффициент размера пробела - whitespace advance = self.whitespace advance * font size
    whitespace_advance:f32,
}

impl GlyphCache{
    /// range = None - loads whole font
    pub fn new(font:&Face,range:Option<Range<u16>>,scale:Scale,display:&Display)->GlyphCache{
        let len=font.number_of_glyphs();

        let space=font.glyph_hor_advance(GlyphId(3)).unwrap() as f32;

        let mut glyphs=HashMap::with_capacity(len as usize);

        let global_height=font.ascender() as f32*scale.vertical;

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

    pub fn new_alphabet(font:&Face,alphabet:&str,scale:Scale,display:&Display)->GlyphCache{
        let mut glyphs=HashMap::with_capacity(alphabet.len());

        let space=font.glyph_hor_advance(GlyphId(3)).unwrap() as f32;

        let global_width=font.global_bounding_box().width() as f32;

        let space_advance=space/global_width;

        // The maximal height of all glyphs.
        let global_height=font.ascender() as f32*scale.vertical;

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

    pub fn insert_char(&mut self,character:char,font:&Face,scale:Scale,display:&Display)->bool{
        if let Some(id)=font.glyph_index(character){
            let global_height=font.ascender() as f32*scale.vertical;

            if let Some(glyph)=build_glyph(id,global_height,scale,font,display){
                self.glyphs.insert(character,glyph);
                true
            }
            else{
                false
            }
        }
        else{
            false
        }
    }

    pub fn insert_str(&mut self,font:&Face,alphabet:&str,scale:Scale,display:&Display){
        for character in alphabet.chars(){
            self.insert_char(character,font,scale,display);
        }
    }

    /// Возращает немасштабированный глиф.
    /// 
    /// Returns a unscaled glyph.
    #[inline(always)]
    pub fn glyph(&self,character:char)->Option<&RawGlyph>{
        self.glyphs.get(&character)
    }

    /// Возращает немасштабированный глиф.
    /// 
    /// Returns a unscaled glyph.
    pub fn glyph_or_undefined(&self,character:char)->&RawGlyph{
        if let Some(glyph)=self.glyphs.get(&character){
            glyph
        }
        else{
            &self.undefined_glyph
        }
    }

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

    /// Returns the height and the width of the text.
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

        let rect=TRect::new(
            offset[0],
            offset[1],
            size[0],
            size[1]
        );

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