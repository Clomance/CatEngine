#![allow(non_upper_case_globals,dead_code,unused_imports)]

//! # Многоканальный аудио движок. A multichannel audio engine.
//! 
//! Аудио движок имеет свой поток для работы со звуком.
//! Также в нём есть хранилище аудио треков, которые можно запустить.
//! 
//! Поддерживает только вывод.
//! Пока что позволяет декодировать только треки формата `mp3`.
//! Все треки переводятся в 24-битный формат.
//! 
//! Поток закрывается с паникой, так что не паникуте!
//! 
//! Больше вы сможете узнать из [книги](https://github.com/Clomance/CatEngine/blob/master/book/RUS/audio.md).
//! 
//! #
//! 
//! The audio engine has it's own thread to work with sound.
//! Also it has a storage of audio tracks that could be played.
//! 
//! Supports only output.
//! For now only 'mp3' format decoding is supported.
//! All tracks are converted to the 24-bit format.
//! 
//! The thread closes with panic, so don't panic!
//! 
//! You can learn more from the [book](https://github.com/Clomance/CatEngine/blob/master/book/EN/audio.md).
//! 
//! #
//! 
//! ```
//! let audio=Audio::default(AudioSettings::new()).unwrap();
//! 
//! // For easier access to the audio engine
//! let mut wrapper=AudioWrapper::new(audio);
//! 
//! wrapper.load_track("audio.mp3","test".to_string());
//! 
//! wrapper.play_track("test",0u32).unwrap();
//! ```

// re-export
pub use cpal;

mod engine_core;
use engine_core::event_loop_handler;

mod channel_system;
use channel_system::*;

mod track_iterator;
use track_iterator::*;

mod track;
pub use track::*;

mod sample;
use sample::SampleTransform;

mod support;

#[cfg(not(feature="raw"))]
mod wrapper;
#[cfg(not(feature="raw"))]
pub use wrapper::AudioWrapper;

mod engine_commands;

#[cfg(not(feature="extended"))]
use engine_commands::AudioEngineCommand;
#[cfg(feature="extended")]
pub use engine_commands::AudioEngineCommand;

pub use engine_commands::AudioCommandResult;

use cpal::{
    Host,
    Device,
    traits::{
        HostTrait,
        DeviceTrait,
        EventLoopTrait
    },
    StreamId,
    EventLoop,
    Format,
};

use std::{
    io,
    thread::{Builder,JoinHandle},
    sync::{
        Arc,
        Mutex,
        LockResult,
        mpsc::{Sender,channel},
    },
};

#[cfg(feature="extended")]
use std::sync::mpsc::SendError;

const audio_thread_stack_size:usize=1024;


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
    pub track_storage_capacity:usize,

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
            track_storage_capacity:8,
            track_playlist_capacity:8,
        }
    }
}

