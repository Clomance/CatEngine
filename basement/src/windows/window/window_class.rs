use crate::windows::{
    WinCore,
    WinError,
    WinColour,
    // level0::brush::Brush,
};

pub use crate::windows::core::{
    window_class::{
        ClassIdentifier,
        ClassAtom,
        WindowClassStyle,
        WindowClassStyles,
        WindowBackgroundColour,
        WindowBackgroundSystemColour,
        WindowClassInfo,
    },
    cursor::SystemCursor,
};

use super::{
    Icon,
    setup_window_procedure,
};

use image::{
    ImageBuffer,
    Bgra
};

use std::{
    ptr::null_mut,
    mem::transmute
};

pub enum CursorIcon{
    None,
    System(SystemCursor),
    BGRA8{
        position:[u32;2],
        image:ImageBuffer<Bgra<u8>,Vec<u8>>,
    }
}

pub enum Background{
    None,
    SystemColour(WindowBackgroundSystemColour),
    RGBColour([u8;3]),
}

/// A window class.
/// 
/// Unregistrered when goes out of scope.
pub struct WindowClass{
    identifier:ClassAtom,
}

impl WindowClass{
    /// Registers a window class for subsequent use in creating windows.
    pub fn new(attributes:WindowClassAttributes)->Result<WindowClass,WinError>{
        let class_name:Vec<u16>=attributes.name
            .encode_utf16()
            .chain([0].into_iter())
            .collect();

        let window_icon=match attributes.window_icon{
            None=>None,
            Some(image)=>Some(Icon::from_bgra(true,[0u32;2],&image).handle())
        };

        let cursor=match attributes.cursor_icon{
            CursorIcon::None=>None,
            CursorIcon::System(cursor)=>unsafe{
                Some(WinCore.cursor.load_system_cursor(cursor).unwrap())
            }
            CursorIcon::BGRA8{position,image}=>unsafe{
                Some(transmute(Icon::from_bgra(false,position,&image).handle()))
            }
        };

        let background=match attributes.background{
            Background::None=>None,
            Background::SystemColour(colour)=>{
                Some(WindowBackgroundColour::system_colour(colour))
            }
            Background::RGBColour([red,green,blue])=>unsafe{
                let brush=WinCore.brush.create_solid(WinColour::new([red,green,blue])).unwrap();
                Some(WindowBackgroundColour::brush(brush))
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
                setup_window_procedure,
                0,
                64,
                None,
                window_icon,
                None,
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

    // Returns the class atom.
    pub const fn atom(&self)->ClassAtom{
        self.identifier
    }

    /// Returns the class atom wrapped into `ClassIdentifier`.
    #[inline(always)]
    pub fn identifier(&self)->ClassIdentifier{
        ClassIdentifier::atom(self.identifier)
    }

    /// Retrieves information about a window class,
    /// including a handle to the small icon associated with the window class.
    /// The function does not retrieve a handle to the small icon.
    pub fn get_info(&self)->WindowClassInfo{
        unsafe{
            let mut info=WindowClassInfo::new();
            WinCore.window_class.get_info(self.identifier(),None,&mut info);
            info
        }
    }
}

impl Drop for WindowClass{
    fn drop(&mut self){
        unsafe{
            let _result=WinCore.window_class.unregister(self.identifier(),None);
        }
    }
}

pub struct WindowClassAttributes{
    /// A name of a class.
    pub name:String,

    /// A window icon.
    /// 
    /// The default is `None`.
    pub window_icon:Option<ImageBuffer<Bgra<u8>,Vec<u8>>>,

    /// A window cursor icon.
    /// 
    /// The default is `CursorIcon::System(SystemCursor::Arrow)`.
    pub cursor_icon:CursorIcon,

    /// A colour or image used for painting the background.
    /// 
    /// The default is `None`.
    pub background:Background,

    /// Disables Close on the window menu.
    /// 
    /// The default is `false`.
    pub no_close:bool,

    /// Enables the drop shadow effect on a window.
    /// The effect is turned on and off through `SPI_SETDROPSHADOW`.
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
        let name=String::from(name);

        Self{
            name,
            window_icon:None,
            cursor_icon:CursorIcon::System(SystemCursor::Arrow),
            background:Background::None,
            no_close:false,
            drop_shadow:false,
            double_clicks:false,
        }
    }
}
