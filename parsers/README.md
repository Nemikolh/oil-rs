
# Oil Language specification

When the project started, the language design
was following relatively closely the triplet
HTML / CSS / Resources (image, fonts, ...).

However, those things lacked fundamental language
needs like scopes, privacy and modules.

In the web world, if you consider web components
or limit yourself to a subset of features and
use a bit of JavaScript, you can re-add those
missing features.

As oil is new we can provide those features from
the start. If you have used React or Angular, you will
probably find yourself at home after reading this
specification.

This specification describes the (now unified) oil
grammar as well as semantics of the language.

This specs is organized as follows:

* [Package definition](#pack-def)
* [Symbol resolution](#symbol-res)
* [Package lookup rule](#lookup-rule)
* [Component syntax](#component)
* [Classes](#class)
* [Model resolution](#model-event)
* [Supported image and fonts](#img-fonts)
* [Packaging your UI](#packaging)

## <a name="pack-def"></a> Package definition

A user interface in oil is defined by files
with the prefix `.oil` and will usually reside
close to a rust application that will use them
during the build step. Here is an example:

```
.
├── Cargo.toml
├── ui
│   ├── main.oil
│   ├── loading.oil
│   └── resources
│       ├── logo.png
│       ├── font.otf
│       ├── font-bold.otf
│       ├── font-light.otf
│       └── button.png
└── src
    ├── main.rs
    └── oil_model.rs
```

A package is simply a file ending with the `.oil`
extension. But we might also refer as an oil library
as it expose only one package called `lib.oil`.

If you are coming from Rust, you can think of a package
as you think of a crate.

If you are coming from JavaScript / NodeJS, think of it
as the file referenced by the `main` field in `package.json`.

> Note:
>   Those files are only needed for development.
>   When publishing your application, they will
>   be packed together with all their dependencies
>   into a [binary format](#packaging).

## <a name="symbol-res"></a> Symbol resolution

In oil, symbols are either imported with the `import`
keyword or defined in the current file.

An import statement can look like the followings:

```js
// (1a)
import * from 'oil-material';
// (1b)
import 'oil-material';
// (2)
import {btn} from 'oil-material';

// (3)
import './loading';
// (4)
import $img from './resources/logo.png';
// (5a)
import $font from {
    normal: './resources/font.otf',
    bold: './resources/font-bold.otf'
}
// (5b)
import $font from './resources/font.otf';
```

The path are resolved relatively to the file importing
them when they start with `./` or `../` otherwise, they
follow the [lookup rule](#lookup-rule).

Here are some explanation for each example:

1. Import all symbols defined by the `oil-material` package.
 `(1a)` and `(1b)` are equivalent: they express the same.

2. Import only the `btn` symbol.

3. Import all symbols from the local package named `loading`

4. Images have a special convention. You can only import `$img`
 from an image. This symbol can only be used within a
 [class](#class) for the `background_image` property.

5. Similarly to images, only `$font` can be imported from fonts
 files. It can also only be assigned to `font_family`.
 `(5a)` and `(5b)` are equivalent. In `(5b)`, the `bold`
 property is created, if the file is present, from a file
 that follow the naming convention `<name>-bold.<ext>` where `<name>`
 is the the original font name and ext its extension.
 This last approach is more automated as the `bold` property
 receive the same as `normal` in case the file couldn't be
 found.

## <a name="lookup-rule"></a> Package lookup rule

Imported packages that do not start with `./` or `../`
are resolved using the following rule where each step
is tried if the previous failed:

1. If the package name is defined by the game, use it to
 resolve the imported symbols.

2. Look at the cargo dependencies for a crate of that name
 and resolve the imported symbols against the list of
 exported symbols of `<crate-name>/ui/lib.oil`.

3. Package couldn't be found! =)

Thanks to this rule, you always have full control on what
is imported and if you need to override a package you can
easily do so by creating your own package that overrides
or replace a few symbol from a dependency and without changing
the main bit, you can have an entirely new look and feel for
your user interface. =)

Cargo is the *de facto* packet manager for oil.

## <a name="component"></a> Component syntax

### Basics

Component are similar to web components. They embed a piece
of UI for easy re-use. A component defines a piece of UI
structure and the layout ordering for its children. Like in
HTML, this is a tree of components.

Component are defined like this:

```js
// (1a)
component my_component = // ...
// (1b)
component my_component [] = // ...


// (2)
component my_component [arg1, arg2] = // ...

// (3a)
component my_component arg -> event // ...
// (3b)
component my_component [arg] -> (event) // ...

// (4)
component my_component [] -> (event1, event2) // ...
```

1. Definition of a component with no parameters. Examples
`(1a)` and `(1b)` are equivalent.

2. Parentheses are used for argument. You can specify
as many as you want.

3. Component can also trigger events such as `close`, `play`, ...
Anything you want. They appear after the `->` syntax. Examples
`(3a)` and `(3b)` are equivalent.

4. As you can have many parameters, you can trigger
many events.

### Parameters

A component parameter is either an argument (input) or an event (output).
When you use a component that declares both events and arguments in
its signature, you are free to ignore some or all events, but you
must provides values for all arguments.

If a component is declared to have parameters, you may wonder
how to pass those parameters to the component. This is pretty
easy actually:

```html
// Like this
<my_component [arg1]={..} [arg2]={..}/>
// We can pass constants for arguments:
<my_component [arg1]="john" [arg2]="doe"/>
```

In the above example, valid definitions for `my_component`
could be:

```js
component my_component [arg1, arg2] = // ...
component my_component [arg1, arg2] -> (event1) = // ...
component my_component [arg1, arg2] -> (event1, event2) = // ...
```

but not:

```js
component my_component [arg1, arg2, arg3] = // ...
```

All input for a component must be satisfied, otherwise you'll get an
error.

Events are binded a bit differently. They can't be binded to constants.
Only to a component parameter (argument or event) or to a view handlers
property:

```html
<my_component [arg1]={..} [arg2]={..} (event1)={..}/>
```

Additionally, all components have a particular property called `class`
which is the only one where style class symbol can be used.

Parameters can be set to the following object types:

* Constants, typically string.
* Model property or model itself.
* Private state object. *(see more about this, [here](#internal-state))*
* Handler property.
* Classes

But can't be set to symbols that are:

* Views
* Component
* Images
* Fonts

#### Using parameters within the component

Parameters can be used to feed other component parameters
and also a few more other place such as:

* Text content:

```html
Hello {name} !
```

* As input to other parameters.

#### Component children

As you might expect, when using a component you can add
children to it:

```html
<my_component>
    Children here!
</my_component>
```

This will be rejected if `my_component` does not specify
where those children should go.

Here is how you do:

```jsx
component my_component = <select:children />;
```

This will insert within `my_component` all children within
it.

We plan to support the following kind of insertion:

| `<select:? />` | Effect                    |
|----------------|---------------------------|
| `children`     | Insert all children       |
| `first`        | Insert the first children |
| `last`         | Insert the last children  |


> Note: Only `children` is supported for now. We need more
>       experience before integrating other selection.


> Question: Do we want to allow "queries" on the type of those
>           children? Like filtering children that are TextNode, etc..

### Global components

Many components are defined by oil and are internal to oil.
They are handling differently than the other components.
Here is the list:

* `group`: Does nothing, just a valid name to group elements.
* `button`: Can receives focus and trigger event.
* `text_input`: Can receive user text input and update a model property.
* `check_box`: Can receive focus and update a logical model property.
* `progress_bar`: Show progress.
* `if`: A control component (see below)
* `for`: A control component (see below)
* `switch`: A control component (see below)

For more details see the
[global components list](../docs/languages_reference/tag_list).

### More control with `if`, `for`, `switch`

Sometime, you want to control what component are displayed
when a specific condition is met. This is the role solved
by `if` and  `switch`.

> TODO: Show examples and explain behavior.

In order to display multiples elements, you can use the
`for` element.

> TODO: Explain behavior

## <a name="class"></a> Classes (like css ones, yes!)

Style! Here we go.

Classes are the basic for style re-use.This is very
useful when you want to control in one place the
appearance of the buttons of a menu for instance.

### Basics

Classes are defined by prefixing an ident with a `.` and followed
with a curly brace block. The following definition is correct:

```css
.btn {}
```

But that one is not:

```css
btn {}
```

A class can contains multiples properties. They are defined like this:

```css
.btn {
    <property> : <value>;
}
```

The list of valid properties is described in the following table:

| Property name           | Accepted values              |
| ----------------------- | ---------------------------- |
| `left`                  | Length                       |
| `right`                 | Length                       |
| `top`                   | Length                       |
| `bottom`                | Length                       |
| `height`                | Length                       |
| `width`                 | Length, `"auto"`, `"expand"` |
| `margin`                | Length, `"auto"`, `"expand"` |
| `margin_left`           | Length, `"auto"`, `"expand"` |
| `margin_right`          | Length, `"auto"`, `"expand"` |
| `margin_top`            | Length, `"auto"`             |
| `margin_bottom`         | Length, `"auto"`             |
| `padding`               | Length                       |
| `padding_left`          | Length                       |
| `padding_right`         | Length                       |
| `padding_top`           | Length                       |
| `padding_bottom`        | Length                       |
| `border`                | Length                       |
| `border_left`           | Length                       |
| `border_right`          | Length                       |
| `border_top`            | Length                       |
| `border_bottom`         | Length                       |
| `font_size`             | Length                       |
| `opacity`               | Unit-less value in [0, 1]    |
| `visible`               | Boolean                      |
| `focus`                 | `"accept"`, `"ignore"`       |
| `font_color`            | Color                        |
| `background_color`      | Color                        |
| `font`                  | Font object                  |
| `background_image`      | Ident                        |
| `background_image_rule` | *TBD*                        |
| `layout`                | *TBD*                        |

A length can be either an expression or a value and should be followed
with a unit. Alternatively some special keywords are possible as values
such as `"auto"` or `"expand"`. They shouldn't be followed by a value.

To apply a class on an element, you use the `class` attribute of the
element like this:

```html
<el class=my_class></el>
```

### Advanced use

So far, classes looks pretty limited as they mainly express static
visual constrains on the style. So how can we express more complex
visual?

First, classes can also be parameterize like component. Here is an
example:

```css
.btn(color) {
    font_color: color;
}
```

It is also possible to conditionally include a property using `if`.
The following is a valid class:

```css
.main_menu(game) {
    width: game.window.width px if game.window.width <= 300;
    width: 300px                if game.window.width  > 300;
    height: game.window.height
}
```

Now, you can see that we can express much more complex layout without
much pain.

Also, you can include other classes within your class. For instance:

```css
.main_menu_dark_theme(game) {
    .main_menu(game);
    background_color: #FFFFFF if game.dark_theme;
}
```
Element can also declare anonymous class:

## <a name="model-event"></a> Model and event handling

### Game properties

So all of this looks good, but how can we display
game content? o far, this doesn't seems possible.

This is the right place to answer that question!

Oil use reactive programming idea to perform two-ways
bindings between the value and changes that the UI would
performs on that value.

In order to access model values, you need first to create
a `view`. View are the only piece that can actually
be rendered and that also receives some data. Everything
that you have seen so far are *just* tools to make View
creation and management easier.

Here is an example of how view looks like:

```js
view game_menu(model, handlers) =
    <game_ui>
        Hello {model.name}!
        <ui_button (click)={handlers.play}>Play</ui_button>
        <ui_button (click)={handlers.quit}>Quit</ui_button>
    </game_ui>
;
```

The `model` name is like the `this` from JavaScript
or the `self` of Rust (except that like Rust it is
explicitly defined). It expose access to the model
data.

Property of the model are naturally accessed with a
dot syntax:

```js
model.settings.window.height
model.settings.window.width
```

Last but not least, `view` are always defined as:

```js
view <view-name>(<model-name>, <handlers-name>) =
    // ...
;
```

where `view-name` is the name for the view, `model-name`
can be any name to resolve model properties (typically
"model") and finally `handlers-name` is the name for
the handlers (typically "handlers").

### Event handling

As you may have noticed above, not only a `view` is given
a model with data that can be read and modified, it also
have handlers.

Handlers allow you to bind event from the ui to actual
code executed by your game engine. It fills the gap
between UI and the game engine.

> Note:
>     The only way for the engine to conveys information
>     to the UI is through the model.
>     On the other hand, the UI can both modify the model
>     to talk to the engine and use the event system.

### <a name="internal-state"></a> Internal state

Quite often, part of the UI have some complex state that is not
part of the model and does not make sense to be stored there.

A typical example would be formating a value into a percentage or
more generally performing a basic mathematical operation.

But it can also be doing some renaming to match a component
requirements.

Additionally when designing a UI, it is quite useful to have fake
data at hand. It comes very handy when designing a piece of UI.
However, if this data is immutable, limited or just very different
from Game data it will prove being less useful.

So `oil` support creating types with default property and associated values.
From the point of view of the UI designer, they behave exactly like the game
ones and have no differences. They can store state and evolve over time.

The main difference is that they are totally hidden from the Game engine.
And this is **good**. It is the private part of the UI that the engine shouldn't
care of. It is by design and considered as a feature.

So how do we declare a type? Here's how:

```
datatype MyType = {

};
```

## <a name="img-fonts"></a> Supported images and fonts format

> TODO

## <a name="packaging"></a> Package your UI

> TODO: move it elsewhere

> Mention about the oil tool that run at build time.

They can then be be either embedded directly
into the executable or loaded from the disk
or even from the network.
