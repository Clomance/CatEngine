use super::Scale;

use cat_engine_basement::graphics::level1::Texture2D;


struct Glyph<T>{
    data:T,
    // Размер
    size:[f32;2],
    // Сдвиг
    offset:[f32;2],
    // Горизонтальное расстояние до следующего глифа
    advance_width:f32,
}

impl<T> Glyph<T>{
    #[inline(always)]
    pub fn raw(
        data:T,
        size:[f32;2],
        offset:[f32;2],
        advance_width:f32,
    )->Glyph<T>{
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
}

/// A glyph represented as a texture.
pub struct TexturedGlyph{
    glyph:Glyph<Texture2D>,
}

impl TexturedGlyph{
    pub (crate) fn raw(
        texture:Texture2D,
        size:[f32;2],
        offset:[f32;2],
        advance_width:f32,
    )->TexturedGlyph{
        Self{
            glyph:Glyph::raw(
                texture,
                size,
                offset,
                advance_width
            ),
        }
    }

    /// Returns glyph's scaled offset.
    pub fn offset(&self,scale:Scale)->[f32;2]{
        self.glyph.offset(scale)
    }

    /// Returns glyph's scaled width.
    pub fn width(&self,horizontal_scale:f32)->f32{
        self.glyph.width(horizontal_scale)
    }

    /// Returns glyph's scaled height.
    pub fn height(&self,vertical_scale:f32)->f32{
        self.glyph.height(vertical_scale)
    }

    /// Returns glyph's scaled size.
    pub fn size(&self,scale:Scale)->[f32;2]{
        self.glyph.size(scale)
    }

    /// Returns glyph's scaled advance width.
    pub fn advance_width(&self,horizontal_scale:f32)->f32{
        self.glyph.advance_width(horizontal_scale)
    }

    /// Returns glyph's scaled bounding box.
    pub fn bounding_box(&self,scale:Scale)->[f32;4]{
        self.glyph.bounding_box(scale)
    }

    pub fn texture(&self)->&Texture2D{
        &self.glyph.data
    }
}