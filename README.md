# oil-rs [![Build Status](https://travis-ci.org/oil-lang/oil-rs.svg?branch=master)](https://travis-ci.org/oil-lang/oil-rs)

Oil is a graphical user interface library for Rust with video games in mind with
a unique language to describe the UI.

It is heavily inpired of JavaScript / HTML / CSS and modern frameworks used to
build UIs. Of course there's a non goal of redoing a web browser here.
That's not the point. We want to offer a small engine with similar capabilities
while retaining performance and convenience.

Oil's goals are completely different from the one of a web browser engine such as
[servo](https://github.com/servo/servo).

The key idea behind familiarity is the ease of learning while bringing *(trying)* the good part
from web development for game development with Rust. We think that if can bring
to UX and UI expert similar tooling for Games, we will be able to see much more
interesting UI in games and offer more opportunities for UI/UX expert.

Okay, now a few more things to keep in mind before getting started:

* The library is young and still in its early development stage. Don't expect speed yet.
* A video game in development is currently using Oil, leading the design decisions for Oil.
  It essentially means some feature might be set as lower/higher priority because of the main project.
* Contributions are welcomed !

## [Getting-started](http://oil-lang.github.io/#getting-started)

Oil now work on Rust stable (1.13 as of writing). You only need to add the following
to you `Cargo.toml` file:

```toml
[dependencies]
oil = "0.3.0"
```

For a concrete example, you should have a look at the examples in the `examples/` folder.

## Roadmap

This library does not allow to do many things right now. Here is a rough sketch
of the planned roadmap:

  * [ ] Language parser and stable AST.
  * [ ] Syntax highlighters for Monaco and Vim.
  * [ ] *Simple* Oil Language Server with Monaco plugin. *(It's okay if it does not fully work)*
  * [ ] Flexbox.
  * [ ] Transitions and animations keys.
  * [ ] Observables.
  * [ ] Rust codegen (Part 1) (only what's needed for observable and basic store)
  * [ ] Proper text support.
  * [ ] Text binding and property binding on the box model.
  * [ ] Full `Store` support.
  * [ ] Reducers.
  * [ ] Background image.
  * [ ] Fonts, font family and weight.
  * [ ] Arbitrary texture binding from store.
  * [ ] Custom shaders.
