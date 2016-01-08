# Actions

```rust
let mut router: oil::Router = ...;

router.handle_event(|| {

});

for event in display.poll_events() {
  //
  match router.inject_event(event) {
    oil::InjectionResult::Consumed => (),
    oil::InjectionResult::Ignored(event) => {
      // do something with ignore event.
    }
  }
}
```
