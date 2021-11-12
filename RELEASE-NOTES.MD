# 1.0.0 - alpha8

### CatEngineBasement

Updated the `drawing` module in `GLCore`.

Added some functions for `X11`.

Optimized: now all `gl` functions are linked at compile time.

Fixed bad gl functions.

Fixed feature errors.


# 1.0.0 - alpha7

### CatEngineBasement

Fixed compile errors with shader programs on some OpenGL versions.

Fixed the window title error. Now it appears fine.

Added `WindowClass::register_indirect`, `WindowBackgroundSystemColour` and `WindowBackgroundColour` to `WinCore` for the `window_class` module.

Updated `Background`: added the `SystemColour` option, renamed `Colour` to `RGBColour`.

Fixed feature errors. Removed the `windowing` feature.

Added docs configurations with the `nightly-docs` feature.

### CatEngine

Some changes for future updates.


# 1.0.0 - alpha6

### The `WindowProcedure` trait
New functions:
 - `create` - called when an application requests that a window be created
 - `close_request` - called as a signal that a window or an application should terminate
 - `destroy` - called when a window is being destroyed, after the window is removed from the screen
 - `set_cursor` - called if the mouse causes the cursor to move within a window and mouse input is not captured (the `set_cursor_event` feature)
 - `resized` - called after window's size has changed
 - `moved` - called after a window has been moved
 - `catch_panic` - called when one of the functions above panics and `catch_unwind` catches the panic in `WndProc`

### Renamed
 - `WindowProcedure::render` to `paint`
 - `DrawParameters` from `Graphics` to `DrawParameters2D`
 - `GCore` to `GLCore`

### Added
 - the `AppWindowProcedure` trait - wraps `WindowProcedure`
 - the `set_cursor_event` feature for handling set cursor events
 - the `InstanceHandle`, `WindowClassInfo` structs
 - cursor, brush, icon raw  functions for Windows (`WinCore`)
 - the `CursorIcon::System` enumetion, it contains the system defined cursor
 - the `DrawParameters` struct to `GLCore`
 - the `GraphicsCore::flush` function

### Other

`CursorIcon::None` now doesn't set any cursor, use it for dynamic changing the cursor icon.
In that case use `Cursor::set` to set the cursor

The `quit` function has got an argument.

Drawing heap objects is safe. It doesn't panic if you set a wrong object id.
All objects with zero vertices are ignored, you can't add or push any.
Also ids returned by `add_heap_object` start from zero.


# 1.0.0 - alpha5

## CatEngineBasement - 0.0.0-alpha5

### Added
 - `WindowHandle` as a replacment for `HWND`
 - `DeviceContextHandle` as a replacment for `HDC` and `DeviceContext` to `WinCore`
 - `Cursor` to `WinCore`
 - `BitmapHandle` as a replacment for `HBITMAP` and `Bitmap` to `WinCore`
 - `WindowClassStyles` for easily setting the class styles
 - `WindowStyle`, `WindowStyles`, `ExtendedWindowStyle`, `ExtendedWindowStyles` for easily setting the window styles
 - `Window::show_window` and `ShowCommand` to `WinCore`
 - `Window::client_to_screen`, `Window::screen_to_client` to `WinCore`
 - a `WindowProcedure::render` function
 - all the GL `Buffer` functions that are available for OpenGL 3.3 to `GLCore`

### Removed
 - `Redraw` from `WindowEvent`.
 - the `VK` prefix from `VirtualKeyCode`.


# 1.0.0 - alpha4

Fixed `CachedFont::text_size` and `CachedFont::text_width` functions.

Now `Graphics::draw_char` set a horisontal advance of any given character.

All the `App` functions takes it as a reference, removed some, renamed the rest.

All the texture functions except `basement::graphics::core` ones now returns `GLError`.

Removed `CatAudio` from the default features.

## CatEngineBasement - 0.0.0-alpha4

Added wrappers for drawing functions (`graphics::level0`).

