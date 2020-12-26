# 0.6.0

### Removed

A lifetime from the `DependentObject` trait.

`DynamicWindow` and the `Window` trait.

`WindowBase` from public view.

The `Window` field from the `WindowPage` trait.

### Function changes

The window `draw` function now takes `&self`, `Graphics2D` and a closure with `&mut Graphics` as an argument.
```
pub fn draw<F:FnOnce(&mut Graphics)>(&self,graphics_base:&Graphics2D,f:F)->Result<(),SwapBuffersError>;
```

The `on_redraw_requested` function of the `WindowPage` trait now takes `&Window`.
```
fn on_redraw_requested(&mut self,window:&Window);
```

Arguments of `ImageBase::new` was swapped:
```
pub fn new([x,y,width,height]:[f32;4],colour_filter:Colour)->ImageBase;
```

Renamed `delete` functions to `remove` ones.

### Other

Renamed `PagedWindow` to `Window`.

Separated graphics of windows, but it's still created within them.

Updated the `glium` crate to `0.29.0`.

Added `DrawDarameters` to `Graphics`.

# 0.5.5

Fixed `cat_audio`'s features.

# 0.5.4

Audio is now a separate crate and added as a default feature.

Fixed the bug of multichannel playing.

# 0.5.3

Added `ImageObject`.

Removed `DefaultWindow`.

# 0.5.2

Updated the `image` crate to `0.23.12`.

Fixed the bug of starting playing an audio output stream after changing a panicked device on some platforms.
Now it saves the last stream's state.

# 0.5.1

Enabled new audio features: `extended_audio`,`raw_audio`.

# 0.5.0

### Updated

The `image` crate to `0.23.11`.
The `ttf-parser` crate to `0.8.3`.

### Changed

`TexturedObject` doesn't owe a texture, it should be added seperatly.

### A new audio system

Now you are able to control each audio channel yourself.

```
let audio=Audio::default(AudioSettings::new()).unwrap();

let mut wrapper=AudioWrapper::new(audio);

wrapper.load_track("audio.mp3","test".to_string());

wrapper.play_track("test").unwrap();

// Pausing one channel of the track
wrapper.audio.pause_track(0).unwrap();
```

# 0.5.0-alpha

### A new text rendering system

It is based on a glyph cache.
Load all the needed characters as textures to the cache and then render them.

It's possible to load different fonts in one cache, so you can create your own, but it still have problems with scaling.

As usual you are able to save a glyph cache (in `CachedFont`) inside the graphics structure to use text as a dependent object.

Works faster but takes much space.

### The old one

Now saved objects support only `CachedFont`.

Fully-rebuilded, too.

# 0.4.5

If an audio output device gets unavailable (for example, it has been unplugged),
the engine tries to replace it with the new one (choose the default).

Removed the `with_host_and_device`, `available_hosts`, `host_from_id`,
`default_output_device`, `output_devices` functions.

The `play_once`, `play_forever` functions are united in the `play_track` one.

Reimported the `cpal` crate.

The `AudioOutputType` enumeration renamed `AudioOutputFormat`.

The audio engine converts all sound tracks to the 24-bit format.

Removed `NoStream` from the `AudioCommandResult` enumeration.

# 0.4.4

Now you can create an empty texture.

Fixed the text rendering problem.

# 0.4.3

Updated crates: `minimp3` - 0.5.0; `image` - 0.23.9.

Now image rendering functions are generalized for all textured objects.

# 0.4.2

### Added

An object system for text graphics and some function for rendering (but there is some artifacts, I will fix them later).

General drawing functions.

### Removed

User's mouse cursor icon and it's feature.

The `alpha_channel` feature and the related functions and fields.

The `draw_all_textured_objects` function.

### Optimized

Glyph rendering.

# 0.4.1

Added output for the `DynamicWindow`.

# 0.4.0

### Fixed

Rendering user's mouse cursor.

Rendering a static textured object.

### Removed

The range in the `MouseCursorIconSettings`.

### Other

Text rendering fully has been remade. Now it works faster.

The `image` crate has been replaced from `crate::texture` to `crate`.

# 0.3.3

The `MouseCursor` is now public.

The `SimpleObject` trait has been replaced with the `DependentObject`,
and textured objects must be implemented for this new trait.

