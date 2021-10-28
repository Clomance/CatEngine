use winapi::{
    um::{
        winuser::{
            // window messages
            WM_NULL,
            WM_CREATE,
            WM_DESTROY,
            WM_MOVE,
            WM_SIZE,
            WM_ACTIVATE,
            WM_SETFOCUS,
            WM_KILLFOCUS,
            WM_ENABLE,
            WM_SETREDRAW,
            WM_SETTEXT,
            WM_GETTEXT,
            WM_GETTEXTLENGTH,
            WM_PAINT,
            WM_CLOSE,
            WM_QUERYENDSESSION,
            WM_QUERYOPEN,
            WM_ENDSESSION,
            WM_QUIT,
            WM_ERASEBKGND,
            WM_SYSCOLORCHANGE,
            WM_SHOWWINDOW,
            WM_WININICHANGE,
            WM_SETTINGCHANGE,
            WM_DEVMODECHANGE,
            WM_ACTIVATEAPP,
            WM_FONTCHANGE,
            WM_TIMECHANGE,
            WM_CANCELMODE,
            WM_SETCURSOR,
            WM_MOUSEACTIVATE,
            WM_CHILDACTIVATE,
            WM_QUEUESYNC,
            WM_GETMINMAXINFO,
            WM_PAINTICON,
            WM_ICONERASEBKGND,
            WM_NEXTDLGCTL,
            WM_SPOOLERSTATUS,
            WM_DRAWITEM,
            WM_MEASUREITEM,
            WM_DELETEITEM,
            WM_VKEYTOITEM,
            WM_CHARTOITEM,
            WM_SETFONT,
            WM_GETFONT,
            WM_SETHOTKEY,
            WM_GETHOTKEY,
            WM_QUERYDRAGICON,
            WM_COMPAREITEM,
            WM_GETOBJECT,
            WM_COMPACTING,
            WM_COMMNOTIFY,
            WM_WINDOWPOSCHANGING,
            WM_WINDOWPOSCHANGED,
            WM_POWER,
            WM_COPYDATA,
            WM_CANCELJOURNAL,
            WM_NOTIFY,
            WM_INPUTLANGCHANGEREQUEST,
            WM_INPUTLANGCHANGE,
            WM_TCARD,
            WM_HELP,
            WM_USERCHANGED,
            WM_NOTIFYFORMAT,
            WM_CONTEXTMENU,
            WM_STYLECHANGING,
            WM_STYLECHANGED,
            WM_DISPLAYCHANGE,
            WM_GETICON,
            WM_SETICON,
            WM_NCCREATE,
            WM_NCDESTROY,
            WM_NCCALCSIZE,
            WM_NCHITTEST,
            WM_NCPAINT,
            WM_NCACTIVATE,
            WM_GETDLGCODE,
            WM_SYNCPAINT,
            WM_NCMOUSEMOVE,
            WM_NCLBUTTONDOWN,
            WM_NCLBUTTONUP,
            WM_NCLBUTTONDBLCLK,
            WM_NCRBUTTONDOWN,
            WM_NCRBUTTONUP,
            WM_NCRBUTTONDBLCLK,
            WM_NCMBUTTONDOWN,
            WM_NCMBUTTONUP,
            WM_NCMBUTTONDBLCLK,
            WM_NCXBUTTONDOWN,
            WM_NCXBUTTONUP,
            WM_NCXBUTTONDBLCLK,
            WM_INPUT_DEVICE_CHANGE,
            WM_INPUT,
            WM_KEYFIRST,
            WM_KEYDOWN,
            WM_KEYUP,
            WM_CHAR,
            WM_DEADCHAR,
            WM_SYSKEYDOWN,
            WM_SYSKEYUP,
            WM_SYSCHAR,
            WM_SYSDEADCHAR,
            WM_UNICHAR,
            WM_KEYLAST,
            WM_IME_STARTCOMPOSITION,
            WM_IME_ENDCOMPOSITION,
            WM_IME_COMPOSITION,
            WM_IME_KEYLAST,
            WM_INITDIALOG,
            WM_COMMAND,
            WM_SYSCOMMAND,
            WM_TIMER,
            WM_HSCROLL,
            WM_VSCROLL,
            WM_INITMENU,
            WM_INITMENUPOPUP,
            WM_GESTURE,
            WM_GESTURENOTIFY,
            WM_MENUSELECT,
            WM_MENUCHAR,
            WM_ENTERIDLE,
            WM_MENURBUTTONUP,
            WM_MENUDRAG,
            WM_MENUGETOBJECT,
            WM_UNINITMENUPOPUP,
            WM_MENUCOMMAND,
            WM_CHANGEUISTATE,
            WM_UPDATEUISTATE,
            WM_QUERYUISTATE,
            WM_CTLCOLORMSGBOX,
            WM_CTLCOLOREDIT,
            WM_CTLCOLORLISTBOX,
            WM_CTLCOLORBTN,
            WM_CTLCOLORDLG,
            WM_CTLCOLORSCROLLBAR,
            WM_CTLCOLORSTATIC,
            WM_MOUSEFIRST,
            WM_MOUSEMOVE,
            WM_LBUTTONDOWN,
            WM_LBUTTONUP,
            WM_LBUTTONDBLCLK,
            WM_RBUTTONDOWN,
            WM_RBUTTONUP,
            WM_RBUTTONDBLCLK,
            WM_MBUTTONDOWN,
            WM_MBUTTONUP,
            WM_MBUTTONDBLCLK,
            WM_MOUSEWHEEL,
            WM_XBUTTONDOWN,
            WM_XBUTTONUP,
            WM_XBUTTONDBLCLK,
            WM_MOUSEHWHEEL,
            WM_MOUSELAST,
            WM_PARENTNOTIFY,
            WM_ENTERMENULOOP,
            WM_EXITMENULOOP,
            WM_NEXTMENU,
            WM_SIZING,
            WM_CAPTURECHANGED,
            WM_MOVING,
            WM_POWERBROADCAST,
            WM_DEVICECHANGE,
            WM_MDICREATE,
            WM_MDIDESTROY,
            WM_MDIACTIVATE,
            WM_MDIRESTORE,
            WM_MDINEXT,
            WM_MDIMAXIMIZE,
            WM_MDITILE,
            WM_MDICASCADE,
            WM_MDIICONARRANGE,
            WM_MDIGETACTIVE,
            WM_MDISETMENU,
            WM_ENTERSIZEMOVE,
            WM_EXITSIZEMOVE,
            WM_DROPFILES,
            WM_MDIREFRESHMENU,
            WM_POINTERDEVICECHANGE,
            WM_POINTERDEVICEINRANGE,
            WM_POINTERDEVICEOUTOFRANGE,
            WM_TOUCH,
            WM_NCPOINTERUPDATE,
            WM_NCPOINTERDOWN,
            WM_NCPOINTERUP,
            WM_POINTERUPDATE,
            WM_POINTERDOWN,
            WM_POINTERUP,
            WM_POINTERENTER,
            WM_POINTERLEAVE,
            WM_POINTERACTIVATE,
            WM_POINTERCAPTURECHANGED,
            WM_TOUCHHITTESTING,
            WM_POINTERWHEEL,
            WM_POINTERHWHEEL,
            WM_IME_SETCONTEXT,
            WM_IME_NOTIFY,
            WM_IME_CONTROL,
            WM_IME_COMPOSITIONFULL,
            WM_IME_SELECT,
            WM_IME_CHAR,
            WM_IME_REQUEST,
            WM_IME_KEYDOWN,
            WM_IME_KEYUP,
            WM_MOUSEHOVER,
            WM_MOUSELEAVE,
            WM_NCMOUSEHOVER,
            WM_NCMOUSELEAVE,
            WM_WTSSESSION_CHANGE,
            WM_TABLET_FIRST,
            WM_TABLET_LAST,
            WM_CUT,
            WM_COPY,
            WM_PASTE,
            WM_CLEAR,
            WM_UNDO,
            WM_RENDERFORMAT,
            WM_RENDERALLFORMATS,
            WM_DESTROYCLIPBOARD,
            WM_DRAWCLIPBOARD,
            WM_PAINTCLIPBOARD,
            WM_VSCROLLCLIPBOARD,
            WM_SIZECLIPBOARD,
            WM_ASKCBFORMATNAME,
            WM_CHANGECBCHAIN,
            WM_HSCROLLCLIPBOARD,
            WM_QUERYNEWPALETTE,
            WM_PALETTEISCHANGING,
            WM_PALETTECHANGED,
            WM_HOTKEY,
            WM_PRINT,
            WM_PRINTCLIENT,
            WM_APPCOMMAND,
            WM_THEMECHANGED,
            WM_CLIPBOARDUPDATE,
            WM_DWMCOMPOSITIONCHANGED,
            WM_DWMNCRENDERINGCHANGED,
            WM_DWMCOLORIZATIONCOLORCHANGED,
            WM_DWMWINDOWMAXIMIZEDCHANGE,
            WM_DWMSENDICONICTHUMBNAIL,
            WM_DWMSENDICONICLIVEPREVIEWBITMAP,
            WM_GETTITLEBARINFOEX,
            WM_HANDHELDFIRST,
            WM_HANDHELDLAST,
            WM_AFXFIRST,
            WM_AFXLAST,
            WM_PENWINFIRST,
            WM_PENWINLAST,
            WM_APP,
            WM_USER,

            // window resize requests
            SIZE_MAXHIDE,
            SIZE_MAXIMIZED,
            SIZE_MAXSHOW,
            SIZE_MINIMIZED,
            SIZE_RESTORED,
        },
    }
};

