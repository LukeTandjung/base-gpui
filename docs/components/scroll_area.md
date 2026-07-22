# Scroll Area

Scroll Area ports Base UI's Scroll Area component model to GPUI.

[Base UI reference](https://base-ui.com/react/components/scroll-area) · [Source](../../src/scroll_area/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::scroll_area::{
    ScrollAreaContent,
    ScrollAreaCorner,
    ScrollAreaRoot,
    ScrollAreaScrollbar,
    ScrollAreaThumb,
    ScrollAreaViewport,
};

ScrollAreaRoot::new()
    .child(
        ScrollAreaCorner::new(),
    )
    .child(
        ScrollAreaScrollbar::new()
                .child(
                    ScrollAreaThumb::new(),
                ),
    )
    .child(
        ScrollAreaViewport::new()
                .child(
                    ScrollAreaContent::new(),
                ),
    );
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `ScrollAreaRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/scroll_area/layers/scroll_area_root.rs)

```rust
use base_gpui::scroll_area::ScrollAreaRoot;

ScrollAreaRoot::new()
    .id("example-id")
    .overflow_edge_threshold(/* threshold: impl Into<ScrollAreaEdgeThreshold> */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ScrollAreaRoot with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<ScrollAreaChild>) -> Self
```

**Accepts**

- `child`: `impl Into<ScrollAreaChild>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<ScrollAreaChild>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<ScrollAreaChild>>`

Adds multiple typed children in iteration order.

#### `.overflow_edge_threshold(...)`

```rust
pub fn overflow_edge_threshold(mut self, threshold: impl Into<ScrollAreaEdgeThreshold>,) -> Self
```

**Accepts**

- `threshold`: `impl Into<ScrollAreaEdgeThreshold>`

Distance thresholds gating the overflow-edge flags; accepts a uniform [`gpui::Pixels`] or a per-edge [`ScrollAreaEdgeThreshold`]. Negative values clamp to zero; the default is zero.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ScrollAreaRootStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ScrollAreaRootStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ScrollAreaRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ScrollAreaContent`

Contains the component's user-provided content.

[Source](../../src/scroll_area/layers/scroll_area_content.rs)

```rust
use base_gpui::scroll_area::ScrollAreaContent;

ScrollAreaContent::new()
    .id("example-id")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ScrollAreaContent with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl IntoElement>`

Adds multiple typed children in iteration order.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ScrollAreaRootStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ScrollAreaRootStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ScrollAreaContent also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ScrollAreaCorner`

Public renderable part of the Scroll Area Corner component.

[Source](../../src/scroll_area/layers/scroll_area_corner.rs)

```rust
use base_gpui::scroll_area::ScrollAreaCorner;

ScrollAreaCorner::new()
    .id("example-id")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ScrollAreaCorner with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ScrollAreaCornerStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ScrollAreaCornerStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ScrollAreaCorner also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ScrollAreaScrollbar`

Public renderable part of the Scroll Area Scrollbar component.

[Source](../../src/scroll_area/layers/scroll_area_scrollbar.rs)

```rust
use base_gpui::scroll_area::ScrollAreaScrollbar;

ScrollAreaScrollbar::new()
    .id("example-id")
    .orientation(/* orientation: ScrollAreaOrientation */)
    .keep_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ScrollAreaScrollbar with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.orientation(...)`

```rust
pub fn orientation(mut self, orientation: ScrollAreaOrientation) -> Self
```

**Accepts**

- `orientation`: `ScrollAreaOrientation`

Which axis this scrollbar tracks; defaults to vertical.

#### `.keep_mounted(...)`

```rust
pub fn keep_mounted(mut self, keep_mounted: bool) -> Self
```

**Accepts**

- `keep_mounted`: `bool`

Keep the scrollbar in the tree while its axis has no overflow; defaults to false.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<ScrollAreaScrollbarChild>) -> Self
```

**Accepts**

- `child`: `impl Into<ScrollAreaScrollbarChild>`

Adds one typed child to this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ScrollAreaScrollbarStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ScrollAreaScrollbarStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ScrollAreaScrollbar also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ScrollAreaThumb`

Interactive or visual handle representing the current value.

[Source](../../src/scroll_area/layers/scroll_area_thumb.rs)

```rust
use base_gpui::scroll_area::ScrollAreaThumb;

ScrollAreaThumb::new()
    .style_with_state(|/* callback arguments */| { /* handle change */ })
    .take_style();
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ScrollAreaThumb with its default configuration.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ScrollAreaThumbStyleState, ScrollbarStyle) -> ScrollbarStyle + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ScrollAreaThumbStyleState, ScrollbarStyle) -> ScrollbarStyle + 'static,`

Adjust the primitive's thumb/track appearance from the thumb's Scroll Area facts. Receives [`ScrollbarStyle::default`] and returns the style to use for the current frame.

#### `.take_style(...)`

```rust
pub fn take_style(self) -> Option<Rc<ThumbStyleFn>>
```

The configured style callback, consumed by the Scrollbar part when it builds the primitive.

ScrollAreaThumb also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ScrollAreaViewport`

Defines the viewport used to lay out or constrain overlay content.

[Source](../../src/scroll_area/layers/scroll_area_viewport.rs)

```rust
use base_gpui::scroll_area::ScrollAreaViewport;

ScrollAreaViewport::new()
    .id("example-id")
    .aria_label("label")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ScrollAreaViewport with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<ScrollAreaViewportChild>) -> Self
```

**Accepts**

- `child`: `impl Into<ScrollAreaViewportChild>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<ScrollAreaViewportChild>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<ScrollAreaViewportChild>>`

Adds multiple typed children in iteration order.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Wrap an arbitrary element as viewport content (Content is optional for vertical-only use).

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Accessible name for the scroll region, exposed via `.aria_label(...)` while the viewport is focusable. Base UI supplies no default label; an unlabeled scroll region announces poorly, so consumers should set one.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ScrollAreaRootStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ScrollAreaRootStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ScrollAreaViewport also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
