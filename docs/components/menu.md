# Menu

The menu family sets `Role::Button`/`Role::MenuItem` on triggers, `Role::Menu` on popups, `Role::MenuItem{,CheckBox,Radio}` on items, and `Role::Group` on (radio) groups. Backdrop, arrow, indicators, group label, portal, positioner, and the structural roots carry no role and stay out of the accessibility tree (mirroring Base UI's `role="presentation"` / `aria-hidden`). Gaps in the pinned gpui revision, omitted rather than faked (blocked on upstream builders): - `aria-haspopup="menu"` / `aria-controls` on triggers and submenu triggers: `aria_expanded` plus the popup's `Role::Menu` stand in. - `aria-labelledby`: replaced by literal-string `.aria_label(...)` sourced from the registered group-label metadata. - `disabled`/`aria-disabled`: disabled parts are inert (withheld tab stops, activation no-ops) but the disabled state is not announced.

[Base UI reference](https://base-ui.com/react/components/menu) · [Source](../../src/menu/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::menu::{
    MenuArrow,
    MenuBackdrop,
    MenuCheckboxItem,
    MenuCheckboxItemIndicator,
    MenuGroup,
    MenuGroupLabel,
    MenuItem,
    MenuLinkItem,
    MenuPopup,
    MenuPortal,
    MenuPositioner,
    MenuRadioGroup,
    MenuRadioItem,
    MenuRadioItemIndicator,
    MenuRoot,
    MenuSeparator,
    MenuSubmenuRoot,
    MenuSubmenuTrigger,
    MenuTrigger,
};

MenuRoot::new()
    .child(
        MenuRadioGroup::new()
                .child(
                    MenuGroupLabel::new(),
                )
                .child(
                    MenuRadioItem::new()
                                .child(
                                    MenuRadioItemIndicator::new(),
                                ),
                )
                .child(
                    MenuSeparator::new(),
                ),
    )
    .child(
        MenuTrigger::new(),
    );
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `MenuRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/menu/layers/menu_root.rs)

```rust
use base_gpui::menu::MenuRoot;

MenuRoot::new()
    .id("example-id")
    .default_open(true)
    .open(true)
    .trigger_id("example-id")
    .default_trigger_id("example-id")
    .disabled(true)
    .modal(true)
    .loop_focus(true)
    .orientation(/* orientation: MenuOrientation */)
    .close_parent_on_esc(true)
    .highlight_item_on_hover(true)
    .on_open_change(|/* callback arguments */| { /* handle change */ })
    .on_open_change_complete(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ })
    .context_menu_parent()
    .menubar_link(/* link: MenuMenubarLink */)
    .is_disabled();
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a MenuRoot with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<MenuChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<MenuChild<P>>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<MenuChild<P>>>) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<MenuChild<P>>>`

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

#### `.trigger_id(...)`

```rust
pub fn trigger_id(mut self, trigger_id: impl Into<ElementId>) -> Self
```

**Accepts**

- `trigger_id`: `impl Into<ElementId>`

Sets the trigger id configuration for this part.

#### `.default_trigger_id(...)`

```rust
pub fn default_trigger_id(mut self, trigger_id: impl Into<ElementId>) -> Self
```

**Accepts**

- `trigger_id`: `impl Into<ElementId>`

Sets the initial trigger id for uncontrolled state.

#### `.disabled(...)`

```rust
pub fn disabled(mut self, disabled: bool) -> Self
```

**Accepts**

- `disabled`: `bool`

When true, prevents user interaction with this part.

#### `.modal(...)`

```rust
pub fn modal(mut self, modal: bool) -> Self
```

**Accepts**

- `modal`: `bool`

Controls whether the overlay behaves modally and blocks interaction outside it.

#### `.loop_focus(...)`

```rust
pub fn loop_focus(mut self, loop_focus: bool) -> Self
```

**Accepts**

- `loop_focus`: `bool`

Controls whether keyboard focus wraps from the last enabled item to the first and vice versa.

#### `.orientation(...)`

```rust
pub fn orientation(mut self, orientation: MenuOrientation) -> Self
```

**Accepts**

- `orientation`: `MenuOrientation`

Sets the component's horizontal or vertical orientation and corresponding keyboard behavior.

#### `.close_parent_on_esc(...)`

```rust
pub fn close_parent_on_esc(mut self, close_parent_on_esc: bool) -> Self
```

**Accepts**

- `close_parent_on_esc`: `bool`

Controls whether close parent on esc behavior is enabled.

#### `.highlight_item_on_hover(...)`

```rust
pub fn highlight_item_on_hover(mut self, highlight_item_on_hover: bool) -> Self
```

**Accepts**

- `highlight_item_on_hover`: `bool`

Controls whether highlight item on hover behavior is enabled.

#### `.on_open_change(...)`

```rust
pub fn on_open_change(mut self, on_open_change: impl Fn(bool, &mut MenuOpenChangeDetails<P>, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_open_change`: `impl Fn(bool, &mut MenuOpenChangeDetails<P>, &mut Window, &mut App) + 'static`

Registers a callback invoked when open change occurs.

#### `.on_open_change_complete(...)`

```rust
pub fn on_open_change_complete(mut self, on_open_change_complete: impl Fn(bool, &MenuOpenChangeDetails<P>, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_open_change_complete`: `impl Fn(bool, &MenuOpenChangeDetails<P>, &mut Window, &mut App)
            + 'static`

Registers a callback invoked when open change complete occurs.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(MenuRootStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(MenuRootStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

#### `.context_menu_parent(...)`

```rust
pub fn context_menu_parent(mut self) -> Self
```

Wiring seam used by `ContextMenuRoot`: types this menu's parent as a context menu, activating the cursor-anchor, always-modal, and grace branches inside the Menu layers.

#### `.menubar_link(...)`

```rust
pub fn menubar_link(mut self, link: MenuMenubarLink) -> Self
```

**Accepts**

- `link`: `MenuMenubarLink`

Wiring seam used by the Menubar's child wiring: injects the menubar link that makes this menu a menubar-parented menu.

#### `.is_disabled(...)`

```rust
pub fn is_disabled(&self) -> bool
```

The menu-level disabled fact, read by menubar child wiring for the initial trigger slot metadata.

MenuRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `MenuSubmenuRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/menu/layers/menu_submenu_root.rs)

```rust
use base_gpui::menu::MenuSubmenuRoot;

MenuSubmenuRoot::new()
    .id("example-id")
    .default_open(true)
    .open(true)
    .disabled(true)
    .loop_focus(true)
    .orientation(/* orientation: MenuOrientation */)
    .close_parent_on_esc(true)
    .highlight_item_on_hover(true)
    .on_open_change(|/* callback arguments */| { /* handle change */ })
    .on_open_change_complete(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a MenuSubmenuRoot with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<MenuSubmenuRootChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<MenuSubmenuRootChild<P>>`

Adds one typed child to this part.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

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

#### `.disabled(...)`

```rust
pub fn disabled(mut self, disabled: bool) -> Self
```

**Accepts**

- `disabled`: `bool`

When true, prevents user interaction with this part.

#### `.loop_focus(...)`

```rust
pub fn loop_focus(mut self, loop_focus: bool) -> Self
```

**Accepts**

- `loop_focus`: `bool`

Controls whether keyboard focus wraps from the last enabled item to the first and vice versa.

#### `.orientation(...)`

```rust
pub fn orientation(mut self, orientation: MenuOrientation) -> Self
```

**Accepts**

- `orientation`: `MenuOrientation`

Sets the component's horizontal or vertical orientation and corresponding keyboard behavior.

#### `.close_parent_on_esc(...)`

```rust
pub fn close_parent_on_esc(mut self, close_parent_on_esc: bool) -> Self
```

**Accepts**

- `close_parent_on_esc`: `bool`

Controls whether close parent on esc behavior is enabled.

#### `.highlight_item_on_hover(...)`

```rust
pub fn highlight_item_on_hover(mut self, highlight_item_on_hover: bool) -> Self
```

**Accepts**

- `highlight_item_on_hover`: `bool`

Controls whether highlight item on hover behavior is enabled.

#### `.on_open_change(...)`

```rust
pub fn on_open_change(mut self, on_open_change: impl Fn(bool, &mut MenuOpenChangeDetails<P>, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_open_change`: `impl Fn(bool, &mut MenuOpenChangeDetails<P>, &mut Window, &mut App) + 'static`

Registers a callback invoked when open change occurs.

#### `.on_open_change_complete(...)`

```rust
pub fn on_open_change_complete(mut self, on_open_change_complete: impl Fn(bool, &MenuOpenChangeDetails<P>, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_open_change_complete`: `impl Fn(bool, &MenuOpenChangeDetails<P>, &mut Window, &mut App)
            + 'static`

Registers a callback invoked when open change complete occurs.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(MenuRootStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(MenuRootStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

MenuSubmenuRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `MenuSubmenuTrigger`

Interactive control that opens, closes, or activates the component.

[Source](../../src/menu/layers/menu_submenu_trigger.rs)

```rust
use base_gpui::menu::MenuSubmenuTrigger;

MenuSubmenuTrigger::new()
    .id("example-id")
    .label("label")
    .disabled(true)
    .open_on_hover(true)
    .delay(/* delay: Duration */)
    .close_delay(/* close_delay: Duration */)
    .label_value()
    .disabled_value()
    .open_on_hover_value()
    .delay_value()
    .close_delay_value()
    .wired(0, /* focus_handle: FocusHandle */)
    .with_contexts(/* parent: MenuContext<P> */, /* child */, 0)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a MenuSubmenuTrigger with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.label(...)`

```rust
pub fn label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Sets the label configuration for this part.

#### `.disabled(...)`

```rust
pub fn disabled(mut self, disabled: bool) -> Self
```

**Accepts**

- `disabled`: `bool`

When true, prevents user interaction with this part.

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

#### `.label_value(...)`

```rust
pub fn label_value(&self) -> Option<SharedString>
```

Sets the label value configuration for this part.

#### `.disabled_value(...)`

```rust
pub fn disabled_value(&self) -> bool
```

Sets the disabled value configuration for this part.

#### `.open_on_hover_value(...)`

```rust
pub fn open_on_hover_value(&self) -> bool
```

Controls whether open on hover value behavior is enabled.

#### `.delay_value(...)`

```rust
pub fn delay_value(&self) -> Duration
```

Sets the delay value configuration for this part.

#### `.close_delay_value(...)`

```rust
pub fn close_delay_value(&self) -> Duration
```

Controls whether close delay value behavior is enabled.

#### `.wired(...)`

```rust
pub fn wired(mut self, index: usize, focus_handle: FocusHandle) -> Self
```

**Accepts**

- `index`: `usize`
- `focus_handle`: `FocusHandle`

Sets the wired configuration for this part.

#### `.with_contexts(...)`

```rust
pub fn with_contexts(mut self, parent: MenuContext<P>, child: MenuContext<P>, item_index: usize,) -> Self
```

**Accepts**

- `parent`: `MenuContext<P>`
- `child`: `MenuContext<P>`
- `item_index`: `usize`

Sets the with contexts configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(MenuSubmenuTriggerStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(MenuSubmenuTriggerStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

MenuSubmenuTrigger also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `MenuTrigger`

Interactive control that opens, closes, or activates the component.

[Source](../../src/menu/layers/menu_trigger.rs)

```rust
use base_gpui::menu::MenuTrigger;

MenuTrigger::new()
    .id("example-id")
    .disabled(true)
    .payload(/* payload: P */)
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

Creates a MenuTrigger with its default configuration.

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

Accessible label announced for the trigger. When set, render the visible trigger text as `Text::new_inaccessible(...)` so the label is not announced twice.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(MenuTriggerStyleState<P>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(MenuTriggerStyleState<P>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

MenuTrigger also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `MenuPortal`

Hosts overlay content outside the normal child layout.

[Source](../../src/menu/layers/menu_portal.rs)

```rust
use base_gpui::menu::MenuPortal;

MenuPortal::new()
    .keep_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a MenuPortal with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<MenuPortalChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<MenuPortalChild<P>>`

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
pub fn style_with_state(mut self, style: impl Fn(MenuPortalStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(MenuPortalStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

MenuPortal also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `MenuPositioner`

Measures the anchor and positions floating content.

[Source](../../src/menu/layers/menu_positioner.rs)

```rust
use base_gpui::menu::MenuPositioner;

MenuPositioner::new()
    .side("example-id")
    .align(/* align: MenuAlign */)
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

Creates a MenuPositioner with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<MenuPositionerChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<MenuPositionerChild<P>>`

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
pub fn side(mut self, side: MenuSide) -> Self
```

**Accepts**

- `side`: `MenuSide`

Sets the preferred side of the anchor on which floating content appears.

#### `.align(...)`

```rust
pub fn align(mut self, align: MenuAlign) -> Self
```

**Accepts**

- `align`: `MenuAlign`

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
pub fn style_with_state(mut self, style: impl Fn(MenuPositionerStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(MenuPositionerStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

MenuPositioner also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `MenuPopup`

Contains the floating interactive content.

[Source](../../src/menu/layers/menu_popup.rs)

```rust
use base_gpui::menu::MenuPopup;

MenuPopup::new()
    .id("example-id")
    .keep_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a MenuPopup with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<MenuPopupChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<MenuPopupChild<P>>`

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
pub fn style_with_state(mut self, style: impl Fn(MenuPopupStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(MenuPopupStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

MenuPopup also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `MenuArrow`

Optional arrow that visually points toward the overlay anchor.

[Source](../../src/menu/layers/menu_arrow.rs)

```rust
use base_gpui::menu::MenuArrow;

MenuArrow::new()
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a MenuArrow with its default configuration.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(MenuArrowStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(MenuArrowStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

MenuArrow also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `MenuBackdrop`

Covers content behind a modal overlay and handles outside interaction.

[Source](../../src/menu/layers/menu_backdrop.rs)

```rust
use base_gpui::menu::MenuBackdrop;

MenuBackdrop::new()
    .keep_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a MenuBackdrop with its default configuration.

#### `.keep_mounted(...)`

```rust
pub fn keep_mounted(mut self, keep_mounted: bool) -> Self
```

**Accepts**

- `keep_mounted`: `bool`

Keeps the part mounted when inactive or closed so child state can be preserved.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(MenuBackdropStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(MenuBackdropStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

MenuBackdrop also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `MenuCheckboxItem`

Represents one interactive item in the component's collection.

[Source](../../src/menu/layers/menu_checkbox_item.rs)

```rust
use base_gpui::menu::MenuCheckboxItem;

MenuCheckboxItem::new()
    .id("example-id")
    .label("label")
    .disabled(true)
    .close_on_click(true)
    .checked(true)
    .default_checked(true)
    .on_checked_change(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a MenuCheckboxItem with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<MenuCheckboxItemChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<MenuCheckboxItemChild<P>>`

Adds one typed child to this part.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.label(...)`

```rust
pub fn label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Sets the label configuration for this part.

#### `.disabled(...)`

```rust
pub fn disabled(mut self, disabled: bool) -> Self
```

**Accepts**

- `disabled`: `bool`

When true, prevents user interaction with this part.

#### `.close_on_click(...)`

```rust
pub fn close_on_click(mut self, close_on_click: bool) -> Self
```

**Accepts**

- `close_on_click`: `bool`

Controls whether close on click behavior is enabled.

#### `.checked(...)`

```rust
pub fn checked(mut self, checked: bool) -> Self
```

**Accepts**

- `checked`: `bool`

Sets the checked configuration for this part.

#### `.default_checked(...)`

```rust
pub fn default_checked(mut self, default_checked: bool) -> Self
```

**Accepts**

- `default_checked`: `bool`

Sets the initial checked for uncontrolled state.

#### `.on_checked_change(...)`

```rust
pub fn on_checked_change(mut self, on_checked_change: impl Fn(bool, &mut MenuItemChangeDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_checked_change`: `impl Fn(bool, &mut MenuItemChangeDetails, &mut Window, &mut App) + 'static`

Registers a callback invoked when checked change occurs.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(MenuCheckboxItemStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(MenuCheckboxItemStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

MenuCheckboxItem also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `MenuCheckboxItemIndicator`

Visual indicator for an item's selected or checked state.

[Source](../../src/menu/layers/menu_checkbox_item_indicator.rs)

```rust
use base_gpui::menu::MenuCheckboxItemIndicator;

MenuCheckboxItemIndicator::new()
    .keep_mounted(true)
    .with_item_facts(true, true, true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a MenuCheckboxItemIndicator with its default configuration.

#### `.keep_mounted(...)`

```rust
pub fn keep_mounted(mut self, keep_mounted: bool) -> Self
```

**Accepts**

- `keep_mounted`: `bool`

Keeps the part mounted when inactive or closed so child state can be preserved.

#### `.with_item_facts(...)`

```rust
pub fn with_item_facts(mut self, checked: bool, highlighted: bool, disabled: bool) -> Self
```

**Accepts**

- `checked`: `bool`
- `highlighted`: `bool`
- `disabled`: `bool`

Sets the with item facts configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(MenuCheckboxItemIndicatorStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(MenuCheckboxItemIndicatorStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

MenuCheckboxItemIndicator also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `MenuGroup`

Groups related child parts and coordinates their shared behavior.

[Source](../../src/menu/layers/menu_group.rs)

```rust
use base_gpui::menu::MenuGroup;

MenuGroup::new()
    .id("example-id")
    .aria_label("label")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a MenuGroup with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Accessible group label. Defaults to the label registered by a `MenuGroupLabel` child.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<MenuGroupChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<MenuGroupChild<P>>`

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
pub fn style_with_state(mut self, style: impl Fn(MenuGroupStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(MenuGroupStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

MenuGroup also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `MenuGroupLabel`

Provides a visible label and associated accessibility semantics.

[Source](../../src/menu/layers/menu_group_label.rs)

```rust
use base_gpui::menu::MenuGroupLabel;

MenuGroupLabel::new()
    .label("label")
    .label_value()
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a MenuGroupLabel with its default configuration.

#### `.label(...)`

```rust
pub fn label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Sets the label configuration for this part.

#### `.label_value(...)`

```rust
pub fn label_value(&self) -> Option<SharedString>
```

Registered label string, read by the parent group's wiring to source its `.aria_label(...)`.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(MenuGroupLabelStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(MenuGroupLabelStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

MenuGroupLabel also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `MenuItem`

Represents one interactive item in the component's collection.

[Source](../../src/menu/layers/menu_item.rs)

```rust
use base_gpui::menu::MenuItem;

MenuItem::new()
    .id("example-id")
    .label("label")
    .disabled(true)
    .close_on_click(true)
    .on_click(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a MenuItem with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.label(...)`

```rust
pub fn label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Sets the label configuration for this part.

#### `.disabled(...)`

```rust
pub fn disabled(mut self, disabled: bool) -> Self
```

**Accepts**

- `disabled`: `bool`

When true, prevents user interaction with this part.

#### `.close_on_click(...)`

```rust
pub fn close_on_click(mut self, close_on_click: bool) -> Self
```

**Accepts**

- `close_on_click`: `bool`

Controls whether close on click behavior is enabled.

#### `.on_click(...)`

```rust
pub fn on_click(mut self, on_click: impl Fn(&mut Window, &mut App) + 'static) -> Self
```

**Accepts**

- `on_click`: `impl Fn(&mut Window, &mut App) + 'static`

Registers a callback invoked when click occurs.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(MenuItemStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(MenuItemStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

MenuItem also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `MenuLinkItem`

Represents one interactive item in the component's collection.

[Source](../../src/menu/layers/menu_link_item.rs)

```rust
use base_gpui::menu::MenuLinkItem;

MenuLinkItem::new()
    .id("example-id")
    .label("label")
    .close_on_click(true)
    .on_activate(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a MenuLinkItem with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.label(...)`

```rust
pub fn label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Sets the label configuration for this part.

#### `.close_on_click(...)`

```rust
pub fn close_on_click(mut self, close_on_click: bool) -> Self
```

**Accepts**

- `close_on_click`: `bool`

Controls whether close on click behavior is enabled.

#### `.on_activate(...)`

```rust
pub fn on_activate(mut self, on_activate: impl Fn(&mut Window, &mut App) + 'static) -> Self
```

**Accepts**

- `on_activate`: `impl Fn(&mut Window, &mut App) + 'static`

Registers a callback invoked when activate occurs.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(MenuLinkItemStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(MenuLinkItemStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

MenuLinkItem also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `MenuRadioGroup`

Groups related child parts and coordinates their shared behavior.

[Source](../../src/menu/layers/menu_radio_group.rs)

```rust
use base_gpui::menu::MenuRadioGroup;

MenuRadioGroup::new()
    .id("example-id")
    .aria_label("label")
    .value(None)
    .default_value(None)
    .disabled(true)
    .on_value_change(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a MenuRadioGroup with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Accessible group label. Defaults to the label registered by a `MenuGroupLabel` child.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<MenuRadioGroupChild<P, V>>) -> Self
```

**Accepts**

- `child`: `impl Into<MenuRadioGroupChild<P, V>>`

Adds one typed child to this part.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.value(...)`

```rust
pub fn value(mut self, value: Option<V>) -> Self
```

**Accepts**

- `value`: `Option<V>`

Sets the current controlled value or the value represented by this part, depending on the part's role.

#### `.default_value(...)`

```rust
pub fn default_value(mut self, default_value: Option<V>) -> Self
```

**Accepts**

- `default_value`: `Option<V>`

Sets the initial value for uncontrolled state. Later user changes are retained by the component.

#### `.disabled(...)`

```rust
pub fn disabled(mut self, disabled: bool) -> Self
```

**Accepts**

- `disabled`: `bool`

When true, prevents user interaction with this part.

#### `.on_value_change(...)`

```rust
pub fn on_value_change(mut self, on_value_change: impl Fn(V, &mut crate::menu::MenuItemChangeDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_value_change`: `impl Fn(V, &mut crate::menu::MenuItemChangeDetails, &mut Window, &mut App)
            + 'static`

Registers a callback invoked when value change occurs.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(MenuRadioGroupStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(MenuRadioGroupStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

MenuRadioGroup also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `MenuRadioItem`

Represents one interactive item in the component's collection.

[Source](../../src/menu/layers/menu_radio_item.rs)

```rust
use base_gpui::menu::MenuRadioItem;

MenuRadioItem::new()
    .prepare_for_group(0, true, None, None)
    .wired_index()
    .item_value()
    .id("example-id")
    .value(/* value: V */)
    .label("label")
    .disabled(true)
    .close_on_click(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a MenuRadioItem with its default configuration.

#### `.prepare_for_group(...)`

```rust
pub fn prepare_for_group(mut self, group_index: usize, group_disabled: bool, group_value: Option<Option<V>>, on_value_change: Option<MenuValueChangeHandler<V>>,) -> Self
```

**Accepts**

- `group_index`: `usize`
- `group_disabled`: `bool`
- `group_value`: `Option<Option<V>>`
- `on_value_change`: `Option<MenuValueChangeHandler<V>>`

Called by `MenuRadioGroup` wiring before item wiring: injects group facts so radio state resolution stays with the group.

#### `.wired_index(...)`

```rust
pub fn wired_index(&self) -> Option<usize>
```

Sets the wired index configuration for this part.

#### `.item_value(...)`

```rust
pub fn item_value(&self) -> Option<&V>
```

Sets the item value configuration for this part.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<MenuRadioItemChild<P, V>>) -> Self
```

**Accepts**

- `child`: `impl Into<MenuRadioItemChild<P, V>>`

Adds one typed child to this part.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.value(...)`

```rust
pub fn value(mut self, value: V) -> Self
```

**Accepts**

- `value`: `V`

Sets the current controlled value or the value represented by this part, depending on the part's role.

#### `.label(...)`

```rust
pub fn label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Sets the label configuration for this part.

#### `.disabled(...)`

```rust
pub fn disabled(mut self, disabled: bool) -> Self
```

**Accepts**

- `disabled`: `bool`

When true, prevents user interaction with this part.

#### `.close_on_click(...)`

```rust
pub fn close_on_click(mut self, close_on_click: bool) -> Self
```

**Accepts**

- `close_on_click`: `bool`

Controls whether close on click behavior is enabled.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(MenuRadioItemStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(MenuRadioItemStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

MenuRadioItem also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `MenuRadioItemIndicator`

Visual indicator for an item's selected or checked state.

[Source](../../src/menu/layers/menu_radio_item_indicator.rs)

```rust
use base_gpui::menu::MenuRadioItemIndicator;

MenuRadioItemIndicator::new()
    .keep_mounted(true)
    .with_item_facts(true, true, true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a MenuRadioItemIndicator with its default configuration.

#### `.keep_mounted(...)`

```rust
pub fn keep_mounted(mut self, keep_mounted: bool) -> Self
```

**Accepts**

- `keep_mounted`: `bool`

Keeps the part mounted when inactive or closed so child state can be preserved.

#### `.with_item_facts(...)`

```rust
pub fn with_item_facts(mut self, checked: bool, highlighted: bool, disabled: bool) -> Self
```

**Accepts**

- `checked`: `bool`
- `highlighted`: `bool`
- `disabled`: `bool`

Sets the with item facts configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(MenuRadioItemIndicatorStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(MenuRadioItemIndicatorStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

MenuRadioItemIndicator also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `MenuSeparator`

Visual separator between neighboring items or groups.

[Source](../../src/menu/layers/menu_separator.rs)

```rust
use base_gpui::menu::MenuSeparator;

MenuSeparator::new()
    .orientation(/* orientation: SeparatorOrientation */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a MenuSeparator with its default configuration.

#### `.orientation(...)`

```rust
pub fn orientation(mut self, orientation: SeparatorOrientation) -> Self
```

**Accepts**

- `orientation`: `SeparatorOrientation`

Sets the component's horizontal or vertical orientation and corresponding keyboard behavior.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(SeparatorStyleState, gpui::Div) -> gpui::Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(SeparatorStyleState, gpui::Div) -> gpui::Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

MenuSeparator also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
