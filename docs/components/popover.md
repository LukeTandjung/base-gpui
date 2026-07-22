# Popover

Popover ports Base UI's Popover component model to GPUI.

[Base UI reference](https://base-ui.com/react/components/popover) · [Source](../../src/popover/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::popover::{
    PopoverArrow,
    PopoverBackdrop,
    PopoverClose,
    PopoverDescription,
    PopoverPopup,
    PopoverPortal,
    PopoverPositioner,
    PopoverRoot,
    PopoverTitle,
    PopoverTrigger,
    PopoverViewport,
};

PopoverRoot::new()
    .child(
        PopoverPortal::new()
                .child(
                    PopoverBackdrop::new(),
                )
                .child(
                    PopoverPositioner::new()
                                .child(
                                    PopoverArrow::new(),
                                )
                                .child(
                                    PopoverPopup::new()
                                                    .child(
                                                        PopoverClose::new(),
                                                    )
                                                    .child(
                                                        PopoverDescription::new(),
                                                    )
                                                    .child(
                                                        PopoverTitle::new(),
                                                    )
                                                    .child(
                                                        PopoverViewport::new(),
                                                    ),
                                ),
                ),
    )
    .child(
        PopoverTrigger::new(),
    );
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `PopoverRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/popover/layers/popover_root.rs)

```rust
use base_gpui::popover::PopoverRoot;

