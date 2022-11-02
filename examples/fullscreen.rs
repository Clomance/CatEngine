use cat_engine::{
    App,
    AppAttributes,

    window::{
        Window,
        Monitor,
        Fullscreen,
        VirtualKeyCode,
    },

    graphics::Graphics,

    system::{
        System,
        StartSystem,
        SystemManager,
        SystemEvent,
        SystemStatus,
    },

    object::{
        ObjectManager,
    }
};

pub struct ExampleSystem;

impl<'s, 'a> System<'s, 'a> for ExampleSystem {
    type SharedData = bool;
    type Objects = ();

    fn set_up(
        &mut self,
        _shared: &mut Self::SharedData,
        _object_manager: ObjectManager
    ) -> Self::Objects {

    }

    fn handle(
        &mut self,
        _objects: &mut Self::Objects,
        event: SystemEvent,
        window: &Window,
        shared: &mut Self::SharedData,
        _system_manager: SystemManager
    ) -> SystemStatus {
        match event{
            SystemEvent::Keyboard { state: true, key: VirtualKeyCode::F } => {
                if *shared{
                    window.set_fullscreen(Fullscreen::None);
                }
                else{
                    window.set_fullscreen(Fullscreen::Monitor(Monitor::get_primary_monitor()));
                }
                *shared = !*shared;
            }

            _=>{}
        }

        SystemStatus::Next
    }

    fn destroy(
        &mut self,
        _shared:&mut Self::SharedData,
        _graphics:&mut Graphics
    ) {

    }
}

impl<'s, 'a> StartSystem<'s, 'a> for ExampleSystem {
    type CreateParameters = ();

    fn create(
        _create_parameters: &mut Self::CreateParameters,
        _window: &Window, 
        _shared: &mut Self::SharedData
    ) -> ExampleSystem {
        ExampleSystem
    }

    fn create_shared_data(_create_parameters: &mut Self::CreateParameters) -> Self::SharedData {
        false
    }
}

fn main() {
    let attributes = AppAttributes::new("ExampleWindow");

    let mut app = App::new::<ExampleSystem>(attributes, &mut ()).unwrap();

    app.run();
}