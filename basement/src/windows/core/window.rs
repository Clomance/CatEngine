use core::mem::transmute;

use winapi::{
    shared::{
        ntdef::LPCWSTR,
        windef::{
            HWND,
            HDC,
            RECT,
            POINT,
            HMENU,
        },
        minwindef::HINSTANCE,
    },

    um::{
        winuser::{
            // ShowWindow,
            // SetFocus,
            // SetForegroundWindow,
            // SetCapture,
            CreateWindowExW,
            DestroyWindow,
            SendMessageW,
            GetDC,
            GetWindowRect,
            GetClientRect,
            RedrawWindow,
            SetWindowPos,
            SetWindowLongPtrW,
            GetWindowLongPtrW,
            GetCursorPos,
            SetCursorPos,
            ShowCursor,
            ClientToScreen,
            ScreenToClient,
            // window styles
            WS_BORDER,
            WS_CAPTION,
            WS_CHILD,
            WS_CHILDWINDOW,
            WS_CLIPCHILDREN,
            WS_CLIPSIBLINGS,
            WS_DISABLED,
            WS_DLGFRAME,
            WS_GROUP,
            WS_HSCROLL,
            WS_ICONIC,
            WS_MAXIMIZE,
            WS_MAXIMIZEBOX,
            WS_MINIMIZE,
            WS_MINIMIZEBOX,
            WS_OVERLAPPED,
            WS_POPUP,
            WS_SIZEBOX,
            WS_SYSMENU,
            WS_TABSTOP,
            WS_THICKFRAME,
            WS_VISIBLE,
            WS_VSCROLL,
            // extended window styles
            WS_EX_ACCEPTFILES,
            WS_EX_APPWINDOW,
            WS_EX_CLIENTEDGE,
            WS_EX_COMPOSITED,
            WS_EX_CONTEXTHELP,
            WS_EX_CONTROLPARENT,
            WS_EX_DLGMODALFRAME,
            WS_EX_LAYERED,
            WS_EX_LAYOUTRTL,
            WS_EX_LEFT,
            WS_EX_LEFTSCROLLBAR,
            WS_EX_LTRREADING,
            WS_EX_MDICHILD,
            WS_EX_NOACTIVATE,
            WS_EX_NOINHERITLAYOUT,
            WS_EX_NOPARENTNOTIFY,
            WS_EX_NOREDIRECTIONBITMAP,
            WS_EX_RIGHT,
            WS_EX_RIGHTSCROLLBAR,
            WS_EX_RTLREADING,
            WS_EX_STATICEDGE,
            WS_EX_TOOLWINDOW,
            WS_EX_TOPMOST,
            WS_EX_TRANSPARENT,
            WS_EX_WINDOWEDGE,
            // other
            GWL_EXSTYLE,
            GWL_STYLE,
            GWLP_USERDATA,
            GWLP_WNDPROC,
            CW_USEDEFAULT,
            RDW_INTERNALPAINT,
            RDW_INVALIDATE,
            SWP_SHOWWINDOW,
            SWP_NOREPOSITION,
            SWP_NOSIZE,
            SWP_NOMOVE,
            SWP_FRAMECHANGED,
        },
    }
};

pub struct Window;

impl Window{
    pub const fn new()->Window{
        Self
    }
}

impl Window{
    #[inline(always)]
    pub unsafe fn create<P>(
        &self,
        class_name:*const u16,
        window_name:*const u16,
        style:u32,
        extended_style:u32,
        [x,y,width,height]:[i32;4],
        parent_window:HWND,
        menu:HMENU,
        instance:HINSTANCE,
        create_parameters:*mut P,
    )->HWND{
        CreateWindowExW(
            extended_style,
            class_name,
            window_name,
            style,
            x,y,width,height,
            parent_window,
            menu,
            instance,
            create_parameters as *mut _,
        )
    }

