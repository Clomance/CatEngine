use crate::{
    Colour,
};

use cat_engine_basement::graphics::level1::texture::texture_2d::Texture2D;

#[cfg(feature="text_graphics")]
use crate::text::{
    Scale,
    CachedFont,
};

use super::{
    //types
    FrameIDType,
    ObjectIDType,
    ElementIndexType,
    // enums
    DrawMode,
    // structs
    SimpleVertex2D,
    TexturedVertex2D,
    DrawParameters,
    PrimitiveType,
};

#[cfg(feature="simple_graphics")]
use super::SimpleGraphics;

#[cfg(feature="texture_graphics")]
use super::TextureGraphics;

#[cfg(feature="text_graphics")]
use super::{
    TextGraphics,
    TextGraphicsAttributes,
};

use ttf_parser::{
    GlyphId,
    Face
};

pub struct Graphics2DAttributes{
    #[cfg(feature="simple_graphics")]
    pub simple_stack_vertices:ElementIndexType,
    #[cfg(feature="simple_graphics")]
    pub simple_stack_indices:i32,
    #[cfg(feature="simple_graphics")]
    pub simple_stack_objects:ObjectIDType,
    #[cfg(feature="simple_graphics")]
    pub simple_heap_vertex_frames:FrameIDType,
    #[cfg(feature="simple_graphics")]
    pub simple_heap_index_frames:FrameIDType,
    #[cfg(feature="simple_graphics")]
    pub simple_heap_objects:ObjectIDType,

    #[cfg(feature="texture_graphics")]
    pub texture_stack_vertices:ElementIndexType,
    #[cfg(feature="texture_graphics")]
    pub texture_stack_indices:i32,
    #[cfg(feature="texture_graphics")]
    pub texture_stack_objects:ObjectIDType,
    #[cfg(feature="texture_graphics")]
    pub texture_heap_vertex_frames:FrameIDType,
    #[cfg(feature="texture_graphics")]
    pub texture_heap_index_frames:FrameIDType,
    #[cfg(feature="texture_graphics")]
    pub texture_heap_objects:ObjectIDType,

    pub glyph_texture_size:[u32;2]
}

impl Graphics2DAttributes{
    pub fn new()->Graphics2DAttributes{
        Self{
            #[cfg(feature="simple_graphics")]
            simple_stack_vertices:128,
            #[cfg(feature="simple_graphics")]
            simple_stack_indices:128,
            #[cfg(feature="simple_graphics")]
            simple_stack_objects:32,
            #[cfg(feature="simple_graphics")]
            simple_heap_vertex_frames:128,
            #[cfg(feature="simple_graphics")]
            simple_heap_index_frames:128,
            #[cfg(feature="simple_graphics")]
            simple_heap_objects:32,

            #[cfg(feature="texture_graphics")]
            texture_stack_vertices:128,
            #[cfg(feature="texture_graphics")]
            texture_stack_indices:128,
            #[cfg(feature="texture_graphics")]
            texture_stack_objects:32,
            #[cfg(feature="texture_graphics")]
            texture_heap_vertex_frames:128,
            #[cfg(feature="texture_graphics")]
            texture_heap_index_frames:128,
            #[cfg(feature="texture_graphics")]
            texture_heap_objects:32,

            #[cfg(feature="text_graphics")]
            glyph_texture_size:[512u32;2]
        }
    }
}

pub struct Graphics2D{
    #[cfg(feature="simple_graphics")]
    simple:SimpleGraphics,
    #[cfg(feature="texture_graphics")]
    texture:TextureGraphics,
    #[cfg(feature="text_graphics")]
    text:TextGraphics,
    draw_parameters:DrawParameters,
}

impl Graphics2D{
    pub fn new(attributes:Graphics2DAttributes)->Graphics2D{
        #[cfg(feature="simple_graphics")]
        let simple=SimpleGraphics::new(
            attributes.simple_stack_vertices,
            attributes.simple_stack_indices,
            attributes.simple_stack_objects,
            attributes.simple_heap_vertex_frames,
            attributes.simple_heap_index_frames,
            attributes.simple_heap_objects
        );

        #[cfg(feature="texture_graphics")]
        let texture=TextureGraphics::new(
            attributes.texture_stack_vertices,
            attributes.texture_stack_indices,
            attributes.texture_stack_objects,
            attributes.texture_heap_vertex_frames,
            attributes.texture_heap_index_frames,
            attributes.texture_heap_objects
        );

        #[cfg(feature="text_graphics")]
        let text=TextGraphics::new(attributes.glyph_texture_size);

        Self{
            #[cfg(feature="simple_graphics")]
            simple,
            #[cfg(feature="texture_graphics")]
            texture,
            #[cfg(feature="text_graphics")]
            text,
            draw_parameters:DrawParameters::new(),
        }
    }

    pub fn draw_parameters(&mut self)->&mut DrawParameters{
        &mut self.draw_parameters
    }
}

/// Text graphics.
#[cfg(feature="text_graphics")]
impl Graphics2D{
    pub fn build_glyph_image(&self,glyph_id:GlyphId,scale:Scale,font:&Face)->Option<([f32;4],&[u8])>{
        self.text.build_glyph_image(glyph_id,scale,font)
    }

    pub fn draw_glyph(&self,glyph_texture:&Texture2D,colour:Colour,position:[f32;2],size:[f32;2]){
        self.text.draw_glyph(glyph_texture,colour,position,size,&self.draw_parameters);
    }

