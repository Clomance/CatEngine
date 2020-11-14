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
        mpsc::{Sender,Receiver,channel,SendError},
    },
};

const audio_thread_stack_size:usize=1024;

/// Команды аудио системы.
/// 
/// Audio system commands.
pub (crate) enum AudioSystemCommand{
// ХРАНИЛИЩЕ \\
    /// Добавляет одноканальный трек в хранилище.
    /// 
    /// Если недостаточно места, то ничего не происходит.
    /// 
    /// Adds a mono-channel track to the storage.
    /// 
    /// If there is not enough space, nothing happens.
    AddMono(MonoTrack),

    /// Добавляет несколько одноканальных треков в хранилище.
    /// 
    /// Если недостаточно места, то ничего не происходит.
    /// 
    /// Adds some mono-channel tracks to the storage.
    /// 
    /// If there is not enough space, nothing happens.
    AddMonos(Vec<MonoTrack>),

    /// Убирает одноканальный трек из хранилища.
    /// 
    /// Если нет такого трека, то ничего не происходит.
    /// 
    /// Removes a mono-channel track from the storage.
    /// 
    /// If there is no such track, nothing happens.
    RemoveMono(usize),

    /// Убирает несколько одноканальных треков из хранилища.
    /// 
    /// Если нет таких треков, то ничего не происходит.
    /// 
    /// Removes some mono-channel tracks from the storage.
    /// 
    /// If there are no such tracks, nothing happens.
    RemoveMonos(Vec<usize>),

    /// Очищает хранилище.
    /// 
    /// Clears the storage.
    ClearStorage,

    /// Снимает паузу с треков из плейлиста,
    /// привязанных к треку из хранилища.
    /// 
    /// Unpauses tracks from the playlist
    /// attached to a track from the storage.
    UnpauseMonoFromStorage(usize),

    /// Снимает паузу с треков из плейлиста,
    /// привязанных к трекам из хранилища.
    /// 
    /// Unpauses tracks from the playlist
    /// attached to tracks from the storage.
    UnpauseMonosFromStorage(Vec<usize>),

    /// Ставит на паузу треки из плейлиста,
    /// привязанные к треку из хранилища.
    /// 
    /// Pauses tracks from the playlist
    /// attached to a track from the storage.
    PauseMonoFromStorage(usize),

    /// Ставит на паузу треки из плейлиста,
    /// привязанные к трекам из хранилища.
    /// 
    /// Stops tracks from the playlist
    /// attached to tracks from the storage.
    PauseMonosFromStorage(Vec<usize>),

    /// Останавливает треки из плейлиста,
    /// привязанные к треку из хранилища.
    /// 
    /// Stops tracks from the playlist
    /// attached to a track from the storage.
    StopMonoFromStorage(usize),

    /// Останавливает треки из плейлиста,
    /// привязанные к трекам из хранилища.
    /// 
    /// Stops tracks from the playlist
    /// attached to tracks from the storage.
    StopMonosFromStorage(Vec<usize>),

    /// Устанавливает громкость треков из плейлиста,
    /// привязанных к треку из хранилища.
    /// 
    /// Sets a volume of tracks from the playlist
    /// attached to a track from the storage.
    SetMonoVolumeFromStorage(usize,f32),

    /// Устанавливает громкость треков из плейлиста,
    /// привязанных к трекам из хранилища.
    /// 
    /// Sets a volume of tracks from the playlist
    /// attached to tracks from the storage.
    SetMonosVolumeFromStorage(Vec<usize>,f32),

    /// Устанавливает громкости треков из плейлиста,
    /// привязанных к трекам из хранилища.
    /// 
    /// Sets volumes of tracks from the playlist
    /// attached to tracks from the storage.
    SetMonosVolumesFromStorage(Vec<(usize,f32)>),

// ПЛЕЙЛИСТ \\
    /// Проигрывает одноканальный трек на данных каналах.
    /// 
    /// Plays a mono-channel track on the given channels.
    PlayMonoOnChannels(TrackSet),

    /// Проигрывает несколько одноканальных треков на данных каналах.
    /// 
    /// Plays some mono-channel tracks on the given channels.
    PlayMonosOnChannels(Vec<TrackSet>),

    /// Снимает с паузы трек из плейлиста.
    /// 
    /// Если уже проигрывается, ничего не происходит.
    UnpauseMonoFromPlaylist(usize),

    /// Снимает с паузы трек из плейлиста.
    /// 
    /// Если уже проигрывается, ничего не происходит.
    UnpauseMonosFromPlaylist(Vec<usize>),

    /// Ставит трек из плейлиста на паузу.
    /// 
    /// Если уже на паузе, ничего не происходит.
    PauseMonoFromPlaylist(usize),

    PauseMonosFromPlaylist(Vec<usize>),

    /// Убирает одноканальный трек из плейлиста.
    /// 
    /// Removes a mono-channel track from the playlist.
    RemoveMonoFromPlaylist(usize),

    RemoveMonosFromPlaylist(Vec<usize>),

    /// Отчищает плейлист (список текущих играющих треков).
    /// 
    /// Clears a playlist (the list of currently playing tracks).
    ClearPlaylist,

// Параметры \\
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

// Остальное \\
    /// Закрывает аудио поток.
    /// 
    /// Closes the audio thead.
    Close,
}

