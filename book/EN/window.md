# Introduction

### Definitions

'Page' - types that are implemented for `WindowPage`.

### Building

Threre are two functions to create a window:
 - with a closure - gives the default settings and a list of monitors (to set to fullscreen)
```
let mut window=Window::new(|monitors,settings|{
    let monitor=monitors.remove(0);
    let fullscreen=cat_engine::glium::glutin::window::Fullscreen::Borderless(monitor);
    window_settings.window_attributes.fullscreen=Some(fullscreen);
}).unwrap();
```

 - with manually setting
```
let graphics_settings=GraphicsSettings::new();
let general_settings=GeneralSettings::new();
let context_builder=ContextBuilder::new();
let window_builder=WindowBuilder::default();
let event_loop=EventLoop::<InnerWindowEvent>::with_user_event();

let mut window=Window::raw(
    window_builder,
    context_builder,
    graphics_settings,
    event_loop,
    general_settings,
).unwrap();
```

### Events

Windows support the next events:
 - requests to close the window
 - changes of the size or the position of the window
 - updates (only with `feature != "lazy"`)
 - requests to redraw the window
 - the window has been suspended or resumed
 - the event loop has been closed
 - the window gained or lost focus
 - keyboard and mouse events
 - modifiers has been changed (Shift,Ctrl,Alt,Logo)
 - a file is being hovered (only with `feature = "file_drop"`)


# Window

Breaking the cycle with `stop_events` function.

### Working with 'pages'

All the events are implemented with the `WindowPage` trait.

```
pub struct Page;

impl WindowPage<'static> for Page{
    type Window=Window;
    type Output=();

    fn on_window_close_requested(&mut self,_window:&mut Window){
        // automatically breaks the cycle
    }

    #[cfg(not(feature="lazy"))]
    fn on_update_requested(&mut self,_window:&mut Window){
        // Some actions
    }

    fn on_redraw_requested(&mut self,_window:&mut Window){
        // Rendering
    }

    fn on_mouse_pressed(&mut self,_window:&mut Window,_button:MouseButton){}
    fn on_mouse_released(&mut self,_window:&mut Window,_button:MouseButton){}
    fn on_mouse_moved(&mut self,_window:&mut Window,_:[f32;2]){}
    fn on_mouse_scrolled(&mut self,_window:&mut Window,_:MouseScrollDelta){}

    fn on_keyboard_pressed(&mut self,_window:&mut Window,button:KeyboardButton){}

    fn on_keyboard_released(&mut self,_window:&mut Window,_button:KeyboardButton){}

    fn on_character_recieved(&mut self,_window:&mut Window,_character:char){}

    fn on_window_resized(&mut self,_window:&mut Window,_new_size:[u32;2]){}

    fn on_suspended(&mut self,_window:&mut Window){}
    fn on_resumed(&mut self,_window:&mut Window){}

    fn on_window_moved(&mut self,_window:&mut Window,_:[i32;2]){}

    fn on_window_focused(&mut self,_window:&mut Window,_:bool){}

    fn on_modifiers_changed(&mut self,_window:&mut Window,_modifiers:ModifiersState){}

    #[cfg(feature="file_drop")]
    fn on_file_dropped(&mut self,_:&mut Window,_:PathBuf){}
    #[cfg(feature="file_drop")]
    fn on_file_hovered(&mut self,_:&mut Window,_:PathBuf){}
    #[cfg(feature="file_drop")]
    fn on_file_hovered_canceled(&mut self,_:&mut Window){}

    fn on_event_loop_closed(&mut self,_:&mut Window){}
}


fn main(){
    let mut window=Window::new(|_,_|{}).unwrap();

    let mut page=Page;

    window.run_page(&mut page);
}
```

### Working with closures

All the events are handled with a closure.

```
let mut window=Window::new(|_,_|{}).unwrap();

window.run(|window,event|{
    match event{
        WindowEvent::CloseRequested=>{
            // automatically breaks the cycle
        }

        WindowEvent::Update=>{
            // Some actions
        }

        WindowEvent::RedrawRequested=>{
            // Rendering
        }
        _=>{}
    }
});
```