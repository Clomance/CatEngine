# Version 0.1.5

### User's mouse cursor icon (feature = "mouse_cursor_icon")

Now all the errors of user's mouse cursor icon panic with an appropriate message.

New settings for user's mouse cursor icon.

### Added

New window types are `DynamicWindow` - a windpw uses 'pages' as a trait object,
and `WindowBase` - a window base without event handlers.

A trait for windows. You can create

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