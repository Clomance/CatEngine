//! # Простой аудио движок. A simple audio engine. `feature = "audio"`.
//! 
//! Аудио движок имеет свой поток для работы со звуком.
//! Также в нём есть массив аудио треков, которые можно запустить.
//! 
//! Использует только один канал для проигрывания треков
//! и только формат `mp3`.
//! 
//! Поддерживает Mono и Stereo 2.0.
//! 
//! Поток закрывается с паникой, так что не паникуте!
//! 
//! Некоторый код был взят из [rodio](https://github.com/RustAudio/rodio).
//! 
//! При переключении трека в режиме паузы, слышен странный звук.
//! 
//! #
//! 
//! The audio engine has it's own thread for working with sound.
//! Also it has audio track array.
//! 
//! Uses only one stream for playing tracks
//! and only `mp3` format.
//! 
//! The audio engine converts all sound tracks to the 24-bit format.
//! 
//! Supports Mono and Stereo 2.0.
//! 
//! The thread closes with panic, so don't panic!
//! 
//! Some code was taken from [rodio](https://github.com/RustAudio/rodio).
//! 
//! A strange sound appears when a track is switched in pause.
//! 
//! #
//! 
//! ```
//! let settings=AudioSettings::new();
//! 
//! let host=cpal::default_host();
//! 
//! let audio=Audio::new(host,|host|{
//!     host.default_output_device().unwrap()
//! },settings.clone()).unwrap();
//! 
//! audio.add_track("audio.mp3"); // a track index = 0
//! 
//! audio.play_track(0,1); // plays the track once
//! ```

// re-import
pub use cpal;

mod engine;
use engine::event_loop_handler;

mod channel_system;
use channel_system::*;

mod track_iterator;
use track_iterator::*;

mod track;
mod sample;

pub use track::*;
use sample::SampleTransform;

use cpal::{
    Host,
    HostId,
    HostUnavailable,
    Device,
    DevicesError,
    Devices,
    OutputDevices,
    traits::{
        HostTrait,
        DeviceTrait,
        EventLoopTrait
    },
    UnknownTypeOutputBuffer,
    StreamData,
    StreamId,
    EventLoop,
    Sample,
    Format,
    StreamError,
};

use std::{
    io,
    vec::IntoIter,
    iter::Cycle,
    path::Path,
    thread::{Builder,JoinHandle},
    sync::{
        Arc,
        Mutex,
        LockResult,
        mpsc::{Sender,Receiver,channel},
    },
};

const audio_thread_stack_size:usize=1024;

pub (crate) enum AudioSystemCommand{
    /// Adds a mono-channel track to the inner array.
    AddMono(MonoTrack),

    /// Plays a mono-channel track on the given channels.
    /// 
    /// A track index - usize,
    /// Channels - Vec<usize>,
    /// Repeats - u32:
    /// 0 - forever, 1 - once, 2.. - repeat twice and so on
    PlayMonoOnChannels(usize,Vec<usize>,u32),

    /// Plays mono-channel tracks on the given channels.
    /// 
    /// A track index - usize,
    /// Channels - Vec<usize>,
    /// Repeats - u32:
    /// 0 - forever, 1 - once, 2.. - repeat twice and so on
    PlayMonosOnChannels(Vec<(usize,Vec<usize>,u32)>),

    Stop,

    SetVolume(f32),
    Close,
}

/// Результат выполнения команды. The result of command accomplishing.
#[derive(Clone,Debug,PartialEq)]
pub enum AudioCommandResult{
    Ok,
    NoSuchTrack,
    ThreadClosed,
    TrackError,
}

impl AudioCommandResult{
    /// Паникует, если результат не `Ok`.
    /// 
    /// Panics if the result isn't `Ok`.
    pub fn unwrap(self){
        if self!=AudioCommandResult::Ok{
            panic!("{:?}",self)
        }
    }

    /// Паникует и выводит сообщение, если результат не `Ok`.
    /// 
    /// Panics и prints the message if the result isn't `Ok`.
    pub fn expect(self,msg:&str){
        if self!=AudioCommandResult::Ok{
            panic!("{} {:?}",msg,self)
        }
    }
}