/// Результат выполнения команды. The result of command accomplishing.
#[derive(Clone,Debug,PartialEq)]
pub enum AudioCommandResult{
    /// Команда отправлена.
    /// 
    /// A command is sent.
    Sent,
    /// Аудио поток остановлен.
    /// 
    /// The audio thread is closed.
    ThreadClosed,
    /// Хранилище треков переполнено.
    /// 
    /// The track storage is overflown.
    StorageOverflow,
    // /// Хранилище треков пусто.
    // /// 
    // /// The track storage is empty.
    // StorageIsEmpty,

    /// Нет такого трека (в хранилище или плейлисте).
    /// 
    /// No such track (in the storage or playlist).
    NoSuchTrack,
}

impl AudioCommandResult{
    /// Паникует, если результат не `Ok`.
    /// 
    /// Panics if the result isn't `Ok`.
    pub fn unwrap(self){
        if self!=AudioCommandResult::Sent{
            panic!("{:?}",self)
        }
    }

    /// Паникует и выводит сообщение, если результат не `Ok`.
    /// 
    /// Panics и prints the message if the result isn't `Ok`.
    pub fn expect(self,msg:&str){
        if self!=AudioCommandResult::Sent{
            panic!("{} {:?}",msg,self)
        }
    }
}

unsafe impl std::marker::Sync for AudioSystemCommand{}
unsafe impl std::marker::Send for AudioSystemCommand{}


#[derive(Clone)]
pub struct AudioSettings{
    /// Общая громкость.
    /// 
    /// По умолчанию 0.5.
    /// 
    /// The general volume.
    /// 
    /// The default is 0.5.
    pub general_volume:f32,

    /// Вместимость хранилища треков.
    /// 
    /// По умолчанию 8.
    /// 
    /// The capacity of the track storage.
    /// 
    /// The default is 8.
    pub track_array_capacity:usize,

    /// Максимальное количество одновременно играющих треков.
    /// 
    /// По умолчанию 8.
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

    fn send_command(&self,command:AudioSystemCommand)
            ->Result<(),SendError<AudioSystemCommand>>{
        self.command.send(command)
    }
}

/// Добавление/удаление треков.
/// 
/// Adding/removing tracks.
impl Audio{
    /// Добавляет трек в хранилище.
    /// 
    /// Если недостаточно места, возвращает `AudioCommandResult::StorageOverflow`.
    /// 
    /// Adds a track to the storage.
    /// 
    /// If there is not enough space, returns `AudioCommandResult::StorageOverflow`.
    pub fn add_track(&mut self,track:MonoTrack)->AudioCommandResult{
        if self.track_array_len<self.track_array_cap{
            match self.command.send(AudioSystemCommand::AddMono(track)){
                Ok(_)=>{
                    self.track_array_len+=1;
                    AudioCommandResult::Sent
                },
                Err(_)=>AudioCommandResult::ThreadClosed,
            }
        }
        else{
            AudioCommandResult::StorageOverflow
        }
    }

    /// Добавляет несколько треков в хранилище.
    /// 
    /// Если недостаточно места, возвращает `AudioCommandResult::StorageOverflow`.
    /// 
    /// Adds some tracks to the storage.
    /// 
    /// If there is not enough space, returns `AudioCommandResult::StorageOverflow`.
    pub fn add_tracks(&mut self,tracks:Vec<MonoTrack>)->AudioCommandResult{
        let len=tracks.len();
        if self.track_array_len+len<self.track_array_cap{
            match self.command.send(AudioSystemCommand::AddMonos(tracks)){
                Ok(_)=>{
                    self.track_array_len+=len;
                    AudioCommandResult::Sent
                },
                Err(_)=>AudioCommandResult::ThreadClosed,
            }
        }
        else{
            AudioCommandResult::StorageOverflow
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
            // Казна пустеет, милорд!
            match self.command.send(AudioSystemCommand::RemoveMono(index)){
                Ok(_)=>{
                    self.track_array_len-=1;
                    AudioCommandResult::Sent
                },
                Err(_)=>AudioCommandResult::ThreadClosed,
            }
        }
        else{
            AudioCommandResult::NoSuchTrack
        }
    }

