use super::{
    InstanceHandle,
    window::{
        WindowHandle,
        default_window_procedure_wide,
    },
    icon::IconHandle,
    cursor::CursorHandle,
    brush::BrushHandle
};

use core::{
    mem::{
        size_of,
        transmute
    },
    ptr::null_mut,
    num::{
        NonZeroU16,
        NonZeroIsize
    }
};

use winapi::{
    um::{
        winuser::{
            RegisterClassExW,
            UnregisterClassW,
            GetClassInfoExW,
            // window class styles
            CS_HREDRAW,
            CS_VREDRAW,
            CS_DBLCLKS,
            CS_OWNDC,
            CS_CLASSDC,
            CS_PARENTDC,
            CS_NOCLOSE,
            CS_SAVEBITS,
            CS_BYTEALIGNCLIENT,
            CS_BYTEALIGNWINDOW,
            CS_GLOBALCLASS,
            CS_DROPSHADOW,

            // window background colours
            COLOR_ACTIVEBORDER,
            COLOR_ACTIVECAPTION,
            COLOR_APPWORKSPACE,
            COLOR_BACKGROUND,
            COLOR_BTNFACE,
            COLOR_BTNSHADOW,
            COLOR_BTNTEXT,
            COLOR_CAPTIONTEXT,
            COLOR_GRAYTEXT,
            COLOR_HIGHLIGHT,
            COLOR_HIGHLIGHTTEXT,
            COLOR_INACTIVEBORDER,
            COLOR_INACTIVECAPTION,
            COLOR_MENU,
            COLOR_MENUTEXT,
            COLOR_SCROLLBAR,
            COLOR_WINDOW,
            COLOR_WINDOWFRAME,
            COLOR_WINDOWTEXT
        }
    }
};



#[repr(u32)]
#[derive(Copy,Clone,Debug)]
pub enum WindowClassStyle{
    /// Redraws the entire window
    /// if a movement or size adjustment changes the height of the client area.
    /// 
    /// 0x0001
    VerticalRedraw=CS_VREDRAW,

    /// Redraws the entire window
    /// if a movement or size adjustment changes the width of the client area.
    /// 
    /// 0x0002
    HorizontalRedraw=CS_HREDRAW,

    /// Sends a double-click message to the window procedure
    /// when the user double-clicks the mouse while the cursor is within a window belonging to the class.
    /// 
    /// 0x0008
    DoubleClicks=CS_DBLCLKS,

    /// Allocates a unique device context for each window in the class.
    /// 
    /// 0x0020
    OwnDeviceContext=CS_OWNDC,

    /// Allocates one device context to be shared by all windows in the class.
    /// Because window classes are process specific,
    /// it is possible for multiple threads of an application to create a window of the same class.
    /// It is also possible for the threads to attempt to use the device context simultaneously.
    /// When this happens, the system allows only one thread to successfully finish its drawing operation.
    /// 
    /// 0x0040
    ClassDeviceContext=CS_CLASSDC,

    /// Sets the clipping rectangle of the child window to that of the parent window so that the child can draw on the parent.
    /// A window with the `WindowClassStyle::ParentDeviceContext` style bit
    /// receives a regular device context from the system's cache of device contexts.
    /// It does not give the child the parent's device context or device context settings.
    /// Specifying `WindowClassStyle::ParentDeviceContext` enhances an application's performance.
    /// 
    /// 0x0080
    ParentDeviceContext=CS_PARENTDC,

    /// Disables Close on the window menu.
    /// 
    /// 0x0200
    NoClose=CS_NOCLOSE,

    /// Saves, as a bitmap, the portion of the screen image obscured by a window of this class.
    /// When the window is removed, the system uses the saved bitmap to restore the screen image,
    /// including other windows that were obscured.
    /// Therefore, the system does not send WM_PAINT messages to windows that were obscured
    /// if the memory used by the bitmap has not been discarded and if other screen actions have not invalidated the stored image.
    /// This style is useful for small windows (for example, menus or dialog boxes)
    /// that are displayed briefly and then removed before other screen activity takes place.
    /// This style increases the time required to display the window, because the system must first allocate memory to store the bitmap.
    /// 
    /// 0x0800
    SaveBits=CS_SAVEBITS,

