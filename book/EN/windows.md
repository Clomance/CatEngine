# General

Windows supports the next events:
 - requested to close the window
 - changed the size or the position of the window
 - updates (only with `feature != "lazy"`)
 - requested to redraw the window
 - the window has been suspended or resumed
 - the event loop has been stopped (to close a 'page' for `PagedWindow` и `DynamicWindow`)
 - the window gained or lost focus
 - keyboard and mouse events
 - modifiers has been changed (Shift,Ctrl,Alt,Logo)
 - a file is being hovered (only with `feature = "file_drop"`)

# WindowBase

A window base with graphic functions included.

You can create you own window with it.

It also includes almost all the features.

# DefaultWindow

This window has wide range of features, but is the most slowly one.

All events are handled and added to the outer handling queue (Window.events)
to work with them outside of the window structure.


# PagedWindow

### Working with 'pages'

All the events are implemented with `WindowPage` trait
and handled immediately after emited.


### Working with closures

All the events are handled with a closure.


# DynamicWindow

The fastest window, but very limited.

All the events are implemented with `WindowPage` trait
and handled immediately after emited.

The window that uses 'pages' as `WindowPage` trait objects,
so you can change one while running another.