    pub fn draw_char(
        &self,
        character:char,
        colour:Colour,
        position:[f32;2],
        horisontal_advance:&mut f32,
        scale:Scale,
        font:&CachedFont
    ){
        let glyph_id=if let Some(id)=font.glyph_id(character){
            id
        }
        else{
            GlyphId(0u16)
        };

        if let Some(glyph)=font.cached_glyph(glyph_id){
            let texture=glyph.texture();
            let advance_width=glyph.advance_width(scale.horizontal);

            if !(horisontal_advance as *mut f32).is_null(){
                *horisontal_advance=advance_width
            }

            let [offset_x,offset_y,width,height]=glyph.bounding_box(scale);

            let position=[
                position[0]+offset_x,
                position[1]+offset_y,
            ];

            self.text.draw_glyph(texture,colour,position,[width,height],&self.draw_parameters);
        }
        else{
            if let Some([offset_x,offset_y,_,_])=self.text.load_glyph(glyph_id,scale,font.font().face()){
                if !(horisontal_advance as *mut f32).is_null(){
                    *horisontal_advance=font.font().face().glyph_hor_advance(glyph_id).unwrap() as f32*scale.horizontal;
                }

                let position=[
                    position[0]+offset_x,
                    position[1]+offset_y,
                ];

                self.text.draw_loaded_glyph(colour,position,&self.draw_parameters);
            }
        }
    }
}

/// Simple graphics.
#[cfg(feature="simple_graphics")]
impl Graphics2D{
    pub fn add_simple_object_raw(
        &mut self,
        vertices:&[SimpleVertex2D],
        indices:&[ElementIndexType],
        primitive_type:PrimitiveType
    )->Option<ObjectIDType>{
        self.simple.add_object_raw(
            vertices,
            indices,
            primitive_type as u32
        )
    }

    /// Removes an object.
    /// 
    /// It's not actually removes it, just clears it's data.
    pub fn remove_simple_object(&mut self,index:ObjectIDType){
        self.simple.remove_object(index);
    }

    pub fn write_heap_simple_object_vertices(&mut self,index:ObjectIDType,vertices:&[SimpleVertex2D]){
        self.simple.write_heap_object_vertices(index,vertices)
    }

    pub fn write_heap_simple_object_indices(&mut self,index:ObjectIDType,indices:&[ElementIndexType]){
        self.simple.write_heap_object_indices(index,indices)
    }

    pub fn draw_heap_simple_object(&self,index:ObjectIDType){
        self.simple.draw_heap_object(index,&self.draw_parameters);
    }

    pub fn push_simple_object_raw(
        &mut self,
        vertices:&[SimpleVertex2D],
        indices:&[ElementIndexType],
        primitive_type:PrimitiveType
    )->Option<ObjectIDType>{
        self.simple.push_object_raw(
            vertices,
            indices,
            primitive_type as u32
        )
    }

    pub fn pop_simple_object(&mut self){
        self.simple.pop_object();
    }

    pub fn clear_stack_simple_objects(&mut self){
        self.simple.clear_stack()
    }

    pub fn write_stack_simple_object_vertices(&mut self,index:ObjectIDType,vertices:&[SimpleVertex2D]){
        self.simple.write_stack_object_vertices(index,vertices)
    }

    pub fn write_stack_simple_object_indices(&mut self,index:ObjectIDType,indices:&[ElementIndexType]){
        self.simple.write_stack_object_indices(index,indices)
    }

    pub fn draw_stack_simple_object(&self,index:ObjectIDType){
        self.simple.draw_stack_object(index,&self.draw_parameters);
    }
}

/// Texture graphics.
#[cfg(feature="texture_graphics")]
impl Graphics2D{
    pub fn add_textured_object_raw(
        &mut self,
        vertices:&[TexturedVertex2D],
        indices:&[ElementIndexType],
        primitive_type:PrimitiveType
    )->Option<ObjectIDType>{
        self.texture.add_object_raw(
            vertices,
            indices,
            primitive_type as u32
        )
    }

    /// Removes an object.
    /// 
    /// It's not actually removes it, just clears it's data.
    pub fn remove_textured_object(&mut self,index:ObjectIDType){
        self.texture.remove_object(index);
    }

    pub fn write_heap_textured_object_vertices(&mut self,index:ObjectIDType,vertices:&[TexturedVertex2D]){
        self.texture.write_heap_object_vertices(index,vertices)
    }

    pub fn write_heap_textured_object_indices(&mut self,index:ObjectIDType,indices:&[ElementIndexType]){
        self.texture.write_heap_object_indices(index,indices)
    }

    pub fn draw_heap_textured_object(&self,index:ObjectIDType,texture:&Texture2D){
        self.texture.draw_heap_object(index,texture,&self.draw_parameters);
    }

    pub fn push_textured_object_raw(
        &mut self,
        vertices:&[TexturedVertex2D],
        indices:&[ElementIndexType],
        primitive_type:PrimitiveType
    )->Option<ObjectIDType>{
        self.texture.push_object_raw(
            vertices,
            indices,
            primitive_type as u32
        )
    }

    pub fn pop_textured_object(&mut self){
        self.texture.pop_object();
    }

    pub fn clear_stack_textured_objects(&mut self){
        self.texture.clear_stack()
    }

    pub fn write_stack_textured_object_vertices(&mut self,index:ObjectIDType,vertices:&[TexturedVertex2D]){
        self.texture.write_stack_object_vertices(index,vertices)
    }

    pub fn write_stack_textured_object_indices(&mut self,index:ObjectIDType,indices:&[ElementIndexType]){
        self.texture.write_stack_object_indices(index,indices)
    }

    pub fn draw_stack_textured_object(&self,index:ObjectIDType,texture:&Texture2D){
        self.texture.draw_stack_object(index,texture,&self.draw_parameters);
    }
}