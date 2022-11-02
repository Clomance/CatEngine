use crate::{graphics::{Graphics, RenderData, ElementIndexType, TexturedVertex, TextVertex, SimpleVertex}, texture::Texture2D, text::GlyphCache};

use super::ObjectEvent;

#[derive(Clone)]
pub struct RenderDataInterface{
    graphics:*mut Graphics,
    layer:usize,
    object:usize,
}

impl RenderDataInterface{
    pub (crate) fn new(
        graphics:*mut Graphics,
        layer:usize,
        object:usize,
    )->RenderDataInterface{
        Self{
            graphics,
            layer,
            object,
        }
    }
}



#[repr(transparent)]
pub struct SimpleRenderDataInterface{
    inner:RenderDataInterface,
}

impl SimpleRenderDataInterface{
    pub (crate) fn new(
        render_data:RenderDataInterface,
    )->SimpleRenderDataInterface{
        Self{
            inner:render_data
        }
    }

    pub fn get_render_data(&self)->RenderData<SimpleVertex,ElementIndexType>{
        unsafe{&mut *self.inner.graphics}.simple.get_render_data(self.inner.layer,self.inner.object).unwrap()
    }
}

pub trait SimpleObject:Sized+'static{
    fn event(&mut self,event:ObjectEvent,render_data:SimpleRenderDataInterface);
}



pub struct TextureRenderDataInterface{
    inner:RenderDataInterface
}

impl TextureRenderDataInterface{
    pub (crate) fn new(
        render_data:RenderDataInterface,
    )->TextureRenderDataInterface{
        Self{
            inner:render_data
        }
    }

    pub fn get_render_data(&self)->RenderData<TexturedVertex,ElementIndexType>{
        unsafe{&mut *self.inner.graphics}.texture.get_render_data(self.inner.layer,self.inner.object)
    }

    pub fn get_texture(&self)->&Texture2D{
        unsafe{
            &*(&mut *self.inner.graphics).texture.get_layer_texture_raw(self.inner.layer)
        }
    }
}

pub trait TextureObject:Sized+'static{
    fn event(&mut self,event:ObjectEvent,render_data:TextureRenderDataInterface);
}



pub struct TextRenderDataInterface{
    inner:RenderDataInterface,
}

impl TextRenderDataInterface{
    pub (crate) fn new(
        render_data:RenderDataInterface,
    )->TextRenderDataInterface{
        Self{
            inner:render_data
        }
    }

    pub fn get_render_data(&self)->RenderData<TextVertex,ElementIndexType>{
        unsafe{&mut *self.inner.graphics}.text.get_render_data(self.inner.layer,self.inner.object)
    }

    pub fn get_font(&self)->&GlyphCache{
        unsafe{
            &*(&mut *self.inner.graphics).text.get_layer_font_raw(self.inner.layer)
        }
    }
}

pub trait TextObject:Sized+'static{
    fn event(&mut self,event:ObjectEvent,render_data:TextRenderDataInterface);
}



pub (crate) trait ObjectInterface{
    fn event(&mut self,event:ObjectEvent,render_data:RenderDataInterface);
}

pub struct SimpleWrapper<O>(pub O);

impl<O:SimpleObject> ObjectInterface for SimpleWrapper<O>{
    fn event(&mut self,event:ObjectEvent,render_data:RenderDataInterface){
        {
            let render_data=SimpleRenderDataInterface::new(render_data);

            <O as SimpleObject>::event(&mut self.0,event,render_data)
        }
    }
}

pub struct TextureWrapper<O>(pub O);

impl<O:TextureObject> ObjectInterface for TextureWrapper<O>{
    fn event(&mut self,event:ObjectEvent,render_data:RenderDataInterface){
        {
            let render_data=TextureRenderDataInterface::new(render_data);

            <O as TextureObject>::event(&mut self.0,event,render_data)
        }
    }
}

pub struct TextWrapper<O>(pub O);

impl<O:TextObject> ObjectInterface for TextWrapper<O>{
    fn event(&mut self,event:ObjectEvent,render_data:RenderDataInterface){
        {
            let render_data=TextRenderDataInterface::new(render_data);

            <O as TextObject>::event(&mut self.0,event,render_data)
        }
    }
}



pub fn drop_simple_object(render_data:RenderDataInterface){
    unsafe{
        (*render_data.graphics).simple.remove_object(render_data.layer,render_data.object)
    }
}

pub fn drop_texture_object(render_data:RenderDataInterface){
    unsafe{
        (*render_data.graphics).texture.remove_object(render_data.layer,render_data.object)
    }
}

pub fn drop_text_object(render_data:RenderDataInterface){
    unsafe{
        (*render_data.graphics).text.remove_object(render_data.layer,render_data.object)
    }
}