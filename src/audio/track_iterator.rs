use crate::support::{
    math::gcd,
    SyncRawPtr,
    SyncRawMutPtr
};

use super::{
    SampleTransform,
    MonoTrack,
};

enum PlayType{
    /// Перестаёт играть
    None,
    /// Пауза при однократном проигрывании
    PausedOnce,
    /// 
    PausedRepeat,
    /// 
    PausedForever,
    /// Сыграть один раз
    Once,
    /// Повторять
    Repeat,
    /// Проигрывать вечно
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

/// Итератор по треку.
/// 
/// Имеет возможность повторять трек и конвертировать его частоту.
pub struct TrackIter{
    data:SyncRawPtr<Vec<f32>>,
    /// Частота дискретизации.
    track_sample_rate:u32,
    /// Длина трека
    track_len:usize,
    /// Индекс текущего значения.
    track_current_frame:usize,
    /// Режим повторений.
    play_type:PlayType,
    /// Повторений осталось.
    repeats:u32,
    /// Громкость трека.
    volume:f32,

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
    pub fn empty()->TrackIter{
        Self{
            data:SyncRawPtr::zero(),
            track_sample_rate:0u32,

            track_len:0usize,

            track_current_frame:0usize,
            play_type:PlayType::None, // Определяется позже
            repeats:0u32, // Определяется позже

            volume:0f32,

            converter_enabled:false, // Определяется позже
            from:0u32,
            to:0u32,
            current_frame:0f32,
            current_frame_pos_in_chunk:0u32,
            next_frame:0f32,
            next_output_frame_pos_in_chunk:0u32,
        }
    }

    /// Возвращает частоту трека.
    pub fn sample_rate(&self)->u32{
        self.track_sample_rate
    }

    /// Снимает трек с паузы.
    /// 
    /// Если уже проигрывается, ничего не происходит.
    pub fn unpause(&mut self){
        self.play_type=match self.play_type{
            PlayType::PausedOnce=>PlayType::Once,

            PlayType::PausedRepeat=>PlayType::Repeat,

            PlayType::PausedForever=>PlayType::Forever,

            _=>return
        }
    }

    /// Ставит трек на паузу.
    /// 
    /// Если уже на паузе, ничего не происходит.
    pub fn pause(&mut self){
        self.play_type=match self.play_type{
            PlayType::Once=>PlayType::PausedOnce,

            PlayType::Repeat=>PlayType::PausedRepeat,

            PlayType::Forever=>PlayType::PausedForever,

            _=>return
        }
    }

    /// Останавливает трек
    /// без возвожности возобновления.
    pub fn stop(&mut self){
        self.play_type=PlayType::None
    }
}

/// Установка параметров.
impl TrackIter{
        /// Установка новой итерации трека.
        pub fn set_track(&mut self,track:&MonoTrack,system_sample_rate:u32,repeats:u32,volume:f32){
            self.data=SyncRawPtr::new(&track.data);
            self.track_sample_rate=track.sample_rate;
            self.volume=volume;
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
    
            // Проверка частоты и настройка конвертера
            if system_sample_rate!=self.track_sample_rate{
                self.current_frame_pos_in_chunk=0;
                self.next_output_frame_pos_in_chunk=0;
    
                self.converter_enabled=true;
                // Вычисление максимального общего делителя
                let gcd=gcd(self.track_sample_rate,system_sample_rate);
    
                self.from=self.track_sample_rate/gcd;
                self.to=system_sample_rate/gcd;
    
                self.current_frame=self.next_track_sample();
                self.next_frame=self.next_track_sample();
            }
            else{
                self.converter_enabled=false;
            }
        }

    /// Устанавливает частоту вывода (системную частоту).
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

    /// Устанавливает громкость трека.
    pub fn set_volume(&mut self,volume:f32){
        self.volume=volume
    }
}


/// Итерации.
impl TrackIter{
    /// Следующее значение трека.
    pub fn next_track_sample(&mut self)->f32{
        match self.play_type{
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

            // Паузы и остановка
            _=>return 0f32,
        }

        let sample=self.data.as_ref()[self.track_current_frame];

        self.track_current_frame+=1;

        sample*self.volume
    }

    fn next_input_frame(&mut self){
        self.current_frame_pos_in_chunk+=1;

        std::mem::swap(&mut self.current_frame,&mut self.next_frame);

        self.next_frame=self.next_track_sample();
    }

    /// Перевод в нужную частоту.
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

    /// Возвращает следующее значение итератора или 
    /// уведомляет о его завершении.
    pub fn next(&mut self)->Option<f32>{
        if let PlayType::None=self.play_type{
            None
        }
        else{
            Some(if self.converter_enabled{
                self.next_converter_sample()
            }
            else{
                self.next_track_sample()
            })
        }
    }
}