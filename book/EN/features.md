# Why do we need features

Features help us to remove unnecessary code and modules and lessen the size of some structures.


# Audio

### audio

Enables audio output.



# Graphics

### texture_graphics

Enables operating and rendering textured objects.

### text_graphics

Enables operating and rendering text.

### simple_graphics

Enables operating and rendering plain (for now) objects.



# Window additions

### auto_hide

The window gets minimized when loses focus.
It gets back when gains focus.

Only the next events are able in minimized state:
 - gaining focus, `WindowEvent::Focused(true)`
 - suspending or resuming the application,
 `WindowEvent::Suspended` и `WindowEvent::Resumed`
 - closing the window, `WindowEvent::Exit`
 - resizing the window, `WindowEvent::Resized(size)`

It's usefull for fullscreen application.

### fps_counter

Adds a simple fps counter. The value updates every second.

### file_drop

Enables the next events:
 - `WindowEvent::HoveredFileCancelled`
 - `WindowEvent::HoveredFile(PathBuf)`
 - `WindowEvent::DroppedFile(PathBuf)`

and the related functions.



## PagedWindow and DynamicWindow

### lazy

The drawing event is emitted only when any action is happened.