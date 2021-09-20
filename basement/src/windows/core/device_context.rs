use crate::implement_handle_wrapper;

use super::{
    bitmap::{
        BitmapHandle,
        // BitmapInfo,
    },
    window::WindowHandle,
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
    shared::windef::HDC,
    shared::windef::HBITMAP,
    um::{
        winuser::{
            ReleaseDC,
        },
        wingdi::{
            CreateCompatibleDC,
            CreateCompatibleBitmap,
            SelectObject,
            GetObjectW,
            SwapBuffers,
            ChoosePixelFormat,
            SetPixelFormat,
            BitBlt,
            PIXELFORMATDESCRIPTOR,

            // pixel buffer properties
            PFD_DRAW_TO_WINDOW,
            PFD_DRAW_TO_BITMAP,
            PFD_SUPPORT_GDI,
            PFD_SUPPORT_OPENGL,
            PFD_GENERIC_ACCELERATED,
            PFD_GENERIC_FORMAT,
            PFD_NEED_PALETTE,
            PFD_NEED_SYSTEM_PALETTE,
            PFD_DOUBLEBUFFER,
            PFD_STEREO,
            PFD_SWAP_LAYER_BUFFERS,
            PFD_DEPTH_DONTCARE,
            PFD_DOUBLEBUFFER_DONTCARE,
            PFD_STEREO_DONTCARE,
            PFD_SWAP_COPY,
            PFD_SWAP_EXCHANGE,

            // types of pixel data
            PFD_TYPE_RGBA,
            PFD_TYPE_COLORINDEX,

            // blt operations
            BLACKNESS,
            CAPTUREBLT,
            DSTINVERT,
            MERGECOPY,
            MERGEPAINT,
            NOMIRRORBITMAP,
            NOTSRCCOPY,
            NOTSRCERASE,
            PATCOPY,
            PATINVERT,
            PATPAINT,
            SRCAND,
            SRCCOPY,
            SRCERASE,
            SRCINVERT,
            SRCPAINT,
            WHITENESS,
        },
    }
};

/// A set of bit flags that specify properties of the pixel buffer.
/// The properties are generally not mutually exclusive; you can set any combination of bit flags, with the exceptions noted.
#[repr(u32)]
pub enum PixelBufferProperty{
    /// The buffer can draw to a window or device surface.
    /// 
    /// 0x00000004
    DrawToWindow=PFD_DRAW_TO_WINDOW,

    /// The buffer can draw to a memory bitmap.
    /// 
    /// 0x00000008
    DrawToDitmap=PFD_DRAW_TO_BITMAP,

    /// The buffer supports GDI drawing.
    /// This flag and `PixelBufferProperty::DoubleBuffer` are mutually exclusive in the current generic implementation.
    /// 
    /// 0x00000010
    SupportGDI=PFD_SUPPORT_GDI,

    /// The buffer supports OpenGL drawing.
    /// 
    /// 0x00000020
    SupportOpenGL=PFD_SUPPORT_OPENGL,

    /// The pixel format is supported by a device driver that accelerates the generic implementation.
    /// If this flag is clear and the `PixelBufferProperty::GenericFormat` flag is set,
    /// the pixel format is supported by the generic implementation only.
    /// 
    /// 0x00001000
    GenericAccelerated=PFD_GENERIC_ACCELERATED,

    /// The pixel format is supported by the GDI software implementation,
    /// which is also known as the generic implementation.
    /// If this bit is clear, the pixel format is supported by a device driver or hardware.
    /// 
    /// 0x00000040
    GenericFormat=PFD_GENERIC_FORMAT,

    /// The buffer uses RGBA pixels on a palette-managed device.
    /// A logical palette is required to achieve the best results for this pixel type.
    /// Colors in the palette should be specified according
    /// to the values of the cRedBits, cRedShift, cGreenBits, cGreenShift, cBluebits, and cBlueShift members.
    /// The palette should be created and realized in the device context before calling `wglMakeCurrent`.
    /// 
    /// 0x00000080
    NeedPalette=PFD_NEED_PALETTE,

