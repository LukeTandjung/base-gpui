# Menubar

Menubar ports Base UI's Menubar component model to GPUI.

[Base UI reference](https://base-ui.com/react/components/menubar) · [Source](../../src/menubar/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::menubar::{
    Menubar,
};

Menubar::new();
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `Menubar`

Public renderable part of the Menubar component.

[Source](../../src/menubar/layers/menubar.rs)

```rust
use base_gpui::menubar::Menubar;

Menubar::new()
    .id("example-id")
    .orientation(/* orientation: MenubarOrientation */)
    .loop_focus(true)
    .modal(true)
    .disabled(true)
    .aria_label("label")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a Menubar with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<MenubarChild>) -> Self
```

**Accepts**

- `child`: `impl Into<MenubarChild>`

Adds one typed child to this part.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.orientation(...)`

```rust
pub fn orientation(mut self, orientation: MenubarOrientation) -> Self
```

**Accepts**

- `orientation`: `MenubarOrientation`

Sets the component's horizontal or vertical orientation and corresponding keyboard behavior.

#### `.loop_focus(...)`

```rust
pub fn loop_focus(mut self, loop_focus: bool) -> Self
```

**Accepts**

- `loop_focus`: `bool`

Controls whether keyboard focus wraps from the last enabled item to the first and vice versa.

#### `.modal(...)`

```rust
pub fn modal(mut self, modal: bool) -> Self
```

**Accepts**

- `modal`: `bool`

Controls whether the overlay behaves modally and blocks interaction outside it.

#### `.disabled(...)`

```rust
pub fn disabled(mut self, disabled: bool) -> Self
```

**Accepts**

- `disabled`: `bool`

When true, prevents user interaction with this part.

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Accessible label for the menubar row; Base UI leaves labeling to the consumer, so this stays optional.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(MenubarStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(MenubarStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

Menubar also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
