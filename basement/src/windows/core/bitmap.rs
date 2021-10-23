use super::{
    device_context::DeviceContextHandle,
};

use core::{
    mem::{
        transmute,
        transmute_copy,
        size_of,
    },
    ptr::NonNull,
};

use winapi::{
    shared::windef::HBITMAP,

    um::wingdi::{
        // structs
        BITMAP,
        BITMAPINFO,
        BITMAPINFOHEADER,
        // functions
        CreateBitmap,
        CreateBitmapIndirect,
        CreateCompatibleBitmap,
        CreateDIBitmap,
        DeleteObject,
        GetObjectW,
        GetBitmapBits,
        GetDIBits,
        SetDIBits,
        // formats of the colors
        DIB_PAL_COLORS,
        DIB_RGB_COLORS,
    },
};

#[derive(Clone,Copy,Debug)]
#[repr(u32)]
pub enum ColourFormat{
    /// The colour table should consist of an array of 16-bit indexes into the current logical palette.
    Palette=DIB_PAL_COLORS,

    /// The colour table should consist of literal red, green, blue (RGB) values.
    RGB=DIB_RGB_COLORS,
}

/// The `BitmapData` structure defines the width,
/// height, colour format, and bit values of a bitmap.
#[derive(Clone,Copy,Debug)]
#[repr(C)]
pub struct BitmapData{
    object_type:i32,

    /// The width, in pixels, of the bitmap.
    /// The width must be greater than zero.
    pub width:i32,

    /// The height, in pixels, of the bitmap. The height must be greater than zero.
    pub height:i32,

    /// The number of bytes in each scan line.
    /// This value must be divisible by 2,
    /// because the system assumes
    /// that the bit values of a bitmap form an array
    /// that is word aligned.
    pub width_bytes:i32,

    /// The count of colour planes.
    pub planes:u16,

    /// The number of bits required to indicate the colour of a pixel.
    pub pixel_bits:u16,

    /// A pointer to the location of the bit values for the bitmap.
    /// The `bits` member must be a pointer to an array of character (1-byte) values.
    pub bits:*mut u8,
}

impl BitmapData{
    pub const fn new()->BitmapData{
        Self{
            object_type:0,
            width:0i32,
            height:0i32,
            width_bytes:0i32,
            planes:0u16,
            pixel_bits:0u16,
            bits:0 as *mut u8,
        }
    }

    pub const fn size(mut self,size:[i32;2])->BitmapData{
        self.width=size[0];
        self.height=size[1];
        self
    }

    pub const fn width_bytes(mut self,width:i32)->BitmapData{
        self.width_bytes=width;
        self
    }

    pub const fn planes(mut self,planes:u16)->BitmapData{
        self.planes=planes;
        self
    }

    pub const fn pixel_bits(mut self,bits:u16)->BitmapData{
        self.pixel_bits=bits;
        self
    }

    pub const fn bits(mut self,bits:*mut u8)->BitmapData{
        self.bits=bits;
        self
    }
}

// #[repr(C)]
// pub struct BitmapInfoHeader{
//     biSize:u32,
//     pub width:i32,
//     pub height:i32,
//     pub planes:u16,
//     pub BitCount:u16,
//     pub Compression:u32,
//     pub image_size:u32,
//     pub XPelsPerMeter:i32,
//     pub YPelsPerMeter:i32,
//     pub ClrUsed:u32,
//     pub ClrImportant:u32,
// }

/// The replacement for `HBITMAP`.
/// Can be wraped with `Option` with null pointer optimization.
#[derive(Clone,Copy)]
#[repr(transparent)]
pub struct BitmapHandle{
    inner:NonNull<HBITMAP>,
}
implement_handle_wrapper!(BitmapHandle,HBITMAP);

// pub struct BitmapInfo{
//     pub inner:BITMAPINFO,
// }

pub struct Bitmap;

impl Bitmap{
    pub const fn new()->Bitmap{
        Self
    }
}

