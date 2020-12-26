use crate::{
    Colour,
    texture::Texture,
    text::CachedFont,
};

#[cfg(feature="text_graphics")]
use crate::text::{TextBase};

use super::{
    GraphicsSettings,
    DependentObject
};


mod objects;
pub (crate) use objects::{
    SimpleObject2D,
    TexturedObject2D,
    TextObject2D,
};

pub use objects::{Vertex2D,TexturedVertex2D};

mod simple_graphics;
pub (crate) use simple_graphics::SimpleGraphics;


mod texture_graphics;
pub (crate) use texture_graphics::TextureGraphics;

#[cfg(feature="text_graphics")]
mod text_graphics;
#[cfg(feature="text_graphics")]
pub (crate) use text_graphics::TextGraphics;


use glium::{
    // enums
    index::PrimitiveType,
    // structs
    Display,
};

/// Графическая основа. A graphics base.
pub struct Graphics2D{
    #[cfg(feature="texture_graphics")]
    pub (crate) texture:TextureGraphics,
    #[cfg(feature="simple_graphics")]
    pub (crate) simple:SimpleGraphics,
    #[cfg(feature="text_graphics")]
    pub (crate) text:TextGraphics,
}

impl Graphics2D{
    pub (crate) fn new(window:&Display,settings:GraphicsSettings,glsl:u16)->Graphics2D{
        Self{
            #[cfg(feature="texture_graphics")]
            texture:TextureGraphics::new(window,settings.texture,glsl),
            #[cfg(feature="simple_graphics")]
            simple:SimpleGraphics::new(window,settings.simple,glsl),
            #[cfg(feature="text_graphics")]
            text:TextGraphics::new(window,settings.text,glsl),
        }
    }
}



/// # Функции для работы с простыми объектами. Functions to work with simple objects.
#[cfg(feature="simple_graphics")]
impl Graphics2D{
    /// Добавляет простой объект в массив.
    /// 
    /// Adds the simple object to the array.
    #[inline(always)]
    pub fn add_simple_object<O,V,I>(&mut self,object:&O)->Option<usize>
        where
            O:DependentObject<
                Vertex2D,
                u8,
                Vertices=V,
                Indices=I
            >,
            V:AsRef<[Vertex2D]>,
            I:AsRef<[u8]>
    {
        self.simple.push_object(object)
    }

    /// Удаляет последний простой объект в массиве,
    /// если такой есть.
    /// 
    /// Removes the last simple object of the array
    /// if there is any.
    #[inline(always)]
    pub fn remove_last_simple_object(&mut self){
        self.simple.remove_last_object();
    }

    /// Отчищает массив объектов.
    /// 
    /// Clears the object array.
    #[inline(always)]
    pub fn clear_simple_object_array(&mut self){
        self.simple.clear_object_array()
    }

    /// Возвращает ссылку на цвет объекта.
    /// 
    /// Returns a reference to the object's colour.
    #[inline(always)]
    pub fn get_simple_object_colour(&mut self,index:usize)->&mut Colour{
        self.simple.get_object_colour(index)
    }

    /// Устанавливает цвет объекта.
    /// 
    /// Паникует, если нет такого объекта.
    /// 
    /// Sets the colour of the object.
    /// 
    /// Panics if there is no such object.
    #[inline(always)]
    pub fn set_simple_object_colour(&mut self,index:usize,colour:Colour){
        self.simple.set_object_colour(index,colour)
    }

    /// Устанавливает тип отрисовки объекта.
    /// 
    /// Паникует, если нет такого объекта.
    /// 
    /// Sets the primitive type of the object.
    /// 
    /// Panics if there is no such object.
    #[inline(always)]
    pub fn set_simple_object_primitive_type(&mut self,index:usize,primitive_type:PrimitiveType){
        self.simple.set_object_primitive_type(index,primitive_type)
    }

    /// Переписывает вершины объекта.
    /// 
    /// Паникует, если нет такого объекта или размер `vertices` отличается от установленного.
    /// 
    /// Rewrites vertices of the object.
    /// 
    /// Panics if there is no such object or the `vertices` size differs from the set.
    #[inline(always)]
    pub fn rewrite_simple_object_vertices(&mut self,index:usize,vertices:&[Vertex2D]){
        self.simple.rewrite_object_vertices(index,vertices)
    }

    /// Переписывает индексы объекта.
    /// 
    /// Паникует, если нет такого объекта или размер `indices` отличается от установленного.
    /// 
    /// Rewrites indices of the object.
    /// 
    /// Panics if there is no such object or the `indices` size differs from the set.
    #[inline(always)]
    pub fn rewrite_simple_object_indices(&mut self,index:usize,indices:&[u8]){
        self.simple.rewrite_object_indices(index,indices)
    }
}

/// # Функции для работы с текстурными объектами. Functions to work with textured objects.
#[cfg(feature="texture_graphics")]
impl Graphics2D{
    /// Добавляет тектуру.
    /// 
    /// Adds a texture.
    #[inline(always)]
    pub fn add_texture(&mut self,texture:Texture){
        self.texture.add_texture(texture)
    }

    /// Удаляет последнюю тектуру.
    /// 
    /// Removes the last texture.
    #[inline(always)]
    pub fn remove_last_texture(&mut self){
        self.texture.remove_last_texture()
    }

