//! # Многоканальный аудио движок. A multichannel audio engine. `feature = "audio"`.
//! 
//! Аудио движок имеет свой поток для работы со звуком.
//! Также в нём есть массив аудио треков, которые можно запустить.
//! 
//! Поддерживает только вывод.
//! Пока что позволяет декодировать только треки формата `mp3`.
//! Все треки переводятся в 24-битный формат.
//! 
//! 
//! Поток закрывается с паникой, так что не паникуте!
//! 
//! #
//! 
//! The audio engine has it's own thread to work with sound.
//! Also it has an audio track array.
//! 
//! Supports only output.
//! For now only 'mp3' format decoding is supported.
//! All tracks are converted to the 24-bit format.
//! 
//! The thread closes with panic, so don't panic!
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
pub use track::*;

mod sample;
use sample::SampleTransform;

mod wrapper;
pub use wrapper::AudioWrapper;

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
    SampleFormat
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
    /// Добавляет одноканальный трек во внутренний массив.
    /// 
    /// Adds a mono-channel track to the inner array.
    AddMono(MonoTrack),

    /// Убирает одноканальный трек из внутреннего массива.
    /// 
    /// Также убирает его из плейлиста.
    /// 
    /// Removes a mono-channel track from the inner array.
    /// 
    /// Removes it from the playlist, too.
    RemoveMono(usize),

    /// Plays a mono-channel track on the given channels.
    /// 
    /// A track index - usize,
    /// Channels - Vec<usize>,
    /// Repeats - u32:
    /// 0 - forever, 1 - once, 2.. - repeat twice and so on
    PlayMonoOnChannels(TrackSet),

    /// Plays mono-channel tracks on the given channels.
    /// 
    /// A track index - usize,
    /// Channels - Vec<usize>,
    /// Repeats - u32:
    /// 0 - forever, 1 - once, 2.. - repeat twice and so on
    PlayMonosOnChannels(Vec<TrackSet>),

    /// Убирает одноканальный трек из плейлиста.
    /// 
    /// Removes a mono-channel track from the playlist.
    RemoveMonoFromPlaylist(usize),

    /// Отчищает плейлист (список текущих играющих треков).
    /// 
    /// Clears a playlist (the list of currently playing tracks).
    ClearPlaylist,

    /// Устанавливает громкость треку в плейлисте.
    /// 
    /// Sets the volume to a track in the playlist.
    SetMonoVolume(usize,f32),

    /// Устанавливает общую громкость.
    /// 
    /// Sets the general volume.
    SetGeneralVolume(f32),
    Close,
}

/// Результат выполнения команды. The result of command accomplishing.
#[derive(Clone,Debug,PartialEq)]
pub enum AudioCommandResult{
    Ok,
    ThreadClosed,
    TrackError,
    TrackArrayOverflow,
    PlaylistOverflow,
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
    /// Общая громкость, применяемая прямо перед выводом.
    /// По умолчанию 0.5.
    ///
    /// Applies right before an output.
    /// The default is 0.5.
    pub general_volume:f32,

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
    pub track_playlist_capacity:usize,
}

impl AudioSettings{
    pub fn new()->AudioSettings{
        Self{
            general_volume:0.5f32,
            track_array_capacity:8,
            track_playlist_capacity:8,
        }
    }
}

/// Внутренние настройки системы.
pub (crate) struct AudioSystemSettings{
    pub general_volume:f32,
    pub output_channels:u16,
    pub format:Format,

    pub track_array_capacity:usize,
    pub track_playlist:usize,
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

    /// Количество треков во внутреннем буфере.
    track_array_len:usize,
    track_array_cap:usize,

    // Количество проигрываемых треков
    playlist_len:usize,
    playlist_cap:usize,
}

