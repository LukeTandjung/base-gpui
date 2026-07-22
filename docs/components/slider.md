# Slider

GPUI-native port of Base UI Slider.

[Base UI reference](https://base-ui.com/react/components/slider) · [Source](../../src/slider/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::slider::{
    SliderControl,
    SliderIndicator,
    SliderLabel,
    SliderRoot,
    SliderThumb,
    SliderTrack,
    SliderValue,
};

SliderRoot::new()
    .child(
        SliderControl::new()
                .child(
                    SliderThumb::new(),
                )
                .child(
                    SliderTrack::new()
                                .child(
                                    SliderIndicator::new(),
                                ),
                ),
    )
    .child(
        SliderLabel::new(),
    )
    .child(
        SliderValue::new(),
    );
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `SliderRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/slider/layers/slider_root.rs)

```rust
use base_gpui::slider::SliderRoot;

SliderRoot::new()
    .with_field_context(/* context: FieldContext */)
    .id("example-id")
    .name("name")
    .default_value(/* default_value: SliderValues */)
    .value(/* value: SliderValues */)
    .min(0.0)
    .max(0.0)
    .step(0.0)
    .large_step(0.0)
    .min_steps_between_values(0.0)
    .orientation(/* orientation: SliderOrientation */)
    .thumb_collision_behavior(/* behavior: SliderThumbCollisionBehavior */)
    .thumb_alignment(/* thumb_alignment: SliderThumbAlignment */)
    .disabled(true)
    .aria_label("aria label")
    .format(0.0)
    .on_value_change(|/* callback arguments */| { /* handle change */ })
    .on_value_committed(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a SliderRoot with its default configuration.

#### `.with_field_context(...)`

```rust
pub fn with_field_context(mut self, context: FieldContext) -> Self
```

**Accepts**

- `context`: `FieldContext`

Sets the with field context configuration for this part.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<SliderChild>) -> Self
```

**Accepts**

- `child`: `impl Into<SliderChild>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<SliderChild>>) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<SliderChild>>`

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

#### `.name(...)`

```rust
pub fn name(mut self, name: impl Into<SharedString>) -> Self
```

**Accepts**

- `name`: `impl Into<SharedString>`

Sets the form field name used when serializing or submitting the value.

#### `.default_value(...)`

```rust
pub fn default_value(mut self, default_value: SliderValues) -> Self
```

**Accepts**

- `default_value`: `SliderValues`

Sets the initial value for uncontrolled state. Later user changes are retained by the component.

#### `.value(...)`

```rust
pub fn value(mut self, value: SliderValues) -> Self
```

**Accepts**

- `value`: `SliderValues`

Sets the current controlled value or the value represented by this part, depending on the part's role.

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

#### `.step(...)`

```rust
pub fn step(mut self, step: f64) -> Self
```

**Accepts**

- `step`: `f64`

Sets the step configuration for this part.

#### `.large_step(...)`

```rust
pub fn large_step(mut self, large_step: f64) -> Self
```

**Accepts**

- `large_step`: `f64`

Sets the large step configuration for this part.

#### `.min_steps_between_values(...)`

```rust
pub fn min_steps_between_values(mut self, min_steps_between_values: f64) -> Self
```

**Accepts**

- `min_steps_between_values`: `f64`

Sets the min steps between values configuration for this part.

#### `.orientation(...)`

```rust
pub fn orientation(mut self, orientation: SliderOrientation) -> Self
```

**Accepts**

- `orientation`: `SliderOrientation`

Sets the component's horizontal or vertical orientation and corresponding keyboard behavior.

#### `.thumb_collision_behavior(...)`

```rust
pub fn thumb_collision_behavior(mut self, behavior: SliderThumbCollisionBehavior) -> Self
```

**Accepts**

- `behavior`: `SliderThumbCollisionBehavior`

Sets the thumb collision behavior configuration for this part.

#### `.thumb_alignment(...)`

```rust
pub fn thumb_alignment(mut self, thumb_alignment: SliderThumbAlignment) -> Self
```

**Accepts**

- `thumb_alignment`: `SliderThumbAlignment`

Sets the thumb alignment configuration for this part.

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

Accessible label for the slider group, mirroring what `SliderLabel` displays. Base UI links the label by id via `aria-labelledby`; gpui has no id-reference builder, so the text is supplied literally. Callers who set this should render the visible `SliderLabel` text with `Text::new_inaccessible(...)` to avoid double-announcing.

#### `.format(...)`

```rust
pub fn format(mut self, format: impl Fn(f64) -> SharedString + 'static) -> Self
```

**Accepts**

- `format`: `impl Fn(f64) -> SharedString + 'static`

Sets the format configuration for this part.

#### `.on_value_change(...)`

```rust
pub fn on_value_change(mut self, on_value_change: impl Fn(SliderValues, &mut SliderValueChangeDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_value_change`: `impl Fn(SliderValues, &mut SliderValueChangeDetails, &mut Window, &mut App)
            + 'static`

Registers a callback invoked when value change occurs.

#### `.on_value_committed(...)`

```rust
pub fn on_value_committed(mut self, on_value_committed: impl Fn(SliderValues, SliderValueCommitDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_value_committed`: `impl Fn(SliderValues, SliderValueCommitDetails, &mut Window, &mut App)
            + 'static`

Registers a callback invoked when value committed occurs.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(SliderRootStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(SliderRootStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

SliderRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `SliderControl`

Public renderable part of the Slider Control component.

[Source](../../src/slider/layers/slider_control.rs)

```rust
use base_gpui::slider::SliderControl;

SliderControl::new()
    .with_slider_context(/* context: SliderContext */)
    .map_children(/* map: impl FnOnce(Vec<SliderControlChild>) -> Vec<SliderControlChild>, */)
    .id("example-id")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a SliderControl with its default configuration.

#### `.with_slider_context(...)`

```rust
pub fn with_slider_context(mut self, context: SliderContext) -> Self
```

**Accepts**

- `context`: `SliderContext`

Sets the with slider context configuration for this part.

#### `.map_children(...)`

```rust
pub fn map_children(mut self, map: impl FnOnce(Vec<SliderControlChild>) -> Vec<SliderControlChild>,) -> Self
```

**Accepts**

- `map`: `impl FnOnce(Vec<SliderControlChild>) -> Vec<SliderControlChild>,`

Sets the map children configuration for this part.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<SliderControlChild>) -> Self
```

**Accepts**

- `child`: `impl Into<SliderControlChild>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<SliderControlChild>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<SliderControlChild>>`

Adds multiple typed children in iteration order.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(SliderControlStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(SliderControlStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

SliderControl also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `SliderIndicator`

Visual representation of the component's current state or value.

[Source](../../src/slider/layers/slider_indicator.rs)

```rust
use base_gpui::slider::SliderIndicator;

SliderIndicator::new()
    .with_slider_context(/* context: SliderContext */)
    .id("example-id")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a SliderIndicator with its default configuration.

#### `.with_slider_context(...)`

```rust
pub fn with_slider_context(mut self, context: SliderContext) -> Self
```

**Accepts**

- `context`: `SliderContext`

Sets the with slider context configuration for this part.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(SliderIndicatorStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(SliderIndicatorStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

SliderIndicator also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `SliderLabel`

Provides a visible label and associated accessibility semantics.

[Source](../../src/slider/layers/slider_label.rs)

```rust
use base_gpui::slider::SliderLabel;

SliderLabel::new()
    .with_slider_context(/* context: SliderContext */)
    .id("example-id")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a SliderLabel with its default configuration.

#### `.with_slider_context(...)`

```rust
pub fn with_slider_context(mut self, context: SliderContext) -> Self
```

**Accepts**

- `context`: `SliderContext`

Sets the with slider context configuration for this part.

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

Base UI links this label to the root via `aria-labelledby`; gpui has no id-reference builder, so the label text is instead supplied to `SliderRoot::aria_label(...)`. Once that is set, pass the visible label text here as `Text::new_inaccessible(...)` so it is not announced twice.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(SliderLabelStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(SliderLabelStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

SliderLabel also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `SliderThumb`

Interactive or visual handle representing the current value.

[Source](../../src/slider/layers/slider_thumb.rs)

```rust
use base_gpui::slider::SliderThumb;

SliderThumb::new()
    .with_slider_context(/* context: SliderContext */)
    .with_thumb_index(0)
    .thumb_index()
    .thumb_disabled()
    .id("example-id")
    .disabled(true)
    .aria_label("aria label")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a SliderThumb with its default configuration.

#### `.with_slider_context(...)`

```rust
pub fn with_slider_context(mut self, context: SliderContext) -> Self
```

**Accepts**

- `context`: `SliderContext`

Sets the with slider context configuration for this part.

#### `.with_thumb_index(...)`

```rust
pub fn with_thumb_index(mut self, index: usize) -> Self
```

**Accepts**

- `index`: `usize`

Sets the with thumb index configuration for this part.

#### `.thumb_index(...)`

```rust
pub fn thumb_index(&self) -> Option<usize>
```

Sets the thumb index configuration for this part.

#### `.thumb_disabled(...)`

```rust
pub fn thumb_disabled(&self) -> bool
```

Sets the thumb disabled configuration for this part.

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

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self
```

**Accepts**

- `aria_label`: `impl Into<SharedString>`

Accessible label for this thumb (e.g. "Minimum" / "Maximum" on a range slider). A plain string per thumb replaces Base UI's optional `getAriaLabel(index)` closure.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one typed child to this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(SliderThumbStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(SliderThumbStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

SliderThumb also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `SliderTrack`

Visual track containing the component's indicator or thumb.

[Source](../../src/slider/layers/slider_track.rs)

```rust
use base_gpui::slider::SliderTrack;

SliderTrack::new()
    .with_slider_context(/* context: SliderContext */)
    .map_children(/* map: impl FnOnce(Vec<SliderTrackChild>) -> Vec<SliderTrackChild>, */)
    .id("example-id")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a SliderTrack with its default configuration.

#### `.with_slider_context(...)`

```rust
pub fn with_slider_context(mut self, context: SliderContext) -> Self
```

**Accepts**

- `context`: `SliderContext`

Sets the with slider context configuration for this part.

#### `.map_children(...)`

```rust
pub fn map_children(mut self, map: impl FnOnce(Vec<SliderTrackChild>) -> Vec<SliderTrackChild>,) -> Self
```

**Accepts**

- `map`: `impl FnOnce(Vec<SliderTrackChild>) -> Vec<SliderTrackChild>,`

Sets the map children configuration for this part.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<SliderTrackChild>) -> Self
```

**Accepts**

- `child`: `impl Into<SliderTrackChild>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<SliderTrackChild>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<SliderTrackChild>>`

Adds multiple typed children in iteration order.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(SliderTrackStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(SliderTrackStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

SliderTrack also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `SliderValue`

Displays or edits the component's current value.

[Source](../../src/slider/layers/slider_value.rs)

```rust
use base_gpui::slider::SliderValue;

SliderValue::new()
    .with_slider_context(/* context: SliderContext */)
    .id("example-id")
    .display(0.0)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a SliderValue with its default configuration.

#### `.with_slider_context(...)`

```rust
pub fn with_slider_context(mut self, context: SliderContext) -> Self
```

**Accepts**

- `context`: `SliderContext`

Sets the with slider context configuration for this part.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.display(...)`

```rust
pub fn display(mut self, display: impl Fn(&[SharedString], &[f64]) -> SharedString + 'static,) -> Self
```

**Accepts**

- `display`: `impl Fn(&[SharedString], &[f64]) -> SharedString + 'static,`

Sets the display configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(SliderValueStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(SliderValueStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

SliderValue also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
