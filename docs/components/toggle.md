# Toggle

The `Toggle` layer enters the AccessKit tree as `Role::Button` with `aria_toggled` reflecting the pressed state (the AccessKit stand-in for `aria-pressed`; screen readers may phrase it as "toggled"/"checked").

[Base UI reference](https://base-ui.com/react/components/toggle) · [Source](../../src/toggle/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::toggle::{
    Toggle,
};

Toggle::new();
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `Toggle`

Public renderable part of the Toggle component.

[Source](../../src/toggle/layers/toggle.rs)

```rust
use base_gpui::toggle::Toggle;

Toggle::new()
    .id("example-id")
    .default_pressed(true)
    .pressed(None)
    .value(/* value: T */)
    .group_value()
    .own_disabled()
    .toggle_id()
    .with_toggle_group(/* context: ToggleGroupContext<T> */, 0, /* focus_handle: FocusHandle */)
    .disabled(true)
    .aria_label("aria label")
    .on_pressed_change(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a Toggle with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.default_pressed(...)`

```rust
pub fn default_pressed(mut self, default_pressed: bool) -> Self
```

**Accepts**

- `default_pressed`: `bool`

Sets the initial pressed for uncontrolled state.

#### `.pressed(...)`

```rust
pub fn pressed(mut self, pressed: Option<bool>) -> Self
```

**Accepts**

- `pressed`: `Option<bool>`

Sets the pressed configuration for this part.

#### `.value(...)`

```rust
pub fn value(mut self, value: T) -> Self
```

**Accepts**

- `value`: `T`

Sets the current controlled value or the value represented by this part, depending on the part's role.

#### `.group_value(...)`

```rust
pub fn group_value(&self) -> Option<&T>
```

The group-membership identity, consumed by the Toggle Group wiring. Has no standalone behavior.

#### `.own_disabled(...)`

```rust
pub fn own_disabled(&self) -> bool
```

The toggle's own disabled prop, consumed by the Toggle Group wiring when resolving the effective per-item disabled fact.

#### `.toggle_id(...)`

```rust
pub fn toggle_id(&self) -> &ElementId
```

The toggle's element id, consumed by the Toggle Group wiring to key the roving focus handle.

#### `.with_toggle_group(...)`

```rust
pub fn with_toggle_group(mut self, context: ToggleGroupContext<T>, index: usize, focus_handle: FocusHandle,) -> Self
```

**Accepts**

- `context`: `ToggleGroupContext<T>`
- `index`: `usize`
- `focus_handle`: `FocusHandle`

Attaches this toggle to a Toggle Group as a composite item. Called by the Toggle Group child wiring; not intended for direct use.

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

The accessible label announced by assistive technology. Icon-only toggles must set this; there is no `aria-labelledby` id-reference wiring in this gpui revision, so the literal string is the only labelling mechanism.

#### `.on_pressed_change(...)`

```rust
pub fn on_pressed_change(mut self, on_pressed_change: impl Fn(bool, &mut TogglePressedChangeDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_pressed_change`: `impl Fn(bool, &mut TogglePressedChangeDetails, &mut Window, &mut App)
            + 'static`

Registers a callback invoked when pressed change occurs.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ToggleStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ToggleStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

Toggle also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
