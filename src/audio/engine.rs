use super::{
    AudioOutputFormat,
    AudioCommandResult,
    AudioSystemCommand,
    RateConverter,
    AudioSettings,
    track::*,
    sample::SampleTransform,
    play_buffer::*,
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

const audio_thread_stack_size:usize=1024;

//     /\__/\
//    /`    '\
//   |  0  0  |
//  ===  --  ===
//   /        \
//  /          \
// |            |
//  \  ||  ||  /
//   \_oo__oo_/#######o
// I am watching you, Mister Programmer.

/// Простой аудио движок. A simple audio engine.
/// 
/// Пока только вывод доступен.
/// 
/// Only output is available now.
pub struct Audio{
    host:Arc<Host>,
    //device:Arc<Mutex<Device>>,
    stream:Arc<Mutex<Option<StreamId>>>,

    tracks:Arc<Mutex<Vec<Track<i16>>>>,
    event_loop:Arc<EventLoop>,

    command:Sender<AudioSystemCommand>,
    thread:Option<JoinHandle<()>>,
}

impl Audio{
    /// Строит аудио движок.
    /// 
    /// Возвращает результат создания аудио потока.
    /// 
    /// Creates an audio engine.
    /// 
    /// Returns the result of starting an audio thread.
    pub fn new(//<D:Fn(&Host)->Device+Send+Sync+'static>(
        host:Host,
        //choose_device:D,
        settings:AudioSettings
    )->io::Result<Audio>{
        // Массив треков
        let tracks=Arc::new(Mutex::new(Vec::with_capacity(settings.track_array_capacity)));
        //
        let stream=Arc::new(Mutex::new(None));

        let t=tracks.clone();
        let s=stream.clone();

        let event_loop=Arc::new(host.event_loop());
        let el=event_loop.clone();
        // Передача команд от управляющего потока выполняющему
        let (sender,receiver)=channel::<AudioSystemCommand>();

        let owner_host=Arc::new(host);
        let host=owner_host.clone();

        let thread_result=Builder::new()
                .name("CatEngine's audio thread".to_string())
                .stack_size(audio_thread_stack_size)
                .spawn(move||{

            let mut device=host.default_output_device().expect("No available device");
            //let device=choose_device(&host);
            let mut format=device.default_output_format().expect("No available device");

            format.channels=settings.output_format.into_channels();

            let main_stream=event_loop.build_output_stream(&device,&format).expect("stream");

            *stream.lock().unwrap()=Some(main_stream.clone());

            event_loop.play_stream(main_stream.clone()).unwrap();

            // Забирает контроль над потоком и начинает обработку аудио потоков
            // Takes control of the current thread and begins the stream processing
            event_loop_handler(
                host,
                //choose_device,
                stream,
                event_loop,
                format,
                settings.output_format,
                receiver,
                tracks,
                settings.volume,
            )
        });

        let thread=match thread_result{
            Ok(thread)=>thread,
            Err(e)=>return Err(e),
        };

        Ok(Self{
            host:owner_host,
            stream:s,

            tracks:t,
            event_loop:el,
            command:sender,
            thread:Some(thread),
        })
    }

    /// Добавляет трек в массив треков.
    /// 
    /// Adds the track to the track array.
    pub fn add_track<P:AsRef<Path>>(&self,path:P)->AudioCommandResult{
        let track=match Track::new(path){
            TrackResult::Ok(track)=>track,
            _=>return AudioCommandResult::TrackError
        };

        match self.tracks.lock(){
            Ok(mut lock)=>{
                lock.push(track);
                AudioCommandResult::Ok
            },
            Err(_)=>AudioCommandResult::ThreadClosed,
        }
    }

    /// Удаляет трек из массива треков.
    /// 
    /// Removes the track from the track array.
    pub fn remove_track(&self,index:usize)->AudioCommandResult{
        let mut lock=match self.tracks.lock(){
            Ok(lock)=>lock,
            Err(_)=>return AudioCommandResult::ThreadClosed,
        };

        if index<lock.len(){
            lock.remove(index);
            AudioCommandResult::Ok
        }
        else{
            AudioCommandResult::NoSuchTrack
        }
    }

    /// Удаляет все треки из массива треков.
    /// 
    /// Removes all tracks from the track array.
    pub fn remove_all_tracks(&self)->AudioCommandResult{
        self.tracks.lock().unwrap().clear();
        AudioCommandResult::Ok
    }

    /// Запускает трек.
    /// 
    /// 0 - постоянно, 1 - один раз, 2.. - повторить дважды и так далее
    /// 
    /// Plays a track.
    /// 
    /// 0 - forever, 1 - once, 2.. - repeat twice and so on
    pub fn play_track(&self,index:usize,repeats:u32)->AudioCommandResult{
        let stream_lock=match self.stream.lock(){
            LockResult::Ok(lock)=>lock,
            LockResult::Err(_)=>return AudioCommandResult::ThreadClosed
        };

        let tracks_lock=match self.tracks.lock(){
            Ok(lock)=>lock,
            Err(_)=>return AudioCommandResult::ThreadClosed,
        };

        if index>=tracks_lock.len(){
            return AudioCommandResult::NoSuchTrack
        }

        let result=match self.command.send(AudioSystemCommand::Play((index,repeats))){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>return AudioCommandResult::ThreadClosed
        };

        if let Some(stream)=stream_lock.as_ref(){
            self.event_loop.play_stream(stream.clone());
        }

        result
    }

    /// Запускает проигрывание канала.
    /// 
    /// Starts playing the stream.
    pub fn play(&self)->AudioCommandResult{
        let stream_lock=match self.stream.lock(){
            LockResult::Ok(stream)=>stream,
            LockResult::Err(_)=>return AudioCommandResult::ThreadClosed
        };

        if let Some(stream)=stream_lock.as_ref(){
            self.event_loop.play_stream(stream.clone());
        }

        AudioCommandResult::Ok
    }

    /// Ставит на паузу проигрывание канала.
    /// 
    /// Pauses the stream.
    pub fn pause(&self)->AudioCommandResult{
        let stream_lock=match self.stream.lock(){
            LockResult::Ok(stream)=>stream,
            LockResult::Err(_)=>return AudioCommandResult::ThreadClosed
        };

        if let Some(stream)=stream_lock.as_ref(){
            self.event_loop.pause_stream(stream.clone());
        }

        AudioCommandResult::Ok
    }

    /// Останавливает проигрывание путём удаления трека из буфера для вывода.
    /// 
    /// Stops playing by removing current track from the playing buffer.
    pub fn stop(&self)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::Stop){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>AudioCommandResult::ThreadClosed
        }
    }

    /// Устанавливает громкость.
    /// 
    /// Sets the volume.
    pub fn set_volume(&self,volume:f32)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::SetVolume(volume)){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>AudioCommandResult::ThreadClosed
        }
    }
}

