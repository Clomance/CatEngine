use crate::{
    OutputObject,
    SampleTransform,
    OutputBuffer,
    WaveFormat,
    source::SourceUnitReference
};

use self::stereo_object::{SourceIter, PlayType};

mod stereo_object;



pub struct SimpleStereoObject{
    _source:SourceUnitReference,
    left:SourceIter,
    right:SourceIter
}

impl SimpleStereoObject{
    pub fn new(repeat:u32,wave_format:&WaveFormat,source:SourceUnitReference)->SimpleStereoObject{
        let mut left=SourceIter::empty();
        let mut right=SourceIter::empty();

        left.set_track(source.source(),wave_format.channels as usize,0,wave_format.sample_rate,repeat,1f32);
        right.set_track(source.source(),wave_format.channels as usize,1,wave_format.sample_rate,repeat,1f32);

        Self{
            _source:source,
            left,
            right
        }
    }

    pub fn finished(&self)->bool{
        self.left.play_type()==PlayType::None
    }

    pub fn pause(&mut self){
        self.left.pause();
        self.right.pause();
    }

    pub fn unpause(&mut self){
        self.left.unpause();
        self.right.unpause();
    }

    pub fn stop(&mut self){
        self.left.stop();
        self.right.stop();
    }
}

impl OutputObject for SimpleStereoObject{
    fn render(&mut self,output:&mut crate::OutputBuffer){
        match output{
            OutputBuffer::I16(buffer)=>{
                for frame in buffer.chunks_exact_mut(2){
                    if let Some(left)=self.left.next(){
                        frame[0]=left.into_i16();
                    }
                    else{
                        frame[0]=0i16
                    }
                    if let Some(right)=self.right.next(){
                        frame[1]=right.into_i16();
                    }
                    else{
                        frame[1]=0i16
                    }
                }
            }

            OutputBuffer::U16(buffer)=>{
                for frame in buffer.chunks_exact_mut(2){
                    if let Some(left)=self.left.next(){
                        frame[0]=left.into_u16();
                    }
                    else{
                        frame[0]=0u16
                    }
                    if let Some(right)=self.right.next(){
                        frame[1]=right.into_u16();
                    }
                    else{
                        frame[1]=0u16
                    }
                }
            }

            OutputBuffer::F32(buffer)=>{
                for frame in buffer.chunks_exact_mut(2){
                    if let Some(left)=self.left.next(){
                        frame[0]=left.into_f32();
                    }
                    else{
                        frame[0]=0f32
                    }
                    if let Some(right)=self.right.next(){
                        frame[1]=right.into_f32();
                    }
                    else{
                        frame[1]=0f32
                    }
                }
            }
        }
    }
}



// pub trait SimpleAudioSystem<'s>{
//     type Objects:'s;

//     fn set_up(&mut self,manager:AudioSystemManager<'s>)->Self::Objects;

//     fn render(&mut self,objects:&mut Self::Objects)->AudioSystemStatus;
// }



// pub struct SimpleSystem;

// impl AudioSystem for SimpleSystem{
//     fn error(&mut self,error:AudioClientError,stream:&mut OutputStream)->AudioSystemStatus{
//         let AudioClientError::None=error else{
//             *stream=OutputStream::new().unwrap();
//             return AudioSystemStatus::Processed
//         };

//         AudioSystemStatus::Processed
//     }
// }


// pub struct SimpleAudio{
//     audio_system:*mut (),
//     system_objects:*mut (),
//     core:AudioCore<SimpleSystem>,
//     sounds:HashMap<String,usize>
// }

// impl SimpleAudio{
//     pub fn new<'s,S:SimpleAudioSystem<'s>>(mut audio_system:S)->SimpleAudio{
//         let mut core=AudioCore::default(SimpleSystem);
//         core.push_object_storage(std::ptr::null_mut());

//         let manager=core.object_manager(0);
//         let system_objects=audio_system.set_up(manager);

//         Self{
//             audio_system,
//             system_objects,

//             core,
//             sounds:HashMap::new()
//         }
//     }

//     pub fn add_sound<N:ToString>(&mut self,name:N,sound:Source)->bool{
//         if let Some(id)=self.core.add_source(sound){
//             self.sounds.insert(name.to_string(),id);
//             true
//         }
//         else{
//             false
//         }
//     }

//     pub fn remove_sound(&mut self,name:&str)->Option<Source>{
//         if let Some(id)=self.sounds.get(name){
//             if let Some(sound)=self.core.remove_source(*id){
//                 self.sounds.remove(name);
//                 return Some(sound)
//             }
//         }

//         None
//     }

//     pub fn play(&mut self,name:&str,repeat:u32)->Option<ObjectReference<SimpleStereoObject>>{
//         if let Some(id)=self.sounds.get(name){
//             let source=self.core.get_source(*id).unwrap();
//             let object=SimpleStereoObject::new(repeat,self.core.wave_format(),source);
//             return unsafe{
//                 std::mem::transmute(self.core.object_manager(0).push_object(object))
//             }
//         }

//         None
//     }

//     pub fn run(&mut self){
//         loop{
//             std::thread::sleep(std::time::Duration::from_millis(500));
//             self.core.render();
//             match self.audio_system.render(&mut self.system_objects){
//                 AudioSystemStatus::Processed=>{}
//                 AudioSystemStatus::Exit=>break,
//                 AudioSystemStatus::Panic=>panic!()
//             }
//         }
//     }
// }