# Button

Button ports Base UI's Button component model to GPUI.

[Base UI reference](https://base-ui.com/react/components/button) · [Source](../../src/button/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::button::{
    ButtonRoot,
};

ButtonRoot::new();
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `ButtonRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/button/layers/button_root.rs)

```rust
use base_gpui::button::ButtonRoot;

ButtonRoot::new()
    .id("example-id")
    .disabled(true)
    .focusable_when_disabled(true)
    .aria_label("label")
    .on_click(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ButtonRoot with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.disabled(...)`

```rust
pub fn disabled(mut self, disabled: bool) -> Self
```

**Accepts**

- `disabled`: `bool`

When true, prevents user interaction with this part.

#### `.focusable_when_disabled(...)`

```rust
pub fn focusable_when_disabled(mut self, focusable_when_disabled: bool) -> Self
```

**Accepts**

- `focusable_when_disabled`: `bool`

Sets the focusable when disabled configuration for this part.

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Accessible name for the button. Required for icon-only buttons; when set alongside a visible text child, render that child with `Text::new_inaccessible(...)` to avoid double-announcing.

#### `.on_click(...)`

```rust
pub fn on_click(mut self, on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_click`: `impl Fn(&ClickEvent, &mut Window, &mut App) + 'static`

Registers a callback invoked when click occurs.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ButtonRootStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ButtonRootStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ButtonRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
