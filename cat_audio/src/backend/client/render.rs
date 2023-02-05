use crate::backend::AudioClientError;

use std::ptr::null_mut;

use winapi::{
    um::audioclient::{
        IAudioRenderClient,
    },
};



pub struct AudioRenderClientInterface{
    pub handle:*mut IAudioRenderClient,
}

impl AudioRenderClientInterface{
    pub const  fn null()->AudioRenderClientInterface{
        Self{
            handle:null_mut()
        }
    }

    #[inline]
    pub fn get_buffer(&self,frames:u32,buffer:&mut *mut u8)->i32{
        unsafe{
            (&mut *self.handle).GetBuffer(frames,buffer)
        }
    }

    #[inline]
    pub fn release_buffer(&self,frames:u32,flags:u32)->i32{
        unsafe{
            (&mut *self.handle).ReleaseBuffer(frames,flags)
        }
    }

    #[inline]
    fn release(&self)->u32{
        unsafe{
            (&mut *self.handle).Release()
        }
    }
}



pub struct AudioRenderClient{
    pub interface:AudioRenderClientInterface
}

impl AudioRenderClient{
    /// https://learn.microsoft.com/en-us/windows/win32/api/audioclient/nf-audioclient-iaudiorenderclient-getbuffer
    pub fn get_buffer<S>(&self,frames:u32,channels:u32)->Result<&mut [S],AudioClientError>{
        unsafe{
            let mut buffer:*mut u8=null_mut();
            let result=self.interface.get_buffer(frames,&mut buffer);

            let buffer=buffer as *mut S;

            if result!=0{
                return Err(AudioClientError::new(result))
            }
            let buffer=std::slice::from_raw_parts_mut(buffer,(channels*frames) as usize);

            Ok(buffer)
        }
    }

    /// https://learn.microsoft.com/en-us/windows/win32/api/audioclient/nf-audioclient-iaudiorenderclient-releasebuffer
    pub fn release_buffer(&self,frames:u32,flags:u32)->Result<(),AudioClientError>{
        let result=self.interface.release_buffer(frames,flags);
        if result!=0{
            return Err(AudioClientError::new(result))
        }

        Ok(())
    }
}

impl Drop for AudioRenderClient{
    fn drop(&mut self){
        self.interface.release();
    }
}