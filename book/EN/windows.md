# General

There are three types of windows. They differ in speed and capabilities (they will be discussed separately below).

Всего три вида окон. Отличаются они по скорости работы и возможностям (ниже будут рассмотрены по отдельности).

Threre are two functions to create a window:
 * with a closure - gives the default settings and the list of monitors (to set to fullscreen)
```
let mut window=DefaultWindow::new(|monitors,settings|{
    let monitor=monitors.remove(0);
    let fullscreen=cat_engine::glium::glutin::window::Fullscreen::Borderless(monitor);
    window_settings.window_attributes.fullscreen=Some(fullscreen);
}).unwrap();
```

* with fully-manually setting
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

Windows support the next events:
 - requests to close the window
 - changes of the size or the position of the window
 - updates (only with `feature != "lazy"`)
 - requests to redraw the window
 - the window has been suspended or resumed
 - the event loop has been stopped (to close a 'page' for `PagedWindow` и `DynamicWindow`)
 - the window gained or lost focus
 - keyboard and mouse events
 - modifiers has been changed (Shift,Ctrl,Alt,Logo)
 - a file is being hovered (only with `feature = "file_drop"`)

# WindowBase

A window base with graphics functions included.

You can create you own window with it.
All it's field are public.

It also includes almost all the features.

# DefaultWindow

This window has wide range of features, but is the most slowly one.

All events are handled and added to the outer handling queue (Window.events)
to work with them outside of the window structure.


# PagedWindow

### Working with 'pages'

All the events are implemented with `WindowPage` trait
and handled immediately after emited.


### Working with closures

All the events are handled with a closure.


# DynamicWindow

The fastest window, but very limited.

All the events are implemented with `WindowPage` trait
and handled immediately after emited.

The window that uses 'pages' as `WindowPage` trait objects,
so you can change one while running another.