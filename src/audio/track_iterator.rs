use crate::support::SyncRawPtr;

use super::MonoTrack;

enum PlayType{
    None,
    Once,
    Repeat,
    Forever,
}

struct Repeats{
    current_frame:usize, // current_line, frame - текущая линия (вырезка из каналов)
    play_type:PlayType,
    repeats:u32, // Повторений осталось
}

pub struct TrackIter{
    data:SyncRawPtr<Vec<f32>>,
    /// Частота дискретизации
    sample_rate:u32,

    len:usize,

    current_frame:usize, // current_line, frame - текущая линия (вырезка из каналов)
    play_type:PlayType,
    repeats:u32, // Повторений осталось
}

impl TrackIter{
    pub fn new(track:&MonoTrack,repeats:u32)->TrackIter{
        let mut iter=Self{
            data:SyncRawPtr::new(&track.data),
            sample_rate:track.sample_rate(),

            len:track.len(),

            current_frame:0,
            play_type:PlayType::None,
            repeats:0,
        };

        match repeats{
            0=>iter.play_type=PlayType::Forever,
            1=>iter.play_type=PlayType::Once,
            _=>{
                iter.play_type=PlayType::Repeat;
                iter.repeats=repeats
            }
        }

        iter
    }

    pub fn sample_rate(&self)->u32{
        self.sample_rate
    }

    pub fn stop(&mut self){
        self.play_type=PlayType::None
    }

    pub fn set_mono_track(&mut self,track:&MonoTrack,repeats:u32){
        self.data=SyncRawPtr::new(&track.data);

        self.current_frame=0;

        self.len=track.len();

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
    pub fn next(&mut self)->f32{
        // println!("Next sample");
        match self.play_type{
            PlayType::None=>{
                return 0f32
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

        let sample=self.data.as_ref()[self.current_frame];

        self.current_frame+=1;

        sample
    }
}