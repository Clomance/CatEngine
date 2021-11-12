use super::{
    InstanceHandle,
    bitmap::BitmapHandle,
};

use core::{
    mem::{
        transmute,
        transmute_copy,
        size_of,
    },
    ptr::{
        null_mut,
        NonNull
    },
};

use winapi::{
    shared::windef::HICON,
    um::{
        winuser::{
            ICONINFO,

            CopyIcon,
            CreateIcon,
            CreateIconFromResource,
            CreateIconFromResourceEx,
            CreateIconIndirect,
        }
    }
};

#[derive(Clone,Copy)]
#[repr(transparent)]
pub struct IconHandle{
    inner:NonNull<()>,
}
implement_handle_wrapper!(IconHandle,HICON);

pub struct Icon;

impl Icon{
    pub const fn new()->Icon{
        Self
    }
}

impl Icon{
    /// Creates an icon that has the specified size, colors, and bit patterns.
    /// 
    /// `planes` - The number of planes in the XOR bitmask of the icon.
    /// 
    /// `pixel_bits` - The number of bits-per-pixel in the XOR bitmask of the icon.
    /// 
    /// `and_plane` - An array of bytes that contains the bit values for the AND bitmask of the icon.
    /// This bitmask describes a monochrome bitmap.
    /// 
    /// `xor_plane` - An array of bytes that contains the bit values for the XOR bitmask of the icon.
    /// This bitmask describes a monochrome or device-dependent color bitmap.
    /// 
    /// The nWidth and nHeight parameters must specify a width and height supported by the current display driver,
    /// because the system cannot create icons of other sizes.
    /// To determine the width and height supported by the display driver,
    /// use the `GetSystemMetrics` function, specifying the `SM_CXICON` or `SM_CYICON` value.
    /// 
    /// CreateIcon applies the following truth table to the AND and XOR bitmasks.
    /// 
    /// AND bitmask | XOR bitmask | Display
    /// 0           | 0           | Black
    /// 0           | 1           | White
    /// 1           | 0           | Screen
    /// 1           | 1           | Reverse screen
    /// 
    /// When you are finished using the icon, destroy it using the `Icon::destroy` function.
    /// 
    /// If the function succeeds, the return value is a handle to an icon.
    /// 
    /// If the function fails, the return value is `None`. To get extended error information, call `WinCore::get_last_error`.
    pub fn create(
        &self,
        instance:Option<InstanceHandle>,
        [width,height]:[i32;2],
        planes:u8,
        pixel_bits:u8,
        and_plane:*const u8,
        xor_plane:*const u8,
    )->Option<IconHandle>{
        unsafe{
            IconHandle::from_raw(
                CreateIcon(
                    InstanceHandle::to_raw(instance),
                    width,height,
                    planes,
                    pixel_bits,
                    transmute(and_plane),
                    transmute(xor_plane),
                )
            )
        }
    }

    // pub fn create_from_resource(
    //     &self,
    //     instance:Option<InstanceHandle>,
    //     [width,height]:[i32;2],
    //     planes:u8,
    //     pixel_bits:u8,
    //     and_plane:*const u8,
    //     xor_plane:*const u8,
    // )->Option<IconHandle>{
    //     unsafe{
    //         IconHandle::from_raw(
    //             CreateIconFromResource(
    //                 InstanceHandle::to_raw(instance),
    //                 width,height,
    //                 planes,
    //                 pixel_bits,
    //                 transmute(and_plane),
    //                 transmute(xor_plane),
    //             )
    //         )
    //     }
    // }

    /// Creates an icon.
    pub fn create_indirect(
        &self,
        mask:BitmapHandle,
        colour:BitmapHandle,
    )->Option<IconHandle>{
        unsafe{
            let mut icon_info=ICONINFO{
                fIcon:0,
                xHotspot:0,
                yHotspot:0,
                hbmMask:mask.as_raw(),
                hbmColor:colour.as_raw()
            };
            IconHandle::from_raw(CreateIconIndirect(&mut icon_info))
        }
    }

    /// Copies the specified icon from another module to the current module.
    /// 
    /// The CopyIcon function enables an application or DLL to get its own handle to an icon owned by another module.
    /// If the other module is freed, the application icon will still be able to use the icon.
    /// 
    /// Before closing, an application must call the `Icon::destroy` function to free any system resources associated with the icon.
    /// 
    /// If the function succeeds, the return value is a handle to the duplicate icon.
    /// 
    /// If the function fails, the return value is `None`. To get extended error information, call `WinCore::get_last_error`.
    pub fn copy(&self,icon:IconHandle)->Option<IconHandle>{
        unsafe{
            IconHandle::from_raw(CopyIcon(icon.as_raw()))
        }
    }
}