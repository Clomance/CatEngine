use crate::InnerWindowEvent;

use glium::glutin::{
    ContextBuilder,
    NotCurrent,
    monitor::MonitorHandle,
    event_loop::EventLoop,
    event::{WindowEvent,MouseScrollDelta,ModifiersState},
    window::WindowBuilder,
    platform::desktop::EventLoopExtDesktop,
};

pub trait App:Sized{
    type StartArgs;
    fn on_create(&mut self)->Self::StartArgs;
    fn on_start(&mut self,args:Self::StartArgs);
    fn on_suspend(&mut self);
    fn on_resume(&mut self);

    fn on_destroy(&mut self);

    fn run(mut self){
        let args:Self::StartArgs=self.on_create();
        self.on_start(args);

        let mut event_loop=EventLoop::<InnerWindowEvent>::with_user_event();

        event_loop.run_return(|event,_,control_flow|{

        });

        self.on_destroy();
    }
}