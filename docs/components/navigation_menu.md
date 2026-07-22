# Navigation Menu

Navigation Menu ports Base UI's Navigation Menu component model to GPUI.

[Base UI reference](https://base-ui.com/react/components/navigation-menu) · [Source](../../src/navigation_menu/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::navigation_menu::{
    NavigationMenuArrow,
    NavigationMenuBackdrop,
    NavigationMenuContent,
    NavigationMenuIcon,
    NavigationMenuItem,
    NavigationMenuLink,
    NavigationMenuList,
    NavigationMenuPopup,
    NavigationMenuPortal,
    NavigationMenuPositioner,
    NavigationMenuRoot,
    NavigationMenuTrigger,
    NavigationMenuViewport,
};

NavigationMenuRoot::new()
    .child(
        NavigationMenuList::new()
                .child(
                    NavigationMenuItem::new()
                                .child(
                                    NavigationMenuContent::new(),
                                )
                                .child(
                                    NavigationMenuTrigger::new()
                                                    .child(
                                                        NavigationMenuIcon::new(),
                                                    ),
                                ),
                )
                .child(
                    NavigationMenuLink::new(),
                ),
    )
    .child(
        NavigationMenuPortal::new()
                .child(
                    NavigationMenuBackdrop::new(),
                )
                .child(
                    NavigationMenuPositioner::new()
                                .child(
                                    NavigationMenuArrow::new(),
                                )
                                .child(
                                    NavigationMenuPopup::new()
                                                    .child(
                                                        NavigationMenuViewport::new(),
                                                    ),
                                ),
                ),
    );
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `NavigationMenuRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/navigation_menu/layers/navigation_menu_root.rs)

```rust
use base_gpui::navigation_menu::NavigationMenuRoot;

