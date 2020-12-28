use cat_audio::{
    AudioSettings,
    Audio,
    AudioWrapper,
    ChanneledTrack,
};

fn main(){
    let settings=AudioSettings::new();
    let audio=Audio::default(settings).unwrap();

    let mut wrapper=AudioWrapper::new(audio);

    // Load a track separately from the wrapper
    let track=ChanneledTrack::new("../resources/audio3.mp3").unwrap();

    // Add the track to the storage before running
    wrapper.push_track(track,"audio3".to_string());

    wrapper.play_track("audio3",1u32); // plays the track once

    wrapper.set_track_volume("audio3",0.5f32).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(3000));
}