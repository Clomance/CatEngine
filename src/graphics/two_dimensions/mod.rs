use crate::{
    Colour,
    texture::Texture,
    text::{Scale,CachedFont},
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
    /// Добавляет простой объект.
    /// 
    /// Возвращает индекс объекта или `None`, если нет места.
    /// 
    /// Adds the simple object.
    /// 
    /// Returns object's index or `None` if there is no space.
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

    /// Удаляет последний простой объект, если такой есть.
    /// 
    /// Removes the last simple object if there is any.
    #[inline(always)]
    pub fn remove_last_simple_object(&mut self){
        self.simple.remove_last_object();
    }

    /// Удаляет все объекты.
    /// 
    /// Removes all objects.
    #[inline(always)]
    pub fn remove_all_simple_objects(&mut self){
        self.simple.clear_object_array()
    }

    /// Возвращает ссылку на цвет объекта.
    /// 
    /// Returns a reference to object's colour.
    #[inline(always)]
    pub fn get_simple_object_colour(&mut self,index:usize)->&mut Colour{
        self.simple.object_colour(index)
    }

    /// Возвращает ссылку на тип отрисовки объекта.
    /// 
    /// Returns a reference to object's primitive type.
    #[inline(always)]
    pub fn get_simple_object_primitive_type(&mut self,index:usize)->&mut PrimitiveType{
        self.simple.object_primitive_type(index)
    }

    /// Переписывает вершины объекта.
    /// 
    /// Паникует, если нет такого объекта или размер `vertices` отличается от установленного.
    /// 
    /// Rewrites object's vertices.
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
    /// Rewrites object's indices.
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
    /// Возвращает индекс текстуры.
    /// 
    /// Adds a texture.
    /// 
    /// Returns texture's index.
    #[inline(always)]
    pub fn add_texture(&mut self,texture:Texture)->usize{
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

    /// Добавляет текстурный объект.
    /// 
    /// Возвращает индекс объекта или `None`, если нет места.
    /// 
    /// Adds a textured object.
    /// 
    /// Returns object's index or `None` if there is no space.
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

    /// Удаляет последний простой объект, если такой есть.
    /// 
    /// Removes the last textured object if there is any.
    #[inline(always)]
    pub fn remove_last_textured_object(&mut self){
        self.texture.remove_last_object();
    }

    /// Удаляет все текстурные объекты.
    /// 
    /// Removes all textured objects.
    #[inline(always)]
    pub fn removes_all_textured_objects(&mut self){
        self.texture.clear_object_array()
    }

    /// Возвращает ссылку на цвет объекта.
    /// 
    /// Returns a reference to object's colour.
    #[inline(always)]
    pub fn get_textured_object_colour(&mut self,index:usize)->&mut Colour{
        self.texture.object_colour(index)
    }

    /// Возвращает ссылку на текстуру объекта.
    /// 
    /// Returns a reference to object's texture.
    #[inline(always)]
    pub fn get_textured_object_texture(&mut self,index:usize)->&mut Texture{
        self.texture.object_texture(index)
    }

    /// Возвращает ссылку на тип отрисовки объекта.
    /// 
    /// Returns a reference to object's primitive type.
    #[inline(always)]
    pub fn get_textured_object_primitive_type(&mut self,index:usize)->&mut PrimitiveType{
        self.texture.object_primitive_type(index)
    }

    /// Переписывает вершины объекта.
    /// 
    /// Паникует, если нет такого объекта или размер `vertices` отличается от установленного.
    /// 
    /// Rewrites object's vertices.
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
    /// Добавляет шрифт.
    /// 
    /// Возвращает индекс шрифта.
    /// 
    /// Adds a font.
    /// 
    /// Returns font's index.
    #[inline(always)]
    pub fn add_font(&mut self,cached_font:CachedFont)->usize{
        self.text.push_font(cached_font)
    }

    /// Удаляет последний шрифт.
    /// 
    /// Removes the last font.
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
    /// Returns a font.
    #[inline(always)]
    pub fn get_font(&self,index:usize)->&CachedFont{
        self.text.get_font(index)
    }

    /// Добавляет текстовой объект.
    /// 
    /// Adds a text object.
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

    /// Удаляет последний текстовой объект, если такой есть.
    /// 
    /// Removes the last text object if there is any.
    #[inline(always)]
    pub fn remove_last_text_object(&mut self){
        self.text.remove_last_object();
    }

    /// Удаляет все текстовые объекты.
    /// 
    /// Removes all text objects.
    #[inline(always)]
    pub fn remove_all_text_objects(&mut self){
        self.text.clear_object_array()
    }

    /// Возвращает ссылку на текст объекта.
    /// 
    /// Returns a reference to object's text.
    #[inline(always)]
    pub fn get_text_object_text(&mut self,index:usize)->&mut String{
        self.text.object_text(index)
    }

    /// Возвращает ссылку на цвет объекта.
    /// 
    /// Returns a reference to object's colour.
    #[inline(always)]
    pub fn get_text_object_colour(&mut self,index:usize)->&mut Colour{
        self.text.object_colour(index)
    }

    /// Возвращает ссылку на шрифт объекта.
    /// 
    /// Returns a reference to object's font.
    #[inline(always)]
    pub fn get_text_object_font(&mut self,index:usize)->&mut usize{
        self.text.object_font(index)
    }

    /// Возвращает ссылку на масштаб объекта.
    /// 
    /// Returns a reference to object's scale.
    #[inline(always)]
    pub fn get_text_object_scale(&mut self,index:usize)->&mut Scale{
        self.text.object_scale(index)
    }

    /// Возвращает ссылку на положение объекта.
    /// 
    /// Returns a reference to object's position.
    #[inline(always)]
    pub fn get_text_object_position(&mut self,index:usize)->&mut [f32;2]{
        self.text.object_position(index)
    }
}