/// A window receives this messages through it's `WindowProc` function.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
#[repr(u32)]
pub enum WindowMessage{
    /// Performs no operation.
    /// 
    /// An application sends this message if it wants to post a message that the recipient window will ignore.
    /// 
    /// For example, if an application has installed a WH_GETMESSAGE hook
    /// and wants to prevent a message from being processed,
    /// the GetMsgProc callback function can change the message number to WM_NULL so the recipient will ignore it.
    /// 
    /// As another example, an application can check
    /// if a window is responding to messages by sending the WM_NULL message with the SendMessageTimeout function.
    /// 
    /// An application returns zero if it processes this message.
    None=WM_NULL,

    /// Sent prior to the WM_CREATE message when a window is first created.
    /// 
    /// `l_param` - A pointer to a `CREATESTRUCT` structure
    /// that contains information about the window being created.
    /// 
    /// If an application processes this message, it should return TRUE to continue creation of the window.
    /// If the application returns FALSE, the CreateWindow or CreateWindowEx function will return a NULL handle.
    NonClientCreate=WM_NCCREATE,

    /// Sent when an application requests that a window be created
    /// by calling the `CreateWindowEx` or `CreateWindow` function.
    /// (The message is sent before the function returns.)
    /// The window procedure of the new window receives this message after the window is created,
    /// but before the window becomes visible.
    /// 
    /// `w_param` is not used.
    /// 
    /// `l_param` - A pointer to a `CREATESTRUCT` structure
    /// that contains information about the window being created.
    /// 
    /// If an application processes this message,
    /// it should return zero to continue creation of the window.
    /// If the application returns –1, the window is destroyed
    /// and the `CreateWindowEx` or `CreateWindow` function returns a `NULL` handle.
    Create=WM_CREATE,


