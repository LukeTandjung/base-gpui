# Number Field

Number Field component family.

[Base UI reference](https://base-ui.com/react/components/number-field) · [Source](../../src/number_field/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::number_field::{
    NumberFieldDecrement,
    NumberFieldGroup,
    NumberFieldIncrement,
    NumberFieldInput,
    NumberFieldRoot,
    NumberFieldScrubArea,
    NumberFieldScrubAreaCursor,
};

NumberFieldRoot::new()
    .child(
        NumberFieldGroup::new()
                .child(
                    NumberFieldDecrement::new(),
                )
                .child(
                    NumberFieldIncrement::new(),
                )
                .child(
                    NumberFieldInput::new(),
                ),
    )
    .child(
        NumberFieldScrubArea::new(),
    )
    .child(
        NumberFieldScrubAreaCursor::new(),
    );
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `NumberFieldRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/number_field/layers/number_field_root.rs)

```rust
use base_gpui::number_field::NumberFieldRoot;

NumberFieldRoot::new()
    .with_field_context(/* context: FieldContext */)
    .id("example-id")
    .name("name")
    .form(/* form: impl Into<SharedString> */)
    .default_value(0.0)
    .value(0.0)
    .min(0.0)
    .max(0.0)
    .step(0.0)
    .step_any()
    .small_step(0.0)
    .large_step(0.0)
    .snap_on_step(true)
    .allow_out_of_range(true)
    .allow_wheel_scrub(true)
    .disabled(true)
    .read_only(true)
    .required(true)
    .on_value_change(0.0)
    .on_value_committed(0.0)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a NumberFieldRoot with its default configuration.

#### `.with_field_context(...)`

```rust
pub fn with_field_context(mut self, context: FieldContext) -> Self
```

**Accepts**

- `context`: `FieldContext`

