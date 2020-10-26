#![cfg(feature="audio")]

use cat_engine::audio::{
    AudioSettings,
    Audio,
    cpal,
};

fn main(){
    let settings=AudioSettings::new();
    let host=cpal::default_host();
    let audio=Audio::new(host,settings.clone()).unwrap();

    // Add a track to the array before running
    audio.add_track("resources/audio3.mp3"); // a track index = 0

    audio.play_track(0,1); // plays the track once

    std::thread::sleep(std::time::Duration::from_millis(3000));
}