fn event_loop_handler(//<D:Fn(&Host)->Device+Send+Sync+'static>(
    host:Arc<Host>,
    //choose_device:D,
    main_stream:Arc<Mutex<Option<StreamId>>>,
    event_loop:Arc<EventLoop>,
    mut format:Format,
    mut output_format:AudioOutputFormat,
    receiver:Receiver<AudioSystemCommand>,
    tracks:Arc<Mutex<Vec<Track<i16>>>>,
    mut volume:f32,
)->!{
    let mut track=PlayingTrack::new();

    let mut rate_converter=RateConverter::new(
        track.sample_rate,
        format.sample_rate.0,
        &mut track,
    );

    event_loop.clone().run(move|stream,result|{
        // Обработчик команд
        match receiver.try_recv(){
            Ok(command)=>match command{
                AudioSystemCommand::Play((i,r))=>{
                    let lock=tracks.lock().unwrap();
                    let t:&Track<i16>=lock.get(i).unwrap();

                    track.set_track_i16(t,r);
                    rate_converter=RateConverter::new(
                        track.sample_rate,
                        format.sample_rate.0,
                        &mut track,
                    );
                }

                AudioSystemCommand::Stop=>{
                    track.stop()
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
                        &mut track,
                        &mut rate_converter,
                        format.channels,
                        volume,buffer
                    ),

                    StreamData::Output{buffer:UnknownTypeOutputBuffer::U16(mut buffer)}
                    =>output(
                        &mut track,
                        &mut rate_converter,
                        format.channels,
                        volume,buffer
                    ),

                    StreamData::Output{buffer:UnknownTypeOutputBuffer::F32(mut buffer)}
                    =>output(
                        &mut track,
                        &mut rate_converter,
                        format.channels,
                        volume,buffer
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

                        format.channels=output_format.into_channels();

                        rate_converter=RateConverter::new(
                            track.sample_rate,
                            format.sample_rate.0,
                            &mut track,
                        );

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

/// Отправляет команду для остановки и ожидает окончание работы потока.
/// 
/// Sends the closing command and waits for the thread to finish.
impl Drop for Audio{
    fn drop(&mut self){
        let _=self.command.send(AudioSystemCommand::Close);

        if let Some(stream)=self.stream.lock().unwrap().as_ref(){
            self.event_loop.play_stream(stream.clone());
        }

        if let Some(thread)=self.thread.take(){
            let _=thread.join();
        }
    }
}

fn output<S:SampleTransform>(
    track:&mut PlayingTrack,
    converter:&mut RateConverter,
    channels:u16,
    volume:f32,
    mut buffer:OutputBuffer<S>
){
    let len=buffer.len()/channels as usize;

    let mut c=0;
    let mut frame=Vec::with_capacity(channels as usize);

    converter.next(track,&mut frame);

    for b in buffer.iter_mut(){

        if c==channels{
            converter.next(track,&mut frame);
            c=0;
        }

        if !frame.is_empty(){
            let sample=frame.remove(0);
            *b=SampleTransform::from(sample,volume)
        }

        c+=1;
    }
}