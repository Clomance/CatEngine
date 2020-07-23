# Why do we need features

Features help to remove unnecessary code and lessed size if some structures.



# General features

### Audio

##### audio

Enables to use audio output.



### Graphics

##### texture_graphics

Enables to use functions to work with textures.

##### text_graphics

Enables to use functions to work with text.

##### simple_graphics

Enables to use functions to work with plain (for now) objects.

There are functions to save these objects for a quicker access:

```
let mut rect=Rectangle::new([100f32;4],[0.0,0.0,0.0,1.0]);
// there is no need for the rectangle after adding it
window.graphics().add_simple_object(&rect).unwrap();
.
.
.
window.draw(|p,g|{
    g.draw_simple_object(0,p).unwrap();
}).unwrap();
```

This a bit speeds up drawing.


### Window additions

##### auto_hide

The window gets minimized when loses focus.
It gets back when gains focus.

Only the next events are able in minimized state:
 - gaining focus, `WindowEvent::Focused(true)`
 - suspending or resuming the application,
 `WindowEvent::Suspended` и `WindowEvent::Resumed`
 - closing the window, `WindowEvent::Exit`
 - resizing the window, `WindowEvent::Resized(size)`

It's usefull for fullscreen application.

##### fps_counter

Adds a simple fps counter. The value updates every second.

##### alpha_smoothing

Adds functions for drawing with changing alpha channel.

##### file_drop

Enables the next events:
 - `WindowEvent::HoveredFileCancelled`
 - `WindowEvent::HoveredFile(PathBuf)`
 - `WindowEvent::DroppedFile(PathBuf)`

and related functions.



# PagedWindow and DynamicWindow

### lazy

The drawing event is emitted only when any action is happened.