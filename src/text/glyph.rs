use crate::texture::Texture2D;

use super::{
    OutlineCurve,
    Scale,
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

    pub fn data(&self)->&T{
        &self.data
    }
}


impl RawGlyph<Texture2D>{
    pub fn texture(&self)->&Texture2D{
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

/// Глиф с контурной основой.
/// Glyph based on an outline.
#[derive(Clone,Debug)]
pub struct OutlinedGlyph{
    offset:[f32;2],
    size:[u32;2],
    scale:Scale,
    curves:Vec<OutlineCurve>,
}

impl OutlinedGlyph{
    /// width, height должны быть целыми
    #[inline(always)]
    pub const fn raw(curves:Vec<OutlineCurve>,[width,height]:[u32;2],offset:[f32;2],scale:Scale)->Self{
        Self{
            offset,
            size:[width,height],
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

    pub fn scale(&self)->Scale{
        self.scale
    }

    pub fn curves(&self)->&Vec<OutlineCurve>{
        &self.curves
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