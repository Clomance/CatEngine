use super::{
    SampleTransform,
    Track,
};

use cpal::{
    SampleFormat,
    ChannelCount,
};

enum PlayType{
    None,
    Once,
    Repeat,
    Forever,
}


pub struct PlayingTrack{
    data:Vec<Vec<f32>>,
    len:usize,

    pub channels:u16,
    pub sample_rate:u32,

    pub current_frame:usize, // current_line, frame - текущая линия (вырезка из каналов)
    play_type:PlayType,
    repeats:u32, // Повторений осталось
}

impl PlayingTrack{
    pub fn new()->PlayingTrack{
        Self{
            data:Vec::new(),
            len:0,

            channels:1,
            sample_rate:44100,

            current_frame:0,
            play_type:PlayType::None,
            repeats:0,
        }
    }

    pub fn stop(&mut self){
        self.play_type=PlayType::None
    }

    // repeat == 0 - play forever
    pub fn set_track_i16(&mut self,track:&Track<i16>,repeats:u32){
        let channels=track.channels() as usize;
        let len=track.len()/channels;
        let mut data=Vec::with_capacity(track.channels() as usize);

        for c in 0..channels{
            let mut channel=Vec::with_capacity(len);

            for &s in track.data()[c..].iter().step_by(channels){
                channel.push(s.into_f32())
            }

            data.push(channel)
        }

        self.data=data;
        self.len=len;

        self.channels=channels as u16;

        self.sample_rate=track.sample_rate();

        self.current_frame=0;

        match repeats{
            0=>self.play_type=PlayType::Forever,
            1=>self.play_type=PlayType::Once,
            _=>{
                self.play_type=PlayType::Repeat;
                self.repeats=repeats
            }
        }
    }

    // 
    pub fn next(&mut self,frame:&mut Vec<f32>){
        match self.play_type{
            PlayType::None=>{
                for c in 0..self.channels{
                    frame.push(0f32)
                }
                return
            }

            PlayType::Once=>{
                if self.current_frame==self.len-1{
                    self.play_type=PlayType::None
                }
            }

            PlayType::Repeat=>{
                if self.current_frame==self.len{
                    self.repeats-=1;
                    if self.repeats==0{
                        self.play_type=PlayType::None
                    }
                    self.current_frame=0;
                }
            }

            PlayType::Forever=>{
                if self.current_frame==self.len{
                    self.current_frame=0
                }
            }
        }

        for channel in &self.data{
            let sample=channel[self.current_frame];
            frame.push(sample)
        }

        self.current_frame+=1;
    }
}