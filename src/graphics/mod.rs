use crate::{
    // types
    Colour,
};

#[cfg(feature="text_graphics")]
use crate::text::{
    Scale,
    CachedFont,
};

mod object_allocation;
use object_allocation::{
    HeapDrawType,
    HeapObject,
    StackObject,
    StackDrawType,
    ObjectAllocation,
};

mod objects;
pub use objects::{
    SimpleVertex2D,
    TexturedVertex2D,
    TextVertex2D,
    ShapeObject,
};

mod glyph_image_builder;
use glyph_image_builder::GlyphImageBuilder;

#[cfg(feature="simple_graphics")]
mod simple_graphics;
#[cfg(feature="simple_graphics")]
use simple_graphics::SimpleGraphics;

#[cfg(feature="texture_graphics")]
mod texture_graphics;
#[cfg(feature="texture_graphics")]
use texture_graphics::TextureGraphics;

#[cfg(feature="text_graphics")]
mod text_graphics;
#[cfg(feature="text_graphics")]
use text_graphics::TextGraphics;

mod graphics_2d;
pub use graphics_2d::{
    Graphics2DAttributes,
    Graphics2D,
};

mod draw_parameters;
pub use draw_parameters::{
    DrawMode,
    DrawParameters,
};

use cat_engine_basement::graphics::level1::Texture2D;

use cat_engine_basement::graphics::GCore;

pub use cat_engine_basement::graphics::core::{
    GraphicsCore,
    ClearMask,
    drawing::PrimitiveType,
    blending::{
        BlendingEquation,
        BlendingFunction,
    },
};

pub type FrameIDType=u16;
pub type ObjectIDType=u16;
pub type ElementIndexType=u16;
const frame_size:usize=3;
/// The minimum of frames per object.
const minimal_frames:usize=3;

pub struct Graphics{
    graphics_2d:Graphics2D,
}

impl Graphics{
    #[cfg(target_os="windows")]
    pub fn new(attributes:Graphics2DAttributes)->Graphics{
        Self{
            graphics_2d:Graphics2D::new(attributes),
        }
    }

    pub fn graphics_2d(&self)->&Graphics2D{
        &self.graphics_2d
    }

    pub fn graphics_2d_mut(&mut self)->&mut Graphics2D{
        &mut self.graphics_2d
    }

    pub fn core(&self)->&GraphicsCore{
        unsafe{
            &GCore
        }
    }

    pub fn draw_parameters(&mut self)->&mut DrawParameters{
        self.graphics_2d.draw_parameters()
    }
}

impl Graphics{
    pub fn clear_colour(&self,colour:Colour){
        unsafe{
            GCore.set_clear_colour(colour);
            GCore.clear(ClearMask::Colour)
        }
    }
}

/// Text graphics.
#[cfg(feature="text_graphics")]
impl Graphics{
    pub fn draw_char(
        &self,
        character:char,
        colour:Colour,
        position:[f32;2],
        horisontal_advance:Option<&mut f32>,
        scale:Scale,
        font:&CachedFont
    ){
        self.graphics_2d.draw_char(
            character,
            colour,
            position,
            horisontal_advance,
            scale,
            font,
        )
    }
}

/// Simple graphics.
#[cfg(feature="simple_graphics")]
impl Graphics{
    pub fn add_simple_object_raw(
        &mut self,
        vertices:&[SimpleVertex2D],
        indices:&[ElementIndexType],
        primitive_type:PrimitiveType
    )->Option<ObjectIDType>{
        self.graphics_2d.add_simple_object_raw(
            vertices,
            indices,
            primitive_type
        )
    }

    pub fn add_simple_object<O:ShapeObject<SimpleVertex2D,ElementIndexType>>(
        &mut self,
        object:&O
    )->Option<ObjectIDType>{
        let vertices=object.vertices();
        let indices=object.indices();
        let primitive_type=object.primitive_type();
        self.graphics_2d.add_simple_object_raw(
            vertices.as_ref(),
            indices.as_ref(),
            primitive_type
        )
    }

    /// Removes an object.
    /// 
    /// It's not actually removes it, just clears it's data.
    pub fn remove_simple_object(&mut self,index:ObjectIDType){
        self.graphics_2d.remove_simple_object(index);
    }

    pub fn write_heap_simple_object_vertices(&mut self,index:ObjectIDType,vertices:&[SimpleVertex2D]){
        self.graphics_2d.write_heap_simple_object_vertices(index,vertices)
    }

    pub fn write_heap_simple_object_indices(&mut self,index:ObjectIDType,indices:&[ElementIndexType]){
        self.graphics_2d.write_heap_simple_object_indices(index,indices)
    }

    pub fn draw_heap_simple_object(&self,index:ObjectIDType){
        self.graphics_2d.draw_heap_simple_object(index);
    }

