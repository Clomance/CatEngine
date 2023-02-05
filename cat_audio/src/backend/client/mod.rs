mod render;
pub use render::{
    AudioRenderClient,
    AudioRenderClientInterface,
};

use std::ptr::null_mut;

use winapi::{
    um::{
        audioclient::{
            IAudioClient,
            IAudioRenderClient,
            IID_IAudioRenderClient
        },
        audiosessiontypes::{
            AUDCLNT_SHAREMODE_EXCLUSIVE,
            AUDCLNT_SHAREMODE_SHARED,
            AUDCLNT_STREAMFLAGS_CROSSPROCESS,
            AUDCLNT_STREAMFLAGS_LOOPBACK,
            AUDCLNT_STREAMFLAGS_EVENTCALLBACK,
            AUDCLNT_STREAMFLAGS_NOPERSIST,
            AUDCLNT_STREAMFLAGS_RATEADJUST,
            AUDCLNT_SESSIONFLAGS_EXPIREWHENUNOWNED,
            AUDCLNT_SESSIONFLAGS_DISPLAY_HIDE,
            AUDCLNT_SESSIONFLAGS_DISPLAY_HIDEWHENEXPIRED
        }
    },
    shared::{
        mmreg::WAVEFORMATEX,
        guiddef::GUID
    },
    ctypes::c_void
};

use super::AudioClientError;



const AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM:u32=0x80000000;
const AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY:u32=0x08000000;



#[repr(u32)]
pub enum ShareMode{
    Exclusive=AUDCLNT_SHAREMODE_EXCLUSIVE,
    Shared=AUDCLNT_SHAREMODE_SHARED
}



/// https://learn.microsoft.com/en-us/windows/win32/coreaudio/audclnt-streamflags-xxx-constants
#[repr(u32)]
pub enum StreamFlag{
    /// The audio stream will be a member of a cross-process audio session.
    CrossProcess=AUDCLNT_STREAMFLAGS_CROSSPROCESS,

    /// The audio stream will operate in loopback mode.
    LoopBack=AUDCLNT_STREAMFLAGS_LOOPBACK,

    /// Processing of the audio buffer by the client will be event driven.
    EventCallback=AUDCLNT_STREAMFLAGS_EVENTCALLBACK,

    /// The volume and mute settings for an audio session will not persist across application restarts.
    NoPersist=AUDCLNT_STREAMFLAGS_NOPERSIST,

    /// This constant is new in Windows 7.
    /// The sample rate of the stream is adjusted to a rate specified by an application.
    RateAbjust=AUDCLNT_STREAMFLAGS_RATEADJUST,

    /// A channel matrixer and a sample rate converter are inserted as necessary
    /// to convert between the uncompressed format supplied to `AudioClient::initialize` and the audio engine mix format.
    AutoConvertPcm=AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM,

    /// When used with `StreamFlag::AutoConvertPcm`,
    /// a sample rate converter with better quality than the default conversion but with a higher performance cost is used.
    /// This should be used if the audio is ultimately intended to be heard by humans as opposed to other scenarios such as pumping silence or populating a meter.
    SrcDefaultQuality=AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY,

    /// The session expires when there are no associated streams and owning session control objects holding references.
    ExpireWhenUnowned=AUDCLNT_SESSIONFLAGS_EXPIREWHENUNOWNED,

    /// The volume control is hidden in the volume mixer user interface when the audio session is created.
    /// If the session associated with the stream already exists before `AudioClient::initialize` opens the stream,
    /// the volume control is displayed in the volume mixer.
    DisplayHide=AUDCLNT_SESSIONFLAGS_DISPLAY_HIDE,

    /// The volume control is hidden in the volume mixer user interface after the session expires.
    HideWhenExpired=AUDCLNT_SESSIONFLAGS_DISPLAY_HIDEWHENEXPIRED
}



pub struct StreamFlags{
    inner:u32
}

impl StreamFlags{
    pub fn new()->StreamFlags{
        Self{
            inner:0u32
        }
    }

    pub const fn set(mut self,flag:StreamFlag)->StreamFlags{
        self.inner|=flag as u32;
        self
    }

    pub const fn remove(mut self,flag:StreamFlag)->StreamFlags{
        self.inner&=!(flag as u32);
        self
    }
}



pub struct WaveFormat{
    pub ptr:*mut WAVEFORMATEX
}



