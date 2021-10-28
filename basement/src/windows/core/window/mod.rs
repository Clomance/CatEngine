use super::*;

mod window;
pub use window::Window;

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
            HWND,
        },
    },
    um::{
        winuser::{
            DefWindowProcA,
            DefWindowProcW,

            // window styles
            WS_BORDER,
            WS_CAPTION,
            WS_CHILDWINDOW,
            WS_CLIPCHILDREN,
            WS_CLIPSIBLINGS,
            WS_DISABLED,
            WS_DLGFRAME,
            WS_HSCROLL,
            WS_MAXIMIZE,
            WS_MAXIMIZEBOX,
            WS_MINIMIZE,
            WS_MINIMIZEBOX,
            WS_OVERLAPPED,
            WS_POPUP,
            WS_SIZEBOX,
            WS_SYSMENU,
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
            WS_EX_MDICHILD,
            WS_EX_NOACTIVATE,
            WS_EX_NOINHERITLAYOUT,
            WS_EX_NOPARENTNOTIFY,
            WS_EX_NOREDIRECTIONBITMAP,
            WS_EX_RIGHT,
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
            // class data
            GCW_ATOM,
            GCL_CBCLSEXTRA,
            GCL_CBWNDEXTRA,
            GCLP_HBRBACKGROUND,
            GCLP_HCURSOR,
            GCLP_HICON,
            GCLP_HICONSM,
            GCLP_HMODULE,
            GCLP_MENUNAME,
            GCL_STYLE,
            GCLP_WNDPROC,
        },
    }
};

/// A replacement for `HWND`.
/// Can be wraped with `Option` with null pointer optimization.
#[derive(Clone,Copy,Debug)]
#[repr(transparent)]
pub struct WindowHandle{
    inner:NonNull<HWND>,
}

implement_handle_wrapper!(WindowHandle,HWND);

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

#[repr(i32)]
#[derive(Clone,Copy,Debug)]
pub enum ClassData{
    /// Retrieves an ATOM value that uniquely identifies the window class.
    /// This is the same atom that the RegisterClassEx function returns.
    Atom=GCW_ATOM,

    /// Retrieves the size, in bytes, of the extra memory associated with the class.
    ClassExtraDataSize=GCL_CBCLSEXTRA,

    /// Retrieves the size, in bytes, of the extra window memory associated with each window in the class.
    /// For information on how to access this memory, see `Window::get_window_long_ptr`.
    WindowExtraDataSize=GCL_CBWNDEXTRA,

    /// Retrieves a handle to the background brush associated with the class.
    BackgroundBrushHandle=GCLP_HBRBACKGROUND,

    /// Retrieves a handle to the cursor associated with the class.
    CursorHandle=GCLP_HCURSOR,

    /// Retrieves a handle to the icon associated with the class.
    IconHandle=GCLP_HICON,

    /// Retrieves a handle to the small icon associated with the class.
    SmallIconHandle=GCLP_HICONSM,

    /// Retrieves a handle to the module that registered the class.
    ModuleHandle=GCLP_HMODULE,

    /// Retrieves the pointer to the menu name string.
    /// The string identifies the menu resource associated with the class.
    MenuName=GCLP_MENUNAME,

    /// Retrieves the window-class style bits.
    ClassStyle=GCL_STYLE,

    /// Retrieves the address of the window procedure,
    /// or a handle representing the address of the window procedure.
    /// You must use the CallWindowProc function to call the window procedure.
    WindowProcedute=GCLP_WNDPROC,

    ExtraData=0i32,
}


/// Calls the default window procedure
/// to provide default processing for any window messages
/// that an application does not process.
/// This function ensures that every message is processed.
/// `default_window_procedure` is called
/// with the same parameters received by the window procedure.
pub unsafe extern "system" fn default_window_procedure(
    window_handle:WindowHandle,
    message:u32,
    w_param:usize,
    l_param:isize
)->isize{
    DefWindowProcW(window_handle.as_raw(),message,w_param,l_param)
}

pub unsafe extern "system" fn default_window_procedure_ansi(
    window_handle:WindowHandle,
    message:u32,
    w_param:usize,
    l_param:isize
)->isize{
    DefWindowProcA(window_handle.as_raw(),message,w_param,l_param)
}