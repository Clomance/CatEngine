use super::{
    Colour,
    bitmap::BitmapHandle,
};

use winapi::{
    shared::windef::HBRUSH,

    um::wingdi::{
        DIB_PAL_COLORS,
        DIB_RGB_COLORS,
        BS_DIBPATTERN,
        BS_DIBPATTERN8X8,
        BS_DIBPATTERNPT,
        BS_HATCHED,
        BS_HOLLOW,
        BS_PATTERN,
        BS_PATTERN8X8,
        BS_SOLID,

        HS_BDIAGONAL,
        HS_CROSS,
        HS_DIAGCROSS,
        HS_FDIAGONAL,
        HS_HORIZONTAL,
        HS_VERTICAL,

        CreateBrushIndirect,
        CreateDIBPatternBrushPt,
        CreateHatchBrush,
        CreatePatternBrush,
        CreateSolidBrush,

        DeleteObject,
    },
    um::winuser::GetSysColorBrush,
};

use core::{
    mem::{
        transmute,
        transmute_copy,
    },
    ptr::NonNull,
};

#[derive(Clone,Copy)]
#[repr(transparent)]
pub struct BrushHandle{
    inner:NonNull<()>,
}
implement_handle_wrapper!(BrushHandle,HBRUSH);

#[derive(Clone,Copy,Debug)]
#[repr(u32)]
pub enum BrushStyle{
    /// A pattern brush defined by a device-independent bitmap (DIB) specification.
    /// If lbStyle is BS_DIBPATTERN, the lbHatch member contains a handle to a packed DIB.
    /// For more information, see discussion in lbHatch.
    DIBPattern=BS_DIBPATTERN,

    /// See BS_DIBPATTERN.
    DIBPattern8X8=BS_DIBPATTERN8X8,

    /// A pattern brush defined by a device-independent bitmap (DIB) specification.
    /// If lbStyle is BS_DIBPATTERNPT, the lbHatch member contains a pointer to a packed DIB.
    /// For more information, see discussion in lbHatch.
    DIBPatternPT=BS_DIBPATTERNPT,

    /// Hatched brush.
    Hatched=BS_HATCHED,

    /// Hollow brush.
    Hollow=BS_HOLLOW,

    /// Pattern brush defined by a memory bitmap.
    Pattern=BS_PATTERN,

    /// See BS_PATTERN.
    Pattern8X8=BS_PATTERN8X8,

    /// Solid brush.
    Solid=BS_SOLID,
}

#[derive(Clone,Copy,Debug)]
#[repr(u32)]
pub enum ColourFormat{
    /// The color table should consist of an array of 16-bit indexes into the current logical palette.
    Palette=DIB_PAL_COLORS,

    /// The color table should consist of literal red, green, blue (RGB) values.
    RGB=DIB_RGB_COLORS,
}

#[derive(Clone,Copy,Debug)]
#[repr(u32)]
pub enum Hatch{
    /// A 45-degree upward, left-to-right hatch.
    BackDiagonal=HS_BDIAGONAL,

    /// Horizontal and vertical cross-hatch.
    Cross=HS_CROSS,

    /// 45-degree crosshatch.
    DiagonalCross=HS_DIAGCROSS,

    /// A 45-degree downward, left-to-right hatch
    ForwardDiagonal=HS_FDIAGONAL,

    /// Horizontal hatch.
    Horizontal=HS_HORIZONTAL,

    /// Vertical hatch.
    Vertical=HS_VERTICAL
}

/// Defines the style, colour, and pattern of a physical brush.
#[derive(Clone,Copy)]
#[repr(C)]
pub struct BrushLog{
    /// The brush style.
    style:BrushStyle,

    /// The color in which the brush is to be drawn.
    /// If lbStyle is the BS_HOLLOW or BS_PATTERN style, lbColor is ignored.
    /// 
    /// If lbStyle is BS_DIBPATTERN or BS_DIBPATTERNPT,
    /// the low-order word of lbColor specifies
    /// whether the bmiColors members of the BITMAPINFO structure contain
    /// explicit red, green, blue (RGB) values or indexes into the currently realized logical palette.
    /// The lbColor member must be one of the following values.
    /// 
    /// If lbStyle is BS_HATCHED or BS_SOLID, lbColor is a COLORREF color value.
    /// To create a COLORREF color value, use the RGB macro.
    /// 
    /// Although lbColor controls the foreground color of a hatch brush,
    /// the SetBkMode and SetBkColor functions control the background color.
    colour:Colour,

