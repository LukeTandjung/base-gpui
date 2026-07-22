# Separator

Separator ports Base UI's Separator component model to GPUI.

[Base UI reference](https://base-ui.com/react/components/separator) · [Source](../../src/separator/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::separator::{
    Separator,
};

Separator::new();
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `Separator`

Visual separator between neighboring items or groups.

[Source](../../src/separator/layers/separator.rs)

```rust
use base_gpui::separator::Separator;

Separator::new()
    .id("example-id")
    .orientation(/* orientation: SeparatorOrientation */)
    .horizontal()
    .vertical()
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a Separator with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Overrides the default `"separator"` element id. Give each separator in a window a distinct, stable id so assistive technology sees stable accessibility nodes across frames.

#### `.orientation(...)`

```rust
pub fn orientation(mut self, orientation: SeparatorOrientation) -> Self
```

**Accepts**

- `orientation`: `SeparatorOrientation`

Sets the component's horizontal or vertical orientation and corresponding keyboard behavior.

#### `.horizontal(...)`

```rust
pub fn horizontal(self) -> Self
```

Sets the horizontal configuration for this part.

#### `.vertical(...)`

```rust
pub fn vertical(self) -> Self
```

Sets the vertical configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(SeparatorStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(SeparatorStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

Separator also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