    /// Defined in the pixel format descriptors of hardware that supports one hardware palette in 256-color mode only.
    /// For such systems to use hardware acceleration,
    /// the hardware palette must be in a fixed order (for example, 3-3-2)
    /// when in RGBA mode or must match the logical palette when in color-index mode.When this flag is set,
    /// you must call `SetSystemPaletteUse` in your program to force a one-to-one mapping of the logical palette and the system palette.
    /// If your OpenGL hardware supports multiple hardware palettes
    /// and the device driver can allocate spare hardware palettes for OpenGL,
    /// this flag is typically clear.
    /// 
    /// This flag is not set in the generic pixel formats.
    /// 
    /// 0x00000100
    NeedSystemPalette=PFD_NEED_SYSTEM_PALETTE,

    /// The buffer is double-buffered.
    /// This flag and `PixelBufferProperty::SupportGDI` are mutually exclusive in the current generic implementation.
    /// 
    /// 0x00000001
    DoubleBuffer=PFD_DOUBLEBUFFER,

    /// The buffer is stereoscopic.
    /// This flag is not supported in the current generic implementation.
    /// 
    /// 0x00000002
    Stereo=PFD_STEREO,

    /// Indicates whether a device can swap individual layer planes with pixel formats
    /// that include double-buffered overlay or underlay planes.
    /// Otherwise all layer planes are swapped together as a group.
    /// When this flag is set, `wglSwapLayerBuffers` is supported.
    /// 
    /// 0x00000800
    SwapLayerBuffers=PFD_SWAP_LAYER_BUFFERS,

    /// The requested pixel format can either have or not have a depth buffer.
    /// To select a pixel format without a depth buffer, you must specify this flag.
    /// The requested pixel format can be with or without a depth buffer.
    /// Otherwise, only pixel formats with a depth buffer are considered.
    /// 
    /// You can specify the following bit flags when calling `DeviceContext::choose_pixel_format`.
    /// 
    /// 0x20000000
    DepthDontCare=PFD_DEPTH_DONTCARE,

    /// The requested pixel format can be either single- or double-buffered.
    /// 
    /// You can specify the following bit flags when calling `DeviceContext::choose_pixel_format`.
    /// 
    /// 0x40000000
    DoubleBufferDontCare=PFD_DOUBLEBUFFER_DONTCARE,

    /// The requested pixel format can be either monoscopic or stereoscopic.
    /// 
    /// You can specify the following bit flags when calling `DeviceContext::choose_pixel_format`.
    /// 
    /// 0x80000000
    StereoDontCare=PFD_STEREO_DONTCARE,

    /// Specifies the content of the back buffer in the double-buffered main color plane following a buffer swap.
    /// Swapping the color buffers causes the content of the back buffer to be copied to the front buffer.
    /// The content of the back buffer is not affected by the swap.
    /// `PixelBufferProperty::SwapCopy` is a hint only and might not be provided by a driver.
    /// 
    /// 0x00000400
    SwapCopy=PFD_SWAP_COPY,

    /// Specifies the content of the back buffer in the double-buffered main color plane following a buffer swap.
    /// Swapping the color buffers causes the exchange of the back buffer's content with the front buffer's content.
    /// Following the swap, the back buffer's content contains the front buffer's content before the swap.
    /// `PixelBufferProperty::SwapExchange` is a hint only and might not be provided by a driver.
    /// 
    /// 0x00000200
    SwapExchange=PFD_SWAP_EXCHANGE
}

/// Specifies the type of pixel data.
#[repr(u8)]
pub enum PixelType{
    /// RGBA pixels. Each pixel has four components in this order: red, green, blue, and alpha.
    /// 
    /// 0
    RGBA=PFD_TYPE_RGBA,

    /// Color-index pixels. Each pixel uses a color-index value.
    /// 
    /// 1
    ColorIndex=PFD_TYPE_COLORINDEX,
}

pub struct PixelBufferProperties{
    flags:u32,
}

impl PixelBufferProperties{
    pub const fn new()->PixelBufferProperties{
        Self{
            flags:0u32,
        }
    }

