# ![logo](https://github.com/Clomance/CatEngine/raw/master/cat_audio/logo_400x400.png)

# CatAudio [![Crate badge]][crates.io] [![Doc badge]][doc.rs]

[Crate badge]:http://meritbadge.herokuapp.com/cat_audio
[crates.io]:https://crates.io/crates/cat_audio

[Doc badge]:https://img.shields.io/badge/documentation-doc.rs-green
[doc.rs]:https://docs.rs/cat_audio/

A multichannel audio engine for CatEngine.

#### Supported platforms
 - Windows, Linux
 - MacOS, iOS, Android might work anyway

#### Rust features
 - `audio` - enables the audio output engine, standard functions
 - `extended_audio` - enables the audio output engine with direct command acces to the core as an addition, standard functions remain
 - `raw_audio` - enables the audio output engine only with direct command acces to the core, standard functions are removed


#### Dependencies
 - [cpal](https://github.com/RustAudio/cpal) and [minimp3](https://github.com/germangb/minimp3-rs)

Alsa delevopment libraries are required for Linux: `libasound2-dev` for Debian and Ubuntu, `alsa-lib-devel` for Fedora (see [cpal](https://github.com/RustAudio/cpal)).