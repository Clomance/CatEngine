use crate::windows::{
    WinCore,
    WinError,
};

pub use crate::windows::core::window_class::{
    ClassIdentifier,
    ClassAtom,
    WindowClassStyle,
    WindowClassStyles,
};

use super::{
    Icon,
    default_window_procedure,
};

use winapi::{
    um::{
        winuser::{
            LoadCursorW,
            // Cursors
            IDC_ARROW,
        },

        wingdi::{
            CreateSolidBrush,
            RGB,
        },
    }
};

use image::{
    ImageBuffer,
    Bgra
};

use std::{
    ptr::{null_mut},
    ffi::OsString,
    os::windows::ffi::OsStrExt,
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
}

/// A window class.
pub struct WindowClass{
    identifier:ClassAtom, // a class atom
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
            CursorIcon::None=>unsafe{
                LoadCursorW(null_mut(),IDC_ARROW)
            },
            CursorIcon::BGRA8{
                position,
                image
            }=>Icon::from_bgra(false,position,&image).handle()
        };

        let background=match attributes.background{
            Background::None=>null_mut(),
            Background::Colour([red,green,blue])=>unsafe{
                CreateSolidBrush(RGB(red,green,blue))
            }
        };


        let mut style=WindowClassStyles::new()
            // to create opengl context
            .set(WindowClassStyle::OwnDeviceContext);

        if attributes.no_close{
            style=style.set(WindowClassStyle::NoClose)
        }
        if attributes.drop_shadow{
            style=style.set(WindowClassStyle::DropShadow)
        }
        if attributes.double_clicks{
            style=style.set(WindowClassStyle::DoubleClicks)
        }

        if let Some(class)=unsafe{
            WinCore.window_class.register(
                class_name.as_ptr(),
                style,
                Some(default_window_procedure),
                0,
                64,
                null_mut(),
                window_icon,
                null_mut(),
                cursor,
                background,
                null_mut(),
            )
        }{
            Ok(Self{
                identifier:class,
            })
        }
        else{
            Err(WinError::get_last_error())
        }
    }

    pub const fn atom(&self)->ClassAtom{
        self.identifier
    }

    #[inline(always)]
    pub fn identifier(&self)->ClassIdentifier{
        ClassIdentifier::atom(self.identifier)
    }
}

impl Drop for WindowClass{
    fn drop(&mut self){
        unsafe{
            let _result=WinCore.window_class.unregister(self.identifier(),null_mut());
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
