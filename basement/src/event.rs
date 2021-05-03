#[cfg(target_os="windows")]
use crate::windows::{
    Ticks,
    VirtualKeyCode,
};

#[derive(Debug)]
pub enum Event{
    EventLoopStart,

    Redraw,

    WindowEvent{
        window_event:WindowEvent,
        window_id:usize,
    },

    #[cfg(target_os="windows")]
    Update(Ticks),

    EventLoopBreak,

    /// After this event the cycle closes forever.
    #[cfg(target_os="windows")]
    EventLoopExit,
}

unsafe impl Sync for Event{}
unsafe impl Send for Event{}

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
    // Redraw,

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
