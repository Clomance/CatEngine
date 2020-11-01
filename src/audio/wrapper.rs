use super::{
    Audio,
    ChanneledTrack,
    MonoTrack,
    TrackSet,
};

use std::{
    path::Path,
    collections::HashMap
};

/// Простой интерфейс для управление треками и аудио движком.
/// A simple interface to operate tracks and the audio engine.
pub struct AudioWrapper{
    audio:Audio,
    track_sets:HashMap<String,Vec<Set>>,
}

struct Set{
    index:usize,
    channels:Vec<usize>
}

impl AudioWrapper{
    pub fn new(audio:Audio)->AudioWrapper{
        Self{
            audio,
            track_sets:HashMap::new(),
        }
    }

    pub fn load_track<P:AsRef<Path>>(&mut self,path:P,name:String){
        let track=ChanneledTrack::new(path).unwrap();
        let sample_rate=track.sample_rate();

        let mut track_sets=Vec::with_capacity(track.channels());

        for (data,channels) in track.into_iter(){
            let index=self.audio.tracks_amount();

            self.audio.add_track(MonoTrack{
                data,
                sample_rate,
            });

            let set=Set{
                index,
                channels,
            };

            track_sets.push(set)
        }

        self.track_sets.insert(name,track_sets);
    }

    pub fn set_general_volume(&self,volume:f32){
        self.audio.set_general_volume(volume);
    }

    pub fn play_track(&self,name:&str){
        let track=self.track_sets.get(name).unwrap();

        let mut track_sets=Vec::with_capacity(track.len());

        for set in track{
            let track_set=TrackSet{
                index:set.index,
                channels:set.channels.clone(),
                repeats:1u32,
                volume:1f32,
            };
            track_sets.push(track_set)
        }

        self.audio.play_tracks(track_sets).unwrap()
    }

    pub fn play(&self){
        self.audio.play();
    }

    pub fn pause(&self){
        self.audio.pause();
    }

    pub fn clear_playlist(&self){
        self.audio.clear_playlist();
    }

    // pub fn add_set(&mut self,index:usize,channels:Vec<usize>,repeats:u32,volume:f32){
    //     let set=TrackSet{
    //         index,
    //         channels,
    //         repeats,
    //         volume,
    //     };
    // }
}