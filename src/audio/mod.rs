//! # Простой аудио движок. A simple audio engine. `feature = "audio"`.
//! 
//! Аудио движок имеет свой поток для работы со звуком.
//! Также в нём есть массив аудио треков, которые можно запустить.
//! 
//! Использует только один канал для проигрывания треков
//! и только формат `mp3`.
//! 
//! Поддерживает Mono и Stereo 2.0.
//! 
//! Поток закрывается с паникой, так что не паникуте!
//! 
//! Некоторый код был взят из [rodio](https://github.com/RustAudio/rodio).
//! 
//! При переключении трека в режиме паузы, слышен странный звук.
//! 
//! #
//! 
//! The audio engine has it's own thread for working with sound.
//! Also it has audio track array.
//! 
//! Uses only one stream for playing tracks
//! and only `mp3` format.
//! 
//! The audio engine converts all sound tracks to the 24-bit format.
//! 
//! Supports Mono and Stereo 2.0.
//! 
//! The thread closes with panic, so don't panic!
//! 
//! Some code was taken from [rodio](https://github.com/RustAudio/rodio).
//! 
//! A strange sound appears when a track is switched in pause.
//! 
//! #
//! 
//! ```
//! let settings=AudioSettings::new();
//! 
//! let host=cpal::default_host();
//! 
//! let audio=Audio::new(host,|host|{
//!     host.default_output_device().unwrap()
//! },settings.clone()).unwrap();
//! 
//! audio.add_track("audio.mp3"); // a track index = 0
//! 
//! audio.play_track(0,1); // plays the track once
//! ```

// re-import
pub use cpal;

mod engine;
pub use engine::Audio;

mod play_buffer;
use play_buffer::PlayingTrack;

mod rate_converter;
use rate_converter::RateConverter;

mod track;
mod sample;

use track::*;
use sample::SampleTransform;

use cpal::{
    Host,
    HostId,
    HostUnavailable,
    Device,
    DevicesError,
    Devices,
    OutputDevices,
    traits::{
        HostTrait,
        DeviceTrait,
        EventLoopTrait
    },
    UnknownTypeOutputBuffer,
    StreamData,
    StreamId,
    EventLoop,
    Sample,
    Format,
    StreamError,
};

use std::{
    io,
    vec::IntoIter,
    iter::Cycle,
    path::Path,
    thread::{Builder,JoinHandle},
    sync::{
        Arc,
        Mutex,
        LockResult,
        mpsc::{Sender,Receiver,channel},
    },
};

enum AudioSystemCommand{
    /// A track index - usize,
    /// Repeats - u32:
    /// 0 - forever, 1 - once, 2.. - repeat twice and so on
    Play((usize,u32)),

    Stop,

    SetVolume(f32),
    Close,
}

/// Результат выполнения команды. The result of command accomplishing.
#[derive(Clone,Debug,PartialEq)]
pub enum AudioCommandResult{
    Ok,
    NoSuchTrack,
    ThreadClosed,
    TrackError,
}

impl AudioCommandResult{
    /// Паникует, если результат не `Ok`.
    /// 
    /// Panics if the result isn't `Ok`.
    pub fn unwrap(self){
        if self!=AudioCommandResult::Ok{
            panic!("{:?}",self)
        }
    }

    /// Паникует и выводит сообщение, если результат не `Ok`.
    /// 
    /// Panics и prints the message if the result isn't `Ok`.
    pub fn expect(self,msg:&str){
        if self!=AudioCommandResult::Ok{
            panic!("{} {:?}",msg,self)
        }
    }
}

unsafe impl std::marker::Sync for AudioSystemCommand{}
unsafe impl std::marker::Send for AudioSystemCommand{}

/// Тип аудио вывода. Audio output type.
#[derive(Clone,Copy)]
pub enum AudioOutputFormat{
    Mono,
    Stereo, // 2.0
}

impl AudioOutputFormat{
    pub fn into_channels(self)->u16{
        match self{
            AudioOutputFormat::Mono=>1u16,
            AudioOutputFormat::Stereo=>2u16,
        }
    }
}

#[derive(Clone)]
pub struct AudioSettings{
    /// Громкость. По умолчанию 0.5.
    /// 
    /// The default is 0.5.
    pub volume:f32,

    /// The default is Stereo.
    pub output_format:AudioOutputFormat,

    /// Вместимость массива для треков.
    /// 
    /// The capacity of the track array.
    /// 
    /// The default is 1.
    pub track_array_capacity:usize,
}

impl AudioSettings{
    pub fn new()->AudioSettings{
        Self{
            volume:0.5f32,
            output_format:AudioOutputFormat::Stereo,
            track_array_capacity:1,
        }
    }
}


enum ChannelName{
    FrontLeft,
    FrontRight,
    FrontCenter,
    // LowFrequency
    BackLeft,
    BackRight,
}