use crate::implement_handle_wrapper;

use super::{
    window_class::ClassIdentifier,
    device_context::DeviceContextHandle,
};

use core::{
    mem::{
        transmute,
        transmute_copy,
    },
    ptr::NonNull,
};

use winapi::{
    shared::{
        windef::{
            HMENU,
            HWND,
        },
        minwindef::HINSTANCE,
    },

    um::{
        winuser::{
            ShowWindow,
            // SetFocus,
            // SetForegroundWindow,
            // SetCapture,
            CreateWindowExW,
            DestroyWindow,
            // SendMessageW,
            GetDC,
            GetWindowRect,
            GetClientRect,
            // RedrawWindow,
            SetWindowPos,
            SetWindowLongPtrW,
            GetWindowLongPtrW,
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
            // show command
            SW_HIDE,
            SW_SHOWNORMAL,
            SW_SHOWMINIMIZED,
            SW_SHOWMAXIMIZED,
            SW_SHOWNOACTIVATE,
            SW_SHOW,
            SW_MINIMIZE,
            SW_SHOWMINNOACTIVE,
            SW_SHOWNA,
            SW_RESTORE,
            SW_SHOWDEFAULT,
            SW_FORCEMINIMIZE,
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
            GWLP_HINSTANCE,
            GWLP_ID,
        },
    }
};

/// Represents a window style.
#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum WindowStyle{
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

/// Represents an extended window style.
#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum ExtendedWindowStyle{
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

#[repr(i32)]
#[derive(Clone,Copy,Debug)]
pub enum WindowData{
    /// Sets a new window style.
    Style=GWL_STYLE,

    /// Sets a new extended window style.
    ExtendedStyle=GWL_EXSTYLE,

    /// Sets a new application instance handle.
    InstanceHandle=GWLP_HINSTANCE,

    /// Sets a new identifier of the child window.
    /// The window cannot be a top-level window.
    ChildID=GWLP_ID,

    /// Sets the user data associated with the window.
    /// This data is intended for use by the application that created the window.
    /// Its value is initially zero.
    UserData=GWLP_USERDATA,

    /// Sets a new address for the window procedure.
    WindowProcedure=GWLP_WNDPROC,

    ///Sets the return value of a message processed in the dialog box procedure.
    MessageResult=0i32,

    /// Sets the new pointer to the dialogue box procedure.
    DialogueBoxProcedure=8i32,

    /// Sets new extra information that is private to the application,
    /// such as handles or pointers.
    User=16i32,
}

/// Controls how the window is to be shown.
#[repr(i32)]
pub enum ShowCommand{
    /// Hides the window and activates another window.
    Hide=SW_HIDE,

    /// Activates and displays a window.
    /// If the window is minimized or maximized,
    /// the system restores it to its original size and position.
    /// An application should specify this flag
    /// when displaying the window for the first time.
    ShowNormal=SW_SHOWNORMAL,

    /// Activates the window and displays it as a minimized window.
    ShowMinimized=SW_SHOWMINIMIZED,

    /// Activates the window and displays it as a maximized window.
    ShowMaximized=SW_SHOWMAXIMIZED,

    /// Displays a window in its most recent size and position.
    /// This value is similar to `ShowCommand::ShowNormal`,
    /// except that the window is not activated.
    ShowNoActivate=SW_SHOWNOACTIVATE,

    /// Activates the window and displays it in its current size and position.
    Show=SW_SHOW,

    /// Minimizes the specified window and activates the next top-level window in the Z order.
    Minimize=SW_MINIMIZE,

    /// Displays the window as a minimized window.
    /// This value is similar to `ShowCommand::ShowMinimized`,
    /// except the window is not activated.
    ShowMinimizedNoActivate=SW_SHOWMINNOACTIVE,

    /// Displays the window in its current size and position.
    /// This value is similar to `ShowCommand::Show`,
    /// except that the window is not activated.
    ShowNoActivate2=SW_SHOWNA,

    /// Activates and displays the window.
    /// If the window is minimized or maximized,
    /// the system restores it to its original size and position.
    /// An application should specify this flag when restoring a minimized window.
    Restore=SW_RESTORE,

    /// Sets the show state based on the `ShowCommand` value
    /// specifiedin the `STARTUPINFO` structure passed to the CreateProcess function
    /// by the program that started the application.
    ShowDefault=SW_SHOWDEFAULT,

    /// Minimizes a window, even if the thread that owns the window is not responding.
    /// This flag should only be used when minimizing windows from a different thread.
    ForceMinimized=SW_FORCEMINIMIZE,
}

/// Represents window styles.
pub struct WindowStyles{
    pub flag:u32
}

impl WindowStyles{
    /// Creates a flag with the given styles.
    pub const fn raw(flag:u32)->WindowStyles{
        Self{
            flag,
        }
    }

