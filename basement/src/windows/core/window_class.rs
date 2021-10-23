use super::{
    InstanceHandle,
    window::WindowHandle,
    icon::IconHandle,
    cursor::CursorHandle,
    brush::BrushHandle,
};

use core::{
    mem::{
        size_of,
        transmute,
        transmute_copy,
    },
    num::NonZeroU16,
};

use winapi::{
    shared::{
        windef::{
            HBRUSH,
        }
    },

    um::{
        winuser::{
            RegisterClassExW,
            UnregisterClassW,
            WNDCLASSEXW,
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
        },
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
    /// A window with the CS_PARENTDC style bit receives a regular device context from the system's cache of device contexts.
    /// It does not give the child the parent's device context or device context settings.
    /// Specifying CS_PARENTDC enhances an application's performance.
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
/// or a class atom..
pub struct ClassIdentifier{
    identifier:isize,
}

impl ClassIdentifier{
    /// `name` specifies the window class name.
    #[inline(always)]
    pub fn name(name:*const u16)->ClassIdentifier{
        Self{
            identifier:name as isize,
        }
    }

    #[inline(always)]
    pub fn atom(atom:ClassAtom)->ClassIdentifier{
        Self{
            identifier:atom.as_raw() as isize,
        }
    }

    pub const fn as_ptr(&self)->*const u16{
        self.identifier as *const u16
    }
}

#[derive(Clone,Copy)]
#[repr(transparent)]
pub struct ClassAtom{
    inner:NonZeroU16
}

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
            transmute_copy(self)
        }
    }
}


/// Contains window class information.
/// It is used with the RegisterClassEx and GetClassInfoEx functions.
#[repr(C)]
pub struct WindowClassInfo{
    size:u32,
    pub styles:WindowClassStyles,
    pub lpfnWndProc:unsafe extern "system" fn(WindowHandle,u32,usize,isize)->isize,
    pub class_data:i32,
    pub window_extra_data:i32,
    pub instance:Option<InstanceHandle>,
    pub icon:Option<IconHandle>,
    pub cursor:Option<CursorHandle>,
    pub background:Option<BrushHandle>,
    pub menu_name:*const u16,
    pub class_name:*const u16,
    pub small_icon:Option<IconHandle>,
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
    pub const fn new()->WindowClass{
        Self
    }

    /// Registers a window class for subsequent use in calls to the `Window::create` function.
    /// 
    /// If the function succeeds,
    /// the return value is a class atom that uniquely identifies the class being registered.
    /// If the function fails, the return value is zero.
    /// To get extended error information, call `WindowsCore::get_last_error`.
    pub fn register(
        &self,
        class_name:*const u16,
        styles:WindowClassStyles,
        window_procedure:unsafe extern "system" fn(WindowHandle,u32,usize,isize)->isize,
        class_data:i32,
        window_data:i32,
        instance:Option<InstanceHandle>,
        window_icon:Option<IconHandle>,
        small_window_icon:Option<IconHandle>,
        cursor:Option<CursorHandle>,
        background:Option<BrushHandle>,
        menu_name:*const u16,
    )->Option<ClassAtom>{
        unsafe{
            let class_attributes=WNDCLASSEXW{
                cbSize:size_of::<WNDCLASSEXW>() as u32,
                style:styles.flag,
                lpfnWndProc:transmute(window_procedure),
                cbClsExtra:class_data,
                cbWndExtra:window_data,
                hInstance:InstanceHandle::to_raw(instance),
                hIcon:IconHandle::to_raw(window_icon),
                hCursor:CursorHandle::to_raw(cursor),
                hbrBackground:BrushHandle::to_raw(background),
                lpszMenuName:menu_name,
                lpszClassName:class_name,
                hIconSm:IconHandle::to_raw(small_window_icon),
            };
            ClassAtom::from_raw(RegisterClassExW(&class_attributes))
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
    pub fn unregister(&self,class:ClassIdentifier,instance:Option<InstanceHandle>)->bool{
        unsafe{
            UnregisterClassW(class.identifier as *const _,InstanceHandle::to_raw(instance))!=0
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
        &self,
        class:ClassIdentifier,
        instance:Option<InstanceHandle>,
        info:&mut WindowClassInfo
    )->bool{
        unsafe{
            GetClassInfoExW(
                InstanceHandle::to_raw(instance),
                class.identifier as *const _,
                transmute(info)
            )!=0
        }
    }
}