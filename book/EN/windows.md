# DefaultWindow

All events are handled and added to the outer handling queue (Window.events)
to work with them outside of the window structure.

# PagedWindow

### Working with 'pages'

All the events are implemented with `WindowPage` trait
and handled immediately after emited.

### Working with closures

All the events are handled with a closure.