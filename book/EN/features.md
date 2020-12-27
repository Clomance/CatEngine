# Audio

### audio

Enables the audio output engine.
Standard functions.

### extended_audio

Enables the audio output engine with direct command acces to the core as an addition.
Standard functions remain.

### raw_audio

Enables the audio output engine only with direct command acces to the core.
Standard functions are removed.



# Graphics

### texture_graphics

Enable operating and rendering textured objects.

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

### ups_counter

Adds a simple ups counter. The value updates every second.


### file_drop

Enables the next events:
 - `WindowEvent::HoveredFileCancelled`
 - `WindowEvent::HoveredFile(PathBuf)`
 - `WindowEvent::DroppedFile(PathBuf)`

and the related functions.

### lazy

The drawing event is emitted only when any action is happened.