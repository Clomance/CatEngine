//! # Рендеринг текста. Text rendering. `feature = "text_graphics"`, `default_features`
//! 
//! Как рендерится символ:
//! 1. С помощью библиотеки `rusttype` создаётся символ (или уже даётся готовый)
//! 2. Этот символ записывается в массив как изображение
//! 3. Изображение загружается в текстуру
//! 4. Текстура выводится на экран
//! 
//! ###
//! 
//! How a sign is rendering:
//! 1. A glyph is built with the `rusttype` crate (or is given the ready one)
//! 2. The glyph is written to the array as an image
//! 3. The image is loaded to the texture
//! 4. The texture is rendered
//! 
//! ```
//! let mut window=PagedWindow::new(|_,s|{
//!     s.vsync=true;
//! }).unwrap();
//! 
//! let font=FontOwner::load("resources/font1").unwrap();
//! 
//! let scale=Scale::new(0.4,0.4);
//! // Creating a new glyph cache for the given characters
//! let glyphs=GlyphCache::new_alphabet(&font,"HelloWorld?",scale,window.display());
//! ```

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

use crate::{
    // types
    Colour,
    // structs
    graphics::Graphics
};

mod glyph;
pub use glyph::*;

mod outline;
pub (crate) use outline::{
    OutlineCurve,
    OutlineCurveBuilder,
};

pub use outline::{
    Scale,
};

mod font;
pub use font::{FontOwner,FaceWrapper,GlyphCache};


use glium::{
    DrawParameters,
    DrawError,
    texture::{
        Texture2d,
        RawImage2d,
        ClientFormat,
    },
    Display,
    Rect,
};

// re-export
pub use ttf_parser;
pub use ab_glyph_rasterizer;

use std::{
    fs::read,
    path::Path,
    collections::HashMap,
    borrow::Cow,
};

/// Основа для рендеринга текста.
/// 
/// A base for text rendering.
pub struct TextBase{
    pub position:[f32;2],
    pub font_size:f32,
    pub colour:Colour,
}

impl TextBase{
    pub const fn new(position:[f32;2],font_size:f32,colour:Colour)->TextBase{
        Self{
            font_size,
            colour,
            position,
        }
    }

    pub const fn zero_position(font_size:f32,colour:Colour)->TextBase{
        Self{
            font_size,
            colour,
            position:[0f32;2],
        }
    }

    #[inline(always)]
    pub fn set_x(&mut self,x:f32){
        self.position[0]=x
    }

    #[inline(always)]
    pub fn set_y(&mut self,y:f32){
        self.position[1]=y
    }

    #[inline(always)]
    pub fn move_to(&mut self,position:[f32;2]){
        self.position=position
    }

    #[inline(always)]
    pub fn shift_x(&mut self,dx:f32){
        self.position[0]+=dx
    }

    #[inline(always)]
    pub fn shift_y(&mut self,dy:f32){
        self.position[1]+=dy
    }

    #[inline(always)]
    pub fn shift(&mut self,dx:f32,dy:f32){
        self.position[0]+=dx;
        self.position[1]+=dy;
    }

    #[inline(always)]
    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.colour[3]=alpha
    }

    #[inline(always)]
    pub fn set_colour(&mut self,colour:Colour){
        self.colour=colour
    }
}

/// Рендеринг символов.
/// 
/// Character rendering.
impl TextBase{
    /// Выводит символ.
    /// 
    /// Draws a character.
    #[inline(always)]
    pub fn draw_char(
        &self,
        character:char,
        font:&FaceWrapper,
        draw_parameters:&DrawParameters,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        let glyph=if let Some(glyph)=font.glyph(character){
            glyph
        }
        else{
            if character.is_whitespace(){
                return Ok(())
            }
            else{
                font.undefined_glyph()
            }
        };

        let outlined=glyph.outlined_glyph(self.font_size);

        graphics.draw_glyph(
            &outlined,
            self.colour,
            self.position,
            draw_parameters
        )
    }

    /// Выводит сдвинутый символ.
    /// 
    /// Draws a shifted character.
    #[inline(always)]
    pub fn draw_shift_char(
        &self,
        character:char,
        font:&FaceWrapper,
        shift:[f32;2],
        draw_parameters:&DrawParameters,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        let glyph=if let Some(glyph)=font.glyph(character){
            glyph
        }
        else{
            if character.is_whitespace(){
                return Ok(())
            }
            else{
                font.undefined_glyph()
            }
        };

        let outlined=glyph.outlined_glyph(self.font_size);

        graphics.draw_shift_glyph(
            &outlined,
            self.colour,
            self.position,
            shift,
            draw_parameters
        )
    }

