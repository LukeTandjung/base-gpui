<!-- curated-component-guide -->

# Tabs

Tabs organize related content into panels and show one panel at a time. They support mouse and keyboard selection, horizontal or vertical layouts, controlled and uncontrolled state, disabled tabs, and an optional animated indicator.

[Base UI reference](https://base-ui.com/react/components/tabs) · [Source](../../src/tabs/mod.rs)

## Anatomy

Import the parts and compose them under `TabsRoot`:

```rust
use base_gpui::tabs::{
    TabsIndicator, TabsList, TabsPanel, TabsRoot, TabsTab,
};

TabsRoot::new()
    .default_value(Some("overview"))
    .child(
        TabsList::new()
            .child(
                TabsTab::new()
                    .value("overview")
                    .child("Overview"),
            )
            .child(
                TabsTab::new()
                    .value("settings")
                    .child("Settings"),
            )
            .child(TabsIndicator::new()),
    )
    .child(
        TabsPanel::new()
            .value("overview")
            .id("overview-panel")
            .child("Overview content"),
    )
    .child(
        TabsPanel::new()
            .value("settings")
            .id("settings-panel")
            .child("Settings content"),
    )
```

Every `TabsTab` and its matching `TabsPanel` must use the same value. `TabsIndicator` belongs inside `TabsList` so it can use the measured tab positions.

## `TabsRoot`

The top-level container. It owns the selected value, coordinates all parts, and defines the orientation.

```rust
use base_gpui::tabs::{TabsOrientation, TabsRoot};

TabsRoot::new()
    .id("account-tabs")
    .default_value(Some("profile"))
    .orientation(TabsOrientation::Horizontal)
    .on_value_change(|value, window, cx| {
        // `value` is the newly selected `Option<&T>`.
    })
    .style_with_state(|state, root| {
        // `state.orientation`
        // `state.activation_direction`
        root
    })
    .child(/* TabsList or TabsPanel */)
```

### Builders

#### `.id(id)`

```rust
pub fn id(self, id: impl Into<ElementId>) -> Self
```

Sets the stable identity used to scope the tabs' runtime state. The default is `"tabs"`; set a unique ID when multiple tab roots may appear in the same view.

#### `.value(value)`

```rust
pub fn value(self, value: Option<T>) -> Self
```

Controls the selected tab. Passing this builder makes the root controlled: user interaction calls `on_value_change`, but the selected value changes only when the caller supplies a new `value`.

#### `.default_value(default_value)`

```rust
pub fn default_value(self, default_value: Option<T>) -> Self
```

Sets the initial selected tab for an uncontrolled root. Later changes to `default_value` do not replace the user's current selection.

Use either `value` or `default_value`, not both.

#### `.on_value_change(on_value_change)`

```rust
pub fn on_value_change(
    self,
    on_value_change: impl Fn(Option<&T>, &mut Window, &mut App) + 'static,
) -> Self
```

Runs when selection is requested. The first argument is the next selected value, or `None` when no tab is selected.

#### `.orientation(orientation)`

```rust
pub fn orientation(self, orientation: TabsOrientation) -> Self
```

Sets `TabsOrientation::Horizontal` or `TabsOrientation::Vertical`. Orientation controls style state, accessibility metadata, and which arrow keys move between tabs. The default is `Horizontal`.

#### `.child(child)` / `.children(children)`

```rust
pub fn child(self, child: impl Into<TabsChild<T>>) -> Self

pub fn children(
    self,
    children: impl IntoIterator<Item = impl Into<TabsChild<T>>>,
) -> Self
```

Adds a `TabsList` or `TabsPanel` to the root.

#### `.style_with_state(style)`

```rust
pub fn style_with_state(
    self,
    style: impl Fn(TabsRootStyleState, Div) -> Div + 'static,
) -> Self
```

Styles the root using its `orientation` and `activation_direction` state.

## `TabsList`

Contains the tabs and optional indicator. It owns keyboard navigation and exposes the tab-list accessibility role.

```rust
use base_gpui::tabs::{TabsIndicator, TabsList, TabsTab};

TabsList::new()
    .activate_on_focus(false)
    .loop_focus(true)
    .aria_label("Account sections")
    .style_with_state(|state, list| {
        // `state.orientation`
        // `state.activation_direction`
        list
    })
    .child(TabsTab::new().value("profile").child("Profile"))
    .child(TabsIndicator::new())
```

### Builders

#### `.activate_on_focus(activate_on_focus)`

```rust
pub fn activate_on_focus(self, activate_on_focus: bool) -> Self
```

When `true`, moving keyboard focus to a tab immediately selects it. When `false`, arrow keys only move focus and the user selects with Enter or Space. The default is `false`.

#### `.loop_focus(loop_focus)`

```rust
pub fn loop_focus(self, loop_focus: bool) -> Self
```

Controls whether keyboard navigation wraps from the final tab to the first tab and vice versa. The default is `true`.

#### `.aria_label(aria_label)`

```rust
pub fn aria_label(self, aria_label: impl Into<SharedString>) -> Self
```

Provides an accessible name for the tab list. Use it when the list has no accessible visible heading. GPUI does not yet expose `aria-labelledby` relationship wiring.

#### `.child(child)` / `.children(children)`

```rust
pub fn child(self, child: impl Into<TabsListChild<T>>) -> Self

pub fn children(
    self,
    children: impl IntoIterator<Item = impl Into<TabsListChild<T>>>,
) -> Self
```

Adds a `TabsTab` or `TabsIndicator` to the list.

#### `.style_with_state(style)`

```rust
pub fn style_with_state(
    self,
    style: impl Fn(TabsListStyleState, Div) -> Div + 'static,
) -> Self
```

Styles the list using its `orientation` and `activation_direction` state.

## `TabsTab`

An interactive tab. It participates in roving focus, selects its matching panel, and exposes tab accessibility semantics.

```rust
use base_gpui::tabs::TabsTab;

TabsTab::new()
    .id("profile-tab")
    .value("profile")
    .disabled(false)
    .aria_label("Profile")
    .style_with_state(|state, tab| {
        // `state.active`
        // `state.disabled`
        // `state.highlighted`
        // `state.orientation`
        tab
    })
    .child("Profile")
```

### Builders

#### `.value(value)`

```rust
pub fn value(self, value: T) -> Self
```

Associates the tab with a selection value. Use the same value on the corresponding `TabsPanel`.

#### `.id(id)`

```rust
pub fn id(self, id: impl Into<ElementId>) -> Self
```

Sets the tab's stable element identity. The default is `"tabs-tab"`; provide unique IDs when rendering multiple tabs.

#### `.disabled(disabled)`

```rust
pub fn disabled(self, disabled: bool) -> Self
```

Prevents selection and removes the tab from keyboard navigation when `true`. The default is `false`.

#### `.index(index)`

```rust
pub fn index(self, index: usize) -> Self
```

Explicitly sets the tab's zero-based position. Normal compound composition assigns indices automatically, so application code generally should not call this builder.

#### `.aria_label(aria_label)`

```rust
pub fn aria_label(self, aria_label: impl Into<SharedString>) -> Self
```

Overrides the tab's accessible name. This is primarily useful for icon-only tabs or when the accessible label should differ from visible content.

#### `.style_with_state(style)`

```rust
pub fn style_with_state(
    self,
    style: impl Fn(TabsTabStyleState, Div) -> Div + 'static,
) -> Self
```

Styles the tab using `active`, `disabled`, `highlighted`, and `orientation`.

`TabsTab` also implements GPUI's `ParentElement` and `Styled` traits, so it accepts visible children and standard GPUI style builders.

## `TabsPanel`

Contains the content associated with one tab. Inactive panels are unmounted by default.

```rust
use base_gpui::tabs::TabsPanel;

TabsPanel::new()
    .id("profile-panel")
    .value("profile")
    .keep_mounted(false)
    .style_with_state(|state, panel| {
        // `state.hidden`
        // `state.orientation`
        // `state.activation_direction`
        panel
    })
    .child("Profile content")
```

### Builders

#### `.value(value)`

```rust
pub fn value(self, value: T) -> Self
```

Associates the panel with a tab value. The panel is active when this value equals the root's selected value.

#### `.id(id)`

```rust
pub fn id(self, id: impl Into<ElementId>) -> Self
```

Sets the panel's stable element ID. An ID is required for the active panel to enter the accessibility tree with `Role::TabPanel`.

#### `.keep_mounted(keep_mounted)`

```rust
pub fn keep_mounted(self, keep_mounted: bool) -> Self
```

Keeps inactive panel content mounted but invisible when `true`. This is useful when panel-local state or expensive child construction must survive selection changes. The default is `false`.

#### `.style_with_state(style)`

```rust
pub fn style_with_state(
    self,
    style: impl Fn(TabsPanelStyleState, Div) -> Div + 'static,
) -> Self
```

Styles the panel using `hidden`, `orientation`, and `activation_direction`.

`TabsPanel` implements `ParentElement` and `Styled`.

## `TabsIndicator`

An optional visual marker for the selected tab. It renders only after the selected tab's bounds have been measured.

```rust
use base_gpui::tabs::TabsIndicator;

TabsIndicator::new()
    .style_with_state(|state, indicator| {
        // `state.selected`
        // `state.active_tab_position`
        // `state.active_tab_size`
        // `state.orientation`
        // `state.activation_direction`
        indicator
    })
```

### Builders

#### `.style_with_state(style)`

```rust
pub fn style_with_state(
    self,
    style: impl Fn(TabsIndicatorStyleState, Div) -> Div + 'static,
) -> Self
```

Styles and positions the indicator from the selected tab's measured position and size. `active_tab_position` and `active_tab_size` are `Option` values because measurement is not available before layout.

`TabsIndicator` implements `ParentElement` and `Styled`.

## Keyboard interaction

| Key | Behavior |
| --- | --- |
| Left / Right | Moves between tabs in horizontal orientation |
| Up / Down | Moves between tabs in vertical orientation |
| Home | Moves to the first enabled tab |
| End | Moves to the last enabled tab |
| Enter / Space | Selects the focused tab when `activate_on_focus` is `false` |

Disabled tabs are skipped. Navigation wraps when `loop_focus` is enabled.

## Accessibility

`TabsList` exposes `Role::TabList`, each `TabsTab` exposes `Role::Tab` and selected state, and the active `TabsPanel` exposes `Role::TabPanel`. Roving focus ensures one enabled tab is in the tab order.

The pinned GPUI revision does not expose ID-reference relationship builders such as `aria-controls` or `aria-labelledby`. Use literal `aria_label` values where provided; these limitations are not silently emulated.
