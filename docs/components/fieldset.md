# Fieldset

Fieldset ports Base UI's Fieldset component model to GPUI.

[Base UI reference](https://base-ui.com/react/components/fieldset) · [Source](../../src/fieldset/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::fieldset::{
    FieldsetLegend,
    FieldsetRoot,
};

FieldsetRoot::new()
    .child(
        FieldsetLegend::new(),
    );
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `FieldsetRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/fieldset/layers/fieldset_root.rs)

```rust
use base_gpui::fieldset::FieldsetRoot;

FieldsetRoot::new()
    .id("example-id")
    .aria_label("label")
    .disabled(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a FieldsetRoot with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<FieldsetChild>) -> Self
```

**Accepts**

- `child`: `impl Into<FieldsetChild>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<FieldsetChild>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<FieldsetChild>>`

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

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Accessible label for the group, announced by screen readers.  Substitutes for Base UI's `aria-labelledby` -> legend-id wiring, which gpui cannot express: pass the legend's text here and keep it in sync manually. When set, build the visible `FieldsetLegend` text with `Text::new_inaccessible(...)` so it is not announced twice.

#### `.disabled(...)`

```rust
pub fn disabled(mut self, disabled: bool) -> Self
```

**Accepts**

- `disabled`: `bool`

When true, prevents user interaction with this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(FieldsetRootStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(FieldsetRootStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

FieldsetRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `FieldsetLegend`

Public renderable part of the Fieldset Legend component.

[Source](../../src/fieldset/layers/fieldset_legend.rs)

```rust
use base_gpui::fieldset::FieldsetLegend;

FieldsetLegend::new()
    .with_fieldset_context(/* context: FieldsetContext */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a legend for a Fieldset.  AccessKit: like Base UI's plain-`div` legend, this element has no role and is deliberately kept out of the a11y tree. Expose the group's label via `FieldsetRoot::aria_label(...)` instead. When you do, render the legend's visible text with `Text::new_inaccessible(...)` (not `text!(...)`) so screen readers do not announce the label twice.

#### `.with_fieldset_context(...)`

```rust
pub fn with_fieldset_context(mut self, context: FieldsetContext) -> Self
```

**Accepts**

- `context`: `FieldsetContext`

Sets the with fieldset context configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(FieldsetLegendStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(FieldsetLegendStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

FieldsetLegend also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