    /// Sets a property.
    pub const fn set(mut self,property:PixelBufferProperty)->PixelBufferProperties{
        self.flags|=property as u32;
        self
    }

    /// Removes a property.
    pub const fn remove(mut self,property:PixelBufferProperty)->PixelBufferProperties{
        self.flags&=!(property as u32);
        self
    }
}

/// Describes the pixel format of a drawing surface.
/// 
/// Please notice carefully, as documented above,
/// that certain pixel format properties are not supported in the current generic implementation.
/// The generic implementation is the Microsoft GDI software implementation of OpenGL.
/// Hardware manufacturers may enhance parts of OpenGL,
/// and may support some pixel format properties not supported by the generic implementation.
pub struct PixelFormat{
    descriptor:PIXELFORMATDESCRIPTOR,
}

impl PixelFormat{
    /// Creates a zeroed pixel format.
    pub const fn new()->PixelFormat{
        Self{
            descriptor:PIXELFORMATDESCRIPTOR{
                nSize:size_of::<PIXELFORMATDESCRIPTOR>() as u16,
                nVersion:1,
                dwFlags:0,
                iPixelType:0,
                cColorBits:0,
                cRedBits:0,
                cRedShift:0,
                cGreenBits:0,
                cGreenShift:0,
                cBlueBits:0,
                cBlueShift:0,
                cAlphaBits:0,
                cAlphaShift:0,
                cAccumBits:0,
                cAccumRedBits:0,
                cAccumGreenBits:0,
                cAccumBlueBits:0,
                cAccumAlphaBits:0,
                cDepthBits:0,
                cStencilBits:0,
                cAuxBuffers:0,
                iLayerType:0,
                bReserved:0,
                dwLayerMask:0,
                dwVisibleMask:0,
                dwDamageMask:0,
            }
        }
    }

    pub const fn descriptor(&self)->&PIXELFORMATDESCRIPTOR{
        &self.descriptor
    }

    pub const fn set_flags(mut self,flags:PixelBufferProperties)->PixelFormat{
        self.descriptor.dwFlags=flags.flags;
        self
    }

    /// Specifies the type of pixel data.
    pub const fn set_pixel_type(mut self,pixel_type:PixelType)->PixelFormat{
        self.descriptor.iPixelType=pixel_type as u8;
        self
    }

    /// Specifies the number of color bitplanes in each color buffer.
    /// For RGBA pixel types, it is the size of the color buffer, excluding the alpha bitplanes.
    /// For color-index pixels, it is the size of the color-index buffer.
    pub const fn set_color_bits(mut self,bits:u8)->PixelFormat{
        self.descriptor.cColorBits=bits;
        self
    }

    pub const fn set_layer_type(mut self,layer_type:u8)->PixelFormat{
        self.descriptor.iLayerType=layer_type;
        self
    }

    pub const fn set_layer_mask(mut self,mask:u32)->PixelFormat{
        self.descriptor.dwLayerMask=mask;
        self
    }
}

/// A raster-operation code.
/// These codes define how the color data for the source rectangle
/// is to be combined with the color data for the destination rectangle
/// to achieve the final color.
#[repr(u32)]
pub enum BitBltOperation{
    /// Fills the destination rectangle using the color associated with index 0 in the physical palette.
    /// (This color is black for the default physical palette.)
    Blackness=BLACKNESS,

    /// Fills the destination rectangle using the color associated with index 1 in the physical palette.
    /// (This color is white for the default physical palette.)
    Whiteness=WHITENESS,

    /// Includes any windows that are layered on top of your window in the resulting image.
    /// By default, the image only contains your window.
    /// Note that this generally cannot be used for printing device contexts.
    CaptuteBlt=CAPTUREBLT,

    /// Inverts the destination rectangle.
    DestinationInvert=DSTINVERT,

    /// Merges the colours of the source rectangle
    /// with the brush currently selected in hdcDest,
    /// by using the Boolean AND operator.
    MergeCopy=MERGECOPY,

    /// Merges the colours of the inverted source rectangle
    /// with the colours of the destination rectangle
    /// by using the Boolean OR operator.
    MergePaint=MERGEPAINT,