Sets the with field context configuration for this part.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<NumberFieldChild>) -> Self
```

**Accepts**

- `child`: `impl Into<NumberFieldChild>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<NumberFieldChild>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<NumberFieldChild>>`

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

#### `.form(...)`

```rust
pub fn form(mut self, form: impl Into<SharedString>) -> Self
```

**Accepts**

- `form`: `impl Into<SharedString>`

Sets the form configuration for this part.

#### `.default_value(...)`

```rust
pub fn default_value(mut self, default_value: Option<f64>) -> Self
```

**Accepts**

- `default_value`: `Option<f64>`

Sets the initial value for uncontrolled state. Later user changes are retained by the component.

#### `.value(...)`

```rust
pub fn value(mut self, value: Option<f64>) -> Self
```

**Accepts**

- `value`: `Option<f64>`

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

#### `.step_any(...)`

```rust
pub fn step_any(mut self) -> Self
```

Sets the step any configuration for this part.

#### `.small_step(...)`

```rust
pub fn small_step(mut self, small_step: f64) -> Self
```

**Accepts**

- `small_step`: `f64`

Sets the small step configuration for this part.

#### `.large_step(...)`

```rust
pub fn large_step(mut self, large_step: f64) -> Self
```

**Accepts**

- `large_step`: `f64`

Sets the large step configuration for this part.

#### `.snap_on_step(...)`

```rust
pub fn snap_on_step(mut self, snap_on_step: bool) -> Self
```

**Accepts**

- `snap_on_step`: `bool`

Sets the snap on step configuration for this part.

#### `.allow_out_of_range(...)`

```rust
pub fn allow_out_of_range(mut self, allow_out_of_range: bool) -> Self
```

**Accepts**

- `allow_out_of_range`: `bool`

Sets the allow out of range configuration for this part.

#### `.allow_wheel_scrub(...)`

```rust
pub fn allow_wheel_scrub(mut self, allow_wheel_scrub: bool) -> Self
```

**Accepts**

- `allow_wheel_scrub`: `bool`

Sets the allow wheel scrub configuration for this part.

#### `.disabled(...)`

```rust
pub fn disabled(mut self, disabled: bool) -> Self
```

**Accepts**

- `disabled`: `bool`

When true, prevents user interaction with this part.

#### `.read_only(...)`

```rust
pub fn read_only(mut self, read_only: bool) -> Self
```

**Accepts**

- `read_only`: `bool`

When true, allows presentation and focus behavior without permitting value changes.

#### `.required(...)`

```rust
pub fn required(mut self, required: bool) -> Self
```

**Accepts**

- `required`: `bool`

Marks the control as required for validation and accessibility state.

#### `.on_value_change(...)`

```rust
pub fn on_value_change(mut self, on_value_change: impl Fn(Option<f64>, NumberFieldChangeDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_value_change`: `impl Fn(Option<f64>, NumberFieldChangeDetails, &mut Window, &mut App) + 'static`

Registers a callback invoked when value change occurs.

#### `.on_value_committed(...)`

```rust
pub fn on_value_committed(mut self, on_value_committed: impl Fn(Option<f64>, NumberFieldCommitDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_value_committed`: `impl Fn(Option<f64>, NumberFieldCommitDetails, &mut Window, &mut App)
            + 'static`

Registers a callback invoked when value committed occurs.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(NumberFieldRootStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(NumberFieldRootStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

NumberFieldRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `NumberFieldDecrement`

Public renderable part of the Number Field Decrement component.

[Source](../../src/number_field/layers/number_field_decrement.rs)

```rust
use base_gpui::number_field::NumberFieldDecrement;

NumberFieldDecrement::new()
    .with_number_field_context(/* context: NumberFieldContext */)
    .id("example-id")
    .aria_label("aria label")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a NumberFieldDecrement with its default configuration.

#### `.with_number_field_context(...)`

```rust
pub fn with_number_field_context(mut self, context: NumberFieldContext) -> Self
```

**Accepts**

- `context`: `NumberFieldContext`

Sets the with number field context configuration for this part.

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

Override the default `"Decrease"` accessible label. Visible glyph children (e.g. `"-"`) should use `Text::new_inaccessible(...)` to avoid being announced in addition to this label.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl IntoElement>`

Adds multiple typed children in iteration order.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(NumberFieldDecrementStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(NumberFieldDecrementStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

NumberFieldDecrement also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `NumberFieldGroup`

Groups related child parts and coordinates their shared behavior.

[Source](../../src/number_field/layers/number_field_group.rs)

```rust
use base_gpui::number_field::NumberFieldGroup;

NumberFieldGroup::new()
    .with_number_field_context(/* context: NumberFieldContext */)
    .id("example-id")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a NumberFieldGroup with its default configuration.

#### `.with_number_field_context(...)`

```rust
pub fn with_number_field_context(mut self, context: NumberFieldContext) -> Self
```

**Accepts**

- `context`: `NumberFieldContext`

Sets the with number field context configuration for this part.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<NumberFieldGroupChild>) -> Self
```

**Accepts**

- `child`: `impl Into<NumberFieldGroupChild>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<NumberFieldGroupChild>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<NumberFieldGroupChild>>`

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
pub fn style_with_state(mut self, style: impl Fn(NumberFieldGroupStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(NumberFieldGroupStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

NumberFieldGroup also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `NumberFieldIncrement`

Public renderable part of the Number Field Increment component.

[Source](../../src/number_field/layers/number_field_increment.rs)

```rust
use base_gpui::number_field::NumberFieldIncrement;

NumberFieldIncrement::new()
    .with_number_field_context(/* context: NumberFieldContext */)
    .id("example-id")
    .aria_label("aria label")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a NumberFieldIncrement with its default configuration.

#### `.with_number_field_context(...)`

```rust
pub fn with_number_field_context(mut self, context: NumberFieldContext) -> Self
```

**Accepts**

- `context`: `NumberFieldContext`

Sets the with number field context configuration for this part.

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

Override the default `"Increase"` accessible label. Visible glyph children (e.g. `"+"`) should use `Text::new_inaccessible(...)` to avoid being announced in addition to this label.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl IntoElement>`

Adds multiple typed children in iteration order.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(NumberFieldIncrementStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(NumberFieldIncrementStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

NumberFieldIncrement also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `NumberFieldInput`

Text input integrated with the component's state and behavior.

[Source](../../src/number_field/layers/number_field_input.rs)

```rust
use base_gpui::number_field::NumberFieldInput;

NumberFieldInput::new()
    .with_number_field_context(/* context: NumberFieldContext */)
    .id("example-id")
    .placeholder("placeholder")
    .auto_focus(true)
    .tab_index(/* tab_index: isize */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a NumberFieldInput with its default configuration.

#### `.with_number_field_context(...)`

```rust
pub fn with_number_field_context(mut self, context: NumberFieldContext) -> Self
```

**Accepts**

- `context`: `NumberFieldContext`

Sets the with number field context configuration for this part.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.placeholder(...)`

```rust
pub fn placeholder(mut self, placeholder: impl Into<gpui::SharedString>) -> Self
```

**Accepts**

- `placeholder`: `impl Into<gpui::SharedString>`

Sets the content shown when the component has no current value.

#### `.auto_focus(...)`

```rust
pub fn auto_focus(mut self, auto_focus: bool) -> Self
```

**Accepts**

- `auto_focus`: `bool`

Controls whether auto focus behavior is enabled.

#### `.tab_index(...)`

```rust
pub fn tab_index(mut self, tab_index: isize) -> Self
```

**Accepts**

- `tab_index`: `isize`

Sets the tab index configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(NumberFieldInputStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(NumberFieldInputStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

NumberFieldInput also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `NumberFieldScrubArea`

Public renderable part of the Number Field Scrub Area component.

[Source](../../src/number_field/layers/number_field_scrub_area.rs)

```rust
use base_gpui::number_field::NumberFieldScrubArea;

NumberFieldScrubArea::new()
    .with_number_field_context(/* context: NumberFieldContext */)
    .id("example-id")
    .direction(/* direction: NumberFieldScrubDirection */)
    .pixel_sensitivity(0.0)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a NumberFieldScrubArea with its default configuration.

#### `.with_number_field_context(...)`

```rust
pub fn with_number_field_context(mut self, context: NumberFieldContext) -> Self
```

**Accepts**

- `context`: `NumberFieldContext`

Sets the with number field context configuration for this part.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.direction(...)`

```rust
pub fn direction(mut self, direction: NumberFieldScrubDirection) -> Self
```

**Accepts**

- `direction`: `NumberFieldScrubDirection`

Sets the direction configuration for this part.

#### `.pixel_sensitivity(...)`

```rust
pub fn pixel_sensitivity(mut self, pixel_sensitivity: f64) -> Self
```

**Accepts**

- `pixel_sensitivity`: `f64`

Sets the pixel sensitivity configuration for this part.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl IntoElement>`

Adds multiple typed children in iteration order.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(NumberFieldScrubAreaStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(NumberFieldScrubAreaStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

NumberFieldScrubArea also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `NumberFieldScrubAreaCursor`

Public renderable part of the Number Field Scrub Area Cursor component.

[Source](../../src/number_field/layers/number_field_scrub_area_cursor.rs)

```rust
use base_gpui::number_field::NumberFieldScrubAreaCursor;

NumberFieldScrubAreaCursor::new()
    .with_number_field_context(/* context: NumberFieldContext */)
    .id("example-id")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a NumberFieldScrubAreaCursor with its default configuration.

#### `.with_number_field_context(...)`

```rust
pub fn with_number_field_context(mut self, context: NumberFieldContext) -> Self
```

**Accepts**

- `context`: `NumberFieldContext`

Sets the with number field context configuration for this part.

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

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl IntoElement>`

Adds multiple typed children in iteration order.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(NumberFieldScrubAreaCursorStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(NumberFieldScrubAreaCursorStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

NumberFieldScrubAreaCursor also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
