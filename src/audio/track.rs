use super::sample::SampleTransform;

use std::path::Path;
use std::fs::File;

use minimp3::Decoder;

use cpal::Sample;

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

/// Аудио трек.
/// An audio track.
#[derive(Clone,Debug)]
struct Track<T:Clone+SampleTransform>{
    data:Vec<T>,
    channels:u16,
    sample_rate:u32,
}

impl Track<i16>{
    pub fn new<P:AsRef<Path>>(path:P)->TrackResult<Track<i16>>{
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

    /// Делит трек на одноканальные треки.
    /// 
    /// Devides a track into mono-channel tracks.
    pub fn to_mono_tracks(&self)->Vec<MonoTrack>{
        let channels=self.channels as usize;
        let len=self.len()/channels;
        let mut tracks=Vec::with_capacity(channels);

        for c in 0..channels{
            let mut channel=Vec::with_capacity(len);

            for &s in self.data()[c..].iter().step_by(channels){
                channel.push(s.into_f32());
            }

            tracks.push(MonoTrack{
                sample_rate:self.sample_rate,
                data:channel,
            })
        }

        tracks
    }
}

impl Into<Track<u16>> for Track<i16>{
    fn into(self)->Track<u16>{
        let mut track=Vec::<u16>::with_capacity(self.len());

        for sample in self.data.into_iter(){
            let sample=Sample::to_u16(&sample);
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
            let sample=Sample::to_f32(&sample);
            track.push(sample);
        }

        Track::<f32>{
            data:track,
            channels:self.channels,
            sample_rate:self.sample_rate,
        }
    }
}

/// Аудио трек с отдельными каналами.
/// An audio track with separate channels.
#[derive(Debug,Clone)]
pub struct ChanneledTrack{
    /// Разделённые каналы трека.
    channels:Vec<Vec<f32>>,
    /// Каналы распределения трека
    /// (Номера аудио каналов для вывода).
    /// 
    /// Данные трека распределяются
    /// по номерам аудио каналов при выводе.
    output_channels:Vec<Vec<usize>>,
    /// Частота дискретизации.
    sample_rate:u32,
}

impl ChanneledTrack{
    pub fn new<P:AsRef<Path>>(path:P)->TrackResult<ChanneledTrack>{
        let track=match Track::new(path){
            TrackResult::Ok(track)=>track,
            TrackResult::NoData=>return TrackResult::NoData,
            TrackResult::FileError(err)=>return TrackResult::FileError(err),
        };
        
        let monos=track.to_mono_tracks();

        let sample_rate=track.sample_rate();
        let channels_amount=track.channels() as usize;

        let mut channels:Vec<Vec<f32>>=Vec::with_capacity(channels_amount);

        let mut output_channels:Vec<Vec<usize>>=Vec::with_capacity(channels_amount);

        for (c,mono) in monos.into_iter().enumerate(){
            let output_channel=vec![c];
            output_channels.push(output_channel);
            channels.push(mono.data);
        }

        // Открытие файла и создание декодеровщика
        // let mut decoder=match File::open(path){
        //     Ok(file)=>Decoder::new(file),
        //     Err(e)=>return TrackResult::FileError(e),
        // };

        // // Массив отдельных каналов
        // let mut channels:Vec<Vec<f32>>;

        // // Частота фрейма - берётся как абсолютная
        // let sample_rate:u32;

        // // Каналы распределения
        // let mut output_channels:Vec<Vec<usize>>;

        // // Инициализация каналов трека и проверка файла
        // match decoder.next_frame(){
        //     Ok(frame)=>{
        //         // Создание каналов распределения
        //         output_channels=Vec::with_capacity(frame.channels);
        //         for c in 0..frame.channels{
        //             output_channels.push(vec![c])
        //         }

        //         sample_rate=frame.sample_rate as u32;

        //         // Размер каналов фрейма
        //         let len=frame.data.len()/frame.channels;

        //         // Выделение памяти под каналы трека
        //         channels=Vec::with_capacity(frame.channels);

        //         let data=frame.data;

        //         // Распределение данных по отдельным каналам
        //         for c in 0..frame.channels{
        //             channels.push(Vec::with_capacity(len));

        //             let step=frame.channels as isize;

        //             let track_channel=&mut channels[c];

        //             for &s in data[c..].iter().step_by(frame.channels){
        //                 track_channel.push(s.into_f32());
        //             }
        //         }
        //     }
        //     Err(_)=>return TrackResult::NoData
        // }

        // Считывание оставшихся данных
        //load_mp3_separate(decoder,&mut channels);

        TrackResult::Ok(Self{
            channels,
            output_channels,
            sample_rate
        })
    }

    pub fn channels(&self)->usize{
        self.channels.len()
    }

    pub fn sample_rate(&self)->u32{
        self.sample_rate
    }

    /// Возвращает канал.
    /// 
    /// Returns a channel.
    pub fn get_channel(&self,index:usize)->Option<&Vec<f32>>{
        self.channels.get(index)
    }

    /// Возвращает канал.
    /// 
    /// Returns a channel.
    pub fn get_mut_channel(&mut self,index:usize)->Option<&mut Vec<f32>>{
        self.channels.get_mut(index)
    }

    pub fn into_iter(self)->std::iter::Zip<std::vec::IntoIter<Vec<f32>>,std::vec::IntoIter<Vec<usize>>>{
        self.channels.into_iter().zip(self.output_channels.into_iter())
    }
}

fn load_mp3_separate(mut decoder:Decoder<File>,channels:&mut Vec<Vec<f32>>){
    while let Ok(frame)=decoder.next_frame(){
        let len=frame.data.len()/frame.channels;

        // Распределение данных по отдельным каналам
        for c in 0..frame.channels{
            let track_channel=&mut channels[c];
            // Выделение памяти под канал трека
            track_channel.reserve_exact(len);

            // Вписывание данных в канал
            for &s in frame.data[c..].iter().step_by(frame.channels){
                track_channel.push(s.into_f32());
            }
        }
    }
}

/// Одноканальный трек.
/// A mono channel track.
pub struct MonoTrack{
    pub data:Vec<f32>,
    pub sample_rate:u32,
}

impl MonoTrack{
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