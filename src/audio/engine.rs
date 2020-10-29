use crate::support::{
    SyncRawPtr,
    SyncRawMutPtr
};

use super::{
    AudioCommandResult,
    AudioSystemCommand,
    AudioSettings,
    track::*,
    sample::SampleTransform,
    ChannelSystem,
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
    mut format:Format,
    audio_output_channels:u16,
    receiver:Receiver<AudioSystemCommand>,
    mut volume:f32,
)->!{
    // Локальный массив одноканальных треков
    let mut o_tracks=Vec::<MonoTrack>::with_capacity(8);

    let mut tracks=SyncRawMutPtr::new(&mut o_tracks);

    let mut channel_system=ChannelSystem::new(format.sample_rate.0,8,format.channels as usize);

    event_loop.clone().run(move|stream,result|{
        // Обработчик команд
        match receiver.try_recv(){
            Ok(command)=>match command{
                AudioSystemCommand::AddMono(track)=>{
                    let tracks=tracks.as_mut();
                    if tracks.len()<tracks.capacity(){
                        tracks.push(track);
                    }
                }
                // Добавление трека для проигрывания
                AudioSystemCommand::PlayMonoOnChannels(index,channels,repeats)=>{
                    let track=&tracks.as_ref()[index];

                    channel_system.add_track(track,channels,repeats);
                }
                // Добавление множества треков для проигрывания
                AudioSystemCommand::PlayMonosOnChannels(sets)=>{
                    for (index,channels,repeats) in sets{
                        let track=&tracks.as_ref()[index];

                        channel_system.add_track(track,channels,repeats);
                    }
                }

                AudioSystemCommand::Stop=>{
                    //track.stop()
                }

                AudioSystemCommand::SetVolume(v)=>{
                    volume=v;
                }

                AudioSystemCommand::Close=>{
                    panic!("Closing CatEngine's audio thread")
                },
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
                        format.channels,
                        volume,
                        buffer
                    ),

                    StreamData::Output{buffer:UnknownTypeOutputBuffer::U16(mut buffer)}
                    =>output(
                        &mut channel_system,
                        format.channels,
                        volume,
                        buffer
                    ),

                    StreamData::Output{buffer:UnknownTypeOutputBuffer::F32(mut buffer)}
                    =>output(
                        &mut channel_system,
                        format.channels,
                        volume,
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

                        format=new_device.default_output_format().expect("No available device");

                        format.channels=audio_output_channels;

                        // Установка новой частоты дискретизации
                        channel_system.set_sample_rate(format.sample_rate.0);

                        let new_stream=event_loop.build_output_stream(&new_device,&format).expect("Build a new stream");

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

fn output<S:SampleTransform>(
    channel_system:&mut ChannelSystem,
    channels:u16,
    volume:f32,
    mut buffer:OutputBuffer<S>
){
    let mut c=0usize;

    let mut frame=channel_system.next_frame();

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