use super::{
    Ticks,
};

pub enum Event{
    Process(ProcessEvent),
    Window(WindowEvent),
}

#[derive(Debug)]
pub enum ProcessEvent{
    EventLoopStart,

    Update(Ticks),
    /// The current thread has made a request to close.
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

    KeyPress(VirtualKeyCode),
    KeyRelease(VirtualKeyCode),
    CharacterInput(char),
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

#[derive(Debug,Clone,Copy)]
#[repr(C)]
pub enum VirtualKeyCode{
    Null,
    /// Left mouse button
    LButton,
    /// Right mouse button
    RButton,
    /// Control-break processing
    Cancel,
    /// Middle mouse button (three-button mouse)
    MButton,
    /// X1 mouse button
    XButton1,
    /// X2 mouse button
    XButton2,
    Undefined0x07,
    /// BACKSPACE key
    Back,
    /// TAB key
    Tab,
    Reserved0x0A,
    Reserved0x0B,
    /// CLEAR key
    Clear,
    /// ENTER key
    RETURN,
    Undefined0x0E,
    Undefined0x0F,
    /// SHIFT key
    SHIFT,
    /// CTRL key
    CONTROL,
    /// ALT key
    MENU,
    /// PAUSE key
    PAUSE,
    /// CAPS LOCK key,
    CAPITAL,
    /// HANGUEL, HANGUL - IME Kana mode, IME Hanguel mode, IME Hangul mode
    KANA,
    /// IME On
    IME_ON,
    /// IME Junja mode
    JUNJA,
    /// IME final mode
    FINAL,
    /// KANJI -  IME Hanja mode, IME Kanji mode
    HANJA,
    /// IME Off
    IME_OFF,
    /// ESC key
    ESCAPE,
    /// IME convert
    CONVERT,
    /// IME nonconvert
    NONCONVERT,
    /// IME accept
    ACCEPT,
    /// IME mode change request
    MODECHANGE,
    /// SPACEBAR
    SPACE,
    /// PAGE UP key
    PRIOR,
    /// PAGE DOWN key
    NEXT,
    /// END key
    END,
    /// HOME key
    HOME,
    /// LEFT ARROW key
    LEFT,
    /// UP ARROW key
    UP,
    /// RIGHT ARROW key
    RIGHT,
    /// DOWN ARROW key
    DOWN,
    /// SELECT key
    SELECT,
    /// PRINT key
    PRINT,
    /// EXECUTE key
    EXECUTE,
    /// PRINT SCREEN key
    SNAPSHOT,
    /// INS key
    INSERT,
    /// DEL key
    DELETE,
    /// HELP key
    HELP,
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
    LWin,
    /// Right Windows key (Natural keyboard)
    RWin,
    /// Applications key (Natural keyboard)
    Apps,
    Reserved0x5E,
    /// Computer Sleep key
    Sleep,
    /// Numeric keypad 0 key
    NUMPAD0,
    /// Numeric keypad 1 key
    NUMPAD1,
    /// Numeric keypad 2 key
    NUMPAD2,
    /// Numeric keypad 3 key
    NUMPAD3,
    /// Numeric keypad 4 key
    NUMPAD4,
    /// Numeric keypad 5 key
    NUMPAD5,
    /// Numeric keypad 6 key
    NUMPAD6,
    /// Numeric keypad 7 key
    NUMPAD7,
    /// Numeric keypad 8 key
    NUMPAD8,
    /// Numeric keypad 9 key
    NUMPAD9,
    /// Multiply key
    MULTIPLY,
    /// Add key
    ADD,
    /// Separator key
    SEPARATOR,
    /// Subtract key
    SUBTRACT,
    /// Decimal key
    DECIMAL,
    /// Divide key
    DIVIDE,
    /// F1 key
    F1,
    /// F2 key
    F2,
    /// /// F3 key
    F3,
    /// F4 key
    F4,
    /// F5 key
    F5,
    /// F6 key
    F6,
    /// F7 key
    F7,
    /// F8 key
    F8,
    /// F9 key
    F9,
    /// F10 key
    F10,
    /// F11 key
    F11,
    /// F12 key
    F12,
    /// F13 key
    F13,
    /// F14 key
    F14,
    /// F15 key
    F15,
    /// F16 key
    F16,
    /// F17 key
    F17,
    /// F18 key
    F18,
    /// F19 key
    F19,
    /// F20 key
    F20,
    /// F21 key
    F21,
    /// F22 key
    F22,
    /// F23 key
    F23,
    /// F24 key
    F24,
    Unassigned0x88,
    Unassigned0x89,
    Unassigned0x8A,
    Unassigned0x8B,
    Unassigned0x8C,
    Unassigned0x8D,
    Unassigned0x8E,
    Unassigned0x8F,
    /// NUM LOCK key
    NUMLOCK,
    /// SCROLL LOCK key
    SCROLL,
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
    LSHIFT,
    /// Right SHIFT key
    RSHIFT,
    /// Left CONTROL key
    LCONTROL,
    /// Right CONTROL key
    RCONTROL,
    /// Left MENU key
    LMENU,
    /// Right MENU key
    RMENU,
    /// Browser Back key
    BROWSER_BACK,
    /// Browser Forward key
    BROWSER_FORWARD,
    /// Browser Refresh key
    BROWSER_REFRESH,
    /// Browser Stop key
    BROWSER_STOP,
    /// Browser Search key
    BROWSER_SEARCH,
    /// Browser Favorites key
    BROWSER_FAVORITES,
    /// Browser Start and Home key
    BROWSER_HOME,
    /// Volume Mute key
    VOLUME_MUTE,
    /// Volume Down key
    VOLUME_DOWN,
    /// Volume Up key
    VOLUME_UP,
    /// Next Track key
    MEDIA_NEXT_TRACK,
    /// Previous Track key
    MEDIA_PREV_TRACK,
    /// Stop Media key
    MEDIA_STOP,
    /// Play/Pause Media key
    MEDIA_PLAY_PAUSE,
    /// Start Mail key
    LAUNCH_MAIL,
    /// Select Media key
    LAUNCH_MEDIA_SELECT,
    /// Start Application 1 key
    LAUNCH_APP1,
    /// Start Application 2 key
    LAUNCH_APP2,
    ReservedB8,
    ReservedB9,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the ';:' key
    OEM_1,
    /// For any country/region, the '+' key
    OEM_PLUS,
    /// For any country/region, the ',' key
    OEM_COMMA,
    /// For any country/region, the '-' key
    OEM_MINUS,
    /// For any country/region, the '.' key
    OEM_PERIOD,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the '/?' key
    OEM_2,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the '`~' key
    OEM_3,
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
    OEM_4,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the '\|' key
    OEM_5,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the ']}' key
    OEM_6,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the 'single-quote/double-quote' key
    OEM_7,
    /// Used for miscellaneous characters; it can vary by keyboard.
    OEM_8,
    Reserved0xE0,
    OEM_Specific0xE1,
    /// Either the angle bracket key or the backslash key on the RT 102-key keyboard
    OEM_102,
    OEM_Specific0xE3,
    OEM_Specific0xE4,
    /// IME PROCESS key
    PROCESSKEY,
    OEM_Specific0xE6,
    /// Used to pass Unicode characters as if they were keystrokes. The PACKET key is the low word of a 32-bit Virtual Key value used for non-keyboard input methods. For more information, see Remark in KEYBDINPUT, SendInput, WM_KEYDOWN, and WM_KEYUP
    PACKET,
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
    ATTN,
    /// CrSel key
    CRSEL,
    EXSEL,// ExSel key
    /// Erase EOF key
    EREOF,
    /// Play key
    PLAY,
    /// Zoom key
    ZOOM,
    /// Reserved
    NONAME,
    /// PA1 key
    PA1,
    /// Clear key
    OEM_CLEAR,
}