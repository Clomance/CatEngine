use super::sample::SampleTransform;

use std::path::Path;
use std::fs::File;

use minimp3::Decoder;

use cpal::{
    Sample as CSample,
    SampleRate,
    SampleFormat,
};

/// Результат загрузки трека.
/// 
/// The result of loading a track.
#[derive(Debug)]
pub enum TrackResult<T>{
    Ok(Track<T>),
    FileError(std::io::Error),
    NoData,
}

impl<T:std::fmt::Debug> TrackResult<T>{
    /// Паникует, если результат не `Ok`.
    /// 
    /// Panics, if the result isn't `Ok`.
    pub fn unwrap(self)->Track<T>{
        if let TrackResult::Ok(track)=self{
            track
        }
        else{
            panic!("{:?}",self)
        }
    }

    /// Паникует и выдаёт сообщение, если результат не `Ok`.
    /// 
    /// Panics and prints the message, if the result isn't `Ok`.
    pub fn expect(self,msg:&str)->Track<T>{
        if let TrackResult::Ok(track)=self{
            track
        }
        else{
            panic!("{:?} {}",self,msg)
        }
    }
}

/// Аудио трек.
/// 
/// Audio track.
#[derive(Clone,Debug)]
pub struct Track<T>{
    data:Vec<T>,
    channels:u16,
    sample_rate:u32,
}

impl Track<i16>{
    pub fn new<P:AsRef<Path>>(path:P)->TrackResult<i16>{
        let mut data=Vec::new();

        let file=match File::open(path){
            Ok(file)=>file,
            Err(e)=>return TrackResult::FileError(e),
        };

        let mut decoder=Decoder::new(file);
        let (channels,sample_rate)=match decoder.next_frame(){
            Ok(mut frame)=>{
                data.append(&mut frame.data);
                (
                    frame.channels,
                    frame.sample_rate as u32,
                )
            }
            Err(_)=>return TrackResult::NoData
        };

        while let Ok(mut frame)=decoder.next_frame(){
            data.append(&mut frame.data);
        }

        TrackResult::Ok(Self{
            data,
            channels:channels as u16,
            sample_rate,
        })
    }
}

impl<T:Clone+SampleTransform> Track<T>{
    pub fn data(&self)->&Vec<T>{
        &self.data
    }

    pub fn channels(&self)->u16{
        self.channels
    }

    pub fn sample_rate(&self)->u32{
        self.sample_rate
    }

    pub fn len(&self)->usize{
        self.data.len()
    }
}

/// - For `i16`, silence corresponds to the value `0`. The minimum and maximum amplitudes are
///   represented by `i16::min_value()` and `i16::max_value()` respectively.
/// - For `u16`, silence corresponds to the value `u16::max_value() / 2`. The minimum and maximum
///   amplitudes are represented by `0` and `u16::max_value()` respectively.
/// - For `f32`, silence corresponds to the value `0.0`. The minimum and maximum amplitudes are
///  represented by `-1.0` and `1.0` respectively.

impl Into<Track<u16>> for Track<i16>{
    fn into(self)->Track<u16>{
        let mut track=Vec::<u16>::with_capacity(self.len());

        for sample in self.data.into_iter(){
            let sample=CSample::to_u16(&sample);
            track.push(sample);
        }

        Track::<u16>{
            data:track,
            channels:self.channels,
            sample_rate:self.sample_rate,
        }
    }
}

impl Into<Track<f32>> for Track<i16>{
    fn into(self)->Track<f32>{
        let mut track=Vec::<f32>::with_capacity(self.len());

        for sample in self.data.into_iter(){
            let sample=CSample::to_f32(&sample);
            track.push(sample);
        }

        Track::<f32>{
            data:track,
            channels:self.channels,
            sample_rate:self.sample_rate,
        }
    }
}