PopoverRoot::new()
    .id("example-id")
    .default_open(true)
    .open(true)
    .on_open_change(|/* callback arguments */| { /* handle change */ })
    .on_open_change_complete(|/* callback arguments */| { /* handle change */ })
    .modal(true)
    .trigger_id("example-id")
    .no_trigger_id()
    .default_trigger_id("example-id")
    .handle(/* handle: PopoverHandle<P> */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a PopoverRoot with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<PopoverChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<PopoverChild<P>>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<PopoverChild<P>>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<PopoverChild<P>>>`

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
pub fn on_open_change(mut self, on_open_change: impl Fn(bool, &mut PopoverOpenChangeDetails<P>, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_open_change`: `impl Fn(bool, &mut PopoverOpenChangeDetails<P>, &mut Window, &mut App) + 'static`

Registers a callback invoked when open change occurs.

#### `.on_open_change_complete(...)`

```rust
pub fn on_open_change_complete(mut self, on_open_change_complete: impl Fn(bool, &PopoverOpenChangeDetails<P>, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_open_change_complete`: `impl Fn(bool, &PopoverOpenChangeDetails<P>, &mut Window, &mut App)
            + 'static`

Registers a callback invoked when open change complete occurs.

#### `.modal(...)`

```rust
pub fn modal(mut self, modal: bool) -> Self
```

**Accepts**

- `modal`: `bool`

Controls whether the overlay behaves modally and blocks interaction outside it.

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
pub fn handle(mut self, handle: PopoverHandle<P>) -> Self
```

**Accepts**

- `handle`: `PopoverHandle<P>`

Sets the handle configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(PopoverRootStyleState<P>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(PopoverRootStyleState<P>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

PopoverRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `PopoverTrigger`

Interactive control that opens, closes, or activates the component.

[Source](../../src/popover/layers/popover_trigger.rs)

```rust
use base_gpui::popover::PopoverTrigger;

PopoverTrigger::new()
    .id("example-id")
    .disabled(true)
    .payload(/* payload: P */)
    .handle(/* handle: PopoverHandle<P> */)
    .open_on_hover(true)
    .delay(/* delay: Duration */)
    .close_delay(/* close_delay: Duration */)
    .aria_label("label")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a PopoverTrigger with its default configuration.

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
pub fn handle(mut self, handle: PopoverHandle<P>) -> Self
```

**Accepts**

- `handle`: `PopoverHandle<P>`

Sets the handle configuration for this part.

#### `.open_on_hover(...)`

```rust
pub fn open_on_hover(mut self, open_on_hover: bool) -> Self
```

**Accepts**

- `open_on_hover`: `bool`

Controls whether open on hover behavior is enabled.

#### `.delay(...)`

```rust
pub fn delay(mut self, delay: Duration) -> Self
```

**Accepts**

- `delay`: `Duration`

Sets the delay before the related transition or interaction begins.

#### `.close_delay(...)`

```rust
pub fn close_delay(mut self, close_delay: Duration) -> Self
```

**Accepts**

- `close_delay`: `Duration`

Sets the delay before an open component closes after the closing interaction.

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Accessible label for the trigger; set this when the visible child is not plain text. Any visible text that duplicates this label should be rendered with `Text::new_inaccessible(...)` to avoid double-announcing.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(PopoverTriggerStyleState<P>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(PopoverTriggerStyleState<P>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

PopoverTrigger also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `PopoverPortal`

Hosts overlay content outside the normal child layout.

[Source](../../src/popover/layers/popover_portal.rs)

```rust
use base_gpui::popover::PopoverPortal;

PopoverPortal::new()
    .keep_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a PopoverPortal with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<PopoverPortalChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<PopoverPortalChild<P>>`

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
pub fn style_with_state(mut self, style: impl Fn(PopoverPortalStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(PopoverPortalStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

PopoverPortal also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `PopoverPositioner`

Measures the anchor and positions floating content.

[Source](../../src/popover/layers/popover_positioner.rs)

```rust
use base_gpui::popover::PopoverPositioner;

PopoverPositioner::new()
    .side("example-id")
    .align(/* align: PopoverAlign */)
    .side_offset("example-id")
    .align_offset(/* align_offset: Pixels */)
    .collision_padding(/* collision_padding: Pixels */)
    .keep_mounted(true)
    .keep_mounted_from_portal()
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a PopoverPositioner with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<PopoverPositionerChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<PopoverPositionerChild<P>>`

Adds one typed child to this part.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.side(...)`

```rust
pub fn side(mut self, side: PopoverSide) -> Self
```

**Accepts**

- `side`: `PopoverSide`

Sets the preferred side of the anchor on which floating content appears.

#### `.align(...)`

```rust
pub fn align(mut self, align: PopoverAlign) -> Self
```

**Accepts**

- `align`: `PopoverAlign`

Sets floating content alignment along the selected side.

#### `.side_offset(...)`

```rust
pub fn side_offset(mut self, side_offset: Pixels) -> Self
```

**Accepts**

- `side_offset`: `Pixels`

Sets the distance between floating content and its anchor.

#### `.align_offset(...)`

```rust
pub fn align_offset(mut self, align_offset: Pixels) -> Self
```

**Accepts**

- `align_offset`: `Pixels`

Offsets floating content along its alignment axis.

#### `.collision_padding(...)`

```rust
pub fn collision_padding(mut self, collision_padding: Pixels) -> Self
```

**Accepts**

- `collision_padding`: `Pixels`

Sets the minimum space retained between floating content and viewport edges.

#### `.keep_mounted(...)`

```rust
pub fn keep_mounted(mut self, keep_mounted: bool) -> Self
```

**Accepts**

- `keep_mounted`: `bool`

Keeps the part mounted when inactive or closed so child state can be preserved.

#### `.keep_mounted_from_portal(...)`

```rust
pub fn keep_mounted_from_portal(mut self) -> Self
```

Sets the keep mounted from portal configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(PopoverPositionerStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(PopoverPositionerStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

PopoverPositioner also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `PopoverPopup`

Contains the floating interactive content.

[Source](../../src/popover/layers/popover_popup.rs)

```rust
use base_gpui::popover::PopoverPopup;

PopoverPopup::new()
    .id("example-id")
    .side("example-id")
    .align(/* align: PopoverAlign */)
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

Creates a PopoverPopup with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<PopoverPopupChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<PopoverPopupChild<P>>`

Adds one typed child to this part.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.side(...)`

```rust
pub fn side(mut self, side: PopoverSide) -> Self
```

**Accepts**

- `side`: `PopoverSide`

Sets the preferred side of the anchor on which floating content appears.

#### `.align(...)`

```rust
pub fn align(mut self, align: PopoverAlign) -> Self
```

**Accepts**

- `align`: `PopoverAlign`

Sets floating content alignment along the selected side.

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

Accessible label for the dialog popup. Until gpui supports `aria-labelledby` relationships, pass the same string rendered inside `PopoverTitle`.

#### `.payload_content(...)`

```rust
pub fn payload_content(mut self, content: impl Fn(Option<&P>, &mut Window, &mut App) -> gpui::AnyElement + 'static,) -> Self
```

**Accepts**

- `content`: `impl Fn(Option<&P>, &mut Window, &mut App) -> gpui::AnyElement + 'static,`

Sets the payload content configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(PopoverPopupStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(PopoverPopupStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

PopoverPopup also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `PopoverArrow`

Optional arrow that visually points toward the overlay anchor.

[Source](../../src/popover/layers/popover_arrow.rs)

```rust
use base_gpui::popover::PopoverArrow;

PopoverArrow::new()
    .side("example-id")
    .align(/* align: PopoverAlign */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a PopoverArrow with its default configuration.

#### `.side(...)`

```rust
pub fn side(mut self, side: PopoverSide) -> Self
```

**Accepts**

- `side`: `PopoverSide`

Sets the preferred side of the anchor on which floating content appears.

#### `.align(...)`

```rust
pub fn align(mut self, align: PopoverAlign) -> Self
```

**Accepts**

- `align`: `PopoverAlign`

Sets floating content alignment along the selected side.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(PopoverArrowStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(PopoverArrowStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

PopoverArrow also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `PopoverBackdrop`

Covers content behind a modal overlay and handles outside interaction.

[Source](../../src/popover/layers/popover_backdrop.rs)

```rust
use base_gpui::popover::PopoverBackdrop;

PopoverBackdrop::new()
    .keep_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a PopoverBackdrop with its default configuration.

#### `.keep_mounted(...)`

```rust
pub fn keep_mounted(mut self, keep_mounted: bool) -> Self
```

**Accepts**

- `keep_mounted`: `bool`

Keeps the part mounted when inactive or closed so child state can be preserved.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(PopoverBackdropStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(PopoverBackdropStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

PopoverBackdrop also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `PopoverClose`

Interactive control that closes the component.

[Source](../../src/popover/layers/popover_close.rs)

```rust
use base_gpui::popover::PopoverClose;

PopoverClose::new()
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

Creates a PopoverClose with its default configuration.

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

Accessible label for the close button; defaults to "Close". Any visible text that duplicates this label should be rendered with `Text::new_inaccessible(...)` to avoid double-announcing.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(PopoverCloseStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(PopoverCloseStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

PopoverClose also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `PopoverDescription`

Provides supporting descriptive content.

[Source](../../src/popover/layers/popover_description.rs)

```rust
use base_gpui::popover::PopoverDescription;

PopoverDescription::new()
    .id("example-id")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a PopoverDescription with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(PopoverDescriptionStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(PopoverDescriptionStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

PopoverDescription also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `PopoverTitle`

Provides the component's visible title.

[Source](../../src/popover/layers/popover_title.rs)

```rust
use base_gpui::popover::PopoverTitle;

PopoverTitle::new()
    .id("example-id")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a PopoverTitle with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(PopoverTitleStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(PopoverTitleStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

PopoverTitle also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `PopoverViewport`

Defines the viewport used to lay out or constrain overlay content.

[Source](../../src/popover/layers/popover_viewport.rs)

```rust
use base_gpui::popover::PopoverViewport;

PopoverViewport::new()
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a PopoverViewport with its default configuration.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(PopoverViewportStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(PopoverViewportStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

PopoverViewport also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
