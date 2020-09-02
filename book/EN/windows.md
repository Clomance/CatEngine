# General

### Definitions

'Page' - types that are implemented for `WindowPage`.

There are three types of windows. They differ in speed and capabilities (they will be discussed separately below):

- `DefaultWindow` - has widest range of capabalities, but is the most slowly one and has a lot of problems
- `PagedWindow` - handles events faster than the others, but slowly switches 'pages', got two way of working, that can be alternated
- `DynamicWindow` - a bit slower than `PagedWindow`, but switches 'pages' much more quickly

### Building

Threre are two functions to create a window:
 - with a closure - gives the default settings and a list of monitors (to set to fullscreen)
```
let mut window=DefaultWindow::new(|monitors,settings|{
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

let mut window=PagedWindow::raw(
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
 - the event loop has been closed (to close a 'page' for `PagedWindow`)
 - the window gained or lost focus
 - keyboard and mouse events
 - modifiers has been changed (Shift,Ctrl,Alt,Logo)
 - a file is being hovered (only with `feature = "file_drop"`)



# WindowBase

A window base with graphics functions,
 an event loop and user's event generator included.

You can create you own window with it.
All it's field are public.

Also almost all the features are included.



# DefaultWindow

All events are handled and added to the outer handling queue (Window.events)
to work with them outside of the window structure.

Doesn't support 'pages'.

Breaking the cycle with `break`.

```
let mut window=DefaultWindow::new(|_,_|{}).unwrap();

while let Some(event)=window.next_event(){
    match event{
        WindowEvent::CloseRequested=>{
            // Break the cycle manually
            break
        }

        WindowEvent::Update=>{
            // Some actions
        }

        WindowEvent::RedrawRequested=>{
            // Rendering
        }
        _=>{}
    }
}
```



# PagedWindow

Breaking the cycle with `stop_events` function.

### Working with 'pages'

All the events are implemented with `WindowPage` trait
and handled immediately after emited.

```
pub struct Page;

impl WindowPage<'static> for Page{
    type Window=PagedWindow;
    type Output=();

    fn on_window_close_requested(&mut self,_window:&mut PagedWindow){
        // automatically breaks the cycle
    }

    #[cfg(not(feature="lazy"))]
    fn on_update_requested(&mut self,_window:&mut PagedWindow){
        // Some actions
    }

    fn on_redraw_requested(&mut self,_window:&mut PagedWindow){
        // Rendering
    }

    fn on_mouse_pressed(&mut self,_window:&mut PagedWindow,_button:MouseButton){}
    fn on_mouse_released(&mut self,_window:&mut PagedWindow,_button:MouseButton){}
    fn on_mouse_moved(&mut self,_window:&mut PagedWindow,_:[f32;2]){}
    fn on_mouse_scrolled(&mut self,_window:&mut PagedWindow,_:MouseScrollDelta){}

    fn on_keyboard_pressed(&mut self,_window:&mut PagedWindow,button:KeyboardButton){}

    fn on_keyboard_released(&mut self,_window:&mut PagedWindow,_button:KeyboardButton){}

    fn on_character_recieved(&mut self,_window:&mut PagedWindow,_character:char){}

    fn on_window_resized(&mut self,_window:&mut PagedWindow,_new_size:[u32;2]){}

    fn on_suspended(&mut self,_window:&mut PagedWindow){}
    fn on_resumed(&mut self,_window:&mut PagedWindow){}

    fn on_window_moved(&mut self,_window:&mut PagedWindow,_:[i32;2]){}

    fn on_window_focused(&mut self,_window:&mut PagedWindow,_:bool){}

    fn on_modifiers_changed(&mut self,_window:&mut PagedWindow,_modifiers:ModifiersState){}

    #[cfg(feature="file_drop")]
    fn on_file_dropped(&mut self,_:&mut PagedWindow,_:PathBuf){}
    #[cfg(feature="file_drop")]
    fn on_file_hovered(&mut self,_:&mut PagedWindow,_:PathBuf){}
    #[cfg(feature="file_drop")]
    fn on_file_hovered_canceled(&mut self,_:&mut PagedWindow){}

    fn on_event_loop_closed(&mut self,_:&mut PagedWindow){}
}


fn main(){
    let mut window=PagedWindow::new(|_,_|{}).unwrap();

    let mut page=Page;

    window.run_page(&mut page);
}
```

### Working with closures

All the events are handled with a closure.

```
let mut window=PagedWindow::new(|_,_|{}).unwrap();

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



# DynamicWindow

All the events are implemented with `WindowPage` trait
and handled immediately after emited.

The window that uses 'pages' as `WindowPage` trait objects,
so you can change one while running another.

Breaking the cycle with `stop_events` function.

```
pub struct Page;

impl<'a> WindowPage<'a> for Page{
    type Window=DynamicWindow<'a>;
    type Output=();

    fn on_window_close_requested(&mut self,_window:&mut DynamicWindow<'a>){
        // automatically breaks the cycle
    }

    #[cfg(not(feature="lazy"))]
    fn on_update_requested(&mut self,_window:&mut DynamicWindow<'a>){
        // Some actions
    }

    fn on_redraw_requested(&mut self,_window:&mut DynamicWindow<'a>){
        // Rendering
    }

    fn on_mouse_pressed(&mut self,_window:&mut DynamicWindow<'a>,_button:MouseButton){}
    fn on_mouse_released(&mut self,_window:&mut DynamicWindow<'a>,_button:MouseButton){}
    fn on_mouse_moved(&mut self,_window:&mut DynamicWindow<'a>,_:[f32;2]){}
    fn on_mouse_scrolled(&mut self,_window:&mut DynamicWindow<'a>,_:MouseScrollDelta){}

    fn on_keyboard_pressed(&mut self,_window:&mut DynamicWindow<'a>,button:KeyboardButton){}

    fn on_keyboard_released(&mut self,_window:&mut DynamicWindow<'a>,_button:KeyboardButton){}

    fn on_character_recieved(&mut self,_window:&mut DynamicWindow<'a>,_character:char){}

    fn on_window_resized(&mut self,_window:&mut DynamicWindow<'a>,_new_size:[u32;2]){}

    fn on_suspended(&mut self,_window:&mut DynamicWindow<'a>){}
    fn on_resumed(&mut self,_window:&mut DynamicWindow<'a>){}

    fn on_window_moved(&mut self,_window:&mut DynamicWindow<'a>,_:[i32;2]){}

    fn on_window_focused(&mut self,_window:&mut DynamicWindow<'a>,_:bool){}

    fn on_modifiers_changed(&mut self,_window:&mut DynamicWindow<'a>,_modifiers:ModifiersState){}

    #[cfg(feature="file_drop")]
    fn on_file_dropped(&mut self,_:&mut DynamicWindow<'a>,_:PathBuf){}
    #[cfg(feature="file_drop")]
    fn on_file_hovered(&mut self,_:&mut DynamicWindow<'a>,_:PathBuf){}
    #[cfg(feature="file_drop")]
    fn on_file_hovered_canceled(&mut self,_:&mut DynamicWindow<'a>){}

    fn on_event_loop_closed(&mut self,_:&mut DynamicWindow<'a>){}
}

fn main(){
    let mut window=DynamicWindow::new(|_,_|{}).unwrap();

    let mut page=Page;

    window.run(&mut page);
}
```