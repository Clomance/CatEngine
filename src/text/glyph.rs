use super::{
    OutlineCurve,
    Scale,
};

use glium::{
    Display,
    texture::{
        Texture2d,
        RawImage2d,
        ClientFormat,
    },
    Rect,
};

use ab_glyph_rasterizer::{Point,point,Rasterizer};

/// Немастабированный глиф.
/// 
/// An unscaled glyph.
pub struct RawGlyph<T>{
    data:T,
    // Размер полноразмерного
    size:[f32;2],
    // Сдвиг для полноразмерного глифа
    offset:[f32;2],
    // Расстояние до следующего глифа
    advance_width:f32,
    // Собственное мастабирование
    // (нужно для правильного соотношения размеров букв)
    scale:f32,
}


/// Общие функции для глифов.
/// 
/// General functions.
impl<T> RawGlyph<T>{
    #[inline(always)]
    pub const fn raw(
        data:T,
        size:[f32;2],
        offset:[f32;2],
        advance_width:f32,
        scale:f32
    )->RawGlyph<T>{
        Self{
            data,
            size,
            offset,
            advance_width,
            scale,
        }
    }

    pub fn offset(&self,font_size:f32){
        // Соотношение сторон: ширина на высоту
        let aspect_ratio=self.size[0]/self.size[1];

        let mut k=self.offset[1]/self.size[1];

        let new_height=self.scale*font_size;

        let y=new_height*k;

        k=self.offset[0]/self.size[0];

        let new_width=new_height*aspect_ratio;

        let x=new_width*k;
    }

    /// Returns the glyph width for the given font size.
    pub fn width(&self,font_size:f32)->f32{
        // Соотношение сторон: ширина на высоту
        let aspect_ratio=self.size[0]/self.size[1];

        self.scale*aspect_ratio*font_size
    }

    pub fn height(&self,font_size:f32)->f32{
        self.scale*font_size
    }

    pub fn size(&self,font_size:f32)->[f32;2]{
        // Соотношение сторон: ширина на высоту
        let aspect_ratio=self.size[0]/self.size[1];

        let new_height=self.scale*font_size;
        [
            aspect_ratio*new_height,
            new_height,
        ]
    }

    pub fn advance_width(&self,font_size:f32)->f32{
        let height=font_size*self.scale;
        let k=height/self.size[1];

        self.advance_width*k
    }

    // the height and advance
    pub fn height_and_advance(&self,font_size:f32)->[f32;2]{
        let new_height=self.scale*font_size;

        let k=new_height/self.size[1];

        let new_advance=self.advance_width*k;

        [
            new_advance,
            new_height
        ]
    }

    pub fn bounding_box(&self,font_size:f32)->[f32;4]{
        // Соотношение сторон: ширина на высоту
        let aspect_ratio=self.size[0]/self.size[1];

        let new_height=self.scale*font_size;

        let new_width=new_height*aspect_ratio;

        let mut k=self.offset[1]/self.size[1];

        let y=new_height*k;

        k=self.offset[0]/self.size[0];

        let x=new_width*k;

        [x,y,new_width,new_height]
    }

    /// Computes a bounding box and an advance width of a glyph.
    pub fn frame(&self,font_size:f32)->GlyphFrame{
        // Соотношение сторон: ширина на высоту
        let aspect_ratio=self.size[0]/self.size[1];

        let new_height=self.scale*font_size;

        let new_width=new_height*aspect_ratio;

        let mut k=self.offset[1]/self.size[1];

        let y=new_height*k;

        k=self.offset[0]/self.size[0];

        let x=new_width*k;

        k=new_width/self.size[0];

        let new_advance=self.advance_width*k;

        GlyphFrame{
            offset:[x,y],
            size:[new_width,new_height],
            advance:new_advance,
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
    pub fn outlined_glyph(&self,font_size:f32)->OutlinedGlyph{
        // Высота глифа для данного размера шрифта
        let height=font_size*self.scale;

        // Коэффициент мастабирования
        let k=height/self.size[1];

        // Ширина глифа для данного размера шрифта
        let width=k*self.size[0];

        // Округлённый размер глифа
        let size=[
            width.ceil() as u32,
            height.ceil() as u32,
        ];

        // Мастабированный сдвиг для правильного
        // переноса на текстуру
        let offset=[
            self.offset[0]*k,
            self.offset[1]*k,
        ];

        // Коэффициент мастабирования для построения глифа
        // (для мастабирования точек)
        let scale=Scale::new(k,k);

        OutlinedGlyph{
            offset:offset,
            size,
            scale,
            curves:self.data.clone()
        }
    }
}


pub struct ScaledGlyph<T>{
    data:T,
    // Мастабированный размер
    size:[u32;2],
    // Мастабированный сдвиг
    offset:[f32;2],
    // Мастабированное расстояние до следующего глифа
    advance_width:f32,
    // Мастабирование
    scale:Scale
}

impl<T> ScaledGlyph<T>{
    /// width, height должны быть целыми
    #[inline(always)]
    pub const fn new(data:T,[x,y,width,height]:[f32;4],advance_width:f32,scale:Scale)->Self{
        Self{
            data,
            offset:[x,y],
            size:[width as u32,height as u32],
            advance_width,
            scale,
        }
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
}

impl ScaledGlyph<Vec<OutlineCurve>>{
    pub fn draw<O:FnMut(usize,f32)>(&self,mut o:O){
        let scale_up=|&Point{x,y}|point(
            (x*self.scale.horizontal)-self.offset[0],
            (y*self.scale.vertical)-self.offset[1],
        );

        self.data.iter().fold(
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

/// Глиф с текстурой основой.
/// 
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

#[derive(Debug)]
pub struct GlyphFrame{
    pub offset:[f32;2],
    pub size:[f32;2],
    pub advance:f32,
}

impl GlyphFrame{
    pub fn bounding_box(&self,position:[f32;2])->[f32;4]{
        [
            position[0]+self.offset[0],
            position[1]-self.offset[1]-self.size[1],
            self.size[0],
            self.size[1]
        ]
    }
}

// fully-scaled glyph
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
    pub const fn new(curves:Vec<OutlineCurve>,[x,y,width,height]:[f32;4],scale:Scale)->Self{
        Self{
            offset:[x,y],
            size:[width as u32,height as u32],
            scale,
            curves,
        }
    }

    pub fn offset(&self)->f32{
        self.offset[1]
    }

    pub (crate) fn offset_y(&self)->f32{
        self.offset[1]
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