impl Bitmap{
    /// Creates a bitmap with the specified width, height, and colour format (colour planes and bits-per-pixel).
    /// Creates a device-dependent bitmap.
    /// 
    /// After a bitmap is created, it can be selected into a device context by calling the `SelectObject` function.
    /// However, the bitmap can only be selected into a device context if the bitmap and the DC have the same format.
    /// 
    /// The `Bitmap::create` function can be used to create colour bitmaps.
    /// However, for performance reasons applications should use `Bitmap::create`
    /// to create monochrome bitmaps and `CreateCompatibleBitmap` to create colour bitmaps.
    /// Whenever a colour bitmap returned from `Bitmap::create` is selected into a device context,
    /// the system checks that the bitmap matches the format of the device context it is being selected into.
    /// Because `CreateCompatibleBitmap` takes a device context,
    /// it returns a bitmap that has the same format as the specified device context.
    /// Thus, subsequent calls to `SelectObject` are faster with a colour bitmap
    /// from `CreateCompatibleBitmap` than with a colour bitmap returned from `Bitmap::create`.
    /// 
    /// If the bitmap is monochrome, zeros represent the foreground colour
    /// and ones represent the background colour for the destination device context.
    /// 
    /// If an application sets the `size[0]` or `size[1]` parameters to zero,
    /// `Bitmap::create` returns the handle to a 1-by-1 pixel, monochrome bitmap.
    /// 
    /// When you no longer need the bitmap, call the `Bitmap::delete` function to delete it.
    /// 
    /// If the function succeeds, the return value is a handle to a bitmap.
    /// 
    /// If the function fails, the return value is `None`.
    /// 
    /// This function can return the following value:
    /// `ERROR_INVALID_BITMAP` - The calculated size of the bitmap is less than zero.
    #[inline(always)]
    pub unsafe fn create(&self,size:[i32;2],planes:u32,colour_bits:u32,data:Option<&u8>)->Option<BitmapHandle>{
        BitmapHandle::from_raw(
            CreateBitmap(
                size[0],
                size[1],
                planes,
                colour_bits,
                transmute(data),
            )
        )
    }

    /// Creates a bitmap with the specified width, height, and colour format (colour planes and bits-per-pixel).
    /// 
    /// If an application sets the bmWidth or bmHeight members to zero,
    /// returns the handle to a 1-by-1 pixel, monochrome bitmap.
    /// 
    /// The function creates a device-dependent bitmap.
    /// 
    /// After a bitmap is created,
    /// it can be selected into a device context by calling the `SelectObject` function.
    /// However, the bitmap can only be selected into a device context
    /// if the bitmap and the DC have the same format.
    /// 
    /// While the `Bitmap::create_indirect` function can be used to create colour bitmaps,
    /// for performance reasons applications should use `Bitmap::create_indirect`
    /// to create monochrome bitmaps and `Bitmap::create_compatible` to create colour bitmaps.
    /// Whenever a colour bitmap from `Bitmap::create_indirect` is selected into a device context,
    /// the system must ensure that the bitmap matches the format of the device context it is being selected into.
    /// Because `Bitmap::create_compatible` takes a device context,
    /// it returns a bitmap that has the same format as the specified device context.
    /// Thus, subsequent calls to SelectObject are faster with a colour bitmap from `Bitmap::create_compatible`
    /// than with a colour bitmap returned from `Bitmap::create_indirect`.
    /// 
    /// If the bitmap is monochrome,zeros represent the foreground colour
    /// and ones represent the background colour for the destination device context.
    /// 
    /// When you no longer need the bitmap,
    /// call the `DeleteObject` function to delete it.
    /// 
    /// If the function succeeds, the return value is a handle to the bitmap.
    /// 
    /// If the function fails, the return value is `None`.
    /// 
    /// This function can return the following values:
    /// `ERROR_INVALID_PARAMETER` - One or more of the input parameters is invalid,
    /// `ERROR_NOT_ENOUGH_MEMORY` - The bitmap is too big for memory to be allocated.
    #[inline(always)]
    pub unsafe fn create_indirect(&self,bitmap_data:&BitmapData)->Option<BitmapHandle>{
        BitmapHandle::from_raw(
            CreateBitmapIndirect(transmute(bitmap_data))
        )
    }

