use crate::support::SyncRawMutPtr;

use super::{
    MonoTrack,
    TrackIter,
    SampleTransform,
};

pub struct ChannelSystem{
    /// Выходящая (системная) частота дискретизации.
    sample_rate:u32,
    /// Треки с циклом повторений и переводчиком частот.
    tracks:Vec<TrackIter>,

    channels:Vec<Vec<usize>>,

    /// Массива каналов.
    channel_frame:Vec<f32>
}


impl ChannelSystem{
    pub fn new(sample_rate:u32,tracks_cap:usize,channels:usize)->ChannelSystem{
        Self{
            sample_rate,

            tracks:Vec::with_capacity(tracks_cap),

            channels:Vec::with_capacity(tracks_cap),

            channel_frame:vec![0f32;channels],
        }
    }

    /// Устанавливает новую частоту системы.
    /// Обновляет все треки.
    pub fn set_system_sample_rate(&mut self,sample_rate:u32){
        self.sample_rate=sample_rate;

        for track in &mut self.tracks{
            track.set_system_sample_rate(sample_rate)
        }
    }

    /// Добавляет моно-канальный трек для проигрывания.
    pub fn add_track(&mut self,track:&MonoTrack,channels:Vec<usize>,repeats:u32){
        let index=self.tracks.len();
        let iter=TrackIter::new(track,self.sample_rate,repeats);

        self.tracks.push(iter);
        let track_iter=&mut self.tracks[index];

        self.channels.push(channels);
    }

    /// Возвращает фрейм каналов.
    /// Распределяет все треки по каналам.
    pub fn next_frame(&mut self)->&Vec<f32>{
        // Отчистка фрейма
        for channel in &mut self.channel_frame{
            *channel=0f32;
        }

        // Перебор треков (65-66)
        for c in 0..self.tracks.len(){
            let track=&mut self.tracks[c];

            // Каналы для вывода трека
            let channels=&self.channels[c];

            // Перебор индексов каналов
            for &channel in channels{
                self.channel_frame[channel]+=track.next();
            }
        }

        &self.channel_frame
    }
}