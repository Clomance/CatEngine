use crate::{
    Source,
    SampleTransform
};

use cat_engine_basement::utility::{sync_raw_ptr::SyncRawPtr, math::gcd};

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum PlayType{
    /// Stop playing.
    None,

    /// 
    PausedOnce,

    /// 
    PausedRepeat,

    /// 
    PausedForever,

    /// 
    PausedStream,

    /// Play once.
    Once,

    /// Repeat n-times.
    Repeat,

    /// Repeat forever.
    Forever,

    /// Wait for new samples.
    Stream
}

pub struct PlayState{
    /// Режим повторений.
    play_type:PlayType,

    /// Повторений осталось.
    repeats:u32
}

impl PlayState{
    pub fn none()->PlayState{
        Self{
            play_type:PlayType::None,
            repeats:0u32
        }
    }
}

pub struct PlaySettings{
    /// Тип прогрывания.
    state:PlayState,

    /// Громкость.
    volume:f32,

    /// Шаг.
    step:usize,

    /// Номер канала.
    channel:usize,
}

impl PlaySettings{
    pub fn new()->PlaySettings{
        Self{
            state:PlayState::none(),
            volume:0f32,
            step:0usize,
            channel:0usize
        }
    }
}

pub struct SourceIter{
    data:SyncRawPtr<Source>,

    settings:PlaySettings,

    len:usize,
    cursor:usize,

    converter_enabled:bool,
    rate_converter:RateConverter
}

impl SourceIter{
    pub fn empty()->SourceIter{
        Self{
            data:SyncRawPtr::null(),
            settings:PlaySettings::new(),

            len:0usize,
            cursor:0usize,

            converter_enabled:false, // Определяется позже
            rate_converter:RateConverter::default()
        }
    }

    /// Установка новой итерации трека.
    pub fn set_track(&mut self,track:&Source,step:usize,channel:usize,system_sample_rate:u32,repeats:u32,volume:f32){
        self.converter_enabled=false;

        self.data=SyncRawPtr::new(&track);

        self.settings.step=step;
        self.settings.channel=channel;
        self.settings.volume=volume;

        self.cursor=channel;

        self.len=track.len();

        match repeats{
            0=>self.settings.state.play_type=PlayType::Forever,
            1=>self.settings.state.play_type=PlayType::Once,
            _=>{
                self.settings.state.play_type=PlayType::Repeat;
                self.settings.state.repeats=repeats
            }
        }

        let track_sample_rate=self.sample_rate();

        // Проверка частоты и настройка конвертера
        if system_sample_rate!=track_sample_rate{
            self.rate_converter.current_sample_pos_in_chunk=0;
            self.rate_converter.next_output_sample_pos_in_chunk=0;

            // Вычисление максимального общего делителя
            let gcd=gcd(track_sample_rate,system_sample_rate);

            self.rate_converter.from=track_sample_rate/gcd;
            self.rate_converter.to=system_sample_rate/gcd;

            self.rate_converter.current_sample=self.next_sample();
            self.rate_converter.next_sample=self.next_sample();

            self.converter_enabled=true;
        }
    }

    pub fn sample_rate(&self)->u32{
        self.data.as_ref().sample_rate()
    }

    pub fn play_type(&self)->PlayType{
        self.settings.state.play_type
    }

    /// Снимает трек с паузы.
    /// 
    /// Если уже проигрывается, ничего не происходит.
    pub fn unpause(&mut self){
        self.settings.state.play_type=
            match self.settings.state.play_type{
                PlayType::PausedOnce=>PlayType::Once,

                PlayType::PausedRepeat=>PlayType::Repeat,

                PlayType::PausedForever=>PlayType::Forever,

                PlayType::PausedStream=>PlayType::Stream,

                _=>return
            }
    }

    /// Ставит трек на паузу.
    /// 
    /// Если уже на паузе, ничего не происходит.
    pub fn pause(&mut self){
        self.settings.state.play_type=
            match self.settings.state.play_type{
                PlayType::Once=>PlayType::PausedOnce,

                PlayType::Repeat=>PlayType::PausedRepeat,

                PlayType::Forever=>PlayType::PausedForever,

                PlayType::Stream=>PlayType::PausedStream,

                _=>return
            }
    }

