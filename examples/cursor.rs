use cat_engine::{
    App,
    AppAttributes,

    window::{
        Window,
        CursorIcon,
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
        ObjectManager
    },
};

pub struct ExampleSystem;

impl<'s, 'a> System<'s, 'a> for ExampleSystem {
    type SharedData = ();
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
        _event: SystemEvent,
        _window: &Window,
        _shared: &mut Self::SharedData,
        _system_manager: SystemManager<'a>
    ) -> SystemStatus {
        SystemStatus::Next
    }

    fn destroy(
        &mut self,
        _shared: &mut Self::SharedData,
        _graphics: &mut Graphics
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

    fn create_shared_data(
        _create_parameters: &mut Self::CreateParameters
    ) -> Self::SharedData {

    }
}

fn main() {
    let mut attributes = AppAttributes::new("ExampleWindow");

    let cursor_icon = image::open("logo_400x400.png").unwrap().to_bgra8();
    attributes.class.cursor_icon = CursorIcon::BGRA8 {
        position: [200u32; 2],
        image: cursor_icon,
    };

    let mut app = App::new::<ExampleSystem>(attributes, &mut ()).unwrap();

    app.run();
}