    /// Sent as a signal that a window or an application should terminate.
    /// 
    /// If an application processes this message, it should return zero.
    /// 
    /// An application can prompt the user for confirmation, prior to destroying a window,
    /// by processing the WM_CLOSE message and calling the DestroyWindow function only if the user confirms the choice.
    /// 
    /// By default, the DefWindowProc function calls the `Window::destroy` function to destroy the window.
    Close=WM_CLOSE,

    /// Sent when a window is being destroyed.
    /// It is sent to the window procedure of the window being destroyed
    /// after the window is removed from the screen.
    /// 
    /// This message is sent first to the window being destroyed
    /// and then to the child windows (if any) as they are destroyed.
    /// During the processing of the message, it can be assumed that all child windows still exist.
    /// If the window being destroyed is part of the clipboard viewer chain (set by calling the SetClipboardViewer function),
    /// the window must remove itself from the chain by processing the `ChangeClipboardChain` function
    /// before returning from the WM_DESTROY message.
    /// 
    /// If an application processes this message, it should return zero.
    Destroy=WM_DESTROY,

    /// Is sent when the system
    /// or another application makes a request to paint a portion of an application's window.
    /// The message is sent when the `UpdateWindow` or `RedrawWindow` function is called,
    /// or by the `DispatchMessage` function
    /// when the application obtains a `WM_PAINT` message
    /// by using the `GetMessage` or `PeekMessage` function.
    /// 
    /// An application returns zero if it processes this message.
    /// 
    /// The WM_PAINT message is generated by the system and should not be sent by an application.
    /// To force a window to draw into a specific device context, use the WM_PRINT or WM_PRINTCLIENT message.
    /// Note that this requires the target window to support the WM_PRINTCLIENT message.
    /// Most common controls support the WM_PRINTCLIENT message.
    /// 
    /// The DefWindowProc function validates the update region.
    /// The function may also send the WM_NCPAINT message to the window procedure
    /// if the window frame must be painted and send the WM_ERASEBKGND message if the window background must be erased.
    /// 
    /// The system sends this message when there are no other messages in the application's message queue.
    /// DispatchMessage determines where to send the message; GetMessage determines which message to dispatch.
    /// GetMessage returns the WM_PAINT message when there are no other messages in the application's message queue,
    /// and DispatchMessage sends the message to the appropriate window procedure.
    /// 
    /// A window may receive internal paint messages as a result of calling RedrawWindow with the RDW_INTERNALPAINT flag set.
    /// In this case, the window may not have an update region.
    /// An application may call the GetUpdateRect function to determine whether the window has an update region.
    /// If GetUpdateRect returns zero, the application need not call the BeginPaint and EndPaint functions.
    /// 
    /// An application must check for any necessary internal painting by looking at its internal data structures for each WM_PAINT message,
    /// because a WM_PAINT message may have been caused by both a non-NULL update region and a call to RedrawWindow with the RDW_INTERNALPAINT flag set.
    /// 
    /// The system sends an internal WM_PAINT message only once.
    /// After an internal WM_PAINT message is returned from GetMessage or PeekMessage or is sent to a window by UpdateWindow,
    /// the system does not post or send further WM_PAINT messages
    /// until the window is invalidated or until RedrawWindow is called again with the RDW_INTERNALPAINT flag set.
    /// 
    /// For some common controls, the default WM_PAINT message processing checks the wParam parameter.
    /// If wParam is non-NULL, the control assumes that the value is an HDC and paints using that device context.
    Paint=WM_PAINT,

