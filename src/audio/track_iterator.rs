use crate::support::{
    math::gcd,
    SyncRawPtr,
    SyncRawMutPtr
};

use super::{
    SampleTransform,
    MonoTrack
};

enum PlayType{
    None,
    Once,
    Repeat,
    Forever,
}

/// Для будущих обновлений
struct Repeats{
    /// Индекс текущего значения
    current_frame:usize,
    /// Режим повторений
    play_type:PlayType,
    /// Повторений осталось
    repeats:u32,
}

pub struct TrackIter{
    data:SyncRawPtr<Vec<f32>>,
    /// Частота дискретизации
    track_sample_rate:u32,
    /// Длина трека
    track_len:usize,
    /// Индекс текущего значения
    track_current_frame:usize,
    play_type:PlayType,
    repeats:u32, // Повторений осталось

    // Поля конвертера (не использовал Option для оптимизации и упрощения кода)

    /// Флаг включения конвертера
    converter_enabled:bool,
    /// Сокращённая частота трека - используется для подсчёта шагов.
    /// We convert chunks of `from` samples into chunks of `to` samples.
    from:u32,
    /// Сокращённая частота системы - используется для подсчёта шагов.
    /// We convert chunks of `from` samples into chunks of `to` samples.
    to:u32,
    /// Текущие значение - нужно для расчёта значения волны между текущим и следующим (взято из трека)
    current_frame:f32,
    /// Position of `current_sample` modulo `from`.
    current_frame_pos_in_chunk:u32,
    /// Слудующее значение - читай `current_frame` (взято из трека)
    /// The samples right after `current_sample` (one per channel), extracted from `input`.
    next_frame:f32,
    /// The position of the next sample that the iterator should return, modulo `to`.
    /// This counter is incremented (modulo `to`) every time the iterator is called.
    next_output_frame_pos_in_chunk:u32,
}

impl TrackIter{
    pub fn new(track:&MonoTrack,system_sample_rate:u32,repeats:u32)->TrackIter{
        let mut iter=Self{
            data:SyncRawPtr::new(&track.data),
            track_sample_rate:track.sample_rate(),

            track_len:track.len(),

            track_current_frame:0usize,
            play_type:PlayType::None,
            repeats:0u32,

            converter_enabled:false,
            from:0u32,
            to:0u32,
            current_frame:0f32,
            current_frame_pos_in_chunk:0u32,
            next_frame:0f32,
            next_output_frame_pos_in_chunk:0u32,
        };

        match repeats{
            0=>iter.play_type=PlayType::Forever,
            1=>iter.play_type=PlayType::Once,
            _=>{
                iter.play_type=PlayType::Repeat;
                iter.repeats=repeats
            }
        }

        if system_sample_rate!=iter.track_sample_rate{
            iter.converter_enabled=true;
            // Вычисление максимального общего делителя
            let gcd=gcd(iter.track_sample_rate,system_sample_rate);

            iter.from=iter.track_sample_rate/gcd;
            iter.to=system_sample_rate/gcd;

            iter.current_frame=iter.next_track_sample();
            iter.next_frame=iter.next_track_sample();
        }

        iter
    }

    pub fn sample_rate(&self)->u32{
        self.track_sample_rate
    }

    pub fn stop(&mut self){
        self.play_type=PlayType::None
    }

    pub fn set_mono_track(&mut self,track:&MonoTrack,repeats:u32){
        self.data=SyncRawPtr::new(&track.data);

        self.track_current_frame=0;

        self.track_len=track.len();

        match repeats{
            0=>self.play_type=PlayType::Forever,
            1=>self.play_type=PlayType::Once,
            _=>{
                self.play_type=PlayType::Repeat;
                self.repeats=repeats
            }
        }
    }

    pub fn set_system_sample_rate(&mut self,sample_rate:u32){
        if sample_rate==self.track_sample_rate{
            self.converter_enabled=false;
        }
        else{
            self.converter_enabled=true;
            // Вычисление максимального общего делителя
            let gcd=gcd(self.track_sample_rate,sample_rate);

            self.from=self.track_sample_rate/gcd;
            self.to=sample_rate/gcd;
        }
    }

    /// Следующее значение трека.
    pub fn next_track_sample(&mut self)->f32{
        // println!("Next sample");
        match self.play_type{
            PlayType::None=>{
                return 0f32
            }

            PlayType::Once=>{
                if self.track_current_frame==self.track_len-1{
                    self.play_type=PlayType::None
                }
            }

            PlayType::Repeat=>{
                if self.track_current_frame==self.track_len{
                    self.repeats-=1;
                    if self.repeats==0{
                        self.play_type=PlayType::None
                    }
                    self.track_current_frame=0;
                }
            }

            PlayType::Forever=>{
                if self.track_current_frame==self.track_len{
                    self.track_current_frame=0
                }
            }
        }

        let sample=self.data.as_ref()[self.track_current_frame];

        self.track_current_frame+=1;

        sample
    }

    fn next_input_frame(&mut self){
        self.current_frame_pos_in_chunk+=1;

        std::mem::swap(&mut self.current_frame,&mut self.next_frame);

        self.next_frame=self.next_track_sample();
    }

    pub fn next_converter_sample(&mut self)->f32{
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

    pub fn next(&mut self)->f32{
        if self.converter_enabled{
            self.next_converter_sample()
        }
        else{
            self.next_track_sample()
        }
    }
}