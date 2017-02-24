## New ideas and changes (in no particular order)

### General:

* Keep the idea of Ambient model but call it `Store`.
* Rename any mention of template (those are now called components)
* The **Store** is the single source of truth. Make sure this is true.
* `oilc` fetch the store from the Rust code. We need to figure out how
  to do this. The easier might be to parse the Rust project next to
  the oil code. Probably with an option to point to the Rust code location.
* One problem in React was the fact that rendering could have side effects.
  Because the language use declarative rendering, side effects could only be
  in the `oil` library. So we should never have this problem.
* Have something similar to observables to do the processing

### Technical:

* We have two compile targets:
    - Rust (the only one at the beginning)
    - Interpreter mode (compiler is embedded in the executable with an Interpreter).
* Resources are always loaded asynchronously from the ui point of view. When they
  are used they can be "present" or not yet loaded. This information needs to be
  obtainable somehow for load screen, etc...
* We want to offer to the Rust programmer a list of resources likely to
  be needed at a current point in time. Also provide a list of resources
  that are no longer needed (but should be managed by the developper).
  (On that last part we could provide a gc method that drops unused resources).

### Core design ideas:

* To make component reusable we need to have a clear typing (duck typed, inferred?)

### Tooling:

* Graph that show the possible views we can go in from the current view we have.
* A debugger that shows the different ui that the user has navigated and the timing
  that it tooks to load the ui and so on.