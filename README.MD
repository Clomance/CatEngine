# ![logo](https://github.com/Clomance/CatEngine/raw/master/logo_400x400.png)

# CatEngine [![Crate badge]][crates.io] [![Doc badge]][doc.rs]

[Crate badge]:http://meritbadge.herokuapp.com/cat_engine
[crates.io]:https://crates.io/crates/cat_engine

[Doc badge]:https://img.shields.io/badge/documentation-doc.rs-green
[doc.rs]:https://docs.rs/cat_engine/


A 2D graphics engine with audio support for Windows and Linux.

Uses OpenGL 2.0 and above.

Supported platforms:
 - Windows, Linux
 - Mac, iOS might work anyway

#### Features

You can disable some unnecessary modules with Rust features (see [list of features](book/EN/features.md)).


#### Dependencies
 - Graphics - [glium](https://github.com/glium/glium)
 - Images - [image](https://github.com/image-rs/image)
 - Fonts - [ttf-parser](https://github.com/RazrFalcon/ttf-parser) and [ab_glyph_rasterizer](https://github.com/alexheretic/ab-glyph)
 - Audio - [cpal](https://github.com/RustAudio/cpal) and [minimp3](https://github.com/germangb/minimp3-rs)

If you use audio feature, alsa delevopment libraries are required for Linux: `libasound2-dev` for Debian and Ubuntu, `alsa-lib-devel` for Fedora (see [cpal](https://github.com/RustAudio/cpal)).


<!-- ## [My own game](https://github.com/Clomance/Visual-Novel) -->

## [Release notes](RELEASE-NOTES.MD)

## [Pre-release](https://github.com/Clomance/CatEngine/tree/pre-release)

## [The book](book/README.MD)