    /// Creates a bitmap compatible with the device that is associated with the specified device context.
    /// 
    /// The colour format of the bitmap created by the `Bitmap::create_compatible` function
    /// matches the colour format of the device identified by the hdc parameter.
    /// This bitmap can be selected into any memory device context that is compatible with the original device.
    /// 
    /// Because memory device contexts allow both colour and monochrome bitmaps,
    /// the format of the bitmap returned by the `Bitmap::create_compatible` function differs
    /// when the specified device context is a memory device context.
    /// However, a compatible bitmap that was created for a nonmemory device context
    /// always possesses the same colour format and uses the same colour palette as the specified device context.
    /// 
    /// When a memory device context is created, it initially has a 1-by-1 monochrome bitmap selected into it.
    /// If this memory device context is used in `Bitmap::create_compatible`,
    /// the bitmap that is created is a monochrome bitmap.
    /// To create a colour bitmap,
    /// use the HDC that was used to create the memory device context,
    /// as shown in the following code:
    /// ```
    ///  HDC memDC = CreateCompatibleDC ( hDC );
    ///  HBITMAP memBM = CreateCompatibleBitmap ( hDC, nWidth, nHeight );
    ///  SelectObject ( memDC, memBM );
    /// ```
    /// 
    /// If an application sets the `size[0]` or `size[1]` parameters to zero,
    /// `Bitmap::create_compatible` returns the handle to a 1-by-1 pixel, monochrome bitmap.
    /// 
    /// If a DIB section, which is a bitmap created by the CreateDIBSection function,
    /// is selected into the device context identified by the hdc parameter,
    /// `Bitmap::create_compatible` creates a DIB section.
    /// 
    /// When you no longer need the bitmap, call the DeleteObject function to delete it.
    /// 
    /// If the function succeeds, the return value is a handle to the compatible bitmap (DDB).
    /// 
    /// If the function fails, the return value is `None`.
    #[inline(always)]
    pub unsafe fn create_compatible(
        &self,
        context:DeviceContextHandle,
        size:[i32;2]
    )->Option<BitmapHandle>{
        BitmapHandle::from_raw(
            CreateCompatibleBitmap(context.as_raw(),size[0],size[1])
        )
    }

    // /// Creates a compatible bitmap (DDB) from a DIB and, optionally, sets the bitmap bits.
    // #[inline(always)]
    // pub unsafe fn create_di(
    //     &self,
    //     context:DeviceContextHandle,
    //     bitmap_header:&BITMAPINFOHEADER,
    //     bits:Option<&u8>,
    //     bitmap_info:&BITMAPINFO,
    // )->Option<BitmapHandle>{
    //     BitmapHandle::from_raw(
    //         CreateDIBitmap(
    //             context.as_raw(),
    //             bitmap_header,
    //             0,
    //             transmute(bits),
    //             bitmap_info,
    //         )
    //     )
    // }

    /// Deletes a bitmap freeing all system resources associated with the object.
    /// After the object is deleted, the specified handle is no longer valid.
    /// 
    /// When a pattern brush is deleted, the bitmap associated with the brush is not deleted.
    /// The bitmap must be deleted independently.
    /// 
    /// If the function succeeds, the return value is `true`.
    /// 
    /// If the specified handle is not valid or is currently selected into a DC,
    /// the return value is `false`.
    #[inline(always)]
    pub fn destroy(&self,handle:BitmapHandle)->bool{
        unsafe{
            DeleteObject(transmute(handle))!=0
        }
    }
}

