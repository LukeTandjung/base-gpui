# Tooltip

Accessibility: the trigger is exposed as `Role::Button` (with an optional `TooltipTrigger::aria_label`) and the open popup as `Role::Tooltip`; all other layers stay out of the accessibility tree, matching Base UI's ARIA-light tooltip. Known gaps in the pinned gpui revision: - `disabled`: no `.aria_disabled(...)` builder exists, so AT cannot see a disabled trigger state. Disabled triggers are removed from tab order and omit the Click a11y action instead; blocked pending an upstream gpui `set_disabled` addition. - `aria-describedby` trigger→popup relationship: no relationship builders exist. Base UI does not emit this for Tooltip either; tooltip content is not an accessible description or name for the trigger. - Live-region announcements of tooltip content on open: no announcement API exists, so tooltips remain sighted-user visual hints.

[Base UI reference](https://base-ui.com/react/components/tooltip) · [Source](../../src/tooltip/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::tooltip::{
    TooltipPopup,
    TooltipPortal,
    TooltipPositioner,
    TooltipProvider,
    TooltipProvider,
    TooltipRoot,
    TooltipTrigger,
    TooltipViewport,
};

TooltipRoot::new()
    .child(
        TooltipPortal::new()
                .child(
                    TooltipPositioner::new()
                                .child(
                                    TooltipPopup::new()
                                                    .child(
                                                        TooltipViewport::new(),
                                                    ),
                                ),
                ),
    )
    .child(
        TooltipTrigger::new(),
    );
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `TooltipRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/tooltip/layers/tooltip_root.rs)

```rust
use base_gpui::tooltip::TooltipRoot;

