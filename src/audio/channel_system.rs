//! # Система указателей
//! 
//! # переписать
//! Треки в хранилище (`tracks_container`) содержат в соответствующих списках (`iter_indices`)
//! номера итераторов в буфере (`play_buffer`). Эти итераторы по соответствующим индексам (`track_owners`)
//! ссылаются на треки в хранилище и соответстующие им номера в списках итераторов.
//! 
//! При удалении трека из хранилища всем связанным с ним итераторам в буфере ставится флаг PlayType::None,
//! который при проверке удаляет соответствующий трек из плейлиста,
//! также очищается список итераторов этого трека.
//! 
//! При удалении трека из плейлиста по соответствующим индексам находятся
//! трек в хранилище и соответстующий ему и треку из плейлиста индекс номера итератора
//! (на который тоже указывает трек из плейлиста).
//! Далее этот индекс удаляется из списка, и у всех итераторов, на которые указывают
//! выше стоящие индексы, убавляют на единицу указатели на номер итератора.

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
    /// Матрица распределения трека по буферу.
    /// Ссылка на массив списков номеров итераторов.
    iter_indices:SyncRawMutPtr<Vec<Vec<usize>>>,

    /// Выходящая (системная) частота дискретизации.
    sample_rate:u32,

    /// Индексы номеров треков в матрице распределения треков.
    /// 
    /// Иными словами
    /// для каждого итератора указывается индекс трека в хранилище и
    /// индекс номера его итератора.
    /// 
    /// первый индекс - индекс трека в хралище,
    /// второй - индекс номера его итератора
    track_owners:Vec<(usize,usize)>,

    /// Спискок проигрываемых треков
    /// с циклом повторений и конвертером частот.
    play_buffer:Vec<TrackIter>,

    /// Свободные ячейки в буфере треков.
    free_slots:Vec<usize>,

    /// Ссылки на буфер треков.
    playlist:Vec<usize>,

    /// Матрица (нет) распределения треков по каналам.
    channels:Vec<Vec<usize>>,

    /// Массива каналов.
    channel_frame:Vec<f32>
}


impl ChannelSystem{
    pub fn new(
        iter_indices:SyncRawMutPtr<Vec<Vec<usize>>>,
        sample_rate:u32,
        tracks_cap:usize,
        channels:usize
    )->ChannelSystem{
        // Индексы треков в хранилище, на которые ссылаются треки из буфера
        let mut track_owners=Vec::with_capacity(tracks_cap);
        // Буфер треков
        let mut play_buffer=Vec::with_capacity(tracks_cap);
        let mut free_slots=Vec::with_capacity(tracks_cap);

        for c in 0..tracks_cap{
            track_owners.push((0usize,0usize));
            play_buffer.push(TrackIter::empty());
            free_slots.push(c);
        }

        Self{
            iter_indices,

            sample_rate,

            track_owners,

            play_buffer,
            free_slots,

            playlist:Vec::with_capacity(tracks_cap),

            channels:Vec::with_capacity(tracks_cap),

            channel_frame:vec![0f32;channels],
        }
    }

    /// Ставит трек из плейлиста проигрываться.
    /// 
    /// Если уже проигрывается или нет такого трека,
    /// ничего не происходит.
    pub fn play(&mut self,index:usize){
        if let Some(&index)=self.playlist.get(index){
            self.play_buffer[index].play();
        }
    }

    /// Ставит трек из плейлиста на паузу.
    /// 
    /// Если уже на паузе или нет такого трека,
    /// ничего не происходит.
    pub fn pause(&mut self,index:usize){
        if let Some(&index)=self.playlist.get(index){
            self.play_buffer[index].pause();
        }
    }

    /// Останавливает итератор
    /// без возвожности возобновления.
    /// 
    /// Если уже остановлен или нет такого итератора,
    /// ничего не происходит.
    pub fn stop_iter(&mut self,index:usize){
        if let Some(iter)=self.play_buffer.get_mut(index){
            iter.stop();
        }
    }