    /// Удаляет трек из хранилища.
    /// 
    /// Если нет таких треков, то ничего не происходит.
    /// Если размер `indices` больше, чем треков в хранилище,
    /// то возвращает `AudioCommandResult::NoSuchTrack`.
    /// 
    /// Removes tracks from the storage.
    /// 
    /// If there are no such tracks, nothing happens.
    /// If the size of `indices` is greater than
    /// the amount of tracks in the storage,
    /// then returns AudioCommandResult::NoSuchTrack`.
    pub fn remove_tracks(&mut self,indices:Vec<usize>)->AudioCommandResult{
        let len=indices.len();
        // Казна пустеет, милорд!
        if self.track_array_len>=len{
            match self.command.send(AudioSystemCommand::RemoveMonos(indices)){
                Ok(_)=>{
                    self.track_array_len-=len;
                    AudioCommandResult::Sent
                },
                Err(_)=>AudioCommandResult::ThreadClosed,
            }
        }
        else{
            AudioCommandResult::NoSuchTrack
        }
    }

    /// Очищает хранилище.
    /// 
    /// Clears the storage.
    pub fn clear_storage(&mut self)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::ClearStorage){
            Ok(_)=>{
                self.track_array_len=0;
                AudioCommandResult::Sent
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
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>return AudioCommandResult::ThreadClosed
        };

        if let Some(stream)=stream_lock.as_ref(){
            self.event_loop.play_stream(stream.clone());
        }
        result
    }
    
    /// Останаливает трек из плейлиста.
    /// 
    /// Stops a track from the playlist.
    pub fn stop_track(&self,index:usize)->AudioCommandResult{
        match self.command.send(
            AudioSystemCommand::RemoveMonoFromPlaylist(index)
        ){
            Ok(())=>AudioCommandResult::Sent,
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
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>return AudioCommandResult::ThreadClosed
        };

        if let Some(stream)=stream_lock.as_ref(){
            self.event_loop.play_stream(stream.clone());
        }

        result
    }

    /// Останавливает треки из плейлиста.
    /// 
    /// Stops tracks in the playlist.
    pub fn stop_tracks(&self,indices:Vec<usize>)->AudioCommandResult{
        match self.command.send(
            AudioSystemCommand::RemoveMonosFromPlaylist(indices)
        ){
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>return AudioCommandResult::ThreadClosed
        }
    }

    /// Очищает весь плейлист.
    /// 
    /// Clears a playlist.
    pub fn clear_playlist(&self)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::ClearPlaylist){
            Ok(())=>AudioCommandResult::Sent,
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

        AudioCommandResult::Sent
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

        AudioCommandResult::Sent
    }

    /// Снимает трек с паузы.
    /// 
    /// Если уже проигрывается
    /// или такого трека нет, ничего не происходит.
    /// 
    /// Unpauses a track.
    /// 
    /// If it's already playing or
    /// there is no such track, nothing happens.
    pub fn unpause_track(&self,index:usize)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::UnpauseMonoFromPlaylist(index)){
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>return AudioCommandResult::ThreadClosed
        }
    }

    /// Снимает треки с паузы.
    /// 
    /// Если уже проигрываются
    /// или таких треков нет, ничего не происходит.
    /// 
    /// Unpauses tracks.
    /// 
    /// If they're already playing or
    /// there are no such tracks, nothing happens.
    pub fn unpause_tracks(&self,indices:Vec<usize>)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::UnpauseMonosFromPlaylist(indices)){
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>return AudioCommandResult::ThreadClosed
        }
    }

    /// Ставит трек из плейлиста на паузу.
    /// 
    /// Если уже на паузе
    /// или такого трека нет, ничего не происходит.
    /// 
    /// Pauses a track.
    /// 
    /// If it's already paused or
    /// there is no such track, nothing happens.
    pub fn pause_track(&self,index:usize)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::PauseMonoFromPlaylist(index)){
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>return AudioCommandResult::ThreadClosed
        }
    }

    /// Ставит треки из плейлиста на паузу.
    /// 
    /// Если уже на паузе
    /// или таких треков нет, ничего не происходит.
    /// 
    /// Pauses tracks.
    /// 
    /// If trey're already paused or
    /// there are no such tracks, nothing happens.
    pub fn pause_tracks(&self,indices:Vec<usize>)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::PauseMonosFromPlaylist(indices)){
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>return AudioCommandResult::ThreadClosed
        }
    }
}

