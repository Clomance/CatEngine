use crate::{
    texture::{ImageBase,Texture},
    graphics::{
        Graphics,
        Graphics2D,
        DependentObject,
        TexturedVertex2D
    },
};

use super::{
    mouse_cursor,
    window_center,
    MouseCursorIconSettings
};

use glium::{
    uniform,
    Display,
    DrawParameters,
    VertexBuffer,
    index::{
        PrimitiveType, // enum
        NoIndices,
    },
    Surface,
};

use std::path::Path;

/// Положение курсора мыши.
/// The mouse cursor position.
pub struct MouseCursor{
    position:[f32;2],
    saved_position:[f32;2],
}

impl MouseCursor{
    /// Инициирует новую позицию курсора.
    /// 
    /// Initiates new cursor position.
    pub const fn new()->MouseCursor{
        Self{
            position:[0f32;2],
            saved_position:[0f32;2],
        }
    }

    /// Сохраняет текущую позицию курсора мыши.
    /// 
    /// Saves the current mouse cursor position.
    pub fn save_position(&mut self){
        self.saved_position=self.position
    }

    /// Вычисляет перемещение от сохранённой позиции.
    /// 
    /// Calculates the movement from the saved position.
    pub fn saved_shift(&self)->[f32;2]{
        [
            self.position[0]-self.saved_position[0],
            self.position[1]-self.saved_position[1]
        ]
    }

    #[inline(always)]
    pub fn x(&self)->f32{
        self.position[0]
    }

    #[inline(always)]
    pub fn y(&self)->f32{
        self.position[1]
    }

    /// Позиция курсора мыши.
    /// 
    /// The mouse cursor position.
    #[inline(always)]
    pub fn position(&self)->[f32;2]{
        self.position
    }

    /// Расстояние от курсора до центра окна.
    /// 
    /// The distance between the cursor and the center of the window.
    pub fn center_radius(&self)->[f32;2]{
        unsafe{[
            self.position[0]-window_center[0],
            self.position[1]-window_center[1]
        ]}
    }

    /// Уставливает позицию курсора мыши.
    /// 
    /// Sets the mouse cursor position.
    #[inline(always)]
    pub (crate) fn set_position(&mut self,position:[f32;2]){
        self.position=position;
    }
}

const d_radius:f32=5f32;

/// Иконка курсора мышки.
#[cfg(feature="mouse_cursor_icon")]
pub struct MouseCursorIcon{
    vertex_buffer:VertexBuffer<TexturedVertex2D>,
    image_base:ImageBase,
    texture:Texture,
    visible:bool,
}

#[cfg(feature="mouse_cursor_icon")]
impl MouseCursorIcon{
    pub fn new<P:AsRef<Path>>(settings:MouseCursorIconSettings<P>,display:&Display)->MouseCursorIcon{
        let image_base=ImageBase::new([1f32;4],
            unsafe{[
                window_center[0]+settings.shift[0],
                window_center[1]+settings.shift[1],
                settings.size[0],
                settings.size[1]
            ]}
        );

        Self{
            vertex_buffer:VertexBuffer::new(display,&image_base.vertices()).unwrap(),
            image_base,
            texture:Texture::from_path(settings.path,display).expect("Loading mouse curcor image error"),
            visible:true,
        }
    }

    pub fn update(&self){
        self.vertex_buffer.as_slice().write(&self.image_base.vertices())
    }

    #[inline(always)]
    pub fn set_visible(&mut self,visible:bool){
        self.visible=visible
    }

    #[inline(always)]
    pub fn switch_visibility(&mut self){
        self.visible=!self.visible
    }

    /// При нажатии кнопки мыши.
    /// 
    /// On a mouse button pressed.
    pub fn pressed(&mut self){
        self.image_base.x1+=d_radius;
        self.image_base.y1+=d_radius;
        self.image_base.x2-=d_radius;
        self.image_base.y2-=d_radius;
        self.vertex_buffer.as_slice().write(&self.image_base.vertices())
    }

    /// При освобождении кнопки мыши.
    /// 
    /// On a mouse button released.
    pub fn released(&mut self){
        self.image_base.x1-=d_radius;
        self.image_base.y1-=d_radius;
        self.image_base.x2+=d_radius;
        self.image_base.y2+=d_radius;
        self.vertex_buffer.as_slice().write(&self.image_base.vertices())
    }

    #[inline(always)]
    pub fn draw(&self,draw_parameters:&mut DrawParameters,graphics:&mut Graphics){
        if self.visible{
            let index=NoIndices(PrimitiveType::TriangleStrip);

            let uni=uniform!{
                texture2d:&self.texture.0,
                shift:unsafe{mouse_cursor.center_radius()},
                window_center:unsafe{window_center},
            };

            graphics.frame.draw(
                &self.vertex_buffer,
                index,
                &graphics.graphics2d.texture.draw_shift,
                &uni,
                draw_parameters
            ).expect("Mouse cursor icon rendering error");
        }
    }
}