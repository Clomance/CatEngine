use crate::system::SystemEvent;

use cat_engine_basement::winapi::{
    EventLoopManager,
    window::Window,
};



pub trait AppSystem{
    type CreateParameters;
    type SharedData;
    type StartSystem;

    fn create_shared_data(create_parameters:&mut Self::CreateParameters)->Self::SharedData;

    fn create_start_system(
        create_parameters:&mut Self::CreateParameters,
        window:&Window,
        shared:&mut Self::SharedData,
    )->Self::StartSystem;

    fn event(
        &mut self,
        event:SystemEvent,
        window:&Window,
        event_loop:&mut EventLoopManager,
        shared:&mut Self::SharedData
    );
}