impl Bitmap{
    /// Retrieves the bits of the specified compatible bitmap
    /// and copies them into a buffer as a DIB using the specified format.
    /// 
    /// `bitmap` must be a compatible bitmap (DDB).
    /// 
    /// If the `bits` parameter is not `None` and the function succeeds,
    /// the return value is the number of scan lines copied from the bitmap.
    /// 
    /// If the requested format for the DIB matches its internal format,
    /// the RGB values for the bitmap are copied.
    /// If the requested format doesn't match the internal format, a colour table is synthesized.
    /// The following table describes the colour table synthesized for each format.
    /// 
    /// - 1_BPP - The colour table consists of a black and a white entry.
    /// - 4_BPP - The colour table consists of a mix of colours identical to the standard VGA palette.
    /// - 8_BPP - The colour table consists of a general mix of 256 colours defined by GDI.
    /// (Included in these 256 colours are the 20 colours found in the default logical palette.)
    /// - 24_BPP - No colour table is returned.
    /// 
    /// If the `bits` parameter is a valid pointer,
    /// the first six members of the `BITMAPINFOHEADER` structure must be initialized
    /// to specify the size and format of the DIB.
    /// The scan lines must be aligned on a DWORD except for RLE compressed bitmaps.
    /// 
    /// A bottom-up DIB is specified by setting the height to a positive number,
    /// while a top-down DIB is specified by setting the height to a negative number.
    /// The bitmap colour table will be appended to the `BITMAPINFO` structure.
    /// 
    /// If `bits` is `None`, `Bitmap::get_bits` examines the first member of the first structure pointed to by lpbi.
    /// This member must specify the size, in bytes, of a `BITMAPCOREHEADER` or a `BITMAPINFOHEADER` structure.
    /// The function uses the specified size to determine how the remaining members should be initialized.
    /// 
    /// If `bits` is `None` and the bit count member of `BITMAPINFO` is initialized to zero,
    /// `Bitmap::get_bits` fills in a `BITMAPINFOHEADER` structure or `BITMAPCOREHEADER` without the colour table.
    /// This technique can be used to query bitmap attributes.
    /// 
    /// The bitmap identified by the `bitmap` parameter must not be selected
    /// into a device context when the application calls this function.
    /// 
    /// The origin for a bottom-up DIB is the lower-left corner of the bitmap;
    /// the origin for a top-down DIB is the upper-left corner.
    /// 
    /// If the `bits` parameter is `None`
    /// and the function successfully fills the `BITMAPINFO` structure,
    /// the return value is nonzero.
    #[inline(always)]
    pub unsafe fn get_bits(
        &self,
        context:DeviceContextHandle,
        bitmap:BitmapHandle,
        bitmap_info:&mut BITMAPINFO,
        start:u32,
        lines:u32,
        bits:Option<&mut u8>,
        colour:ColourFormat
    )->bool{
        GetDIBits(
            context.as_raw(),
            bitmap.as_raw(),
            start,
            lines,
            transmute(bits),
            bitmap_info,
            colour as u32
        )!=0
    }

    /// Sets the pixels in a compatible bitmap (DDB) using the colour data found in the specified DIB.
    /// 
    /// Optimal bitmap drawing speed is obtained when the bitmap bits are indexes into the system palette.
    /// 
    /// Applications can retrieve the system palette colours and indexes
    /// by calling the `GetSystemPaletteEntries` function.
    /// After the colours and indexes are retrieved, the application can create the DIB.
    /// For more information, see System Palette.
    /// 
    /// The device context identified by the hdc parameter is used
    /// only if the `ColourFormat::Palette` constant is set for the `colour` parameter;
    /// otherwise it is ignored.
    /// 
    /// The bitmap identified by the hbmp parameter must not be selected
    /// into a device context when the application calls this function.
    /// 
    /// The scan lines must be aligned on a DWORD except for RLE-compressed bitmaps.
    /// 
    /// The origin for bottom-up DIBs is the lower-left corner of the bitmap;
    /// the origin for top-down DIBs is the upper-left corner of the bitmap.
    /// 
    /// ICM: Colour management is performed if colour management has been enabled
    /// with a call to `SetICMMode` with the `iEnableICM` parameter set to `ICM_ON`.
    /// If the bitmap specified by lpbmi has a `BITMAPV4HEADER`
    /// that specifies the gamma and endpoints members,
    /// or a `BITMAPV5HEADER` that specifies either the gamma
    /// and endpoints membersor the `profileData` and `profileSize` members,
    /// then the call treats the bitmap's pixels as being expressed
    /// in the colour space described by those members,
    /// rather than in the device context's source colour space.
    /// 
    /// If the function succeeds, the return value is the number of scan lines copied.
    /// 
    /// If the function fails, the return value is zero.
    /// 
    /// This can be the following value:
    /// `ERROR_INVALID_PARAMETER` - One or more of the input parameters is invalid.
    #[inline(always)]
    pub unsafe fn set_bits(
        &self,
        context:Option<DeviceContextHandle>,
        bitmap:BitmapHandle,
        bitmap_info:&BITMAPINFO,
        start:u32,
        lines:u32,
        bits:&u8,
        colour:ColourFormat
    )->i32{
        SetDIBits(
            DeviceContextHandle::to_raw(context),
            bitmap.as_raw(),
            start,
            lines,
            transmute(bits),
            bitmap_info,
            colour as u32
        )
    }
}

impl Bitmap{
    #[inline(always)]
    pub unsafe fn get_object_data(
        &self,
        handle:BitmapHandle,
        object_data:&mut BitmapData,
    )->bool{
        GetObjectW(
            handle.as_raw() as *mut _,
            size_of::<BitmapData>() as i32,
            transmute(object_data)
        )!=0
    }
}