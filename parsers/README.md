
# Oil Language specification

When the project started, the language design
was following relatively closely the triplet
HTML / CSS / Resources (image, fonts, ...).

However, those things lacked fundamental language
needs like scopes, privacy and modules.

In the web world, if you consider web components
or limit yourself to a subset of features and
use a bit of JavaScript, you can re-add those
missing basic features.

As oil is new, we can provides those features while
still preserving a web tech look and feel in the
language. Especially because web dev are already
used to have external tools to fill that hole.

This specification describes the (now unified) oil
grammar as well as semantics of the language.

This specs is organized as follows:

* [Package definition](#pack-def)
* [Symbol resolution](#symbol-res)
* [Package lookup rule](#lookup-rule)
* [Template syntax](#template)
* [Classes](#class)
* [Model resolution](#model-event)
* [Supported image and fonts](#img-fonts)
* [Packaging your UI](#packaging)

## <a name="pack-def"></a> Package definition

A user interface in oil is defined by files
with the prefix `.oil` and will usually reside
close to a rust core application that will use
them:

```
.
├── Cargo.lock
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

Those files are only needed for development.
When publishing your application, they will
be packed together with all their dependencies
into a [binary format](#packaging).

They can then be be either embedded directly
into the executable or loaded from the disk
or even from the network.

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
follow the [lookup rule](#lookup-rule) defined below.

Here are some explanation for each example:

1. Import all symbols defined by the `oil-material` package.
 `(1a)` and `(1b)` are equivalent: they express the same.

2. Import only the `btn` symbol.

3. Import all symbols from the local package named `loading`

4. Images have a special convention. You can only import `$img`
 from an image. This symbol can only be used within a
 [class](#class) for the `background-image` property.

5. Similarly to images, only `$font` can be imported from fonts
 files. It can also only be assigned to `font-family`.
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

Besides, as cargo is used as a lookup point, you can
use it to publish and manage your UI components.

## <a name="template"></a> Template syntax

### Basics

Template are similar to web components. They embed a piece
of UI for easy re-use. This is very useful when you want
to control in one place the appearance of the buttons of
a menu for instance. This is also a fundamental component
for easily having a uniform look and feel in your game.

Template can be defined like this:

```js
// (1a)
template my-template = // ...
// (1b)
template my-template [] = // ...


// (2)
template my-template [arg1, arg2] = // ...

// (3a)
template my-template arg -> event // ...
// (3b)
template my-template [arg] -> (event) // ...

// (4)
template my-template [] -> (event1, event2) // ...
```

1. Definition of a template with no parameters.
`(1a)` and `(1b)` are equivalent.

2. Parentheses are used for argument. You can specify
as many as you want.

3. Template can also trigger events such as `close`, `play`, ...
Anything you want. They appear after the `->` syntax.
`(3a)` and `(3b)` are equivalent.

4. As you can have many parameters, you can trigger
many events.

### Parameters

A template parameter is either an input used or an event triggered.
When you use a template that declares both events and arguments in
its signature, you are free to ignore some or all events, but you
must provides values for all arguments.

If a template is declared to have parameters, you may wonder
how to pass those parameters to the template. This is pretty
easy actually:

```html
// Like this
<my-template [arg1]={..} [arg2]={..}/>
// We can pass constants for arguments:
<my-template [arg1]="john" [arg2]="doe"/>
```

In the above example, valid definitions for `my-template`
could be:

```js
template my-template [arg1, arg2] = // ...
template my-template [arg1, arg2] -> (event1) = // ...
template my-template [arg1, arg2] -> (event1, event2) = // ...
```

but not:

```js
template my-template [arg1, arg2, arg3] = // ...
```

All input for a template must be satisfied, otherwise you'll get an
error.

Events are binded a bit differently. They can't be binded to constants.
Only to a template parameter (argument or event) or to a view handlers
property:

```html
<my-template [arg1]={..} [arg2]={..} (event1)={..}/>
```

Additionally, all templates have a particular property called `class`
which is the only one where style class symbol can be used.

Parameters can be set to the following object types:

* Constants, typically string.
* Model property or model itself.
* Handler property.
* Classes

But can't be set to symbols that are:

* Views
* Template
* Images
* Fonts

#### Using parameters within the template

Parameters can be used to feed other template parameters
and also a few more other place such as:

* Text content:

```html
Hello {name} !
```

* As input to other parameters.

#### Template children

As you might expect, when using a template you can add
children to it:

```html
<my-template>
    Children here!
</my-template>
```

This will be rejected if `my-template` does not specify
where those children should go.

Here is how you do:

```jsx
template my-template = <select:children />;
```

This will insert within `my-template` all children within
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

### Global templates

Many templates are defined by oil and are internal to oil.
They are handling differently than the other templates.
Here is the list:

* `group`: Does nothing, just a valid name to group elements.
* `button`: Can receives focus and trigger event.
* `text-input`: Can receive user text input and update a model property.
* `check-box`: Can receive focus and update a logical model property.
* `progress-bar`: Show progress.
* `if`: A control template (see below)
* `for`: A control template (see below)
* `switch`: A control template (see below)

For more details see the
[global templates list](../docs/languages_reference/tag_list).

### More control with `if`, `for`, `switch`

Sometime, you want to control what template are displayed
when a specific condition is met. This is the role solved
by `if` and  `switch`.

> TODO: Show examples and explain behavior.

In order to display multiples elements, you can use the
`for` element.

> TODO: Explain behavior

## <a name="class"></a> Classes (like css ones, yes!)

Style! Here we go.

### Basics

Classes are defined by prefixing an ident with a `.` and followed
with a curly brace block. The following are all valid definition
of classes:

```css
.btn {}
.btn-danger {
    background-color: #CB0000
}
```

But that one is not:

```css
btn {}
```

When a class is applied to an element, you can

### Available properties

> TODO

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
view game-menu(model, handlers) =
    <game-ui>
        Hello {model.name}!
        <ui-button (click)={handlers.play}>Play</ui-button>
        <ui-button (click)={handlers.quit}>Quit</ui-button>
    </game-ui>
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


## <a name="img-fonts"></a> Supported images and fonts format

> TODO

## <a name="packaging"></a> Package your UI

> TODO: move it elsewhere

> Mention about the oil tool that run at build time.