    /// Выводит повёрнутый символ.
    /// 
    /// Draws a rotated character.
    #[inline(always)]
    pub fn draw_rotate_char(
        &self,
        character:char,
        font:&FaceWrapper,
        rotation_center:[f32;2],
        angle:f32,
        draw_parameters:&DrawParameters,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        let glyph=if let Some(glyph)=font.glyph(character){
            glyph
        }
        else{
            if character.is_whitespace(){
                return Ok(())
            }
            else{
                font.undefined_glyph()
            }
        };

        let outlined=glyph.outlined_glyph(self.font_size);

        graphics.draw_rotate_glyph(
            &outlined,
            self.colour,
            self.position,
            rotation_center,
            angle,
            draw_parameters
        )
    }

    /// Строит и выводит один символ.
    /// 
    /// Берёт соответствующий глиф из данного хранилища.
    /// 
    /// Builds and draws a character.
    /// 
    /// Takes a corresponding glyph from the given cache.
    #[inline(always)]
    pub fn draw_char_glyph_cache(
        &self,
        character:char,
        glyph_cache:&GlyphCache,
        draw_parameters:&DrawParameters,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        let glyph=if let Some(glyph)=glyph_cache.glyph(character){
            glyph
        }
        else{
            if character.is_whitespace(){
                return Ok(())
            }
            else{
                glyph_cache.undefined_glyph()
            }
        };

        let frame=glyph.frame(self.font_size);

        let mut rect=frame.bounding_box(self.position);

        graphics.draw_glyph_cache(
            glyph,
            self.colour,
            rect,
            draw_parameters
        )
    }

    /// Выводит сдвинутый символ.
    /// 
    /// Берёт соответствующий глиф из данного хранилища.
    /// 
    /// Draws a shifted character.
    /// 
    /// Takes a corresponding glyph from the given cache.
    #[inline(always)]
    pub fn draw_shift_char_glyph_cache(
        &self,
        character:char,
        shift:[f32;2],
        glyph_cache:&GlyphCache,
        draw_parameters:&DrawParameters,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        let glyph=if let Some(glyph)=glyph_cache.glyph(character){
            glyph
        }
        else{
            if character.is_whitespace(){
                return Ok(())
            }
            else{
                glyph_cache.undefined_glyph()
            }
        };

        let frame=glyph.frame(self.font_size);

        let mut rect=frame.bounding_box(self.position);

        graphics.draw_shift_glyph_cache(
            glyph,
            self.colour,
            rect,
            shift,
            draw_parameters
        )
    }

    /// Выводит символ.
    /// 
    /// Берёт соответствующий глиф из данного хранилища.
    /// 
    /// Draws a character.
    /// 
    /// Takes a corresponding glyph from the given cache.
    #[inline(always)]
    pub fn draw_rotate_char_glyph_cache(
        &self,
        character:char,
        rotation_center:[f32;2],
        angle:f32,
        glyph_cache:&GlyphCache,
        draw_parameters:&DrawParameters,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        let glyph=if let Some(glyph)=glyph_cache.glyph(character){
            glyph
        }
        else{
            if character.is_whitespace(){
                return Ok(())
            }
            else{
                glyph_cache.undefined_glyph()
            }
        };

        let frame=glyph.frame(self.font_size);

        let mut rect=frame.bounding_box(self.position);

        graphics.draw_rotate_glyph_cache(
            glyph,
            self.colour,
            rect,
            rotation_center,
            angle,
            draw_parameters
        )
    }
}

/// Рендеринг текста.
/// 
/// Text rendering.
impl TextBase{
    /// Выводит строку.
    /// 
    /// Draws a string.
    pub fn draw_str(
        &self,
        s:&str,
        font:&FaceWrapper,
        draw_parameters:&DrawParameters,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        let mut position=self.position;

        let mut glyph;

        let whitespace_advance=font.whitespace_advance(self.font_size);

        for character in s.chars(){
            glyph=if let Some(glyph)=font.glyph(character){
                glyph
            }
            else{
                if character==' '{
                    position[0]+=whitespace_advance;
                    continue
                }

                font.undefined_glyph()
            };

            let advance_width=glyph.advance_width(self.font_size);

            let outlined=glyph.outlined_glyph(self.font_size);

            graphics.draw_glyph(
                &outlined,
                self.colour,
                position,
                draw_parameters
            )?;

            position[0]+=advance_width;
        }

        Ok(())
    }

