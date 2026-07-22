# Preview Card

Preview Card ports Base UI's Preview Card component model to GPUI.

[Base UI reference](https://base-ui.com/react/components/preview-card) · [Source](../../src/preview_card/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::preview_card::{
    PreviewCardArrow,
    PreviewCardBackdrop,
    PreviewCardPopup,
    PreviewCardPortal,
    PreviewCardPositioner,
    PreviewCardRoot,
    PreviewCardTrigger,
    PreviewCardViewport,
};

PreviewCardRoot::new()
    .child(
        PreviewCardPortal::new()
                .child(
                    PreviewCardBackdrop::new(),
                )
                .child(
                    PreviewCardPositioner::new()
                                .child(
                                    PreviewCardPopup::new()
                                                    .child(
                                                        PreviewCardArrow::new(),
                                                    )
                                                    .child(
                                                        PreviewCardViewport::new(),
                                                    ),
                                ),
                ),
    )
    .child(
        PreviewCardTrigger::new(),
    );
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `PreviewCardRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/preview_card/layers/preview_card_root.rs)

```rust
use base_gpui::preview_card::PreviewCardRoot;

PreviewCardRoot::new()
    .id("example-id")
    .default_open(true)
    .open(true)
    .delay(/* delay: std::time::Duration */)
    .close_delay(/* close_delay: std::time::Duration */)
    .on_open_change(|/* callback arguments */| { /* handle change */ })
    .on_open_change_complete(|/* callback arguments */| { /* handle change */ })
    .trigger_id("example-id")
    .no_trigger_id()
    .default_trigger_id("example-id")
    .handle(/* handle: PreviewCardHandle<P> */);
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a PreviewCardRoot with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<PreviewCardChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<PreviewCardChild<P>>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<PreviewCardChild<P>>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<PreviewCardChild<P>>>`

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

#### `.delay(...)`

```rust
pub fn delay(mut self, delay: std::time::Duration) -> Self
```

**Accepts**

- `delay`: `std::time::Duration`

Sets the delay before the related transition or interaction begins.

#### `.close_delay(...)`

```rust
pub fn close_delay(mut self, close_delay: std::time::Duration) -> Self
```

**Accepts**

- `close_delay`: `std::time::Duration`

Sets the delay before an open component closes after the closing interaction.

#### `.on_open_change(...)`

```rust
pub fn on_open_change(mut self, on_open_change: impl Fn(bool, &mut PreviewCardOpenChangeDetails<P>, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_open_change`: `impl Fn(bool, &mut PreviewCardOpenChangeDetails<P>, &mut Window, &mut App)
            + 'static`

Registers a callback invoked when open change occurs.

#### `.on_open_change_complete(...)`

```rust
pub fn on_open_change_complete(mut self, on_open_change_complete: impl Fn(bool, &PreviewCardOpenChangeDetails<P>, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_open_change_complete`: `impl Fn(bool, &PreviewCardOpenChangeDetails<P>, &mut Window, &mut App)
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
pub fn handle(mut self, handle: PreviewCardHandle<P>) -> Self
```

**Accepts**

- `handle`: `PreviewCardHandle<P>`

Sets the handle configuration for this part.

PreviewCardRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `PreviewCardTrigger`

Interactive control that opens, closes, or activates the component.

[Source](../../src/preview_card/layers/preview_card_trigger.rs)

```rust
use base_gpui::preview_card::PreviewCardTrigger;

PreviewCardTrigger::new()
    .id("example-id")
    .payload(/* payload: P */)
    .handle(/* handle: PreviewCardHandle<P> */)
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

Creates a PreviewCardTrigger with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.payload(...)`

```rust
pub fn payload(mut self, payload: P) -> Self
```

**Accepts**

- `payload`: `P`

Sets the payload configuration for this part.

#### `.handle(...)`

```rust
pub fn handle(mut self, handle: PreviewCardHandle<P>) -> Self
```

**Accepts**

- `handle`: `PreviewCardHandle<P>`

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

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Accessible name for the link trigger. When set, construct the visible label children with `Text::new_inaccessible(...)` instead of `text!` so the label is not announced twice.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(PreviewCardTriggerStyleState<P>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(PreviewCardTriggerStyleState<P>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

PreviewCardTrigger also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `PreviewCardPortal`

Hosts overlay content outside the normal child layout.

[Source](../../src/preview_card/layers/preview_card_portal.rs)

```rust
use base_gpui::preview_card::PreviewCardPortal;

PreviewCardPortal::new()
    .keep_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a PreviewCardPortal with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<PreviewCardPortalChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<PreviewCardPortalChild<P>>`

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
pub fn style_with_state(mut self, style: impl Fn(PreviewCardPortalStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(PreviewCardPortalStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

PreviewCardPortal also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `PreviewCardPositioner`

Measures the anchor and positions floating content.

[Source](../../src/preview_card/layers/preview_card_positioner.rs)

```rust
use base_gpui::preview_card::PreviewCardPositioner;

PreviewCardPositioner::new()
    .side("example-id")
    .align(/* align: PreviewCardAlign */)
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

Creates a PreviewCardPositioner with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<PreviewCardPositionerChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<PreviewCardPositionerChild<P>>`

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
pub fn side(mut self, side: PreviewCardSide) -> Self
```

**Accepts**

- `side`: `PreviewCardSide`

Sets the preferred side of the anchor on which floating content appears.

#### `.align(...)`

```rust
pub fn align(mut self, align: PreviewCardAlign) -> Self
```

**Accepts**

- `align`: `PreviewCardAlign`

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
pub fn style_with_state(mut self, style: impl Fn(PreviewCardPositionerStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(PreviewCardPositionerStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

PreviewCardPositioner also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `PreviewCardPopup`

Contains the floating interactive content.

[Source](../../src/preview_card/layers/preview_card_popup.rs)

```rust
use base_gpui::preview_card::PreviewCardPopup;

PreviewCardPopup::new()
    .id("example-id")
    .side("example-id")
    .align(/* align: PreviewCardAlign */)
    .keep_mounted(true)
    .payload_content(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a PreviewCardPopup with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<PreviewCardPopupChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<PreviewCardPopupChild<P>>`

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
pub fn side(mut self, side: PreviewCardSide) -> Self
```

**Accepts**

- `side`: `PreviewCardSide`

Sets the preferred side of the anchor on which floating content appears.

#### `.align(...)`

```rust
pub fn align(mut self, align: PreviewCardAlign) -> Self
```

**Accepts**

- `align`: `PreviewCardAlign`

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
pub fn style_with_state(mut self, style: impl Fn(PreviewCardPopupStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(PreviewCardPopupStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

PreviewCardPopup also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `PreviewCardArrow`

Optional arrow that visually points toward the overlay anchor.

[Source](../../src/preview_card/layers/preview_card_arrow.rs)

```rust
use base_gpui::preview_card::PreviewCardArrow;

PreviewCardArrow::new()
    .side("example-id")
    .align(/* align: PreviewCardAlign */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a PreviewCardArrow with its default configuration.

#### `.side(...)`

```rust
pub fn side(mut self, side: PreviewCardSide) -> Self
```

**Accepts**

- `side`: `PreviewCardSide`

Sets the preferred side of the anchor on which floating content appears.

#### `.align(...)`

```rust
pub fn align(mut self, align: PreviewCardAlign) -> Self
```

**Accepts**

- `align`: `PreviewCardAlign`

Sets floating content alignment along the selected side.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(PreviewCardArrowStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(PreviewCardArrowStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

PreviewCardArrow also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `PreviewCardBackdrop`

Covers content behind a modal overlay and handles outside interaction.

[Source](../../src/preview_card/layers/preview_card_backdrop.rs)

```rust
use base_gpui::preview_card::PreviewCardBackdrop;

PreviewCardBackdrop::new()
    .keep_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a PreviewCardBackdrop with its default configuration.

#### `.keep_mounted(...)`

```rust
pub fn keep_mounted(mut self, keep_mounted: bool) -> Self
```

**Accepts**

- `keep_mounted`: `bool`

Keeps the part mounted when inactive or closed so child state can be preserved.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(PreviewCardBackdropStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(PreviewCardBackdropStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

PreviewCardBackdrop also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `PreviewCardViewport`

Defines the viewport used to lay out or constrain overlay content.

[Source](../../src/preview_card/layers/preview_card_viewport.rs)

```rust
use base_gpui::preview_card::PreviewCardViewport;

PreviewCardViewport::new()
    .payload_content(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a PreviewCardViewport with its default configuration.

#### `.payload_content(...)`

```rust
pub fn payload_content(mut self, content: impl Fn(Option<&P>, &mut Window, &mut App) -> AnyElement + 'static,) -> Self
```

**Accepts**

- `content`: `impl Fn(Option<&P>, &mut Window, &mut App) -> AnyElement + 'static,`

Sets the payload content configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(PreviewCardViewportStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(PreviewCardViewportStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

PreviewCardViewport also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
