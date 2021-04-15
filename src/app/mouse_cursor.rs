use super::window_center;

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