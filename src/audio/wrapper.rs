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
    collections::HashMap,
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
    index:usize,
    channels:Vec<usize>
}

impl AudioWrapper{
    pub fn new(audio:Audio)->AudioWrapper{
        Self{
            audio,
            names:Vec::new(),
            track_sets:Vec::new(),
        }
    }

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

        let mut track_sets=Vec::with_capacity(track.channels());

        for (data,channels) in track.into_iter(){
            let index=self.audio.tracks_amount();

            if let AudioCommandResult::Sent=self.audio.add_track(MonoTrack{
                data,
                sample_rate,
            }){
                let set=Set{
                    index,
                    channels,
                };

                track_sets.push(set)
            }
            else{
                return false
            }
        }

        // Добавление трека
        self.names.push(name);
        self.track_sets.push(track_sets);

        true
    }

    /// Удаляет трек из хранилища.
    /// 
    /// Remove a track from the storage.
    pub fn remove_track(&mut self,name:&str)->AudioCommandResult{
        if let Some(track_sets)=self.remove_track_inner(name){
            let len=track_sets.len();

            let track_indices:Vec<usize>=track_sets.into_iter().map(|set|set.index).collect();

            for sets in &mut self.track_sets{
                for set in sets{
                    set.index-=len;
                }
            }

            self.audio.remove_tracks(track_indices)
        }
        else{
            AudioCommandResult::Sent
        }
    }

    /// Очищает хранилище треков.
    /// 
    /// Clears the track storage.
    pub fn clear_storage(&mut self)->AudioCommandResult{
        self.names.clear();
        self.track_sets.clear();
        self.audio.clear_storage()
    }

    /// Возвращает сет трека.
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
}

/// Проигрывание треков.
/// 
/// Play tracks.
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
    /// Устанавливает общую громкость.
    /// 
    /// Sets the general volume.
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

// /// Низкоуровневые функции.
// /// 
// /// Low-level functions.
// impl AudioWrapper{
//     pub fn remove_mono_track(&mut self,index:usize)->AudioCommandResult{
//         self.audio.remove_track(index)
//     }

//     // pub fn remove_mono_track_from_playlist(&mut self,index:usize)->AudioCommandResult{
//     //     self.audio.remove_track_from_playlist(index)
//     // }

//     /// Ставит трек проигрываться.
//     /// 
//     /// Если уже проигрывается или такого трека нет, ничего не происходит.
//     /// 
//     /// Unpauses a track.
//     /// 
//     /// If it's already playing or there is no such track, nothing happens.
//     pub fn unpause_mono_track(&self,index:usize)->AudioCommandResult{
//         self.audio.unpause_track(index)
//     }

//     /// Ставит трек на паузу.
//     /// 
//     /// Если уже на паузе или такого трека нет, ничего не происходит.
//     /// 
//     /// Pauses a track.
//     /// 
//     /// If it's already paused or there is no such track, nothing happens.
//     pub fn pause_mono_track(&self,index:usize)->AudioCommandResult{
//         self.audio.pause_track(index)
//     }
// }