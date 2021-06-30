use crate::windows::WinError;

use super::{
    // structures
    WindowClass,
    Monitor,
    // enums
    WindowEvent,
    LoopControl,
    // functions
    window_subclass_procedure,
};

use winapi::{
    shared::{
        ntdef::{LPSTR,LPCWSTR},
        windef::{
            HWND,
            HDC,
            HGLRC,
            RECT,
            POINT,
        }
    },

    um::{
        processthreadsapi::GetCurrentThreadId,
        wingdi::SwapBuffers,
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
            ReleaseDC,
            UpdateWindow,
            SetWindowPos,
            SetWindowLongPtrW,
            GetCursorPos,
            SetCursorPos,
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
            CW_USEDEFAULT,
            WM_APP,
        },
        commctrl::{
            // functions
            SetWindowSubclass,
            DefSubclassProc,
        },
        //errhandlingapi::{GetLastError},
    }
};

use std::{
    ptr::null_mut,
    ffi::{
        OsString,
    },
    os::windows::ffi::OsStrExt,
    mem::{
        transmute,
        zeroed
    },
    sync::{
        Arc,
        Mutex,
        RwLock,
    },
    collections::VecDeque,
};


pub enum Fullscreen{
    None,
    Monitor(Monitor)
}

/// Arguments that are passed to window subclass procedure.
/// 
/// The 'handler' argument defines reference to a event handler.
/// You can define your own handler.
/// To remove the default one in `EventLoop` use the `own_event_handler` feature.
/// 
/// The 'additional' argument is passed to the event handler as the `WindowEvent`'s 'argument' field.
pub struct WindowSubclassArguments{
    pub (crate) main_thread_id:usize,
    pub (crate) window_id:usize,
}

impl WindowSubclassArguments{
    pub fn new(main_thread_id:usize,window_id:usize)->WindowSubclassArguments{
        Self{
            main_thread_id,
            window_id,
        }
    }
}

/// A window with it's context.
/// 
/// The window mustn't outlive its class otherwise the class won't be unregistered properly.
/// 
/// Note that all windows/classes that an application creates/registers are destroyed/unregistered
/// when it terminates.
/// So there no need to do it when the application closes,
/// but it makes sense when you create-destroy windows and register-unregister classes at the run time.
pub struct Window{
    handle:HWND,
    context:HDC,
}

impl Window{
    pub fn new(
        class:&WindowClass,
        attributes:WindowAttributes,
        subclass_args:&WindowSubclassArguments,
    )->Result<Window,WinError>{
        let window_name:Vec<u16>=attributes.name
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect();

        let mut style=WS_SYSMENU|WS_BORDER;
        let mut extended_style=0;

        // Enabling file dropping
        #[cfg(feature="file_drop")]{
            extended_style|=WS_EX_ACCEPTFILES;
        }

        if attributes.visible{
            style|=WS_VISIBLE;
        }

        if attributes.topmost{
            extended_style|=WS_EX_TOPMOST;
        }

        // Размер, установленный пользователем
        let [mut width,mut height]=if let Some(size)=attributes.size{
            size
        }
        else{
            [CW_USEDEFAULT,CW_USEDEFAULT]
        };
        // Положение, установленное пользователем
        let [mut x,mut y]=if let Some(position)=attributes.position{
            position
        }
        else{
            [CW_USEDEFAULT,CW_USEDEFAULT]
        };

        match attributes.fullscreen{
            Fullscreen::None=>{
                style|=WS_SIZEBOX|WS_CAPTION|WS_MAXIMIZEBOX|WS_MINIMIZEBOX;
            }
            Fullscreen::Monitor(monitor)=>{
                if let Some(info)=monitor.get_monitor_info(){
                    style|=WS_POPUP;
                    style&=!(WS_SIZEBOX|WS_CAPTION|WS_MAXIMIZEBOX|WS_MINIMIZEBOX);

                    extended_style|=WS_EX_APPWINDOW;

                    x=info.rcMonitor.left;
                    y=info.rcMonitor.top;
                    width=info.rcMonitor.right-info.rcMonitor.left;
                    height=info.rcMonitor.bottom-info.rcMonitor.top;
                }
            }
        };

        unsafe{
            let window_handle=CreateWindowExW(
                extended_style, // Extended Window Style
                class.as_ptr(),
                window_name.as_ptr() as LPCWSTR,
                style, // Window Style
                x,
                y,
                width,
                height,
                null_mut(), // parent window
                null_mut(), // window menu
                null_mut(),
                null_mut()
            );

            if window_handle.is_null(){
                Err(WinError::get_last_error())
            }
            else{
                let mut subclass_id=0u32;
                SetWindowSubclass(
                    window_handle,
                    Some(window_subclass_procedure),
                    &mut subclass_id as *mut u32 as usize,
                    subclass_args as *const WindowSubclassArguments as usize,
                );

                Ok(
                    Self{
                        handle:window_handle,
                        context:GetDC(window_handle),
                    }
                )
            }
        }
    }

    pub fn handle(&self)->HWND{
        self.handle
    }

    pub fn context(&self)->HDC{
        self.context
    }
}

/// Requests and sending events.
impl Window{
    // pub fn request_redraw(&self){
    //     unsafe{
    //         UpdateWindow(self.handle);
    //     }
    // }

    pub (crate) fn destroy_local(&self){
        unsafe{
            DestroyWindow(self.handle);
        }
    }