    /// Aligns the window's client area on a byte boundary (in the x direction).
    /// This style affects the width of the window and its horizontal placement on the display.
    /// 
    /// 0x1000
    ByteAlignClient=CS_BYTEALIGNCLIENT,

    /// Aligns the window on a byte boundary (in the x direction).
    /// This style affects the width of the window and its horizontal placement on the display.
    /// 
    /// 0x2000
    ByteAlignWindow=CS_BYTEALIGNWINDOW,

    /// Indicates that the window class is an application global class.
    /// For more information, see the "Application Global Classes" section of About Window Classes.
    /// 
    /// 0x4000
    GlobalClass=CS_GLOBALCLASS,

    /// Enables the drop shadow effect on a window.
    /// The effect is turned on and off through SPI_SETDROPSHADOW.
    /// Typically, this is enabled for small,
    /// short-lived windows such as menus to emphasize their Z-order relationship to other windows.
    /// Windows created from a class with this style must be top-level windows;
    /// they may not be child windows.
    /// 
    /// 0x00020000
    DropShadow=CS_DROPSHADOW,
}



/// Represents class styles.
#[derive(Clone,Copy,Debug)]
pub struct WindowClassStyles{
    flag:u32
}

impl WindowClassStyles{
    /// Creates a flag with no styles set.
    pub const fn new()->WindowClassStyles{
        Self{
            flag:0u32,
        }
    }

    /// Sets a style.
    pub const fn set(mut self,style:WindowClassStyle)->WindowClassStyles{
        self.flag|=style as u32;
        self
    }

    /// Removes a style.
    pub const fn remove(mut self,style:WindowClassStyle)->WindowClassStyles{
        self.flag&=!(style as u32);
        self
    }
}



/// A class identifier.
/// Contains a null-terminated string (it specifies the window class name)
/// or a class atom.
#[derive(Clone,Copy,Debug)]
#[repr(transparent)]
pub struct ClassIdentifier{
    inner:isize,
}

impl ClassIdentifier{
    /// `name` specifies the window class name.
    #[inline(always)]
    pub fn from_name(name:*const u16)->ClassIdentifier{
        Self{
            inner:name as isize,
        }
    }

    #[inline(always)]
    pub fn from_atom(atom:ClassAtom)->ClassIdentifier{
        Self{
            inner:atom.as_raw() as isize,
        }
    }

    pub (crate) const fn as_ptr(&self)->*const u16{
        self.inner as *const u16
    }
}



#[derive(Clone,Copy)]
#[repr(transparent)]
pub struct ClassAtom(NonZeroU16);

impl ClassAtom{
    #[inline(always)]
    pub fn from_raw(raw:u16)->Option<ClassAtom>{
        unsafe{
            transmute(raw)
        }
    }

    #[inline(always)]
    pub unsafe fn from_raw_unchecked(raw:u16)->ClassAtom{
        transmute(raw)
    }

    #[inline(always)]
    pub fn to_raw(handle:Option<ClassAtom>)->u16{
        unsafe{
            transmute(handle)
        }
    }

    #[inline(always)]
    pub fn as_raw(&self)->u16{
        unsafe{
            transmute(self.0)
        }
    }
}

