# Field

Base UI Field's ARIA wiring is id-reference based and this gpui revision has no relationship builders, so parts of it are intentionally omitted (see `issues/port-baseui-field.md`, "AccessKit accessibility follow-up"):

[Base UI reference](https://base-ui.com/react/components/field) · [Source](../../src/field/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::field::{
    FieldControl,
    FieldDescription,
    FieldError,
    FieldItem,
    FieldLabel,
    FieldRoot,
    FieldValidity,
};

FieldRoot::new()
    .child(
        FieldItem::new()
                .child(
                    FieldControl::new(),
                )
                .child(
                    FieldDescription::new(),
                )
                .child(
                    FieldError::new(),
                )
                .child(
                    FieldLabel::new(),
                )
                .child(
                    FieldValidity::new(),
                ),
    );
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `FieldRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/field/layers/field_root.rs)

```rust
use base_gpui::field::FieldRoot;

FieldRoot::new()
    .id("example-id")
    .name("name")
    .disabled(true)
    .invalid(true)
    .dirty(true)
    .touched(true)
    .validation_mode("example-id")
    .validation_debounce("example-id")
    .validate("example-id")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a FieldRoot with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<FieldChild>) -> Self
```

**Accepts**

- `child`: `impl Into<FieldChild>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<FieldChild>>) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<FieldChild>>`

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

#### `.disabled(...)`

```rust
pub fn disabled(mut self, disabled: bool) -> Self
```

**Accepts**

- `disabled`: `bool`

When true, prevents user interaction with this part.

#### `.invalid(...)`

```rust
pub fn invalid(mut self, invalid: bool) -> Self
```

**Accepts**

- `invalid`: `bool`

Sets the invalid configuration for this part.

#### `.dirty(...)`

```rust
pub fn dirty(mut self, dirty: bool) -> Self
```

**Accepts**

- `dirty`: `bool`

Sets the dirty configuration for this part.

#### `.touched(...)`

```rust
pub fn touched(mut self, touched: bool) -> Self
```

**Accepts**

- `touched`: `bool`

Sets the touched configuration for this part.

#### `.validation_mode(...)`

```rust
pub fn validation_mode(mut self, validation_mode: FieldValidationMode) -> Self
```

**Accepts**

- `validation_mode`: `FieldValidationMode`

Sets the validation mode configuration for this part.

#### `.validation_debounce(...)`

```rust
pub fn validation_debounce(mut self, validation_debounce: Duration) -> Self
```

**Accepts**

- `validation_debounce`: `Duration`

Sets the validation debounce configuration for this part.

#### `.validate(...)`

```rust
pub fn validate(mut self, validate: impl Fn(&FieldValue, &mut Window, &mut App) -> FieldValidationResult + 'static,) -> Self
```

**Accepts**

- `validate`: `impl Fn(&FieldValue, &mut Window, &mut App) -> FieldValidationResult + 'static,`

Sets the validate configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(FieldRootStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(FieldRootStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

FieldRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `FieldControl`

Public renderable part of the Field Control component.

[Source](../../src/field/layers/field_control.rs)

```rust
use base_gpui::field::FieldControl;

FieldControl::new()
    .with_field_context(/* context: FieldContext */)
    .id("example-id")
    .name("name")
    .value(/* value: impl Into<SharedString> */)
    .default_value(/* default_value: impl Into<SharedString> */)
    .placeholder("placeholder")
    .disabled(true)
    .read_only(true)
    .required(true)
    .auto_focus(true)
    .tab_index(/* tab_index: isize */)
    .tab_stop(true)
    .focus_handle(/* focus_handle: FocusHandle */)
    .on_edge_left(|/* callback arguments */| { /* handle change */ })
    .on_edge_right(|/* callback arguments */| { /* handle change */ })
    .select_all_on_focus(true)
    .on_value_change(|/* callback arguments */| { /* handle change */ })
    .on_enter(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a FieldControl with its default configuration.

#### `.with_field_context(...)`

```rust
pub fn with_field_context(mut self, context: FieldContext) -> Self
```

**Accepts**

- `context`: `FieldContext`

Sets the with field context configuration for this part.

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

#### `.value(...)`

```rust
pub fn value(mut self, value: impl Into<SharedString>) -> Self
```

**Accepts**

- `value`: `impl Into<SharedString>`

Sets the current controlled value or the value represented by this part, depending on the part's role.

#### `.default_value(...)`

```rust
pub fn default_value(mut self, default_value: impl Into<SharedString>) -> Self
```

**Accepts**

- `default_value`: `impl Into<SharedString>`

Sets the initial value for uncontrolled state. Later user changes are retained by the component.

#### `.placeholder(...)`

```rust
pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self
```

**Accepts**

- `placeholder`: `impl Into<SharedString>`

Sets the content shown when the component has no current value.

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

#### `.tab_stop(...)`

```rust
pub fn tab_stop(mut self, tab_stop: bool) -> Self
```

**Accepts**

- `tab_stop`: `bool`

Overrides window Tab-order participation; composite containers such as the Toolbar use this to keep a single roving tab stop.

#### `.focus_handle(...)`

```rust
pub fn focus_handle(mut self, focus_handle: FocusHandle) -> Self
```

**Accepts**

- `focus_handle`: `FocusHandle`

Overrides the keyed focus handle so composite containers can own the control's roving focus handle.

#### `.on_edge_left(...)`

```rust
pub fn on_edge_left(mut self, on_edge_left: impl Fn(SharedString, &mut Window, &mut gpui::Context<InputRuntime>) -> bool + 'static,) -> Self
```

**Accepts**

- `on_edge_left`: `impl Fn(SharedString, &mut Window, &mut gpui::Context<InputRuntime>) -> bool
            + 'static,`

Registers a callback invoked when edge left occurs.

#### `.on_edge_right(...)`

```rust
pub fn on_edge_right(mut self, on_edge_right: impl Fn(SharedString, &mut Window, &mut gpui::Context<InputRuntime>) -> bool + 'static,) -> Self
```

**Accepts**

- `on_edge_right`: `impl Fn(SharedString, &mut Window, &mut gpui::Context<InputRuntime>) -> bool
            + 'static,`

Registers a callback invoked when edge right occurs.

#### `.select_all_on_focus(...)`

```rust
pub fn select_all_on_focus(mut self, select_all_on_focus: bool) -> Self
```

**Accepts**

- `select_all_on_focus`: `bool`

Controls whether select all on focus behavior is enabled.

#### `.on_value_change(...)`

```rust
pub fn on_value_change(mut self, on_value_change: impl Fn(SharedString) + 'static) -> Self
```

**Accepts**

- `on_value_change`: `impl Fn(SharedString) + 'static`

Registers a callback invoked when value change occurs.

#### `.on_enter(...)`

```rust
pub fn on_enter(mut self, on_enter: impl Fn(SharedString) + 'static) -> Self
```

**Accepts**

- `on_enter`: `impl Fn(SharedString) + 'static`

Registers a callback invoked when enter occurs.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(InputStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(InputStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

FieldControl also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `FieldDescription`

Provides supporting descriptive content.

[Source](../../src/field/layers/field_description.rs)

```rust
use base_gpui::field::FieldDescription;

FieldDescription::new()
    .with_field_context(/* context: FieldContext */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a FieldDescription with its default configuration.

#### `.with_field_context(...)`

```rust
pub fn with_field_context(mut self, context: FieldContext) -> Self
```

**Accepts**

- `context`: `FieldContext`

Sets the with field context configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(FieldDescriptionStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(FieldDescriptionStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

FieldDescription also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `FieldError`

Public renderable part of the Field Error component.

[Source](../../src/field/layers/field_error.rs)

```rust
use base_gpui::field::FieldError;

FieldError::new()
    .with_field_context(/* context: FieldContext */)
    .match_(/* key: FieldValidityKey */)
    .match_always(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a FieldError with its default configuration.

#### `.with_field_context(...)`

```rust
pub fn with_field_context(mut self, context: FieldContext) -> Self
```

**Accepts**

- `context`: `FieldContext`

Sets the with field context configuration for this part.

#### `.match_(...)`

```rust
pub fn match_(mut self, key: FieldValidityKey) -> Self
```

**Accepts**

- `key`: `FieldValidityKey`

Sets the match  configuration for this part.

#### `.match_always(...)`

```rust
pub fn match_always(mut self, always: bool) -> Self
```

**Accepts**

- `always`: `bool`

Sets the match always configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(FieldErrorStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(FieldErrorStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

FieldError also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `FieldItem`

Represents one interactive item in the component's collection.

[Source](../../src/field/layers/field_item.rs)

```rust
use base_gpui::field::FieldItem;

FieldItem::new()
    .disabled(true)
    .with_field_context(/* context: FieldContext */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a FieldItem with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<FieldItemChild>) -> Self
```

**Accepts**

- `child`: `impl Into<FieldItemChild>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<FieldItemChild>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<FieldItemChild>>`

Adds multiple typed children in iteration order.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.disabled(...)`

```rust
pub fn disabled(mut self, disabled: bool) -> Self
```

**Accepts**

- `disabled`: `bool`

When true, prevents user interaction with this part.

#### `.with_field_context(...)`

```rust
pub fn with_field_context(mut self, context: FieldContext) -> Self
```

**Accepts**

- `context`: `FieldContext`

Sets the with field context configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(FieldItemStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(FieldItemStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

FieldItem also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `FieldLabel`

Provides a visible label and associated accessibility semantics.

[Source](../../src/field/layers/field_label.rs)

```rust
use base_gpui::field::FieldLabel;

FieldLabel::new()
    .with_field_context(/* context: FieldContext */)
    .text(/* text: impl Into<SharedString> */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a FieldLabel with its default configuration.

#### `.with_field_context(...)`

```rust
pub fn with_field_context(mut self, context: FieldContext) -> Self
```

**Accepts**

- `context`: `FieldContext`

Sets the with field context configuration for this part.

#### `.text(...)`

```rust
pub fn text(mut self, text: impl Into<SharedString>) -> Self
```

**Accepts**

- `text`: `impl Into<SharedString>`

Sets the visible label text and registers it on the field runtime so the registered control can expose it as its accessible name. Rendered with `Text::new_inaccessible` to avoid double-announcing once the control carries the `.aria_label`.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(FieldLabelStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(FieldLabelStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

FieldLabel also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `FieldValidity`

Public renderable part of the Field Validity component.

[Source](../../src/field/layers/field_validity.rs)

```rust
use base_gpui::field::FieldValidity;

FieldValidity::new()
    .with_field_context(/* context: FieldContext */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a FieldValidity with its default configuration.

#### `.with_field_context(...)`

```rust
pub fn with_field_context(mut self, context: FieldContext) -> Self
```

**Accepts**

- `context`: `FieldContext`

Sets the with field context configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(FieldValidityStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(FieldValidityStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

FieldValidity also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
