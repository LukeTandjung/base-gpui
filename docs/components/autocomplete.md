# Autocomplete

Autocomplete ports Base UI's Autocomplete component model to GPUI.

[Base UI reference](https://base-ui.com/react/components/autocomplete) · [Source](../../src/autocomplete/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::autocomplete::{
    AutocompleteRoot,
    AutocompleteValue,
};

AutocompleteRoot::new()
    .child(
        AutocompleteValue::new(),
    );
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `AutocompleteRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/autocomplete/layers/autocomplete_root.rs)

```rust
use base_gpui::autocomplete::AutocompleteRoot;

AutocompleteRoot::new()
    .id("example-id")
    .name("name")
    .mode(/* mode: AutocompleteMode */)
    .default_value(/* default_value: impl Into<SharedString> */)
    .value(/* value: impl Into<SharedString> */)
    .on_value_change(|/* callback arguments */| { /* handle change */ })
    .default_open(true)
    .open(true)
    .on_open_change(|/* callback arguments */| { /* handle change */ })
    .on_item_highlighted(|/* callback arguments */| { /* handle change */ })
    .disabled(true)
    .read_only(true)
    .required(true)
    .open_on_input_click(true)
    .auto_highlight(/* auto_highlight: ComboboxAutoHighlight */)
    .keep_highlight(true)
    .highlight_item_on_hover(true)
    .submit_on_item_click(true)
    .loop_focus(true)
    .limit(0)
    .filter(|/* callback arguments */| { /* handle change */ })
    .item_to_string_value(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a AutocompleteRoot with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<AutocompleteChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<AutocompleteChild<T>>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<AutocompleteChild<T>>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<AutocompleteChild<T>>>`

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

#### `.mode(...)`

```rust
pub fn mode(mut self, mode: AutocompleteMode) -> Self
```

**Accepts**

- `mode`: `AutocompleteMode`

Sets the mode configuration for this part.

#### `.default_value(...)`

```rust
pub fn default_value(mut self, default_value: impl Into<SharedString>) -> Self
```

**Accepts**

- `default_value`: `impl Into<SharedString>`

The Autocomplete value axis is the input text (Base UI maps `defaultValue` to Combobox `defaultInputValue`).

#### `.value(...)`

```rust
pub fn value(mut self, value: impl Into<SharedString>) -> Self
```

**Accepts**

- `value`: `impl Into<SharedString>`

Controlled input text (Base UI `value` → Combobox `inputValue`).

#### `.on_value_change(...)`

```rust
pub fn on_value_change(mut self, on_value_change: impl Fn(&SharedString, &mut ComboboxChangeDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_value_change`: `impl Fn(&SharedString, &mut ComboboxChangeDetails, &mut Window, &mut App)
            + 'static`

Registers a callback invoked when value change occurs.

#### `.default_open(...)`

```rust
pub fn default_open(mut self, default_open: bool) -> Self
```

**Accepts**

- `default_open`: `bool`

Sets the initial open state for an uncontrolled component.

#### `.open(...)`

```rust
pub fn open(mut self, open: bool) -> Self
```

**Accepts**

- `open`: `bool`

Controls whether the component is open.

#### `.on_open_change(...)`

```rust
pub fn on_open_change(mut self, on_open_change: impl Fn(bool, &mut ComboboxChangeDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_open_change`: `impl Fn(bool, &mut ComboboxChangeDetails, &mut Window, &mut App) + 'static`

Registers a callback invoked when open change occurs.

#### `.on_item_highlighted(...)`

```rust
pub fn on_item_highlighted(mut self, on_item_highlighted: impl Fn(Option<&T>, &ComboboxItemHighlightDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_item_highlighted`: `impl Fn(Option<&T>, &ComboboxItemHighlightDetails, &mut Window, &mut App)
            + 'static`

Registers a callback invoked when item highlighted occurs.

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

#### `.open_on_input_click(...)`

```rust
pub fn open_on_input_click(mut self, open_on_input_click: bool) -> Self
```

**Accepts**

- `open_on_input_click`: `bool`

Controls whether open on input click behavior is enabled.

#### `.auto_highlight(...)`

```rust
pub fn auto_highlight(mut self, auto_highlight: ComboboxAutoHighlight) -> Self
```

**Accepts**

- `auto_highlight`: `ComboboxAutoHighlight`

Controls whether auto highlight behavior is enabled.

#### `.keep_highlight(...)`

```rust
pub fn keep_highlight(mut self, keep_highlight: bool) -> Self
```

**Accepts**

- `keep_highlight`: `bool`

Sets the keep highlight configuration for this part.

#### `.highlight_item_on_hover(...)`

```rust
pub fn highlight_item_on_hover(mut self, highlight_item_on_hover: bool) -> Self
```

**Accepts**

- `highlight_item_on_hover`: `bool`

Controls whether highlight item on hover behavior is enabled.

#### `.submit_on_item_click(...)`

```rust
pub fn submit_on_item_click(mut self, submit_on_item_click: bool) -> Self
```

**Accepts**

- `submit_on_item_click`: `bool`

Documented no-op hook until Form exposes programmatic submit.

#### `.loop_focus(...)`

```rust
pub fn loop_focus(mut self, loop_focus: bool) -> Self
```

**Accepts**

- `loop_focus`: `bool`

Controls whether keyboard focus wraps from the last enabled item to the first and vice versa.

#### `.limit(...)`

```rust
pub fn limit(mut self, limit: usize) -> Self
```

**Accepts**

- `limit`: `usize`

Sets the limit configuration for this part.

#### `.filter(...)`

```rust
pub fn filter(mut self, filter: impl Fn(&T, Option<&SharedString>, &str) -> bool + 'static,) -> Self
```

**Accepts**

- `filter`: `impl Fn(&T, Option<&SharedString>, &str) -> bool + 'static,`

Custom filter replacing the default case-insensitive contains match (only applies in `List`/`Both` modes).

#### `.item_to_string_value(...)`

```rust
pub fn item_to_string_value(mut self, resolver: impl Fn(&T) -> SharedString + 'static) -> Self
```

**Accepts**

- `resolver`: `impl Fn(&T) -> SharedString + 'static`

Feeds display, filtering, fill-on-press, and Field serialization (Base UI `itemToStringValue` → Combobox `itemToStringLabel`).

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ComboboxRootStyleState<T>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ComboboxRootStyleState<T>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

AutocompleteRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `AutocompleteValue`

Displays or edits the component's current value.

[Source](../../src/autocomplete/layers/autocomplete_value.rs)

```rust
use base_gpui::autocomplete::AutocompleteValue;

AutocompleteValue::new()
    .with_context(/* context: ComboboxContext<T> */)
    .formatter(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a AutocompleteValue with its default configuration.

#### `.with_context(...)`

```rust
pub fn with_context(mut self, context: ComboboxContext<T>) -> Self
```

**Accepts**

- `context`: `ComboboxContext<T>`

Attaches the root context; called by `AutocompleteRoot` during child routing.

#### `.formatter(...)`

```rust
pub fn formatter(mut self, formatter: impl Fn(&str) -> SharedString + 'static) -> Self
```

**Accepts**

- `formatter`: `impl Fn(&str) -> SharedString + 'static`

Formatter closure over the current value; ignored when static children are present.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Static children take precedence over the raw value (Base UI children fallback order).

AutocompleteValue also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
