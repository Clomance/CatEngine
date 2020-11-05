use crate::support::SyncRawMutPtr;

use super::{
    AudioCommandResult,
    AudioSystemCommand,
    track::*,
    sample::SampleTransform,
    ChannelSystem,
    AudioSystemSettings,
};

use cpal::{
    Host,
    HostId,
    HostUnavailable,
    Device,
    DevicesError,
    Devices,
    OutputDevices,
    traits::{
        HostTrait,
        DeviceTrait,
        EventLoopTrait
    },
    UnknownTypeOutputBuffer,
    StreamData,
    StreamId,
    EventLoop,
    Sample,
    Format,
    StreamError,
    OutputBuffer,
};

use std::{
    io,
    vec::IntoIter,
    iter::Cycle,
    path::Path,
    thread::{Builder,JoinHandle},
    sync::{
        Arc,
        Mutex,
        LockResult,
        mpsc::{
            Sender,
            Receiver,
            channel,
            TryRecvError
        },
    },
};

/// Создание и запуск потока обработки.
pub (crate) fn event_loop_handler(//<D:Fn(&Host)->Device+Send+Sync+'static>(
    host:Arc<Host>,
    //choose_device:D,
    main_stream:Arc<Mutex<Option<StreamId>>>,
    event_loop:Arc<EventLoop>,
    receiver:Receiver<AudioSystemCommand>,
    mut settings:AudioSystemSettings,
)->!{
    // Локальное хранилище одноканальных треков, не должно превышать заданного размера
    // При изменении размера придётся переопределить все треки в плейлисте,
    // так как они прямо ссылаются на ячейки в хранилище
    let mut track_container=Vec::<MonoTrack>::with_capacity(settings.track_array_capacity);

    // Матрица распределения треков по буферу
    // Номера итераторов
    let mut iter_indices=Vec::<Vec<usize>>::with_capacity(settings.track_array_capacity);

    // Свободные ячейки для треков
    let mut free_container_slots=Vec::<usize>::with_capacity(settings.track_array_capacity);

    // Ссылки на треки в хранилище (список загруженных треков)
    let mut track_list=Vec::<usize>::with_capacity(settings.track_array_capacity);

    // Подготовка
    for c in 0..settings.track_array_capacity{
        track_container.push(MonoTrack{data:Vec::new(),sample_rate:0u32});
        iter_indices.push(Vec::with_capacity(1));
        free_container_slots.push(c);
    }

    // Ссылки нужны, чтобы передать их потоку,
    // в ином случае данные могут копироваться или передвигаться
    // (в случае с `iter_indices` точно, так как она используется ещё и в `channel_system`)
    // из-за того, что функция `run` (находится ниже) забирает все права на переменные
    let mut track_container_ref=SyncRawMutPtr::new(&mut track_container);
    let mut iter_indices_ref=SyncRawMutPtr::new(&mut iter_indices);
    let mut free_container_slots_ref=SyncRawMutPtr::new(&mut free_container_slots);
    let mut track_list_ref=SyncRawMutPtr::new(&mut track_list);

    // Создание системы распределения треков и каналов
    let mut channel_system=ChannelSystem::new(
        iter_indices_ref.clone(),
        settings.format.sample_rate.0,
        settings.track_playlist,
        settings.format.channels as usize
    );

    event_loop.clone().run(move|stream,result|{
        // Обработчик команд
        match receiver.try_recv(){
            Ok(command)=>match command{
                // Добавление трека в список и хранилище
                // Если хранилище переполнено, то ничего не происходит
                AudioSystemCommand::AddMono(track)=>
                    // Здесь проверка не нужна, так как уже есть внешняя,
                    // но без неё никак
                    // Получение свободной номера ячейки
                    if let Some(index)=free_container_slots_ref.as_mut().pop(){
                        // Добавление номера в список треков
                        track_list_ref.as_mut().push(index);
                        // Установка трека
                        track_container_ref.as_mut()[index]=track;
                        // Очистка списка итераторов
                        iter_indices_ref.as_mut()[index].clear();
                    }

                // Удаление трека из списка и хранилища
                // Если такого трека нет, то ничего не происходит
                AudioSystemCommand::RemoveMono(index)=>
                    // Здесь проверка не нужна, так как уже есть внешняя,
                    // но пусть пока будет
                    if index<track_list_ref.as_ref().len(){
                        // Получение номера трека в буфере
                        let track_index=track_list_ref.as_mut().remove(index);
 
                        // Добавление номера ячейки в очередь
                        free_container_slots_ref.as_mut().push(track_index);

                        // Остановка итераторов (установка флагов PlayType::None)
                        let iters=&iter_indices_ref.as_mut()[track_index];
                        for &i in iters{
                            channel_system.stop_iter(i);
                        }
                    }

                // Добавление треков в список и хранилище
                // Если хранилище переполнено, то ничего не происходит
                AudioSystemCommand::AddMonos(tracks)=>
                    for track in tracks{
                        // Здесь проверка не нужна, так как уже есть внешняя,
                        // но без неё никак
                        // Получение свободной номера ячейки
                        if let Some(index)=free_container_slots_ref.as_mut().pop(){
                            // Добавление номера в список треков
                            track_list_ref.as_mut().push(index);
                            // Установка трека
                            track_container_ref.as_mut()[index]=track;
                            // Очистка списка итераторов
                            iter_indices_ref.as_mut()[index].clear();
                        }
                    }

                // Удаление треков из списка и хранилища
                // Если таких треков нет, то ничего не происходит
                AudioSystemCommand::RemoveMonos(indices)=>
                    for index in indices.into_iter().rev(){
                        // Здесь проверка не нужна, так как уже есть внешняя,
                        // но пусть пока будет
                        if index<track_list_ref.as_ref().len(){
                            // Получение номера трека в буфере
                            let track_index=track_list_ref.as_mut().remove(index);

                            // Добавление номера ячейки в очередь
                            free_container_slots_ref.as_mut().push(track_index);

                            // Остановка итераторов (установка флагов PlayType::None)
                            let iters=&iter_indices_ref.as_mut()[track_index];
                            for &i in iters{
                                channel_system.stop_iter(i);
                            }
                        }
                    }

                // Очищает список треков
                AudioSystemCommand::ClearTrackArray=>{
                    track_list_ref.as_mut().clear();
                    channel_system.clear_playlist()
                }

                // Добавление трека в плейлист
                // Если плейлист переполнен, то ничего не происходит
                AudioSystemCommand::PlayMonoOnChannels(TrackSet{index,channels,repeats,volume})=>
                    // Получение номера трека в хранилище
                    if let Some(&track_index)=track_list_ref.as_ref().get(index){
                        let track=track_container_ref.as_ref().get(track_index).unwrap();
                        // Здесь проверка не нужна, так как уже есть внутреняя -
                        // переполнения плейлиста не должно быть
                        channel_system.add_track(track_index,track,channels,repeats,volume);
                    }

                // Удаление трека из плейлиста
                // Если нет такого трека, ничего не происходит
                AudioSystemCommand::RemoveMonoFromPlaylist(index)=>
                    // Проверка не нужна, так как она проводится внутри
                    channel_system.remove_track(index),

                // Добавление множества треков в плейлист
                // Если плейлист переполнен, то ничего не происходит
                AudioSystemCommand::PlayMonosOnChannels(sets)=>
                    for TrackSet{index,channels,repeats,volume} in sets{
                        if let Some(&track_index)=track_list_ref.as_ref().get(index){
                            let track=track_container_ref.as_ref().get(track_index).unwrap();
                            // Здесь проверка не нужна, так как уже есть внешняя -
                            // переполнения не должно быть
                            channel_system.add_track(track_index,track,channels,repeats,volume);
                        }
                    }

                // Удаление треков из плейлиста
                // Если нет таких треков, ничего не происходит
                AudioSystemCommand::RemoveMonosFromPlaylist(indices)=>
                    for index in indices.into_iter().rev(){
                        // Проверка не нужна, так как она проводится внутри
                        channel_system.remove_track(index)
                    }

                // Ставит трек из плейлиста проигрываться
                // Если уже проигрывается или нет такого трека,
                // ничего не происходит
                AudioSystemCommand::PlayMono(index)=>
                    channel_system.play(index),

                // Ставит трек из плейлиста на паузу
                // Если уже проигрывается или нет такого трека,
                // ничего не происходит
                AudioSystemCommand::PauseMono(index)=>
                    channel_system.pause(index),

                // Ставит треки из плейлиста проигрываться
                // Если уже проигрывается или нет таких треков,
                // ничего не происходит
                AudioSystemCommand::PlayMonos(indices)=>
                    for index in indices{
                        channel_system.play(index)
                    }

                // Ставит треки из плейлиста на паузу
                // Если уже на паузе или нет таких треков,
                // ничего не происходит
                AudioSystemCommand::PauseMonos(indices)=>
                    for index in indices{
                        channel_system.pause(index)
                    }

                // Очищает весь плейлист и
                // очищает списки итераторов (`iter_indices`)
                AudioSystemCommand::ClearPlaylist=>
                    channel_system.clear_playlist(),

                // Устанавливает громкость трека
                // Если нет такого трека, ничего не происходит
                AudioSystemCommand::SetMonoVolume(index,volume)=>
                    channel_system.set_track_volume(index,volume),

                AudioSystemCommand::SetMonosVolume(indices,volume)=>{
                    for index in indices{
                        channel_system.set_track_volume(index,volume)
                    }
                }

                // Устанавливает общую громкость
                AudioSystemCommand::SetGeneralVolume(v)=>
                    settings.general_volume=v,

                // Закрывает поток
                AudioSystemCommand::Close=> // Поток умер :)
                    panic!("Closing CatEngine's audio thread"),

                _=>{}
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
                    StreamData::Output{buffer:UnknownTypeOutputBuffer::I16(mut buffer)}
                    =>output(
                        &mut channel_system,
                        settings.format.channels,
                        settings.general_volume,
                        buffer
                    ),

                    StreamData::Output{buffer:UnknownTypeOutputBuffer::U16(mut buffer)}
                    =>output(
                        &mut channel_system,
                        settings.format.channels,
                        settings.general_volume,
                        buffer
                    ),

                    StreamData::Output{buffer:UnknownTypeOutputBuffer::F32(mut buffer)}
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
                        let new_device=host.default_output_device().expect("No available device");

                        settings.format=new_device.default_output_format().expect("No available device");

                        // Установка новой частоты дискретизации
                        channel_system.set_system_sample_rate(settings.format.sample_rate.0);

                        channel_system.set_system_channels(settings.format.channels);

                        let new_stream=event_loop.build_output_stream(&new_device,&settings.format).expect("Build a new stream");

                        *main_stream.lock().unwrap()=Some(new_stream.clone());

                        event_loop.play_stream(new_stream.clone()).unwrap();
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