    /// Creates a flag with no styles set.
    pub const fn new()->WindowStyles{
        Self{
            flag:0u32,
        }
    }

    /// Sets a style.
    pub const fn set(mut self,style:WindowStyle)->WindowStyles{
        self.flag|=style as u32;
        self
    }

    /// Removes a style.
    pub const fn remove(mut self,style:WindowStyle)->WindowStyles{
        self.flag&=!(style as u32);
        self
    }
}

/// Represents extended window styles.
pub struct ExtendedWindowStyles{
    pub flag:u32
}

impl ExtendedWindowStyles{
    /// Creates a flag with the given styles.
    pub const fn raw(flag:u32)->ExtendedWindowStyles{
        Self{
            flag,
        }
    }

    /// Creates a flag with no styles set.
    pub const fn new()->ExtendedWindowStyles{
        Self{
            flag:0u32,
        }
    }

    /// Sets a style.
    pub const fn set(mut self,style:ExtendedWindowStyle)->ExtendedWindowStyles{
        self.flag|=style as u32;
        self
    }

    /// Removes a style.
    pub const fn remove(mut self,style:ExtendedWindowStyle)->ExtendedWindowStyles{
        self.flag&=!(style as u32);
        self
    }
}

/// The replacement for `HWND`.
/// Can be wraped with `Option` with null pointer optimization.
#[derive(Clone,Copy)]
#[repr(transparent)]
pub struct WindowHandle{
    inner:NonNull<HWND>,
}

implement_handle_wrapper!(WindowHandle,HWND);

pub struct Window;

impl Window{
    pub const fn new()->Window{
        Self
    }
}

