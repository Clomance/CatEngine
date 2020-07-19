# General



# WindowBase

A window base with graphic functions included.

You can create you own window with it.

# DefaultWindow

All events are handled and added to the outer handling queue (Window.events)
to work with them outside of the window structure.


# PagedWindow

### Working with 'pages'

All the events are implemented with `WindowPage` trait
and handled immediately after emited.

### Working with closures

All the events are handled with a closure.


# DynamicWindow

All the events are implemented with `WindowPage` trait
and handled immediately after emited.

The window that uses 'pages' as `WindowPage` trait objects,
so you can change one while running another.