unsafe impl std::marker::Sync for AudioSystemCommand{}
unsafe impl std::marker::Send for AudioSystemCommand{}


#[derive(Clone)]
pub struct AudioSettings{
    /// Громкость. По умолчанию 0.5.
    /// 
    /// The default is 0.5.
    pub volume:f32,

    /// The default is 2.
    pub audio_output_channels:u16,

    /// Вместимость массива для треков.
    /// 
    /// The capacity of the track array.
    /// 
    /// The default is 8.
    pub track_array_capacity:usize,

    /// Максимальное количество одновременно играющих треков.
    /// 
    /// The maximum amount of playing tracks at one time.
    /// 
    /// The default is 8.
    pub playing_track_array_capacity:usize,
}

impl AudioSettings{
    pub fn new()->AudioSettings{
        Self{
            volume:0.5f32,
            audio_output_channels:2u16,
            track_array_capacity:8,
            playing_track_array_capacity:8,
        }
    }
}


//     /\__/\
//    /`    '\
//   |  0  0  |
//  ===  --  ===
//   /        \
//  /          \
// |            |
//  \  ||  ||  /
//   \_oo__oo_/#######o
// I am watching you, Mister Programmer.

/// Простой аудио движок. A simple audio engine.
/// 
/// Пока только вывод доступен.
/// 
/// Only output is available now.
pub struct Audio{
    host:Arc<Host>,
    //device:Arc<Mutex<Device>>,
    stream:Arc<Mutex<Option<StreamId>>>,

    event_loop:Arc<EventLoop>,

    command:Sender<AudioSystemCommand>,
    thread:Option<JoinHandle<()>>,
}

impl Audio{
    /// Строит аудио движок.
    /// 
    /// Возвращает результат создания аудио потока.
    /// 
    /// Creates an audio engine.
    /// 
    /// Returns the result of starting an audio thread.
    pub fn new(//<D:Fn(&Host)->Device+Send+Sync+'static>(
        host:Host,
        //choose_device:D,
        settings:AudioSettings
    )->io::Result<Audio>{
        //
        let stream=Arc::new(Mutex::new(None));
        let s=stream.clone();

        let event_loop=Arc::new(host.event_loop());
        let el=event_loop.clone();
        // Передача команд от управляющего потока выполняющему
        let (sender,receiver)=channel::<AudioSystemCommand>();

        let owner_host=Arc::new(host);
        let host=owner_host.clone();

        let thread_result=Builder::new()
                .name("CatEngine's audio thread".to_string())
                .stack_size(audio_thread_stack_size)
                .spawn(move||{

            let mut device=host.default_output_device().expect("No available device");
            //let device=choose_device(&host);
            let mut format=device.default_output_format().expect("No available device");

            format.channels=settings.audio_output_channels;

            let main_stream=event_loop.build_output_stream(&device,&format).expect("stream");

            *stream.lock().unwrap()=Some(main_stream.clone());

            event_loop.play_stream(main_stream.clone()).unwrap();

            // Забирает контроль над потоком и начинает обработку аудио потоков
            // Takes control of the current thread and begins the stream processing
            event_loop_handler(
                host,
                //choose_device,
                stream,
                event_loop,
                format,
                settings.audio_output_channels,
                receiver,
                settings.volume,
            )
        });

        let thread=match thread_result{
            Ok(thread)=>thread,
            Err(e)=>return Err(e),
        };

        Ok(Self{
            host:owner_host,
            stream:s,

            event_loop:el,
            command:sender,
            thread:Some(thread),
        })
    }