    /// Sent to a window if the mouse causes the cursor to move within a window and mouse input is not captured.
    /// 
    /// `w_param` - A handle to the window that contains the cursor.
    /// 
    /// `l_param` - The low-order word of lParam specifies the hit-test result for the cursor position.
    /// See the return values for WM_NCHITTEST for possible values.
    /// The high-order word of lParam specifies the mouse window message which triggered this event, such as WM_MOUSEMOVE.
    /// When the window enters menu mode, this value is zero.
    /// 
    /// If an application processes this message, it should return TRUE to halt further processing or FALSE to continue.
    /// 
    /// The `DefWindowProc` function passes the `WM_SETCURSOR` message to a parent window before processing.
    /// If the parent window returns TRUE, further processing is halted.
    /// Passing the message to a window's parent window gives the parent window control over the cursor's setting in a child window.
    /// The `DefWindowProc` function also uses this message to set the cursor to an arrow
    /// if it is not in the client area,
    /// or to the registered class cursor if it is in the client area.
    /// If the low-order word of the lParam parameter is HTERROR
    /// and the high-order word of lParam specifies that one of the mouse buttons is pressed,
    /// DefWindowProc calls the MessageBeep function.
    #[cfg(feature="set_cursor_event")]
    SetCursor=WM_SETCURSOR,

    /// Sent to a window after its size has changed.
    /// 
    /// `w_param` - The type of resizing requested.
    /// This parameter can be one of the `WindowMessageResize` values.
    /// 
    /// `l_param` - The low-order word specifies the new width of the client area.
    /// The high-order word specifies the new height of the client area.
    /// 
    /// If an application processes this message, it should return zero.
    Size=WM_SIZE,

    /// Sent after a window has been moved.
    /// 
    /// `l_param` - The x and y coordinates of the upper-left corner of the client area of the window.
    /// The low-order word contains the x-coordinate while the high-order word contains the y coordinate.
    /// 
    /// If an application processes this message, it should return zero.
    /// 
    /// The DefWindowProc function sends the WM_SIZE and WM_MOVE messages
    /// when it processes the WM_WINDOWPOSCHANGED message.
    /// The WM_SIZE and WM_MOVE messages are not sent
    /// if an application handles the WM_WINDOWPOSCHANGED message without calling DefWindowProc.
    Move=WM_MOVE,
}

/// The type of resizing requested.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
#[repr(usize)]
pub enum WindowResizeType{
    /// Message is sent to all pop-up windows
    /// when some other window is maximized.
    MaximizedHide=SIZE_MAXHIDE,

    /// The window has been maximized.
    Maximized=SIZE_MAXIMIZED,

    /// Message is sent to all pop-up windows
    /// when some other window has been restored to its former size.
    MaximizedShow=SIZE_MAXSHOW,

    /// The window has been minimized.
    Minimized=SIZE_MINIMIZED,

    /// The window has been resized,
    /// but neither the `WindowMessageResize::Minimized` nor `WindowMessageResize::Maximized` value applies.
    Restore=SIZE_RESTORED,
}