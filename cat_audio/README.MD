# ![logo](https://github.com/Clomance/CatEngine/cat_audio/raw/master/cat_audio_logo_400_400.png)

# CatAudio [![Crate badge]][crates.io] [![Doc badge]][doc.rs]

[Crate badge]:http://meritbadge.herokuapp.com/cat_audio
[crates.io]:https://crates.io/crates/cat_audio

[Doc badge]:https://img.shields.io/badge/documentation-doc.rs-green
[doc.rs]:https://docs.rs/cat_audio/

An audio engine for CatEngine.

Supported platforms:
 - Windows, Linux
 - MacOS, iOS, Android might work anyway

#### Features

You can disable some unnecessary modules with Rust features (see [list of features](book/EN/features.md)).


#### Dependencies
[cpal](https://github.com/RustAudio/cpal) and [minimp3](https://github.com/germangb/minimp3-rs)

Alsa delevopment libraries are required for Linux: `libasound2-dev` for Debian and Ubuntu, `alsa-lib-devel` for Fedora (see [cpal](https://github.com/RustAudio/cpal)).