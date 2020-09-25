use glium::{
    Display,
    texture::{
        Texture2d,
        RawImage2d,
        ClientFormat,
    },
    Rect,
};

pub struct Glyph{
    texture:Texture2d,
    size:[f32;2],
    // Сдвиг для полноразмерного символа
    offset:[f32;2],
    // Расстояние до следующего символа
    advance:f32,
    // Собственное мастабирование
    // (нужно для правильного соотношения размеров букв)
    scale:f32,
}

impl Glyph{
    pub fn raw(
        texture:Texture2d,
        size:[f32;2],
        offset:[f32;2],
        advance:f32,
        scale:f32
    )->Glyph{

        Self{
            texture,
            size,
            offset,
            advance,
            scale,
        }
    }

    pub fn texture(&self)->&Texture2d{
        &self.texture
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

    // the height and advance
    pub fn height_advance(&self,font_size:f32)->[f32;2]{
        let new_height=self.scale*font_size;

        let k=new_height/self.size[1];

        let new_advance=self.advance*k;

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

    // Computes a bounding box and an advance width of a glyph
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

        let new_advance=self.advance*k;

        GlyphFrame{
            offset:[x,y],
            size:[new_width,new_height],
            advance:new_advance,
        }
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