    /// A hatch style.
    /// The meaning depends on the brush style defined by lbStyle.
    /// 
    /// If lbStyle is BS_DIBPATTERN,
    /// the lbHatch member contains a handle to a packed DIB.
    /// To obtain this handle, an application calls the GlobalAlloc function with GMEM_MOVEABLE (or LocalAlloc with LMEM_MOVEABLE)
    /// to allocate a block of memory and then fills the memory with the packed DIB.
    /// A packed DIB consists of a BITMAPINFO structure immediately followed by the array of bytes that define the pixels of the bitmap.
    /// 
    /// If lbStyle is BS_DIBPATTERNPT,
    /// the lbHatch member contains a pointer to a packed DIB.
    /// The pointer derives from the memory block created by LocalAlloc with LMEM_FIXED set or by GlobalAlloc with GMEM_FIXED set,
    /// or it is the pointer returned by a call like LocalLock (handle_to_the_dib).
    /// A packed DIB consists of a BITMAPINFO structure immediately followed by the array of bytes that define the pixels of the bitmap.
    /// 
    /// If `style` is BS_HATCHED,
    /// the lbHatch member specifies the orientation of the lines used to create the hatch.
    /// It can be one of the following values.
    /// 
    /// If `style` is BS_PATTERN,
    /// lbHatch is a handle to the bitmap that defines the pattern.
    /// The bitmap cannot be a DIB section bitmap, which is created by the CreateDIBSection function.
    /// 
    /// If `style` is BS_SOLID or BS_HOLLOW, `hatch` is ignored.
    hatch:usize,
}

impl BrushLog{
    pub const fn hollow()->BrushLog{
        Self{
            style:BrushStyle::Hollow,
            colour:Colour::new([0u8;3]),
            hatch:0,
        }
    }

    #[inline(always)]
    pub fn solid(colour:Colour)->BrushLog{
        Self{
            style:BrushStyle::Solid,
            colour:colour,
            hatch:0,
        }
    }

    #[inline]
    pub fn hatched(colour:Colour,hatch:Hatch)->BrushLog{
        Self{
            style:BrushStyle::Solid,
            colour:colour,
            hatch:hatch as usize,
        }
    }

    #[inline]
    pub fn pattern(bitmap:BitmapHandle)->BrushLog{
        Self{
            style:BrushStyle::Pattern,
            colour:Colour::new([0u8;3]),
            hatch:unsafe{transmute(bitmap)},
        }
    }

    // #[inline]
    // pub fn device_dependent_pattern(colour_format:ColourFormat,bitmap:BitmapHandle)->BrushLog{
    //     Self{

    //     }
    // }
}

/// A brush is a bitmap that the system uses to paint the interiors of filled shapes.
/// 
/// After an application creates a brush,
/// it can select it into any device context
/// by calling the `DeviceContext::select_brush` function.
/// 
/// ICM: No colour is done at brush creation.
/// However, colour management is performed
/// when the brush is selected into an ICM-enabled device context.
/// 
/// When you no longer need the created brush, call the `Brush::destroy` function to delete it.
pub struct Brush;

impl Brush{
    pub const fn new()->Brush{
        Self
    }
}

impl Brush{
    /// Creates a logical brush that has the specified style, colour, and pattern.
    /// 
    /// A brush created by using a monochrome bitmap (one color plane, one bit per pixel)
    /// is drawn using the current text and background colors.
    /// Pixels represented by a bit set to 0 are drawn with the current text color;
    /// pixels represented by a bit set to 1 are drawn with the current background color.
    /// 
    /// If the function succeeds, the return value identifies a logical brush.
    /// 
    /// If the function fails, the return value is `None`.
    pub fn create_indirect(&self,brush_log:&BrushLog)->Option<BrushHandle>{
        unsafe{
            BrushHandle::from_raw(CreateBrushIndirect(transmute(brush_log)))
        }
    }

    /// Creates a logical brush that has the pattern specified
    /// by the specified device-independent bitmap (DIB).
    /// The brush can subsequently be selected into any device context
    /// that is associated with a device that supports raster operations.
    /// 
    /// `packed_bitmap` - A pointer to a packed DIB consisting of a `BITMAPINFO` structur
    /// immediately followed by an array of bytes defining the pixels of the bitmap.
    /// 
    /// `colour_format` - Specifies whether the `bmiColors` member of the `BITMAPINFO` structure contains a valid colour table and,
    /// if so, whether the entries in this color table contain explicit red, green, blue (RGB) values or palette indexes.
    /// 
    /// If the function succeeds, the return value identifies a logical brush.
    /// 
    /// If the function fails, the return value is `None`.
    pub fn create_dib_pattern(&self,packed_bitmap:*const (),colour_format:ColourFormat)->Option<BrushHandle>{
        unsafe{
            BrushHandle::from_raw(CreateDIBPatternBrushPt(packed_bitmap as *const _,colour_format as u32))
        }
    }

