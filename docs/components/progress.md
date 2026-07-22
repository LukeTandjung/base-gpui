# Progress

GPUI-native port of Base UI Progress.

[Base UI reference](https://base-ui.com/react/components/progress) · [Source](../../src/progress/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::progress::{
    ProgressIndicator,
    ProgressLabel,
    ProgressRoot,
    ProgressTrack,
    ProgressValue,
};

ProgressRoot::new()
    .child(
        ProgressLabel::new(),
    )
    .child(
        ProgressTrack::new()
                .child(
                    ProgressIndicator::new(),
                ),
    )
    .child(
        ProgressValue::new(),
    );
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `ProgressRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/progress/layers/progress_root.rs)

```rust
use base_gpui::progress::ProgressRoot;

ProgressRoot::new()
    .id("example-id")
    .value(0.0)
    .min(0.0)
    .max(0.0)
    .format(0.0)
    .label("label")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ProgressRoot with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<ProgressChild>) -> Self
```

**Accepts**

- `child`: `impl Into<ProgressChild>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<ProgressChild>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<ProgressChild>>`

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
pub fn value(mut self, value: Option<f64>) -> Self
```

**Accepts**

- `value`: `Option<f64>`

The task-completion value; `None` means indeterminate (Base UI's `@default null`).

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

Custom formatter receiving the raw (unclamped) value; never invoked when indeterminate.

#### `.label(...)`

```rust
pub fn label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Accessible label for the progress bar; the literal-string replacement for Base UI's `aria-labelledby` wiring to `ProgressLabel`. Pass the same string rendered inside `ProgressLabel`.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ProgressStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ProgressStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ProgressRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ProgressIndicator`

Visual representation of the component's current state or value.

[Source](../../src/progress/layers/progress_indicator.rs)

```rust
use base_gpui::progress::ProgressIndicator;

ProgressIndicator::new()
    .with_progress_context(/* context: ProgressContext */)
    .id("example-id")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ProgressIndicator with its default configuration.

#### `.with_progress_context(...)`

```rust
pub fn with_progress_context(mut self, context: ProgressContext) -> Self
```

**Accepts**

- `context`: `ProgressContext`

Sets the with progress context configuration for this part.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ProgressStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ProgressStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ProgressIndicator also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ProgressLabel`

Provides a visible label and associated accessibility semantics.

[Source](../../src/progress/layers/progress_label.rs)

```rust
use base_gpui::progress::ProgressLabel;

ProgressLabel::new()
    .with_progress_context(/* context: ProgressContext */)
    .id("example-id")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ProgressLabel with its default configuration.

#### `.with_progress_context(...)`

```rust
pub fn with_progress_context(mut self, context: ProgressContext) -> Self
```

**Accepts**

- `context`: `ProgressContext`

Sets the with progress context configuration for this part.

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

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ProgressStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ProgressStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ProgressLabel also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ProgressTrack`

Visual track containing the component's indicator or thumb.

[Source](../../src/progress/layers/progress_track.rs)

```rust
use base_gpui::progress::ProgressTrack;

ProgressTrack::new()
    .with_progress_context(/* context: ProgressContext */)
    .id("example-id")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ProgressTrack with its default configuration.

#### `.with_progress_context(...)`

```rust
pub fn with_progress_context(mut self, context: ProgressContext) -> Self
```

**Accepts**

- `context`: `ProgressContext`

Sets the with progress context configuration for this part.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<ProgressTrackChild>) -> Self
```

**Accepts**

- `child`: `impl Into<ProgressTrackChild>`

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
pub fn style_with_state(mut self, style: impl Fn(ProgressStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ProgressStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ProgressTrack also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ProgressValue`

Displays or edits the component's current value.

[Source](../../src/progress/layers/progress_value.rs)

```rust
use base_gpui::progress::ProgressValue;

ProgressValue::new()
    .with_progress_context(/* context: ProgressContext */)
    .id("example-id")
    .display(0.0)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ProgressValue with its default configuration.

#### `.with_progress_context(...)`

```rust
pub fn with_progress_context(mut self, context: ProgressContext) -> Self
```

**Accepts**

- `context`: `ProgressContext`

Sets the with progress context configuration for this part.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.display(...)`

```rust
pub fn display(mut self, display: impl Fn(Option<&str>, Option<f64>) -> SharedString + 'static,) -> Self
```

**Accepts**

- `display`: `impl Fn(Option<&str>, Option<f64>) -> SharedString + 'static,`

Sets the display configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ProgressStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ProgressStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ProgressValue also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