    /// Останавливает трек без возвожности возобновления.
    pub fn stop(&mut self){
        self.settings.state.play_type=PlayType::None
    }

    fn next_input_sample(&mut self){
        self.rate_converter.current_sample_pos_in_chunk+=1;

        std::mem::swap(&mut self.rate_converter.current_sample,&mut self.rate_converter.next_sample);

        self.rate_converter.next_sample=self.next_sample();
    }

    pub fn next_sample(&mut self)->f32{
        match self.play_type(){
            PlayType::Once=>{
                if self.cursor==self.len-self.settings.step{
                    self.settings.state.play_type=PlayType::None
                }
            }

            PlayType::Repeat=>{
                if self.cursor==self.len{
                    self.settings.state.repeats-=1;
                    if self.settings.state.repeats==0{
                        self.settings.state.play_type=PlayType::None
                    }
                    self.cursor=self.settings.channel;
                }
            }

            PlayType::Forever=>{
                if self.cursor==self.len{
                    self.cursor=self.settings.channel
                }
            }

            // Паузы и остановка
            _=>return 0f32,
        }

        let sample=unsafe{self.data.as_ref().data().get_unchecked(self.cursor)};

        self.cursor+=self.settings.step;

        sample.into_f32()*self.settings.volume
    }

    /// Перевод в нужную частоту.
    pub fn next_converter_sample(&mut self)->f32{
        // The frame we are going to return from this function will be a linear interpolation
        // between `self.current_frame` and `self.next_sample`.

        if self.rate_converter.next_output_sample_pos_in_chunk>=self.rate_converter.to{
            // If we jump to the next frame, we reset the whole state.
            self.rate_converter.next_output_sample_pos_in_chunk=0;

            self.next_input_sample();
            while self.rate_converter.current_sample_pos_in_chunk<self.rate_converter.from{
                self.next_input_sample();
            }
            self.rate_converter.current_sample_pos_in_chunk=0;
        }
        else{
            // Finding the position of the first sample of the linear interpolation.
            let req_left_sample=(
                self.rate_converter.from*self.rate_converter.next_output_sample_pos_in_chunk/self.rate_converter.to
            )%self.rate_converter.from;

            // Advancing `self.current_frame`, `self.next_sample` and
            // `self.current_sample_pos_in_chunk` until the latter variable
            // matches `req_left_sample`.
            while self.rate_converter.current_sample_pos_in_chunk<req_left_sample{
                self.next_input_sample();
            }
        }

        // Merging `self.current_frame` and `self.next_sample` into `self.output_buffer`.
        // Note that `self.output_buffer` can be truncated if there is not enough data in
        // `self.next_sample`.
        //let mut result = None;
        let numerator=(self.rate_converter.from*self.rate_converter.next_output_sample_pos_in_chunk)%self.rate_converter.to;

        let sample=SampleTransform::lerp(self.rate_converter.current_sample,self.rate_converter.next_sample,numerator,self.rate_converter.to);

        // Incrementing the counter for the next iteration.
        self.rate_converter.next_output_sample_pos_in_chunk+=1;

        sample
    }

    /// Возвращает следующее значение итератора или 
    /// уведомляет о его завершении.
    pub fn next(&mut self)->Option<f32>{
        if let PlayType::None=self.settings.state.play_type{
            None
        }
        else{
            Some(
                if self.converter_enabled{
                    self.next_converter_sample()
                }
                else{
                    self.next_sample()
                }
            )
        }
    }
}

#[derive(Default)]
pub struct RateConverter{
    /// Сокращённая частота трека - используется для подсчёта шагов.
    /// We convert chunks of `from` samples into chunks of `to` samples.
    from:u32,

    /// Сокращённая частота системы - используется для подсчёта шагов.
    /// We convert chunks of `from` samples into chunks of `to` samples.
    to:u32,

    /// Текущие значение - нужно для расчёта значения волны между текущим и следующим
    current_sample:f32,

    /// Position of `current_sample` modulo `from`.
    current_sample_pos_in_chunk:u32,

    /// Слудующее значение - читай `current_frame`
    /// The samples right after `current_sample` (one per channel), extracted from `input`.
    next_sample:f32,

    /// The position of the next sample that the iterator should return, modulo `to`.
    /// This counter is incremented (modulo `to`) every time the iterator is called.
    next_output_sample_pos_in_chunk:u32,
}