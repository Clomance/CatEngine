use crate::windows::{
    Window,
    WinCore,
    WinError,
    core::device_context::{
        PixelType,
        PixelFormat,
        PixelBufferProperty,
        PixelBufferProperties,
        DeviceContextHandle,
    },
    core::window::WindowHandle,
    core::render_context::RenderContextHandle,
};

use winapi::{
    um::{
        wingdi::{
            // functions
            wglGetProcAddress,
        },
    }
};

pub struct OpenGLRenderContext{
    window:WindowHandle,
    window_context:DeviceContextHandle,
    render_context:RenderContextHandle,
    // vsync function (need to be replaced)
    wglSwapIntervalEXT:wglSwapIntervalEXT_t,
}

impl OpenGLRenderContext{
    pub fn new(
        window:&Window,
        attributes:OpenGLRenderContextAttributes
    )->Result<OpenGLRenderContext,WinError>{
        let pixel_format=PixelFormat::new()
            .set_color_bits(32)
            .set_flags(
                PixelBufferProperties::new()
                .set(PixelBufferProperty::DrawToWindow)
                .set(PixelBufferProperty::SupportOpenGL)
                .set(PixelBufferProperty::DoubleBuffer)
            )
            .set_pixel_type(PixelType::RGBA);

        unsafe{
            let window_context=window.get_context_unchecked();
    
            let pixel_format_index=WinCore.device_context.choose_pixel_format(window_context,&pixel_format);
            if pixel_format_index==0{
                return Err(WinError::get_last_error())
            }

            if !WinCore.device_context.set_pixel_format(window_context,pixel_format_index,&pixel_format){
                return Err(WinError::get_last_error())
            }

            // // Создание временного контектса для создания расширенного
            // Создание контектса
            let render_context=if let Some(render_context)=WinCore.render_context.wgl_create_context(window_context){
                render_context
            }
            else{
                return Err(WinError::get_last_error())
            };

            if !WinCore.render_context.wgl_make_current(Some(window_context),Some(render_context)){
                return Err(WinError::get_last_error())
            }

            // Загрузка функций расширенного OpenGL
            // let wglChoosePixelFormatARB:wglChoosePixelFormatARB_t=core::mem::transmute({
            //     wglGetProcAddress("wglChoosePixelFormatARB\0".as_ptr() as *const i8)
            // });

            // let wglCreateContextAttribsARB:wglCreateContextAttribsARB_t=core::mem::transmute(
            //     wglGetProcAddress("wglCreateContextAttribsARB\0".as_ptr() as *const i8)
            // );

            let wglSwapIntervalEXT:wglSwapIntervalEXT_t=core::mem::transmute(
                wglGetProcAddress("wglSwapIntervalEXT\0".as_ptr() as *const i8)
            );

            // let attributes=[
            //     WGL_DRAW_TO_WINDOW_ARB,1i32,
            //     WGL_SUPPORT_OPENGL_ARB,1i32,
            //     WGL_DOUBLE_BUFFER_ARB,1i32,
            //     WGL_PIXEL_TYPE_ARB,WGL_TYPE_RGBA_ARB,
            //     WGL_COLOR_BITS_ARB,attributes.colour_bits as i32,
            //     WGL_DEPTH_BITS_ARB,attributes.depth_bits as i32,
            //     WGL_STENCIL_BITS_ARB,0i32,
            //     0i32,
            // ];

            // let mut pixel_format=0i32;
            // let mut num_formats=0u32;

            // let mut result=
            //     wglChoosePixelFormatARB.expect("wglChoosePixelFormatARB is not loaded")(window_context,attributes.as_ptr(),null_mut(),1,&mut pixel_format,&mut num_formats);

            // if result==0{
            //     return Err(WinError::get_last_error())
            // }

            // let render_context=
            //     wglCreateContextAttribsARB.expect("wglCreateContextAttribsARB is not loaded")(window_context,temp_context,&pixel_format);
            // if render_context.is_null(){
            //     return Err(WinError::get_last_error())
            // }

            // // Удаление временного контекста и переключение к новому
            // if wglMakeCurrent(window_context,render_context)==0{
            //     return Err(WinError::get_last_error())
            // }
            // if wglDeleteContext(temp_context)==0{
            //     return Err(WinError::get_last_error())
            // }

            // vsync
            wglSwapIntervalEXT.expect("wglSwapIntervalEXT is not loaded")(attributes.vsync as i32);

            Ok(Self{
                window:window.handle,
                window_context,
                render_context,
                wglSwapIntervalEXT,
            })
        }
    }

