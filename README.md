# Spair

[![Crates.io](https://img.shields.io/crates/v/spair)](https://crates.io/crates/spair)
[![docs.rs](https://img.shields.io/docsrs/spair)](https://docs.rs/spair)
![Build](https://github.com/aclueless/spair/workflows/Rust/badge.svg)

An incremental and fine-grained render frontend framework for **S**ingle **P**age **A**pplication **i**n **R**ust.

This project is in its *early stage*, things are still missing and *frequent breaking changes are expected*.

## Features

* Incremtental render
    * The render method create the DOM tree on the first run, then update it on subsequence runs.
* Queue render for fine-grained render.
    * Must be enabled in Cargo.toml, like: `spair = { version="x.y.z", features = ["queue-render"] }`
    * (It is just a queue render, not using any kind of signals)
    * You have to wrap your values in `spair::QrVal` or `spair::QrVec`
    * Current version of queue render may not very efficient because each fine-grained-render need to borrow the component state separately by it own.
* Component state can be accessed in every part of render code.
    * Spair's components tend to be big, [`Render`] is used for code splitting.
    * You can access the component state in every [`Render`] without having to pass it around.
    * Component state can also be accessed from queue render, too.
* (Almost) no macro is required for constructing DOM.
    * But Spair is quite verbose because of this.
    * (But a macro can be built on top of Spair).
    * `spair::set_arm!()` is the only macro you have to use, it saves your days, (without it, `match_if` is a nightmare to maintainers) 
* Routing
    * Just basic support, currently, you have to implement the routing logic by yourself.
* fetch command
    * JSON, RON
* async command
    * Can be used for fetching instead of the built-in fetch command.
* svg
* Missing things here and there...
    * Errr, this is not a feature, obviously. I just put this here to remind potential users not to surprise about missing things :D.
    * For example, Spair currently just implements a handful number of events. 

### Not support (yet), do you want to contribute?
These features are extremely low (or even not) on my todo list.

* `#[derive(spair::Routes)]`
* Event delegation, under `features = ["event-delegation"]` if it is ever implemented.
* SSR (under a feature flag, too)
* RSX macro (under a feature flag, too)

## Cargo features
You can enabled a feature in your Cargo.toml like this:
`spair = { version="x.y.z", features = ["feature-name"] }`

| feature-name       | desciption                   |
| ------------------ | ---------------------------- |
|`keyed-list`        | Support `keyed-list` for incremental mode |
|`fetch-json`        | Fetch and parse as a JSON |
|`fetch-ron`         | Fetch and parse as a RON  |
|`svg`               | Support svg element       |
|`queue-render`      | Support fined-grained render (*)|

(*) Lists render by queue-render are always keyed.

## Run examples

Prerequisites:

* [Rust] with `wasm32-unknown-unknown` target.
* [Trunk] 

In an example folder:

    trunk serve

or, if it's slow, use: (especialy `examples/boids` or `examples/game_of_life`)

    trunk serve --release

Open your browser at http://localhost:8080

## Documentation

Not yet. `/examples/*` is the best place to start now.

Sections below provide first looks into Spair.

## Static-mode and update-mode

Spair works by iterating through every elements and attributes/properties in
the current DOM, which is empty before the first render, creating new items
or modifying existing items, it's the update-mode. But there are elements or
attributes that will never change. You can tell Spair to just create them but
ignore them when iterating over them later by turn on the static-mode.

| items                    | update-mode                  | static-mode            | notes                                                                      |
| ------------------------ | ---------------------------- | ---------------------- | -------------------------------------------------------------------------- |
| attributes / properties  | *default*                    | `.static_attributes()` | call `.static_attributes()` after you are done with update-mode-attributes |
| elements                 | *default*, `.update_nodes()` | `.static_nodes()`      | only apply to elements (include `.relement()`), *not* apply to texts/renderable-items|
| texts / renderable-items | `.rupdate(value)`            | `.rstatic(value)`      | not affected by mode introduced by `.update_nodes()` or `.static_nodes()`  |

* `.update_nodes()` and `.static_nodes()` can be switched back and forth as
many times as you want.
* Again, please rememeber that `.relement()` is affected by `.update_nodes()` and `.static_nodes()`.

```rust
element
    // default to update-mode attributes
    .value(&some_value) // will be checked and updated if changed
    .class_if("class-name", bool_value)
    .static_attributes() // we are done with update-mode attributes!
    .class("class-name") // class="class-name" is added on creation, but ignored on subsequence renders
    // just add child-elements, default to update mode.
    .p(|p| {}) // create and update a <p>
    .rupdate(value) // create and update a text
    .rstatic(value) // a create-only text - not affected by update-mode (default).
    .static_nodes()
    .div(|d| {}) // a create-only <div> (because creating in static-mode)
    .rupdate(value) // an updatable text - not affected by `.static_nodes()`
    .rstatic(value) // a create-only text - because of `rstatic`, not cause by `static_nodes`

```
* **Important note**: when an element is creating in static mode, all its
content will be ignored (not update) after the first render.

```rust
element
    .static_nodes() // Elements append after this will be in static-mode
    .p(|p| {
        // This closure only execute once on the creation of <p>.
        // In the future update, this closure will be IGNORED,
        // therefore, all child-nodes of <p> will NOT be updated despite
        // being created in update-mode.
        p.span(|s| {})
            .rupdate(value); // NEW VALUE OF `value` WILL NEVER BE RENDERED.
    });
```

## Example
*Look in `/examples` for full examples*

This is the `render` method of `examples/counter`:
```rust
impl spair::Component for State {
    type Routes = ();
    fn render(&self, element: spair::Element<Self>) {
        let comp = element.comp();
        element
            .static_nodes()
            .p(|p| {
                p.static_nodes()
                    .rstatic("The initial value is ")
                    .rstatic(self.value);
            })
            .rstatic(Button("-", comp.handler(State::decrement)))
            .rupdate(self.value)
            .rstatic(Button("+", comp.handler(State::increment)));
    }
}
```

## `Render` and `StaticRender` traits

You can split your codes into small pieces by implement [`Render`] or
[`StaticRender`] on your data types and pass the values to `.rupdate()`
or `.rstatic()` respectively.

[`Render`] and [`StaticRender`] are implemented for primitives (`i8`,
..., `u64`, `f32`, `f64`, `bool`, `usize`, `isize`). They are simply
converted to strings and rendered as text nodes.

## Access to the component state.

When implementing [`Render`], [`StaticRender`] or [`ListItemRender`] for
your data types, you may want to access the state of your component:

```rust
impl spair::Render<State> for &YourType {
    fn render(self, nodes: spair::Nodes<State>) {
        let state = nodes.state(); // type of `state` is `&State`

        nodes.rupdate(state.value);
    }
}
```

## Reconciliation? - No, you must use [`.match_if()`]

Spair does not do reconciliation, users must do it by themselves. When an
expected element is not found, Spair create it, but if Spair found an
element at the expected index, Spair just assume it is the expected element.
Therefore, when you want to render different elements base on a condition,
you must tell Spair to do that via [`.match_if()`].

The following code is extracted from `examples/fetch/src/lib.rs`:
```rust
element
    .match_if(|mi| match self.branch.as_ref() {
        Some(branch) => spair::set_arm!(mi) // `spair::set_arm!()` use `line!()` internally to set `render_on_arm_index()`
            // Render the content of `Some(branch)`
            .rupdate(branch)
            // some code removed
            .done(),
        None => spair::set_arm!(mi)
            // There is no value: `None`? Then just render a button
            .button(|b| {/* some code removed */})
            .done(),
    })
```

**DON'T DO THIS, IT DOES NOT WORK**
```rust
if some_condition {
    element.div(|d| {})
} else {
    element.p(|p| {})
}
```
## Child components

Example: `examples/components`

## Notes

### Names conflict with Rust keywords
HTML's tags and attributes are implemented as methods in Spair. Names that
are conflicted with Rust's keywords are implemented using raw identifers
such as `r#type`, `r#for`...

### Element and attribute/property with the same name.
There are elements named `<span>`, `<label>`... there are also attributes
named `span`, `label`... and Spair implement all of them as methods. It's
obviously not able to implement them on the same object. (Actually, Spair
use traits for these, but conflicts are still there).

Therefore, to immediately add elements witch such names, call
`.update_nodes()` or `.static_nodes()`.

To set attributes/properties with such names, you have to call
`.attributes_only()` or `.static_attributes_only()` first. After setting
attributes/properties, you have to explicitly switch to nodes-mode using
`.update_nodes()` or `.static_nodes()`.

## Common errors
Using Spair, you may encounter common mistakes listed in this section.
They are really annoying. How these problems can be avoided?
### `static_attributes()`, `static_nodes()`
If you set attributes or add nodes in static-mode it will never be updated. It is
easy to misplace an update-mode item under static-mode. For example, you have
an app and have already converted all things that are you considered static to
static-mode. Now, after a while, You decide to add something that you want it
to be updated on change. But you placed it under a branch of the DOM tree without
noticing that the branch is under static-mode. Finally, you give the new version
of the app a test, at first, you may scratch head and check back and forth many
times because it is renderd, but never update its value.

[Rust]: https://www.rust-lang.org/
[Trunk]: https://trunkrs.dev/

[`Render`]: https://docs.rs/spair/latest/spair/trait.Render.html
[`StaticRender`]: https://docs.rs/spair/latest/spair/trait.StaticRender.html
[`ElementRender`]: https://docs.rs/spair/latest/spair/trait.ElementRender.html
[`.match_if()`]: https://docs.rs/spair/latest/spair/render/html/nodes/trait.HemsHandMade.html#method.match_if