    pub fn push_simple_object_raw(
        &mut self,
        vertices:&[SimpleVertex2D],
        indices:&[ElementIndexType],
        primitive_type:PrimitiveType
    )->Option<ObjectIDType>{
        self.graphics_2d.push_simple_object_raw(
            vertices,
            indices,
            primitive_type
        )
    }

    pub fn push_simple_object<O:ShapeObject<SimpleVertex2D,ElementIndexType>>(
        &mut self,
        object:&O
    )->Option<ObjectIDType>{
        let vertices=object.vertices();
        let indices=object.indices();
        let primitive_type=object.primitive_type();
        self.graphics_2d.push_simple_object_raw(
            vertices.as_ref(),
            indices.as_ref(),
            primitive_type
        )
    }

    pub fn pop_simple_object(&mut self){
        self.graphics_2d.pop_simple_object();
    }

    pub fn clear_stack_simple_objects(&mut self){
        self.graphics_2d.clear_stack_simple_objects()
    }

    pub fn write_stack_simple_object_vertices(&mut self,index:ObjectIDType,vertices:&[SimpleVertex2D]){
        self.graphics_2d.write_stack_simple_object_vertices(index,vertices)
    }

    pub fn write_stack_simple_object_indices(&mut self,index:ObjectIDType,indices:&[ElementIndexType]){
        self.graphics_2d.write_stack_simple_object_indices(index,indices)
    }

    pub fn draw_stack_simple_object(&self,index:ObjectIDType){
        self.graphics_2d.draw_stack_simple_object(index);
    }
}

/// Texture graphics.
#[cfg(feature="texture_graphics")]
impl Graphics{
    pub fn add_textured_object_raw(
        &mut self,
        vertices:&[TexturedVertex2D],
        indices:&[ElementIndexType],
        primitive_type:PrimitiveType
    )->Option<ObjectIDType>{
        self.graphics_2d.add_textured_object_raw(
            vertices,
            indices,
            primitive_type
        )
    }

    pub fn add_textured_object<O:ShapeObject<TexturedVertex2D,ElementIndexType>>(
        &mut self,
        object:&O
    )->Option<ObjectIDType>{
        let vertices=object.vertices();
        let indices=object.indices();
        let primitive_type=object.primitive_type();
        self.graphics_2d.add_textured_object_raw(
            vertices.as_ref(),
            indices.as_ref(),
            primitive_type
        )
    }

    /// Removes an object.
    /// 
    /// It's not actually removes it, just clears it's data.
    pub fn remove_textured_object(&mut self,index:ObjectIDType){
        self.graphics_2d.remove_textured_object(index);
    }

    pub fn clear_stack_textured_objects(&mut self){
        self.graphics_2d.clear_stack_textured_objects()
    }

    pub fn write_heap_textured_object_vertices(&mut self,index:ObjectIDType,vertices:&[TexturedVertex2D]){
        self.graphics_2d.write_heap_textured_object_vertices(index,vertices)
    }

    pub fn write_heap_textured_object_indices(&mut self,index:ObjectIDType,indices:&[ElementIndexType]){
        self.graphics_2d.write_heap_textured_object_indices(index,indices)
    }

    pub fn draw_heap_textured_object(&self,index:ObjectIDType,texture:&Texture2D){
        self.graphics_2d.draw_heap_textured_object(index,texture);
    }

    pub fn push_textured_object_raw(
        &mut self,
        vertices:&[TexturedVertex2D],
        indices:&[ElementIndexType],
        primitive_type:PrimitiveType
    )->Option<ObjectIDType>{
        self.graphics_2d.push_textured_object_raw(
            vertices,
            indices,
            primitive_type
        )
    }

    pub fn push_textured_object<O:ShapeObject<TexturedVertex2D,ElementIndexType>>(
        &mut self,
        object:&O
    )->Option<ObjectIDType>{
        let vertices=object.vertices();
        let indices=object.indices();
        let primitive_type=object.primitive_type();
        self.graphics_2d.push_textured_object_raw(
            vertices.as_ref(),
            indices.as_ref(),
            primitive_type
        )
    }

    pub fn pop_textured_object(&mut self){
        self.graphics_2d.pop_textured_object();
    }

    pub fn write_stack_textured_object_vertices(&mut self,index:ObjectIDType,vertices:&[TexturedVertex2D]){
        self.graphics_2d.write_stack_textured_object_vertices(index,vertices)
    }

    pub fn write_stack_textured_object_indices(&mut self,index:ObjectIDType,indices:&[ElementIndexType]){
        self.graphics_2d.write_stack_textured_object_indices(index,indices)
    }

    pub fn draw_stack_textured_object(&self,index:ObjectIDType,texture:&Texture2D){
        self.graphics_2d.draw_stack_textured_object(index,texture);
    }
}