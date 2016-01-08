
# Oil - Design -

Oil is designed around the following components:

* `markup`
* `data-bindings`
* `style`
* `external dependencies`
* `action`

Every components has its own logic and can be considered separately from the rest.

## Markup

All your applications will start with the markup. In `oil`'s terminology, the markup is where
you will defines the `view`s and `template`s of your application. Think of it like the building blocks
your UI is made of. An `oil` user interface is made of multiples views that are shown independently.
Views have different context and you can have multiple at the same time.
Think of a `view` in the same way you would think about your windows on your screen.
Templates are a way to express re-usability. Let's say you create a nice UI component that you
would like to reuse in different places. But you don't want to copy the markup to each places.
You just want to refer to that chunk defined here. That's the problem solved by the `template`.

## Style

> TODO
