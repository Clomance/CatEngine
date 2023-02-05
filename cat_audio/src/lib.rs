mod backend;

use backend::{
    client::{
        ShareMode,
        AudioClient,
        AudioRenderClient,
        StreamFlags,
    },
    host::{
        DeviceRole,
        Host,
        DeviceEnumerator
    },
};

pub use backend::{
    AudioClientError
};

mod simple;
pub use simple::{
    // SimpleAudio,
    // SimpleAudioSystem,
    SimpleStereoObject,
};

mod sample;
pub use sample::SampleTransform;

mod source;
use source::{
    Sources
};
pub use source::{
    Source,
    SourceData,
    SourceSample,
    ResourceManager,
    SourceUnitReference,
};

mod system;

mod object;
pub use object::{
    ObjectReference,
    ArrayReference,
    OutputObject,
    Objects,
    ObjectStorage,
    ObjectManager,
    SystemObjectManager,
};

use winapi::{
    ctypes::c_void,
    um::{
        combaseapi::{
            CoTaskMemFree,
        },
    }
};



pub enum OutputBuffer<'a>{
    F32(&'a mut [f32]),
    I16(&'a mut [i16]),
    U16(&'a mut [u16])
}



#[derive(Debug,Clone)]
pub struct WaveFormat{
    pub channels:u16,
    pub sample_bits:u16,
    pub sample_rate:u32,
}



pub struct OutputStream{
    pub wave_format:WaveFormat,
    pub client:AudioClient,
    pub render_client:AudioRenderClient,
}

impl OutputStream{
    pub fn new()->Result<OutputStream,AudioClientError>{
        let device_enumerator=DeviceEnumerator::new().unwrap();

        let device=device_enumerator.default_render_device(DeviceRole::Multimedia).unwrap();

        let client=device.activate().unwrap();

        let wave_format=client.get_mix_format().unwrap();

        let format=unsafe{*wave_format.ptr};

        let format=WaveFormat{
            sample_bits:format.wBitsPerSample,
            sample_rate:format.nSamplesPerSec,
            channels:format.nChannels
        };

        let flags=StreamFlags::new();
        client.initialize(ShareMode::Shared,flags,10000000,0,wave_format.ptr,None).unwrap();

        unsafe{CoTaskMemFree(wave_format.ptr as *mut c_void);}

        let render_client=client.get_render_service().unwrap();

        client.start().unwrap();

        Ok(
            Self{
                wave_format:format,
                client,
                render_client,
            }
        )
    }

    pub fn wave_format(&self)->&WaveFormat{
        &self.wave_format
    }

    pub fn render(&mut self,objects:&mut ObjectStorage)->Result<(),AudioClientError>{
        let buffer_size=self.client.get_buffer_size()?;

        let padding=self.client.get_current_padding()?;
        let frames=buffer_size-padding;

        let buffer=self.render_client.get_buffer::<f32>(frames,self.wave_format.channels as u32)?;

        let mut output=OutputBuffer::F32(buffer);

        objects.render(&mut output);

        self.render_client.release_buffer(frames,0)?;

        Ok(())
    }
}



pub enum AudioSystemStatus{
    Processed,
    Exit,
    Panic
}



pub trait AudioSystem{
    fn error(&mut self,error:AudioClientError,stream:&mut OutputStream)->AudioSystemStatus;
}



pub struct AudioManager<'m>{
    pub stream:&'m mut OutputStream,
    pub sources:ResourceManager<'m>
}



pub struct AudioCoreManager<'m>{
    pub stream:&'m mut OutputStream,
    pub sources:ResourceManager<'m>,
    pub objects:ObjectManager<'m>,
}



pub struct AudioCore<S:AudioSystem>{
    _host:Host,

    stream:OutputStream,

    manager:S,

    sources:Sources,
    objects:Objects
}

impl<S:AudioSystem> AudioCore<S>{
    pub fn default(manager:S)->AudioCore<S>{
        let host=Host::initialize().unwrap();

        let stream=OutputStream::new().unwrap();

        Self{
            _host:host,

            stream,

            manager,

            sources:Sources::new(10),
            objects:Objects::new()
        }
    }

    pub fn wave_format(&self)->&WaveFormat{
        &self.stream.wave_format
    }

    pub fn core_manager(&mut self)->AudioCoreManager{
        AudioCoreManager{
            stream:&mut self.stream,
            sources:self.sources.manager(),
            objects:self.objects.manager()
        }
    }

    pub fn render(&mut self)->AudioSystemStatus{
        for storage in &mut self.objects.storages{
            if let Err(error)=self.stream.render(storage){
                return self.manager.error(error,&mut self.stream)
            }
        }

        AudioSystemStatus::Processed
    }
}