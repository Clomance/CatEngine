use super::{
    OutlineCurve,
    Scale,
};

use glium::{
    texture::Texture2d,
};

use ab_glyph_rasterizer::{Point,point,Rasterizer};

/// Немасштабированный глиф.
/// An unscaled glyph.
pub struct RawGlyph<T>{
    // Данные изображения символа:
    // текстура либо контур
    data:T,
    // Размер
    size:[f32;2],
    // Сдвиг
    offset:[f32;2],
    // Горизонтальное расстояние до следующего глифа
    advance_width:f32,
}


/// Общие функции для немасштабированных глифов.
/// General functions for unscaled glyphs.
impl<T> RawGlyph<T>{
    #[inline(always)]
    pub const fn raw(
        data:T,
        size:[f32;2],
        offset:[f32;2],
        advance_width:f32,
    )->RawGlyph<T>{
        Self{
            data,
            size,
            offset,
            advance_width,
        }
    }

    /// Returns glyph's scaled offset (X and Y in a bounding box).
    pub fn offset(&self,scale:Scale)->[f32;2]{
        [
            self.offset[0]*scale.horizontal,
            self.offset[1]*scale.vertical
        ]
    }

    /// Returns glyph's scaled width.
    pub fn width(&self,horizontal_scale:f32)->f32{
        self.size[0]*horizontal_scale
    }

    /// Returns glyph's scaled height.
    pub fn height(&self,vertical_scale:f32)->f32{
        self.size[1]*vertical_scale
    }

    /// Returns glyph's scaled size.
    pub fn size(&self,scale:Scale)->[f32;2]{
        [
            self.size[0]*scale.horizontal,
            self.size[1]*scale.vertical,
        ]
    }

    /// Returns glyph's scaled advance width.
    pub fn advance_width(&self,horizontal_scale:f32)->f32{
        self.advance_width*horizontal_scale
    }

    /// Returns glyph's scaled bounding box.
    pub fn bounding_box(&self,scale:Scale)->[f32;4]{
        [
            self.offset[0]*scale.horizontal,
            self.offset[1]*scale.vertical,
            self.size[0]*scale.horizontal,
            self.size[1]*scale.vertical,
        ]
    }

    /// Returns glyph's scaled frame.
    pub fn frame(&self,scale:Scale)->GlyphFrame{
        GlyphFrame{
            offset:[
                self.offset[0]*scale.horizontal,
                self.offset[1]*scale.vertical
            ],
            size:[
                self.size[0]*scale.horizontal,
                self.size[1]*scale.vertical
            ],
            advance_width:self.advance_width*scale.horizontal,
        }
    }

    pub fn scale(&self,scale:Scale)->ScaledGlyph<T>{
        ScaledGlyph{
            data:&self.data,
            offset:[
                self.offset[0]*scale.horizontal,
                self.offset[1]*scale.vertical
            ],
            size:[
                (self.size[0]*scale.horizontal).ceil() as u32,
                (self.size[1]*scale.vertical).ceil() as u32
            ],
            scale,
            advance_width:self.advance_width*scale.horizontal,
        }
    }
}


impl RawGlyph<Texture2d>{
    pub fn texture(&self)->&Texture2d{
        &self.data
    }
}

impl RawGlyph<Vec<OutlineCurve>>{
    /// Возвращает мастабированный глиф.
    /// 
    /// Returns a scaled glyph.
    pub fn outlined_glyph(&self,scale:Scale)->OutlinedGlyph{
        // Ширина глифа для данного размера шрифта
        let width=self.size[0]*scale.horizontal;

        let height=self.size[1]*scale.vertical;

        // Округление размер глифа
        let size=[
            width.ceil() as u32,
            height.ceil() as u32,
        ];

        // Мастабированный сдвиг для правильного
        // переноса на текстуру
        let offset=[
            self.offset[0]*scale.horizontal,
            self.offset[1]*scale.vertical,
        ];

        OutlinedGlyph{
            offset:offset,
            size,
            scale,
            curves:self.data.clone()
        }
    }
}

/// Мастабированный глиф.
/// A scaled glyph.
pub struct ScaledGlyph<'a,T:'a>{
    data:&'a T,
    // Мастабированный размер
    size:[u32;2],
    // Мастабированный сдвиг
    offset:[f32;2],
    // Мастабированное расстояние до следующего глифа
    advance_width:f32,
    // Мастабирование
    scale:Scale
}

