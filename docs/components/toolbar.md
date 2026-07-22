# Toolbar

Toolbar component family ported from Base UI.

[Base UI reference](https://base-ui.com/react/components/toolbar) · [Source](../../src/toolbar/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::toolbar::{
    ToolbarButton,
    ToolbarGroup,
    ToolbarInput,
    ToolbarLink,
    ToolbarRoot,
    ToolbarSeparator,
};

ToolbarRoot::new()
    .child(
        ToolbarGroup::new()
                .child(
                    ToolbarButton::new(),
                )
                .child(
                    ToolbarInput::new(),
                )
                .child(
                    ToolbarLink::new(),
                ),
    )
    .child(
        ToolbarSeparator::new(),
    );
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `ToolbarRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/toolbar/layers/toolbar_root.rs)

```rust
use base_gpui::toolbar::ToolbarRoot;

ToolbarRoot::new()
    .id("example-id")
    .orientation(/* orientation: ToolbarOrientation */)
    .loop_focus(true)
    .disabled(true)
    .aria_label("aria label")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ToolbarRoot with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<ToolbarChild>) -> Self
```

**Accepts**

- `child`: `impl Into<ToolbarChild>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<ToolbarChild>>) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<ToolbarChild>>`

Adds multiple typed children in iteration order.

#### `.orientation(...)`

```rust
pub fn orientation(mut self, orientation: ToolbarOrientation) -> Self
```

**Accepts**

- `orientation`: `ToolbarOrientation`

Sets the component's horizontal or vertical orientation and corresponding keyboard behavior.

#### `.loop_focus(...)`

```rust
pub fn loop_focus(mut self, loop_focus: bool) -> Self
```

**Accepts**

- `loop_focus`: `bool`

Controls whether keyboard focus wraps from the last enabled item to the first and vice versa.

#### `.disabled(...)`

```rust
pub fn disabled(mut self, disabled: bool) -> Self
```

**Accepts**

- `disabled`: `bool`

When true, prevents user interaction with this part.

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self
```

**Accepts**

- `aria_label`: `impl Into<SharedString>`

Accessible name for the toolbar, announced by screen readers. There is no `aria-labelledby` id-reference builder in this gpui revision, so the name is a literal string.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ToolbarRootStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ToolbarRootStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ToolbarRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ToolbarButton`

Public renderable part of the Toolbar Button component.

[Source](../../src/toolbar/layers/toolbar_button.rs)

```rust
use base_gpui::toolbar::ToolbarButton;

ToolbarButton::new()
    .id("example-id")
    .disabled(true)
    .focusable_when_disabled(true)
    .aria_label("aria label")
    .on_click(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ })
    .own_disabled()
    .own_focusable_when_disabled()
    .item_id()
    .with_toolbar(/* context: ToolbarContext */, 0, /* focus_handle: FocusHandle */, true);
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ToolbarButton with its default configuration.

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

#### `.focusable_when_disabled(...)`

```rust
pub fn focusable_when_disabled(mut self, focusable_when_disabled: bool) -> Self
```

**Accepts**

- `focusable_when_disabled`: `bool`

Sets the focusable when disabled configuration for this part.

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self
```

**Accepts**

- `aria_label`: `impl Into<SharedString>`

Accessible name for icon-only buttons. When set alongside a visible text child, render that text with `Text::new_inaccessible(...)` instead of `text!(...)` so screen readers do not announce the name twice; without an `aria_label`, keep `text!(...)` so the child text remains the accessible name source.

#### `.on_click(...)`

```rust
pub fn on_click(mut self, on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_click`: `impl Fn(&ClickEvent, &mut Window, &mut App) + 'static`

Registers a callback invoked when click occurs.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ToolbarButtonStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ToolbarButtonStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

#### `.own_disabled(...)`

```rust
pub fn own_disabled(&self) -> bool
```

The button's own disabled prop, consumed by the toolbar child wiring when resolving the effective per-item disabled fact.

#### `.own_focusable_when_disabled(...)`

```rust
pub fn own_focusable_when_disabled(&self) -> bool
```

The button's `focusable_when_disabled` flag, consumed by the toolbar child wiring for item metadata.

#### `.item_id(...)`

```rust
pub fn item_id(&self) -> &ElementId
```

The button's element id, consumed by the toolbar child wiring to key the roving focus handle.

#### `.with_toolbar(...)`

```rust
pub fn with_toolbar(mut self, context: ToolbarContext, index: usize, focus_handle: FocusHandle, cascade_disabled: bool,) -> Self
```

**Accepts**

- `context`: `ToolbarContext`
- `index`: `usize`
- `focus_handle`: `FocusHandle`
- `cascade_disabled`: `bool`

Attaches this button to a toolbar as a composite item. Called by the toolbar child wiring; not intended for direct use.

ToolbarButton also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ToolbarGroup`

Groups related child parts and coordinates their shared behavior.

[Source](../../src/toolbar/layers/toolbar_group.rs)

```rust
use base_gpui::toolbar::ToolbarGroup;

ToolbarGroup::new()
    .id("example-id")
    .disabled(true)
    .aria_label("aria label")
    .style_with_state(|/* callback arguments */| { /* handle change */ })
    .own_disabled()
    .split_children()
    .with_toolbar(/* context: ToolbarContext */, true, /* children */);
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ToolbarGroup with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Overrides the default `"toolbar-group"` element id. Give each group in a window a distinct, stable id so assistive technology sees stable accessibility nodes across frames.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<ToolbarGroupChild>) -> Self
```

**Accepts**

- `child`: `impl Into<ToolbarGroupChild>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<ToolbarGroupChild>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<ToolbarGroupChild>>`

Adds multiple typed children in iteration order.

#### `.disabled(...)`

```rust
pub fn disabled(mut self, disabled: bool) -> Self
```

**Accepts**

- `disabled`: `bool`

When true, prevents user interaction with this part.

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self
```