    /// Destroys the specified window.
    /// 
    /// The function sends `WM_DESTROY` and `WM_NCDESTROY` messages
    /// to the window to deactivate it and remove the keyboard focus from it.
    /// The function also destroys the window's menu,
    /// flushes the thread message queue,
    /// destroys timers, removes clipboard ownership,
    /// and breaks the clipboard viewer chain (if the window is at the top of the viewer chain).
    /// 
    /// If the specified window is a parent or owner window,
    /// `Window::destroy` automatically destroys the associated child or owned windows
    /// when it destroys the parent or owner window.
    /// The function first destroys child or owned windows,
    /// and then it destroys the parent or owner window.
    /// 
    /// `Window::destroy` also destroys modeless dialog boxes created by the `CreateDialog` function.
    /// 
    /// If the function succeeds, the return value is nonzero.
    /// If the function fails, the return value is zero.
    /// To get extended error information, call `GetLastError`.
    /// 
    /// A thread cannot use `Window::destroy` to destroy a window created by a different thread.
    /// 
    /// If the window being destroyed is a child window that does not have the `WS_EX_NOPARENTNOTIFY` style, a `WM_PARENTNOTIFY` message is sent to the parent.
    #[inline(always)]
    pub unsafe fn destroy(&self,window:HWND)->bool{
        DestroyWindow(window)!=0
    }

    /// Retrieves the dimensions of the bounding rectangle of the specified window.
    /// The dimensions are given in screen coordinates that are relative to the upper-left corner of the screen.
    /// 
    /// If the function succeeds, the return value is nonzero.
    /// If the function fails, the return value is zero.
    /// To get extended error information, call `GetLastError`.
    /// 
    /// In conformance with conventions for the `RECT` structure,
    /// the bottom-right coordinates of the returned rectangle are exclusive.
    /// In other words, the pixel at (right, bottom) lies immediately outside the rectangle.
    /// 
    /// `Window::get_window_rectangle` is virtualized for DPI.
    /// 
    /// In Windows Vista and later, the Window Rect now includes the area occupied by the drop shadow.
    /// 
    /// Calling `Window::get_window_rectangle` will have different behavior depending on
    /// whether the window has ever been shown or not.
    /// If the window has not been shown before,
    /// `Window::get_window_rectangle` will not include the area of the drop shadow.
    /// 
    /// To get the window bounds excluding the drop shadow,
    /// use DwmGetWindowAttribute, specifying `DWMWA_EXTENDED_FRAME_BOUNDS`.
    /// Note that unlike the Window Rect,
    /// the DWM Extended Frame Bounds are not adjusted for DPI.
    /// Getting the extended frame bounds can only be done after the window has been shown at least once.
    #[inline(always)]
    pub unsafe fn get_window_rectangle(&self,window:HWND,rectangle:&mut [i32;4])->bool{
        GetWindowRect(window,transmute(rectangle))!=0
    }

    /// Retrieves the coordinates of a window's client area.
    /// The client coordinates specify the upper-left and lower-right corners of the client area.
    /// Because client coordinates are relative to the upper-left corner of a window's client area,
    /// the coordinates of the upper-left corner are (0,0).
    /// 
    /// If the function succeeds, the return value is nonzero.
    /// 
    /// If the function fails, the return value is zero.
    /// To get extended error information, call `GetLastError`.
    /// 
    /// In conformance with conventions for the `RECT` structure,
    /// the bottom-right coordinates of the returned rectangle are exclusive.
    /// In other words, the pixel at (right, bottom) lies immediately outside the rectangle.
    #[inline(always)]
    pub unsafe fn get_client_rectangle(&self,window:HWND,rectangle:&mut [i32;4])->bool{
        GetClientRect(window,transmute(rectangle))!=0
    }

