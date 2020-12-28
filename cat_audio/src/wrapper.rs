use super::{
    Audio,
    ChanneledTrack,
    MonoTrack,
    TrackSet,
    TrackResult,
    AudioCommandResult,
};

use std::{
    path::Path,
    cmp::Ordering,
};

/// Простой интерфейс для управления аудио движком.
/// A simple interface for operating the audio engine.
pub struct AudioWrapper{
    pub audio:Audio,
    names:Vec<String>,
    track_sets:Vec<Vec<Set>>,
}

/// Номер трека и его каналы.
/// 
/// Track's index and channels.
pub struct Set{
    /// Track's storage index.
    pub index:usize,
    /// Channels for output.
    pub channels:Vec<usize>
}

/// Локальные функции.
impl AudioWrapper{
    fn search_track(&self,name:&str)->Option<usize>{
        for (c,track_name) in self.names.iter().enumerate(){
            if name.cmp(track_name)==Ordering::Equal{
                return Some(c)
            }
        }

        None
    }

    fn remove_track_inner(&mut self,name:&str)->Option<Vec<Set>>{
        if let Some(set)=self.search_track(name){
            self.names.remove(set);
            Some(self.track_sets.remove(set))
        }
        else{
            None
        }
    }
}

impl AudioWrapper{
    pub fn new(audio:Audio)->AudioWrapper{
        Self{
            audio,
            names:Vec::new(),
            track_sets:Vec::new(),
        }
    }

    /// Загружает трек в хранилище.
    /// 
    /// Возвращает `true`, если загрузка прошла без ошибок.
    /// 
    /// Loads a track to the storage.
    /// 
    /// Returns `true` if loaded with no errors.
    pub fn push_track(&mut self,track:ChanneledTrack,name:String)->bool{
        let sample_rate=track.sample_rate();

        let mut tracks=Vec::with_capacity(track.channels());

        let mut track_sets=Vec::with_capacity(track.channels());

        for (data,channels) in track.into_iter(){
            tracks.push(MonoTrack{data,sample_rate});
            track_sets.push(Set{index:0,channels});
        }

        if let AudioCommandResult::Indices(indices)=self.audio.add_tracks(tracks){
            for (c,index) in indices.into_iter().enumerate(){
                track_sets[c].index=index
            }
            // Добавление трека
            self.names.push(name);
            self.track_sets.push(track_sets);
            true
        }
        else{
            false
        }
    }

    /// Загружает трек в хранилище.
    /// 
    /// Возвращает `true`, если загрузка прошла без ошибок.
    /// 
    /// Loads a track to the storage.
    /// 
    /// Returns `true` if loaded with no errors.
    pub fn load_track<P:AsRef<Path>>(&mut self,path:P,name:String)->bool{
        let track=if let TrackResult::Ok(track)=ChanneledTrack::new(path){
            track
        }
        else{
            return false
        };

        let sample_rate=track.sample_rate();

        let mut tracks=Vec::with_capacity(track.channels());

        let mut track_sets=Vec::with_capacity(track.channels());

        for (data,channels) in track.into_iter(){
            tracks.push(MonoTrack{data,sample_rate});
            track_sets.push(Set{index:0,channels});
        }

        if let AudioCommandResult::Indices(indices)=self.audio.add_tracks(tracks){
            for (c,index) in indices.into_iter().enumerate(){
                track_sets[c].index=index
            }
            // Добавление трека
            self.names.push(name);
            self.track_sets.push(track_sets);
            true
        }
        else{
            false
        }
    }

    /// Удаляет трек из хранилища.
    /// 
    /// Removes a track from the storage.
    pub fn remove_track(&mut self,name:&str)->AudioCommandResult{
        if let Some(track_sets)=self.remove_track_inner(name){
            let track_indices:Vec<usize>=track_sets.into_iter().map(|set|set.index).collect();

            self.audio.remove_tracks(track_indices)
        }
        else{
            AudioCommandResult::Sent
        }
    }

    /// Очищает хранилище и плейлист.
    /// 
    /// Clears the storage and the playlist.
    pub fn clear_storage(&mut self)->AudioCommandResult{
        self.names.clear();
        self.track_sets.clear();
        self.audio.clear_storage()
    }

    /// Добавляет новые сеты.
    /// 
    /// Adds new sets.
    pub fn push_sets(&mut self,name:String,sets:Vec<Set>){
        self.names.push(name);
        self.track_sets.push(sets);
    }

