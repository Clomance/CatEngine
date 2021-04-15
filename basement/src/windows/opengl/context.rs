use winapi::{
    shared::{
        minwindef::{
            PROC,
            HMODULE,
        },
        windef::{
            HGLRC,
            HDC,
        },
    },
    um::{
        wingdi::{
            // constants
            PFD_DRAW_TO_WINDOW,
            PFD_SUPPORT_OPENGL,
            PFD_DOUBLEBUFFER,
            PFD_TYPE_RGBA,
            PFD_MAIN_PLANE,
            // functions
            ChoosePixelFormat,
            SetPixelFormat,
            wglCreateContext,
            wglMakeCurrent,
            wglDeleteContext,
            SwapBuffers,
            wglGetProcAddress,
            // structs
            PIXELFORMATDESCRIPTOR,
        },
        libloaderapi::{
            LoadLibraryA,
            FreeLibrary,
            GetProcAddress,
        },
    }
};

use std::{
    ptr::null_mut,
    mem::size_of,
};

pub struct RenderContext{
    window_context:HDC,
    render_context:HGLRC,
}

impl RenderContext{
    pub fn opengl(window_context:HDC,attributes:RenderContextAttributes)->Option<RenderContext>{
        unsafe{
            let pixel_format_descriptor=PIXELFORMATDESCRIPTOR{
                nSize:size_of::<PIXELFORMATDESCRIPTOR>() as u16,
                nVersion:1,
                dwFlags:PFD_DRAW_TO_WINDOW|PFD_SUPPORT_OPENGL|PFD_DOUBLEBUFFER,    // Flags
                iPixelType:PFD_TYPE_RGBA,        // The kind of framebuffer. RGBA or palette.
                cColorBits:attributes.colour_bits,                   // Colordepth of the framebuffer.
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
                cDepthBits:0, // Number of bits for the depthbuffer
                cStencilBits:0, // Number of bits for the stencilbuffer
                cAuxBuffers:0, // Number of Aux buffers in the framebuffer.
                iLayerType:PFD_MAIN_PLANE,
                bReserved:0,
                dwLayerMask:0,
                dwVisibleMask:0,
                dwDamageMask:0,
            };

            let pixel_format=ChoosePixelFormat(window_context,&pixel_format_descriptor);
            if pixel_format==0{
                return None
            }

            let result=SetPixelFormat(window_context,pixel_format,&pixel_format_descriptor);
            if result==0{
                return None
            }

            // Создание временного контектса для создания расширенного
            let temp_context=wglCreateContext(window_context);
            if temp_context.is_null(){
                return None
            }

            if wglMakeCurrent(window_context,temp_context)==0{
                return None
            }

            // Загрузка функций расширенного OpenGL
            let wglChoosePixelFormatARB:wglChoosePixelFormatARB_t=core::mem::transmute({
                wglGetProcAddress("wglChoosePixelFormatARB\0".as_ptr() as *const i8)
            });

            let wglCreateContextAttribsARB:wglCreateContextAttribsARB_t=core::mem::transmute(
                wglGetProcAddress("wglCreateContextAttribsARB\0".as_ptr() as *const i8)
            );

            let wglSwapIntervalEXT:wglSwapIntervalEXT_t=core::mem::transmute(
                wglGetProcAddress("wglSwapIntervalEXT\0".as_ptr() as *const i8)
            );

            let attributes=[
                WGL_DRAW_TO_WINDOW_ARB,1i32,
                WGL_SUPPORT_OPENGL_ARB,1i32,
                WGL_DOUBLE_BUFFER_ARB,1i32,
                WGL_PIXEL_TYPE_ARB,WGL_TYPE_RGBA_ARB,
                WGL_COLOR_BITS_ARB,32i32,
                WGL_DEPTH_BITS_ARB,0i32,
                WGL_STENCIL_BITS_ARB,0i32,
                0i32,
            ];

            let mut pixel_format=0i32;
            let mut num_formats=0u32;

            let mut result=
                wglChoosePixelFormatARB.expect("wglChoosePixelFormatARB is not loaded")(window_context,attributes.as_ptr(),null_mut(),1,&mut pixel_format,&mut num_formats);

            if result==0{
                panic!("No such pixel format");
            }

            let render_context=
                wglCreateContextAttribsARB.expect("wglCreateContextAttribsARB is not loaded")(window_context,temp_context,&pixel_format);
            if render_context.is_null(){
                panic!("Error :)");
            }

            // Удаление временного контекста и переключение к новому
            if wglMakeCurrent(window_context,render_context)==0{
                panic!("Error");
            }
            if wglDeleteContext(temp_context)==0{
                panic!("ErrorDelete");
            }

            // vsync
            wglSwapIntervalEXT.expect("wglSwapIntervalEXT is not loaded")(1);

            Some(Self{
                window_context,
                render_context,
            })
        }
    }

    pub fn raw(&self)->HGLRC{
        self.render_context
    }
}