The `image` mod has been renamed to `texture`.

The `image` crate version has been changed to `0.23.8`.

# 0.3.2

### Added

Functions to change saved simple objects directly:
```
window.graphics().set_simple_object_colour(0,White);
window.graphics().rewrite_simple_object_vertices(0,&[100f32;4])
```

### Removed

The `draw_raw()` function of all the windows.

### Fixed

Removing simple objects now deletes their indices.
Removing all the simple objects does not delete the buffer offsets.

### Other

The range system for textures has been replaced with an object one.

All the field of the `Graphics` are now public.

Window drawing function takes `&mut self` instead of `&self`.

# 0.3.1

### Fixed

Feature errors.

### Other

The graphics engine is now divided into 2D and 3D (3D is not implemented yet).

# 0.3.0

### Added

A `file_drop` feature - enables the next events:
 - `WindowEvent::HoveredFileCancelled`
 - `WindowEvent::HoveredFile(PathBuf)`
 - `WindowEvent::DroppedFile(PathBuf)`

and related functions.

A new event - `WindowEvent::ModifiersChanged(ModifiersState)`.

Rotating functions for simple objects.

### Removed

User's mouse cursor and everything related to it.

### Other

Changes in the `WindowSettings` fields.

The `WindowEvent::Draw` event has been renamed to `WindowEvent::RedrawRequested`.
The `WindowEvent::Exit` event has been renamed to `WindowEvent::CloseRequested`.
The `WindowPage::on_close_requested()` function has been renamed to `WindowPage::on_window_close_requested()`.

The range system for simple objects has been replaced with an object one.
Now you can add an object to the array and then quickly draw it or the whole array.

Some changes in the `SimpleObject` trait.

All the geometric shapes (`cat_engine::graphics::*;`) has been moved to `cat_engine::shapes::*`.

# Version 0.2.1

### Added

Update events. You can configure update frequency in the `WindowSettings` (`GeneralSettings`).

`WindowEvent::EventLoopClosed` and `WindowEvent::Update` events.

A `type Output` field and new functions (`on_update_requested()`, `on_event_loop_closed()->Output`) for the `WindowPage` trait. The function `run_page()` of the `PagedWindow` now return the `Output`.

A type defenition `PageRef<'a>` for `WindowPage` trait objects.

### Deleted

The `WindowEvent::Hide(bool)` event. Now only the `WindowEvent::Focused(bool)` event is available.

### Fixed

The `auto_hide` feature now works fine with the `DynamicWindow`.

### Other

Replaced the local `MouseButton` enum with `glutin` one.

The `WindowBase` has got a `event_loop_proxy` field with `EventLoopProxy<InnerWindowEvent>` type
and the `event_loop` field type changed to `EventLoop<InnerWindowEvent>`.

The `initial_colour` field has been moved to the `GeneralSettings`.


# Version 0.2.0

Now all the windows are compatible.
You can convert one into another, except `DynamicWindow` - you can only convert into it.

# Version 0.1.5

### User's mouse cursor icon (feature = "mouse_cursor_icon")

Now all the errors of user's mouse cursor icon panic with an appropriate message.

New settings for user's mouse cursor icon.

### Added

The new window types are `DynamicWindow` - a window that uses 'pages' as `WindowPage` trait objects,
and `WindowBase` - a window base without event handlers.

`Window` - a trait for windows. You can create easily your own window with `WindowBase`.
All the windows are implemented with it except `WindowBase`.

### Deleted

The `paged_format` feature.

All the window modifying functions.
Now the only way to customize the window is:
```
let wi=window.display().gl_window();
let w=wi.window();
w.set_minimized(false);
w.set_visible(false);
```

### Other changes

The window of the default window format is now named `DefaultWindow`.

The `WindowPage` trait got the `type Window` field and a life parameter.

# Version 0.1.4

### Fixed

The closure event listeners works correct with the `WindowEvent::Resized` event.

For the `auto_hide` feature:
 when the current 'page' is closed and
 the current event listener is for the window minimized state 
 the next page will be opened with the same listener.

### Features

Alpha smoothing functions are now able only with the `alpha_smoothing` feature.

# Version 0.1.3

Now all drawing functions return the result of drawing.

Added the center of rotation for images.

Added lazy feature (see [features](FEATURES.MD)).