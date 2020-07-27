use crate::Colour;

use super::GraphicsSettings;

#[cfg(feature="simple_graphics")]
use super::{
    SimpleGraphics,
    SimpleObject,
    Vertex2D,
};

#[cfg(feature="texture_graphics")]
use super::{TextureGraphics,TexturedVertex2D};
#[cfg(feature="texture_graphics")]
use crate::image::{ImageBase,Texture};


#[cfg(feature="text_graphics")]
use super::TextGraphics;

use glium::{
    // enums
    index::PrimitiveType,
    // structs
    Display,
};

use image::RgbaImage;

/// Графическая основа. A graphics base.
/// 
/// # Области. Ranges.
/// 
/// Вы можете выбрать область в буфере вершин и сохранить туда вершины объектов.
/// Это помогает ускорить процесс отрисовки неменяющихся объектов.
/// 
/// You can choose a range of the vertex buffer and save there vertexes of objects.
/// It speeds up drawing unchanging objects.
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
            text:TextGraphics::new(window,settings.text_vertex_buffer_size,glsl),
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
    pub fn add_simple_object<O:SimpleObject>(&mut self,object:&O)->Option<usize>{
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
    pub fn add_textured_object(&mut self,image_base:&ImageBase,texture:Texture)->Option<usize>{
        self.texture.push_object(image_base,texture)
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