    /// Выводит строку.
    /// 
    /// Берёт соответствующие глифы из данного хранилища.
    /// 
    /// Draws a string.
    /// 
    /// Takes corresponding glyphs from the given cache.
    pub fn draw_str_glyph_cache(
        &self,
        s:&str,
        glyph_cache:&GlyphCache,
        draw_parameters:&DrawParameters,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        let mut position=self.position;

        let mut glyph;

        for character in s.chars(){
            glyph=if let Some(glyph)=glyph_cache.glyph(character){
                glyph
            }
            else{
                if character==' '{
                    position[0]+=glyph_cache.whitespace_advance(self.font_size);
                    continue
                }

                glyph_cache.undefined_glyph()
            };

            let frame=glyph.frame(self.font_size);

            let mut rect=frame.bounding_box(position);

            graphics.draw_glyph_cache(
                glyph,
                self.colour,
                rect,
                draw_parameters
            )?;

            position[0]+=frame.advance;
        }

        Ok(())
    }

    /// Выводит повёрнутую строку.
    /// 
    /// Берёт соответствующие глифы из данного хранилища.
    /// 
    /// Draws a rotated string.
    /// 
    /// Takes corresponding glyphs from the given cache.
    pub fn draw_rotate_str_glyph_cache(
        &self,
        s:&str,
        rotation_center:[f32;2],
        angle:f32,
        glyph_cache:&GlyphCache,
        draw_parameters:&DrawParameters,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        let mut position=self.position;

        let mut glyph;

        for character in s.chars(){
            glyph=if let Some(glyph)=glyph_cache.glyph(character){
                glyph
            }
            else{
                if character==' '{
                    position[0]+=glyph_cache.whitespace_advance(self.font_size);
                    continue
                }

                glyph_cache.undefined_glyph()
            };

            let frame=glyph.frame(self.font_size);

            let mut rect=frame.bounding_box(position);

            graphics.draw_rotate_glyph_cache(
                glyph,
                self.colour,
                rect,
                rotation_center,
                angle,
                draw_parameters
            )?;

            position[0]+=frame.advance;

            
        }

        Ok(())
    }

    /// Выводит часть строки.
    /// Если текст выведен полностью, возвращает `true`.
    /// 
    /// Берёт соответствующие глифы из данного хранилища.
    /// 
    /// Draws a part of a string.
    /// Returns `true`, if the whole string is drawn.
    /// 
    /// Takes corresponding glyphs from the given cache.
    pub fn draw_str_part_glyph_cache(
        &self,
        s:&str,
        chars:usize,
        glyph_cache:&GlyphCache,
        draw_parameters:&DrawParameters,
        graphics:&mut Graphics
    )->Result<bool,DrawError>{
        let mut whole=true; // Флаг вывода всего текста

        let mut position=self.position;

        let mut glyph;

        for (i,character) in s.chars().enumerate(){
            // Выход из цикла при достижении лимита символов
            if i==chars{
                whole=false;
                break
            }

            glyph=if let Some(glyph)=glyph_cache.glyph(character){
                glyph
            }
            else{
                if character==' '{
                    position[0]+=glyph_cache.whitespace_advance(self.font_size);
                    continue
                }

                glyph_cache.undefined_glyph()
            };

            let frame=glyph.frame(self.font_size);


            let mut rect=frame.bounding_box(position);

            graphics.draw_glyph_cache(
                glyph,
                self.colour,
                rect,
                draw_parameters
            )?;

            position[0]+=frame.advance;
        }

        Ok(whole)
    }

    /// Выводит часть повёрнутой строки.
    /// Если текст выведен полностью, возвращает true.
    /// 
    /// Берёт соответствующие глифы из данного хранилища.
    /// 
    /// Draws a part of a rotated string.
    /// Returns true, if the whole string is drawn.
    /// 
    /// Takes corresponding glyphs from the given cache.
    pub fn draw_rotate_str_part_glyph_cache(
        &self,
        s:&str,
        chars:usize,
        rotation_center:[f32;2],
        angle:f32,
        glyph_cache:&GlyphCache,
        draw_parameters:&DrawParameters,
        graphics:&mut Graphics
    )->Result<bool,DrawError>{
        let mut whole=true; // Флаг вывода всего текста

        let mut position=self.position;

        let mut glyph;

        for (i,character) in s.chars().enumerate(){
            // Выход из цикла при достижении лимита символов
            if i==chars{
                whole=false;
                break
            }

            glyph=if let Some(glyph)=glyph_cache.glyph(character){
                glyph
            }
            else{
                if character==' '{
                    position[0]+=glyph_cache.whitespace_advance(self.font_size);
                    continue
                }

                glyph_cache.undefined_glyph()
            };

            let frame=glyph.frame(self.font_size);


            let mut rect=frame.bounding_box(position);

            graphics.draw_rotate_glyph_cache(
                glyph,
                self.colour,
                rect,
                rotation_center,
                angle,
                draw_parameters
            )?;

            position[0]+=frame.advance;
        }

        Ok(whole)
    }
}