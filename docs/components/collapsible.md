# Collapsible

Collapsible ports Base UI's Collapsible component model to GPUI.

[Base UI reference](https://base-ui.com/react/components/collapsible) · [Source](../../src/collapsible/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::collapsible::{
    CollapsiblePanel,
    CollapsibleRoot,
    CollapsibleTrigger,
};

CollapsibleRoot::new()
    .child(
        CollapsiblePanel::new(),
    )
    .child(
        CollapsibleTrigger::new(),
    );
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `CollapsibleRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/collapsible/layers/collapsible_root.rs)

```rust
use base_gpui::collapsible::CollapsibleRoot;

CollapsibleRoot::new()
    .id("example-id")
    .default_open(true)
    .open(None)
    .disabled(true)
    .on_open_change(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a CollapsibleRoot with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<CollapsibleChild>) -> Self
```

**Accepts**

- `child`: `impl Into<CollapsibleChild>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<CollapsibleChild>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<CollapsibleChild>>`

Adds multiple typed children in iteration order.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.default_open(...)`

```rust
pub fn default_open(mut self, default_open: bool) -> Self
```

**Accepts**

- `default_open`: `bool`

Sets the initial open state for an uncontrolled component.

#### `.open(...)`

```rust
pub fn open(mut self, open: Option<bool>) -> Self
```

**Accepts**

- `open`: `Option<bool>`

Controls whether the component is open.

#### `.disabled(...)`

```rust
pub fn disabled(mut self, disabled: bool) -> Self
```

**Accepts**

- `disabled`: `bool`

When true, prevents user interaction with this part.

#### `.on_open_change(...)`

```rust
pub fn on_open_change(mut self, on_open_change: impl Fn(bool, &mut CollapsibleOpenChangeDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_open_change`: `impl Fn(bool, &mut CollapsibleOpenChangeDetails, &mut Window, &mut App)
            + 'static`

Registers a callback invoked when open change occurs.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(CollapsibleRootStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(CollapsibleRootStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

CollapsibleRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `CollapsibleTrigger`

Interactive control that opens, closes, or activates the component.

[Source](../../src/collapsible/layers/collapsible_trigger.rs)

```rust
use base_gpui::collapsible::CollapsibleTrigger;

CollapsibleTrigger::new()
    .id("example-id")
    .aria_label("aria label")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a CollapsibleTrigger with its default configuration.

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

Sets the accessible label announced for this trigger.  Required for icon-only triggers. When the trigger also has a visible text label child, pass that child as `Text::new_inaccessible(...)` so screen readers do not announce the label twice; without an `aria_label`, leave child text accessible (`text!(...)`) so it names the button.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(CollapsibleTriggerStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(CollapsibleTriggerStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

CollapsibleTrigger also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `CollapsiblePanel`

Contains content associated with the active item.

[Source](../../src/collapsible/layers/collapsible_panel.rs)

```rust
use base_gpui::collapsible::CollapsiblePanel;

CollapsiblePanel::new()
    .keep_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a CollapsiblePanel with its default configuration.

#### `.keep_mounted(...)`

```rust
pub fn keep_mounted(mut self, keep_mounted: bool) -> Self
```

**Accepts**

- `keep_mounted`: `bool`

Keeps the part mounted when inactive or closed so child state can be preserved.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(CollapsiblePanelStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(CollapsiblePanelStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

CollapsiblePanel also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
