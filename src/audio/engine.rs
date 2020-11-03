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
    // Локальный массив одноканальных треков
    let mut tracks_container=Vec::<MonoTrack>::with_capacity(settings.track_array_capacity);

    let mut channel_system=ChannelSystem::new(
        settings.format.sample_rate.0,
        settings.track_playlist,
        settings.format.channels as usize
    );

    event_loop.clone().run(move|stream,result|{
        // Обработчик команд
        match receiver.try_recv(){
            Ok(command)=>match command{
                // Добавление трека в массив
                AudioSystemCommand::AddMono(track)=>{
                    // Здесь проверка не нужна, так как уже есть внешняя,
                    // но эта может потом пригодится
                    //if tracks_container.len()<tracks_container.capacity(){
                        tracks_container.push(track);
                    //}
                }

                // Добавление трека в плейлист
                AudioSystemCommand::PlayMonoOnChannels(TrackSet{index,channels,repeats,volume})=>{
                    // Здесь проверка не нужна, так как уже есть внешняя,
                    // но она в любом случае выполняется при обращении через индекс
                    if let Some(track)=tracks_container.get(index){
                        // Здесь проверка не нужна, так как уже есть внешняя -
                        // переполнения не должно быть
                        channel_system.add_track(track,channels,repeats,volume);
                    }
                }

                // Добавление множества треков в плейлист
                AudioSystemCommand::PlayMonosOnChannels(sets)=>{
                    for TrackSet{index,channels,repeats,volume} in sets{
                        if let Some(track)=tracks_container.get(index){
                            // Здесь проверка не нужна, так как уже есть внешняя -
                            // переполнения не должно быть
                            channel_system.add_track(track,channels,repeats,volume);
                        }
                    }
                }

                // Очищает весь плейлист
                AudioSystemCommand::ClearPlaylist=>
                    channel_system.clear_playlist(),

                AudioSystemCommand::SetMonoVolume(index,volume)=>
                    channel_system.set_track_volume(index,volume),

                AudioSystemCommand::SetGeneralVolume(v)=>
                    settings.general_volume=v,

                AudioSystemCommand::Close=>
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
            c=0;
        }

        if !frame.is_empty(){
            let sample=frame[c];
            *b=SampleTransform::from(sample,volume)
        }

        c+=1;
    }
}