impl Window{
    /// Creates a window.
    /// 
    /// The class can be any name registered with `WindowClass::register`,
    /// provided that the module that registers the class is also the module that creates the window.
    /// The class can also be any of the predefined system class names.
    #[inline(always)]
    pub unsafe fn create<P>(
        &self,
        class:ClassIdentifier,
        window_name:*const u16,
        style:WindowStyles,
        extended_style:ExtendedWindowStyles,
        [x,y,width,height]:[i32;4],
        parent_window:Option<WindowHandle>,
        menu:HMENU,
        instance:HINSTANCE,
        create_parameters:*mut P,
    )->Option<WindowHandle>{
        WindowHandle::from_raw(
            CreateWindowExW(
                extended_style.flag,
                class.as_ptr(),
                window_name,
                style.flag,
                x,y,width,height,
                WindowHandle::to_raw(parent_window),
                menu,
                instance,
                create_parameters as *mut _,
            )
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
    /// If the function succeeds, returns `true`.
    /// If the function fails, returns `false`.
    /// To get extended error information, call `WinCore::get_last_error`.
    /// 
    /// A thread cannot use `Window::destroy` to destroy a window created by a different thread.
    /// 
    /// If the window being destroyed is a child window that does not have the `WS_EX_NOPARENTNOTIFY` style, a `WM_PARENTNOTIFY` message is sent to the parent.
    #[inline(always)]
    pub unsafe fn destroy(&self,window:WindowHandle)->bool{
        DestroyWindow(window.as_raw())!=0
    }

    /// Retrieves a handle to a device context (DC) for the client area of a specified window or for the entire screen.
    /// You can use the returned handle in subsequent GDI functions to draw in the DC.
    /// The device context is an opaque data structure, whose values are used internally by GDI.
    /// 
    /// `window` is a handle to the window whose DC is to be retrieved;
    /// if this value is NULL, `Window::get_device_context` retrieves the DC for the entire screen.
    /// 
    /// Note that the handle to the DC can only be used by a single thread at any one time.
    /// After painting with a common DC,
    /// the `Window::release_device_context` function must be called to release the DC.
    /// Class and private DCs do not have to be released.
    /// `Window::release_device_context` must be called from the same thread that called `Window::get_device_context`.
    /// The number of DCs is limited only by available memory.
    /// 
    /// If the function succeeds,
    /// the return value is a handle to the DC for the specified window's client area.
    /// If the function fails, the return value is `NULL`.
    #[inline(always)]
    pub fn get_device_context(&self,window:Option<WindowHandle>)->Option<DeviceContextHandle>{
        unsafe{
            DeviceContextHandle::from_raw(GetDC(WindowHandle::to_raw(window)))
        }
    }

    #[inline(always)]
    pub unsafe fn get_device_context_unchecked(&self,window:Option<WindowHandle>)->DeviceContextHandle{
        DeviceContextHandle::from_raw_unchecked(GetDC(WindowHandle::to_raw(window)))
    }

    /// Retrieves the dimensions of the bounding rectangle of the specified window.
    /// The dimensions are given in screen coordinates that are relative to the upper-left corner of the screen.
    /// 
    /// If the function succeeds, the return value is nonzero.
    /// If the function fails, the return value is zero.
    /// To get extended error information, call `WinCore::get_last_error`.
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
    pub unsafe fn get_window_rectangle(&self,window:WindowHandle,rectangle:&mut [i32;4])->bool{
        GetWindowRect(window.as_raw(),transmute(rectangle))!=0
    }

    /// Retrieves the coordinates of a window's client area.
    /// The client coordinates specify the upper-left and lower-right corners of the client area.
    /// Because client coordinates are relative to the upper-left corner of a window's client area,
    /// the coordinates of the upper-left corner are (0,0).
    /// 
    /// If the function succeeds, the return value is nonzero.
    /// 
    /// If the function fails, the return value is zero.
    /// To get extended error information, call `WinCore::get_last_error`.
    /// 
    /// In conformance with conventions for the `RECT` structure,
    /// the bottom-right coordinates of the returned rectangle are exclusive.
    /// In other words, the pixel at (right, bottom) lies immediately outside the rectangle.
    #[inline(always)]
    pub unsafe fn get_client_rectangle(&self,window:WindowHandle,rectangle:&mut [i32;4])->bool{
        GetClientRect(window.as_raw(),transmute(rectangle))!=0
    }

    /// Changes the size, position, and Z order of a child, pop-up, or top-level window.
    /// These windows are ordered according to their appearance on the screen.
    /// The topmost window receives the highest rank and is the first window in the Z order.
    /// 
    /// If the function succeeds, the return value is nonzero.
    /// If the function fails, the return value is zero.
    /// To get extended error information, call `WinCore::get_last_error`.
    /// 
    /// If you have changed certain window data using SetWindowLong,
    /// you must call SetWindowPos for the changes to take effect.
    /// Use the following combination for `flag`: `SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED`.
    /// 
    /// A window can be made a topmost window either by setting
    /// the `insert_after` parameter to `WindowHandle_TOPMOST` and ensuring that the `SWP_NOZORDER` flag is not set,
    /// or by setting a window's position in the Z order
    /// so that it is above any existing topmost windows.
    /// When a non-topmost window is made topmost,
    /// its owned windows are also made topmost.
    /// Its owners, however, are not changed.
    /// 
    /// If neither the `SWP_NOACTIVATE` nor `SWP_NOZORDER` flag is specified
    /// (that is, when the application requests that a window be simultaneously activated and its position in the Z order changed),
    /// the value specified in `insert_after` is used only in the following circumstances.
    /// 
    /// Neither the `WindowHandle_TOPMOST` nor `WindowHandle_NOTOPMOST` flag is specified in `insert_after`.
    /// The window identified by `window` is not the active window.
    /// An application cannot activate an inactive window without also bringing it to the top of the Z order.
    /// Applications can change an activated window's position in the Z order without restrictions,
    /// or it can activate a window and then move it to the top of the topmost or non-topmost windows.
    /// If a topmost window is repositioned to the bottom (`WindowHandle_BOTTOM`) of the Z order or after any non-topmost window,
    /// it is no longer topmost.
    /// When a topmost window is made non-topmost,
    /// its owners and its owned windows are also made non-topmost windows.
    /// 
    /// A non-topmost window can own a topmost window, but the reverse cannot occur.
    /// Any window (for example, a dialog box) owned by a topmost window is itself made a topmost window,
    /// to ensure that all owned windows stay above their owner.
    /// 
    /// If an application is not in the foreground, and should be in the foreground,
    /// it must call the `SetForegroundWindow` function.
    /// 
    /// To use `Window::set_window_position` to bring a window to the top,
    /// the process that owns the window must have `SetForegroundWindow` permission.
    #[inline(always)]
    pub unsafe fn set_window_position(&self,window:WindowHandle,insert_after:Option<WindowHandle>,[x,y,width,height]:[i32;4],flag:u32)->bool{
        SetWindowPos(window.as_raw(),WindowHandle::to_raw(insert_after),x,y,width,height,flag)!=0
    }

    /// Changes an attribute of the specified window.
    /// The function also sets a value at the specified offset in the extra window memory.
    /// 
    /// If the function succeeds, the return value is the previous value of the specified offset.
    /// If the function fails, the return value is zero.
    /// To get extended error information, call `WinCore::get_last_error`.
    /// If the previous value is zero and the function succeeds,
    /// the return value is zero,
    /// but the function does not clear the last error information.
    /// To determine success or failure,
    /// clear the last error information by calling `SetLastError` with 0,
    /// then call `Window::set_window_long_ptr`.
    /// Function failure will be indicated
    /// by a return value of zero and a `WinCore::get_last_error` result that is nonzero.
    #[inline(always)]
    pub unsafe fn set_window_long_ptr(&self,window:WindowHandle,index:WindowData,value:isize)->isize{
        SetWindowLongPtrW(window.as_raw(),index as i32,value)
    }

    /// Retrieves information about the specified window.
    /// The function also retrieves the value at a specified offset into the extra window memory.
    /// 
    /// If the function succeeds, the return value is the requested value.
    /// If the function fails, the return value is zero.
    /// To get extended error information, call `WinCore::get_last_error`.
    /// If `Window::set_window_long_ptr` has not been called previously,
    /// `Window::get_window_long_ptr` returns zero for values in the extra window or class memory.
    #[inline(always)]
    pub unsafe fn get_window_long_ptr(&self,window:WindowHandle,index:WindowData)->isize{
        GetWindowLongPtrW(window.as_raw(),index as i32)
    }

    /// Sets the specified window's show state.
    /// 
    /// The `command` parameter is ignored the first time an application calls `ShowCommand::Show`,
    /// if the program that launched the application provides a `STARTUPINFO` structure.
    /// Otherwise, the first time `Window::show_window` is called,
    /// the value should be the value obtained by the `WinMain` function in its `command` parameter.
    /// 
    /// To perform certain special effects when showing or hiding a window, use `AnimateWindow`.
    /// The first time an application calls `Window::show_window`,
    /// it should use the `WinMain` function's `command` parameter as its `command` parameter.
    /// Subsequent calls to `Window::show_window` must use one of the values in the given list,
    /// instead of the one specified by the `WinMain` function's `command` parameter.
    /// As noted in the discussion of the `command` parameter,
    /// the `command` value is ignored in the first call to `Window::show_window`
    /// if the program that launched the application specifies startup information in the structure.
    /// In this case, `Window::show_window` uses the information specified in the `STARTUPINFO` structure to show the window.
    /// On subsequent calls, the application must call `Window::show_window` with `command` set to `ShowCommand::ShowDefault`
    /// to use the startup information provided by the program that launched the application.
    /// This behavior is designed for the following situations:
    /// - Applications create their main window by calling `CreateWindow` with the `WS_VISIBLE` flag set.
    /// - Applications create their main window by calling `CreateWindow` with the `WS_VISIBLE` flag cleared,
    /// and later call `Window::show_window` with the `ShowCommand::Show` flag set to make it visible.
    /// 
    /// If the window was previously visible, returns `true`.
    /// If the window was previously hidden, returns `false`.
    #[inline(always)]
    pub unsafe fn show_window(&self,window:WindowHandle,command:ShowCommand)->bool{
        ShowWindow(window.as_raw(),command as i32)!=0
    }

    /// Converts the client-area coordinates of a specified point to screen coordinates.
    /// 
    /// `point` contains the client coordinates to be converted.
    /// The new screen coordinates are copied into this structure if the function succeeds.
    /// 
    /// The function replaces the client-area coordinates in `point` with the screen coordinates.
    /// The screen coordinates are relative to the upper-left corner of the screen.
    /// Note, a screen-coordinate point that is above the window's client area has a negative y-coordinate.
    /// Similarly, a screen coordinate to the left of a client area has a negative x-coordinate.
    /// 
    /// All coordinates are device coordinates.
    /// 
    /// If the function succeeds, returns `true`.
    /// If the function fails, return `false`.
    #[inline(always)]
    pub unsafe fn client_to_screen(&self,window:WindowHandle,point:&mut [i32;2])->bool{
        ClientToScreen(window.as_raw(),transmute(point))!=0
    }

    /// Converts the screen coordinates of a specified point on the screen to client-area coordinates.
    /// 
    /// `point` contains the client coordinates to be converted.
    /// 
    /// The function uses the window identified by the `window` parameter
    /// and the screen coordinates given in `point` to compute client coordinates.
    /// It then replaces the screen coordinates with the client coordinates.
    /// The new coordinates are relative to the upper-left corner of the specified window's client area.
    /// 
    /// The `Window::screen_to_client` function assumes the specified point is in screen coordinates.
    /// 
    /// All coordinates are in device units.
    /// 
    /// Do not use `Window::screen_to_client` when in a mirroring situation, that is,
    /// when changing from left-to-right layout to right-to-left layout.
    /// Instead, use `MapWindowPoints`.
    /// For more information, see "Window Layout and Mirroring" in Window Features.
    /// 
    /// If the function succeeds, returns `true`.
    /// If the function fails, return `false`.
    #[inline(always)]
    pub unsafe fn screen_to_client(&self,window:WindowHandle,point:&mut [i32;2])->bool{
        ScreenToClient(window.as_raw(),transmute(point))!=0
    }
}