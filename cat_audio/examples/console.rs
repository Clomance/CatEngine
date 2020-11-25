#![allow(non_upper_case_globals)]

use cat_audio::{
    AudioSettings,
    Audio,
    cpal,
    cpal::traits::HostTrait,
    cpal::traits::DeviceTrait,
    AudioWrapper,
};

const commands:&'static str=
"\
    load [path] [name] - добавляет трек в хранилище\n\
    remove [name] - удаляет трек из хранилища\n\
    clear - очищает хранилище треков\n\
    play - возобноляет проигрывание аудио потока\n\
    play [name] - начинает проигрывание трека\n\
    pause - ставит на паузу аудио поток\n\
    pause [name] - ставит трек на паузу\n\
    pause [name] --d - снимает паузу с трека\n\
    stop - очищает плейлист\n\
    stop [name] - останаливает трек\n\
    set --g volume [value] - устанавливает общую громкость\n\
    set --t [name] volume [value] - устанавливает громкость трека\n\
    close - закрывает поток и консоль\n\
";

fn main(){
    let settings=AudioSettings::new();
    let host=cpal::default_host();
    let audio=Audio::new(host,|host|{
            host.default_output_device().unwrap()
        },
        |device|{
            device.default_output_format().unwrap()
        },
        settings
    ).unwrap();

    let mut audio_wrapper=AudioWrapper::new(audio);

    audio_wrapper.load_track("../resources/audio1.mp3","K".to_string());
    // let sets=audio_wrapper.get_mut_track_sets("K").unwrap();
    // sets[0].channels.push(0); Additional channels
    // sets[1].channels.push(1); Additional channels
    audio_wrapper.load_track("../resources/audio3.mp3","N".to_string());


    let input=std::io::stdin();
    let mut line=String::new();
    while let Ok(_)=input.read_line(&mut line){
        let mut parts=line.trim().split_ascii_whitespace();
        if let Some(command)=parts.next(){
            match command{
                "help"=>{
                    print!("{}",commands);
                }

                "load"=>if let Some(path)=parts.next(){
                    if let Some(name)=parts.next(){
                        println!("Loading {}...",path);
                        if audio_wrapper.load_track(path,name.to_string()){
                            println!("Loaded");
                        }
                        else{
                            println!("Error");
                        }
                    }
                }

                "remove"=>if let Some(option)=parts.next(){
                    match option{
                        "--s"=>{
                            if let Some(_track)=parts.next(){
                                //let index:usize=track.parse().unwrap();
                                //audio_wrapper.remove_mono_track(index);
                            }
                        }
                        name=>audio_wrapper.remove_track(name).unwrap()
                    }
                }

                "clear"=>audio_wrapper.clear_storage().unwrap(),

                "play"=>if let Some(option)=parts.next(){
                    match option{
                        // single channel
                        "--s"=>{
                            if let Some(track)=parts.next(){
                                let _index:usize=track.parse().unwrap();
                                //audio_wrapper.unpause_mono_track(index);

                            //     // Номер канала
                            //     if let Some(channel)=parts.next(){
                            //         let channel:usize=channel.parse().unwrap();

                            //         if let Some(repeats)=parts.next(){
                            //             let repeats=repeats.parse().unwrap();
                            //             audio.play_track(track,&[channel],repeats).unwrap();
                            //         }
                            //         else{
                            //             audio.play_track(track,&[channel],1).unwrap();
                            //         }
                            //     }
                            }
                        }

                        name=>if let Some(repeats)=parts.next(){
                            let repeats=repeats.parse::<u32>().unwrap();
                            audio_wrapper.play_track(name,repeats).unwrap()
                        }
                    }
                }
                else{
                    // Убрать паузу с потока
                    audio_wrapper.play();
                }

                "pause"=>if let Some(option)=parts.next(){
                    match option{
                        // single channel
                        "--s"=>{
                            if let Some(track)=parts.next(){
                                let _index:usize=track.parse().unwrap();
                                //audio_wrapper.pause_mono_track(index).unwrap();
                            }
                        }

                        name=>if let Some(parameter)=parts.next(){
                            if parameter=="--d"{
                                audio_wrapper.unpause_track(name).unwrap()
                            }
                        }
                        else{
                            audio_wrapper.pause_track(name).unwrap()
                        }
                    }
                }
                else{
                    audio_wrapper.pause().unwrap()
                }

                "set"=>if let Some(option)=parts.next(){
                    match option{
                        "--g"=>if let Some(parametr)=parts.next(){
                            if let Some(value)=parts.next(){
                                match parametr{
                                    "volume"=>{
                                        let volume=value.parse().unwrap();
                                        audio_wrapper.set_general_volume(volume);
                                    }
                                    _=>{}
                                }
                            }
                        }
                        "--t"=>if let Some(name)=parts.next(){
                            if let Some(parametr)=parts.next(){
                                if let Some(value)=parts.next(){
                                    match parametr{
                                        "volume"=>{
                                            let volume:f32=value.parse().unwrap();
                                            audio_wrapper.set_track_volume(name,volume).unwrap()
                                        }
                                        _=>{}
                                    }
                                }
                            }
                        }
                        _=>{}
                    }
                }

                "stop"=>if let Some(name)=parts.next(){
                    audio_wrapper.stop_track(name).unwrap()
                }
                else{
                    audio_wrapper.clear_playlist().unwrap()
                }

                "close"=>break,
                _=>{}
            }
        }

        line.clear();
    }
}