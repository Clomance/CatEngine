use super::{
    *,
    Colour,
    ColourResult,
    bitmap::{
        BitmapHandle,
    },
    window::WindowHandle,
};

mod device_context;
pub use device_context::DeviceContext;

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

            // alpha blend
            AC_SRC_OVER,
            AC_SRC_ALPHA,
        },
    }
};

/// A set of bit flags that specify properties of the pixel buffer.
/// The properties are generally not mutually exclusive; you can set any combination of bit flags, with the exceptions noted.
#[derive(Clone,Copy)]
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
#[derive(Clone,Copy)]
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
#[derive(Clone)]
#[repr(transparent)]
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
#[derive(Clone,Copy)]
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

#[derive(Clone,Copy)]
#[repr(u32)]
pub enum PatternBltOperation{
    /// Fills the destination rectangle using the color associated with index 0 in the physical palette.
    /// (This color is black for the default physical palette.)
    Blackness=BLACKNESS,

    /// Fills the destination rectangle using the color associated with index 1 in the physical palette.
    /// (This color is white for the default physical palette.)
    Whiteness=WHITENESS,

    /// Inverts the destination rectangle.
    DestinationInvert=DSTINVERT,

    /// Copies the brush currently selected in hdcDest, into the destination bitmap.
    PatternCopy=PATCOPY,

    /// Combines the colours of the brush currently selected in hdcDest,
    /// with the colours of the destination rectangle
    /// by using the Boolean XOR operator.
    PatternInvert=PATINVERT,
}

/// A replacement for `HDC`.
/// Can be wraped with `Option` with null pointer optimization.
#[derive(Clone,Copy)]
#[repr(transparent)]
pub struct DeviceContextHandle{
    inner:NonNull<HDC>,
}
implement_handle_wrapper!(DeviceContextHandle,HDC);