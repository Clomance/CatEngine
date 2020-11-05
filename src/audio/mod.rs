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
//! Больше вы сможете узнать из [книги](https://github.com/Clomance/CatEngine/blob/master/book/README-RUS.MD).
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
//! You can learn more from the [book](https://github.com/Clomance/CatEngine/blob/master/book/README.MD).
//! 
//! #
//! 
//! ```
//! let settings=AudioSettings::new();
//! 
//! let audio=Audio::default(settings.clone()).unwrap();
//! 
//! // For easier access to the audio engine
//! let mut wrapper=AudioWrapper::new(audio);
//! 
//! wrapper.load_track("audio.mp3","test".to_string());
//! 
//! wrapper.play_track("test").unwrap();
//! ```

// re-export
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

/// Команды аудио системы.
/// 
/// Audio system commands.
pub (crate) enum AudioSystemCommand{
    /// Добавляет одноканальный трек в хранилище.
    /// 
    /// Adds a mono-channel track to the storage.
    AddMono(MonoTrack),

    /// Добавляет одноканальные треки в хранилище.
    /// 
    /// Adds mono-channel tracks to the storage.
    AddMonos(Vec<MonoTrack>),

    /// Убирает одноканальный трек из хранилища.
    /// 
    /// Removes a mono-channel track from the storage.
    RemoveMono(usize),

    /// Убирает одноканальные треки из хранилища.
    /// 
    /// Removes a mono-channel track from the storage.
    RemoveMonos(Vec<usize>),

    /// Очищает хранилище.
    /// 
    /// Clears the storage.
    ClearStorage,

    /// Проигрывает одноканальный трек на данных каналах.
    /// 
    /// Plays a mono-channel track on the given channels.
    PlayMonoOnChannels(TrackSet),

    /// Проигрывает одноканальные треки на данных каналах.
    /// 
    /// Plays mono-channel tracks on the given channels.
    PlayMonosOnChannels(Vec<TrackSet>),

    /// Ставит трек проигрываться.
    /// 
    /// Если уже проигрывается, ничего не происходит.
    PlayMono(usize),

    PlayMonos(Vec<usize>),

    /// Ставит трек на паузу.
    /// 
    /// Если уже на паузе, ничего не происходит.
    PauseMono(usize),

    PauseMonos(Vec<usize>),

    /// Убирает одноканальный трек из плейлиста.
    /// 
    /// Removes a mono-channel track from the playlist.
    RemoveMonoFromPlaylist(usize),

    RemoveMonosFromPlaylist(Vec<usize>),

    /// Отчищает плейлист (список текущих играющих треков).
    /// 
    /// Clears a playlist (the list of currently playing tracks).
    ClearPlaylist,

    /// Устанавливает громкость трека в плейлисте.
    /// 
    /// Sets a volume to a track in the playlist.
    SetMonoVolume(usize,f32),

    /// Устанавливает громкость треков в плейлисте.
    /// 
    /// Sets a volume to tracks in the playlist.
    SetMonosVolume(Vec<usize>,f32),

    /// Устанавливает громкости треков в плейлисте.
    /// 
    /// Sets volumes to tracks in the playlist.
    SetMonosVolumes(Vec<(usize,f32)>),

    /// Устанавливает общую громкость.
    /// 
    /// Sets the general volume.
    SetGeneralVolume(f32),

    /// Закрывает аудио поток.
    /// 
    /// Closes the audio thead.
    Close,
}

/// Результат выполнения команды. The result of command accomplishing.
#[derive(Clone,Debug,PartialEq)]
pub enum AudioCommandResult{
    Ok,
    ThreadClosed,
    TrackArrayIsEmpty,
    TrackArrayOverflow,
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
        })
    }

    /// Строит аудио движок с настройками по умолчанию.
    /// 
    /// Возвращает результат создания аудио потока.
    /// 
    /// Creates an audio engine with default settings.
    /// 
    /// Returns the result of starting an audio thread.
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

            let main_stream=event_loop.build_output_stream(&device,&format).expect("No available device");

            *stream.lock().expect("The audio thread is down")=Some(main_stream.clone());

            event_loop.play_stream(main_stream.clone()).expect("No available device");

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
        })
    }

    /// Возвращает количество треков
    /// во внутреннем массиве.
    /// 
    /// Returns the amount of track
    /// in the inner array.
    pub fn tracks_amount(&self)->usize{
        self.track_array_len
    }
}

/// Добавление/удаление треков.
/// 
/// Adding/removing tracks.
impl Audio{
    /// Добавляет трек в хранилище.
    /// 
    /// Adds the track to the storage.
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

    /// Добавляет треки в хранилище.
    /// 
    /// Adds tracks to the storage.
    pub fn add_tracks(&mut self,tracks:Vec<MonoTrack>)->AudioCommandResult{
        let len=tracks.len();
        if self.track_array_len+len<self.track_array_cap{
            match self.command.send(AudioSystemCommand::AddMonos(tracks)){
                Ok(_)=>{
                    self.track_array_len+=len;
                    AudioCommandResult::Ok
                },
                Err(_)=>AudioCommandResult::ThreadClosed,
            }
        }
        else{
            AudioCommandResult::TrackArrayOverflow
        }
    }

