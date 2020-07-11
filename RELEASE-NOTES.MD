# Version 0.1.4

### Fixed

The closure event listeners works correct with the `WindowEvent::Resized` event.

For the `auto_hide` feature:
 when the current 'page' is closed and
 the current event listener is for the window minimized state 
 the next page will be opened with the same listener.

# Version 0.1.3

Now all drawing functions return the result of drawing.

Added the center of rotation for images.

Added lazy feature (see [features](FEATURES.MD)).