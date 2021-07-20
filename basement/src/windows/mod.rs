mod error;
pub use error::WinError;

mod images;
pub use images::{
    Bitmap,
    Icon,
};

mod opengl;
pub use opengl::{
    OpenGraphicsLibrary,
    OpenGLRenderContext,
    OpenGLRenderContextAttributes,
};

mod monitor;
pub use monitor::Monitor;

mod window;
pub use window::{
    WindowProcedure,
    Fullscreen,
    Window,
    WindowAttributes,
    CursorIcon,
    Background,
    WindowClass,
    WindowClassAttributes,
    // functions
    quit
};

mod event_loop;
pub use event_loop::{
    Ticks,
    EventLoop,
    LoopControl,
    EventInterval,
    EventLoopAttributes,
};

pub use winapi;

#[derive(Debug,Clone,Copy)]
pub enum VirtualKeyCode{
    Null,
    /// Left mouse button
    VK_LBUTTON,
    /// Right mouse button
    VK_RBUTTON,
    /// Control-break processing
    VK_CANCEL,
    /// Middle mouse button (three-button mouse)
    VK_MBUTTON,
    /// X1 mouse button
    VK_XBUTTON1,
    /// X2 mouse button
    VK_XBUTTON2,
    Undefined0x07,
    /// BACKSPACE key
    VK_BACK,
    /// TAB key
    VK_TAB,
    Reserved0x0A,
    Reserved0x0B,
    /// CLEAR key
    VK_CLEAR,
    /// ENTER key
    VK_RETURN,
    Undefined0x0E,
    Undefined0x0F,
    /// SHIFT key
    VK_SHIFT,
    /// CTRL key
    VK_CONTROL,
    /// ALT key
    VK_MENU,
    /// PAUSE key
    VK_PAUSE,
    /// CAPS LOCK key,
    VK_CAPITAL,
    /// VK_HANGUEL, VK_HANGUL - IME Kana mode, IME Hanguel mode, IME Hangul mode
    VK_KANA,
    /// IME On
    VK_IME_ON,
    /// IME Junja mode
    VK_JUNJA,
    /// IME final mode
    VK_FINAL,
    /// VK_KANJI -  IME Hanja mode, IME Kanji mode
    VK_HANJA,
    /// IME Off
    VK_IME_OFF,
    /// ESC key
    VK_ESCAPE,
    /// IME convert
    VK_CONVERT,
    /// IME nonconvert
    VK_NONCONVERT,
    /// IME accept
    VK_ACCEPT,
    /// IME mode change request
    VK_MODECHANGE,
    /// SPACEBAR
    VK_SPACE,
    /// PAGE UP key
    VK_PRIOR,
    /// PAGE DOWN key
    VK_NEXT,
    /// END key
    VK_END,
    /// HOME key
    VK_HOME,
    /// LEFT ARROW key
    VK_LEFT,
    /// UP ARROW key
    VK_UP,
    /// RIGHT ARROW key
    VK_RIGHT,
    /// DOWN ARROW key
    VK_DOWN,
    /// SELECT key
    VK_SELECT,
    /// PRINT key
    VK_PRINT,
    /// EXECUTE key
    VK_EXECUTE,
    /// PRINT SCREEN key
    VK_SNAPSHOT,
    /// INS key
    VK_INSERT,
    /// DEL key
    VK_DELETE,
    /// HELP key
    VK_HELP,
    /// 0 key
    Zero,
    /// 1 key
    One,
    /// 2 key
    Two,
    /// 3 key
    Three,
    /// 4 key
    Four,
    /// 5 key
    Five,
    /// 6 key
    Six,
    /// 7 key
    Seven,
    /// 8 key
    Eight,
    /// 9 key
    Nine,
    Undefined0x3A,
    Undefined0x3B,
    Undefined0x3C,
    Undefined0x3D,
    Undefined0x3E,
    Undefined0x3F,
    Undefined0x40,
    /// A key
    A,
    /// B key
    B,
    /// C key
    C,
    /// D key
    D,
    /// E key
    E,
    /// F key
    F,
    /// G key
    G,
    /// H key
    H,
    /// I key
    I,
    /// J key
    J,
    /// K key
    K,
    /// L key
    L,
    /// M key
    M,
    /// N key
    N,
    /// O key
    O,
    /// P key
    P,
    /// Q key
    Q,
    /// R key
    R,
    /// S key
    S,
    /// T key
    T,
    /// U key
    U,
    /// V key
    V,
    /// W key
    W,
    /// X key
    X,
    /// Y key
    Y,
    /// Z key
    Z,
    /// Left Windows key (Natural keyboard)
    VK_LWIN,
    /// Right Windows key (Natural keyboard)
    VK_RWIN,
    /// Applications key (Natural keyboard)
    VK_APPS, 
    Reserved0x5E,
    /// Computer Sleep key
    VK_SLEEP,
    /// Numeric keypad 0 key
    VK_NUMPAD0,
    /// Numeric keypad 1 key
    VK_NUMPAD1,
    /// Numeric keypad 2 key
    VK_NUMPAD2,
    /// Numeric keypad 3 key
    VK_NUMPAD3,
    /// Numeric keypad 4 key
    VK_NUMPAD4,
    /// Numeric keypad 5 key
    VK_NUMPAD5,
    /// Numeric keypad 6 key
    VK_NUMPAD6,
    /// Numeric keypad 7 key
    VK_NUMPAD7,
    /// Numeric keypad 8 key
    VK_NUMPAD8,
    /// Numeric keypad 9 key
    VK_NUMPAD9,
    /// Multiply key
    VK_MULTIPLY,
    /// Add key
    VK_ADD,
    /// Separator key
    VK_SEPARATOR,
    /// Subtract key
    VK_SUBTRACT,
    /// Decimal key
    VK_DECIMAL,
    /// Divide key
    VK_DIVIDE,
    /// F1 key
    VK_F1,
    /// F2 key
    VK_F2,
    /// /// F3 key
    VK_F3,
    /// F4 key
    VK_F4,
    /// F5 key
    VK_F5,
    /// F6 key
    VK_F6,
    /// F7 key
    VK_F7,
    /// F8 key
    VK_F8,
    /// F9 key
    VK_F9,
    /// F10 key
    VK_F10,
    /// F11 key
    VK_F11,
    /// F12 key
    VK_F12,
    /// F13 key
    VK_F13,
    /// F14 key
    VK_F14,
    /// F15 key
    VK_F15,
    /// F16 key
    VK_F16,
    /// F17 key
    VK_F17,
    /// F18 key
    VK_F18,
    /// F19 key
    VK_F19,
    /// F20 key
    VK_F20,
    /// F21 key
    VK_F21,
    /// F22 key
    VK_F22,
    /// F23 key
    VK_F23,
    /// F24 key
    VK_F24,
    Unassigned0x88,
    Unassigned0x89,
    Unassigned0x8A,
    Unassigned0x8B,
    Unassigned0x8C,
    Unassigned0x8D,
    Unassigned0x8E,
    Unassigned0x8F,
    /// NUM LOCK key
    VK_NUMLOCK,
    /// SCROLL LOCK key
    VK_SCROLL,
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
    /// Left SHIFT key
    VK_LSHIFT,
    /// Right SHIFT key
    VK_RSHIFT,
    /// Left CONTROL key
    VK_LCONTROL,
    /// Right CONTROL key
    VK_RCONTROL,
    /// Left MENU key
    VK_LMENU,
    /// Right MENU key
    VK_RMENU,
    /// Browser Back key
    VK_BROWSER_BACK,
    /// Browser Forward key
    VK_BROWSER_FORWARD,
    /// Browser Refresh key
    VK_BROWSER_REFRESH,
    /// Browser Stop key
    VK_BROWSER_STOP,
    /// Browser Search key
    VK_BROWSER_SEARCH,
    /// Browser Favorites key
    VK_BROWSER_FAVORITES,
    /// Browser Start and Home key
    VK_BROWSER_HOME,
    /// Volume Mute key
    VK_VOLUME_MUTE,
    /// Volume Down key
    VK_VOLUME_DOWN,
    /// Volume Up key
    VK_VOLUME_UP,
    /// Next Track key
    VK_MEDIA_NEXT_TRACK,
    /// Previous Track key
    VK_MEDIA_PREV_TRACK,
    /// Stop Media key
    VK_MEDIA_STOP,
    /// Play/Pause Media key
    VK_MEDIA_PLAY_PAUSE,
    /// Start Mail key
    VK_LAUNCH_MAIL,
    /// Select Media key
    VK_LAUNCH_MEDIA_SELECT,
    /// Start Application 1 key
    VK_LAUNCH_APP1,
    /// Start Application 2 key
    VK_LAUNCH_APP2,
    ReservedB8,
    ReservedB9,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the ';:' key
    VK_OEM_1,
    /// For any country/region, the '+' key
    VK_OEM_PLUS,
    /// For any country/region, the ',' key
    VK_OEM_COMMA,
    /// For any country/region, the '-' key
    VK_OEM_MINUS,
    /// For any country/region, the '.' key
    VK_OEM_PERIOD,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the '/?' key
    VK_OEM_2,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the '`~' key
    VK_OEM_3,
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
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the '[{' key
    VK_OEM_4,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the '\|' key
    VK_OEM_5,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the ']}' key
    VK_OEM_6,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the 'single-quote/double-quote' key
    VK_OEM_7,
    /// Used for miscellaneous characters; it can vary by keyboard.
    VK_OEM_8,
    Reserved0xE0,
    OEM_Specific0xE1,
    /// Either the angle bracket key or the backslash key on the RT 102-key keyboard
    VK_OEM_102,
    OEM_Specific0xE3,
    OEM_Specific0xE4,
    /// IME PROCESS key
    VK_PROCESSKEY,
    OEM_Specific0xE6,
    /// Used to pass Unicode characters as if they were keystrokes. The VK_PACKET key is the low word of a 32-bit Virtual Key value used for non-keyboard input methods. For more information, see Remark in KEYBDINPUT, SendInput, WM_KEYDOWN, and WM_KEYUP
    VK_PACKET,
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
    /// Attn key
    VK_ATTN,
    /// CrSel key
    VK_CRSEL,
    VK_EXSEL,// ExSel key
    /// Erase EOF key
    VK_EREOF,
    /// Play key
    VK_PLAY,
    /// Zoom key
    VK_ZOOM,
    /// Reserved
    VK_NONAME,
    /// PA1 key
    VK_PA1,
    /// Clear key
    VK_OEM_CLEAR,
}