# Context Menu

Context Menu ports Base UI's Context Menu component model to GPUI.

[Base UI reference](https://base-ui.com/react/components/context-menu) · [Source](../../src/context_menu/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::context_menu::{
    ContextMenuRoot,
    ContextMenuTrigger,
};

ContextMenuRoot::new()
    .child(
        ContextMenuTrigger::new(),
    );
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `ContextMenuRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/context_menu/layers/context_menu_root.rs)

```rust
use base_gpui::context_menu::ContextMenuRoot;

ContextMenuRoot::new()
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

Creates a ContextMenuRoot with its default configuration.

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

ContextMenuRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ContextMenuTrigger`

Interactive control that opens, closes, or activates the component.

[Source](../../src/context_menu/layers/context_menu_trigger.rs)

```rust
use base_gpui::context_menu::ContextMenuTrigger;

ContextMenuTrigger::new()
    .id("example-id")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ContextMenuTrigger with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ContextMenuTriggerStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ContextMenuTriggerStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ContextMenuTrigger also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
