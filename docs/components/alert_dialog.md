# Alert Dialog

Alert Dialog component ported from Base UI, composing the Dialog module.

[Base UI reference](https://base-ui.com/react/components/alert-dialog) · [Source](../../src/alert_dialog/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::alert_dialog::{
    AlertDialogPopup,
    AlertDialogRoot,
    AlertDialogTrigger,
};

AlertDialogRoot::new()
    .child(
        AlertDialogPopup::new(),
    )
    .child(
        AlertDialogTrigger::new(),
    );
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `AlertDialogRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/alert_dialog/layers/alert_dialog_root.rs)

```rust
use base_gpui::alert_dialog::AlertDialogRoot;

AlertDialogRoot::new()
    .id("example-id")
    .default_open(true)
    .open(true)
    .on_open_change(|/* callback arguments */| { /* handle change */ })
    .on_open_change_complete(|/* callback arguments */| { /* handle change */ })
    .trigger_id("example-id")
    .no_trigger_id()
    .default_trigger_id("example-id")
    .handle(/* handle: AlertDialogHandle<P> */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a AlertDialogRoot with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<DialogChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<DialogChild<P>>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<DialogChild<P>>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<DialogChild<P>>>`

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
pub fn on_open_change(mut self, on_open_change: impl Fn(bool, &mut DialogOpenChangeDetails<P>, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_open_change`: `impl Fn(bool, &mut DialogOpenChangeDetails<P>, &mut Window, &mut App) + 'static`

Registers a callback invoked when open change occurs.

#### `.on_open_change_complete(...)`

```rust
pub fn on_open_change_complete(mut self, on_open_change_complete: impl Fn(bool, &DialogOpenChangeDetails<P>, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_open_change_complete`: `impl Fn(bool, &DialogOpenChangeDetails<P>, &mut Window, &mut App)
            + 'static`

Registers a callback invoked when open change complete occurs.

#### `.trigger_id(...)`

```rust
pub fn trigger_id(mut self, trigger_id: impl Into<ElementId>) -> Self
```

**Accepts**

- `trigger_id`: `impl Into<ElementId>`

Sets the trigger id configuration for this part.

#### `.no_trigger_id(...)`

```rust
pub fn no_trigger_id(mut self) -> Self
```

Sets the no trigger id configuration for this part.

#### `.default_trigger_id(...)`

```rust
pub fn default_trigger_id(mut self, trigger_id: impl Into<ElementId>) -> Self
```

**Accepts**

- `trigger_id`: `impl Into<ElementId>`

Sets the initial trigger id for uncontrolled state.

#### `.handle(...)`

```rust
pub fn handle(mut self, handle: AlertDialogHandle<P>) -> Self
```

**Accepts**

- `handle`: `AlertDialogHandle<P>`

Sets the handle configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(DialogRootStyleState<P>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(DialogRootStyleState<P>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

AlertDialogRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `AlertDialogTrigger`

Interactive control that opens, closes, or activates the component.

[Source](../../src/alert_dialog/layers/alert_dialog_trigger.rs)

```rust
use base_gpui::alert_dialog::AlertDialogTrigger;

AlertDialogTrigger::new()
    .id("example-id")
    .disabled(true)
    .aria_label("label")
    .payload(/* payload: P */)
    .handle(/* handle: AlertDialogHandle<P> */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a AlertDialogTrigger with its default configuration.

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
pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Accessible name for the trigger, used when the visible child is an icon or other non-text content. Forwarded to the underlying Dialog trigger's `aria_label`.

#### `.payload(...)`

```rust
pub fn payload(mut self, payload: P) -> Self
```

**Accepts**

- `payload`: `P`

Sets the payload configuration for this part.

#### `.handle(...)`

```rust
pub fn handle(mut self, handle: AlertDialogHandle<P>) -> Self
```

**Accepts**

- `handle`: `AlertDialogHandle<P>`

Sets the handle configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(DialogTriggerStyleState<P>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(DialogTriggerStyleState<P>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

AlertDialogTrigger also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `AlertDialogPopup`

Contains the floating interactive content.

[Source](../../src/alert_dialog/layers/alert_dialog_popup.rs)

```rust
use base_gpui::alert_dialog::AlertDialogPopup;

AlertDialogPopup::new()
    .id("example-id")
    .keep_mounted(true)
    .aria_label("label")
    .payload_content(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a AlertDialogPopup with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<DialogPopupChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<DialogPopupChild<P>>`

Adds one typed child to this part.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.keep_mounted(...)`

```rust
pub fn keep_mounted(mut self, keep_mounted: bool) -> Self
```

**Accepts**

- `keep_mounted`: `bool`

Keeps the part mounted when inactive or closed so child state can be preserved.

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Accessible name for the alert dialog. gpui has no `aria-labelledby` id-reference builder, so consumers pass the title string directly.

#### `.payload_content(...)`

```rust
pub fn payload_content(mut self, content: impl Fn(Option<&P>, &mut Window, &mut App) -> AnyElement + 'static,) -> Self
```

**Accepts**

- `content`: `impl Fn(Option<&P>, &mut Window, &mut App) -> AnyElement + 'static,`

Sets the payload content configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(DialogPopupStyleState<P>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(DialogPopupStyleState<P>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

AlertDialogPopup also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
