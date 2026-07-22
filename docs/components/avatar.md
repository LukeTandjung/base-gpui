# Avatar

Avatar ports Base UI's Avatar component model to GPUI.

[Base UI reference](https://base-ui.com/react/components/avatar) · [Source](../../src/avatar/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::avatar::{
    AvatarFallback,
    AvatarImage,
    AvatarRoot,
};

AvatarRoot::new()
    .child(
        AvatarFallback::new(),
    )
    .child(
        AvatarImage::new(/* source: impl Into<ImageSource> */),
    );
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `AvatarRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/avatar/layers/avatar_root.rs)

```rust
use base_gpui::avatar::AvatarRoot;

AvatarRoot::new()
    .child_element(/* element: impl IntoElement */)
    .aria_label("label")
    .id("example-id")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a AvatarRoot with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<AvatarChild>) -> Self
```

**Accepts**

- `child`: `impl Into<AvatarChild>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<AvatarChild>>) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<AvatarChild>>`

Adds multiple typed children in iteration order.

#### `.child_element(...)`

```rust
pub fn child_element(mut self, element: impl IntoElement) -> Self
```

**Accepts**

- `element`: `impl IntoElement`

Sets the child element configuration for this part.

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Accessible label for the avatar (the GPUI stand-in for `<img alt>`).  When set, the root enters the AccessKit tree as `Role::Image` with this label. When unset, the avatar has no role and produces no AccessKit node — omitting the label is how you mark an avatar decorative (there is no `aria-hidden` builder in this gpui revision). There is also no live-region API, so loading status is never announced.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(AvatarRootStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(AvatarRootStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

AvatarRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `AvatarFallback`

Public renderable part of the Avatar Fallback component.

[Source](../../src/avatar/layers/avatar_fallback.rs)

```rust
use base_gpui::avatar::AvatarFallback;

AvatarFallback::new()
    .delay(/* delay: Duration */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a AvatarFallback with its default configuration.

#### `.delay(...)`

```rust
pub fn delay(mut self, delay: Duration) -> Self
```

**Accepts**

- `delay`: `Duration`

Sets the delay before the related transition or interaction begins.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(AvatarFallbackStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(AvatarFallbackStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

AvatarFallback also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `AvatarImage`

Public renderable part of the Avatar Image component.

[Source](../../src/avatar/layers/avatar_image.rs)

```rust
use base_gpui::avatar::AvatarImage;

AvatarImage::new()
    .on_loading_status_change(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new(source: impl Into<ImageSource>) -> Self
```

**Accepts**

- `source`: `impl Into<ImageSource>`

Creates a AvatarImage with its default configuration.

#### `.on_loading_status_change(...)`

```rust
pub fn on_loading_status_change(mut self, on_loading_status_change: impl Fn(AvatarImageLoadingStatus, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_loading_status_change`: `impl Fn(AvatarImageLoadingStatus, &mut Window, &mut App) + 'static`

Registers a callback invoked when loading status change occurs.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(AvatarImageStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(AvatarImageStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

AvatarImage also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
