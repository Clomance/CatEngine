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
    collections::HashMap
};

/// Простой интерфейс для управления аудио движком.
/// A simple interface for operating the audio engine.
pub struct AudioWrapper{
    pub audio:Audio,
    track_table:HashMap<String,usize>,
    track_sets:Vec<Vec<Set>>,
}

/// Номер трека и его каналы.
struct Set{
    index:usize,
    channels:Vec<usize>
}

impl AudioWrapper{
    pub fn new(audio:Audio)->AudioWrapper{
        Self{
            audio,
            track_table:HashMap::new(),
            track_sets:Vec::new(),
        }
    }

    /// Загружает трек.
    /// 
    /// Loads a track.
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

            if let AudioCommandResult::Ok=self.audio.add_track(MonoTrack{
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

        let index=self.track_sets.len();
        self.track_sets.push(track_sets);

        self.track_table.insert(name,index);

        true
    }

    /// Удаляет трек из хранилища.
    /// 
    /// Remove a track from the storage.
    pub fn remove_track(&mut self,name:&str)->AudioCommandResult{
        if let Some(track_sets_index)=self.track_table.remove(name){
            let track_sets=self.track_sets.remove(track_sets_index);

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
            AudioCommandResult::Ok
        }
    }

    // /// Устанавливает громкость играющего трека.
    // /// 
    // /// Sets the volume of a playing track.
    // pub fn set_track_volume(&mut self,track:usize,volume:f32)->AudioCommandResult{
    //     self.audio.set_track_volume(track,volume)
    // }

    /// Устанавливает общую громкость.
    /// 
    /// Sets the general volume.
    pub fn set_general_volume(&self,volume:f32)->AudioCommandResult{
        self.audio.set_general_volume(volume)
    }

    /// Запускает трек.
    /// 
    /// Plays a track.
    pub fn play_track(&self,name:&str)->AudioCommandResult{
        if let Some(&track_sets_index)=self.track_table.get(name){
            let track_sets=&self.track_sets[track_sets_index];
            let mut full_track_sets=Vec::with_capacity(track_sets.len());

            for set in track_sets{
                let track_set=TrackSet{
                    index:set.index,
                    channels:set.channels.clone(),
                    repeats:1u32,
                    volume:1f32,
                };
                full_track_sets.push(track_set)
            }

            self.audio.play_tracks(full_track_sets)
        }
        else{
            AudioCommandResult::Ok
        }
    }

    // /// Останавливает трек.
    // /// 
    // /// Stops a track.
    // pub fn stop_track(&self,name:&str)->AudioCommandResult{
    //     if let Some(&track_sets_index)=self.track_table.get(name){
    //         let track_sets=&self.track_sets[track_sets_index];

    //         let track_indices:Vec<usize>=track_sets.iter().map(|set|set.index).collect();

    //         self.audio.stop_tracks(track_indices)
    //     }
    //     else{
    //         AudioCommandResult::Ok
    //     }
    // }

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

    // /// Запускает проигрывание трека.
    // /// 
    // /// Unpauses a track.
    // pub fn unpause_track(&self,name:&str)->AudioCommandResult{
    //     if let Some(&track_sets_index)=self.track_table.get(name){
    //         let track_sets=&self.track_sets[track_sets_index];

    //         let track_indices:Vec<usize>=track_sets.iter().map(|set|set.index).collect();

    //         self.audio.unpause_tracks(track_indices)
    //     }
    //     else{
    //         AudioCommandResult::Ok
    //     }
    // }

    // /// Ставит трек на паузу.
    // /// 
    // /// Pauses a track.
    // pub fn pause_track(&self,name:&str)->AudioCommandResult{
    //     if let Some(&track_sets_index)=self.track_table.get(name){
    //         let track_sets=&self.track_sets[track_sets_index];

    //         let track_indices:Vec<usize>=track_sets.iter().map(|set|set.index).collect();

    //         self.audio.pause_tracks(track_indices)
    //     }
    //     else{
    //         AudioCommandResult::Ok
    //     }
    // }

    /// Очищает плейлист.
    /// 
    /// Clears the playlist.
    pub fn clear_playlist(&self)->AudioCommandResult{
        self.audio.clear_playlist()
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