    /// Удаляет все треки из плейлиста.
    pub fn clear_playlist(&mut self){
        for iters in self.iter_indices.as_mut(){
            iters.clear()
        }
        // Занесение пустых слотов в очередь
        unsafe{
            self.free_slots.set_len(self.free_slots.capacity());
            for (c,slot) in self.free_slots.iter_mut().enumerate(){
                *slot=c;
            }
        }
        self.playlist.clear();
        self.channels.clear();
    }
}

/// Добавление и удаление треков.
impl ChannelSystem{
    /// Добавляет моно-канальный трек для проигрывания.
    /// 
    /// Если плейлист переполнен, то ничего не происходит.
    pub fn add_track(
        &mut self,
        owner_index:usize,
        track:&MonoTrack,
        channels:Vec<usize>,
        repeats:u32,
        volume:f32
    ){
        // Выбор свободного слота (номер итератора в буфере)
        if let Some(iter_index)=self.free_slots.pop(){
            // Ссылка на итератор
            let iter=&mut self.play_buffer[iter_index];
            // Установка трека в итератор
            iter.set_track(track,self.sample_rate,repeats,volume);

            // Ссылка на список номеров итераторов
            let iter_indices=&mut self.iter_indices.as_mut()[owner_index];

            // Запись индекса трека в хранилище и индекса номера его итератора
            self.track_owners[iter_index]=(owner_index,iter_indices.len());

            // Добавление номера итератора в список номеров итераторов
            iter_indices.push(iter_index);

            // Добавление индекса трека в буфере
            self.playlist.push(iter_index);
            // Добавление каналов трека
            self.channels.push(channels);
        }
    }

    /// Удаляет трек из плейлиста.
    /// 
    /// Если нет такого трека, ничего не происходит.
    pub fn remove_track(&mut self,index:usize){
        if index<self.playlist.len(){
            // Получение номера итератора и удаление его из плейлиста
            let iter_index=self.playlist.remove(index);

            // Удаление каналов распределения
            self.channels.remove(index);

            // Добавление свободного номера в очередь
            self.free_slots.push(iter_index);

            // Номер трека в хранилище и индекс номера итератора
            let (track_owner,iter_index)=self.track_owners[iter_index];

            // Ссылка на список номеров итераторов
            let iter_indices=&mut self.iter_indices.as_mut()[track_owner];

            // Удаление номера итератора
            iter_indices.remove(iter_index);

            if iter_index<iter_indices.len(){
                // Уменьшение индексов номеров итераторов из списка,
                // находящихся выше
                for &i in &iter_indices[iter_index..]{
                    self.track_owners[i].1-=1;
                }
            }
        }
    }
}

/// Установка параметров.
impl ChannelSystem{
    /// Устанавливает новую частоту системы.
    /// Обновляет все треки.
    pub fn set_system_sample_rate(&mut self,sample_rate:u32){
        self.sample_rate=sample_rate;

        for &track in &self.playlist{
            self.play_buffer[track].set_system_sample_rate(sample_rate)
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
    /// 
    /// Если нет такого трека, ничего не происходит.
    pub fn set_track_volume(&mut self,index:usize,volume:f32){
        if let Some(&track)=self.playlist.get(index){
            self.play_buffer[track].set_volume(volume)
        }
    }
}

/// Итерации.
impl ChannelSystem{
    /// Возвращает фрейм каналов.
    /// Распределяет все треки по каналам.
    pub fn next_frame(&mut self)->&mut Vec<f32>{
        // Отчистка фрейма каналов
        for channel in &mut self.channel_frame{
            *channel=0f32;
        }

        // Перебор треков
        let mut c=0usize;
        'tracks:while c<self.playlist.len(){
            let track=&mut self.play_buffer[self.playlist[c]];

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