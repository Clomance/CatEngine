# Intoduction

The audio engine is enabled with either `audio`, `extended_audio` or `raw_audio` feature.

It consists of two parts:
 - a storage (container of tracks that can be played)
 - a playlist (list of currently playing tracks)

To simplify the system and add some features only mono-channel tracks are used.
You have to divide multi-channel tracks into mono channels and load, play and edit each channel seperatly.

The engine has it's own thread that is closed with panic after finishing (I can't close it other way),
so don't panic!

Only supports output.
Only the `mp3` format is supported to decode.
All the tracks are converted to 24-bit format to simply work with them.

There is three variants of operating the engine:
 - with `AudioWrapper` which simplify working, but has less possibilities
 - with engine's own functions
 - with direct commands, without unnecessary checks indices and overflows



# Working with `AudioWrapper`

A simple interface. Good for usual playback.

```rust
let settings=AudioSettings::new();
let audio=Audio::default(settings).unwrap();

let mut wrapper=AudioWrapper::new(audio);

wrapper.load_track("resources/audio.mp3","audio".to_string());

wrapper.play_track("audio",1u32).unwrap(); // Проигрывает трек один раз

wrapper.set_track_volume("audio",0.5f32).unwrap();
```

Additional features:
 - access to the lower level (see `Working with engine's functions`)
```rust
// Pauses a track in the playlist
wrapper.audio.pause_track(0).unwrap();
```
 - adding, receiving, editing sets
```rust
let sets=vec![
    Set{
        index:0, // track's index in the storage
        channels:vec![0usize,1] // channels that track will be played on
    }
];
wrapper.push_sets("new".to_string(),sets);
let new_sets=wrapper.get_track_sets("new").unwrap();
```



# Working with engine's functions

Makes it possible to selectively operate tracks with both playlist and storage.

For example, it's possible to load a two-channel track and play only one channel.
```rust
let settings=AudioSettings::new();
let audio=Audio::default(settings).unwrap();

// Loaded with the wrapper because it's easier
let mut wrapper=AudioWrapper::new(audio);
wrapper.load_track("resources/audio.mp3","audio3".to_string());

// Parameters of playing: play once on channels 0 and 1
let set=TrackSet::once(0,vec![0,1]);
wrapper.audio.play_track(set).unwrap();

// Setting the tracks volume
wrapper.audio.set_track_volume(0,0.5f32).unwrap();
```


<!-- ##### Access through playlist and storage -->



# Working with direct commands

The idea of this method is to completly get rid of outer checks. Although the engine's core handles all the errors and doesn't let the system fail, but no result is returned.
I advise you to learn the engine within to effectively and correctly operate it.


### Extended mode

Enabled with `features=["extended_audio"]` and adds functions to operate some inner field of `Audio`
and send direct commands to the engine's core.


### Raw mod

Enabled with `features=["raw_audio"]` and disable all the fields and fuctions of `Audio`
that doesn't have direct access to the engine's core.