    /// Возвращает сеты трека.
    /// 
    /// Returns track's sets.
    pub fn get_track_sets(&self,name:&str)->Option<&Vec<Set>>{
        if let Some(track_sets_index)=self.search_track(name){
            Some(&self.track_sets[track_sets_index])
        }
        else{
            None
        }
    }

    /// Возвращает сеты трека.
    /// 
    /// Returns track's sets.
    pub fn get_mut_track_sets(&mut self,name:&str)->Option<&mut Vec<Set>>{
        if let Some(track_sets_index)=self.search_track(name){
            Some(&mut self.track_sets[track_sets_index])
        }
        else{
            None
        }
    }
}

/// Проигрывание треков.
/// 
/// Playing tracks.
impl AudioWrapper{
    /// Запускает трек.
    /// 
    /// Plays a track.
    /// 
    /// Repeats:
    /// 0 - forever, 1 - once, 2 - twice and so on...
    pub fn play_track(&self,name:&str,repeats:u32)->AudioCommandResult{
        if let Some(track_sets_index)=self.search_track(name){
            let track_sets=&self.track_sets[track_sets_index];
            let mut full_track_sets=Vec::with_capacity(track_sets.len());

            for set in track_sets{
                let track_set=TrackSet{
                    index:set.index,
                    channels:set.channels.clone(),
                    repeats,
                    volume:1f32,
                };
                full_track_sets.push(track_set)
            }

            self.audio.play_tracks(full_track_sets)
        }
        else{
            AudioCommandResult::NoSuchTrack
        }
    }

    /// Останавливает трек.
    /// 
    /// Stops a track.
    pub fn stop_track(&self,name:&str)->AudioCommandResult{
        if let Some(track_sets_index)=self.search_track(name){
            let track_sets=&self.track_sets[track_sets_index];

            // Получение индексов треков
            let track_indices:Vec<usize>=track_sets.iter().map(|set|set.index).collect();

            self.audio.stop_tracks_storage(track_indices)
        }
        else{
            AudioCommandResult::NoSuchTrack
        }
    }

    /// Запускает поток проигрывания.
    /// 
    /// Starts playing the output stream.
    pub fn play(&self)->AudioCommandResult{
        self.audio.play()
    }

    /// Ставит на паузу поток проигрывания.
    /// 
    /// Pauses the output stream.
    pub fn pause(&self)->AudioCommandResult{
        self.audio.pause()
    }

    /// Возобновляет проигрывание трека.
    /// 
    /// Unpauses a track.
    pub fn unpause_track(&self,name:&str)->AudioCommandResult{
        if let Some(track_sets_index)=self.search_track(name){
            let track_sets=&self.track_sets[track_sets_index];

            // Получение индексов треков
            let track_indices:Vec<usize>=track_sets.iter().map(|set|set.index).collect();

            self.audio.unpause_tracks_storage(track_indices)
        }
        else{
            AudioCommandResult::NoSuchTrack
        }
    }

    /// Ставит трек на паузу.
    /// 
    /// Pauses a track.
    pub fn pause_track(&self,name:&str)->AudioCommandResult{
        if let Some(track_sets_index)=self.search_track(name){
            let track_sets=&self.track_sets[track_sets_index];

            // Получение индексов треков
            let track_indices:Vec<usize>=track_sets.iter().map(|set|set.index).collect();

            self.audio.pause_tracks_storage(track_indices)
        }
        else{
            AudioCommandResult::NoSuchTrack
        }
    }

    /// Очищает плейлист.
    /// 
    /// Clears the playlist.
    pub fn clear_playlist(&self)->AudioCommandResult{
        self.audio.clear_playlist()
    }
}

/// Функции установки параметров.
/// 
/// Setting functions.
impl AudioWrapper{
    /// Устанавливает громкость трека.
    /// 
    /// Sets track's volume.
    pub fn set_track_volume(&self,name:&str,volume:f32)->AudioCommandResult{
        if let Some(track_sets_index)=self.search_track(name){
            // Получение индексов треков
            let tracks:Vec<usize>=self.track_sets[track_sets_index].iter().map(|set|set.index).collect();

            self.audio.set_tracks_volume_storage(tracks,volume)
        }
        else{
            AudioCommandResult::NoSuchTrack
        }
    }

    /// Устанавливает общую громкость.
    /// 
    /// Sets the general volume.
    pub fn set_general_volume(&self,volume:f32)->AudioCommandResult{
        self.audio.set_general_volume(volume)
    }
}