    /// Добавляет трек в массив треков.
    /// 
    /// Adds the track to the track array.
    pub fn add_track(&self,track:MonoTrack)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::AddMono(track)){
            Ok(_)=>{
                AudioCommandResult::Ok
            },
            Err(_)=>AudioCommandResult::ThreadClosed,
        }
    }

    // /// Удаляет трек из массива треков.
    // /// 
    // /// Removes the track from the track array.
    // pub fn remove_track(&self,index:usize)->AudioCommandResult{
    //     let mut lock=match self.tracks.lock(){
    //         Ok(lock)=>lock,
    //         Err(_)=>return AudioCommandResult::ThreadClosed,
    //     };

    //     if index<lock.len(){
    //         lock.remove(index);
    //         AudioCommandResult::Ok
    //     }
    //     else{
    //         AudioCommandResult::NoSuchTrack
    //     }
    // }

    // /// Удаляет все треки из массива треков.
    // /// 
    // /// Removes all tracks from the track array.
    // pub fn remove_all_tracks(&self)->AudioCommandResult{
    //     self.tracks.lock().unwrap().clear();
    //     AudioCommandResult::Ok
    // }

    /// Запускает трек.
    /// 
    /// 0 - постоянно, 1 - один раз, 2.. - повторить дважды и так далее
    /// 
    /// Plays a track.
    /// 
    /// 0 - forever, 1 - once, 2.. - repeat twice and so on
    pub fn play_track(&self,index:usize,channels:&[usize],repeats:u32)->AudioCommandResult{
        let stream_lock=match self.stream.lock(){
            LockResult::Ok(lock)=>lock,
            LockResult::Err(_)=>return AudioCommandResult::ThreadClosed
        };

        let mut channels_vec=Vec::with_capacity(channels.len());
        for &c in channels{
            channels_vec.push(c)
        }

        let result=match self.command.send(
            AudioSystemCommand::PlayMonoOnChannels(index,channels_vec,repeats)
        ){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>return AudioCommandResult::ThreadClosed
        };

        if let Some(stream)=stream_lock.as_ref(){
            self.event_loop.play_stream(stream.clone());
        }

        result
    }

    /// Запускает треки.
    /// 
    /// 0 - постоянно, 1 - один раз, 2.. - повторить дважды и так далее
    /// 
    /// Plays tracks.
    /// 
    /// 0 - forever, 1 - once, 2.. - repeat twice and so on
    pub fn play_tracks(&self,sets:Vec<(usize,Vec<usize>,u32)>)->AudioCommandResult{
        let stream_lock=match self.stream.lock(){
            LockResult::Ok(lock)=>lock,
            LockResult::Err(_)=>return AudioCommandResult::ThreadClosed
        };

        let result=match self.command.send(
            AudioSystemCommand::PlayMonosOnChannels(sets)
        ){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>return AudioCommandResult::ThreadClosed
        };

        if let Some(stream)=stream_lock.as_ref(){
            self.event_loop.play_stream(stream.clone());
        }

        result
    }

    /// Запускает проигрывание канала.
    /// 
    /// Starts playing the stream.
    pub fn play(&self)->AudioCommandResult{
        let stream_lock=match self.stream.lock(){
            LockResult::Ok(stream)=>stream,
            LockResult::Err(_)=>return AudioCommandResult::ThreadClosed
        };

        if let Some(stream)=stream_lock.as_ref(){
            self.event_loop.play_stream(stream.clone());
        }

        AudioCommandResult::Ok
    }

    /// Ставит на паузу проигрывание канала.
    /// 
    /// Pauses the stream.
    pub fn pause(&self)->AudioCommandResult{
        let stream_lock=match self.stream.lock(){
            LockResult::Ok(stream)=>stream,
            LockResult::Err(_)=>return AudioCommandResult::ThreadClosed
        };

        if let Some(stream)=stream_lock.as_ref(){
            self.event_loop.pause_stream(stream.clone());
        }

        AudioCommandResult::Ok
    }

    /// Останавливает проигрывание путём удаления трека из буфера для вывода.
    /// 
    /// Stops playing by removing current track from the playing buffer.
    pub fn stop(&self)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::Stop){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>AudioCommandResult::ThreadClosed
        }
    }

    /// Устанавливает громкость.
    /// 
    /// Sets the volume.
    pub fn set_volume(&self,volume:f32)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::SetVolume(volume)){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>AudioCommandResult::ThreadClosed
        }
    }
}

/// Отправляет команду для остановки и ожидает окончание работы потока.
/// 
/// Sends a command to close and waits for the thread to finish.
impl Drop for Audio{
    fn drop(&mut self){
        let _=self.command.send(AudioSystemCommand::Close);

        if let Some(stream)=self.stream.lock().unwrap().as_ref(){
            self.event_loop.play_stream(stream.clone());
        }

        if let Some(thread)=self.thread.take(){
            let _=thread.join();
        }
    }
}