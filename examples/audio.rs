#![cfg(feature="audio")]

use cat_engine::audio::{
    AudioSettings,
    Audio,
};

fn main(){
    let settings=AudioSettings::new();
    let audio=Audio::new(settings).unwrap();

    // Add a track to the folder before running
    audio.add_track("resources/audio.mp3"); // track index = 0
 
    audio.play_once(0); // plays the track with index 0

    std::thread::sleep(std::time::Duration::from_millis(2000));
}