TooltipRoot::new()
    .id("example-id")
    .default_open(true)
    .open(true)
    .on_open_change(|/* callback arguments */| { /* handle change */ })
    .on_open_change_complete(|/* callback arguments */| { /* handle change */ })
    .disabled(true)
    .disable_hoverable_popup(true)
    .track_cursor_axis(/* track_cursor_axis: TooltipTrackCursorAxis */)
    .trigger_id("example-id")
    .no_trigger_id()
    .default_trigger_id("example-id")
    .handle(/* handle: TooltipHandle<P> */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a TooltipRoot with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<TooltipChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<TooltipChild<P>>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<TooltipChild<P>>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<TooltipChild<P>>>`

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
pub fn on_open_change(mut self, on_open_change: impl Fn(bool, &mut TooltipOpenChangeDetails<P>, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_open_change`: `impl Fn(bool, &mut TooltipOpenChangeDetails<P>, &mut Window, &mut App) + 'static`

Registers a callback invoked when open change occurs.

#### `.on_open_change_complete(...)`

```rust
pub fn on_open_change_complete(mut self, on_open_change_complete: impl Fn(bool, &TooltipOpenChangeDetails<P>, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_open_change_complete`: `impl Fn(bool, &TooltipOpenChangeDetails<P>, &mut Window, &mut App)
            + 'static`

Registers a callback invoked when open change complete occurs.

#### `.disabled(...)`

```rust
pub fn disabled(mut self, disabled: bool) -> Self
```

**Accepts**

- `disabled`: `bool`

When true, prevents user interaction with this part.

#### `.disable_hoverable_popup(...)`

```rust
pub fn disable_hoverable_popup(mut self, disable_hoverable_popup: bool) -> Self
```

**Accepts**

- `disable_hoverable_popup`: `bool`

Controls whether disable hoverable popup behavior is enabled.

#### `.track_cursor_axis(...)`

```rust
pub fn track_cursor_axis(mut self, track_cursor_axis: TooltipTrackCursorAxis) -> Self
```

**Accepts**

- `track_cursor_axis`: `TooltipTrackCursorAxis`

Sets the track cursor axis configuration for this part.

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
pub fn handle(mut self, handle: TooltipHandle<P>) -> Self
```

**Accepts**

- `handle`: `TooltipHandle<P>`

Sets the handle configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(TooltipRootStyleState<P>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(TooltipRootStyleState<P>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

TooltipRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `TooltipTrigger`

Interactive control that opens, closes, or activates the component.

[Source](../../src/tooltip/layers/tooltip_trigger.rs)

```rust
use base_gpui::tooltip::TooltipTrigger;

TooltipTrigger::new()
    .id("example-id")
    .disabled(true)
    .payload(/* payload: P */)
    .handle(/* handle: TooltipHandle<P> */)
    .delay(/* delay: Duration */)
    .close_delay(/* close_delay: Duration */)
    .close_on_click(true)
    .aria_label("label")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a TooltipTrigger with its default configuration.

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
pub fn handle(mut self, handle: TooltipHandle<P>) -> Self
```

**Accepts**

- `handle`: `TooltipHandle<P>`

Sets the handle configuration for this part.

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

#### `.close_on_click(...)`

```rust
pub fn close_on_click(mut self, close_on_click: bool) -> Self
```

**Accepts**

- `close_on_click`: `bool`

Controls whether close on click behavior is enabled.

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Accessible name for the trigger. There is no derived default because trigger children are arbitrary elements, so icon-only triggers must set this. When a visible text child duplicates this label, render it with `Text::new_inaccessible(...)` instead of `text!(...)` to avoid double-announcing. Tooltip content is *not* an accessible description of the trigger (no `aria-describedby` relationship exists in this gpui revision), so do not rely on the popup for the trigger's name.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(TooltipTriggerStyleState<P>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(TooltipTriggerStyleState<P>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

TooltipTrigger also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `TooltipPortal`

Hosts overlay content outside the normal child layout.

[Source](../../src/tooltip/layers/tooltip_portal.rs)

```rust
use base_gpui::tooltip::TooltipPortal;

TooltipPortal::new()
    .keep_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a TooltipPortal with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<TooltipPortalChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<TooltipPortalChild<P>>`

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
pub fn style_with_state(mut self, style: impl Fn(TooltipPortalStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(TooltipPortalStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

TooltipPortal also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `TooltipPositioner`

Measures the anchor and positions floating content.

[Source](../../src/tooltip/layers/tooltip_positioner.rs)

```rust
use base_gpui::tooltip::TooltipPositioner;

TooltipPositioner::new()
    .side("example-id")
    .align(/* align: TooltipAlign */)
    .side_offset("example-id")
    .align_offset(/* align_offset: Pixels */)
    .collision_padding(/* collision_padding: Pixels */)
    .keep_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a TooltipPositioner with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<TooltipPositionerChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<TooltipPositionerChild<P>>`

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
pub fn side(mut self, side: TooltipSide) -> Self
```

**Accepts**

- `side`: `TooltipSide`

Sets the preferred side of the anchor on which floating content appears.

#### `.align(...)`

```rust
pub fn align(mut self, align: TooltipAlign) -> Self
```

**Accepts**

- `align`: `TooltipAlign`

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

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(TooltipPositionerStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(TooltipPositionerStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

TooltipPositioner also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `TooltipPopup`

Contains the floating interactive content.

[Source](../../src/tooltip/layers/tooltip_popup.rs)

```rust
use base_gpui::tooltip::TooltipPopup;

TooltipPopup::new()
    .id("example-id")
    .side("example-id")
    .align(/* align: TooltipAlign */)
    .keep_mounted(true)
    .payload_content(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a TooltipPopup with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<TooltipPopupChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<TooltipPopupChild<P>>`

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
pub fn side(mut self, side: TooltipSide) -> Self
```

**Accepts**

- `side`: `TooltipSide`

Sets the preferred side of the anchor on which floating content appears.

#### `.align(...)`

```rust
pub fn align(mut self, align: TooltipAlign) -> Self
```

**Accepts**

- `align`: `TooltipAlign`

Sets floating content alignment along the selected side.

#### `.keep_mounted(...)`

```rust
pub fn keep_mounted(mut self, keep_mounted: bool) -> Self
```

**Accepts**

- `keep_mounted`: `bool`

Keeps the part mounted when inactive or closed so child state can be preserved.

#### `.payload_content(...)`

```rust
pub fn payload_content(mut self, content: impl Fn(Option<&P>, &mut Window, &mut App) -> gpui::AnyElement + 'static,) -> Self
```

**Accepts**

- `content`: `impl Fn(Option<&P>, &mut Window, &mut App) -> gpui::AnyElement + 'static,`

Sets the payload content configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(TooltipPopupStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(TooltipPopupStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

TooltipPopup also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `TooltipProvider`

Public renderable part of the Tooltip Provider component.

[Source](../../src/tooltip/layers/tooltip_provider.rs)

```rust
use base_gpui::tooltip::TooltipProvider;

TooltipProvider::new()
    .id("example-id")
    .delay(/* delay: Duration */)
    .close_delay(/* close_delay: Duration */)
    .timeout(/* timeout: Duration */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a TooltipProvider with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<TooltipProviderChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<TooltipProviderChild<P>>`

Adds one typed child to this part.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

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

#### `.timeout(...)`

```rust
pub fn timeout(mut self, timeout: Duration) -> Self
```

**Accepts**

- `timeout`: `Duration`

Sets the duration before the component automatically performs its timeout behavior.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(TooltipProviderStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(TooltipProviderStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

TooltipProvider also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `TooltipProvider`

Public renderable part of the Tooltip Provider component.

[Source](../../src/tooltip/layers/tooltip_root.rs)

```rust
use base_gpui::tooltip::TooltipProvider;

TooltipProvider::new()
    .id("example-id")
    .delay(/* delay: Duration */)
    .close_delay(/* close_delay: Duration */)
    .timeout(/* timeout: Duration */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a TooltipProvider with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<TooltipProviderChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<TooltipProviderChild<P>>`

Adds one typed child to this part.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

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

#### `.timeout(...)`

```rust
pub fn timeout(mut self, timeout: Duration) -> Self
```

**Accepts**

- `timeout`: `Duration`

Sets the duration before the component automatically performs its timeout behavior.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(TooltipProviderStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(TooltipProviderStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

TooltipProvider also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `TooltipViewport`

Defines the viewport used to lay out or constrain overlay content.

[Source](../../src/tooltip/layers/tooltip_viewport.rs)

```rust
use base_gpui::tooltip::TooltipViewport;

TooltipViewport::new()
    .payload_content(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a TooltipViewport with its default configuration.

#### `.payload_content(...)`

```rust
pub fn payload_content(mut self, content: impl Fn(Option<&P>, &mut Window, &mut App) -> AnyElement + 'static,) -> Self
```

**Accepts**

- `content`: `impl Fn(Option<&P>, &mut Window, &mut App) -> AnyElement + 'static,`

Sets the payload content configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(TooltipViewportStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(TooltipViewportStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

TooltipViewport also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
