#[cfg(all(target_os="windows",feature="windows"))]
use crate::windows::OpenGraphicsLibrary;



pub struct Renderbuffer{
    
}

impl Renderbuffer{
    pub const fn new()->Renderbuffer{
        Self{
            glGenRenderbuffers:0,
            glDeleteRenderbuffers:0,

            glBindFramebuffer:0,

            glFramebufferTexture:0,
            glFramebufferTextureLayer:0,
            glFramebufferTexture1D:0,
            glFramebufferTexture2D:0,
            glFramebufferTexture3D:0,

            glFramebufferRenderbuffer:0,

            glBlitFramebuffer:0,

            glCheckFramebufferStatus:0,
            glGetFramebufferAttachmentParameteriv:0,
            glIsFramebuffer:0,
        }
    }

    #[cfg(all(target_os="windows",feature="windows"))]
    pub fn load(&mut self,library:&OpenGraphicsLibrary){
        unsafe{
            self.glGenFramebuffers=transmute(library.get_proc_address("glGenFramebuffers\0"));
            self.glDeleteFramebuffers=transmute(library.get_proc_address("glDeleteFramebuffers\0"));

            self.glBindFramebuffer=transmute(library.get_proc_address("glBindFramebuffer\0"));

            self.glFramebufferTexture=transmute(library.get_proc_address("glFramebufferTexture\0"));
            self.glFramebufferTextureLayer=transmute(library.get_proc_address("glFramebufferTextureLayer\0"));
            self.glFramebufferTexture1D=transmute(library.get_proc_address("glFramebufferTexture1D\0"));
            self.glFramebufferTexture2D=transmute(library.get_proc_address("glFramebufferTexture2D\0"));
            self.glFramebufferTexture3D=transmute(library.get_proc_address("glFramebufferTexture3D\0"));

            self.glFramebufferRenderbuffer=transmute(library.get_proc_address("glFramebufferRenderbuffer\0"));

            self.glBlitFramebuffer=transmute(library.get_proc_address("glBlitFramebuffer\0"));

            self.glCheckFramebufferStatus=transmute(library.get_proc_address("glCheckFramebufferStatus\0"));
            self.glGetFramebufferAttachmentParameteriv=transmute(library.get_proc_address("glGetFramebufferAttachmentParameteriv\0"));
            self.glIsFramebuffer=transmute(library.get_proc_address("glIsFramebuffer\0"));
        }
    }
}