#[derive(Copy,Clone,Debug)]
#[repr(i32)]
pub enum WindowBackgroundSystemColour{
    ActiveBorder=COLOR_ACTIVEBORDER,
    ActioveCaption=COLOR_ACTIVECAPTION,
    AppWorkspace=COLOR_APPWORKSPACE,
    Background=COLOR_BACKGROUND,
    ButtonFace=COLOR_BTNFACE,
    ButtonShadow=COLOR_BTNSHADOW,
    ButtonText=COLOR_BTNTEXT,
    CaptionText=COLOR_CAPTIONTEXT,
    GrayText=COLOR_GRAYTEXT,
    Highlight=COLOR_HIGHLIGHT,
    HighlightText=COLOR_HIGHLIGHTTEXT,
    InactiveBorder=COLOR_INACTIVEBORDER,
    InactioveCaption=COLOR_INACTIVECAPTION,
    Menu=COLOR_MENU,
    MenuText=COLOR_MENUTEXT,
    Scrollbar=COLOR_SCROLLBAR,
    Window=COLOR_WINDOW,
    WindowFrame=COLOR_WINDOWFRAME,
    WindowText=COLOR_WINDOWTEXT,
}



/// A handle to the class background brush.
/// This member can be a handle to the brush to be used for painting the background,
/// or it can be a colour value.
#[derive(Clone,Copy,Debug)]
#[repr(transparent)]
pub struct WindowBackgroundColour(NonZeroIsize);

impl WindowBackgroundColour{
    #[inline(always)]
    pub fn system_colour(colour:WindowBackgroundSystemColour)->WindowBackgroundColour{
        Self(unsafe{transmute(colour as isize+1)})
    }

    #[inline(always)]
    pub fn brush(brush:BrushHandle)->WindowBackgroundColour{
        Self(unsafe{transmute(brush.as_raw())})
    }
}



/// Contains window class information.
/// It is used with the `WindowClass::register` and `WindowClass::get_info` functions.
#[derive(Clone)]
#[repr(C)]
pub struct WindowClassInfo{
    size:u32,
    /// The class styles.
    /// This member can be any combination of `WindowClassStyle`.
    /// 
    /// The default is no styles.
    pub styles:WindowClassStyles,

    /// A pointer to the window procedure.
    /// You must use the `CallWindowProc` function to call the window procedure.
    /// 
    /// The default is `default_window_procedure` (`DefWindowProcW`).
    pub window_procedure:unsafe extern "system" fn(WindowHandle,u32,usize,isize)->isize,

    /// The number of extra bytes to allocate following the window-class structure.
    /// The system initializes the bytes to zero.
    /// 
    /// The default is `0`.
    pub extra_class_data:i32,

    /// The number of extra bytes to allocate following the window instance.
    /// The system initializes the bytes to zero.
    /// If an application uses `WindowClassInfo` to register a dialog box created
    /// by using the CLASS directive in the resource file,
    /// it must set this member to `8`.
    /// 
    /// The default is `0`.
    pub extra_window_data:i32,

    /// A handle to the instance that contains the window procedure for the class.
    /// 
    /// The default is `None`.
    pub instance:Option<InstanceHandle>,

    /// A handle to the class icon.
    /// This member must be a handle to an icon resource.
    /// If this member is `None`, the system provides a default icon.
    /// 
    /// The default is `None`.
    pub icon:Option<IconHandle>,

    /// A handle to the class cursor.
    /// This member must be a handle to a cursor resource.
    /// If this member is `None`, an application must explicitly set the cursor shape
    /// whenever the mouse moves into the application's window.
    /// 
    /// The default is `None`.
    pub cursor:Option<CursorHandle>,

    /// A handle to the class background brush.
    /// This member can be a handle to the brush to be used for painting the background,
    /// or it can be a colour value.
    /// 
    /// The system automatically deletes class background brushes
    /// when the class is unregistered by using `Class::unregister`.
    /// An application should not delete these brushes.
    /// 
    /// When this member is `None`, an application must paint its own background
    /// whenever it is requested to paint in it's client area.
    /// To determine whether the background must be painted,
    /// an application can either process the `WM_ERASEBKGND` message
    /// or test the `fErase` member of the `PAINTSTRUCT` structure filled by the `BeginPaint` function.
    /// 
    /// The default is `None`.
    pub background:Option<WindowBackgroundColour>,

    /// Pointer to a null-terminated character string
    /// that specifies the resource name of the class menu,
    /// as the name appears in the resource file.
    /// If you use an integer to identify the menu, use the `MAKEINTRESOURCE` macro.
    /// If this member is `None`, windows belonging to this class have no default menu.
    /// 
    /// The default is `null`.
    pub menu_name:*const u16,

