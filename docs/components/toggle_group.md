# Toggle Group

Toggle Group ports Base UI's Toggle Group component model to GPUI.

[Base UI reference](https://base-ui.com/react/components/toggle-group) · [Source](../../src/toggle_group/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::toggle_group::{
    ToggleGroup,
};

ToggleGroup::new();
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `ToggleGroup`

Groups related child parts and coordinates their shared behavior.

[Source](../../src/toggle_group/layers/toggle_group.rs)

```rust
use base_gpui::toggle_group::ToggleGroup;

ToggleGroup::new()
    .id("example-id")
    .aria_label("aria label")
    .default_value(/* default_value: Vec<T> */)
    .value(/* value: Vec<T> */)
    .disabled(true)
    .orientation(/* orientation: ToggleGroupOrientation */)
    .multiple(true)
    .loop_focus(true)
    .on_value_change(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ToggleGroup with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<ToggleGroupChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<ToggleGroupChild<T>>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<ToggleGroupChild<T>>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<ToggleGroupChild<T>>>`

Adds multiple typed children in iteration order.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self
```

**Accepts**

- `aria_label`: `impl Into<SharedString>`

Sets a literal accessible name for this part.

#### `.default_value(...)`

```rust
pub fn default_value(mut self, default_value: Vec<T>) -> Self
```

**Accepts**

- `default_value`: `Vec<T>`

Sets the initial value for uncontrolled state. Later user changes are retained by the component.

#### `.value(...)`

```rust
pub fn value(mut self, value: Vec<T>) -> Self
```

**Accepts**

- `value`: `Vec<T>`

Sets the current controlled value or the value represented by this part, depending on the part's role.

#### `.disabled(...)`

```rust
pub fn disabled(mut self, disabled: bool) -> Self
```

**Accepts**

- `disabled`: `bool`

When true, prevents user interaction with this part.

#### `.orientation(...)`

```rust
pub fn orientation(mut self, orientation: ToggleGroupOrientation) -> Self
```

**Accepts**

- `orientation`: `ToggleGroupOrientation`

Sets the component's horizontal or vertical orientation and corresponding keyboard behavior.

#### `.multiple(...)`

```rust
pub fn multiple(mut self, multiple: bool) -> Self
```

**Accepts**

- `multiple`: `bool`

Controls whether more than one value may be selected at the same time.

#### `.loop_focus(...)`

```rust
pub fn loop_focus(mut self, loop_focus: bool) -> Self
```

**Accepts**

- `loop_focus`: `bool`

Controls whether keyboard focus wraps from the last enabled item to the first and vice versa.

#### `.on_value_change(...)`

```rust
pub fn on_value_change(mut self, on_value_change: impl Fn(&[T], &mut ToggleGroupValueChangeDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_value_change`: `impl Fn(&[T], &mut ToggleGroupValueChangeDetails, &mut Window, &mut App)
            + 'static`

Registers a callback invoked when value change occurs.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ToggleGroupStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ToggleGroupStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ToggleGroup also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
