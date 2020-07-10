# ![logo](https://github.com/Clomance/CatEngine/raw/master/logo_400x400.png)

# CatEngine [![Crate badge]][crates.io] [![Doc badge]][doc.rs]

[Crate badge]:http://meritbadge.herokuapp.com/cat_engine
[crates.io]:https://crates.io/crates/cat_engine

[Doc badge]:https://img.shields.io/badge/documentation-doc.rs-green
[doc.rs]:https://docs.rs/cat_engine/


A 2D graphics engine with audio support.

Uses OpenGL 2.0 and above.

#### Dependencies
 - Graphics - [glium](https://github.com/glium/glium)
 - Images - [image](https://github.com/image-rs/image)
 - Fonts - [rusttype](https://gitlab.redox-os.org/redox-os/rusttype)
 - Music - [cpal](https://github.com/RustAudio/cpal) and [minimp3](https://github.com/germangb/minimp3-rs)

If you use audio feature, alsa delevopment libraries are required for Linux: `libasound2-dev` for Debian and Ubuntu, `alsa-lib-devel` for Fedora (see [cpal](https://github.com/RustAudio/cpal)).

## [My own game](https://github.com/Clomance/Visual-Novel)

## [Release notes](RELEASE-NOTES.MD)

## List of features
[English version](FEATURES.MD)
[Русская версия](FEATURES-RUS.MD)

Default features = ["texture_graphics", "simple_graphics", "text_graphics"]

Other features = ["audio", "mouse_cursor_icon", "auto_hide", "fps_counter", "paged_format"]