NavigationMenuRoot::new()
    .id("example-id")
    .default_value(None)
    .value(None)
    .delay(/* delay: Duration */)
    .close_delay(/* close_delay: Duration */)
    .orientation(/* orientation: NavigationMenuOrientation */)
    .nested(true)
    .aria_label("aria label")
    .on_value_change(|/* callback arguments */| { /* handle change */ })
    .on_open_change_complete(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a NavigationMenuRoot with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<NavigationMenuChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<NavigationMenuChild<T>>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<NavigationMenuChild<T>>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<NavigationMenuChild<T>>>`

Adds multiple typed children in iteration order.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.default_value(...)`

```rust
pub fn default_value(mut self, default_value: Option<T>) -> Self
```

**Accepts**

- `default_value`: `Option<T>`

Sets the initial value for uncontrolled state. Later user changes are retained by the component.

#### `.value(...)`

```rust
pub fn value(mut self, value: Option<T>) -> Self
```

**Accepts**

- `value`: `Option<T>`

Sets the current controlled value or the value represented by this part, depending on the part's role.

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

#### `.orientation(...)`

```rust
pub fn orientation(mut self, orientation: NavigationMenuOrientation) -> Self
```

**Accepts**

- `orientation`: `NavigationMenuOrientation`

Sets the component's horizontal or vertical orientation and corresponding keyboard behavior.

#### `.nested(...)`

```rust
pub fn nested(mut self, nested: bool) -> Self
```

**Accepts**

- `nested`: `bool`

Marks this root as nested inside another navigation menu's content (renders inline; style state reports `nested`).

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self
```

**Accepts**

- `aria_label`: `impl Into<SharedString>`

Accessible label for the navigation landmark (non-nested roots only; nested roots carry no role).

#### `.on_value_change(...)`

```rust
pub fn on_value_change(mut self, on_value_change: impl Fn(Option<T>, &mut NavigationMenuValueChangeDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_value_change`: `impl Fn(Option<T>, &mut NavigationMenuValueChangeDetails, &mut Window, &mut App)
            + 'static`

Registers a callback invoked when value change occurs.

#### `.on_open_change_complete(...)`

```rust
pub fn on_open_change_complete(mut self, on_open_change_complete: impl Fn(Option<T>, &NavigationMenuValueChangeDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_open_change_complete`: `impl Fn(Option<T>, &NavigationMenuValueChangeDetails, &mut Window, &mut App)
            + 'static`

Registers a callback invoked when open change complete occurs.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(NavigationMenuRootStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(NavigationMenuRootStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

NavigationMenuRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `NavigationMenuTrigger`

Interactive control that opens, closes, or activates the component.

[Source](../../src/navigation_menu/layers/navigation_menu_trigger.rs)

```rust
use base_gpui::navigation_menu::NavigationMenuTrigger;

NavigationMenuTrigger::new()
    .disabled(true)
    .is_disabled()
    .aria_label("aria label")
    .style_with_state(|/* callback arguments */| { /* handle change */ })
    .wired(/* value: T */, /* focus_handle: FocusHandle */, 0, 0);
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a NavigationMenuTrigger with its default configuration.

#### `.disabled(...)`

```rust
pub fn disabled(mut self, disabled: bool) -> Self
```

**Accepts**

- `disabled`: `bool`

When true, prevents user interaction with this part.

#### `.is_disabled(...)`

```rust
pub fn is_disabled(&self) -> bool
```

Controls whether disabled behavior is enabled.

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self
```

**Accepts**

- `aria_label`: `impl Into<SharedString>`

Accessible name for the trigger button; pass one when the visible caption is not plain accessible text (no id-reference labelling exists). Render the visible caption with `Text::new_inaccessible(...)` when set, to avoid double announcement.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<NavigationMenuTriggerChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<NavigationMenuTriggerChild<T>>`

Adds one typed child to this part.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(NavigationMenuTriggerStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(NavigationMenuTriggerStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

#### `.wired(...)`

```rust
pub fn wired(mut self, value: T, focus_handle: FocusHandle, entry_index: usize, order: usize,) -> Self
```

**Accepts**

- `value`: `T`
- `focus_handle`: `FocusHandle`
- `entry_index`: `usize`
- `order`: `usize`

Internal wiring seam: the enclosing item hands the trigger its value, focus handle, roving entry index, and order.

NavigationMenuTrigger also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `NavigationMenuPortal`

Hosts overlay content outside the normal child layout.

[Source](../../src/navigation_menu/layers/navigation_menu_portal.rs)

```rust
use base_gpui::navigation_menu::NavigationMenuPortal;

NavigationMenuPortal::new()
    .keep_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a NavigationMenuPortal with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<NavigationMenuPortalChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<NavigationMenuPortalChild<T>>`

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
pub fn style_with_state(mut self, style: impl Fn(NavigationMenuPortalStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(NavigationMenuPortalStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

NavigationMenuPortal also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `NavigationMenuPositioner`

Measures the anchor and positions floating content.

[Source](../../src/navigation_menu/layers/navigation_menu_positioner.rs)

```rust
use base_gpui::navigation_menu::NavigationMenuPositioner;

NavigationMenuPositioner::new()
    .side("example-id")
    .align(/* align: NavigationMenuAlign */)
    .side_offset("example-id")
    .align_offset(/* align_offset: Pixels */)
    .collision_padding(/* collision_padding: Pixels */)
    .arrow_padding(/* arrow_padding: Pixels */)
    .keep_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a NavigationMenuPositioner with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<NavigationMenuPositionerChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<NavigationMenuPositionerChild<T>>`

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
pub fn side(mut self, side: NavigationMenuSide) -> Self
```

**Accepts**

- `side`: `NavigationMenuSide`

Sets the preferred side of the anchor on which floating content appears.

#### `.align(...)`

```rust
pub fn align(mut self, align: NavigationMenuAlign) -> Self
```

**Accepts**

- `align`: `NavigationMenuAlign`

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

#### `.arrow_padding(...)`

```rust
pub fn arrow_padding(mut self, arrow_padding: Pixels) -> Self
```

**Accepts**

- `arrow_padding`: `Pixels`

Sets the arrow padding configuration for this part.

#### `.keep_mounted(...)`

```rust
pub fn keep_mounted(mut self, keep_mounted: bool) -> Self
```

**Accepts**

- `keep_mounted`: `bool`

Keeps the part mounted when inactive or closed so child state can be preserved.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(NavigationMenuPositionerStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(NavigationMenuPositionerStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

NavigationMenuPositioner also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `NavigationMenuPopup`

Contains the floating interactive content.

[Source](../../src/navigation_menu/layers/navigation_menu_popup.rs)

```rust
use base_gpui::navigation_menu::NavigationMenuPopup;

NavigationMenuPopup::new()
    .side("example-id")
    .align(/* align: NavigationMenuAlign */)
    .aria_label("aria label")
    .keep_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a NavigationMenuPopup with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<NavigationMenuPopupChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<NavigationMenuPopupChild<T>>`

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
pub fn side(mut self, side: NavigationMenuSide) -> Self
```

**Accepts**

- `side`: `NavigationMenuSide`

Sets the preferred side of the anchor on which floating content appears.

#### `.align(...)`

```rust
pub fn align(mut self, align: NavigationMenuAlign) -> Self
```

**Accepts**

- `align`: `NavigationMenuAlign`

Sets floating content alignment along the selected side.

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self
```

**Accepts**

- `aria_label`: `impl Into<SharedString>`

Accessible label for the popup group; use the same text as the root's `.aria_label(...)`.

#### `.keep_mounted(...)`

```rust
pub fn keep_mounted(mut self, keep_mounted: bool) -> Self
```

**Accepts**

- `keep_mounted`: `bool`

Keeps the part mounted when inactive or closed so child state can be preserved.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(NavigationMenuPopupStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(NavigationMenuPopupStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

NavigationMenuPopup also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `NavigationMenuArrow`

Optional arrow that visually points toward the overlay anchor.

[Source](../../src/navigation_menu/layers/navigation_menu_arrow.rs)

```rust
use base_gpui::navigation_menu::NavigationMenuArrow;

NavigationMenuArrow::new()
    .side("example-id")
    .align(/* align: NavigationMenuAlign */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a NavigationMenuArrow with its default configuration.

#### `.side(...)`

```rust
pub fn side(mut self, side: NavigationMenuSide) -> Self
```

**Accepts**

- `side`: `NavigationMenuSide`

Sets the preferred side of the anchor on which floating content appears.

#### `.align(...)`

```rust
pub fn align(mut self, align: NavigationMenuAlign) -> Self
```

**Accepts**

- `align`: `NavigationMenuAlign`

Sets floating content alignment along the selected side.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(NavigationMenuArrowStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(NavigationMenuArrowStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

NavigationMenuArrow also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `NavigationMenuBackdrop`

Covers content behind a modal overlay and handles outside interaction.

[Source](../../src/navigation_menu/layers/navigation_menu_backdrop.rs)

```rust
use base_gpui::navigation_menu::NavigationMenuBackdrop;

NavigationMenuBackdrop::new()
    .keep_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a NavigationMenuBackdrop with its default configuration.

#### `.keep_mounted(...)`

```rust
pub fn keep_mounted(mut self, keep_mounted: bool) -> Self
```

**Accepts**

- `keep_mounted`: `bool`

Keeps the part mounted when inactive or closed so child state can be preserved.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(NavigationMenuBackdropStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(NavigationMenuBackdropStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

NavigationMenuBackdrop also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `NavigationMenuContent`

Contains the component's user-provided content.

[Source](../../src/navigation_menu/layers/navigation_menu_content.rs)

```rust
use base_gpui::navigation_menu::NavigationMenuContent;

NavigationMenuContent::new()
    .keep_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ })
    .wired(/* value: T */)
    .content_value();
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a NavigationMenuContent with its default configuration.

#### `.keep_mounted(...)`

```rust
pub fn keep_mounted(mut self, keep_mounted: bool) -> Self
```

**Accepts**

- `keep_mounted`: `bool`

Keeps the part mounted when inactive or closed so child state can be preserved.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(NavigationMenuContentStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(NavigationMenuContentStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

#### `.wired(...)`

```rust
pub fn wired(mut self, value: T) -> Self
```

**Accepts**

- `value`: `T`

Internal wiring seam: the enclosing item hands the content its value.

#### `.content_value(...)`

```rust
pub fn content_value(&self) -> Option<&T>
```

Sets the content value configuration for this part.

NavigationMenuContent also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `NavigationMenuIcon`

Optional visual icon for the component.

[Source](../../src/navigation_menu/layers/navigation_menu_icon.rs)

```rust
use base_gpui::navigation_menu::NavigationMenuIcon;

NavigationMenuIcon::new()
    .with_value(/* value: T */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a NavigationMenuIcon with its default configuration.

#### `.with_value(...)`

```rust
pub fn with_value(mut self, value: T) -> Self
```

**Accepts**

- `value`: `T`

Internal wiring seam: the enclosing trigger hands the icon its item value.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(NavigationMenuIconStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(NavigationMenuIconStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

NavigationMenuIcon also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `NavigationMenuItem`

Represents one interactive item in the component's collection.

[Source](../../src/navigation_menu/layers/navigation_menu_item.rs)

```rust
use base_gpui::navigation_menu::NavigationMenuItem;

NavigationMenuItem::new()
    .value(/* value: T */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a NavigationMenuItem with its default configuration.

#### `.value(...)`

```rust
pub fn value(mut self, value: T) -> Self
```

**Accepts**

- `value`: `T`

Required: the item's value. Items without a value are not registered.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<NavigationMenuItemChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<NavigationMenuItemChild<T>>`

Adds one typed child to this part.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(NavigationMenuItemStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(NavigationMenuItemStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

NavigationMenuItem also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `NavigationMenuLink`

Public renderable part of the Navigation Menu Link component.

[Source](../../src/navigation_menu/layers/navigation_menu_link.rs)

```rust
use base_gpui::navigation_menu::NavigationMenuLink;

NavigationMenuLink::new()
    .active(true)
    .close_on_click(true)
    .aria_label("aria label")
    .on_activate(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a NavigationMenuLink with its default configuration.

#### `.active(...)`

```rust
pub fn active(mut self, active: bool) -> Self
```

**Accepts**

- `active`: `bool`

Sets the active configuration for this part.

#### `.close_on_click(...)`

```rust
pub fn close_on_click(mut self, close_on_click: bool) -> Self
```

**Accepts**

- `close_on_click`: `bool`

Controls whether close on click behavior is enabled.

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self
```

**Accepts**

- `aria_label`: `impl Into<SharedString>`

Accessible name for the link; pass one when the visible caption is not plain accessible text (no id-reference labelling exists). Render the visible caption with `Text::new_inaccessible(...)` when set, to avoid double announcement.

#### `.on_activate(...)`

```rust
pub fn on_activate(mut self, on_activate: impl Fn(&mut Window, &mut App) + 'static) -> Self
```

**Accepts**

- `on_activate`: `impl Fn(&mut Window, &mut App) + 'static`

Registers a callback invoked when activate occurs.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(NavigationMenuLinkStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(NavigationMenuLinkStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

NavigationMenuLink also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `NavigationMenuList`

Contains and coordinates the component's collection items.

[Source](../../src/navigation_menu/layers/navigation_menu_list.rs)

```rust
use base_gpui::navigation_menu::NavigationMenuList;

NavigationMenuList::new()
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a NavigationMenuList with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<NavigationMenuListChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<NavigationMenuListChild<T>>`

Adds one typed child to this part.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(NavigationMenuListStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(NavigationMenuListStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

NavigationMenuList also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `NavigationMenuViewport`

Defines the viewport used to lay out or constrain overlay content.

[Source](../../src/navigation_menu/layers/navigation_menu_viewport.rs)

```rust
use base_gpui::navigation_menu::NavigationMenuViewport;

NavigationMenuViewport::new()
    .with_contents(/* contents: Vec<NavigationMenuContent<T>> */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a NavigationMenuViewport with its default configuration.

#### `.with_contents(...)`

```rust
pub fn with_contents(mut self, contents: Vec<NavigationMenuContent<T>>) -> Self
```

**Accepts**

- `contents`: `Vec<NavigationMenuContent<T>>`

Internal wiring seam: receives the contents collected from items.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(NavigationMenuViewportStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(NavigationMenuViewportStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

NavigationMenuViewport also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
