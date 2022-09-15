use crate::winapi::{
    window::Window,
    Error,
    backend::core::device_context::{
        DeviceContext as DeviceContextFunctions,
        PixelType,
        PixelFormat,
        PixelBufferProperty,
        PixelBufferProperties,
        DeviceContextHandle,
    },
    backend::core::window::WindowHandle,
    backend::core::render_context::{
        RenderContext as RenderContextFunctions,
        RenderContextHandle
    },
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
    // vsync function (needs to be replaced)
    swap_interval_extension:Option<unsafe extern "system" fn(interval:i32)->i32>,
}

impl OpenGLRenderContext{
    pub fn new(
        window:&Window,
        attributes:OpenGLRenderContextAttributes
    )->Result<OpenGLRenderContext,Error>{
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
    
            let pixel_format_index=DeviceContextFunctions::choose_pixel_format(window_context,&pixel_format);
            if pixel_format_index==0{
                return Err(Error::get_last_error())
            }

            if !DeviceContextFunctions::set_pixel_format(window_context,pixel_format_index,&pixel_format){
                return Err(Error::get_last_error())
            }

            // // Создание временного контектса для создания расширенного
            // Создание контектса
            let render_context=if let Some(render_context)=RenderContextFunctions::create_context(window_context){
                render_context
            }
            else{
                return Err(Error::get_last_error())
            };

            if !RenderContextFunctions::make_current(Some(window_context),Some(render_context)){
                return Err(Error::get_last_error())
            }

            let swap_interval_extension:Option<unsafe extern "system" fn(interval:i32)->i32>=core::mem::transmute(
                wglGetProcAddress("wglSwapIntervalEXT\0".as_ptr() as *const i8)
            );

            // vsync
            swap_interval_extension.expect("wglSwapIntervalEXT is not loaded")(attributes.vsync as i32);

            Ok(Self{
                window:window.handle,
                window_context,
                render_context,
                swap_interval_extension,
            })
        }
    }

    pub fn handle(&self)->RenderContextHandle{
        self.render_context
    }

    /// Makes a specified OpenGL rendering context the calling thread's current rendering context.
    pub fn make_current(&self,current:bool)->Result<(),Error>{
        unsafe{
            let result=if current{
                RenderContextFunctions::make_current(Some(self.window_context),Some(self.render_context))
            }
            else{
                RenderContextFunctions::make_current(None,None)
            };

            if result{
                Ok(())
            }
            else{
                Err(Error::get_last_error())
            }
        }
    }

    pub fn swap_buffers(&self)->Result<(),Error>{
        if DeviceContextFunctions::swap_buffers(self.window_context){
            Ok(())
        }
        else{
            Err(Error::get_last_error())
        }
    }

    pub fn set_vsync(&self,enabled:bool)->Result<(),Error>{
        unsafe{
            if self.swap_interval_extension.expect("wglSwapIntervalEXT is not loaded")(enabled as i32)==1{
                Ok(())
            }
            else{
                Err(Error::get_last_error())
            }
        }
    }
}

impl Drop for OpenGLRenderContext{
    fn drop(&mut self){
        unsafe{
            RenderContextFunctions::make_current(Some(self.window_context),None);
            RenderContextFunctions::delete_context(self.render_context);
            DeviceContextFunctions::release(self.window,self.window_context);
        }
    }
}

#[derive(Clone)]
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