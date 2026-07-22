# Checkbox

Checkbox ports Base UI's Checkbox component model to GPUI.

[Base UI reference](https://base-ui.com/react/components/checkbox) · [Source](../../src/checkbox/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::checkbox::{
    CheckboxIndicator,
    CheckboxRoot,
};

CheckboxRoot::new()
    .child(
        CheckboxIndicator::new(),
    );
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `CheckboxRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/checkbox/layers/checkbox_root.rs)

```rust
use base_gpui::checkbox::CheckboxRoot;

CheckboxRoot::new()
    .id("example-id")
    .name("name")
    .default_checked(true)
    .checked(None)
    .indeterminate(true)
    .value(/* value: impl Into<SharedString> */)
    .form(/* form: impl Into<SharedString> */)
    .parent(true)
    .unchecked_value(/* unchecked_value: impl Into<SharedString> */)
    .disabled(true)
    .read_only(true)
    .required(true)
    .on_checked_change(|/* callback arguments */| { /* handle change */ })
    .aria_label("aria label")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a CheckboxRoot with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<CheckboxChild>) -> Self
```

**Accepts**

- `child`: `impl Into<CheckboxChild>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<CheckboxChild>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<CheckboxChild>>`

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

#### `.default_checked(...)`

```rust
pub fn default_checked(mut self, default_checked: bool) -> Self
```

**Accepts**

- `default_checked`: `bool`

Sets the initial checked for uncontrolled state.

#### `.checked(...)`

```rust
pub fn checked(mut self, checked: Option<bool>) -> Self
```

**Accepts**

- `checked`: `Option<bool>`

Sets the checked configuration for this part.

#### `.indeterminate(...)`

```rust
pub fn indeterminate(mut self, indeterminate: bool) -> Self
```

**Accepts**

- `indeterminate`: `bool`

Sets the indeterminate configuration for this part.

#### `.value(...)`

```rust
pub fn value(mut self, value: impl Into<SharedString>) -> Self
```

**Accepts**

- `value`: `impl Into<SharedString>`

Sets the current controlled value or the value represented by this part, depending on the part's role.

#### `.form(...)`

```rust
pub fn form(mut self, form: impl Into<SharedString>) -> Self
```

**Accepts**

- `form`: `impl Into<SharedString>`

Sets the form configuration for this part.

#### `.parent(...)`

```rust
pub fn parent(mut self, parent: bool) -> Self
```

**Accepts**

- `parent`: `bool`

Sets the parent configuration for this part.

#### `.unchecked_value(...)`

```rust
pub fn unchecked_value(mut self, unchecked_value: impl Into<SharedString>) -> Self
```

**Accepts**

- `unchecked_value`: `impl Into<SharedString>`

Sets the unchecked value configuration for this part.

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

#### `.on_checked_change(...)`

```rust
pub fn on_checked_change(mut self, on_checked_change: impl Fn(bool, &mut crate::checkbox::CheckboxCheckedChangeDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_checked_change`: `impl Fn(bool, &mut crate::checkbox::CheckboxCheckedChangeDetails, &mut Window, &mut App)
            + 'static`

Registers a callback invoked when checked change occurs.

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self
```

**Accepts**

- `aria_label`: `impl Into<SharedString>`

Sets a literal accessible name for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(CheckboxRootStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(CheckboxRootStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

CheckboxRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `CheckboxIndicator`

Visual representation of the component's current state or value.

[Source](../../src/checkbox/layers/checkbox_indicator.rs)

```rust
use base_gpui::checkbox::CheckboxIndicator;

CheckboxIndicator::new()
    .keep_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a CheckboxIndicator with its default configuration.

#### `.keep_mounted(...)`

```rust
pub fn keep_mounted(mut self, keep_mounted: bool) -> Self
```

**Accepts**

- `keep_mounted`: `bool`

Keeps the part mounted when inactive or closed so child state can be preserved.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(CheckboxIndicatorStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(CheckboxIndicatorStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

CheckboxIndicator also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
