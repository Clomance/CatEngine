use cat_engine::{
    App,
    AppAttributes,

    window::{
        Window,
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

    object::ObjectManager
};

pub struct ExampleSystem;

impl<'s, 'a> System<'s, 'a> for ExampleSystem{
    type SharedData = ();
    type Objects = ();

    fn set_up(
        &mut self,
        _shared: &mut Self::SharedData,
        _objects:ObjectManager,
        _resources:ResourceManager,
        _components:ComponentManager
    ) -> Self::Objects {

    }

    fn handle(
        &mut self,
        _event: SystemEvent,
        _objects: &mut Self::Objects,
        _shared: &mut Self::SharedData,
        _system_manager: SystemManager
    ) -> SystemStatus {

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

    fn create_shared_data(
        _create_parameters: &mut Self::CreateParameters
    ) -> Self::SharedData {

    }
}

fn main() {
    let attributes = AppAttributes::new("ExampleWindow");

    let mut app = App::new::<ExampleSystem>(attributes, &mut ()).unwrap();

    app.run();
}