    /// A pointer to a null-terminated string or is an atom.
    /// If this parameter is an atom,
    /// it must be a class atom created by a previous call to the `Class::register` function.
    /// The atom must be in the low-order word of `class_name`; the high-order word must be zero.
    /// 
    /// If `class_name` is a string, it specifies the window class name.
    /// The class name can be any name registered with `Class::register`,
    /// or any of the predefined control-class names.
    /// 
    /// The maximum length for `class_name` is 256.
    /// If `class_name` is greater than the maximum length, the `Class::register` function will fail.
    /// 
    /// The default is `null`.
    pub class_name:*const u16,

    /// A handle to a small icon that is associated with the window class.
    /// If this member is `None`, the system searches the icon resource specified
    /// by the `icon` member for an icon of the appropriate size to use as the small icon.
    /// 
    /// The default is `None`.
    pub small_icon:Option<IconHandle>,
}

impl WindowClassInfo{
    pub const fn new()->WindowClassInfo{
        Self{
            size:size_of::<WindowClassInfo>() as u32,
            styles:WindowClassStyles::new(),
            window_procedure:default_window_procedure_wide,
            extra_class_data:0i32,
            extra_window_data:0i32,
            instance:None,
            icon:None,
            cursor:None,
            background:None,
            menu_name:null_mut(),
            class_name:null_mut(),
            small_icon:None,
        }
    }
}



/// All window classes that an application registers are unregistered when it terminates.
/// 
/// No window classes registered by a DLL are unregistered when the DLL is unloaded.
/// A DLL must explicitly unregister its classes when it is unloaded.
/// 
/// Before calling `WindowClass::unregister`,
/// an application must destroy all windows created with the specified class.
pub struct WindowClass;

impl WindowClass{
    /// Registers a window class for subsequent use in calls to the `Window::create` function.
    /// 
    /// If the function succeeds,
    /// the return value is a class atom that uniquely identifies the class being registered.
    /// If the function fails, the return value is zero.
    /// To get extended error information, call `WindowsCore::get_last_error`.
    pub fn register(
        class_info:&WindowClassInfo,
    )->Option<ClassAtom>{
        unsafe{
            ClassAtom::from_raw(RegisterClassExW(transmute(class_info)))
        }
    }

    #[inline(always)]
    pub fn register_indirect(info:&WindowClassInfo)->Option<ClassAtom>{
        unsafe{
            ClassAtom::from_raw(RegisterClassExW(transmute(info)))
        }
    }

    /// Unregisters a window class, freeing the memory required for the class.
    /// 
    /// System classes, such as dialog box controls, cannot be unregistered.
    /// 
    /// If the function succeeds, the return value is `true`.
    /// If the class could not be found or if a window still exists that was created with the class,
    /// the return value is `false`.
    /// To get extended error information, call `WindowsCore::get_last_error`.
    #[inline(always)]
    pub fn unregister(class:ClassIdentifier,instance:Option<InstanceHandle>)->bool{
        unsafe{
            UnregisterClassW(class.inner as *const _,InstanceHandle::to_raw(instance))!=0
        }
    }

    /// Retrieves information about a window class,
    /// including a handle to the small icon associated with the window class.
    /// The function does not retrieve a handle to the small icon.
    /// 
    /// If the function finds a matching class and successfully copies the data,
    /// the return value is `true`.
    /// If the function does not find a matching class and successfully copy the data,
    /// the return value is `false`.
    /// To get extended error information, call `WindowsCore::get_last_error`.
    #[inline(always)]
    pub fn get_info(
        class:ClassIdentifier,
        instance:Option<InstanceHandle>,
        info:&mut WindowClassInfo
    )->bool{
        unsafe{
            GetClassInfoExW(
                InstanceHandle::to_raw(instance),
                class.inner as *const _,
                transmute(info)
            )!=0
        }
    }
}