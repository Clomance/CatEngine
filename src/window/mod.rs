#[macro_use]
mod event_handlers; // макросы для обратки событий

mod window_base;
pub (crate) use window_base::WindowBase;

mod window_page;
pub use window_page::WindowPage;

mod window;
pub use window::Window;

mod settings;
pub use settings::*;

mod mouse_cursor;
pub use mouse_cursor::MouseCursor;

use glium::glutin::event::{
    ModifiersState,
    MouseScrollDelta,
    MouseButton,
};

use std::path::PathBuf;

/// Положение курсора мыши. The mouse cursor position.
pub static mut mouse_cursor:MouseCursor=MouseCursor::new();

/// Ширина окна. The window width.
pub static mut window_width:f32=0f32;
/// Высота окна. The window height.
pub static mut window_height:f32=0f32;
/// Центр окна. The window center. [x, y]
pub static mut window_center:[f32;2]=[0f32;2];

/// Счётчик кадров в секунду. A frame per seconds counter. feature = "fps_counter"
/// 
/// Обновляется раз в секунду. Updates once a second.
#[cfg(feature="fps_counter")]
pub static mut fps:u32=0;

#[derive(PartialEq)]
pub (crate) enum EventLoopState<O:PartialEq>{
    Running,
    CloseRequested,
    Closed(O),
}

/// Внутренние события для управления окном.
/// Inner events to operate the window.
#[derive(Clone,Debug)]
pub enum InnerWindowEvent{
    /// Emitted with `stop_events()` function.
    EventLoopCloseRequested,
    Update,
}

/// Внешние события окна.
/// Outer window events.
#[derive(Clone,Debug)]
pub enum WindowEvent{
    /// feature != "lazy"
    #[cfg(not(feature="lazy"))]
    Update,

    /// Кадр окна можно обновить.
    /// 
    /// The window should be redrawn.
    RedrawRequested,

    /// The window has been requested to close.
    CloseRequested,

    /// Event loop has been stopped,
    /// means that a page (closure) will be closed.
    EventLoopClosed,

    /// Приложение приостановлено.
    /// 
    /// Emitted when the application has been suspended.
    Suspended,
    /// Приложение возобновлено.
    /// 
    /// Emitted when the application has been resumed.
    Resumed,

    /// Окно получило или потеряло фокус.
    /// True - получило, false - потеряло.
    /// 
    /// The window gained or lost focus.
    /// The parameter is true if the window has gained focus,
    /// and false if it has lost focus.
    Focused(bool),

    /// Размера окна изменён.
    /// Содержит новый размер.
    /// 
    /// The size of the window has changed.
    /// Contains the client area's new dimensions.
    Resized([u32;2]),

    /// Окно сдвинуто.
    /// Содержит новую позицию.
    /// 
    /// The position of the window has changed.
    /// Contains the window's new position.
    Moved([i32;2]),

    /// Сдвиг мышки (сдвиг за пределы экрана игнорируется).
    /// 
    /// Mouse movement (moving beyond the window border is ignored).
    MouseMovementDelta([f32;2]),

    /// Describes a difference in the mouse scroll wheel state.
    MouseWheelScroll(MouseScrollDelta),
    MousePressed(MouseButton),
    MouseReleased(MouseButton),

    KeyboardPressed(KeyboardButton),
    KeyboardReleased(KeyboardButton),
    CharacterInput(char),

    /// Состояние Shift, Ctrl, Alt или Logo изменено.
    /// 
    /// Shift, Ctrl, Alt or Logo state has been changed.
    ModifiersChanged(ModifiersState),

    /// A file has been dropped into the window.
    /// When the user drops multiple files at once,
    /// this event will be emitted for each file separately.
    /// 
    /// feature = "file_drop"
    #[cfg(feature="file_drop")]
    DroppedFile(PathBuf),
    /// A file is being hovered over the window.
    /// When the user hovers multiple files at once,
    /// this event will be emitted for each file separately.
    ///
    /// feature = "file_drop"
    #[cfg(feature="file_drop")]
    HoveredFile(PathBuf),
    /// A file was hovered, but has exited the window.
    /// There will be a single HoveredFileCancelled event triggered even
    /// if multiple files were hovered.
    /// 
    /// feature = "file_drop"
    #[cfg(feature="file_drop")]
    HoveredFileCancelled,
}

#[derive(Clone,PartialEq,Debug)]
#[repr(u32)]
pub enum KeyboardButton{
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    Screenshot,
    Scroll,
    Pause,
    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,
    Left,
    Up,
    Right,
    Down,
    Backspace,
    Enter,
    Space,
    Compose,
    Caret,
    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    AbntC1,
    AbntC2,
    Add,
    Apostrophe,
    Apps,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
    Decimal,
    Divide,
    Equals,
    Grave,
    Kana,
    Kanji,
    LeftAlt,
    LeftBracket,
    LeftControl,
    LeftShift,
    LeftWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    Multiply,
    Mute,
    MyComputer,
    NavigateForward,
    NavigateBackward,
    NextTrack,
    NoConvert,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    OEM102,
    Period,
    PlayPause,
    Power,
    PrevTrack,
    RightAlt,
    RightBracket,
    RightControl,
    RightShift,
    RightWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    Subtract,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
    Copy,
    Paste,
    Cut,
    Unknown,
}