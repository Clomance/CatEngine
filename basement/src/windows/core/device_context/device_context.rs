use super::{
    *,
    Colour,
    ColourResult,
    bitmap::{
        BitmapHandle,
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
            PatBlt,
            AlphaBlend,

            GetBrushOrgEx,
            SetBrushOrgEx,
            SetDCBrushColor,

            AC_SRC_OVER,
            AC_SRC_ALPHA,
        },
    }
};

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
    /// The `DeviceContext::create_compatible` function can only be used with devices that support raster operations.
    /// An application can determine whether a device supports these operations by calling the `GetDeviceCaps` function.
    /// 
    /// When you no longer need the memory DC, call the `DeleteDC` function.
    /// We recommend that you call `DeleteDC` to delete the DC.
    /// However, you can also call `DeleteObject` with the HDC to delete the DC.
    /// 
    /// If hdc is `None`, the thread that calls `DeviceContext::create_compatible` owns the HDC that is created.
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
    pub fn create_compatible(
        &self,
        context:Option<DeviceContextHandle>
    )->Option<DeviceContextHandle>{
        unsafe{
            DeviceContextHandle::from_raw(
                CreateCompatibleDC(DeviceContextHandle::to_raw(context))
            )
        }
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
    pub fn release(&self,window:WindowHandle,context:DeviceContextHandle)->bool{
        unsafe{
            ReleaseDC(window.as_raw(),context.as_raw())!=0
        }
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
    pub fn swap_buffers(&self,context:DeviceContextHandle)->bool{
        unsafe{
            SwapBuffers(context.as_raw())!=0
        }
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
    pub fn choose_pixel_format(&self,context:DeviceContextHandle,format:&PixelFormat)->i32{
        unsafe{
            ChoosePixelFormat(context.as_raw(),format.descriptor())
        }
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
    pub fn set_pixel_format(
        &self,
        context:DeviceContextHandle,
        format_index:i32,
        format:&PixelFormat
    )->bool{
        unsafe{
            SetPixelFormat(context.as_raw(),format_index,format.descriptor())!=0
        }
    }
}

/// Controls blending by specifying the blending functions for source and destination bitmaps.
#[derive(Clone,Copy)]
#[repr(C)]
pub struct BlendFunction{
    /// The source blend operation.
    /// Currently, the only source and destination blend operation
    /// that has been defined is AC_SRC_OVER.
    operation:u8,

    /// Must be zero.
    flags:u8,

    /// Specifies an alpha transparency value to be used on the entire source bitmap.
    /// The value is combined with any per-pixel alpha values in the source bitmap.
    /// If you set the value to 0, it is assumed that your image is transparent.
    /// Set the value to 255 (opaque) when you only want to use per-pixel alpha values.
    source_constant_alpha:u8,

    /// This flag is set when the bitmap has an Alpha channel (that is, per-pixel alpha).
    /// Note that the APIs use premultiplied alpha,
    /// which means that the red, green and blue channel values in the bitmap
    /// must be premultiplied with the alpha channel value.
    /// For example, if the alpha channel value is x, the red, green and blue channels
    /// must be multiplied by x and divided by 0xff prior to the call.
    alpha_format:u8,
}

impl BlendFunction{
    pub const fn new(source_constant_alpha:u8)->BlendFunction{
        Self{
            operation:AC_SRC_OVER,
            flags:0u8,
            source_constant_alpha,
            alpha_format:AC_SRC_ALPHA
        }
    }
}

/// Bit-block transfer functions.
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
    pub fn bit_blt(
        &self,
        destination_context:DeviceContextHandle,
        [dx,dy]:[i32;2],
        source_context:DeviceContextHandle,
        [sx,sy]:[i32;2],
        [width,height]:[i32;2],
        operation:BitBltOperation,
    )->bool{
        unsafe{
            BitBlt(
                destination_context.as_raw(),
                dx,dy,width,height,
                source_context.as_raw(),
                sx,sy,
                operation as u32
            )!=0
        }
    }

    /// Paints the specified rectangle using the brush that is currently selected into the specified device context.
    /// The brush color and the surface color or colors are combined by using the specified raster operation.
    /// 
    /// The values of the `operation` parameter for this function are a limited subset of the full 256 ternary raster-operation codes;
    /// in particular, an operation code that refers to a source rectangle cannot be used.
    /// 
    /// Not all devices support the `DeviceContext::pattern_blt` function.
    /// For more information, see the description of the `RC_BITBLT` capability in the `GetDeviceCaps` function.
    /// 
    /// If the function succeeds, the return value is `true`.
    /// 
    /// If the function fails, the return value is `false`.
    pub fn pattern_blt(&self,context:DeviceContextHandle,[x,y,width,height]:[i32;4],operation:PatternBltOperation)->bool{
        unsafe{
            PatBlt(context.as_raw(),x,y,width,height,operation as u32)!=0
        }
    }

    /// Displays bitmaps that have transparent or semitransparent pixels.
    /// 
    /// When the `AlphaFormat` member is `AC_SRC_ALPHA` (always),
    /// the source bitmap must be 32 bpp.
    /// If it is not, the AlphaBlend function will fail.
    /// 
    /// When the `BlendOp` member is `AC_SRC_OVER` (always),
    /// the source bitmap is placed over the destination bitmap
    /// based on the alpha values of the source pixels.
    /// ```
    /// colour = source * (source_alpha/255) + destination * (1-(destination_alpha/255))
    /// ```
    /// 
    /// If the source bitmap does not use `source_constant_alpha` (that is, it equals 0xFF),
    /// the per-pixel alpha determines the blend of the source and destination bitmaps,
    /// as shown in the following table.
    /// ```
    /// colour = source + destination * (1-source_alpha)
    /// ```
    /// 
    /// If the source has both the `source_constant_alpha` (that is, it is not 0xFF) and per-pixel alpha,
    /// the source is pre-multiplied by the `source_constant_alpha`
    /// and then the blend is based on the per-pixel alpha.
    /// The following tables show this.
    /// Note that `source_constant_alpha` is divided by 255 because it has a value that ranges from 0 to 255.
    /// ```
    /// colour = source + source_alpha/255
    /// ```
    /// 
    /// If the source rectangle and destination rectangle are not the same size,
    /// the source bitmap is stretched to match the destination rectangle.
    /// If the `SetStretchBltMode` function is used,
    /// the `iStretchMode` value is automatically converted to COLORONCOLOR for this function
    /// (that is, BLACKONWHITE, WHITEONBLACK, and HALFTONE are changed to COLORONCOLOR).
    /// 
    /// The destination coordinates are transformed
    /// by using the transformation currently specified for the destination device context.
    /// The source coordinates are transformed
    /// by using the transformation currently specified for the source device context.
    /// 
    /// An error occurs (and the function returns `false`)
    /// if the source device context identifies an enhanced metafile device context.
    /// 
    /// If destination and source bitmaps do not have the same colour format,
    /// `Bitmap::alpha_blend` converts the source bitmap to match the destination bitmap.
    /// 
    /// `Bitmap::alpha_blend` does not support mirroring.
    /// If either the width or height of the source or destination is negative,
    /// this call will fail.
    /// 
    /// When rendering to a printer, first call GetDeviceCaps with SHADEBLENDCAPS to determine
    /// if the printer supports blending with AlphaBlend. Note that, for a display DC,
    /// all blending operations are supported and these flags represent whether the operations are accelerated.
    /// 
    /// If the source and destination are the same surface,
    /// that is, they are both the screen or the same memory bitmap
    /// and the source and destination rectangles overlap,
    /// an error occurs and the function returns `false`.
    /// 
    /// The source rectangle must lie completely within the source surface,
    /// otherwise an error occurs and the function returns `false`.
    /// 
    /// `Bitmap::alpha_blend` fails if the width or height of the source or destination is negative.
    /// 
    /// The `source_constant_alpha` member of `BlendFunction` specifies an alpha transparency value to be used
    /// on the entire source bitmap. The `source_constant_alpha` value is combined with any per-pixel alpha values.
    /// If `source_constant_alpha` is 0, it is assumed that the image is transparent.
    /// Set the `source_constant_alpha` value to 255 (which indicates that the image is opaque)
    /// when you only want to use per-pixel alpha values.
    /// 
    /// If the function succeeds, the return value is `true`.
    /// 
    /// If the function fails, the return value is `false`.
    #[inline(always)]
    pub unsafe fn alpha_blend(
        &self,
        destination_context:DeviceContextHandle,
        [dx,dy,dwidth,dheight]:[i32;4],
        source_context:DeviceContextHandle,
        [sx,sy,swidth,sheight]:[i32;4],
        function:BlendFunction,
    )->bool{
        AlphaBlend(
            destination_context.as_raw(),
            dx,dy,dwidth,dheight,
            source_context.as_raw(),
            sx,sy,swidth,sheight,
            transmute(function)
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
    /// ICM: If the object being selected is a brush or a pen, colour management is performed.
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
    /// the return value is `NULL`.
    /// Otherwise, it is `HGDI_ERROR`.
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
    pub fn select_bitmap(&self,context:DeviceContextHandle,handle:BitmapHandle)->Option<BitmapHandle>{
        unsafe{
            transmute(SelectObject(context.as_raw(),transmute(handle)))
        }
    }
}

/// Brush functions.
impl DeviceContext{
    /// Sets the brush origin that GDI assigns to the next brush an application selects into the specified device context.
    /// 
    /// `x` - The x-coordinate, in device units, of the new brush origin.
    /// If this value is greater than the brush width,
    /// its value is reduced using the modulus operator (nXOrg mod brush width).
    /// 
    /// `y` - The y-coordinate, in device units, of the new brush origin.
    /// If this value is greater than the brush height,
    /// its value is reduced using the modulus operator (nYOrg mod brush height).
    /// 
    /// The brush origin is a pair of coordinates specifying the location of one pixel in the bitmap.
    /// The default brush origin coordinates are (0,0).
    /// For horizontal coordinates, the value 0 corresponds to the leftmost column of pixels;
    /// the width corresponds to the rightmost column.
    /// For vertical coordinates, the value 0 corresponds to the uppermost row of pixels;
    /// the height corresponds to the lowermost row.
    /// 
    /// The system automatically tracks the origin of all window-managed device contexts
    /// and adjusts their brushes as necessary to maintain an alignment of patterns on the surface.
    /// The brush origin that is set with this call is relative to the upper-left corner of the client area.
    /// 
    /// An application should call SetBrushOrgEx after setting the bitmap stretching mode to HALFTONE by using SetStretchBltMode.
    /// This must be done to avoid brush misalignment.
    /// 
    /// The system automatically tracks the origin of all window-managed device contexts
    /// and adjusts their brushes as necessary to maintain an alignment of patterns on the surface.
    /// 
    /// If the function succeeds, the return value is `true`.
    /// 
    /// If the function fails, the return value is `false`.
    pub fn set_brush_origin(
        &self,
        context:DeviceContextHandle,
        [x,y]:[i32;2],
        previous_point:Option<&mut [i32;2]>
    )->bool{
        unsafe{
            SetBrushOrgEx(context.as_raw(),x,y,transmute(previous_point))!=0
        }
    }

    /// Retrieves the current brush origin for the specified device context.
    /// 
    /// The brush origin is a set of coordinates with values between 0 and 7,
    /// specifying the location of one pixel in the bitmap.
    /// The default brush origin coordinates are (0,0).
    /// For horizontal coordinates, the value 0 corresponds to the leftmost column of pixels;
    /// the value 7 corresponds to the rightmost column.
    /// For vertical coordinates, the value 0 corresponds to the uppermost row of pixels;
    /// the value 7 corresponds to the lowermost row.
    /// When the system positions the brush at the start of any painting operation,
    /// it maps the origin of the brush to the location in the window's client area specified by the brush origin.
    /// For example, if the origin is set to (2,3),
    /// the system maps the origin of the brush (0,0) to the location (2,3) on the window's client area.
    /// 
    /// If an application uses a brush to fill the backgrounds of both a parent and a child window with matching colors,
    /// it may be necessary to set the brush origin after painting the parent window but before painting the child window.
    /// 
    /// The system automatically tracks the origin of all window-managed device contexts
    /// and adjusts their brushes as necessary to maintain an alignment of patterns on the surface.
    /// 
    /// If the function succeeds, the return value is `true`.
    /// 
    /// If the function fails, the return value is `false`.
    pub fn get_brush_origin(&self,context:DeviceContextHandle,point:&mut [i32;2])->bool{
        unsafe{
            GetBrushOrgEx(context.as_raw(),transmute(point))!=0
        }
    }

    /// Sets the current device context (DC) brush color to the specified color value.
    /// If the device cannot represent the specified color value,
    /// the color is set to the nearest physical color.
    /// 
    /// When the stock DC_BRUSH is selected in a DC,
    /// all the subsequent drawings will be done using the DC brush color
    /// until the stock brush is deselected.
    /// The default DC_BRUSH color is WHITE.
    /// 
    /// The function returns the previous DC_BRUSH color,
    /// even if the stock brush DC_BRUSH is not selected in the DC:
    /// however, this will not be used in drawing operations
    /// until the stock DC_BRUSH is selected in the DC.
    /// 
    /// The GetStockObject function with an argument of DC_BRUSH
    /// or DC_PEN can be used interchangeably with the SetDCPenColor and SetDCBrushColor functions.
    /// 
    /// ICM: Colour management is performed if ICM is enabled.
    /// 
    /// If the function succeeds, the return value specifies the previous DC brush color as a COLORREF value.
    /// 
    /// If the function fails, the return value is `CLR_INVALID`.
    pub fn set_brush_colour(&self,context:DeviceContextHandle,colour:Colour)->ColourResult{
        unsafe{
            transmute(SetDCBrushColor(context.as_raw(),colour.as_raw()))
        }
    }
}