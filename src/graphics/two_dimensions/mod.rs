use crate::{
    Colour,
    texture::Texture,
};

#[cfg(feature="text_graphics")]
use crate::text::{TextBase,rusttype::Font};

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
            text:TextGraphics::new(window,settings.text_image_buffer_size,glsl),
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
    pub fn add_simple_object<'o,O,V,I>(&mut self,object:&'o O)->Option<usize>
        where
            O:DependentObject<
                'o,
                Vertex2D,
                u8,
                Vertices=V,
                Indices=I
            >,
            V:AsRef<[Vertex2D]>+'o,
            I:AsRef<[u8]>+'o
    {
        self.simple.push_object(object)
    }

    /// Удаляет последний простой объект в массиве,
    /// если такой есть.
    /// 
    /// Deletes the last simple object of the array
    /// if there is any.
    #[inline(always)]
    pub fn delete_last_simple_object(&mut self){
        self.simple.delete_last_object();
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
    /// Добавляет простой объект в массив.
    /// 
    /// Adds the simple object to the array.
    #[inline(always)]
    pub fn add_textured_object<'o,O,V,I>(
        &mut self,
        object:&'o O,
        texture:Texture
    )->Option<usize>
        where
            O:DependentObject<
                'o,
                TexturedVertex2D,
                u8,
                Vertices=V,
                Indices=I
            >,
            V:AsRef<[TexturedVertex2D]>+'o,
            I:AsRef<[u8]>+'o
    {
        self.texture.push_object(object,texture)
    }

    /// Удаляет последний простой объект в массиве,
    /// если такой есть.
    /// 
    /// Deletes the last textured object of the array
    /// if there is any.
    #[inline(always)]
    pub fn delete_last_textured_object(&mut self){
        self.texture.delete_last_object();
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
    /// Adds a font to the array.
    #[inline(always)]
    pub fn add_font(
        &mut self,
        font:Font<'static>,
    )->Option<usize>{
        self.text.push_font(font)
    }

    /// Возращает шрифт.
    /// 
    /// Returns a font.
    #[inline(always)]
    pub fn get_font(&self,index:usize)->&Font<'static>{
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
        self.text.push_object(text,text_base,font)
    }

    /// Удаляет последний текстовой объект в массиве,
    /// если такой есть.
    /// 
    /// Deletes the last text object of the array
    /// if there is any.
    #[inline(always)]
    pub fn delete_last_text_object(&mut self){
        self.text.delete_last_object();
    }
}