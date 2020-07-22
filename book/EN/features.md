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

Enables to use functions to work with plain objects.



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