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

##### mouse_cursor_icon

Replaces the default mouse cursor with user's image.

The cursor points to the center of the image.

The default path to the image is `./mouse_cursor_icon.png`.

The ImageBase for the image binds to the texture vertex buffer.
The default range is 4..8.

##### auto_hide

The window gets minimized when loses focus.
It gets back when gains focus.

Only the next event are able in minimized state:
- получение фокуса, `WindowEvent::Focused(true)`
 - приостановка или возобновление приложения,
 `WindowEvent::Suspended` и `WindowEvent::Resumed`
 - закрытие окна, `WindowEvent::Exit`


##### fps_counter

Adds a simple fps counter. The value updates every second.

##### paged_format

Replaces the default window structure with the new format one.



# The default window structure

All events are handled and added to the outer handling queue (Window.events)
to work with them outside of the window structure.



# The new format window structure

### Working with 'pages'

All the events are implemented with `WindowPage`
and handled immediately after emited.

### Working with closures

All the events are handled in a closure.

### lazy

The drawing event is emitted only when any action is happend.