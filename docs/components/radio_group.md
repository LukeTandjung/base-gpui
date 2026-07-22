# Radio Group

Radio Group ports Base UI's Radio Group component model to GPUI.

[Base UI reference](https://base-ui.com/react/components/radio-group) · [Source](../../src/radio_group/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::radio_group::{
    RadioGroupIndicator,
    RadioGroupRadio,
    RadioGroupRoot,
};

RadioGroupRoot::new()
    .child(
        RadioGroupRadio::new()
                .child(
                    RadioGroupIndicator::new(),
                ),
    );
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `RadioGroupRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/radio_group/layers/radio_group_root.rs)

```rust
use base_gpui::radio_group::RadioGroupRoot;

RadioGroupRoot::new()
    .id("example-id")
    .name("name")
    .form(/* form: impl Into<SharedString> */)
    .default_value(None)
    .value(None)
    .disabled(true)
    .read_only(true)
    .required(true)
    .aria_label("aria label")
    .on_value_change(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a RadioGroupRoot with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<RadioGroupChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<RadioGroupChild<T>>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<RadioGroupChild<T>>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<RadioGroupChild<T>>>`

Adds multiple typed children in iteration order.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.name(...)`

```rust
pub fn name(mut self, name: impl Into<SharedString>) -> Self
```

**Accepts**

- `name`: `impl Into<SharedString>`

Sets the form field name used when serializing or submitting the value.

#### `.form(...)`

```rust
pub fn form(mut self, form: impl Into<SharedString>) -> Self
```

**Accepts**

- `form`: `impl Into<SharedString>`

Sets the form configuration for this part.

#### `.default_value(...)`

```rust
pub fn default_value(mut self, default_value: Option<T>) -> Self
```

**Accepts**

- `default_value`: `Option<T>`

Sets the initial value for uncontrolled state. Later user changes are retained by the component.

#### `.value(...)`

```rust
pub fn value(mut self, value: Option<T>) -> Self
```

**Accepts**

- `value`: `Option<T>`

Sets the current controlled value or the value represented by this part, depending on the part's role.

#### `.disabled(...)`

```rust
pub fn disabled(mut self, disabled: bool) -> Self
```

**Accepts**

- `disabled`: `bool`

When true, prevents user interaction with this part.

#### `.read_only(...)`

```rust
pub fn read_only(mut self, read_only: bool) -> Self
```

**Accepts**

- `read_only`: `bool`

When true, allows presentation and focus behavior without permitting value changes.

#### `.required(...)`

```rust
pub fn required(mut self, required: bool) -> Self
```

**Accepts**

- `required`: `bool`

Marks the control as required for validation and accessibility state.

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self
```

**Accepts**

- `aria_label`: `impl Into<SharedString>`

Sets the accessible label announced by screen readers. This is the literal-string substitute for Base UI's `aria-labelledby` id wiring, which has no gpui builder. When set, render the group's visible label text with `Text::new_inaccessible(...)` instead of `text!(...)` so the label is not announced twice.

#### `.on_value_change(...)`

```rust
pub fn on_value_change(mut self, on_value_change: impl Fn(Option<&T>, &mut RadioGroupValueChangeDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_value_change`: `impl Fn(Option<&T>, &mut RadioGroupValueChangeDetails, &mut Window, &mut App)
            + 'static`

Registers a callback invoked when value change occurs.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(RadioGroupRootStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(RadioGroupRootStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

RadioGroupRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `RadioGroupIndicator`

Visual representation of the component's current state or value.

[Source](../../src/radio_group/layers/radio_group_indicator.rs)

```rust
use base_gpui::radio_group::RadioGroupIndicator;

RadioGroupIndicator::new()
    .keep_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a RadioGroupIndicator with its default configuration.

#### `.keep_mounted(...)`

```rust
pub fn keep_mounted(mut self, keep_mounted: bool) -> Self
```

**Accepts**

- `keep_mounted`: `bool`

Keeps the part mounted when inactive or closed so child state can be preserved.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(RadioGroupIndicatorStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(RadioGroupIndicatorStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

RadioGroupIndicator also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `RadioGroupRadio`

Public renderable part of the Radio Group Radio component.

[Source](../../src/radio_group/layers/radio_group_radio.rs)

```rust
use base_gpui::radio_group::RadioGroupRadio;

RadioGroupRadio::new()
    .id("example-id")
    .value(/* value: T */)
    .disabled(true)
    .read_only(true)
    .required(true)
    .aria_label("aria label")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a RadioGroupRadio with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<RadioGroupRadioChild>) -> Self
```

**Accepts**

- `child`: `impl Into<RadioGroupRadioChild>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<RadioGroupRadioChild>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<RadioGroupRadioChild>>`

Adds multiple typed children in iteration order.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.value(...)`

```rust
pub fn value(mut self, value: T) -> Self
```

**Accepts**

- `value`: `T`

Sets the current controlled value or the value represented by this part, depending on the part's role.

#### `.disabled(...)`

```rust
pub fn disabled(mut self, disabled: bool) -> Self
```

**Accepts**

- `disabled`: `bool`

When true, prevents user interaction with this part.

#### `.read_only(...)`

```rust
pub fn read_only(mut self, read_only: bool) -> Self
```

**Accepts**

- `read_only`: `bool`

When true, allows presentation and focus behavior without permitting value changes.

#### `.required(...)`

```rust
pub fn required(mut self, required: bool) -> Self
```

**Accepts**

- `required`: `bool`

Marks the control as required for validation and accessibility state.

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self
```

**Accepts**

- `aria_label`: `impl Into<SharedString>`

Sets the accessible label announced by screen readers. This is the literal-string substitute for Base UI's `aria-labelledby` id wiring, which has no gpui builder. When set, render the radio's visible label text with `Text::new_inaccessible(...)` instead of `text!(...)` so the label is not announced twice.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(RadioGroupRadioStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(RadioGroupRadioStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

RadioGroupRadio also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