impl<'a,T> ScaledGlyph<'a,T>{
    /// width, height должны быть целыми
    #[inline(always)]
    pub const fn new(data:&'a T,[x,y,width,height]:[f32;4],advance_width:f32,scale:Scale)->Self{
        Self{
            data,
            offset:[x,y],
            size:[width as u32,height as u32],
            advance_width,
            scale,
        }
    }

    pub fn data(&'a self)->&'a T{
        self.data
    }

    #[inline(always)]
    pub fn offset_x(&self)->f32{
        self.offset[0]
    }

    #[inline(always)]
    pub fn offset_y(&self)->f32{
        self.offset[1]
    }

    #[inline(always)]
    pub fn size(&self)->[u32;2]{
        self.size
    }

    pub fn advance_width(&self)->f32{
        self.advance_width
    }

    pub fn positioned_bounding_box(&self,position:[f32;2])->[f32;4]{
        [
            position[0]+self.offset[0],
            position[1]-self.offset[1]-self.size[1] as f32,
            self.size[0] as f32,
            self.size[1] as f32
        ]
    }
}

impl<'a> ScaledGlyph<'a,Vec<OutlineCurve>>{
    /// Create an outlined glyph with copied data.
    pub fn outline(self)->OutlinedGlyph{
        OutlinedGlyph{
            offset:self.offset,
            size:self.size,
            scale:self.scale,
            curves:self.data.clone(),
        }
    }
}

/// Глиф с текстурой основой.
/// Glyph based on a texture.
pub struct TexturedGlyph<'a>{
    // Текстура
    texture:&'a Texture2d,
    // Размер области для вставки текстуры
    size:[f32;2],
}

impl<'a> TexturedGlyph<'a>{
    pub fn raw(texture:&'a Texture2d,size:[f32;2])->TexturedGlyph<'a>{
        Self{
            texture,
            size,
        }
    }

    pub fn texture(&self)->&Texture2d{
        self.texture
    }

    pub fn size(&self)->[f32;2]{
        self.size
    }
}

/// Глиф с контурной основой.
/// Glyph based on an outline.
#[derive(Clone,Debug)]
pub struct OutlinedGlyph{
    // Scaled
    offset:[f32;2],
    // Scaled
    size:[u32;2],
    // Scale for `OutlineCurve`
    scale:Scale,
    curves:Vec<OutlineCurve>,
}

impl OutlinedGlyph{
    /// width, height должны быть целыми
    #[inline(always)]
    pub const fn raw(curves:Vec<OutlineCurve>,offset:[f32;2],[width,height]:[f32;2],scale:Scale)->Self{
        Self{
            offset,
            size:[width as u32,height as u32],
            scale,
            curves,
        }
    }

    pub fn offset(&self)->[f32;2]{
        self.offset
    }

    #[inline(always)]
    pub fn size(&self)->[u32;2]{
        self.size
    }

    pub fn draw<O:FnMut(usize,f32)>(&self,mut o:O){
        let scale_up=|&Point{x,y}|point(
            (x*self.scale.horizontal)-self.offset[0],
            (y*self.scale.vertical)-self.offset[1],
        );

        self.curves.iter().fold(
            Rasterizer::new(
                self.size[0] as usize,
                self.size[1] as usize
            ),
            |mut rasterizer,curve|match curve{
                OutlineCurve::Line(p0, p1)=>{
                    rasterizer.draw_line(scale_up(p0),scale_up(p1));
                    rasterizer
                }
                OutlineCurve::Quad(p0,p1,p2)=>{
                    rasterizer.draw_quad(
                        scale_up(p0),
                        scale_up(p1),
                        scale_up(p2),
                    );
                    rasterizer
                }
                OutlineCurve::Cubic(p0,p1,p2,p3)=>{
                    rasterizer.draw_cubic(
                        scale_up(p0),
                        scale_up(p1),
                        scale_up(p2),
                        scale_up(p3),
                    );
                    rasterizer
                }
            }
        )
        .for_each_pixel(|c,f|{
            o(c,f)
        });
    }
}

/// Contains glyph's size, offset and advance width.
#[derive(Debug)]
pub struct GlyphFrame{
    pub offset:[f32;2],
    pub size:[f32;2],
    pub advance_width:f32,
}