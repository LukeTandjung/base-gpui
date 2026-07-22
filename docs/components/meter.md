# Meter

GPUI-native port of Base UI Meter.

[Base UI reference](https://base-ui.com/react/components/meter) · [Source](../../src/meter/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::meter::{
    MeterIndicator,
    MeterLabel,
    MeterRoot,
    MeterTrack,
    MeterValue,
};

MeterRoot::new()
    .child(
        MeterLabel::new(),
    )
    .child(
        MeterTrack::new()
                .child(
                    MeterIndicator::new(),
                ),
    )
    .child(
        MeterValue::new(),
    );
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `MeterRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/meter/layers/meter_root.rs)

```rust
use base_gpui::meter::MeterRoot;

MeterRoot::new()
    .id("example-id")
    .value(0.0)
    .min(0.0)
    .max(0.0)
    .format(0.0)
    .aria_label("label")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a MeterRoot with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<MeterChild>) -> Self
```

**Accepts**

- `child`: `impl Into<MeterChild>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<MeterChild>>) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<MeterChild>>`

Adds multiple typed children in iteration order.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.value(...)`

```rust
pub fn value(mut self, value: f64) -> Self
```

**Accepts**

- `value`: `f64`

The current value of the meter (Base UI's required `value`).

#### `.min(...)`

```rust
pub fn min(mut self, min: f64) -> Self
```

**Accepts**

- `min`: `f64`

Sets the min configuration for this part.

#### `.max(...)`

```rust
pub fn max(mut self, max: f64) -> Self
```

**Accepts**

- `max`: `f64`

Sets the max configuration for this part.

#### `.format(...)`

```rust
pub fn format(mut self, format: impl Fn(f64) -> String + 'static) -> Self
```

**Accepts**

- `format`: `impl Fn(f64) -> String + 'static`

Custom formatter receiving the raw (unclamped) value.

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Accessible label for the meter; the literal-string replacement for Base UI's `aria-labelledby` wiring to `MeterLabel`. Pass the same string rendered inside `MeterLabel`. The formatted value text is appended automatically as the `aria-valuetext` fallback.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(MeterStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(MeterStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

MeterRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `MeterIndicator`

Visual representation of the component's current state or value.

[Source](../../src/meter/layers/meter_indicator.rs)

```rust
use base_gpui::meter::MeterIndicator;

MeterIndicator::new()
    .with_meter_context(/* context: MeterContext */)
    .id("example-id")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a MeterIndicator with its default configuration.

#### `.with_meter_context(...)`

```rust
pub fn with_meter_context(mut self, context: MeterContext) -> Self
```

**Accepts**

- `context`: `MeterContext`

Sets the with meter context configuration for this part.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(MeterStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(MeterStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

MeterIndicator also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `MeterLabel`

Provides a visible label and associated accessibility semantics.

[Source](../../src/meter/layers/meter_label.rs)

```rust
use base_gpui::meter::MeterLabel;

MeterLabel::new()
    .with_meter_context(/* context: MeterContext */)
    .id("example-id")
    .text(/* text: impl Into<SharedString> */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a MeterLabel with its default configuration.

#### `.with_meter_context(...)`

```rust
pub fn with_meter_context(mut self, context: MeterContext) -> Self
```

**Accepts**

- `context`: `MeterContext`

Sets the with meter context configuration for this part.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one typed child to this part.

#### `.text(...)`

```rust
pub fn text(mut self, text: impl Into<SharedString>) -> Self
```

**Accepts**

- `text`: `impl Into<SharedString>`

Visible label text, kept out of the a11y tree so it is not announced twice — the same string should be passed to `MeterRoot::aria_label`.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(MeterStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(MeterStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

MeterLabel also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `MeterTrack`

Visual track containing the component's indicator or thumb.

[Source](../../src/meter/layers/meter_track.rs)

```rust
use base_gpui::meter::MeterTrack;

MeterTrack::new()
    .with_meter_context(/* context: MeterContext */)
    .id("example-id")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a MeterTrack with its default configuration.

#### `.with_meter_context(...)`

```rust
pub fn with_meter_context(mut self, context: MeterContext) -> Self
```

**Accepts**

- `context`: `MeterContext`

Sets the with meter context configuration for this part.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<MeterTrackChild>) -> Self
```

**Accepts**

- `child`: `impl Into<MeterTrackChild>`

Adds one typed child to this part.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(MeterStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(MeterStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

MeterTrack also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `MeterValue`

Displays or edits the component's current value.

[Source](../../src/meter/layers/meter_value.rs)

```rust
use base_gpui::meter::MeterValue;

MeterValue::new()
    .with_meter_context(/* context: MeterContext */)
    .id("example-id")
    .display(0.0)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a MeterValue with its default configuration.

#### `.with_meter_context(...)`

```rust
pub fn with_meter_context(mut self, context: MeterContext) -> Self
```

**Accepts**

- `context`: `MeterContext`

Sets the with meter context configuration for this part.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.display(...)`

```rust
pub fn display(mut self, display: impl Fn(&str, f64) -> SharedString + 'static) -> Self
```

**Accepts**

- `display`: `impl Fn(&str, f64) -> SharedString + 'static`

Sets the display configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(MeterStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(MeterStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

MeterValue also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
