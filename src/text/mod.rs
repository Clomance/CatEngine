use crate::{
    // types
    Colour,
    // structs
    graphics::Graphics
};

use glium::{DrawParameters,DrawError};

pub use rusttype; // re-export

use rusttype::{
    Font,
    Scale,
    Point,
    PositionedGlyph,
};

use std::{
    fs::read,
    path::Path
};

/// Загружает шрифт.
/// Loads a font.
pub fn load_font<P:AsRef<Path>>(path:P)->Option<Font<'static>>{
    if let Ok(data)=read(path){
        Font::try_from_vec(data)
    }
    else{
        None
    }
}

pub fn char_width(character:char,font_size:f32,font:&Font)->f32{
    let scale=Scale::uniform(font_size);

    let glyph=font.glyph(character).scaled(scale);

    glyph.h_metrics().advance_width
}

pub fn char_height(character:char,font_size:f32,font:&Font)->f32{
    let scale=Scale::uniform(font_size);
    let point=Point{
        x:0f32,
        y:0f32,
    };

    let glyph=font.glyph(character).scaled(scale).positioned(point);

    if let Some(bounding_box)=glyph.pixel_bounding_box(){
        bounding_box.height() as f32
    }
    else{
        0f32
    }
}

pub fn char_size(character:char,font_size:f32,font:&Font)->[f32;2]{
    let scale=Scale::uniform(font_size);
    let point=Point{
        x:0f32,
        y:0f32,
    };

    let v_metrics=font.v_metrics(scale);
    v_metrics.ascent-v_metrics.descent;

    let glyph=font.glyph(character).scaled(scale);
    let width=glyph.h_metrics().advance_width;

    let height=if let Some(bounding_box)=glyph.positioned(point).pixel_bounding_box(){
        bounding_box.height() as f32
    }
    else{
        0f32
    };

    [width,height]
}

/// Расчитывает ширину текста.
/// Calculates the text width.
pub fn text_width(text:&str,font_size:f32,font:&Font)->f32{
    let scale=Scale::uniform(font_size);
    let mut width=0f32;
    for c in text.chars(){
        let glyph=font.glyph(c).scaled(scale);
        width+=glyph.h_metrics().advance_width;
    }

    width
}

/// Расчитывает высоту текста.
/// Calculates the height of the text.
pub fn text_height(text:&str,font_size:f32,font:&Font)->f32{
    let scale=Scale::uniform(font_size);
    let point=Point{
        x:0f32,
        y:0f32,
    };

    let mut height=0f32;

    for c in text.chars(){
        let glyph=font.glyph(c).scaled(scale).positioned(point);
        if let Some(bounding_box)=glyph.pixel_bounding_box(){
            let glyph_height=bounding_box.height() as f32;
            if height<glyph_height{
                height=glyph_height;
            }
        }
    }

    height
}

/// Расчитывает и возвращает ширину и высоту текста.
/// Calculates and returns the width and the height of the text.
/// [width, height]
pub fn text_size(text:&str,font_size:f32,font:&Font)->[f32;2]{
    let scale=Scale::uniform(font_size);
    let point=Point{
        x:0f32,
        y:0f32,
    };
    let mut width=0f32;
    let mut height=0f32;
    for c in text.chars(){
        let glyph=font.glyph(c).scaled(scale);
        width+=glyph.h_metrics().advance_width;
        let glyph=glyph.positioned(point);
        if let Some(bounding_box)=glyph.pixel_bounding_box(){
            let glyph_height=bounding_box.height() as f32;
            if height<glyph_height{
                height=glyph_height;
            }
        }
    }

    [width,height]
}

/// Основа для рендеринга текста.
/// 
/// A base for  textrendering.
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
    pub fn set_position(&mut self,position:[f32;2]){
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

    /// Выводит уже готовый символ.
    /// 
    /// Draws an already built glyph.
    #[inline(always)]
    pub fn draw_glyph(
        &self,
        glyph:PositionedGlyph,
        draw_parameters:&mut DrawParameters,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        graphics.draw_glyph(glyph,self.colour,draw_parameters)
    }

    /// Строит и выводит один символ.
    /// 
    /// Builds and draws a character.
    #[inline(always)]
    pub fn draw_char(
        &self,
        character:char,
        font:&Font,
        draw_parameters:&mut DrawParameters,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        graphics.draw_char(character,self,font,draw_parameters)
    }

    /// Выводит строку.
    /// 
    /// Draws a string.
    #[inline(always)]
    pub fn draw_str(
        &self,
        s:&str,
        font:&Font,
        draw_parameters:&mut DrawParameters,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        graphics.draw_str(s,self,font,draw_parameters)
    }

    /// Выводит часть строки.
    /// Если текст выведен полностью, возвращает true.
    /// 
    /// Draws a part of the string.
    /// Returns true, if the whole string was drawn.
    pub fn draw_str_part(
        &self,
        s:&str,
        chars:usize,
        font:&Font,
        draw_parameters:&mut DrawParameters,
        graphics:&mut Graphics
    )->Result<bool,DrawError>{
        let mut whole=true; // Флаг вывода всего текста

        let scale=Scale::uniform(self.font_size);

        // позиция для вывода символа
        let mut point=Point{
            x:self.position[0],
            y:self.position[1]
        };

        let mut width_offset; // сдвиг для следующего символа

        for (i,character) in s.chars().enumerate(){
            if i==chars{
                whole=false;
                break
            }
            // Получение символа
            let scaled_glyph=font.glyph(character).scaled(scale);

            // ширина символа
            width_offset=scaled_glyph.h_metrics().advance_width;

            // символ с позицией
            let glyph=scaled_glyph.positioned(point);

            graphics.draw_glyph(glyph,self.colour,draw_parameters)?;

            point.x+=width_offset;
        }

        Ok(whole)
    }
}