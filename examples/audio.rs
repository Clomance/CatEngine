use std::{path::Path, fs::File};

use cat_engine::{
    App,
    AppAttributes,
    window::Window,
    graphics::GraphicsManager,
    object::{
        ObjectManager,
        ObjectReference,
    },
    system::{
        System,
        StartSystem,
        SystemManager,
        SystemEvent,
        SystemStatus,
        ResourceManager,
        ComponentManager,
    },
};

use cat_engine::audio::{
    SimpleStereoObject,
    Source,
    SourceData,
};
use minimp3::Decoder;


pub struct ExampleSystem;

impl<'s, 'a> System<'s, 'a> for ExampleSystem {
    type SharedData = ();
    type Objects = ObjectReference<'a, SimpleStereoObject>;

    fn set_up(
        &mut self,
        _shared: &mut Self::SharedData,
        mut object_manager: ObjectManager,
        mut resources: ResourceManager,
        component: ComponentManager
    ) -> Self::Objects {
        let track = load_track("resources/cold-war-kids-first.mp3");
        let sound_data = SourceData::I16(track);
        let sound = Source::new(sound_data, 2, 44100);

        resources.audio.add(sound);
        let source = resources.audio.get(0).unwrap();

        let wave_format = component.audio_stream.wave_format();

        let audio_object = SimpleStereoObject::new(1, &wave_format, source);

        object_manager.audio.push(audio_object)
    }

    fn handle(
        &mut self,
        event: SystemEvent,
        objects: &mut Self::Objects,
        _shared: &mut Self::SharedData,
        _system_manager: SystemManager
    ) -> SystemStatus {
        match event{
            SystemEvent::Update=>{
                if objects.finished(){
                    return SystemStatus::Exit
                }
            }
            _=>{}
        }

        SystemStatus::Next
    }

    fn destroy(
        &mut self,
        _shared: &mut Self::SharedData,
        _graphics: GraphicsManager
    ) {

    }
}

impl<'s, 'a> StartSystem<'s, 'a> for ExampleSystem{
    type CreateParameters = ();

    fn create(
        _create_parameters: &mut Self::CreateParameters,
        _window: &Window,
        _shared: &mut Self::SharedData
    ) -> ExampleSystem {
        ExampleSystem
    }

    fn create_shared_data(
        _create_parameters: &mut Self::CreateParameters
    ) -> Self::SharedData {
        
    }
}



fn main(){
    let attributes=AppAttributes::new("ExampleWindow");

    let mut app=App::new::<ExampleSystem>(attributes,&mut ()).unwrap();

    app.run();
}



pub fn load_track<P:AsRef<Path>>(path:P)->Vec<i16>{
    let mut data=Vec::new();

    let file=match File::open(path){
        Ok(file)=>file,
        Err(_)=>return data,
    };

    let mut decoder=Decoder::new(file);

    let (_channels,_sample_rate)=match decoder.next_frame(){
        Ok(mut frame)=>{
            data.append(&mut frame.data);
            (
                frame.channels,
                frame.sample_rate as u32,
            )
        }
        Err(_)=>return data
    };

    while let Ok(mut frame)=decoder.next_frame(){
        data.append(&mut frame.data);
    }

    data
}