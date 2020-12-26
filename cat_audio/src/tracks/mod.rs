mod track;
pub use track::*;

/// Результат загрузки трека.
/// 
/// The result of loading a track.
#[derive(Debug)]
pub enum TrackResult<T>{
    Ok(T),
    FileError(std::io::Error),
    NoData,
}

impl<T:std::fmt::Debug> TrackResult<T>{
    /// Паникует, если результат не `Ok`.
    /// 
    /// Panics, if the result isn't `Ok`.
    pub fn unwrap(self)->T{
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
    pub fn expect(self,msg:&str)->T{
        if let TrackResult::Ok(track)=self{
            track
        }
        else{
            panic!("{:?} {}",self,msg)
        }
    }
}

/// Одноканальный трек.
/// A mono-channel track.
pub struct MonoTrack{
    pub data:Vec<f32>,
    pub sample_rate:u32,
}

impl MonoTrack{
    pub fn new(data:Vec<f32>,sample_rate:u32)->MonoTrack{
        Self{
            data,
            sample_rate,
        }
    }

    pub fn len(&self)->usize{
        self.data.len()
    }

    pub fn sample_rate(&self)->u32{
        self.sample_rate
    }
}

pub struct TrackSet{
    /// Индекс трека.
    /// 
    /// A track index.
    pub index:usize,
    /// Каналы для распределения.
    /// 
    /// Output channels.
    pub channels:Vec<usize>,
    /// Количество повторенй.
    /// 
    /// 0 - постоянно
    /// 
    /// The amount of repeats.
    /// 
    /// 0 - forever
    pub repeats:u32,
    /// Громкость трека.
    /// 
    /// A track volume.
    pub volume:f32,
}

impl TrackSet{
    pub fn once(index:usize,channels:Vec<usize>)->TrackSet{
        Self{
            index,
            channels,
            repeats:1u32,
            volume:1f32,
        }
    }
}