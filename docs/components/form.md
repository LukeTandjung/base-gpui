# Form

GPUI-native port of Base UI `Form`.

[Base UI reference](https://base-ui.com/react/components/form) · [Source](../../src/form/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::form::{
    Form,
};

Form::new();
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `Form`

Public renderable part of the Form component.

[Source](../../src/form/layers/form.rs)

```rust
use base_gpui::form::Form;

Form::new()
    .id("example-id")
    .validation_mode("example-id")
    .errors(/* errors: FormErrors */)
    .aria_label("aria label")
    .on_form_submit(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a Form with its default configuration.

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

#### `.validation_mode(...)`

```rust
pub fn validation_mode(mut self, validation_mode: FieldValidationMode) -> Self
```

**Accepts**

- `validation_mode`: `FieldValidationMode`

Sets the validation mode configuration for this part.

#### `.errors(...)`

```rust
pub fn errors(mut self, errors: FormErrors) -> Self
```

**Accepts**

- `errors`: `FormErrors`

Sets the errors configuration for this part.

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self
```

**Accepts**

- `aria_label`: `impl Into<SharedString>`

Sets a literal accessible name for this part.

#### `.on_form_submit(...)`

```rust
pub fn on_form_submit(mut self, on_form_submit: impl Fn(FormValues, FormSubmitDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_form_submit`: `impl Fn(FormValues, FormSubmitDetails, &mut Window, &mut App) + 'static`

Registers a callback invoked when form submit occurs.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(FormStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(FormStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

Form also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
