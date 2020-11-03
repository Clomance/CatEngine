use crate::support::SyncRawMutPtr;

use super::{
    MonoTrack,
    TrackIter,
    SampleTransform,
};

use std::{
    ops::AddAssign,
    cmp::Ordering
};

pub struct ChannelSystem{
    /// Выходящая (системная) частота дискретизации.
    sample_rate:u32,
    /// Спискок проигрываемых треков
    /// с циклом повторений и переводчиком частот.
    tracks:Vec<TrackIter>,

    /// Матрица (нет) распределения треков по каналам.
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

    /// Устанавливает нового количество каналов.
    pub fn set_system_channels(&mut self,channels:u16){
        let current_channels=self.channel_frame.len();
        match current_channels.cmp(&(channels as usize)){
            Ordering::Equal=>{} // Количество каналов не изменилось

            Ordering::Greater=>unsafe{ // Количество каналов уменьшилось
                self.channel_frame.set_len(channels as usize);
            }

            Ordering::Less=>{ // Количество каналов увеличилось
                let add=channels as usize-current_channels;
                self.channel_frame.reserve_exact(add);
                for _ in 0..add{
                    self.channel_frame.push(0f32);
                }
            }
        }
    }

    /// Устанавливает громкость трека.
    pub fn set_track_volume(&mut self,index:usize,volume:f32){
        if let Some(track)=self.tracks.get_mut(index){
            track.set_volume(volume)
        }
    }

    /// Добавляет моно-канальный трек для проигрывания.
    pub fn add_track(&mut self,track:&MonoTrack,channels:Vec<usize>,repeats:u32,volume:f32){
        let index=self.tracks.len();
        let iter=TrackIter::new(track,self.sample_rate,repeats,volume);

        self.tracks.push(iter);
        let track_iter=&mut self.tracks[index];

        self.channels.push(channels);
    }

    /// Удаляет трек из списка проигрываемых.
    pub fn remove_track(&mut self,index:usize){
        if index<self.tracks.len(){
            self.tracks.remove(index);
            self.channels.remove(index);
        }
    }

    /// Удаляет все треки из списка проигрываемых.
    pub fn clear_playlist(&mut self){
        self.tracks.clear();
        self.channels.clear();
    }

    /// Возвращает фрейм каналов.
    /// Распределяет все треки по каналам.
    pub fn next_frame(&mut self)->&mut Vec<f32>{
        // Отчистка фрейма каналов
        for channel in &mut self.channel_frame{
            *channel=0f32;
        }

        // Перебор треков
        let mut c=0usize;
        'tracks:while c<self.tracks.len(){
            let track=&mut self.tracks[c];

            // Каналы для вывода трека
            let channels=&self.channels[c];

            // Перебор индексов каналов
            for &channel in channels{
                if let Some(sample)=track.next(){
                    // Добавляем значение трека в канал, если такой есть
                    if let Some(channel)=self.channel_frame.get_mut(channel){
                        channel.add_assign(sample);
                    }
                }
                else{
                    // Удаление завершённых треков
                    // (полностью проигранных)
                    self.remove_track(c);
                    continue 'tracks
                }
            }
            c+=1;
        }

        &mut self.channel_frame
    }
}