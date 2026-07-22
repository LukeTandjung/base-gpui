# Input

Field-aware Base UI `Input` component.

[Base UI reference](https://base-ui.com/react/components/input) · [Source](../../src/input/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::input::{
    Input,
};

Input::new();
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `Input`

Text input integrated with the component's state and behavior.

[Source](../../src/input/layers/input.rs)

```rust
use base_gpui::input::Input;

Input::new()
    .with_field_context(/* context: FieldContext */)
    .id("example-id")
    .name("name")
    .value(/* value: impl Into<SharedString> */)
    .default_value(/* default_value: impl Into<SharedString> */)
    .aria_label("aria label")
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

Creates a Input with its default configuration.

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

#### `.aria_label(...)`

```rust
pub fn aria_label(self, aria_label: impl Into<SharedString>) -> Self
```

**Accepts**

- `aria_label`: `impl Into<SharedString>`

Sets the accessible name announced by assistive technology. There is no `aria-labelledby`-style id wiring in this gpui revision, so pass the visible label text here as well when using `FieldLabel`.  NOTE: not yet wired into the accessibility tree. `FieldControl` (in the `field` module) exposes no accessible-label API in this revision, and the `Div` handed to `style_with_state` is not stateful, so gpui's `StatefulInteractiveElement::aria_label` cannot be applied here. Tracked as a gap: `FieldControl` needs an `aria_label` builder that forwards to its stateful root element.

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

Overrides the input's focus handle so composite containers can own the control's roving focus handle.

#### `.on_edge_left(...)`

```rust
pub fn on_edge_left(mut self, on_edge_left: impl Fn(SharedString, &mut Window, &mut gpui::Context<InputRuntime>) -> bool + 'static,) -> Self
```

**Accepts**

- `on_edge_left`: `impl Fn(SharedString, &mut Window, &mut gpui::Context<InputRuntime>) -> bool
            + 'static,`

Consulted when a plain Left arrow is pressed with the caret at position 0 and no selection; returning `true` consumes the press.

#### `.on_edge_right(...)`

```rust
pub fn on_edge_right(mut self, on_edge_right: impl Fn(SharedString, &mut Window, &mut gpui::Context<InputRuntime>) -> bool + 'static,) -> Self
```

**Accepts**

- `on_edge_right`: `impl Fn(SharedString, &mut Window, &mut gpui::Context<InputRuntime>) -> bool
            + 'static,`

Consulted when a plain Right arrow is pressed with the caret at the end of the text and no selection; returning `true` consumes the press.

#### `.select_all_on_focus(...)`

```rust
pub fn select_all_on_focus(mut self, select_all_on_focus: bool) -> Self
```

**Accepts**

- `select_all_on_focus`: `bool`

Selects the whole text whenever the input gains focus.

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

Input also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
