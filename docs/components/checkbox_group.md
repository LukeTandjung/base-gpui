# Checkbox Group

Checkbox Group.

[Base UI reference](https://base-ui.com/react/components/checkbox-group) · [Source](../../src/checkbox_group/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::checkbox_group::{
    CheckboxGroup,
};

CheckboxGroup::new();
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `CheckboxGroup`

Groups related child parts and coordinates their shared behavior.

[Source](../../src/checkbox_group/layers/checkbox_group.rs)

```rust
use base_gpui::checkbox_group::CheckboxGroup;

CheckboxGroup::new()
    .id("example-id")
    .default_value(/* values: impl IntoIterator<Item = impl Into<SharedString>> */)
    .value(/* values: impl IntoIterator<Item = impl Into<SharedString>> */)
    .all_values(/* values: impl IntoIterator<Item = impl Into<SharedString>> */)
    .disabled(true)
    .aria_label("aria label")
    .on_value_change(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a CheckboxGroup with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl IntoElement>`

Adds multiple typed children in iteration order.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.default_value(...)`

```rust
pub fn default_value(mut self, values: impl IntoIterator<Item = impl Into<SharedString>>,) -> Self
```

**Accepts**

- `values`: `impl IntoIterator<Item = impl Into<SharedString>>`

Sets the initial value for uncontrolled state. Later user changes are retained by the component.

#### `.value(...)`

```rust
pub fn value(mut self, values: impl IntoIterator<Item = impl Into<SharedString>>) -> Self
```

**Accepts**

- `values`: `impl IntoIterator<Item = impl Into<SharedString>>`

Sets the current controlled value or the value represented by this part, depending on the part's role.

#### `.all_values(...)`

```rust
pub fn all_values(mut self, values: impl IntoIterator<Item = impl Into<SharedString>>) -> Self
```

**Accepts**

- `values`: `impl IntoIterator<Item = impl Into<SharedString>>`

Sets the all values configuration for this part.

#### `.disabled(...)`

```rust
pub fn disabled(mut self, disabled: bool) -> Self
```

**Accepts**

- `disabled`: `bool`

When true, prevents user interaction with this part.

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self
```

**Accepts**

- `aria_label`: `impl Into<SharedString>`

Sets a literal accessible name for this part.

#### `.on_value_change(...)`

```rust
pub fn on_value_change(mut self, on_value_change: impl Fn(Vec<SharedString>, &mut CheckboxGroupValueChangeDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_value_change`: `impl Fn(Vec<SharedString>, &mut CheckboxGroupValueChangeDetails, &mut Window, &mut App)
            + 'static`

Registers a callback invoked when value change occurs.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(CheckboxGroupStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(CheckboxGroupStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

CheckboxGroup also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