    /// Prevents the bitmap from being mirrored.
    NoMirrorBitmap=NOMIRRORBITMAP,

    /// Copies the inverted source rectangle to the destination.
    NotSourceCopy=NOTSRCCOPY,

    /// Combines the colours of the source and destination rectangles
    /// by using the Boolean OR operator and then inverts the resultant color.
    NotSourceErase=NOTSRCERASE,

    /// Copies the brush currently selected in hdcDest, into the destination bitmap.
    PatternCopy=PATCOPY,

    /// Combines the colours of the brush currently selected in hdcDest,
    /// with the colours of the destination rectangle
    /// by using the Boolean XOR operator.
    PatternInvert=PATINVERT,

    /// Combines the colours of the brush currently selected in hdcDest,
    /// with the colours of the inverted source rectangle
    /// by using the Boolean OR operator.
    /// The result of this operation is combined
    /// with the colours of the destination rectangle
    /// by using the Boolean OR operator.
    PatternPaint=PATPAINT,

    /// Combines the colours of the source and destination rectangles
    /// by using the Boolean AND operator.
    SourceAnd=SRCAND,

    /// Copies the source rectangle directly to the destination rectangle.
    SourceCopy=SRCCOPY,

    /// Combines the inverted colours of the destination rectangle
    /// with the colours of the source rectangle
    /// by using the Boolean AND operator.
    SourceErase=SRCERASE,

    /// Combines the colours of the source and destination rectangles
    /// by using the Boolean XOR operator.
    SourceInvert=SRCINVERT,

    /// Combines the colours of the source and destination rectangles
    /// by using the Boolean OR operator.
    SourcePaint=SRCPAINT,
}

/// The replacement for `HDC`.
/// Can be wraped with `Option` with null pointer optimization.
#[derive(Clone,Copy)]
#[repr(transparent)]
pub struct DeviceContextHandle{
    inner:NonNull<HDC>,
}
implement_handle_wrapper!(DeviceContextHandle,HDC);

/// Wraps the Windows API functions releative to a device context.
pub struct DeviceContext;

impl DeviceContext{
    pub const fn new()->DeviceContext{
        Self
    }
}

impl DeviceContext{
    /// Creates a memory device context (DC) compatible with the specified device.
    /// 
    /// `context` is handle to an existing DC. If this handle is `None`,
    /// the function creates a memory DC compatible with the application's current screen.
    /// 
    /// A memory DC exists only in memory. When the memory DC is created,
    /// its display surface is exactly one monochrome pixel wide and one monochrome pixel high.
    /// Before an application can use a memory DC for drawing operations,
    /// it must select a bitmap of the correct width and height into the DC.
    /// To select a bitmap into a DC, use the CreateCompatibleBitmap function,
    /// specifying the height, width, and color organization required.
    /// 
    /// When a memory DC is created, all attributes are set to normal default values.
    /// The memory DC can be used as a normal DC.
    /// You can set the attributes; obtain the current settings of its attributes;
    /// and select pens, brushes, and regions.
    /// 
    /// The CreateCompatibleDC function can only be used with devices that support raster operations.
    /// An application can determine whether a device supports these operations by calling the GetDeviceCaps function.
    /// 
    /// When you no longer need the memory DC, call the DeleteDC function.
    /// We recommend that you call DeleteDC to delete the DC.
    /// However, you can also call DeleteObject with the HDC to delete the DC.
    /// 
    /// If hdc is NULL, the thread that calls CreateCompatibleDC owns the HDC that is created.
    /// When this thread is destroyed, the HDC is no longer valid.
    /// Thus, if you create the HDC and pass it to another thread,
    /// then exit the first thread, the second thread will not be able to use the HDC.
    /// 
    /// ICM: If the DC that is passed to this function is enabled for Image Color Management (ICM),
    /// the DC created by the function is ICM-enabled.
    /// The source and destination color spaces are specified in the DC.
    /// 
    /// If the function succeeds, the return value is the handle to a memory DC.
    /// 
    /// If the function fails, the return value is `None`.
    #[inline(always)]
    pub unsafe fn create_compatible_context(
        &self,
        context:Option<DeviceContextHandle>
    )->Option<DeviceContextHandle>{
        DeviceContextHandle::from_raw(
            CreateCompatibleDC(DeviceContextHandle::to_raw(context))
        )
    }