`Texture` and `Buffer` write functions from `graphics::level0` are now just wrappers.

All the buffer functions except `basement::graphics::core` ones returns `GLError`.

Added new features - `support` (enables the support module) and `windowing` (enables).


# 1.0.0 - alpha3

Fixed the `Texture::write_image` and `Texture::rewrite_image` functions.

Added `FontError` for the `FontOwner::load` function.

Renamed and removed some glyphs structures. Removed unnecessary functions.

Now the `Graphics::draw_char` function scale characters relative to the original size.
Later scaling was relative to glyph's current size,
so dynamically build and textured glyphs had different initial sizes.

## CatEngineBasement - 0.0.0-alpha3

Removed all the events to the `windows` module.

Now some posted to the thread messages are also handled with `EventLoop`.
The messages:
 - `WindowEvent::KeyPress`, `WindowEvent::KeyRelease`
 - `WindowEvent::CharacterInput`
 - `WindowEvent::MouseMove`, `WindowEvent::MousePress`, `WindowEvent::MouseRelease`, `WindowEvent::MouseScroll`

Added `core::texture::Texture::wrap_s`, `-::wrap_t` and `-::wrap_r` functions.


# 1.0.0 - alpha2

Updated the `ttf-parser` crate to `0.12.3` and the `minimp3` crate to `0.5.1`.

## CatEngineBasement - 0.0.0-alpha2

### Added
- new formats for `Texture2DInternalFormat` and `ImageDataFormat`
- `core::window::Window::get_window_long_ptr` and `-::set_window_long_ptr` functions and `WindowData` for them
- `WindowClass` to `core`
- a `VertexIntegerComponents` enum for the `core::vertex_array::VertexArray::attribute_pointer` function
- a `core::vertex_array::VertexArray::attribute_pointer_integer` function and `VertexIntegerComponents` and `IntegerDataType` for it
- a `Seconds` option for `EventInterval`

`ImageDataFormat` and `ImageDataType` are merged into `ImageDataFormat`.


# 1.0.0 - alpha1

## CatEngineBasement - 0.0.0-alpha1

### Added
 - `WindowsCore`. It wraps nessesary Windows API functions
 - an `opengl` feature. It enables using wrapped GL functions and context
 - `Window::set_fullscreen` and `Window::show_cursor` functions
 - a `vsync` field for `OpenGLRenderContextAttributes`, but other fields are removed
 - an `auto_redraw` flag that defines whether a new redraw event is requested directly after processing the last one. To set the flag use `Window::set_auto_redraw`
 - a fps lock, `EventLoop` controls frame rate, but you need to disable `auto_redraw` and vsync (`OpenGLRenderContext::set_vsync`)
 - an ability to disable update and redraw events and without changing the `LoopControl` flag. Just set an event rate to zero: `EventInterval::EventsPerSecond(0)`, `EventInterval::Ticks(0)` or `EventInterval::NanoSeconds(0)`

### Changed
 - `WindowInner::storage` -> `WindowInner::storage_ref`, `WindowInner::storage` now returns mutable reference

### Fixed
 - target module mistakes

### Events optimizations

#### The highest optimization.
If you don't need updates and high performance:
 - disable V-Sync,
 - set the `auto_redraw` flag to `false`,
 - set the maximum of frame rate (30 fps is enough),
 - set the 'lazy' mode for `EventLoop`
```rust
    render_context_attributes.vsync=false;
    window_attributes.auto_redraw=false;
    event_loop_attributes.redraw_request_interval=EventInterval::EventsPerSecond(30);
    event_control=EventControl::Lazy;
```

#### The highest performance
If you need updates and the lowest response time:
 - disable V-Sync,
 - set the `auto_redraw` flag to `false`,
 - set the needed frame rate,
 - set the default mode for `EventLoop`
