mod context;
pub use context::{
    OpenGLRenderContext,
    OpenGLRenderContextAttributes,
};

use winapi::{
    shared::{
        minwindef::{
            PROC,
            HMODULE,
        },
    },
    um::{
        wingdi::wglGetProcAddress,
        libloaderapi::{
            LoadLibraryA,
            FreeLibrary,
            GetProcAddress,
        },
    }
};

use gl::{
    load_with,
};

pub struct OpenGraphicsLibrary{
    /// Для некоторых старых функций (начиная с OpenGL 1.1).
    /// Оставил тут для дальнейшей доработки и оптимизаций.
    /// Например, точечная загрузка нужных функций, а не всех сразу,
    /// и удаление их, если они больше не используются.
    module:HMODULE,
}

impl OpenGraphicsLibrary{
    pub fn new()->OpenGraphicsLibrary{
        let module=unsafe{LoadLibraryA("opengl32.dll\0".as_ptr() as *const i8)};

        Self{
            module,
        }
    }

    pub fn get_proc_address(&self,name:&str)->PROC{
        get_proc_address(self.module,name)
    }

    pub fn load_functions(&self){
        // Загрузка всех доступных функций
        load_with(|s|{
            let name=format!("{}\0",s);
            get_proc_address(self.module,&name) as *const _
        });
    }
}

pub fn get_proc_address(module:HMODULE,name:&str)->PROC{
    unsafe{
        let ptr=wglGetProcAddress(name.as_ptr() as *const i8);
        match ptr as usize {
            0 | 1 | 2 | 3 | usize::MAX=>GetProcAddress(module,name.as_ptr() as *const i8),
            _=>ptr,
        }
    }
}