    pub fn render_context(&self)->RenderContextHandle{
        self.render_context
    }

    /// Makes a specified OpenGL rendering context the calling thread's current rendering context.
    pub fn make_current(&self,current:bool)->Result<(),WinError>{
        unsafe{
            let result=if current{
                WinCore.render_context.wgl_make_current(Some(self.window_context),Some(self.render_context))
            }
            else{
                WinCore.render_context.wgl_make_current(None,None)
            };

            if result{
                Ok(())
            }
            else{
                Err(WinError::get_last_error())
            }
        }
    }

    pub fn swap_buffers(&self)->Result<(),WinError>{
        unsafe{
            if WinCore.device_context.swap_buffers(self.window_context){
                Ok(())
            }
            else{
                Err(WinError::get_last_error())
            }
        }
    }

    pub fn set_vsync(&self,enabled:bool)->Result<(),WinError>{
        unsafe{
            if self.wglSwapIntervalEXT.expect("wglSwapIntervalEXT is not loaded")(enabled as i32)==1{
                Ok(())
            }
            else{
                Err(WinError::get_last_error())
            }
        }
    }
}

impl Drop for OpenGLRenderContext{
    fn drop(&mut self){
        unsafe{
            WinCore.render_context.wgl_make_current(None,None);
            WinCore.render_context.wgl_delete_context(self.render_context);
            WinCore.device_context.release(self.window,self.window_context);
        }
    }
}

pub struct OpenGLRenderContextAttributes{
    pub vsync:bool,
}

impl OpenGLRenderContextAttributes{
    pub fn new()->OpenGLRenderContextAttributes{
        Self{
            vsync:true,
        }
    }
}

// /// from [rust-tutorials](https://rust-tutorials.github.io/triangle-from-scratch/loading_opengl/win32.html)
// /// Type for [wglChoosePixelFormatARB](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
// pub type wglChoosePixelFormatARB_t=Option<
//     unsafe extern "system" fn(
//         hdc:DeviceContextHandle,
//         piAttribIList:*const i32,
//         pfAttribFList:*const f32,
//         nMaxFormats:u32,
//         piFormats:*mut i32,
//         nNumFormats:*mut u32
//     )->i32,
// >;
// /// Type for [wglCreateContextAttribsARB](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_create_context.txt)
// pub type wglCreateContextAttribsARB_t=Option<
//     unsafe extern "system" fn(
//         hDC:DeviceContextHandle,
//         hShareContext:HGLRC,
//         attribList:*const i32,
//     )->HGLRC,
// >;
/// Type for [wglSwapIntervalEXT](https://www.khronos.org/registry/OpenGL/extensions/EXT/WGL_EXT_swap_control.txt)
pub type wglSwapIntervalEXT_t=Option<unsafe extern "system" fn(interval:i32)->i32>;



// const WGL_NUMBER_PIXEL_FORMATS_ARB            0x2000
// const WGL_DRAW_TO_WINDOW_ARB:i32=0x2001;
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
// const WGL_SUPPORT_OPENGL_ARB:i32=0x2010;
// const WGL_DOUBLE_BUFFER_ARB:i32=0x2011;
// WGL_STEREO_ARB                          0x2012
// const WGL_PIXEL_TYPE_ARB:i32=0x2013;
// const WGL_COLOR_BITS_ARB:i32=0x2014;
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
// const WGL_DEPTH_BITS_ARB:i32=0x2022;
// const WGL_STENCIL_BITS_ARB:i32=0x2023;
// WGL_AUX_BUFFERS_ARB                     0x2024

// WGL_NO_ACCELERATION_ARB                 0x2025
// WGL_GENERIC_ACCELERATION_ARB            0x2026
// WGL_FULL_ACCELERATION_ARB               0x2027

// WGL_SWAP_EXCHANGE_ARB                   0x2028
// WGL_SWAP_COPY_ARB                       0x2029
// WGL_SWAP_UNDEFINED_ARB                  0x202A

// const WGL_TYPE_RGBA_ARB:i32=0x202B;
// WGL_TYPE_COLORINDEX_ARB                 0x202C