impl Audio{
    /// Строит аудио движок.
    /// 
    /// Возвращает результат создания аудио потока.
    /// 
    /// Creates an audio engine.
    /// 
    /// Returns the result of starting an audio thread.
    pub fn new<
        D:Fn(&Host)->Device+Send+Sync+'static,
        F:Fn(&Device)->Format+Send+Sync+'static,
    >(
        host:Host,
        choose_device:D,
        choose_format:F,
        settings:AudioSettings
    )->io::Result<Audio>{
        //
        let stream=Arc::new(Mutex::new(None));
        let s=stream.clone();

        let event_loop=Arc::new(host.event_loop());
        let el=event_loop.clone();
        // Канал для передачи команд от управляющего потока выполняющему
        let (sender,receiver)=channel::<AudioSystemCommand>();

        let owner_host=Arc::new(host);
        let host=owner_host.clone();

        let track_array_cap=settings.track_array_capacity;

        let playlist_cap=settings.track_playlist_capacity;

        let thread_result=Builder::new()
                .name("CatEngine's audio thread".to_string())
                .stack_size(audio_thread_stack_size)
                .spawn(move||{

            // Выполнение замыкания
            let mut device=choose_device(host.as_ref());

            let mut format=choose_format(&device);

            let main_stream=event_loop.build_output_stream(&device,&format).expect("stream");

            *stream.lock().unwrap()=Some(main_stream.clone());

            event_loop.play_stream(main_stream.clone()).unwrap();

            let system_settings=AudioSystemSettings{
                general_volume:settings.general_volume,
                output_channels:format.channels,
                format,

                track_array_capacity:settings.track_array_capacity,
                track_playlist:settings.track_playlist_capacity,
            };

            // Забирает контроль над потоком и начинает обработку аудио потоков
            // Takes control of the current thread and begins the stream processing
            event_loop_handler(
                host,
                //choose_device,
                stream,
                event_loop,
                receiver,
                system_settings,
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

            track_array_len:0usize,
            track_array_cap,

            playlist_len:0usize,
            playlist_cap,
        })
    }

    pub fn default(settings:AudioSettings)->io::Result<Audio>{
        let host=cpal::default_host();
        //
        let stream=Arc::new(Mutex::new(None));
        let s=stream.clone();

        let event_loop=Arc::new(host.event_loop());
        let el=event_loop.clone();
        // Канал для передачи команд от управляющего потока выполняющему
        let (sender,receiver)=channel::<AudioSystemCommand>();

        let owner_host=Arc::new(host);
        let host=owner_host.clone();

        let track_array_cap=settings.track_array_capacity;

        let playlist_cap=settings.track_playlist_capacity;

        let thread_result=Builder::new()
                .name("CatEngine's audio thread".to_string())
                .stack_size(audio_thread_stack_size)
                .spawn(move||{

            let mut device=host.default_output_device().expect("No available device");

            let mut format=device.default_output_format().expect("No available device");

            let main_stream=event_loop.build_output_stream(&device,&format).expect("stream");

            *stream.lock().unwrap()=Some(main_stream.clone());

            event_loop.play_stream(main_stream.clone()).unwrap();

            let system_settings=AudioSystemSettings{
                general_volume:settings.general_volume,
                output_channels:format.channels,
                format,

                track_array_capacity:settings.track_array_capacity,
                track_playlist:settings.track_playlist_capacity,
            };

            // Забирает контроль над потоком и начинает обработку аудио потоков
            // Takes control of the current thread and begins the stream processing
            event_loop_handler(
                host,
                stream,
                event_loop,
                receiver,
                system_settings,
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

            track_array_len:0usize,
            track_array_cap,

            playlist_len:0usize,
            playlist_cap,
        })
    }

    pub fn tracks_amount(&self)->usize{
        self.track_array_len
    }

    pub fn playlist_len(&self)->usize{
        self.playlist_len
    }

    /// Добавляет трек в массив треков.
    /// 
    /// Adds the track to the track array.
    pub fn add_track(&mut self,track:MonoTrack)->AudioCommandResult{
        if self.track_array_len<self.track_array_cap{
            match self.command.send(AudioSystemCommand::AddMono(track)){
                Ok(_)=>{
                    self.track_array_len+=1;
                    AudioCommandResult::Ok
                },
                Err(_)=>AudioCommandResult::ThreadClosed,
            }
        }
        else{
            AudioCommandResult::TrackArrayOverflow
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
    pub fn play_track(&self,set:TrackSet)->AudioCommandResult{
        if self.playlist_len<self.playlist_cap{
            let stream_lock=match self.stream.lock(){
                LockResult::Ok(lock)=>lock,
                LockResult::Err(_)=>return AudioCommandResult::ThreadClosed
            };

            // Отправка команды
            let result=match self.command.send(
                AudioSystemCommand::PlayMonoOnChannels(set)
            ){
                Ok(())=>AudioCommandResult::Ok,
                Err(_)=>return AudioCommandResult::ThreadClosed
            };

            if let Some(stream)=stream_lock.as_ref(){
                self.event_loop.play_stream(stream.clone());
            }
            result
        }
        else{
            AudioCommandResult::PlaylistOverflow
        }
    }

    /// Запускает треки.
    /// 
    /// 0 - постоянно, 1 - один раз, 2.. - повторить дважды и так далее
    /// 
    /// Plays tracks.
    /// 
    /// 0 - forever, 1 - once, 2.. - repeat twice and so on
    pub fn play_tracks(&self,sets:Vec<TrackSet>)->AudioCommandResult{
        if self.playlist_len+sets.len()<self.playlist_cap{
            let stream_lock=match self.stream.lock(){
                LockResult::Ok(lock)=>lock,
                LockResult::Err(_)=>return AudioCommandResult::ThreadClosed
            };

            // Отправка команды
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
        else{
            AudioCommandResult::PlaylistOverflow
        }
    }

    /// Запускает поток проигрывания.
    /// 
    /// Starts playing the output stream.
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

    /// Ставит на паузу поток проигрывания.
    /// 
    /// Pauses the output stream.
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

    /// Очищает весь плейлист.
    /// 
    /// Clears a playlist.
    pub fn clear_playlist(&self)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::ClearPlaylist){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>AudioCommandResult::ThreadClosed
        }
    }

    /// Устанавливет громкость играющего трека.
    /// 
    /// Sets the volume of a playing track.
    pub fn set_track_volume(&self,track:usize,volume:f32)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::SetGeneralVolume(volume)){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>AudioCommandResult::ThreadClosed
        }
    }

    /// Устанавливает громкость.
    /// 
    /// Sets the general volume.
    pub fn set_general_volume(&self,volume:f32)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::SetGeneralVolume(volume)){
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