    /// Удаляет трек из хранилища.
    /// 
    /// Если такого трека нет, то ничего не происходит.
    /// 
    /// Removes the track from the storage.
    /// 
    /// If there are no such track, nothing happens.
    pub fn remove_track(&mut self,index:usize)->AudioCommandResult{
        if self.track_array_len>0{
            match self.command.send(AudioSystemCommand::RemoveMono(index)){
                Ok(_)=>{
                    self.track_array_len-=1;
                    AudioCommandResult::Ok
                },
                Err(_)=>AudioCommandResult::ThreadClosed,
            }
        }
        else{
            AudioCommandResult::TrackArrayIsEmpty
        }
    }

    /// Удаляет трек из хранилища.
    /// 
    /// Если такого трека нет, то ничего не происходит.
    /// 
    /// Removes tracks from the storage.
    /// 
    /// If there are no such tracks, nothing happens.
    pub fn remove_tracks(&mut self,indices:Vec<usize>)->AudioCommandResult{
        let len=indices.len();
        if self.track_array_len>=len{
            match self.command.send(AudioSystemCommand::RemoveMonos(indices)){
                Ok(_)=>{
                    self.track_array_len-=len;
                    AudioCommandResult::Ok
                },
                Err(_)=>AudioCommandResult::ThreadClosed,
            }
        }
        else{
            AudioCommandResult::TrackArrayIsEmpty
        }
    }

    /// Очищает хранилища.
    /// 
    /// Clears the storage.
    pub fn clear_track_array(&mut self)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::ClearStorage){
            Ok(_)=>{
                self.track_array_len=0;
                AudioCommandResult::Ok
            },
            Err(_)=>AudioCommandResult::ThreadClosed,
        }
    }
}

/// Проигрывание треков.
/// 
/// Play tracks.
impl Audio{
    /// Проигрывает трек.
    /// 
    /// Plays a track.
    pub fn play_track(&self,set:TrackSet)->AudioCommandResult{
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
     
    /// Останаливает трек.
    /// 
    /// Stops a track.
    pub fn stop_track(&self,index:usize)->AudioCommandResult{
        match self.command.send(
            AudioSystemCommand::RemoveMonoFromPlaylist(index)
        ){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>return AudioCommandResult::ThreadClosed
        }
    }

    /// Запускает треки.
    /// 
    /// Plays tracks.
    pub fn play_tracks(&self,sets:Vec<TrackSet>)->AudioCommandResult{
        // Проверка размера плейлиста
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

    /// Останавливает треки.
    /// 
    /// Stops tracks.
    pub fn stop_tracks(&self,indices:Vec<usize>)->AudioCommandResult{
        match self.command.send(
            AudioSystemCommand::RemoveMonosFromPlaylist(indices)
        ){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>return AudioCommandResult::ThreadClosed
        }
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

    /// Ставит трек проигрываться.
    /// 
    /// Если уже проигрывается
    /// или такого трека нет, ничего не происходит.
    /// 
    /// Unpauses a track.
    /// 
    /// If it's already playing or
    /// there is no such track, nothing happens.
    pub fn unpause_track(&self,index:usize)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::PlayMono(index)){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>return AudioCommandResult::ThreadClosed
        }
    }

    /// Ставит треки проигрываться.
    /// 
    /// Если уже проигрываются
    /// или таких треков нет, ничего не происходит.
    /// 
    /// Unpauses tracks.
    /// 
    /// If they're already playing or
    /// there are no such tracks, nothing happens.
    pub fn unpause_tracks(&self,indices:Vec<usize>)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::PlayMonos(indices)){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>return AudioCommandResult::ThreadClosed
        }
    }

    /// Ставит трек на паузу.
    /// 
    /// Если уже на паузе
    /// или такого трека нет, ничего не происходит.
    /// 
    /// Pauses a track.
    /// 
    /// If it's already paused or
    /// there is no such track, nothing happens.
    pub fn pause_track(&self,index:usize)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::PauseMono(index)){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>return AudioCommandResult::ThreadClosed
        }
    }

    /// Ставит треки на паузу.
    /// 
    /// Если уже на паузе
    /// или таких треков нет, ничего не происходит.
    /// 
    /// Pauses tracks.
    /// 
    /// If trey're already paused or
    /// there are no such tracks, nothing happens.
    pub fn pause_tracks(&self,indices:Vec<usize>)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::PauseMonos(indices)){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>return AudioCommandResult::ThreadClosed
        }
    }
}

/// Функции установки параметров.
/// 
/// Setting functions.
impl Audio{
    /// Устанавливет громкость играющего трека.
    /// 
    /// Sets the volume of a playing track.
    pub fn set_track_volume(&self,index:usize,volume:f32)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::SetMonoVolume(index,volume)){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>AudioCommandResult::ThreadClosed
        }
    }

    /// Устанавливает общую громкость.
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