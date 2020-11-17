# Intoduction

The audio engine is enabled with `audio`, `extended_audio` and `raw_audio` features.



# Working with `AudioWrapper`

A simple interface. Good for usual playback.

```
let settings=AudioSettings::new();
let audio=Audio::default(settings).unwrap();

let mut wrapper=AudioWrapper::new(audio);

wrapper.load_track("resources/audio.mp3","audio".to_string());

wrapper.play_track("audio",1u32).unwrap(); // Проигрывает трек один раз

wrapper.set_track_volume("audio",0.5f32).unwrap();
```

# Working with engine functions

##### Acces through playlist and storage



# Working with direct commands

The idea of this method is you completly get rid of outer checks. Although the engine's core handles all the errors and doesn't let the system fail, but there is no result of the command accomplishing is returned.
I advise you to learn the engine within to effectively and correctly operate it.


### Extended mode

Enabled with `features=["extended_audio"]` and adds functions to operate some inner field of `Audio`
and send direct commands to the engine's core.


### Raw mod

Enabled with `features=["raw_audio"]` and disable all the fields and fuctions of `Audio`
that doesn't have direct access to the engine's core.