    /// Changes the size, position, and Z order of a child, pop-up, or top-level window.
    /// These windows are ordered according to their appearance on the screen.
    /// The topmost window receives the highest rank and is the first window in the Z order.
    /// 
    /// If the function succeeds, the return value is nonzero.
    /// If the function fails, the return value is zero.
    /// To get extended error information, call `GetLastError`.
    /// 
    /// If you have changed certain window data using SetWindowLong,
    /// you must call SetWindowPos for the changes to take effect.
    /// Use the following combination for uFlags: `SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED`.
    /// 
    /// A window can be made a topmost window either by setting
    /// the hWndInsertAfter parameter to HWND_TOPMOST and ensuring that the `SWP_NOZORDER` flag is not set,
    /// or by setting a window's position in the Z order
    /// so that it is above any existing topmost windows.
    /// When a non-topmost window is made topmost,
    /// its owned windows are also made topmost.
    /// Its owners, however, are not changed.
    /// 
    /// If neither the SWP_NOACTIVATE nor SWP_NOZORDER flag is specified
    /// (that is, when the application requests that a window be simultaneously activated and its position in the Z order changed),
    /// the value specified in hWndInsertAfter is used only in the following circumstances.
    /// 
    /// Neither the HWND_TOPMOST nor HWND_NOTOPMOST flag is specified in hWndInsertAfter.
    /// The window identified by hWnd is not the active window.
    /// An application cannot activate an inactive window without also bringing it to the top of the Z order. Applications can change an activated window's position in the Z order without restrictions, or it can activate a window and then move it to the top of the topmost or non-topmost windows.
    /// If a topmost window is repositioned to the bottom (HWND_BOTTOM) of the Z order or after any non-topmost window, it is no longer topmost.
    /// When a topmost window is made non-topmost, its owners and its owned windows are also made non-topmost windows.
    /// 
    /// A non-topmost window can own a topmost window, but the reverse cannot occur.
    /// Any window (for example, a dialog box) owned by a topmost window is itself made a topmost window,
    /// to ensure that all owned windows stay above their owner.
    /// 
    /// If an application is not in the foreground, and should be in the foreground,
    /// it must call the SetForegroundWindow function.
    /// 
    /// To use SetWindowPos to bring a window to the top,
    /// the process that owns the window must have SetForegroundWindow permission.
    #[inline(always)]
    pub unsafe fn set_window_position(&self,window:HWND,insert_after:HWND,[x,y,width,height]:[i32;4],flag:u32)->bool{
        SetWindowPos(window,insert_after,x,y,width,height,flag)!=0
    }

