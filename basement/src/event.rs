#[cfg(target_os="windows")]
use crate::windows::{
    Ticks,
    VirtualKeyCode,
};

#[derive(Debug)]
pub enum ProcessEvent{
    EventLoopStart,

    #[cfg(target_os="windows")]
    Update(Ticks),
    /// The current thread has made a request to close.
    #[cfg(target_os="windows")]
    Quit,

    EventLoopBreak,
}

#[derive(Debug)]
pub enum WindowEvent{
    MouseMove([u16;2]),
    MousePress{
        cursor_position:[u16;2],
        button:MouseButton,
    },
    MouseRelease{
        cursor_position:[u16;2],
        button:MouseButton,
    },

    MouseScroll(i16),

    #[cfg(target_os="windows")]
    KeyPress(VirtualKeyCode),
    #[cfg(target_os="windows")]
    KeyRelease(VirtualKeyCode),
    CharacterInput(char),

    Redraw,

    Resize([u16;2]),

    Move([i16;2]),

    CloseRequest,

    Destroy,
}

/// Describes mouse buttons.
/// 
/// Описывает кнопки мыши.
#[derive(Debug,Clone,Copy)]
pub enum MouseButton{
    Left,
    Middle,
    Right,
    Button4,
    Button5,
}