pub struct AudioClientInterface{
    pub handle:*mut IAudioClient
}

impl AudioClientInterface{
    pub fn null()->AudioClientInterface{
        Self{
            handle:null_mut()
        }
    }

    pub fn get_mix_format(&self,format:&mut *mut WAVEFORMATEX)->i32{
        unsafe{
            (&mut *self.handle).GetMixFormat(format)
        }
    }

    pub fn initialize(
        &self,
        share_mode:ShareMode,
        stream_flags:StreamFlags,
        buffer_duration:i64,
        periodicity:i64,
        wave_format:*mut WAVEFORMATEX,
        session:Option<&GUID>
    )->i32{
        unsafe{
            (&mut *self.handle).Initialize(
                share_mode as u32,
                stream_flags.inner,
                buffer_duration,
                periodicity,
                wave_format,
                std::mem::transmute(session)
            )
        }
    }

    pub fn set_event_handle(&self,handle:*mut c_void)->i32{
        unsafe{
            (&mut *self.handle).SetEventHandle(handle)
        }
    }

    pub fn get_render_service(
        &self,
        client:&mut AudioRenderClientInterface,
    )->i32{
        unsafe{
            (&mut *self.handle).GetService(
                &IID_IAudioRenderClient,
                &mut client.handle as *mut *mut IAudioRenderClient as *mut *mut c_void
            )
        }
    }

    pub fn get_buffer_size(&self,frames:&mut u32)->i32{
        unsafe{
            (&mut *self.handle).GetBufferSize(frames)
        }
    }

    pub fn get_current_padding(&self,padding:&mut u32)->i32{
        unsafe{
            (&mut *self.handle).GetCurrentPadding(padding)
        }
    }

    pub fn start(&self)->i32{
        unsafe{
            (&mut *self.handle).Start()
        }
    }

    pub fn stop(&self)->i32{
        unsafe{
            (&mut *self.handle).Stop()
        }
    }

    fn release(&self)->u32{
        unsafe{
            (&mut *self.handle).Release()
        }
    }
}



pub struct AudioClient{
    pub interface:AudioClientInterface
}

impl AudioClient{
    pub fn get_mix_format(&self)->Result<WaveFormat,()>{
        let mut wave_format:*mut WAVEFORMATEX=null_mut();
        let result=self.interface.get_mix_format(&mut wave_format);

        if result!=0{
            return Err(());
        }

        Ok(
            WaveFormat{
                ptr:wave_format
            }
        )
    }

    pub fn initialize(
        &self,
        share_mode:ShareMode,
        stream_flags:StreamFlags,
        buffer_duration:i64,
        periodicity:i64,
        wave_format:*mut WAVEFORMATEX,
        session:Option<&GUID>
    )->Result<(),AudioClientError>{
        let result=self.interface.initialize(
            share_mode,
            stream_flags,
            buffer_duration,
            periodicity,
            wave_format,
            session
        );
        if result!=0{
            return Err(AudioClientError::new(result))
        }

        Ok(())
    }

    pub fn set_event_handle(&self,handle:*mut c_void)->Result<(),()>{
        let result=self.interface.set_event_handle(handle);
        if result!=0{
            return Err(())
        }

        Ok(())
    }

    pub fn get_render_service(&self)->Result<AudioRenderClient,()>{
        let mut render_client_iterface=AudioRenderClientInterface::null();
        let result=self.interface.get_render_service(&mut render_client_iterface);
        if result!=0{
            return Err(())
        }

        Ok(
            AudioRenderClient{
                interface:render_client_iterface
            }
        )
    }

    pub fn get_buffer_size(&self)->Result<u32,AudioClientError>{
        let mut buffer_size=0;
        let result=self.interface.get_buffer_size(&mut buffer_size);
        if result!=0{
            return Err(AudioClientError::new(result))
        }

        Ok(buffer_size)
    }

    pub fn get_current_padding(&self)->Result<u32,AudioClientError>{
        let mut padding=0;
        let result=self.interface.get_current_padding(&mut padding);
        if result!=0{
            return Err(AudioClientError::new(result))
        }

        Ok(padding)
    }

    pub fn start(&self)->Result<(),()>{
        let result=self.interface.start();
        if result!=0{
            return Err(())
        }

        Ok(())
    }
}

impl Drop for AudioClient{
    fn drop(&mut self){
        self.interface.release();
    }
}