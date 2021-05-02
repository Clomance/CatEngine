use crate::windows::WinError;

use super::{
    Icon,
    Bitmap,
};

use winapi::{
    shared::{
        minwindef::{
            LPARAM,
            UINT,
            WPARAM,
            LRESULT,
        },
        windef::{
            HWND,
            HBITMAP,
        }
    },

    um::{
        winuser::{
            RegisterClassExW,
            UnregisterClassW,
            LoadCursorFromFileW,
            WNDCLASSEXW,
            PostQuitMessage,
            DefWindowProcW,
            CreateIconIndirect,
            ICONINFO,
            DestroyWindow,
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
            CreateBitmap,
            DeleteObject,
        },
        //errhandlingapi::{GetLastError},
    }
};

use image::{
    RgbaImage,
    RgbImage,
    ImageBuffer,
    Bgra
};

use gl::{
    Viewport,
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

pub enum CursorIcon{
    None,
    BGRA8{
        position:[u32;2],
        image:ImageBuffer<Bgra<u8>,Vec<u8>>,
    }
}


pub enum Background{
    None,
    Colour([u8;3]),
    // #[cfg(feature="window_background_image")]
    // BGRA8Image(ImageBuffer<Bgra<u8>,Vec<u8>>),
}


pub struct WindowClass{
    name:Vec<u16>,
    // #[cfg(feature="window_background_image")]
    // background_image:Option<Bitmap>,
}

impl WindowClass{
    pub fn new(attributes:WindowClassAttributes)->Result<WindowClass,WinError>{
        let class_name:Vec<u16>=attributes.name
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect();

        let window_icon=match attributes.window_icon{
            None=>null_mut(),
            Some(image)=>Icon::from_bgra(true,[0u32;2],&image).handle()
        };

        let cursor=match attributes.cursor_icon{
            CursorIcon::None=>null_mut(),
            CursorIcon::BGRA8{
                position,
                image
            }=>Icon::from_bgra(false,position,&image).handle()
        };


        // #[cfg(feature="window_background_image")]
        // let mut background_image=None;

        let background=match attributes.background{
            Background::None=>null_mut(),
            Background::Colour([red,green,blue])=>unsafe{
                CreateSolidBrush(RGB(red,green,blue))
            }
            // #[cfg(feature="window_background_image")]
            // Background::BGRA8Image(image)=>unsafe{
            //     let bitmap=Bitmap::from_bgra(&image);
            //     let brush=CreatePatternBrush(bitmap.handle());
            //     background_image=Some(bitmap);
            //     brush
            // }
        };

        // to create opengl context
        let mut style=CS_OWNDC;

        if attributes.no_close{
            style|=CS_NOCLOSE
        }
        if attributes.drop_shadow{
            style|=CS_DROPSHADOW
        }

        if attributes.double_clicks{
            style|=CS_DBLCLKS
        }

        let class_attributes=WNDCLASSEXW{
            cbSize:std::mem::size_of::<WNDCLASSEXW>() as UINT,
            style,
            lpfnWndProc:Some(DefWindowProcW),
            cbClsExtra:0,
            cbWndExtra:0,
            hInstance:null_mut(),
            hIcon:window_icon,
            hCursor:cursor,
            hbrBackground:background,
            lpszMenuName:null_mut(),
            lpszClassName:class_name.as_ptr(),
            hIconSm:null_mut(),
        };

        let class=unsafe{RegisterClassExW(&class_attributes)};

        if class==0{
            Err(WinError::get_last_error())
        }
        else{
            Ok(Self{
                name:class_name,
                // #[cfg(feature="window_background_image")]
                // background_image,
            })
        }
    }

    pub fn as_ptr(&self)->*const u16{
        self.name.as_ptr()
    }

    pub fn unregister(&self){
        unsafe{
            let _result=UnregisterClassW(self.name.as_ptr(),null_mut());
            // #[cfg(feature="window_background_image")]
            // if let Some(background_image)=self.background_image.take(){
            //     background_image.destroy()
            // }
        }
    }
}

impl Drop for WindowClass{
    fn drop(&mut self){
        unsafe{
            let _result=UnregisterClassW(self.name.as_ptr(),null_mut());
            // #[cfg(feature="window_background_image")]
            // if let Some(background_image)=self.background_image.take(){
            //     background_image.destroy()
            // }
        }
    }
}

pub struct WindowClassAttributes{
    /// The name of a class.
    pub name:OsString,

    /// The window icon.
    /// 
    /// `None` means the system default window icon.
    /// 
    /// The default is `None`.
    pub window_icon:Option<ImageBuffer<Bgra<u8>,Vec<u8>>>,

    /// The window cursor icon.
    /// 
    /// `None` means the system default cursor icon.
    /// 
    /// The default is `None`.
    pub cursor_icon:CursorIcon,

    /// A colour or an image used for painting the background.
    /// 
    /// If `None` is set, the system default background is used.
    /// 
    /// The default is `None`.
    pub background:Background,

    /// Disables Close on the window menu.
    /// 
    /// The default is `false`.
    pub no_close:bool,

    /// Enables the drop shadow effect on a window.
    /// The effect is turned on and off through SPI_SETDROPSHADOW.
    /// Typically, this is enabled for small,
    /// short-lived windows such as menus to emphasize their Z-order relationship to other windows.
    /// Windows created from a class with this style must be top-level windows;
    /// they may not be child windows.
    /// 
    /// The default is `false`.
    pub drop_shadow:bool,

    /// Sends a double-click message to the window procedure
    /// when the user double-clicks the mouse
    /// while the cursor is within a window.
    /// 
    /// The default is `false`.
    pub double_clicks:bool,
}

impl WindowClassAttributes{
    pub fn new(name:&str)->WindowClassAttributes{
        let name=OsString::from(name);

        Self{
            name,
            window_icon:None,
            cursor_icon:CursorIcon::None,
            background:Background::None,
            no_close:false,
            drop_shadow:false,
            double_clicks:false,
        }
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