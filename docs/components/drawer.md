# Drawer

Drawer ports Base UI's Drawer component model to GPUI.

[Base UI reference](https://base-ui.com/react/components/drawer) · [Source](../../src/drawer/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::drawer::{
    DrawerBackdrop,
    DrawerContent,
    DrawerIndent,
    DrawerIndentBackground,
    DrawerPopup,
    DrawerPortal,
    DrawerProvider,
    DrawerRoot,
    DrawerSwipeArea,
    DrawerViewport,
};

DrawerRoot::new()
    .child(
        DrawerIndent::new(),
    )
    .child(
        DrawerIndentBackground::new(),
    )
    .child(
        DrawerPortal::new()
                .child(
                    DrawerBackdrop::new(),
                )
                .child(
                    DrawerViewport::new()
                                .child(
                                    DrawerPopup::new()
                                                    .child(
                                                        DrawerContent::new(),
                                                    ),
                                ),
                ),
    )
    .child(
        DrawerSwipeArea::new(),
    );
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `DrawerRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/drawer/layers/drawer_root.rs)

```rust
use base_gpui::drawer::DrawerRoot;

DrawerRoot::new()
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
    .swipe_direction(/* swipe_direction: DrawerSwipeDirection */)
    .snap_points(/* snap_points: Vec<DrawerSnapPoint> */)
    .snap_to_sequential_points(true)
    .default_snap_point(None)
    .snap_point(None)
    .on_snap_point_change(|/* callback arguments */| { /* handle change */ })
    .nested_in(/* reporter: DrawerNestedReporter */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a DrawerRoot with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<DrawerChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<DrawerChild<P>>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<DrawerChild<P>>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<DrawerChild<P>>>`

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

#### `.swipe_direction(...)`

```rust
pub fn swipe_direction(mut self, swipe_direction: DrawerSwipeDirection) -> Self
```

**Accepts**

- `swipe_direction`: `DrawerSwipeDirection`

Sets the swipe direction configuration for this part.

#### `.snap_points(...)`

```rust
pub fn snap_points(mut self, snap_points: Vec<DrawerSnapPoint>) -> Self
```

**Accepts**

- `snap_points`: `Vec<DrawerSnapPoint>`

Sets the snap points configuration for this part.

#### `.snap_to_sequential_points(...)`

```rust
pub fn snap_to_sequential_points(mut self, snap_to_sequential_points: bool) -> Self
```

**Accepts**

- `snap_to_sequential_points`: `bool`

Sets the snap to sequential points configuration for this part.

#### `.default_snap_point(...)`

```rust
pub fn default_snap_point(mut self, default_snap_point: Option<DrawerSnapPoint>) -> Self
```

**Accepts**

- `default_snap_point`: `Option<DrawerSnapPoint>`

Sets the initial snap point for uncontrolled state.

#### `.snap_point(...)`

```rust
pub fn snap_point(mut self, snap_point: Option<DrawerSnapPoint>) -> Self
```

**Accepts**

- `snap_point`: `Option<DrawerSnapPoint>`

Calling this builder marks the snap point controlled even when `None`.

#### `.on_snap_point_change(...)`

```rust
pub fn on_snap_point_change(mut self, on_snap_point_change: impl Fn(Option<DrawerSnapPoint>, &mut DrawerSnapPointChangeDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_snap_point_change`: `impl Fn(Option<DrawerSnapPoint>, &mut DrawerSnapPointChangeDetails, &mut Window, &mut App)
            + 'static`

Registers a callback invoked when snap point change occurs.

#### `.nested_in(...)`

```rust
pub fn nested_in(mut self, reporter: DrawerNestedReporter) -> Self
```

**Accepts**

- `reporter`: `DrawerNestedReporter`

Links this drawer as nested inside a parent drawer; obtain the reporter from the parent's `DrawerContext::nested_reporter()`.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(DialogRootStyleState<P>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(DialogRootStyleState<P>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

DrawerRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `DrawerPortal`

Hosts overlay content outside the normal child layout.

[Source](../../src/drawer/layers/drawer_portal.rs)

```rust
use base_gpui::drawer::DrawerPortal;

DrawerPortal::new()
    .keep_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a DrawerPortal with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<DrawerPortalChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<DrawerPortalChild<P>>`

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

DrawerPortal also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `DrawerPopup`

Contains the floating interactive content.

[Source](../../src/drawer/layers/drawer_popup.rs)

```rust
use base_gpui::drawer::DrawerPopup;

DrawerPopup::new()
    .id("example-id")
    .keep_mounted(true)
    .role(/* role: Role */)
    .aria_label("label")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a DrawerPopup with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<DrawerPopupChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<DrawerPopupChild<P>>`

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

Overrides the popup's accessibility role. Defaults to [`Role::Dialog`].

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Sets the popup's accessible name. gpui has no `aria-labelledby` builder, so pass the drawer title text explicitly here.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(DrawerPopupStyleState<P>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(DrawerPopupStyleState<P>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

DrawerPopup also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `DrawerBackdrop`

Covers content behind a modal overlay and handles outside interaction.

[Source](../../src/drawer/layers/drawer_backdrop.rs)

```rust
use base_gpui::drawer::DrawerBackdrop;

DrawerBackdrop::new()
    .keep_mounted(true)
    .force_render(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a DrawerBackdrop with its default configuration.

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

Renders a nested drawer's backdrop anyway (default `false`).

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(DrawerBackdropStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(DrawerBackdropStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

DrawerBackdrop also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `DrawerContent`

Contains the component's user-provided content.

[Source](../../src/drawer/layers/drawer_content.rs)

```rust
use base_gpui::drawer::DrawerContent;

DrawerContent::new()
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a DrawerContent with its default configuration.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(DrawerContentStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(DrawerContentStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

DrawerContent also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `DrawerIndent`

Public renderable part of the Drawer Indent component.

[Source](../../src/drawer/layers/drawer_indent.rs)

```rust
use base_gpui::drawer::DrawerIndent;

DrawerIndent::new()
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a DrawerIndent with its default configuration.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(DrawerIndentStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(DrawerIndentStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

DrawerIndent also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `DrawerIndentBackground`

Public renderable part of the Drawer Indent Background component.

[Source](../../src/drawer/layers/drawer_indent_background.rs)

```rust
use base_gpui::drawer::DrawerIndentBackground;

DrawerIndentBackground::new()
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a DrawerIndentBackground with its default configuration.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(DrawerIndentBackgroundStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(DrawerIndentBackgroundStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

DrawerIndentBackground also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `DrawerProvider`

Public renderable part of the Drawer Provider component.

[Source](../../src/drawer/layers/drawer_provider.rs)

```rust
use base_gpui::drawer::DrawerProvider;

DrawerProvider::new();
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a DrawerProvider with its default configuration.

DrawerProvider also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `DrawerSwipeArea`

Public renderable part of the Drawer Swipe Area component.

[Source](../../src/drawer/layers/drawer_swipe_area.rs)

```rust
use base_gpui::drawer::DrawerSwipeArea;

DrawerSwipeArea::new()
    .id("example-id")
    .disabled(true)
    .swipe_direction(/* swipe_direction: DrawerSwipeDirection */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a DrawerSwipeArea with its default configuration.

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

#### `.swipe_direction(...)`

```rust
pub fn swipe_direction(mut self, swipe_direction: DrawerSwipeDirection) -> Self
```

**Accepts**

- `swipe_direction`: `DrawerSwipeDirection`

Overrides the open direction (default: opposite of the root `swipe_direction`).

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(DrawerSwipeAreaStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(DrawerSwipeAreaStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

DrawerSwipeArea also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `DrawerViewport`

Defines the viewport used to lay out or constrain overlay content.

[Source](../../src/drawer/layers/drawer_viewport.rs)

```rust
use base_gpui::drawer::DrawerViewport;

DrawerViewport::new()
    .id("example-id")
    .keep_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a DrawerViewport with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<DrawerViewportChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<DrawerViewportChild<P>>`

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
pub fn style_with_state(mut self, style: impl Fn(DrawerViewportStyleState<P>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(DrawerViewportStyleState<P>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

DrawerViewport also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
