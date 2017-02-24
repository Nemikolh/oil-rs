
# Oil Language specification

## Introduction

* [Events](#events)

* [Package definition](#pack-def)
* [Symbol resolution](#symbol-res)
* [Package lookup rule](#lookup-rule)
* [Component syntax](#component)
* [Classes](#class)
* [Model resolution](#model-event)
* [Supported image and fonts](#img-fonts)
* [Packaging your UI](#packaging)

## <a name="events"></a> Events

Everything is a stream of events, or an `Observable`. We have access to many
operators that allows transformations on the stream of events.

Here is how we can declare complex component that use observables:

```
component some_component[] -> (event1) {
    obs playgame =
    return (
        <button click={} ></button>
    )
}
```