/// Установка параметров.
/// 
/// Setting parameters.
impl Audio{
    /// Устанавливает громкость играющего трека.
    /// 
    /// Sets the volume of a playing track.
    pub fn set_track_volume(&self,index:usize,volume:f32)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::SetMonoVolume(index,volume)){
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>AudioCommandResult::ThreadClosed
        }
    }

    /// Устанавливает громкость играющих треков.
    /// 
    /// Sets the volume of playing tracks.
    pub fn set_tracks_volume(&self,indices:Vec<usize>,volume:f32)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::SetMonosVolume(indices,volume)){
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>AudioCommandResult::ThreadClosed
        }
    }

    /// Устанавливает громкости играющих треков.
    /// 
    /// Sets volumes of playing tracks.
    pub fn set_tracks_volumes(&self,sets:Vec<(usize,f32)>)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::SetMonosVolumes(sets)){
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>AudioCommandResult::ThreadClosed
        }
    }

    /// Устанавливает общую громкость.
    /// 
    /// Sets the general volume.
    pub fn set_general_volume(&self,volume:f32)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::SetGeneralVolume(volume)){
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>AudioCommandResult::ThreadClosed
        }
    }
}

/// Упраление через хранилище треков.
/// 
/// Operating through the track storage.
impl Audio{
    /// Снимает паузу с треков из плейлиста,
    /// привязанных к треку из хранилища.
    /// 
    /// Unpauses tracks from the playlist
    /// attached to a track from the storage.
    pub fn unpause_track_storage(&self,index:usize)->AudioCommandResult{
        match self.command.send(
            AudioSystemCommand::UnpauseMonoFromStorage(index)
        ){
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>return AudioCommandResult::ThreadClosed
        }
    }

    /// Снимает паузу с треков из плейлиста,
    /// привязанных к трекам из хранилища.
    /// 
    /// Unpauses tracks from the playlist
    /// attached to tracks from the storage.
    pub fn unpause_tracks_storage(&self,indices:Vec<usize>)->AudioCommandResult{
        match self.command.send(
            AudioSystemCommand::UnpauseMonosFromStorage(indices)
        ){
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>return AudioCommandResult::ThreadClosed
        }
    }

    /// Ставит на паузу треки из плейлиста,
    /// привязанных к треку из хранилища.
    /// 
    /// Pauses tracks from the playlist
    /// attached to a track from the storage.
    pub fn pause_track_storage(&self,index:usize)->AudioCommandResult{
        match self.command.send(
            AudioSystemCommand::PauseMonoFromStorage(index)
        ){
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>return AudioCommandResult::ThreadClosed
        }
    }

    /// Ставит на паузу треки из плейлиста,
    /// привязанные к трекам из хранилища.
    /// 
    /// Pauses tracks from the playlist attached to tracks
    /// from the storage.
    pub fn pause_tracks_storage(&self,indices:Vec<usize>)->AudioCommandResult{
        match self.command.send(
            AudioSystemCommand::PauseMonosFromStorage(indices)
        ){
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>return AudioCommandResult::ThreadClosed
        }
    }

    /// Останавливает треки из плейлиста,
    /// привязанные к треку из хранилища.
    /// 
    /// Stops tracks from the playlist
    /// attached to a track from the storage.
    pub fn stop_track_storage(&self,index:usize)->AudioCommandResult{
        match self.command.send(
            AudioSystemCommand::StopMonoFromStorage(index)
        ){
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>return AudioCommandResult::ThreadClosed
        }
    }

    /// Останавливает треки из плейлиста,
    /// привязанные к трекам из хранилища.
    /// 
    /// Stops tracks from the playlist
    /// attached to tracks from the storage.
    pub fn stop_tracks_storage(&self,indices:Vec<usize>)->AudioCommandResult{
        match self.command.send(
            AudioSystemCommand::StopMonosFromStorage(indices)
        ){
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>return AudioCommandResult::ThreadClosed
        }
    }

    /// Устанавливает громкость треков из плейлиста,
    /// привязанных к треку из хранилища.
    /// 
    /// Sets a volume of tracks from the playlist
    /// attached to a track from the storage.
    pub fn set_track_volume_storage(&self,index:usize,volume:f32)->AudioCommandResult{
        match self.command.send(
            AudioSystemCommand::SetMonoVolumeFromStorage(index,volume)
        ){
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>return AudioCommandResult::ThreadClosed
        }
    }

    /// Устанавливает громкость треков из плейлиста,
    /// привязанных к трекам из хранилища.
    /// 
    /// Sets a volume of tracks from the playlist
    /// attached to tracks from the storage.
    pub fn set_tracks_volume_storage(&self,indices:Vec<usize>,volume:f32)->AudioCommandResult{
        match self.command.send(
            AudioSystemCommand::SetMonosVolumeFromStorage(indices,volume)
        ){
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>return AudioCommandResult::ThreadClosed
        }
    }

    /// Устанавливает громкости треков из плейлиста,
    /// привязанных к трекам из хранилища.
    /// 
    /// Sets volumes of tracks from the playlist
    /// attached to tracks from the storage.
    pub fn set_tracks_volumes_storage(&self,sets:Vec<(usize,f32)>)->AudioCommandResult{
        match self.command.send(
            AudioSystemCommand::SetMonosVolumesFromStorage(sets)
        ){
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>return AudioCommandResult::ThreadClosed
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