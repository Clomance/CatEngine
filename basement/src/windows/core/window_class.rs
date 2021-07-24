use crate::windows::{
    WinCore,
    WinError,
};

use winapi::{
    shared::{
        minwindef::{
            LPARAM,
            UINT,
            WPARAM,
            LRESULT,
            HINSTANCE,
        },
        windef::{
            HWND,
            HBITMAP,
            HICON,
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

        wingdi::{
            CreateSolidBrush,
            CreatePatternBrush,
            RGB,
            DeleteObject,
        },
    }
};

use std::{
    ptr::{null_mut},
    ffi::{
        CString,
        OsString,
    },
    os::windows::ffi::OsStrExt,
    mem::transmute,
};

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
    pub unsafe fn register(
        &self,
        class_name:*const u16,
        style:u32,
        window_procedire:Option<unsafe extern "system" fn(HWND,u32,usize,isize)->isize>,
        class_extra_data:i32,
        window_extra_data:i32,
        instance:HINSTANCE,
        window_icon:HICON,
        small_window_icon:HICON,
        cursor:HICON,
        background:HBRUSH,
        menu_name:*const u16,
    )->u16{
        let class_attributes=WNDCLASSEXW{
            cbSize:std::mem::size_of::<WNDCLASSEXW>() as u32,
            style,
            lpfnWndProc:window_procedire,
            cbClsExtra:class_extra_data,
            cbWndExtra:window_extra_data,
            hInstance:instance,
            hIcon:window_icon,
            hCursor:cursor,
            hbrBackground:background,
            lpszMenuName:menu_name,
            lpszClassName:class_name,
            hIconSm:small_window_icon,
        };

        RegisterClassExW(&class_attributes)
    }

    /// Unregisters a window class, freeing the memory required for the class.
    /// 
    /// `name` is a null-terminated string or a class atom.
    /// If `name` is a string, it specifies the window class name.
    /// The atom must be in the low-order word of `name`; the high-order word must be zero.
    /// 
    /// If the function succeeds, the return value is `true`.
    /// If the class could not be found or if a window still exists that was created with the class,
    /// the return value is `false`.
    /// To get extended error information, call `WindowsCore::get_last_error`.
    pub unsafe fn unregister(&self,name:*const u16,instance:HINSTANCE)->bool{
        UnregisterClassW(name,instance)!=0
    }

    /// Retrieves information about a window class,
    /// including a handle to the small icon associated with the window class.
    /// The function does not retrieve a handle to the small icon.
    /// 
    /// `name` is a null-terminated string or a class atom.
    /// If `name` is a string, it specifies the window class name.
    /// The atom must be in the low-order word of `name`; the high-order word must be zero.
    /// 
    /// If the function finds a matching class and successfully copies the data,
    /// the return value is `true`.
    /// If the function does not find a matching class and successfully copy the data,
    /// the return value is `flase`.
    /// To get extended error information, call `WindowsCore::get_last_error`..
    pub unsafe fn get_info(&self,name:*const u16,instance:HINSTANCE,info:&mut WNDCLASSEXW)->bool{
        GetClassInfoExW(instance,name,info)!=0
    }
}


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