    /// Releases a device context (DC),
    /// freeing it for use by other applications.
    /// The effect of the ReleaseDC function depends on the type of DC.
    /// It frees only common and window DCs.
    /// It has no effect on class or private DCs.
    /// 
    /// If the DC was released, returns `true`.
    /// If the DC was not released, returns `false`.
    #[inline(always)]
    pub unsafe fn release(&self,window:WindowHandle,context:DeviceContextHandle)->bool{
        ReleaseDC(window.as_raw(),context.as_raw())!=0
    }
}


impl DeviceContext{
    /// Exchanges the front and back buffersif the current pixel format
    /// for the window referencedby the specified device context includes a back buffer.
    /// 
    /// If the current pixel format for the window referenced by the device context does not include a back buffer,
    /// this call has no effect and the content of the back buffer is undefined when the function returns.
    /// With multithread applications,
    /// flush the drawing commands in any other threads drawing to the same window before calling SwapBuffers.
    /// 
    /// If the function succeeds, returns `true`.
    /// If the function fails, returns `false`.
    /// To get extended error information, call `WinCore::get_last_error`.
    #[inline(always)]
    pub unsafe fn swap_buffers(&self,context:DeviceContextHandle)->bool{
        SwapBuffers(context.as_raw())!=0
    }

    /// Attempts to match an appropriate pixel format
    /// supported by a device context to a given pixel format specification.
    /// 
    /// You must ensure that the pixel format matched by the ChoosePixelFormat function satisfies your requirements.
    /// For example, if you request a pixel format with a 24-bit RGB color buffer
    /// but the device context offers only 8-bit RGB color buffers,
    /// the function returns a pixel format with an 8-bit RGB color buffer.
    /// 
    /// If the function succeeds,
    /// the return value is a pixel format index (one-based)
    /// that is the closest match to the given pixel format descriptor.
    /// If the function fails, the return value is zero.
    /// To get extended error information, call `WinCore::get_last_error`.
    #[inline(always)]
    pub unsafe fn choose_pixel_format(&self,context:DeviceContextHandle,format:&PixelFormat)->i32{
        ChoosePixelFormat(context.as_raw(),format.descriptor())
    }

    /// Sets the pixel format of the specified device context to the format specified by the PixelFormat index.
    /// 
    /// If hdc references a window, calling the `DeviceContext::set_pixel_format` function also changes the pixel format of the window.
    /// Setting the pixel format of a window more than once
    /// can lead to significant complications for the Window Manager and for multithread applications,
    /// so it is not allowed.
    /// An application can only set the pixel format of a window one time.
    /// Once a window's pixel format is set, it cannot be changed.
    /// 
    /// You should select a pixel format in the device context before calling the `wglCreateContext` function.
    /// The `wglCreateContext` function creates a rendering context
    /// for drawing on the device in the selected pixel format of the device context.
    /// 
    /// An OpenGL window has its own pixel format.
    /// Because of this,only device contexts retrieved for the client area of an OpenGL window are allowed to draw into the window.
    /// As a result, an OpenGL window should be created with the `WS_CLIPCHILDREN` and `WS_CLIPSIBLINGS` styles.
    /// Additionally, the window class attribute should not include the `CS_PARENTDC` style.
    /// 
    /// If the function succeeds, returns `true`.
    /// If the function fails, returns `false`.
    /// To get extended error information, call `WinCore::get_last_error`.
    #[inline(always)]
    pub unsafe fn set_pixel_format(
        &self,
        context:DeviceContextHandle,
        format_index:i32,
        format:&PixelFormat
    )->bool{
        SetPixelFormat(context.as_raw(),format_index,format.descriptor())!=0
    }
}