**Accepts**

- `aria_label`: `impl Into<SharedString>`

Accessible name for the group, announced by screen readers. There is no `aria-labelledby` id-reference builder in this gpui revision, so the name is a literal string.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ToolbarGroupStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ToolbarGroupStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

#### `.own_disabled(...)`

```rust
pub fn own_disabled(&self) -> bool
```

The group's own disabled prop, consumed by the toolbar child wiring when computing the merged cascade for contained items.

#### `.split_children(...)`

```rust
pub fn split_children(mut self) -> (Self, Vec<ToolbarGroupChild>)
```

Detaches the typed children so the toolbar child wiring can flatten them into the toolbar's single item order. Called by the toolbar child wiring; not intended for direct use.

#### `.with_toolbar(...)`

```rust
pub fn with_toolbar(mut self, context: ToolbarContext, merged_disabled: bool, children: Vec<ToolbarGroupChild>,) -> Self
```

**Accepts**

- `context`: `ToolbarContext`
- `merged_disabled`: `bool`
- `children`: `Vec<ToolbarGroupChild>`

Reattaches the wired children and the toolbar context. Called by the toolbar child wiring; not intended for direct use.

ToolbarGroup also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ToolbarInput`

Text input integrated with the component's state and behavior.

[Source](../../src/toolbar/layers/toolbar_input.rs)

```rust
use base_gpui::toolbar::ToolbarInput;

