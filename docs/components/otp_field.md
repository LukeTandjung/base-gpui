# OTP Field

OTP Field: a group of one-character slots editing a single OTP string.

[Base UI reference](https://base-ui.com/react/components/otp-field) · [Source](../../src/otp_field/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::otp_field::{
    OTPFieldInput,
    OTPFieldRoot,
};

OTPFieldRoot::new()
    .child(
        OTPFieldInput::new(),
    );
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `OTPFieldRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/otp_field/layers/otp_field_root.rs)

```rust
use base_gpui::otp_field::OTPFieldRoot;

OTPFieldRoot::new()
    .with_field_context(/* context: FieldContext */)
    .id("example-id")
    .name("name")
    .aria_label("aria label")
    .length(0)
    .default_value(/* default_value: impl Into<SharedString> */)
    .value(/* value: impl Into<SharedString> */)
    .validation_type("example-id")
    .normalize_value(|/* callback arguments */| { /* handle change */ })
    .mask(true)
    .auto_submit(true)
    .disabled(true)
    .read_only(true)
    .required(true)
    .on_value_change(|/* callback arguments */| { /* handle change */ })
    .on_value_complete(|/* callback arguments */| { /* handle change */ })
    .on_value_invalid("example-id")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a OTPFieldRoot with its default configuration.

#### `.with_field_context(...)`

```rust
pub fn with_field_context(mut self, context: FieldContext) -> Self
```

**Accepts**

- `context`: `FieldContext`

Sets the with field context configuration for this part.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<OTPFieldChild>) -> Self
```

**Accepts**

- `child`: `impl Into<OTPFieldChild>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<OTPFieldChild>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<OTPFieldChild>>`

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

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self
```

**Accepts**

- `aria_label`: `impl Into<SharedString>`

Accessible name for the OTP group. Mirror the visible `FieldLabel` text here manually: relationship props (`aria-labelledby`) do not exist in this gpui revision.

#### `.length(...)`

```rust
pub fn length(mut self, length: usize) -> Self
```

**Accepts**

- `length`: `usize`

Sets the length configuration for this part.

#### `.default_value(...)`

```rust
pub fn default_value(mut self, default_value: impl Into<SharedString>) -> Self
```

**Accepts**

- `default_value`: `impl Into<SharedString>`

Sets the initial value for uncontrolled state. Later user changes are retained by the component.

#### `.value(...)`

```rust
pub fn value(mut self, value: impl Into<SharedString>) -> Self
```

**Accepts**

- `value`: `impl Into<SharedString>`

Sets the current controlled value or the value represented by this part, depending on the part's role.

#### `.validation_type(...)`

```rust
pub fn validation_type(mut self, validation_type: OTPFieldValidationType) -> Self
```

**Accepts**

- `validation_type`: `OTPFieldValidationType`

Sets the validation type configuration for this part.

#### `.normalize_value(...)`

```rust
pub fn normalize_value(mut self, normalize_value: impl Fn(String) -> String + 'static) -> Self
```

**Accepts**

- `normalize_value`: `impl Fn(String) -> String + 'static`

Sets the normalize value configuration for this part.

#### `.mask(...)`

```rust
pub fn mask(mut self, mask: bool) -> Self
```

**Accepts**

- `mask`: `bool`

Sets the mask configuration for this part.

#### `.auto_submit(...)`

```rust
pub fn auto_submit(mut self, auto_submit: bool) -> Self
```

**Accepts**

- `auto_submit`: `bool`

Controls whether auto submit behavior is enabled.

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
pub fn on_value_change(mut self, on_value_change: impl Fn(SharedString, OTPFieldChangeDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_value_change`: `impl Fn(SharedString, OTPFieldChangeDetails, &mut Window, &mut App) + 'static`

Registers a callback invoked when value change occurs.

#### `.on_value_complete(...)`

```rust
pub fn on_value_complete(mut self, on_value_complete: impl Fn(SharedString, OTPFieldChangeDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_value_complete`: `impl Fn(SharedString, OTPFieldChangeDetails, &mut Window, &mut App) + 'static`

Registers a callback invoked when value complete occurs.

#### `.on_value_invalid(...)`

```rust
pub fn on_value_invalid(mut self, on_value_invalid: impl Fn(SharedString, OTPFieldChangeDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_value_invalid`: `impl Fn(SharedString, OTPFieldChangeDetails, &mut Window, &mut App) + 'static`

Registers a callback invoked when value invalid occurs.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(OTPFieldRootStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(OTPFieldRootStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

OTPFieldRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `OTPFieldInput`

Text input integrated with the component's state and behavior.

[Source](../../src/otp_field/layers/otp_field_input.rs)

```rust
use base_gpui::otp_field::OTPFieldInput;

OTPFieldInput::new()
    .with_otp_field_context(/* context: OTPFieldContext */)
    .with_slot_index(0)
    .id("example-id")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a OTPFieldInput with its default configuration.

#### `.with_otp_field_context(...)`

```rust
pub fn with_otp_field_context(mut self, context: OTPFieldContext) -> Self
```

**Accepts**

- `context`: `OTPFieldContext`

Sets the with otp field context configuration for this part.

#### `.with_slot_index(...)`

```rust
pub fn with_slot_index(mut self, index: usize) -> Self
```

**Accepts**

- `index`: `usize`

Sets the with slot index configuration for this part.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(OTPFieldInputStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(OTPFieldInputStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

OTPFieldInput also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
