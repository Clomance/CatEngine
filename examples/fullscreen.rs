use cat_engine::{
    App,
    AppAttributes,

    window::{
        Window,
        Monitor,
        Fullscreen,
        VirtualKeyCode,
    },

    graphics::GraphicsManager,

    system::{
        System,
        StartSystem,
        SystemManager,
        SystemEvent,
        SystemStatus,
        ResourceManager,
        ComponentManager,
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
        _objects: ObjectManager,
        _resources: ResourceManager,
        _components: ComponentManager
    ) -> Self::Objects {

    }

    fn handle(
        &mut self,
        event: SystemEvent,
        _objects: &mut Self::Objects,
        shared: &mut Self::SharedData,
        manager: SystemManager
    ) -> SystemStatus {
        match event {
            SystemEvent::Keyboard { state: true, key: VirtualKeyCode::F } => {
                if *shared {
                    manager.window.set_fullscreen(Fullscreen::None);
                }
                else {
                    manager.window.set_fullscreen(Fullscreen::Monitor(Monitor::get_primary_monitor()));
                }
                *shared = !*shared;
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