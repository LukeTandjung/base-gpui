# Toast

Toast: stacked, auto-dismissing notifications.

[Base UI reference](https://base-ui.com/react/components/toast) · [Source](../../src/toast/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::toast::{
    ToastAction,
    ToastClose,
    ToastContent,
    ToastDescription,
    ToastPortal,
    ToastProvider,
    ToastRoot,
    ToastTitle,
    ToastViewport,
};

ToastRoot::new()
    .child(
        ToastAction::new(),
    )
    .child(
        ToastClose::new(),
    )
    .child(
        ToastContent::new(),
    )
    .child(
        ToastDescription::new(),
    )
    .child(
        ToastTitle::new(),
    );
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `ToastRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/toast/layers/toast_root.rs)

```rust
use base_gpui::toast::ToastRoot;

ToastRoot::new()
    .swipe_direction(/* directions: impl IntoIterator<Item = ToastSwipeDirection> */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ToastRoot with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<ToastRootChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<ToastRootChild<P>>`

Adds one typed child to this part.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.swipe_direction(...)`

```rust
pub fn swipe_direction(mut self, directions: impl IntoIterator<Item = ToastSwipeDirection>,) -> Self
```

**Accepts**

- `directions`: `impl IntoIterator<Item = ToastSwipeDirection>`

Permitted swipe-to-dismiss directions; default `[Down, Right]`, an empty set disables swiping.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ToastRootStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ToastRootStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ToastRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ToastPortal`

Hosts overlay content outside the normal child layout.

[Source](../../src/toast/layers/toast_portal.rs)

```rust
use base_gpui::toast::ToastPortal;

ToastPortal::new();
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ToastPortal with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<ToastPortalChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<ToastPortalChild<P>>`

Adds one typed child to this part.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

ToastPortal also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ToastAction`

Public renderable part of the Toast Action component.

[Source](../../src/toast/layers/toast_action.rs)

```rust
use base_gpui::toast::ToastAction;

ToastAction::new()
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ToastAction with its default configuration.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ToastActionStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ToastActionStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ToastAction also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ToastClose`

Interactive control that closes the component.

[Source](../../src/toast/layers/toast_close.rs)

```rust
use base_gpui::toast::ToastClose;

ToastClose::new()
    .aria_label("label")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ToastClose with its default configuration.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Accessible label for the close button; defaults to "Close". Callers rendering a visible text child that duplicates this label should pass it as `Text::new_inaccessible(...)` (or override the label) to avoid double-announcing.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ToastCloseStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ToastCloseStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ToastClose also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ToastContent`

Contains the component's user-provided content.

[Source](../../src/toast/layers/toast_content.rs)

```rust
use base_gpui::toast::ToastContent;

ToastContent::new()
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ToastContent with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<ToastRootChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<ToastRootChild<P>>`

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
pub fn style_with_state(mut self, style: impl Fn(ToastContentStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ToastContentStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ToastContent also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ToastDescription`

Provides supporting descriptive content.

[Source](../../src/toast/layers/toast_description.rs)

```rust
use base_gpui::toast::ToastDescription;

ToastDescription::new()
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ToastDescription with its default configuration.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ToastDescriptionStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ToastDescriptionStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ToastDescription also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ToastProvider`

Public renderable part of the Toast Provider component.

[Source](../../src/toast/layers/toast_provider.rs)

```rust
use base_gpui::toast::ToastProvider;

ToastProvider::new()
    .id("example-id")
    .timeout(/* timeout: Duration */)
    .limit(0)
    .manager(/* manager: ToastManager<P> */);
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ToastProvider with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<ToastProviderChild<P>>) -> Self
```

**Accepts**

- `child`: `impl Into<ToastProviderChild<P>>`

Adds one typed child to this part.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.timeout(...)`

```rust
pub fn timeout(mut self, timeout: Duration) -> Self
```

**Accepts**

- `timeout`: `Duration`

Default auto-dismiss timeout for toasts without their own (5000 ms).

#### `.limit(...)`

```rust
pub fn limit(mut self, limit: usize) -> Self
```

**Accepts**

- `limit`: `usize`

Maximum non-limited toasts (older ones are flagged `limited`; default 3).

#### `.manager(...)`

```rust
pub fn manager(mut self, manager: ToastManager<P>) -> Self
```

**Accepts**

- `manager`: `ToastManager<P>`

Binds an imperative manager created with `create_toast_manager`.

ToastProvider also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ToastTitle`

Provides the component's visible title.

[Source](../../src/toast/layers/toast_title.rs)

```rust
use base_gpui::toast::ToastTitle;

ToastTitle::new()
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ToastTitle with its default configuration.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ToastTitleStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ToastTitleStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ToastTitle also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ToastViewport`

Defines the viewport used to lay out or constrain overlay content.

[Source](../../src/toast/layers/toast_viewport.rs)

```rust
use base_gpui::toast::ToastViewport;

ToastViewport::new()
    .id("example-id")
    .aria_label("label")
    .content_builder(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ToastViewport with its default configuration.

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

Accessible label for the notifications region; defaults to "Notifications" (Base UI viewport `aria-label` parity).

#### `.content_builder(...)`

```rust
pub fn content_builder(mut self, builder: impl Fn(&ToastFacts<P>) -> ToastRoot<P> + 'static,) -> Self
```

**Accepts**

- `builder`: `impl Fn(&ToastFacts<P>) -> ToastRoot<P> + 'static,`

The typed per-toast content builder: receives typed toast facts and returns the `ToastRoot` subtree for that toast.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ToastViewportStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ToastViewportStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ToastViewport also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