    /// Creates a logical brush that has the specified hatch pattern and color.
    /// 
    /// The foreground color of the brush that is used for the hatches.
    /// To create a `COLORREF` color value, use the RGB macro.
    /// 
    /// It can call `SetBkMode` to affect the rendering of the brush.
    /// 
    /// If an application uses a hatch brush to fill the backgrounds of both a parent and a child window with matching color,
    /// you must set the brush origin before painting the background of the child window.
    /// You can do this by calling the `SetBrushOrgEx` function.
    /// Your application can retrieve the current brush origin by calling the `GetBrushOrgEx` function.
    /// 
    /// If the function succeeds, the return value identifies a logical brush.
    /// 
    /// If the function fails, the return value is `None`.
    pub fn create_hatch(&self,hatch:Hatch,colour:Colour)->Option<BrushHandle>{
        unsafe{
            BrushHandle::from_raw(CreateHatchBrush(transmute(hatch),colour.as_raw()))
        }
    }

    /// Creates a logical brush with the specified bitmap pattern.
    /// The bitmap can be a DIB section bitmap,
    /// which is created by the `CreateDIBSection` function, or it can be a device-dependent bitmap.
    /// 
    /// A pattern brush is a bitmap that the system uses to paint the interiors of filled shapes.
    /// 
    /// You can delete a pattern brush without affecting the associated bitmap.
    /// Therefore, you can then use this bitmap to create any number of pattern brushes.
    /// 
    /// A brush created by using a monochrome (1 bit per pixel) bitmap has the text and background colours of the device context to which it is drawn.
    /// Pixels represented by a 0 bit are drawn with the current text color;
    /// pixels represented by a 1 bit are drawn with the current background color.
    /// 
    /// If the function succeeds, the return value identifies a logical brush.
    /// 
    /// If the function fails, the return value is `None`.
    pub fn create_pattern(&self,bitmap:BitmapHandle)->Option<BrushHandle>{
        unsafe{
            BrushHandle::from_raw(CreatePatternBrush(bitmap.as_raw()))
        }
    }

    /// Creates a logical brush that has the specified solid colour.
    /// 
    /// A solid brush is a bitmap that the system uses to paint the interiors of filled shapes.
    /// 
    /// To paint with a system color brush, an application should use `Brush::get_system(nIndex)`
    /// instead of `Brush::create_solid(GetSysColor(nIndex))`,
    /// because `Brush::get_system` returns a cached brush instead of allocating a new one.
    /// 
    /// If the function succeeds, the return value identifies a logical brush.
    /// 
    /// If the function fails, the return value is `None`.
    pub fn create_solid(&self,colour:Colour)->Option<BrushHandle>{
        unsafe{
            BrushHandle::from_raw(CreateSolidBrush(colour.as_raw()))
        }
    }

    /// Retrieves a handle identifying a logical brush that corresponds to the specified color index.
    /// 
    /// `index` - A colour index.
    /// This value corresponds to the color used to paint one of the window elements.
    /// See `GetSysColor` for system colour index values.
    /// 
    /// An application can retrieve the current system colours by calling the `GetSysColor` function.
    /// An application can set the current system colours by calling the `SetSysColors` function.
    /// 
    /// An application must not register a window class for a window using a system brush.
    /// To register a window class with a system colour,
    /// see the documentation of the `hbrBackground` member of the `WNDCLASS` or `WNDCLASSEX` structures.
    /// 
    /// System color brushes track changes in system colours.
    /// In other words, when the user changes a system colour, the associated system colour brush automatically changes to the new colour.
    /// 
    /// To paint with a system color brush, an application should use `Brush::get_system` (nIndex) instead of `Brush::create_solid` ( GetSysColor (nIndex)),
    /// because `Brush::get_system` returns a cached brush instead of allocating a new one.
    /// 
    /// System colour brushes are owned by the system so you don't need to destroy them.
    /// Although you don't need to delete the logical brush that `Brush::get_system` returns,
    /// no harm occurs by calling `Brush::destroy`.
    /// 
    /// The return value identifies a logical brush if the nIndex parameter is supported by the current platform.
    /// Otherwise, it returns `None`.
    pub fn get_system(&self,index:i32)->Option<BrushHandle>{
        unsafe{
            BrushHandle::from_raw(GetSysColorBrush(index))
        }
    }

    /// Deletes a logical brush, freeing all system resources associated with the object.
    /// After the object is deleted, the specified handle is no longer valid.
    /// 
    /// Do not delete a brush while it is still selected into a DC.
    /// 
    /// When a pattern brush is deleted, the bitmap associated with the brush is not deleted.
    /// The bitmap must be deleted independently.
    /// 
    /// If the function succeeds, the return value is `true`.
    /// 
    /// If the specified handle is not valid or is currently selected into a DC, the return value is `false`.
    pub fn destroy(&self,brush:BrushHandle)->bool{
        unsafe{
            DeleteObject(transmute(brush))!=0
        }
    }
}