    /// Удаляет все тектуры.
    /// 
    /// Removes all textures.
    #[inline(always)]
    pub fn remove_all_textures(&mut self){
        self.texture.remove_all_textures()
    }

    /// Добавляет простой объект в массив.
    /// 
    /// Adds the simple object to the array.
    #[inline(always)]
    pub fn add_textured_object<O,V,I>(
        &mut self,
        object:&O,
        texture:usize
    )->Option<usize>
        where
            O:DependentObject<
                TexturedVertex2D,
                u8,
                Vertices=V,
                Indices=I
            >,
            V:AsRef<[TexturedVertex2D]>,
            I:AsRef<[u8]>
    {
        self.texture.push_object(object,texture)
    }

    /// Удаляет последний простой объект в массиве,
    /// если такой есть.
    /// 
    /// removes the last textured object of the array
    /// if there is any.
    #[inline(always)]
    pub fn remove_last_textured_object(&mut self){
        self.texture.remove_last_object();
    }

    /// Отчищает массив объектов.
    /// 
    /// Clears the object array.
    #[inline(always)]
    pub fn clear_textured_object_array(&mut self){
        self.texture.clear_object_array()
    }

    /// Возвращает ссылку на цвет объекта.
    /// 
    /// Returns a reference to the object's colour.
    #[inline(always)]
    pub fn get_textured_object_colour(&mut self,index:usize)->&mut Colour{
        self.texture.get_object_colour(index)
    }

    /// Возвращает ссылку на текстуру объекта.
    /// 
    /// Returns a reference to the object's texture.
    #[inline(always)]
    pub fn get_textured_object_texture(&mut self,index:usize)->&mut Texture{
        self.texture.get_object_texture(index)
    }

    /// Устанавливает цвет объекта.
    /// 
    /// Паникует, если нет такого объекта.
    /// 
    /// Sets the colour of the object.
    /// 
    /// Panics if there is no such object.
    #[inline(always)]
    pub fn set_textured_object_colour(&mut self,index:usize,colour:Colour){
        self.texture.set_object_colour(index,colour)
    }

    /// Устанавливает тип отрисовки объекта.
    /// 
    /// Паникует, если нет такого объекта.
    /// 
    /// Sets the primitive type of the object.
    /// 
    /// Panics if there is no such object.
    #[inline(always)]
    pub fn set_textured_object_primitive_type(&mut self,index:usize,primitive_type:PrimitiveType){
        self.texture.set_object_primitive_type(index,primitive_type)
    }

    /// Переписывает вершины объекта.
    /// 
    /// Паникует, если нет такого объекта или размер `vertices` отличается от установленного.
    /// 
    /// Rewrites vertices of the object.
    /// 
    /// Panics if there is no such object or the `vertices` size differs from the set.
    #[inline(always)]
    pub fn rewrite_textured_object_vertices(&mut self,index:usize,vertices:&[TexturedVertex2D]){
        self.texture.rewrite_object_vertices(index,vertices)
    }

    /// Переписывает индексы объекта.
    /// 
    /// Паникует, если нет такого объекта или размер `indices` отличается от установленного.
    /// 
    /// Rewrites indices of the object.
    /// 
    /// Panics if there is no such object or the `indices` size differs from the set.
    #[inline(always)]
    pub fn rewrite_textured_object_indices(&mut self,index:usize,indices:&[u8]){
        self.texture.rewrite_object_indices(index,indices)
    }
}

/// # Функции для работы с текстовыми объектами. Functions to work with text objects.
#[cfg(feature="text_graphics")]
impl Graphics2D{
    /// Добавляет шрифт в массив.
    /// 
    /// Adds a cached font to the array.
    #[inline(always)]
    pub fn add_font(
        &mut self,
        cached_font:CachedFont,
    )->Option<usize>{
        self.text.push_font(cached_font)
    }

    /// Удаляет последний шрифт.
    /// 
    /// Removes the font.
    #[inline(always)]
    pub fn remove_last_font(&mut self){
        self.text.remove_last_font()
    }

    /// Удаляет все шрифты.
    /// 
    /// Removes all fonts.
    #[inline(always)]
    pub fn remove_all_fonts(&mut self){
        self.text.remove_all_fonts()
    }

    /// Возращает шрифт.
    /// 
    /// Returns a cached font.
    #[inline(always)]
    pub fn get_glyph_cache(&self,index:usize)->&CachedFont{
        self.text.get_font(index)
    }

    /// Добавляет текстовой объект в массив.
    /// 
    /// Adds a text object to the array.
    #[inline(always)]
    pub fn add_text_object(
        &mut self,
        text:String,
        text_base:&TextBase,
        font:usize,
    )->Option<usize>{
        self.text.push_object(
            text,
            text_base.position,
            text_base.scale,
            text_base.colour,
            font
        )
    }

    /// Удаляет последний текстовой объект в массиве,
    /// если такой есть.
    /// 
    /// removes the last text object of the array
    /// if there is any.
    #[inline(always)]
    pub fn remove_last_text_object(&mut self){
        self.text.remove_last_object();
    }

    /// Отчищает массив объектов.
    /// 
    /// Clears the object array.
    #[inline(always)]
    pub fn clear_text_object_array(&mut self){
        self.text.clear_object_array()
    }
}