impl Drop for RenderContext{
    fn drop(&mut self){
        unsafe{
            wglMakeCurrent(self.window_context,null_mut());
            wglDeleteContext(self.render_context);
        }
    }
}

pub struct RenderContextAttributes{
    /// The default is 32.
    colour_bits:u8,
}

impl RenderContextAttributes{
    pub fn new()->RenderContextAttributes{
        Self{
            colour_bits:32u8,
        }
    }
}

/// from [rust-tutorials](https://rust-tutorials.github.io/triangle-from-scratch/loading_opengl/win32.html)
/// Type for [wglChoosePixelFormatARB](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub type wglChoosePixelFormatARB_t=Option<
    unsafe extern "system" fn(
        hdc:HDC,
        piAttribIList:*const i32,
        pfAttribFList:*const f32,
        nMaxFormats:u32,
        piFormats:*mut i32,
        nNumFormats:*mut u32
    )->i32,
>;
/// Type for [wglCreateContextAttribsARB](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_create_context.txt)
pub type wglCreateContextAttribsARB_t=Option<
    unsafe extern "system" fn(
        hDC:HDC,
        hShareContext:HGLRC,
        attribList:*const i32,
    )->HGLRC,
>;
/// Type for [wglSwapIntervalEXT](https://www.khronos.org/registry/OpenGL/extensions/EXT/WGL_EXT_swap_control.txt)
pub type wglSwapIntervalEXT_t=Option<unsafe extern "system" fn(interval:i32)->i32>;



// const WGL_NUMBER_PIXEL_FORMATS_ARB            0x2000
const WGL_DRAW_TO_WINDOW_ARB:i32=0x2001;
// const WGL_DRAW_TO_BITMAP_ARB                  0x2002
// const WGL_ACCELERATION_ARB                    0x2003
// const WGL_NEED_PALETTE_ARB                    0x2004
// const WGL_NEED_SYSTEM_PALETTE_ARB             0x2005
// const WGL_SWAP_LAYER_BUFFERS_ARB              0x2006
// const WGL_SWAP_METHOD_ARB                     0x2007
// const WGL_NUMBER_OVERLAYS_ARB                 0x2008
// const WGL_NUMBER_UNDERLAYS_ARB                0x2009
// const WGL_TRANSPARENT_ARB                     0x200A
// const WGL_TRANSPARENT_RED_VALUE_ARB           0x2037
// WGL_TRANSPARENT_GREEN_VALUE_ARB         0x2038
// WGL_TRANSPARENT_BLUE_VALUE_ARB          0x2039
// WGL_TRANSPARENT_ALPHA_VALUE_ARB         0x203A
// WGL_TRANSPARENT_INDEX_VALUE_ARB         0x203B
// WGL_SHARE_DEPTH_ARB                     0x200C
// WGL_SHARE_STENCIL_ARB                   0x200D
// WGL_SHARE_ACCUM_ARB                     0x200E
// WGL_SUPPORT_GDI_ARB                     0x200F
const WGL_SUPPORT_OPENGL_ARB:i32=0x2010;
const WGL_DOUBLE_BUFFER_ARB:i32=0x2011;
// WGL_STEREO_ARB                          0x2012
const WGL_PIXEL_TYPE_ARB:i32=0x2013;
const WGL_COLOR_BITS_ARB:i32=0x2014;
// WGL_RED_BITS_ARB                        0x2015
// WGL_RED_SHIFT_ARB                       0x2016
// WGL_GREEN_BITS_ARB                      0x2017
// WGL_GREEN_SHIFT_ARB                     0x2018
// WGL_BLUE_BITS_ARB                       0x2019
// WGL_BLUE_SHIFT_ARB                      0x201A
// WGL_ALPHA_BITS_ARB                      0x201B
// WGL_ALPHA_SHIFT_ARB                     0x201C
// WGL_ACCUM_BITS_ARB                      0x201D
// WGL_ACCUM_RED_BITS_ARB                  0x201E
// WGL_ACCUM_GREEN_BITS_ARB                0x201F
// WGL_ACCUM_BLUE_BITS_ARB                 0x2020
// WGL_ACCUM_ALPHA_BITS_ARB                0x2021
const WGL_DEPTH_BITS_ARB:i32=0x2022;
const WGL_STENCIL_BITS_ARB:i32=0x2023;
// WGL_AUX_BUFFERS_ARB                     0x2024

// WGL_NO_ACCELERATION_ARB                 0x2025
// WGL_GENERIC_ACCELERATION_ARB            0x2026
// WGL_FULL_ACCELERATION_ARB               0x2027

// WGL_SWAP_EXCHANGE_ARB                   0x2028
// WGL_SWAP_COPY_ARB                       0x2029
// WGL_SWAP_UNDEFINED_ARB                  0x202A

const WGL_TYPE_RGBA_ARB:i32=0x202B;
// WGL_TYPE_COLORINDEX_ARB                 0x202C