# Dialog

Dialog component ported from Base UI.

[Base UI reference](https://base-ui.com/react/components/dialog) · [Source](../../src/dialog/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::dialog::{
    DialogBackdrop,
    DialogClose,
    DialogDescription,
    DialogPopup,
    DialogPortal,
    DialogRoot,
    DialogTitle,
    DialogTrigger,
    DialogViewport,
};

DialogRoot::new()
    .child(
        DialogPortal::new()
                .child(
                    DialogBackdrop::new(),
                )
                .child(
                    DialogViewport::new()
                                .child(
                                    DialogPopup::new()
                                                    .child(
                                                        DialogClose::new(),
                                                    )
                                                    .child(
                                                        DialogDescription::new(),
                                                    )
                                                    .child(
                                                        DialogTitle::new(),
                                                    ),
                                ),
                ),
    )
    .child(
        DialogTrigger::new(),
    );
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `DialogRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/dialog/layers/dialog_root.rs)

```rust
use base_gpui::dialog::DialogRoot;

DialogRoot::new()
    .id("example-id")
    .default_open(true)
    .open(true)
    .on_open_change(|/* callback arguments */| { /* handle change */ })
    .on_open_change_complete(|/* callback arguments */| { /* handle change */ })
    .modal(true)
    .modal_mode(/* modal_mode: DialogModalMode */)
    .trap_focus()
    .disable_pointer_dismissal(true)
    .trigger_id("example-id")
    .no_trigger_id()
    .default_trigger_id("example-id")
    .handle(/* handle: DialogHandle<P> */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a DialogRoot with its default configuration.

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

#### `.modal(...)`

```rust
pub fn modal(mut self, modal: bool) -> Self
```

**Accepts**

- `modal`: `bool`

Controls whether the overlay behaves modally and blocks interaction outside it.

#### `.modal_mode(...)`

```rust
pub fn modal_mode(mut self, modal_mode: DialogModalMode) -> Self
```

**Accepts**

- `modal_mode`: `DialogModalMode`

Sets the modal mode configuration for this part.

#### `.trap_focus(...)`

```rust
pub fn trap_focus(mut self) -> Self
```

Controls whether trap focus behavior is enabled.

#### `.disable_pointer_dismissal(...)`

```rust
pub fn disable_pointer_dismissal(mut self, disable_pointer_dismissal: bool) -> Self
```

**Accepts**

- `disable_pointer_dismissal`: `bool`

Controls whether disable pointer dismissal behavior is enabled.

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
pub fn handle(mut self, handle: DialogHandle<P>) -> Self
```

**Accepts**

- `handle`: `DialogHandle<P>`

Sets the handle configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(DialogRootStyleState<P>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(DialogRootStyleState<P>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

DialogRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `DialogTrigger`

Interactive control that opens, closes, or activates the component.

[Source](../../src/dialog/layers/dialog_trigger.rs)

```rust
use base_gpui::dialog::DialogTrigger;

DialogTrigger::new()
    .id("example-id")
    .disabled(true)
    .payload(/* payload: P */)
    .handle(/* handle: DialogHandle<P> */)
    .aria_label("label")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a DialogTrigger with its default configuration.

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

#### `.payload(...)`

```rust
pub fn payload(mut self, payload: P) -> Self
```

**Accepts**

- `payload`: `P`

Sets the payload configuration for this part.

#### `.handle(...)`

```rust
pub fn handle(mut self, handle: DialogHandle<P>) -> Self
```

**Accepts**

- `handle`: `DialogHandle<P>`

Sets the handle configuration for this part.

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Sets a literal accessible name for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(DialogTriggerStyleState<P>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(DialogTriggerStyleState<P>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

DialogTrigger also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `DialogPortal`

Hosts overlay content outside the normal child layout.

[Source](../../src/dialog/layers/dialog_portal.rs)

```rust
use base_gpui::dialog::DialogPortal;

DialogPortal::new()
    .keep_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a DialogPortal with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<DialogPortalChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<DialogPortalChild<P>>`

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

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(DialogPortalStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(DialogPortalStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

DialogPortal also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `DialogPopup`

Contains the floating interactive content.

[Source](../../src/dialog/layers/dialog_popup.rs)

```rust
use base_gpui::dialog::DialogPopup;

DialogPopup::new()
    .id("example-id")
    .keep_mounted(true)
    .role(/* role: Role */)
    .aria_label("label")
    .payload_content(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a DialogPopup with its default configuration.

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

#### `.role(...)`

```rust
pub fn role(mut self, role: Role) -> Self
```

**Accepts**

- `role`: `Role`

Overrides the popup's accessibility role. Defaults to [`Role::Dialog`]; Alert Dialog sets [`Role::AlertDialog`].

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Accessible name for the dialog. gpui has no `aria-labelledby` id-reference builder, so consumers pass the title string directly.

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

DialogPopup also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `DialogBackdrop`

Covers content behind a modal overlay and handles outside interaction.

[Source](../../src/dialog/layers/dialog_backdrop.rs)

```rust
use base_gpui::dialog::DialogBackdrop;

DialogBackdrop::new()
    .keep_mounted(true)
    .force_render(true)
    .force_rendered(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a DialogBackdrop with its default configuration.

#### `.keep_mounted(...)`

```rust
pub fn keep_mounted(mut self, keep_mounted: bool) -> Self
```

**Accepts**

- `keep_mounted`: `bool`

Keeps the part mounted when inactive or closed so child state can be preserved.

#### `.force_render(...)`

```rust
pub fn force_render(mut self, force_render: bool) -> Self
```

**Accepts**

- `force_render`: `bool`

Controls whether force render behavior is enabled.

#### `.force_rendered(...)`

```rust
pub fn force_rendered(self, force_rendered: bool) -> Self
```

**Accepts**

- `force_rendered`: `bool`

Controls whether force rendered behavior is enabled.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(DialogBackdropStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(DialogBackdropStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

DialogBackdrop also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `DialogClose`

Interactive control that closes the component.

[Source](../../src/dialog/layers/dialog_close.rs)

```rust
use base_gpui::dialog::DialogClose;

DialogClose::new()
    .id("example-id")
    .disabled(true)
    .aria_label("label")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a DialogClose with its default configuration.

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

Accessible name for the close button (typically icon-only, e.g. "Close").

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(DialogCloseStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(DialogCloseStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

DialogClose also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `DialogDescription`

Provides supporting descriptive content.

[Source](../../src/dialog/layers/dialog_description.rs)

```rust
use base_gpui::dialog::DialogDescription;

DialogDescription::new()
    .id("example-id")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a DialogDescription with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(DialogDescriptionStyleState<P>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(DialogDescriptionStyleState<P>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

DialogDescription also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `DialogTitle`

Provides the component's visible title.

[Source](../../src/dialog/layers/dialog_title.rs)

```rust
use base_gpui::dialog::DialogTitle;

DialogTitle::new()
    .id("example-id")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a DialogTitle with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(DialogTitleStyleState<P>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(DialogTitleStyleState<P>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

DialogTitle also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `DialogViewport`

Defines the viewport used to lay out or constrain overlay content.

[Source](../../src/dialog/layers/dialog_viewport.rs)

```rust
use base_gpui::dialog::DialogViewport;

DialogViewport::new()
    .keep_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a DialogViewport with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<DialogViewportChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<DialogViewportChild<P>>`

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

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(DialogViewportStyleState<P>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(DialogViewportStyleState<P>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

DialogViewport also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