/// Внутренние настройки системы.
pub (crate) struct AudioSystemSettings{
    pub general_volume:f32,
    pub output_channels:u16,
    pub format:Format,

    pub track_storage_capacity:usize,
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
    playing_flag:Arc<Mutex<bool>>,
    stream:Arc<Mutex<Option<StreamId>>>,

    event_loop:Arc<EventLoop>,

    command:Sender<AudioEngineCommand>,
    thread:Option<JoinHandle<()>>,

    // Флаги занятости слотов
    #[cfg(not(feature="raw"))]
    storage_slots:Vec<bool>,
    #[cfg(not(feature="raw"))]
    free_storage_slots:Vec<usize>,
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

        let playing_flag=Arc::new(Mutex::new(true));
        let playing_flag1=playing_flag.clone();

        let event_loop=Arc::new(host.event_loop());
        let el=event_loop.clone();
        // Канал для передачи команд от управляющего потока выполняющему
        let (sender,receiver)=channel::<AudioEngineCommand>();

        let owner_host=Arc::new(host);
        let host=owner_host.clone();

        #[cfg(not(feature="raw"))]
        let track_storage_capacity=settings.track_storage_capacity;

        let thread_result=Builder::new()
                .name("CatEngine's audio thread".to_string())
                .stack_size(audio_thread_stack_size)
                .spawn(move||{

            // Выполнение замыкания
            let device=choose_device(host.as_ref());

            let format=choose_format(&device);

            let main_stream=event_loop.build_output_stream(&device,&format).expect("stream");

            *stream.lock().unwrap()=Some(main_stream.clone());

            event_loop.play_stream(main_stream.clone()).unwrap();

            let system_settings=AudioSystemSettings{
                general_volume:settings.general_volume,
                output_channels:format.channels,
                format,

                track_storage_capacity:settings.track_storage_capacity,
                track_playlist:settings.track_playlist_capacity,
            };

            // Забирает контроль над потоком и начинает обработку аудио потоков
            // Takes control of the current thread and begins the stream processing
            event_loop_handler(
                host,
                playing_flag,
                system_settings,
                stream,
                event_loop,
                receiver,
            )
        });

        let thread=match thread_result{
            Ok(thread)=>thread,
            Err(e)=>return Err(e),
        };

        Ok(Self::init(playing_flag1,s,el,sender,thread,
            #[cfg(not(feature="raw"))]track_storage_capacity))
    }

    /// Строит аудио движок с хостом, устройством и потоком по умолчанию.
    /// 
    /// Возвращает результат создания аудио потока.
    /// 
    /// Creates an audio engine with default host, device and streams.
    /// 
    /// Returns the result of starting an audio thread.
    pub fn default(settings:AudioSettings)->io::Result<Audio>{
        let host=cpal::default_host();
        // 
        let stream=Arc::new(Mutex::new(None));
        let s=stream.clone();

        let playing_flag=Arc::new(Mutex::new(true));
        let playing_flag1=playing_flag.clone();

        let event_loop=Arc::new(host.event_loop());
        let el=event_loop.clone();
        // Канал для передачи команд от управляющего потока выполняющему
        let (sender,receiver)=channel::<AudioEngineCommand>();

        let owner_host=Arc::new(host);
        let host=owner_host.clone();

        #[cfg(not(feature="raw"))]
        let track_storage_capacity=settings.track_storage_capacity;

        let thread_result=Builder::new()
                .name("CatEngine's audio thread".to_string())
                .stack_size(audio_thread_stack_size)
                .spawn(move||{

            let device=host.default_output_device().expect("No available device");

            let format=device.default_output_format().expect("No available device");

            let main_stream=event_loop.build_output_stream(&device,&format).expect("No available device");

            *stream.lock().expect("The audio thread is down")=Some(main_stream.clone());

            event_loop.play_stream(main_stream.clone()).expect("No available device");

            let system_settings=AudioSystemSettings{
                general_volume:settings.general_volume,
                output_channels:format.channels,
                format,

                track_storage_capacity:settings.track_storage_capacity,
                track_playlist:settings.track_playlist_capacity,
            };

            // Забирает контроль над потоком и начинает обработку аудио потоков
            // Takes control of the current thread and begins the stream processing
            event_loop_handler(
                host,
                playing_flag,
                system_settings,
                stream,
                event_loop,
                receiver,
            )
        });

        let thread=match thread_result{
            Ok(thread)=>thread,
            Err(e)=>return Err(e),
        };

        Ok(Self::init(playing_flag1,s,el,sender,thread,
            #[cfg(not(feature="raw"))]track_storage_capacity))
    }

    #[inline]
    fn init(
        playing_flag:Arc<Mutex<bool>>,
        stream:Arc<Mutex<Option<StreamId>>>,
        event_loop:Arc<EventLoop>,
        sender:Sender<AudioEngineCommand>,
        thread:JoinHandle<()>,
        #[cfg(not(feature="raw"))]track_storage_capacity:usize,
    )->Audio{
        #[cfg(not(feature="raw"))]
        let mut storage_slots=Vec::with_capacity(track_storage_capacity);
        #[cfg(not(feature="raw"))]
        let mut free_storage_slots=Vec::with_capacity(track_storage_capacity);
        #[cfg(not(feature="raw"))]
        for c in 0..track_storage_capacity{
            free_storage_slots.push(c);
            storage_slots.push(false);
        }

        Self{
            playing_flag,
            stream,

            event_loop,
            command:sender,
            thread:Some(thread),

            #[cfg(not(feature="raw"))]
            free_storage_slots,
            #[cfg(not(feature="raw"))]
            storage_slots
        }
    }

    /// Возвращает количество треков в хранилище.
    /// 
    /// Returns the amount of track in the storage.
    #[cfg(not(feature="raw"))]
    pub fn tracks_amount(&self)->usize{
        self.storage_slots.len()-self.free_storage_slots.len()
    }
}

/// feature="extended"
#[cfg(feature="extended")]
impl Audio{
    /// Отправляет команду аудио системе.
    /// 
    /// Sends a audio system command.
    pub fn send_command(
        &self,
        command:AudioEngineCommand
    )->Result<(),SendError<AudioEngineCommand>>{
        self.command.send(command)
    }

    /// Возращает флаги ячеек хранилища.
    /// true - занято, false - свободно
    /// 
    /// Returns storage slot flags.
    /// true - busy, false - free
    #[cfg(not(feature="raw"))]
    pub fn storage_slots(&mut self)->&mut Vec<bool>{
        &mut self.storage_slots
    }

    /// Возращает индексы свободных ячеек хранилища.
    /// 
    /// Returns indices of free storage slots.
    #[cfg(not(feature="raw"))]
    pub fn free_storage_slots(&mut self)->&mut Vec<usize>{
        &mut self.free_storage_slots
    }
}

/// Добавление/удаление треков.
/// 
/// Adding/removing tracks.
#[cfg(not(feature="raw"))]
impl Audio{
    /// Добавляет трек в хранилище.
    /// 
    /// Если недостаточно места, возвращает `AudioCommandResult::StorageOverflow`.
    /// 
    /// Adds a track to the storage.
    /// 
    /// If there is not enough space, returns `AudioCommandResult::StorageOverflow`.
    pub fn add_track(&mut self,track:MonoTrack)->AudioCommandResult{
        if let Some(slot)=self.free_storage_slots.pop(){
            self.storage_slots[slot]=true;

            match self.command.send(AudioEngineCommand::AddMono(track,slot)){
                Ok(_)=>AudioCommandResult::Index(slot),
                Err(_)=>AudioCommandResult::ThreadClosed,
            }
        }
        else{
            AudioCommandResult::StorageOverflow
        }
    }

    /// Добавляет несколько треков в хранилище.
    /// 
    /// Если недостаточно места даже для одного,
    /// возвращает `AudioCommandResult::StorageOverflow`.
    /// 
    /// Adds some tracks to the storage.
    /// 
    /// If there is not enough space even for one,
    /// returns `AudioCommandResult::StorageOverflow`.
    pub fn add_tracks(&mut self,tracks:Vec<MonoTrack>)->AudioCommandResult{
        let mut indices=Vec::with_capacity(tracks.len());
        let mut track_sets=Vec::with_capacity(tracks.len());

        for track in tracks{
            if let Some(slot)=self.free_storage_slots.pop(){
                self.storage_slots[slot]=true;
                indices.push(slot);
                track_sets.push((track,slot))
            }
        }

        if !indices.is_empty(){
            match self.command.send(AudioEngineCommand::AddMonos(track_sets)){
                Ok(_)=>AudioCommandResult::Indices(indices),
                Err(_)=>AudioCommandResult::ThreadClosed,
            }
        }
        else{
            AudioCommandResult::StorageOverflow
        }
    }

    /// Удаляет трек из хранилища.
    /// 
    /// Removes the track from the storage.
    pub fn remove_track(&mut self,index:usize)->AudioCommandResult{
        if let Some(slot)=self.storage_slots.get_mut(index){
            if *slot{
                *slot=false;
                // Казна пустеет, милорд!
                match self.command.send(AudioEngineCommand::RemoveMono(index)){
                    Ok(_)=>AudioCommandResult::Sent,
                    Err(_)=>AudioCommandResult::ThreadClosed,
                }
            }
            else{
                AudioCommandResult::NoSuchTrack
            }
        }
        else{
            AudioCommandResult::NoSuchTrack
        }
    }

    /// Удаляет треки из хранилища.
    /// 
    /// Removes tracks from the storage.
    pub fn remove_tracks(&mut self,indices:Vec<usize>)->AudioCommandResult{
        let mut track_indices=Vec::with_capacity(indices.len());

        for index in indices{
            if let Some(slot)=self.storage_slots.get_mut(index){
                if *slot{
                    *slot=false;
                    track_indices.push(index);
                }
            }
        }

        if !track_indices.is_empty(){
            // Казна пустеет, милорд!
            match self.command.send(AudioEngineCommand::RemoveMonos(track_indices)){
                Ok(_)=>AudioCommandResult::Sent,
                Err(_)=>AudioCommandResult::ThreadClosed,
            }
        }
        else{
            AudioCommandResult::NoSuchTrack
        }
    }

    /// Очищает хранилище и плейлист.
    /// 
    /// Clears the storage and the playlist.
    pub fn clear_storage(&mut self)->AudioCommandResult{
        self.free_storage_slots.clear();
        for (c,slot) in self.storage_slots.iter_mut().enumerate(){
            *slot=false;
            self.free_storage_slots.push(c);
        }

        match self.command.send(AudioEngineCommand::ClearPlaylist){
            Ok(_)=>AudioCommandResult::Sent,
            Err(_)=>AudioCommandResult::ThreadClosed,
        }
    }
}

/// Проигрывание треков.
/// 
/// Play tracks.
#[cfg(not(feature="raw"))]
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
            AudioEngineCommand::PlayMonoOnChannels(set)
        ){
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>return AudioCommandResult::ThreadClosed
        };

        if let Some(stream)=stream_lock.as_ref(){
            self.event_loop.play_stream(stream.clone()).unwrap();
        }

        *self.playing_flag.lock().unwrap()=true;

        result
    }

    /// Запускает треки.
    /// 
    /// Plays tracks.
    pub fn play_tracks(&self,sets:Vec<TrackSet>)->AudioCommandResult{
        let stream_lock=match self.stream.lock(){
            LockResult::Ok(lock)=>lock,
            LockResult::Err(_)=>return AudioCommandResult::ThreadClosed
        };

        // Отправка команды
        let result=match self.command.send(
            AudioEngineCommand::PlayMonosOnChannels(sets)
        ){
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>return AudioCommandResult::ThreadClosed
        };

        if let Some(stream)=stream_lock.as_ref(){
            self.event_loop.play_stream(stream.clone()).unwrap();
        }

        *self.playing_flag.lock().unwrap()=true;

        result
    }

    /// Останаливает трек из плейлиста.
    /// 
    /// Stops a track from the playlist.
    pub fn stop_track(&self,index:usize)->AudioCommandResult{
        match self.command.send(
            AudioEngineCommand::RemoveMonoFromPlaylist(index)
        ){
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>return AudioCommandResult::ThreadClosed
        }
    }

    /// Останавливает треки из плейлиста.
    /// 
    /// Stops tracks in the playlist.
    pub fn stop_tracks(&self,indices:Vec<usize>)->AudioCommandResult{
        match self.command.send(
            AudioEngineCommand::RemoveMonosFromPlaylist(indices)
        ){
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>return AudioCommandResult::ThreadClosed
        }
    }

    /// Очищает весь плейлист.
    /// 
    /// Clears a playlist.
    pub fn clear_playlist(&self)->AudioCommandResult{
        match self.command.send(AudioEngineCommand::ClearPlaylist){
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
            self.event_loop.play_stream(stream.clone()).unwrap();
        }

        *self.playing_flag.lock().unwrap()=true;

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
            self.event_loop.pause_stream(stream.clone()).unwrap();
        }

        *self.playing_flag.lock().unwrap()=false;

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
        match self.command.send(AudioEngineCommand::UnpauseMonoFromPlaylist(index)){
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
        match self.command.send(AudioEngineCommand::UnpauseMonosFromPlaylist(indices)){
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
        match self.command.send(AudioEngineCommand::PauseMonoFromPlaylist(index)){
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
        match self.command.send(AudioEngineCommand::PauseMonosFromPlaylist(indices)){
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>return AudioCommandResult::ThreadClosed
        }
    }
}

/// Установка параметров.
/// 
/// Setting parameters.
#[cfg(not(feature="raw"))]
impl Audio{
    /// Устанавливает громкость играющего трека.
    /// 
    /// Sets the volume of a playing track.
    pub fn set_track_volume(&self,index:usize,volume:f32)->AudioCommandResult{
        match self.command.send(AudioEngineCommand::SetMonoVolume(index,volume)){
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>AudioCommandResult::ThreadClosed
        }
    }

    /// Устанавливает громкость играющих треков.
    /// 
    /// Sets the volume of playing tracks.
    pub fn set_tracks_volume(&self,indices:Vec<usize>,volume:f32)->AudioCommandResult{
        match self.command.send(AudioEngineCommand::SetMonosVolume(indices,volume)){
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>AudioCommandResult::ThreadClosed
        }
    }

    /// Устанавливает громкости играющих треков.
    /// 
    /// Sets volumes of playing tracks.
    pub fn set_tracks_volumes(&self,sets:Vec<(usize,f32)>)->AudioCommandResult{
        match self.command.send(AudioEngineCommand::SetMonosVolumes(sets)){
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>AudioCommandResult::ThreadClosed
        }
    }

    /// Устанавливает общую громкость.
    /// 
    /// Sets the general volume.
    pub fn set_general_volume(&self,volume:f32)->AudioCommandResult{
        match self.command.send(AudioEngineCommand::SetGeneralVolume(volume)){
            Ok(())=>AudioCommandResult::Sent,
            Err(_)=>AudioCommandResult::ThreadClosed
        }
    }
}

/// Упраление через хранилище треков.
/// 
/// Operating through the track storage.
#[cfg(not(feature="raw"))]
impl Audio{
    /// Снимает паузу с треков из плейлиста,
    /// привязанных к треку из хранилища.
    /// 
    /// Unpauses tracks from the playlist
    /// attached to a track from the storage.
    pub fn unpause_track_storage(&self,index:usize)->AudioCommandResult{
        match self.command.send(
            AudioEngineCommand::UnpauseMonoFromStorage(index)
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
            AudioEngineCommand::UnpauseMonosFromStorage(indices)
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
            AudioEngineCommand::PauseMonoFromStorage(index)
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
            AudioEngineCommand::PauseMonosFromStorage(indices)
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
            AudioEngineCommand::StopMonoFromStorage(index)
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
            AudioEngineCommand::StopMonosFromStorage(indices)
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
            AudioEngineCommand::SetMonoVolumeFromStorage(index,volume)
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
            AudioEngineCommand::SetMonosVolumeFromStorage(indices,volume)
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
            AudioEngineCommand::SetMonosVolumesFromStorage(sets)
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
        let _=self.command.send(AudioEngineCommand::Close);

        if let Some(stream)=self.stream.lock().unwrap().as_ref(){
            let _=self.event_loop.play_stream(stream.clone());
        }

        if let Some(thread)=self.thread.take(){
            let _=thread.join();
        }
    }
}