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