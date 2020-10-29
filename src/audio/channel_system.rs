use crate::support::SyncRawMutPtr;

use super::{
    MonoTrack,
    TrackIter,
    SampleTransform,
};

pub struct ChannelSystem{
    /// Выходящая частота дискретизации.
    sample_rate:u32,
    // 
    tracks:Vec<TrackIter>,

    rate_converters:Vec<RateConverter>,

    channels:Vec<Vec<usize>>,

    /// Массива каналов.
    channel_frame:Vec<f32>
}


impl ChannelSystem{
    pub fn new(sample_rate:u32,tracks_cap:usize,channels:usize)->ChannelSystem{
        Self{
            sample_rate,

            tracks:Vec::with_capacity(tracks_cap),

            rate_converters:Vec::with_capacity(tracks_cap),

            channels:Vec::with_capacity(tracks_cap),

            channel_frame:vec![0f32;channels],
        }
    }

    pub fn set_sample_rate(&mut self,sample_rate:u32){
        self.sample_rate=sample_rate;

        for rate_converter in &mut self.rate_converters{
            rate_converter.set_sample_rate(sample_rate)
        }
    }

    pub fn add_track(&mut self,track:&MonoTrack,channels:Vec<usize>,repeats:u32){
        let index=self.tracks.len();
        let iter=TrackIter::new(track,repeats);

        self.tracks.push(iter);
        let track_iter=&mut self.tracks[index];

        let converter=RateConverter::new(track.sample_rate(),self.sample_rate,track_iter);

        self.rate_converters.push(converter);
        self.channels.push(channels);
    }

    pub fn next_frame(&mut self)->&Vec<f32>{
        for channel in &mut self.channel_frame{
            *channel=0f32;
        }

        for c in 0..self.tracks.len(){
            let converter=&mut self.rate_converters[c];

            let channels=&self.channels[c];

            // Перебор индексов каналов
            for &channel in channels{
                self.channel_frame[channel]+=converter.next();
            }
        }

        &self.channel_frame
    }
}


/// Taken from rodio and modified (I will create my own later)
/// 
/// Iterator that converts from a certain sample rate to another.
struct RateConverter{
    /// Трек из которого беруться данные
    track:SyncRawMutPtr<TrackIter>,
    /// We convert chunks of `from` samples into chunks of `to` samples.
    from:u32,
    /// We convert chunks of `from` samples into chunks of `to` samples.
    to:u32,
    /// One sample per channel, extracted from `input`.
    current_frame:f32,
    /// Position of `current_sample` modulo `from`.
    current_frame_pos_in_chunk:u32,
    /// The samples right after `current_sample` (one per channel), extracted from `input`.
    next_frame:f32,
    /// The position of the next sample that the iterator should return, modulo `to`.
    /// This counter is incremented (modulo `to`) every time the iterator is called.
    next_output_frame_pos_in_chunk:u32,
}

impl RateConverter{
    ///
    ///
    /// # Panic
    ///
    /// Panicks if `from` or `to` are equal to 0.
    ///
    pub fn new(
        from:u32,
        to:u32,
        track:&mut TrackIter,
    )->RateConverter{
        assert!(from >= 1);
        assert!(to >= 1);

        // finding greatest common divisor
        let gcd={
            fn gcd(a:u32,b:u32)->u32{
                if b==0{
                    a
                }
                else{
                    gcd(b,a%b)
                }
            }

            gcd(from,to)
        };

        let (first_samples,next_samples)=if from!=to{
            (track.next(),
            track.next())
        }
        else{
            (0f32,0f32)
        };

        RateConverter{
            track:SyncRawMutPtr::new(track),
            from:from/gcd,
            to:to/gcd,
            current_frame_pos_in_chunk:0,
            next_output_frame_pos_in_chunk:0,
            current_frame:first_samples,
            next_frame:next_samples,
        }
    }

    pub fn set_sample_rate(&mut self,sample_rate:u32){
        let track_sample_rate=self.track.as_ref().sample_rate();

        let gcd={
            fn gcd(a:u32,b:u32)->u32{
                if b==0{
                    a
                }
                else{
                    gcd(b,a%b)
                }
            }

            gcd(track_sample_rate,sample_rate)
        };

        self.from=track_sample_rate/gcd;
        self.to=sample_rate/gcd;
    }

    fn next_input_frame(&mut self){
        self.current_frame_pos_in_chunk+=1;

        std::mem::swap(&mut self.current_frame,&mut self.next_frame);

        self.next_frame=self.track.as_mut().next();
    }

    pub fn next(&mut self)->f32{
        if self.from==self.to{
            return self.track.as_mut().next()
        }

        // The frame we are going to return from this function will be a linear interpolation
        // between `self.current_frame` and `self.next_frame`.

        if self.next_output_frame_pos_in_chunk>=self.to{
            // If we jump to the next frame, we reset the whole state.
            self.next_output_frame_pos_in_chunk=0;

            self.next_input_frame();
            while self.current_frame_pos_in_chunk<self.from{
                self.next_input_frame();
            }
            self.current_frame_pos_in_chunk=0;
        }
        else{
            // Finding the position of the first sample of the linear interpolation.
            let req_left_sample=(
                self.from*self.next_output_frame_pos_in_chunk/self.to
            )%self.from;

            // Advancing `self.current_frame`, `self.next_frame` and
            // `self.current_frame_pos_in_chunk` until the latter variable
            // matches `req_left_sample`.
            while self.current_frame_pos_in_chunk<req_left_sample{
                self.next_input_frame();
                debug_assert!(self.current_frame_pos_in_chunk<self.from);
            }
        }

        // Merging `self.current_frame` and `self.next_frame` into `self.output_buffer`.
        // Note that `self.output_buffer` can be truncated if there is not enough data in
        // `self.next_frame`.
        //let mut result = None;
        let numerator=(self.from*self.next_output_frame_pos_in_chunk)%self.to;

        let sample=SampleTransform::lerp(self.current_frame,self.next_frame,numerator,self.to);

        // Incrementing the counter for the next iteration.
        self.next_output_frame_pos_in_chunk+=1;

        sample
    }
}