ToolbarInput::new()
    .id("example-id")
    .value(/* value: impl Into<SharedString> */)
    .default_value(/* default_value: impl Into<SharedString> */)
    .placeholder("placeholder")
    .on_value_change(|/* callback arguments */| { /* handle change */ })
    .on_enter(|/* callback arguments */| { /* handle change */ })
    .disabled(true)
    .focusable_when_disabled(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ })
    .own_disabled()
    .own_focusable_when_disabled()
    .item_id()
    .with_toolbar(/* context: ToolbarContext */, 0, /* focus_handle: FocusHandle */, true);
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ToolbarInput with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.value(...)`

```rust
pub fn value(mut self, value: impl Into<SharedString>) -> Self
```

**Accepts**

- `value`: `impl Into<SharedString>`

Sets the current controlled value or the value represented by this part, depending on the part's role.

#### `.default_value(...)`

```rust
pub fn default_value(mut self, default_value: impl Into<SharedString>) -> Self
```

**Accepts**

- `default_value`: `impl Into<SharedString>`

Sets the initial value for uncontrolled state. Later user changes are retained by the component.

#### `.placeholder(...)`

```rust
pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self
```

**Accepts**

- `placeholder`: `impl Into<SharedString>`

Sets the content shown when the component has no current value.

#### `.on_value_change(...)`

```rust
pub fn on_value_change(mut self, on_value_change: impl Fn(SharedString) + 'static) -> Self
```

**Accepts**

- `on_value_change`: `impl Fn(SharedString) + 'static`

Registers a callback invoked when value change occurs.

#### `.on_enter(...)`

```rust
pub fn on_enter(mut self, on_enter: impl Fn(SharedString) + 'static) -> Self
```

**Accepts**

- `on_enter`: `impl Fn(SharedString) + 'static`

Registers a callback invoked when enter occurs.

#### `.disabled(...)`

```rust
pub fn disabled(mut self, disabled: bool) -> Self
```

**Accepts**

- `disabled`: `bool`

When true, prevents user interaction with this part.

#### `.focusable_when_disabled(...)`

```rust
pub fn focusable_when_disabled(mut self, focusable_when_disabled: bool) -> Self
```

**Accepts**

- `focusable_when_disabled`: `bool`

Sets the focusable when disabled configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ToolbarInputStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ToolbarInputStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

#### `.own_disabled(...)`

```rust
pub fn own_disabled(&self) -> bool
```

The input's own disabled prop, consumed by the toolbar child wiring when resolving the effective per-item disabled fact.

#### `.own_focusable_when_disabled(...)`

```rust
pub fn own_focusable_when_disabled(&self) -> bool
```

The input's `focusable_when_disabled` flag, consumed by the toolbar child wiring for item metadata.

#### `.item_id(...)`

```rust
pub fn item_id(&self) -> &ElementId
```

The input's element id, consumed by the toolbar child wiring to key the roving focus handle.

#### `.with_toolbar(...)`

```rust
pub fn with_toolbar(mut self, context: ToolbarContext, index: usize, focus_handle: FocusHandle, cascade_disabled: bool,) -> Self
```

**Accepts**

- `context`: `ToolbarContext`
- `index`: `usize`
- `focus_handle`: `FocusHandle`
- `cascade_disabled`: `bool`

Attaches this input to a toolbar as a composite item. Called by the toolbar child wiring; not intended for direct use.

ToolbarInput also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ToolbarLink`

Public renderable part of the Toolbar Link component.

[Source](../../src/toolbar/layers/toolbar_link.rs)

```rust
use base_gpui::toolbar::ToolbarLink;

ToolbarLink::new()
    .id("example-id")
    .on_click(|/* callback arguments */| { /* handle change */ })
    .aria_label("aria label")
    .style_with_state(|/* callback arguments */| { /* handle change */ })
    .item_id()
    .with_toolbar(/* context: ToolbarContext */, 0, /* focus_handle: FocusHandle */);
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ToolbarLink with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.on_click(...)`

```rust
pub fn on_click(mut self, on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_click`: `impl Fn(&ClickEvent, &mut Window, &mut App) + 'static`

Registers a callback invoked when click occurs.

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self
```

**Accepts**

- `aria_label`: `impl Into<SharedString>`

Accessible name for icon-only links. When set alongside a visible text child, render that text with `Text::new_inaccessible(...)` instead of `text!(...)` so screen readers do not announce the name twice; without an `aria_label`, keep `text!(...)` so the child text remains the accessible name source.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ToolbarLinkStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ToolbarLinkStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

#### `.item_id(...)`

```rust
pub fn item_id(&self) -> &ElementId
```

The link's element id, consumed by the toolbar child wiring to key the roving focus handle.

#### `.with_toolbar(...)`

```rust
pub fn with_toolbar(mut self, context: ToolbarContext, index: usize, focus_handle: FocusHandle,) -> Self
```

**Accepts**

- `context`: `ToolbarContext`
- `index`: `usize`
- `focus_handle`: `FocusHandle`

Attaches this link to a toolbar as a composite item. Called by the toolbar child wiring; not intended for direct use.

ToolbarLink also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ToolbarSeparator`

Visual separator between neighboring items or groups.

[Source](../../src/toolbar/layers/toolbar_separator.rs)

```rust
use base_gpui::toolbar::ToolbarSeparator;

ToolbarSeparator::new()
    .orientation(/* orientation: SeparatorOrientation */)
    .style_with_state(|/* callback arguments */| { /* handle change */ })
    .with_toolbar(/* context: ToolbarContext */);
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ToolbarSeparator with its default configuration.

#### `.orientation(...)`

```rust
pub fn orientation(mut self, orientation: SeparatorOrientation) -> Self
```

**Accepts**

- `orientation`: `SeparatorOrientation`

Overrides the derived perpendicular orientation.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(SeparatorStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(SeparatorStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

#### `.with_toolbar(...)`

```rust
pub fn with_toolbar(mut self, context: ToolbarContext) -> Self
```

**Accepts**

- `context`: `ToolbarContext`

Attaches the toolbar context so the default orientation can be derived. Called by the toolbar child wiring; not intended for direct use.

ToolbarSeparator also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