impl DeviceContext{
    /// Performs a bit-block transfer of the color data corresponding to a rectangle of pixels
    /// from the specified source device context into a destination device context.
    /// 
    /// Only does clipping on the destination DC.
    /// 
    /// If a rotation or shear transformation is in effect in the source device context,
    /// `DeviceContext::bit_blt` returns an error.
    /// If other transformations exist in the source device context
    /// (and a matching transformation is not in effect in the destination device context),
    /// the rectangle in the destination device context is stretched, compressed, or rotated, as necessary.
    /// 
    /// If the color formats of the source and destination device contexts do not match,
    /// the `DeviceContext::bit_blt` function converts the source color format to match the destination format.
    /// 
    /// When an enhanced metafile is being recorded,
    /// an error occurs if the source device context identifies an enhanced-metafile device context.
    /// 
    /// Not all devices support the BitBlt function.
    /// For more information, see the `RC_BITBLT` raster capability entry in the `GetDeviceCaps` function
    /// as well as the following functions: `MaskBlt`, `PlgBlt`, and `StretchBlt`.
    /// 
    /// Returns an error if the source and destination device contexts represent different devices.
    /// To transfer data between DCs for different devices,
    /// convert the memory bitmap to a DIB by calling `GetDIBits`.
    /// To display the DIB to the second device, call `SetDIBits` or `StretchDIBits`.
    /// 
    /// ICM: No color management is performed when blits occur.
    /// 
    /// If the function succeeds, the return value is `true`.
    /// If the function fails, the return value is `false`.
    /// To get extended error information, call `WinCore::get_last_error`.
    #[inline(always)]
    pub unsafe fn bit_blt(
        &self,
        destination_context:DeviceContextHandle,
        [dx,dy]:[i32;2],
        source_context:DeviceContextHandle,
        [sx,sy]:[i32;2],
        [width,height]:[i32;2],
        operation:BitBltOperation,
    )->bool{
        BitBlt(
            destination_context.as_raw(),
            dx,dy,width,height,
            source_context.as_raw(),
            sx,sy,
            operation as u32
        )!=0
    }


}

impl DeviceContext{
    /// Selects an object into the specified device context (DC).
    /// The new object replaces the previous object of the same type.
    /// 
    /// This function returns the previously selected object of the specified type.
    /// An application should always replace a new object with the original,
    /// default object after it has finished drawing with the new object.
    /// 
    /// An application cannot select a single bitmap into more than one DC at a time.
    /// 
    /// ICM: If the object being selected is a brush or a pen, color management is performed.
    /// 
    /// If the selected object is not a region and the function succeeds,
    /// the return value is a handle to the object being replaced.
    /// If the selected object is a region and the function succeeds,
    /// the return value is one of the following values:
    /// `SIMPLEREGION` - Region consists of a single rectangle,
    /// `COMPLEXREGION` - Region consists of more than one rectangle,
    /// `NULLREGION` - Region is empty.
    /// 
    /// If an error occurs and the selected object is not a region,
    /// the return value is NULL.
    /// Otherwise, it is HGDI_ERROR.
    #[inline(always)]
    pub unsafe fn select_object(
        &self,
        context:DeviceContextHandle,
        handle:*mut (),
    )->*mut (){
        SelectObject(context.as_raw(),handle as *mut _) as *mut ()
    }

    /// Selects a bitmap into the specified device context (DC).
    /// The new bitmap replaces the previous one.
    /// 
    /// Same as `DeviceContext::select_object` but only for bitmaps.
    /// 
    /// This function returns the previously selected bitmap.
    /// An application should always replace a new object with the original,
    /// default object after it has finished drawing with the new object.
    /// 
    /// An application cannot select a single bitmap into more than one DC at a time.
    /// 
    /// If the function succeeds,
    /// the return value is a handle to the bitmap being replaced.
    /// 
    /// If an error occurs, the return value is `None`.
    #[inline(always)]
    pub unsafe fn select_bitmap(
        &self,
        context:DeviceContextHandle,
        handle:BitmapHandle,
    )->Option<BitmapHandle>{
        BitmapHandle::from_raw(
            SelectObject(context.as_raw(),handle.as_raw() as *mut _) as *mut _
        )
    }
}