    /// Changes an attribute of the specified window.
    /// The function also sets a value at the specified offset in the extra window memory.
    #[inline(always)]
    pub unsafe fn set_window_long_ptr(&self,window:HWND,index:i32,ptr:isize)->isize{
        SetWindowLongPtrW(window,index,ptr)
    }
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum WindowStyles{
    /// The window has a thin-line border.
    /// 
    /// 0x00800000
    Border=WS_BORDER,

    /// The window has a title bar (includes the WS_BORDER style).
    /// 
    /// 0x00C00000
    Caption=WS_CAPTION,

    /// The window is a child window. A window with this style cannot have a menu bar.
    /// This style cannot be used with the WS_POPUP style.
    /// 
    /// 0x40000000
    ChildWindow=WS_CHILDWINDOW,

    /// Excludes the area occupied by child windows when drawing occurs within the parent window.
    /// This style is used when creating the parent window.
    /// 
    /// 0x02000000
    ClipChildren=WS_CLIPCHILDREN,

    /// Clips child windows relative to each other;
    /// that is, when a particular child window receives a WM_PAINT message,
    /// the WS_CLIPSIBLINGS style clips all other overlapping child windows
    /// out of the region of the child window to be updated.
    /// If WS_CLIPSIBLINGS is not specified and child windows overlap,
    /// it is possible, when drawing within the client area of a child window,
    /// to draw within the client area of a neighboring child window.
    /// 
    /// 0x04000000
    ClipSiblings=WS_CLIPSIBLINGS,

    /// The window has a border of a style typically used with dialog boxes.
    /// A window with this style cannot have a title bar.
    /// 
    /// 0x00400000
    DialogueFrame=WS_DLGFRAME,

    /// The window has a horizontal scroll bar.
    /// 
    /// 0x00100000
    HorizontalScroll=WS_HSCROLL,

    /// The window has a vertical scroll bar.
    /// 
    /// 0x00200000
    VerticalScroll=WS_VSCROLL,

    /// The window is initially maximized.
    /// 
    /// 0x01000000
    Maximized=WS_MAXIMIZE,

    /// The window is a control that can receive the keyboard focus when the user presses the TAB key.
    /// Pressing the TAB key changes the keyboard focus to the next control with the WS_TABSTOP style.
    /// You can turn this style on and off to change dialog box navigation.
    /// To change this style after a window has been created, use the SetWindowLong function.
    /// For user-created windows and modeless dialogs to work with tab stops, alter the message loop to call the IsDialogMessage function.
    /// 
    /// The window has a maximize button. Cannot be combined with the WS_EX_CONTEXTHELP style. The WS_SYSMENU style must also be specified.
    /// 
    /// 0x00010000
    MaximizeBox=WS_MAXIMIZEBOX,

    /// The window is initially minimized.
    /// 
    /// 0x20000000
    Minimized=WS_MINIMIZE,

    /// The window is the first control of a group of controls.
    /// The group consists of this first control and all controls defined after it, up to the next control with the WS_GROUP style.
    /// The first control in each group usually has the WS_TABSTOP style so that the user can move from group to group.
    /// The user can subsequently change the keyboard focus from one control in the group to the next control in the group by using the direction keys.
    /// You can turn this style on and off to change dialog box navigation.
    /// To change this style after a window has been created, use the SetWindowLong function.
    /// 
    /// The window has a minimize button.
    /// Cannot be combined with the WS_EX_CONTEXTHELP style.
    /// The WS_SYSMENU style must also be specified.
    /// 
    /// 0x00020000
    MinimizeBox=WS_MINIMIZEBOX,

    /// The window is an overlapped window.
    /// An overlapped window has a title bar and a border.
    /// 
    /// 0x00000000
    OverLapped=WS_OVERLAPPED,

    /// The window is a pop-up window.
    /// This style cannot be used with the WS_CHILD style.
    /// 
    /// 0x80000000
    PopUp=WS_POPUP,

    /// The window has a sizing border.
    /// 
    /// 0x00040000
    SizeBox=WS_SIZEBOX,

    /// The window has a window menu on its title bar.
    /// The WS_CAPTION style must also be specified.
    /// 
    /// 0x00080000
    SystemMenu=WS_SYSMENU,

    /// The window is initially visible.
    /// This style can be turned on and off by using the ShowWindow or SetWindowPos function.
    /// 
    /// 0x10000000
    Visible=WS_VISIBLE,

    /// The window is initially disabled.
    /// A disabled window cannot receive input from the user.
    /// To change this after a window has been created, use the EnableWindow function.
    /// 
    /// 0x08000000
    Disabled=WS_DISABLED,
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum ExtendedWindowStyles{
    /// The window accepts drag-drop files.
    /// 
    /// 0x00000010
    AcceptFiles=WS_EX_ACCEPTFILES,

    /// Forces a top-level window onto the taskbar when the window is visible.
    /// 
    /// 0x00040000
    AppWwindow=WS_EX_APPWINDOW,

    /// The window has a border with a sunken edge.
    /// 
    /// 0x00000200
    ClientEdge=WS_EX_CLIENTEDGE,

    /// Paints all descendants of a window in bottom-to-top painting order using double-buffering.
    /// Bottom-to-top painting order allows a descendent window to have translucency (alpha) and transparency (colour-key) effects,
    /// but only if the descendent window also has the WS_EX_TRANSPARENT bit set.
    /// Double-buffering allows the window and its descendents to be painted without flicker.
    /// This cannot be used if the window has a class style of either CS_OWNDC or CS_CLASSDC.
    /// Windows 2000: This style is not supported.
    /// 
    /// 0x02000000
    Composited=WS_EX_COMPOSITED,

    /// The title bar of the window includes a question mark.
    /// When the user clicks the question mark, the cursor changes to a question mark with a pointer.
    /// If the user then clicks a child window, the child receives a WM_HELP message.
    /// The child window should pass the message to the parent window procedure,
    /// which should call the WinHelp function using the HELP_WM_HELP command.
    /// The Help application displays a pop-up window that typically contains help for the child window.
    /// WS_EX_CONTEXTHELP cannot be used with the WS_MAXIMIZEBOX or WS_MINIMIZEBOX styles.
    /// 
    /// 0x00000400
    ContextHelp=WS_EX_CONTEXTHELP,

    /// The window itself contains child windows that should take part in dialog box navigation.
    /// If this style is specified, the dialog manager recurses into children of this window
    /// when performing navigation operations such as handling the TAB key,
    /// an arrow key, or a keyboard mnemonic.
    /// 
    /// 0x00010000
    ControlParent=WS_EX_CONTROLPARENT,

    /// The window has a double border;
    /// the window can, optionally,
    /// be created with a title bar by specifying the WS_CAPTION style in the dwStyle parameter.
    /// 
    /// 0x00000001
    DLGMODALFRAME=WS_EX_DLGMODALFRAME,

    /// The window is a layered window.
    /// This style cannot be used if the window has a class style of either CS_OWNDC or CS_CLASSDC.
    /// Windows 8: The WS_EX_LAYERED style is supported for top-level windows and child windows.
    /// Previous Windows versions support WS_EX_LAYERED only for top-level windows.
    /// 
    /// 0x00080000
    Layered=WS_EX_LAYERED,

    /// If the shell language is Hebrew, Arabic, or another language that supports reading order alignment,
    /// the horizontal origin of the window is on the right edge.
    /// Increasing horizontal values advance to the left.
    /// 
    /// 0x00400000
    LayoutRTL=WS_EX_LAYOUTRTL,

    /// The window has generic left-aligned properties.
    /// The window text is displayed using left-to-right reading-order properties.
    /// 
    /// The vertical scroll bar (if present) is to the right of the client area.
    /// 
    /// This is the default.
    /// 
    /// 0x00000000
    Default=WS_EX_LEFT,

    /// If the shell language is Hebrew, Arabic, or another language that supports reading order alignment,
    /// the vertical scroll bar (if present) is to the left of the client area.
    /// For other languages, the style is ignored.
    /// 
    /// 0x00004000
    LeftScrollBar=WS_EX_LEFTSCROLLBAR,

    /// The window is a MDI child window.
    /// 
    /// 0x00000040
    MDIChild=WS_EX_MDICHILD,

    /// A top-level window created with this style does not become the foreground window when the user clicks it.
    /// The system does not bring this window to the foreground when the user minimizes or closes the foreground window.
    /// The window should not be activated through programmatic access or via keyboard navigation by accessible technology, such as Narrator.
    /// To activate the window, use the SetActiveWindow or SetForegroundWindow function.
    /// The window does not appear on the taskbar by default.
    /// To force the window to appear on the taskbar, use the WS_EX_APPWINDOW style.
    /// 
    /// 0x08000000
    NoActive=WS_EX_NOACTIVATE,

    /// The window does not pass its window layout to its child windows.
    /// 
    /// 0x00100000
    NoInheritLayout=WS_EX_NOINHERITLAYOUT,

    /// The child window created with this style does not send the WM_PARENTNOTIFY message
    /// to its parent window when it is created or destroyed.
    /// 
    /// 0x00000004
    NoParentNotify=WS_EX_NOPARENTNOTIFY,

    /// The window does not render to a redirection surface.
    /// This is for windows that do not have visible content or that use mechanisms other than surfaces to provide their visual.
    /// 
    /// 0x00200000
    NoDirectionBitmap=WS_EX_NOREDIRECTIONBITMAP,

    /// The window has generic "right-aligned" properties.
    /// This depends on the window class.
    /// This style has an effect only if the shell language is Hebrew, Arabic, or another language that supports reading-order alignment; otherwise, the style is ignored.
    /// Using the WS_EX_RIGHT style for static or edit controls has the same effect as using the SS_RIGHT or ES_RIGHT style, respectively.
    /// Using this style with button controls has the same effect as using BS_RIGHT and BS_RIGHTBUTTON styles.
    /// 
    /// 0x00001000
    Right=WS_EX_RIGHT,

    /// If the shell language is Hebrew, Arabic, or another language that supports reading-order alignment,
    /// the window text is displayed using right-to-left reading-order properties.
    /// For other languages, the style is ignored.
    /// 
    /// 0x00002000
    RTLReading=WS_EX_RTLREADING,

    /// The window has a three-dimensional border style intended to be used for items that do not accept user input.
    /// 
    /// 0x00020000
    StaticEdge=WS_EX_STATICEDGE,

    /// The window is intended to be used as a floating toolbar.
    /// A tool window has a title bar that is shorter than a normal title bar, and the window title is drawn using a smaller font.
    /// A tool window does not appear in the taskbar or in the dialog that appears when the user presses ALT+TAB.
    /// If a tool window has a system menu, its icon is not displayed on the title bar.
    /// However, you can display the system menu by right-clicking or by typing ALT+SPACE.
    /// 
    /// 0x00000080
    ToolWindow=WS_EX_TOOLWINDOW,

    /// The window should be placed above all non-topmost windows and should stay above them,
    /// even when the window is deactivated.
    /// To add or remove this style, use the SetWindowPos function.
    /// 
    /// 0x00000008
    TopMost=WS_EX_TOPMOST,

    /// The window should not be painted until siblings beneath the window (that were created by the same thread) have been painted.
    /// The window appears transparent because the bits of underlying sibling windows have already been painted.
    /// To achieve transparency without these restrictions, use the SetWindowRgn function.
    /// 
    /// 0x00000020
    Transparent=WS_EX_TRANSPARENT,

    /// The window has a border with a raised edge.
    /// 
    /// 0x00000100
    WindowEdge=WS_EX_WINDOWEDGE,
}