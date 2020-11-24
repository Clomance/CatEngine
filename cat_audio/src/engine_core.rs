use crate::support::SyncRawMutPtr;

use super::{
    AudioEngineCommand,
    track::*,
    sample::SampleTransform,
    ChannelSystem,
    AudioSystemSettings,
};

use cpal::{
    Host,
    traits::{
        HostTrait,
        DeviceTrait,
        EventLoopTrait
    },
    UnknownTypeOutputBuffer,
    StreamData,
    StreamId,
    EventLoop,
    StreamError,
    OutputBuffer,
};

use std::sync::{
    Arc,
    Mutex,
    mpsc::Receiver,
};

// Хранилище (хранилище треков, `track_storage`) - массив треков,
// на которые ссылкаются итераторы в буфере.

// Буфер (буфер итераторов, `play_buffer`) - массив итераторов,
// на которые индексно ссылаются плейлист (`playlist`)
// и матрица распределения итераторов (`iter_indices`).

/// Создание и запуск потока обработки.
pub (crate) fn event_loop_handler(
    host:Arc<Host>,
    playing_flag:Arc<Mutex<bool>>,
    mut settings:AudioSystemSettings,
    main_stream:Arc<Mutex<Option<StreamId>>>,
    event_loop:Arc<EventLoop>,
    receiver:Receiver<AudioEngineCommand>,
)->!{
    // Локальное хранилище одноканальных треков,
    // не должно превышать заданного размера
    // При изменении размера (скорее всего с перемещением)
    // придётся переопределить все треки в буфере,
    // так как они прямо ссылаются на ячейки в хранилище
    let mut track_storage=Vec::<MonoTrack>::with_capacity(settings.track_storage_capacity);

    // Матрица распределения треков по буферу
    // Номера итераторов
    let mut iter_indices=Vec::<Vec<usize>>::with_capacity(settings.track_storage_capacity);

    // Подготовка
    for _ in 0..settings.track_storage_capacity{
        track_storage.push(MonoTrack{data:Vec::new(),sample_rate:0u32});
        iter_indices.push(Vec::with_capacity(1));
    }

    // Ссылки нужны, чтобы передать их потоку,
    // в ином случае данные могут копироваться или передвигаться
    // (в случае с `iter_indices` точно, так как она используется ещё и в `channel_system`)
    // из-за того, что функция `run` (находится ниже) забирает все права на переменные
    let mut track_storage_ref=SyncRawMutPtr::new(&mut track_storage);
    let mut iter_indices_ref=SyncRawMutPtr::new(&mut iter_indices);

    // Создание системы распределения треков и каналов
    let mut channel_system=ChannelSystem::new(
        iter_indices_ref.clone(),
        settings.format.sample_rate.0,
        settings.track_playlist,
        settings.format.channels as usize
    );

    event_loop.clone().run(move|_,result|{
        // Обработчик команд
        match receiver.try_recv(){
            Ok(command)=>match command{
            // ХРАНИЛИЩЕ \\
                // Добавление трека в ячейку хранилища
                // Если нет такой ячейки, то ничего не происходит
                AudioEngineCommand::AddMono(track,index)=>
                    if let Some(slot)=track_storage_ref.as_mut().get_mut(index){
                        // Установка трека
                        *slot=track;
                        // Очистка списка итераторов
                        iter_indices_ref.as_mut()[index].clear();
                    }

                // Добавление треков в ячейки хранилища
                // Если нет таких ячеек, то ничего не происходит
                AudioEngineCommand::AddMonos(tracks)=>
                    for (track,index) in tracks{
                        if let Some(slot)=track_storage_ref.as_mut().get_mut(index){
                            // Установка трека
                            *slot=track;
                            // Очистка списка итераторов
                            iter_indices_ref.as_mut()[index].clear();
                        }
                    }

                // Удаление трека из хранилища
                // Если такого трека нет, то ничего не происходит
                AudioEngineCommand::RemoveMono(index)=>
                    if let Some(iters)=iter_indices_ref.as_mut().get_mut(index){
                        // Остановка итераторов (установка флагов PlayType::None)
                        for &mut i in iters{
                            channel_system.stop_iter(i);
                        }
                    }


                // Удаление треков из хранилища
                // Если таких треков нет, то ничего не происходит
                AudioEngineCommand::RemoveMonos(indices)=>
                    for index in indices.into_iter(){
                        if let Some(iters)=iter_indices_ref.as_mut().get_mut(index){
                            // Остановка итераторов (установка флагов PlayType::None)
                            for &mut i in iters{
                                channel_system.stop_iter(i);
                            }
                        }
                    }

                // Снимает паузу с треков из плейлиста,
                // привязанных к треку из хранилища
                AudioEngineCommand::UnpauseMonoFromStorage(index)=>
                    if let Some(iters)=iter_indices_ref.as_ref().get(index){
                        for &iter in iters{
                            channel_system.unpause_buffer_iter(iter)
                        }
                    }

                // Снимает паузу с треков из плейлиста,
                // привязанных к трекам из хранилища
                AudioEngineCommand::UnpauseMonosFromStorage(indices)=>
                    for index in indices{
                        if let Some(iters)=iter_indices_ref.as_ref().get(index){
                            for &iter in iters{
                                channel_system.unpause_buffer_iter(iter)
                            }
                        }
                    }

                // Ставит на паузу треки из плейлиста,
                // привязанные к треку из хранилища
                AudioEngineCommand::PauseMonoFromStorage(index)=>
                    if let Some(iters)=iter_indices_ref.as_ref().get(index){
                        for &iter in iters{
                            channel_system.pause_buffer_iter(iter)
                        }
                    }

                // Ставит на паузу треки из плейлиста,
                // привязанные к трекам из хранилища
                AudioEngineCommand::PauseMonosFromStorage(indices)=>
                    for index in indices{
                        if let Some(iters)=iter_indices_ref.as_ref().get(index){
                            for &iter in iters{
                                channel_system.pause_buffer_iter(iter)
                            }
                        }
                    }

                // Останавливает треки из плейлиста,
                // привязанные к треку из хранилища
                AudioEngineCommand::StopMonoFromStorage(index)=>
                    if let Some(iters)=iter_indices_ref.as_ref().get(index){
                        for &iter in iters{
                            channel_system.stop_buffer_iter(iter)
                        }
                    }

                // Останавливает треки из плейлиста,
                // привязанные к трекам из хранилища
                AudioEngineCommand::StopMonosFromStorage(indices)=>
                    for index in indices{
                        if let Some(iters)=iter_indices_ref.as_ref().get(index){
                            for &iter in iters{
                                channel_system.stop_buffer_iter(iter)
                            }
                        }
                    }

                // Устанавливает громкость треков из плейлиста,
                // привязанных к треку из хранилища
                AudioEngineCommand::SetMonoVolumeFromStorage(index,volume)=>
                    if let Some(iters)=iter_indices_ref.as_ref().get(index){
                        for &iter in iters{
                            channel_system.set_volume_buffer_iter(iter,volume)
                        }
                    }

                // Устанавливает громкость треков из плейлиста,
                // привязанных к трекам из хранилища
                AudioEngineCommand::SetMonosVolumeFromStorage(indices,volume)=>
                    for index in indices{
                        if let Some(iters)=iter_indices_ref.as_ref().get(index){
                            for &iter in iters{
                                channel_system.set_volume_buffer_iter(iter,volume)
                            }
                        }
                    }

                // Устанавливает громкости треков из плейлиста,
                // привязанных к трекам из хранилища
                AudioEngineCommand::SetMonosVolumesFromStorage(sets)=>
                    for (index,volume) in sets{
                        if let Some(iters)=iter_indices_ref.as_ref().get(index){
                            for &iter in iters{
                                channel_system.set_volume_buffer_iter(iter,volume)
                            }
                        }
                    }

            // ПЛЕЙЛИСТ \\
                // Добавление трека в плейлист
                // Если плейлист переполнен, то ничего не происходит
                AudioEngineCommand::PlayMonoOnChannels(TrackSet{index,channels,repeats,volume})=>
                    // Получение номера трека в хранилище
                    if let Some(track)=track_storage_ref.as_ref().get(index){
                        // Здесь проверка не нужна, так как уже есть внутреняя -
                        // переполнения плейлиста не должно быть
                        channel_system.add_track(index,track,channels,repeats,volume);
                    }

                // Удаление трека из плейлиста
                // Если нет такого трека, ничего не происходит
                AudioEngineCommand::RemoveMonoFromPlaylist(index)=>
                    // Проверка не нужна, так как она проводится внутри
                    channel_system.remove_track(index),

                // Добавление множества треков в плейлист
                // Если плейлист переполнен, то ничего не происходит
                AudioEngineCommand::PlayMonosOnChannels(sets)=>
                    for TrackSet{index,channels,repeats,volume} in sets{
                        if let Some(track)=track_storage_ref.as_ref().get(index){
                            // Здесь проверка не нужна, так как уже есть внутреняя -
                            // переполнения плейлиста не должно быть
                            channel_system.add_track(index,track,channels,repeats,volume);
                        }
                    }

                // Удаление треков из плейлиста
                // Если нет таких треков, ничего не происходит
                AudioEngineCommand::RemoveMonosFromPlaylist(indices)=>
                    for index in indices.into_iter().rev(){
                        // Проверка не нужна, так как она проводится внутри
                        channel_system.remove_track(index)
                    }

                // Ставит трек из плейлиста проигрываться
                // Если уже проигрывается или нет такого трека,
                // ничего не происходит
                AudioEngineCommand::UnpauseMonoFromPlaylist(index)=>
                    channel_system.unpause(index),

                // Ставит треки из плейлиста проигрываться
                // Если уже проигрывается или нет таких треков,
                // ничего не происходит
                AudioEngineCommand::UnpauseMonosFromPlaylist(indices)=>
                for index in indices{
                    channel_system.unpause(index)
                }

                // Ставит трек из плейлиста на паузу
                // Если уже проигрывается или нет такого трека,
                // ничего не происходит
                AudioEngineCommand::PauseMonoFromPlaylist(index)=>
                    channel_system.pause(index),

                // Ставит треки из плейлиста на паузу
                // Если уже на паузе или нет таких треков,
                // ничего не происходит
                AudioEngineCommand::PauseMonosFromPlaylist(indices)=>
                    for index in indices{
                        channel_system.pause(index)
                    }

                // Очищает весь плейлист и
                // очищает списки итераторов (`iter_indices`)
                AudioEngineCommand::ClearPlaylist=>
                    channel_system.clear_playlist(),

                // Устанавливает громкость трека
                // Если нет такого трека, ничего не происходит
                AudioEngineCommand::SetMonoVolume(index,volume)=>
                    channel_system.set_track_volume(index,volume),

                AudioEngineCommand::SetMonosVolume(indices,volume)=>{
                    for index in indices{
                        channel_system.set_track_volume(index,volume)
                    }
                }

                AudioEngineCommand::SetMonosVolumes(sets)=>{
                    for set in sets{
                        channel_system.set_track_volume(set.0,set.1)
                    }
                }

                // Устанавливает общую громкость
                AudioEngineCommand::SetGeneralVolume(v)=>
                    settings.general_volume=v,
            // ОСТАЛЬНОЕ \\
                // Закрывает поток
                AudioEngineCommand::Close=> // Поток умер :)
                    panic!("Closing CatEngine's audio thread"),
            }
            Err(_)=>{
                // Ошибки игнорируются,
                // так как `TryRecvError::Empty` означает, что команд нет,
                // а `TryRecvError::Disconnected` во всех случаях перехватывается деконструктором,
                // и вызывается паника
            }
        }

        // Вывод звука
        match result{
            Ok(data)=>{
                // let stream_lock=main_stream.lock().unwrap();
                match data{
                    StreamData::Output{buffer:UnknownTypeOutputBuffer::I16(buffer)}
                    =>output(
                        &mut channel_system,
                        settings.format.channels,
                        settings.general_volume,
                        buffer
                    ),

                    StreamData::Output{buffer:UnknownTypeOutputBuffer::U16(buffer)}
                    =>output(
                        &mut channel_system,
                        settings.format.channels,
                        settings.general_volume,
                        buffer
                    ),

                    StreamData::Output{buffer:UnknownTypeOutputBuffer::F32(buffer)}
                    =>output(
                        &mut channel_system,
                        settings.format.channels,
                        settings.general_volume,
                        buffer
                    ),

                    _=>{}
                }
            }

            Err(error)=>{
                match error{
                    // Выбор нового устройства, если прежнее не доступно
                    StreamError::DeviceNotAvailable=>{
                        let mut stream_lock=main_stream.lock().unwrap();

                        let new_device=host.default_output_device().expect("No available device");

                        settings.format=new_device.default_output_format().expect("No available device");

                        // Установка новой частоты дискретизации
                        channel_system.set_system_sample_rate(settings.format.sample_rate.0);

                        channel_system.set_system_channels(settings.format.channels);

                        let new_stream=event_loop.build_output_stream(&new_device,&settings.format).expect("Build a new stream");

                        if playing_flag.lock().unwrap().clone(){
                            event_loop.play_stream(new_stream.clone()).unwrap();
                        }
                        else{
                            event_loop.pause_stream(new_stream.clone()).unwrap();
                        }

                        *stream_lock=Some(new_stream.clone());
                    }
                    // Паникует, если какая-то другая ошибка
                    // (пока не знаю, как нормально обработать)
                    StreamError::BackendSpecific{err}=>{
                        panic!("{}",err)
                    }
                }
            }
        }
    })
}

/// Вывод звука и постобработка (`feature="audio_post_processing"`)
fn output<
    S:SampleTransform,
    #[cfg(feature="audio_post_processing")]P:FnMut(&mut Vec<f32>)
>(
    channel_system:&mut ChannelSystem,
    channels:u16,
    volume:f32,
    mut buffer:OutputBuffer<S>,
    #[cfg(feature="audio_post_processing")]post_processing:P,
){
    let mut c=0usize;

    let mut frame=channel_system.next_frame();

    #[cfg(feature="audio_post_processing")]
    post_processing(frame);

    for b in buffer.iter_mut(){
        if c==channels as usize{
            frame=channel_system.next_frame();
            #[cfg(feature="audio_post_processing")]
            post_processing(frame);
            c=0;
        }

        let sample=frame[c];
        *b=SampleTransform::from(sample,volume);

        c+=1;
    }
}