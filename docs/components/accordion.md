# Accordion

Accessible collapsible-panel component ported from Base UI's Accordion.

[Base UI reference](https://base-ui.com/react/components/accordion) · [Source](../../src/accordion/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::accordion::{
    AccordionHeader,
    AccordionItem,
    AccordionPanel,
    AccordionRoot,
    AccordionTrigger,
};

AccordionRoot::new()
    .child(
        AccordionItem::new(/* value: T */)
                .child(
                    AccordionHeader::new()
                                .child(
                                    AccordionTrigger::new(),
                                ),
                )
                .child(
                    AccordionPanel::new(),
                ),
    );
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `AccordionRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/accordion/layers/accordion_root.rs)

```rust
use base_gpui::accordion::AccordionRoot;

AccordionRoot::new()
    .id("example-id")
    .default_value(/* default_value: Vec<T> */)
    .value(/* value: Vec<T> */)
    .disabled(true)
    .multiple(true)
    .keep_mounted(true)
    .orientation(/* orientation: AccordionOrientation */)
    .on_value_change(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a AccordionRoot with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<AccordionRootChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<AccordionRootChild<T>>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<AccordionRootChild<T>>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<AccordionRootChild<T>>>`

Adds multiple typed children in iteration order.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

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

#### `.multiple(...)`

```rust
pub fn multiple(mut self, multiple: bool) -> Self
```

**Accepts**

- `multiple`: `bool`

Controls whether more than one value may be selected at the same time.

#### `.keep_mounted(...)`

```rust
pub fn keep_mounted(mut self, keep_mounted: bool) -> Self
```

**Accepts**

- `keep_mounted`: `bool`

Keeps the part mounted when inactive or closed so child state can be preserved.

#### `.orientation(...)`

```rust
pub fn orientation(mut self, orientation: AccordionOrientation) -> Self
```

**Accepts**

- `orientation`: `AccordionOrientation`

Sets the component's horizontal or vertical orientation and corresponding keyboard behavior.

#### `.on_value_change(...)`

```rust
pub fn on_value_change(mut self, on_value_change: impl Fn(&[T], &mut AccordionValueChangeDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_value_change`: `impl Fn(&[T], &mut AccordionValueChangeDetails, &mut Window, &mut App)
            + 'static`

Registers a callback invoked when value change occurs.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(AccordionRootStyleState<T>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(AccordionRootStyleState<T>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

AccordionRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `AccordionTrigger`

Interactive control that opens, closes, or activates the component.

[Source](../../src/accordion/layers/accordion_trigger.rs)

```rust
use base_gpui::accordion::AccordionTrigger;

AccordionTrigger::new()
    .id("example-id")
    .aria_label("aria label")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a AccordionTrigger with its default configuration.

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

Accessible name for triggers whose visible content is iconic/non-textual. When set, render the visible label with `Text::new_inaccessible(...)` so screen readers do not announce the name twice.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(AccordionTriggerStyleState<T>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(AccordionTriggerStyleState<T>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

AccordionTrigger also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `AccordionHeader`

Public renderable part of the Accordion Header component.

[Source](../../src/accordion/layers/accordion_header.rs)

```rust
use base_gpui::accordion::AccordionHeader;

AccordionHeader::new()
    .id("example-id")
    .heading_level(0)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a AccordionHeader with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.heading_level(...)`

```rust
pub fn heading_level(mut self, heading_level: usize) -> Self
```

**Accepts**

- `heading_level`: `usize`

Sets the heading level configuration for this part.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<AccordionHeaderChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<AccordionHeaderChild<T>>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<AccordionHeaderChild<T>>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<AccordionHeaderChild<T>>>`

Adds multiple typed children in iteration order.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(AccordionHeaderStyleState<T>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(AccordionHeaderStyleState<T>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

AccordionHeader also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `AccordionItem`

Represents one interactive item in the component's collection.

[Source](../../src/accordion/layers/accordion_item.rs)

```rust
use base_gpui::accordion::AccordionItem;

AccordionItem::new()
    .id("example-id")
    .disabled(true)
    .on_open_change(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new(value: T) -> Self
```

**Accepts**

- `value`: `T`

Creates a AccordionItem with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<AccordionItemChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<AccordionItemChild<T>>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<AccordionItemChild<T>>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<AccordionItemChild<T>>>`

Adds multiple typed children in iteration order.

#### `.disabled(...)`

```rust
pub fn disabled(mut self, disabled: bool) -> Self
```

**Accepts**

- `disabled`: `bool`

When true, prevents user interaction with this part.

#### `.on_open_change(...)`

```rust
pub fn on_open_change(mut self, on_open_change: impl Fn(bool, &mut AccordionItemOpenChangeDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_open_change`: `impl Fn(bool, &mut AccordionItemOpenChangeDetails, &mut Window, &mut App)
            + 'static`

Registers a callback invoked when open change occurs.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(AccordionItemStyleState<T>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(AccordionItemStyleState<T>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

AccordionItem also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `AccordionPanel`

Contains content associated with the active item.

[Source](../../src/accordion/layers/accordion_panel.rs)

```rust
use base_gpui::accordion::AccordionPanel;

AccordionPanel::new()
    .id("example-id")
    .keep_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a AccordionPanel with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.keep_mounted(...)`

```rust
pub fn keep_mounted(mut self, keep_mounted: bool) -> Self
```

**Accepts**

- `keep_mounted`: `bool`

Keeps the part mounted when inactive or closed so child state can be preserved.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(AccordionPanelStyleState<T>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(AccordionPanelStyleState<T>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

AccordionPanel also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