```rust
    render_context_attributes.vsync=false;
    window_attributes.auto_redraw=false;
    event_loop_attributes.redraw_request_interval=EventInterval::EventsPerSecond(30);
    event_control=EventControl::Run;
```

#### The smoothest image
If you need updates and the lowest response time:
 - enable V-Sync,
 - set the `auto_redraw` flag to `true`,
 - set the frame rate lock to zero,
 - set the needed mode for `EventLoop`
```rust
    render_context_attributes.vsync=true;
    window_attributes.auto_redraw=true;
    event_loop_attributes.redraw_request_interval=EventInterval::EventsPerSecond(0);
    event_control=EventControl::Run;
```

# 1.0.0 - pre-alpha

### Whole engine redesign.
 - Raw OpenGL functions
 - Raw wrappers
 - New object allocation systems: heap- and stack-type
 - A new windowing system
 - A new app working system
 - A basement with secrets and cats
 - More unaccomplished features
 - More language mistakes :)
 - Less comments and docs


# 0.6.5

### CatAudio

Uncreased the stack size up to 1 MiB (1024 * 1024 bytes) due to problems on some versions of Linux.


# 0.6.4

Now you can draw on everything that implements the `Surface` trait.
```rust
let texture=Texture::empty([1920u32,1080u32],window.display()).unwrap();
let mut frame_buffer=SimpleFrameBuffer::new(window.display(),&texture.0).unwrap();

let mut frame_buffer_graphics=Graphics{
    graphics2d:graphics,
    draw_parameters:default_draw_parameters(),
    frame:&mut frame_buffer,
};

frame_buffer_graphics.clear_colour(White);

frame_buffer_graphics.draw_textured_object(cat).unwrap();

frame_buffer_graphics.draw_rotate_textured_object(gear,unsafe{window_center},angle).unwrap();
```

Made the `default_draw_parameters` function public.


# 0.6.3

Fixed setting the index buffer size of the simple graphics.

Added some functions to work with graphic objects and some was renamed.


# 0.6.2

### CatAudio

Now you can directly push `ChanneledTrack` to a wrapper.


# 0.6.1

Added an update per second counter.

Fixed an update rate for closures.

Updated the `ttf-parser` crate to `0.9.0` and the `ab_glyph_rasterizer` to `0.1.4`.


# 0.6.0

### Removed

A lifetime from the `DependentObject` trait.

`DynamicWindow` and the `Window` trait.

`WindowBase` from public view.

The `Window` field from the `WindowPage` trait.

### Function changes

The window `draw` function now takes `&self`, `Graphics2D` and a closure with `&mut Graphics` as an argument.
```rust
pub fn draw<F:FnOnce(&mut Graphics)>(&self,graphics_base:&Graphics2D,f:F)->Result<(),SwapBuffersError>;
```

The `on_redraw_requested` function of the `WindowPage` trait now takes `&Window`.
```rust
fn on_redraw_requested(&mut self,window:&Window);
```

Arguments of `ImageBase::new` was swapped:
```rust
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

```rust
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
```rust
window.graphics().set_simple_object_colour(0,White);
window.graphics().rewrite_simple_object_vertices(0,&[100f32;4])
```

### Removed

The `draw_raw` function of all the windows.

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
The `WindowPage::on_close_requested` function has been renamed to `WindowPage::on_window_close_requested`.

The range system for simple objects has been replaced with an object one.
Now you can add an object to the array and then quickly draw it or the whole array.

Some changes in the `SimpleObject` trait.

All the geometric shapes (`cat_engine::graphics::*;`) has been moved to `cat_engine::shapes::*`.


# Version 0.2.1

### Added

Update events. You can configure update frequency in the `WindowSettings` (`GeneralSettings`).

`WindowEvent::EventLoopClosed` and `WindowEvent::Update` events.

A `type Output` field and new functions (`on_update_requested`, `on_event_loop_closed->Output`) for the `WindowPage` trait. The `PagedWindow::run_page` function now returns the `Output`.

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
```rust
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