use crate::windows::{
    WindowReference,
};

use super::Ticks;

#[derive(Debug,Clone)]
pub enum Event{
    EventLoopStart,

    WindowEvent{
        window_reference:WindowReference,
        window_event:WindowEvent,
        argument:u64,
    },

    Update(Ticks),

    EventLoopBreak,

    EventLoopClose,
}

unsafe impl Sync for Event{}
unsafe impl Send for Event{}

#[derive(Debug,Clone)]
pub enum WindowEvent{
    MouseMove([u16;2]),
    MousePress(VirtualKeyCode),
    MouseRelease(VirtualKeyCode),

    KeyPress(VirtualKeyCode),
    KeyRelease(VirtualKeyCode),

    Redraw,

    Resize([u16;2]),

    Move([i16;2]),

    CloseRequest,

    Destroy,
}



pub enum MouseButton{
    Left,
    Middle,
    Right,
}

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

#[derive(Debug,Clone,Copy)]
pub enum VirtualKeyCode{
    Null,
    VK_LBUTTON, // Left mouse button
    VK_RBUTTON, // Right mouse button
    VK_CANCEL, // Control-break processing
    VK_MBUTTON, // Middle mouse button (three-button mouse)
    VK_XBUTTON1, // X1 mouse button
    VK_XBUTTON2, // X2 mouse button
    Undefined0x07,
    VK_BACK, // BACKSPACE key
    VK_TAB, // TAB key
    Reserved0x0A,
    Reserved0x0B,
    VK_CLEAR, // CLEAR key
    VK_RETURN, // ENTER key
    Undefined0x0E,
    Undefined0x0F,
    VK_SHIFT, // SHIFT key
    VK_CONTROL, // CTRL key
    VK_MENU, // ALT key
    VK_PAUSE, // PAUSE key
    VK_CAPITAL, // CAPS LOCK key,
    VK_KANA, // VK_HANGUEL, VK_HANGUL - IME Kana mode, IME Hanguel mode, IME Hangul mode
    VK_IME_ON, // IME On
    VK_JUNJA, // IME Junja mode
    VK_FINAL, // IME final mode
    VK_HANJA, // VK_KANJI -  IME Hanja mode, IME Kanji mode
    VK_IME_OFF, // IME Off
    VK_ESCAPE, // ESC key
    VK_CONVERT, // IME convert
    VK_NONCONVERT, // IME nonconvert
    VK_ACCEPT, // IME accept
    VK_MODECHANGE, // IME mode change request
    VK_SPACE, // SPACEBAR
    VK_PRIOR, // PAGE UP key
    VK_NEXT, // PAGE DOWN key
    VK_END, // END key
    VK_HOME, // HOME key
    VK_LEFT, // LEFT ARROW key
    VK_UP, // UP ARROW key
    VK_RIGHT, // RIGHT ARROW key
    VK_DOWN, // DOWN ARROW key
    VK_SELECT, // SELECT key
    VK_PRINT, // PRINT key
    VK_EXECUTE, // EXECUTE key
    VK_SNAPSHOT, // PRINT SCREEN key
    VK_INSERT, // INS key
    VK_DELETE, // DEL key
    VK_HELP, // HELP key
    Zero, // 0 key
    One, // 1 key
    Two, // 2 key
    Three, // 3 key
    Four, // 4 key
    Five, // 5 key
    Six, // 6 key
    Seven, // 7 key
    Eight, // 8 key
    Nine, // 9 key
    Undefined0x3A,
    Undefined0x3B,
    Undefined0x3C,
    Undefined0x3D,
    Undefined0x3E,
    Undefined0x3F,
    Undefined0x40,
    A, // A key
    B, // B key
    C, // C key
    D, // D key
    E, // E key
    F, // F key
    G, // G key
    H, // H key
    I, // I key
    J, // J key
    K, // K key
    L, // L key
    M, // M key
    N, // N key
    O, // O key
    P, // P key
    Q, // Q key
    R, // R key
    S, // S key
    T, // T key
    U, // U key
    V, // V key
    W, // W key
    X, // X key
    Y, // Y key
    Z, // Z key
    VK_LWIN, // Left Windows key (Natural keyboard)
    VK_RWIN, // Right Windows key (Natural keyboard)
    VK_APPS,  // Applications key (Natural keyboard)
    Reserved0x5E,
    VK_SLEEP, // Computer Sleep key
    VK_NUMPAD0, // Numeric keypad 0 key
    VK_NUMPAD1, // Numeric keypad 1 key
    VK_NUMPAD2, // Numeric keypad 2 key
    VK_NUMPAD3, // Numeric keypad 3 key
    VK_NUMPAD4, // Numeric keypad 4 key
    VK_NUMPAD5, // Numeric keypad 5 key
    VK_NUMPAD6, // Numeric keypad 6 key
    VK_NUMPAD7, // Numeric keypad 7 key
    VK_NUMPAD8, // Numeric keypad 8 key
    VK_NUMPAD9, // Numeric keypad 9 key
    VK_MULTIPLY, // Multiply key
    VK_ADD, // Add key
    VK_SEPARATOR, // Separator key
    VK_SUBTRACT, // Subtract key
    VK_DECIMAL, // Decimal key
    VK_DIVIDE, // Divide key
    VK_F1, // F1 key
    VK_F2, // F2 key
    VK_F3, // // F3 key
    VK_F4, // F4 key
    VK_F5, // F5 key
    VK_F6, // F6 key
    VK_F7, // F7 key
    VK_F8, // F8 key
    VK_F9, // F9 key
    VK_F10, // F10 key
    VK_F11, // F11 key
    VK_F12, // F12 key
    VK_F13, // F13 key
    VK_F14, // F14 key
    VK_F15, // F15 key
    VK_F16, // F16 key
    VK_F17, // F17 key
    VK_F18, // F18 key
    VK_F19, // F19 key
    VK_F20, // F20 key
    VK_F21, // F21 key
    VK_F22, // F22 key
    VK_F23, // F23 key
    VK_F24, // F24 key
    Unassigned0x88,
    Unassigned0x89,
    Unassigned0x8A,
    Unassigned0x8B,
    Unassigned0x8C,
    Unassigned0x8D,
    Unassigned0x8E,
    Unassigned0x8F,
    VK_NUMLOCK, // NUM LOCK key
    VK_SCROLL, // SCROLL LOCK key
    OEM_Specific0x92,
    OEM_Specific0x93,
    OEM_Specific0x94,
    OEM_Specific0x95,
    OEM_Specific0x96,
    Unassigned0x97,
    Unassigned0x98,
    Unassigned0x99,
    Unassigned0x9A,
    Unassigned0x9B,
    Unassigned0x9C,
    Unassigned0x9D,
    Unassigned0x9E,
    Unassigned0x9F,
    VK_LSHIFT, // Left SHIFT key
    VK_RSHIFT, // Right SHIFT key
    VK_LCONTROL, // Left CONTROL key
    VK_RCONTROL, // Right CONTROL key
    VK_LMENU, // Left MENU key
    VK_RMENU, // Right MENU key
    VK_BROWSER_BACK, // Browser Back key
    VK_BROWSER_FORWARD, // Browser Forward key
    VK_BROWSER_REFRESH, // Browser Refresh key
    VK_BROWSER_STOP, // Browser Stop key
    VK_BROWSER_SEARCH, // Browser Search key
    VK_BROWSER_FAVORITES, // Browser Favorites key
    VK_BROWSER_HOME, // Browser Start and Home key
    VK_VOLUME_MUTE, // Volume Mute key
    VK_VOLUME_DOWN, // Volume Down key
    VK_VOLUME_UP, // Volume Up key
    VK_MEDIA_NEXT_TRACK, // Next Track key
    VK_MEDIA_PREV_TRACK, // Previous Track key
    VK_MEDIA_STOP, // Stop Media key
    VK_MEDIA_PLAY_PAUSE, // Play/Pause Media key
    VK_LAUNCH_MAIL, // Start Mail key
    VK_LAUNCH_MEDIA_SELECT, // Select Media key
    VK_LAUNCH_APP1, // Start Application 1 key
    VK_LAUNCH_APP2, // Start Application 2 key
    ReservedB8,
    ReservedB9,
    VK_OEM_1, // Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the ';:' key
    VK_OEM_PLUS, // For any country/region, the '+' key
    VK_OEM_COMMA, // For any country/region, the ',' key
    VK_OEM_MINUS, // For any country/region, the '-' key
    VK_OEM_PERIOD, // For any country/region, the '.' key
    VK_OEM_2, // Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the '/?' key
    VK_OEM_3, // Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the '`~' key
    Reserved0xC1,
    Reserved0xC2,
    Reserved0xC3,
    Reserved0xC4,
    Reserved0xC5,
    Reserved0xC6,
    Reserved0xC7,
    Reserved0xC8,
    Reserved0xC9,
    Reserved0xCA,
    Reserved0xCB,
    Reserved0xCC,
    Reserved0xCD,
    Reserved0xCE,
    Reserved0xCF,
    Reserved0xD0,
    Reserved0xD1,
    Reserved0xD2,
    Reserved0xD3,
    Reserved0xD4,
    Reserved0xD5,
    Reserved0xD6,
    Reserved0xD7,
    Unassigned0xD8,
    Unassigned0xD9,
    Unassigned0xDA,
    VK_OEM_4, // Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the '[{' key
    VK_OEM_5, // Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the '\|' key
    VK_OEM_6, // Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the ']}' key
    VK_OEM_7, // Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the 'single-quote/double-quote' key
    VK_OEM_8, // Used for miscellaneous characters; it can vary by keyboard.
    Reserved0xE0,
    OEM_Specific0xE1,
    VK_OEM_102, // Either the angle bracket key or the backslash key on the RT 102-key keyboard
    OEM_Specific0xE3,
    OEM_Specific0xE4,
    VK_PROCESSKEY, // IME PROCESS key
    OEM_Specific0xE6,
    VK_PACKET, // Used to pass Unicode characters as if they were keystrokes. The VK_PACKET key is the low word of a 32-bit Virtual Key value used for non-keyboard input methods. For more information, see Remark in KEYBDINPUT, SendInput, WM_KEYDOWN, and WM_KEYUP
    Unassigned0xE8,
    OEM_Specific0xE9,
    OEM_Specific0xEA,
    OEM_Specific0xEB,
    OEM_Specific0xEC,
    OEM_Specific0xED,
    OEM_Specific0xEE,
    OEM_Specific0xEF,
    OEM_Specific0xF0,
    OEM_Specific0xF1,
    OEM_Specific0xF2,
    OEM_Specific0xF3,
    OEM_Specific0xF4,
    OEM_Specific0xF5,
    VK_ATTN, // Attn key
    VK_CRSEL, // CrSel key
    VK_EXSEL,// ExSel key
    VK_EREOF, // Erase EOF key
    VK_PLAY, // Play key
    VK_ZOOM, // Zoom key
    VK_NONAME, // Reserved
    VK_PA1, // PA1 key
    VK_OEM_CLEAR, // Clear key
}