    pub fn destroy(&self){
        unsafe{
            SendMessageW(self.handle,WM_APP+1,0,0);
        }
    }
}

/// Window sizes and positions.
impl Window{
    /// Returns the window size.
    /// 
    /// Возвращает размеры окна.
    /// 
    /// [width, height]
    pub fn size(&self)->[u32;2]{
        let mut window_rectangle=[0i32;4];
        unsafe{
            let ptr=&mut window_rectangle as *mut [i32;4];
            GetWindowRect(self.handle,ptr as usize as *mut RECT);
        }
        let [x1,y1,x2,y2]=window_rectangle;
        [
            (x2-x1) as u32,
            (y2-y1) as u32,
        ]
    }

    /// Returns coordinates of window's upper-left corner.
    /// 
    /// Возвращает координаты верхнего левого угла окна.
    /// 
    /// [x, y]
    pub fn position(&self)->[i32;2]{
        let mut window_rectangle=[0i32;4];
        unsafe{
            let ptr=&mut window_rectangle as *mut [i32;4];
            GetWindowRect(self.handle,ptr as usize as *mut RECT);
        }
        let [x1,y1,_,_]=window_rectangle;
        [
            x1,
            y1,
        ]
    }

    /// Returns the window rectangle with the coordinates of it's upper-left and lower-right corners.
    /// 
    /// Возвращает оконный прямоугольник с координатами верхнего левого и нижнего правого углов.
    /// 
    /// [x1, y1, x2, y2]
    pub fn rectangle(&self)->[i32;4]{
        let mut window_rectangle=[0i32;4];
        unsafe{
            let ptr=&mut window_rectangle as *mut [i32;4];
            GetWindowRect(self.handle,ptr as usize as *mut RECT);
        }
        window_rectangle
    }

    /// Returns window's client area size.
    /// 
    /// Возвращает размеры клиентской области окна.
    /// 
    /// [width, height]
    pub fn client_size(&self)->[u32;2]{
        let mut client_rectangle=[0i32;4];
        unsafe{
            let ptr=&mut client_rectangle as *mut [i32;4];
            GetClientRect(self.handle,ptr as usize as *mut RECT);
        }
        let [_,_,width,height]=client_rectangle;
        [
            width as u32,
            height as u32,
        ]
    }

    // pub unsafe fn set_window_position(&self){
    //     SetWindowPos(&self,)
    // }
}

impl Window{
    pub unsafe fn set_extended_style(&self,style:u32){
        SetWindowLongPtrW(self.handle,GWL_EXSTYLE,style as isize);
    }

    pub unsafe fn set_style(&self,style:u32){
        SetWindowLongPtrW(self.handle,GWL_STYLE,style as isize);
    }

    // pub fn set_fullscreen(&self,fullscreen:Fullscreen){
    //     match fullscreen{
    //         Fullscreen::None=>{

    //         }
    //         Fullscreen::Monitor(monitor)=>{
    //             if let Some(info)=monitor.get_monitor_info(){
    //                 let style = WS_POPUP;
    //                 style&=!(WS_SIZEBOX|WS_CAPTION|WS_MAXIMIZEBOX|WS_MINIMIZEBOX);

    //                 let extended_style=WS_EX_APPWINDOW;

    //                 x=info.rcMonitor.left;
    //                 y=info.rcMonitor.top;
    //                 width=info.rcMonitor.right-info.rcMonitor.left;
    //                 height=info.rcMonitor.bottom-info.rcMonitor.top;
    //             }
    //         }
    //     }
    // }
}

/// Cursor functions.
impl Window{
    /// Returns the window's cursor position.
    /// 
    /// Возвращает положение курсора окна.
    pub fn cursor_position(&self)->[i32;2]{
        unsafe{
            let mut point:POINT=std::mem::zeroed();
            GetCursorPos(&mut point);
            ScreenToClient(self.handle,&mut point);
            transmute(point)
        }
    }

    /// Sets the window's cursor position.
    /// 
    /// Устанавливает положение курсора окна.
    pub fn set_cursor_position(&self,[x,y]:[i32;2]){
        unsafe{
            let mut point=POINT{x,y};
            ClientToScreen(self.handle,&mut point);
            SetCursorPos(point.x,point.y);
        }
    }
}

impl Drop for Window{
    fn drop(&mut self){
        unsafe{
            ReleaseDC(self.handle,self.context);
            self.destroy();
        }
    }
}


pub struct WindowAttributes{
    /// The window name and title.
    pub name:OsString,

    /// The window size.
    /// 
    /// The default is `None`.
    pub size:Option<[i32;2]>,

    /// The window position.
    pub position:Option<[i32;2]>,

    /// The default is `true`.
    pub visible:bool,

    /// The window should be placed above all non-topmost windows
    /// and should stay above them,
    /// even when the window is deactivated.
    pub topmost:bool,

    /// If `Fullscreen::PrimaryMonitor` or `Fullscreen::Monitor` is set the size and position are ignored,
    /// but if some error accures they are used to build a window.
    /// 
    /// The default is `false`.
    pub fullscreen:Fullscreen,
}

impl WindowAttributes{
    pub fn new(name:&str)->WindowAttributes{
        Self{
            name:OsString::from(name),
            size:None,
            position:None,
            visible:true,
            topmost:false,
            fullscreen:Fullscreen::None,
        }
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
    /// Bottom-to-top painting order allows a descendent window to have translucency (alpha) and transparency (color-key) effects,
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