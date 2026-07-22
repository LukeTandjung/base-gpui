# Combobox

Roles/aria props and a11y actions are wired per the port issue's follow-up plan. The following Base UI ARIA attributes have **no gpui builder** in the pinned revision and are intentionally omitted:

[Base UI reference](https://base-ui.com/react/components/combobox) · [Source](../../src/combobox/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::combobox::{
    ComboboxArrow,
    ComboboxBackdrop,
    ComboboxChip,
    ComboboxChipRemove,
    ComboboxChips,
    ComboboxClear,
    ComboboxCollection,
    ComboboxEmpty,
    ComboboxGroup,
    ComboboxGroupLabel,
    ComboboxIcon,
    ComboboxInput,
    ComboboxInputGroup,
    ComboboxItem,
    ComboboxItemIndicator,
    ComboboxLabel,
    ComboboxList,
    ComboboxPopup,
    ComboboxPortal,
    ComboboxPositioner,
    ComboboxRoot,
    ComboboxSeparator,
    ComboboxStatus,
    ComboboxTrigger,
    ComboboxValue,
};

ComboboxRoot::new()
    .child(
        ComboboxChip::new()
                .child(
                    ComboboxChipRemove::new(),
                ),
    )
    .child(
        ComboboxInputGroup::new()
                .child(
                    ComboboxChips::new(),
                )
                .child(
                    ComboboxClear::new(),
                )
                .child(
                    ComboboxIcon::new(),
                )
                .child(
                    ComboboxInput::new(),
                )
                .child(
                    ComboboxTrigger::new(),
                )
                .child(
                    ComboboxValue::new(),
                ),
    )
    .child(
        ComboboxLabel::new(),
    )
    .child(
        ComboboxPortal::new()
                .child(
                    ComboboxBackdrop::new(),
                )
                .child(
                    ComboboxPositioner::new()
                                .child(
                                    ComboboxArrow::new(),
                                )
                                .child(
                                    ComboboxList::new()
                                                    .child(
                                                        ComboboxCollection::new(/* items: Vec<T> */, 0),
                                                    )
                                                    .child(
                                                        ComboboxGroup::new()
                                                                            .child(
                                                                                ComboboxGroupLabel::new(),
                                                                            )
                                                                            .child(
                                                                                ComboboxItem::new()
                                                                                                        .child(
                                                                                                            ComboboxItemIndicator::new(),
                                                                                                        ),
                                                                            )
                                                                            .child(
                                                                                ComboboxSeparator::new(),
                                                                            ),
                                                    ),
                                )
                                .child(
                                    ComboboxPopup::new()
                                                    .child(
                                                        ComboboxEmpty::new(),
                                                    )
                                                    .child(
                                                        ComboboxStatus::new(),
                                                    ),
                                ),
                ),
    );
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `ComboboxRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/combobox/layers/combobox_root.rs)

```rust
use base_gpui::combobox::ComboboxRoot;

ComboboxRoot::new()
    .id("example-id")
    .name("name")
    .default_value(None)
    .value(None)
    .on_value_change(|/* callback arguments */| { /* handle change */ })
    .multiple(true)
    .default_values(/* default_values: Vec<T> */)
    .values(/* values: Vec<T> */)
    .on_values_change(|/* callback arguments */| { /* handle change */ })
    .default_input_value(/* default_input_value: impl Into<SharedString> */)
    .input_value(/* input_value: impl Into<SharedString> */)
    .on_input_value_change(|/* callback arguments */| { /* handle change */ })
    .default_open(true)
    .open(true)
    .on_open_change(|/* callback arguments */| { /* handle change */ })
    .on_item_highlighted(|/* callback arguments */| { /* handle change */ })
    .disabled(true)
    .read_only(true)
    .required(true)
    .open_on_input_click(true)
    .auto_highlight(/* auto_highlight: ComboboxAutoHighlight */)
    .highlight_item_on_hover(true)
    .loop_focus(true)
    .limit(0)
    .filter(|/* callback arguments */| { /* handle change */ })
    .filter_none()
    .item_to_string_label(|/* callback arguments */| { /* handle change */ })
    .item_to_string_value(|/* callback arguments */| { /* handle change */ })
    .multiple_value_formatter(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ComboboxRoot with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<ComboboxChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<ComboboxChild<T>>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<ComboboxChild<T>>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<ComboboxChild<T>>>`

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

#### `.name(...)`

```rust
pub fn name(mut self, name: impl Into<SharedString>) -> Self
```

**Accepts**

- `name`: `impl Into<SharedString>`

Sets the form field name used when serializing or submitting the value.

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

#### `.on_value_change(...)`

```rust
pub fn on_value_change(mut self, on_value_change: impl Fn(Option<&T>, &mut ComboboxChangeDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_value_change`: `impl Fn(Option<&T>, &mut ComboboxChangeDetails, &mut Window, &mut App)
            + 'static`

Registers a callback invoked when value change occurs.

#### `.multiple(...)`

```rust
pub fn multiple(mut self, multiple: bool) -> Self
```

**Accepts**

- `multiple`: `bool`

Controls whether more than one value may be selected at the same time.

#### `.default_values(...)`

```rust
pub fn default_values(mut self, default_values: Vec<T>) -> Self
```

**Accepts**

- `default_values`: `Vec<T>`

Sets the initial values for uncontrolled state.

#### `.values(...)`

```rust
pub fn values(mut self, values: Vec<T>) -> Self
```

**Accepts**

- `values`: `Vec<T>`

Sets the values configuration for this part.

#### `.on_values_change(...)`

```rust
pub fn on_values_change(mut self, on_values_change: impl Fn(&[T], &mut ComboboxChangeDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_values_change`: `impl Fn(&[T], &mut ComboboxChangeDetails, &mut Window, &mut App) + 'static`

Registers a callback invoked when values change occurs.

#### `.default_input_value(...)`

```rust
pub fn default_input_value(mut self, default_input_value: impl Into<SharedString>) -> Self
```

**Accepts**

- `default_input_value`: `impl Into<SharedString>`

Sets the initial input value for uncontrolled state.

#### `.input_value(...)`

```rust
pub fn input_value(mut self, input_value: impl Into<SharedString>) -> Self
```

**Accepts**

- `input_value`: `impl Into<SharedString>`

Sets the input value configuration for this part.

#### `.on_input_value_change(...)`

```rust
pub fn on_input_value_change(mut self, on_input_value_change: impl Fn(&SharedString, &mut ComboboxChangeDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_input_value_change`: `impl Fn(&SharedString, &mut ComboboxChangeDetails, &mut Window, &mut App)
            + 'static`

Registers a callback invoked when input value change occurs.

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
pub fn on_open_change(mut self, on_open_change: impl Fn(bool, &mut ComboboxChangeDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_open_change`: `impl Fn(bool, &mut ComboboxChangeDetails, &mut Window, &mut App) + 'static`

Registers a callback invoked when open change occurs.

#### `.on_item_highlighted(...)`

```rust
pub fn on_item_highlighted(mut self, on_item_highlighted: impl Fn(Option<&T>, &ComboboxItemHighlightDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_item_highlighted`: `impl Fn(Option<&T>, &ComboboxItemHighlightDetails, &mut Window, &mut App)
            + 'static`

Registers a callback invoked when item highlighted occurs.

#### `.disabled(...)`

```rust
pub fn disabled(mut self, disabled: bool) -> Self
```

**Accepts**

- `disabled`: `bool`

When true, prevents user interaction with this part.

#### `.read_only(...)`

```rust
pub fn read_only(mut self, read_only: bool) -> Self
```

**Accepts**

- `read_only`: `bool`

When true, allows presentation and focus behavior without permitting value changes.

#### `.required(...)`

```rust
pub fn required(mut self, required: bool) -> Self
```

**Accepts**

- `required`: `bool`

Marks the control as required for validation and accessibility state.

#### `.open_on_input_click(...)`

```rust
pub fn open_on_input_click(mut self, open_on_input_click: bool) -> Self
```

**Accepts**

- `open_on_input_click`: `bool`

Controls whether open on input click behavior is enabled.

#### `.auto_highlight(...)`

```rust
pub fn auto_highlight(mut self, auto_highlight: ComboboxAutoHighlight) -> Self
```

**Accepts**

- `auto_highlight`: `ComboboxAutoHighlight`

Controls whether auto highlight behavior is enabled.

#### `.highlight_item_on_hover(...)`

```rust
pub fn highlight_item_on_hover(mut self, highlight_item_on_hover: bool) -> Self
```

**Accepts**

- `highlight_item_on_hover`: `bool`

Controls whether highlight item on hover behavior is enabled.

#### `.loop_focus(...)`

```rust
pub fn loop_focus(mut self, loop_focus: bool) -> Self
```

**Accepts**

- `loop_focus`: `bool`

Controls whether keyboard focus wraps from the last enabled item to the first and vice versa.

#### `.limit(...)`

```rust
pub fn limit(mut self, limit: usize) -> Self
```

**Accepts**

- `limit`: `usize`

Sets the limit configuration for this part.

#### `.filter(...)`

```rust
pub fn filter(mut self, filter: impl Fn(&T, Option<&SharedString>, &str) -> bool + 'static,) -> Self
```

**Accepts**

- `filter`: `impl Fn(&T, Option<&SharedString>, &str) -> bool + 'static,`

Custom filter replacing the default case-insensitive contains match.

#### `.filter_none(...)`

```rust
pub fn filter_none(mut self) -> Self
```

Disables internal filtering for externally filtered lists.

#### `.item_to_string_label(...)`

```rust
pub fn item_to_string_label(mut self, resolver: impl Fn(&T) -> SharedString + 'static) -> Self
```

**Accepts**

- `resolver`: `impl Fn(&T) -> SharedString + 'static`

Sets the item to string label configuration for this part.

#### `.item_to_string_value(...)`

```rust
pub fn item_to_string_value(mut self, serializer: impl Fn(&T) -> SharedString + 'static,) -> Self
```

**Accepts**

- `serializer`: `impl Fn(&T) -> SharedString + 'static,`

Sets the item to string value configuration for this part.

#### `.multiple_value_formatter(...)`

```rust
pub fn multiple_value_formatter(mut self, formatter: impl Fn(&[SharedString], &[T]) -> SharedString + 'static,) -> Self
```

**Accepts**

- `formatter`: `impl Fn(&[SharedString], &[T]) -> SharedString + 'static,`

Sets the multiple value formatter configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ComboboxRootStyleState<T>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ComboboxRootStyleState<T>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ComboboxRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ComboboxTrigger`

Interactive control that opens, closes, or activates the component.

[Source](../../src/combobox/layers/combobox_trigger.rs)

```rust
use base_gpui::combobox::ComboboxTrigger;

ComboboxTrigger::new()
    .id("example-id")
    .disabled(true)
    .aria_label("label")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ComboboxTrigger with its default configuration.

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

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Accessible label for the trigger button (e.g. "Open suggestions").

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ComboboxTriggerStyleState<T>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ComboboxTriggerStyleState<T>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ComboboxTrigger also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ComboboxPortal`

Hosts overlay content outside the normal child layout.

[Source](../../src/combobox/layers/combobox_portal.rs)

```rust
use base_gpui::combobox::ComboboxPortal;

ComboboxPortal::new()
    .force_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ComboboxPortal with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<ComboboxPortalChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<ComboboxPortalChild<T>>`

Adds one typed child to this part.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.force_mounted(...)`

```rust
pub fn force_mounted(mut self, force_mounted: bool) -> Self
```

**Accepts**

- `force_mounted`: `bool`

Controls whether force mounted behavior is enabled.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ComboboxPortalStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ComboboxPortalStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ComboboxPortal also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ComboboxPositioner`

Measures the anchor and positions floating content.

[Source](../../src/combobox/layers/combobox_positioner.rs)

```rust
use base_gpui::combobox::ComboboxPositioner;

ComboboxPositioner::new()
    .anchor(/* anchor: Bounds<Pixels> */)
    .side("example-id")
    .align(/* align: ComboboxAlign */)
    .side_offset("example-id")
    .align_offset(/* align_offset: Pixels */)
    .collision_padding(/* collision_padding: Pixels */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ComboboxPositioner with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<ComboboxPositionerChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<ComboboxPositionerChild<T>>`

Adds one typed child to this part.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.anchor(...)`

```rust
pub fn anchor(mut self, anchor: Bounds<Pixels>) -> Self
```

**Accepts**

- `anchor`: `Bounds<Pixels>`

Explicit anchor override; defaults to input-group-else-input.

#### `.side(...)`

```rust
pub fn side(mut self, side: ComboboxSide) -> Self
```

**Accepts**

- `side`: `ComboboxSide`

Sets the preferred side of the anchor on which floating content appears.

#### `.align(...)`

```rust
pub fn align(mut self, align: ComboboxAlign) -> Self
```

**Accepts**

- `align`: `ComboboxAlign`

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

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ComboboxPositionerStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ComboboxPositionerStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ComboboxPositioner also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ComboboxPopup`

Contains the floating interactive content.

[Source](../../src/combobox/layers/combobox_popup.rs)

```rust
use base_gpui::combobox::ComboboxPopup;

ComboboxPopup::new()
    .side("example-id")
    .align(/* align: ComboboxAlign */)
    .force_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ComboboxPopup with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<ComboboxPopupChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<ComboboxPopupChild<T>>`

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
pub fn side(mut self, side: ComboboxSide) -> Self
```

**Accepts**

- `side`: `ComboboxSide`

Sets the preferred side of the anchor on which floating content appears.

#### `.align(...)`

```rust
pub fn align(mut self, align: ComboboxAlign) -> Self
```

**Accepts**

- `align`: `ComboboxAlign`

Sets floating content alignment along the selected side.

#### `.force_mounted(...)`

```rust
pub fn force_mounted(mut self, force_mounted: bool) -> Self
```

**Accepts**

- `force_mounted`: `bool`

Controls whether force mounted behavior is enabled.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ComboboxPopupStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ComboboxPopupStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ComboboxPopup also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ComboboxArrow`

Optional arrow that visually points toward the overlay anchor.

[Source](../../src/combobox/layers/combobox_arrow.rs)

```rust
use base_gpui::combobox::ComboboxArrow;

ComboboxArrow::new()
    .side("example-id")
    .align(/* align: ComboboxAlign */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ComboboxArrow with its default configuration.

#### `.side(...)`

```rust
pub fn side(mut self, side: ComboboxSide) -> Self
```

**Accepts**

- `side`: `ComboboxSide`

Sets the preferred side of the anchor on which floating content appears.

#### `.align(...)`

```rust
pub fn align(mut self, align: ComboboxAlign) -> Self
```

**Accepts**

- `align`: `ComboboxAlign`

Sets floating content alignment along the selected side.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ComboboxArrowStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ComboboxArrowStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ComboboxArrow also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ComboboxBackdrop`

Covers content behind a modal overlay and handles outside interaction.

[Source](../../src/combobox/layers/combobox_backdrop.rs)

```rust
use base_gpui::combobox::ComboboxBackdrop;

ComboboxBackdrop::new()
    .force_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ComboboxBackdrop with its default configuration.

#### `.force_mounted(...)`

```rust
pub fn force_mounted(mut self, force_mounted: bool) -> Self
```

**Accepts**

- `force_mounted`: `bool`

Controls whether force mounted behavior is enabled.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ComboboxBackdropStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ComboboxBackdropStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ComboboxBackdrop also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ComboboxChip`

Public renderable part of the Combobox Chip component.

[Source](../../src/combobox/layers/combobox_chip.rs)

```rust
use base_gpui::combobox::ComboboxChip;

ComboboxChip::new()
    .index(0)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ComboboxChip with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<ComboboxChipChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<ComboboxChipChild<T>>`

Adds one typed child to this part.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.index(...)`

```rust
pub fn index(mut self, index: usize) -> Self
```

**Accepts**

- `index`: `usize`

Selection-order position of this chip; assigned by `ComboboxChips`.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ComboboxChipStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ComboboxChipStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ComboboxChip also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ComboboxChipRemove`

Public renderable part of the Combobox Chip Remove component.

[Source](../../src/combobox/layers/combobox_chip_remove.rs)

```rust
use base_gpui::combobox::ComboboxChipRemove;

ComboboxChipRemove::new()
    .index(0)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ComboboxChipRemove with its default configuration.

#### `.index(...)`

```rust
pub fn index(mut self, index: usize) -> Self
```

**Accepts**

- `index`: `usize`

Selection-order position of the owning chip; assigned by `ComboboxChip`.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ComboboxChipRemoveStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ComboboxChipRemoveStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ComboboxChipRemove also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ComboboxChips`

Public renderable part of the Combobox Chips component.

[Source](../../src/combobox/layers/combobox_chips.rs)

```rust
use base_gpui::combobox::ComboboxChips;

ComboboxChips::new()
    .chip_builder(0)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ComboboxChips with its default configuration.

#### `.chip_builder(...)`

```rust
pub fn chip_builder(mut self, builder: impl Fn(&T, usize) -> ComboboxChip<T> + 'static,) -> Self
```

**Accepts**

- `builder`: `impl Fn(&T, usize) -> ComboboxChip<T> + 'static,`

Sets the chip builder configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ComboboxChipsStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ComboboxChipsStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ComboboxChips also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ComboboxClear`

Public renderable part of the Combobox Clear component.

[Source](../../src/combobox/layers/combobox_clear.rs)

```rust
use base_gpui::combobox::ComboboxClear;

ComboboxClear::new()
    .id("example-id")
    .keep_mounted(true)
    .aria_label("label")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ComboboxClear with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.keep_mounted(...)`

```rust
pub fn keep_mounted(mut self, keep_mounted: bool) -> Self
```

**Accepts**

- `keep_mounted`: `bool`

Keeps the part mounted when inactive or closed so child state can be preserved.

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Accessible label for the clear button; defaults to "Clear".

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ComboboxClearStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ComboboxClearStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ComboboxClear also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ComboboxCollection`

Public renderable part of the Combobox Collection component.

[Source](../../src/combobox/layers/combobox_collection.rs)

```rust
use base_gpui::combobox::ComboboxCollection;

ComboboxCollection::new();
```

### Builders

#### `.new(...)`

```rust
pub fn new(items: Vec<T>, builder: impl Fn(&T, usize) -> ComboboxItem<T> + 'static) -> Self
```

**Accepts**

- `items`: `Vec<T>`
- `builder`: `impl Fn(&T, usize) -> ComboboxItem<T> + 'static`

Creates a ComboboxCollection with its default configuration.

ComboboxCollection also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ComboboxEmpty`

Public renderable part of the Combobox Empty component.

[Source](../../src/combobox/layers/combobox_empty.rs)

```rust
use base_gpui::combobox::ComboboxEmpty;

ComboboxEmpty::new()
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ComboboxEmpty with its default configuration.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ComboboxEmptyStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ComboboxEmptyStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ComboboxEmpty also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ComboboxGroup`

Groups related child parts and coordinates their shared behavior.

[Source](../../src/combobox/layers/combobox_group.rs)

```rust
use base_gpui::combobox::ComboboxGroup;

ComboboxGroup::new()
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ComboboxGroup with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<ComboboxGroupChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<ComboboxGroupChild<T>>`

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
pub fn style_with_state(mut self, style: impl Fn(ComboboxGroupStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ComboboxGroupStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ComboboxGroup also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ComboboxGroupLabel`

Provides a visible label and associated accessibility semantics.

[Source](../../src/combobox/layers/combobox_group_label.rs)

```rust
use base_gpui::combobox::ComboboxGroupLabel;

ComboboxGroupLabel::new()
    .label("label")
    .text("label")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ComboboxGroupLabel with its default configuration.

#### `.label(...)`

```rust
pub fn label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Sets the label configuration for this part.

#### `.text(...)`

```rust
pub fn text(self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Sets the text configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ComboboxGroupLabelStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ComboboxGroupLabelStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ComboboxGroupLabel also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ComboboxIcon`

Optional visual icon for the component.

[Source](../../src/combobox/layers/combobox_icon.rs)

```rust
use base_gpui::combobox::ComboboxIcon;

ComboboxIcon::new()
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ComboboxIcon with its default configuration.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ComboboxIconStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ComboboxIconStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ComboboxIcon also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ComboboxInput`

Text input integrated with the component's state and behavior.

[Source](../../src/combobox/layers/combobox_input.rs)

```rust
use base_gpui::combobox::ComboboxInput;

ComboboxInput::new()
    .id("example-id")
    .placeholder("placeholder")
    .aria_label("label")
    .disabled(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ })
    .input_style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ComboboxInput with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.placeholder(...)`

```rust
pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self
```

**Accepts**

- `placeholder`: `impl Into<SharedString>`

Sets the content shown when the component has no current value.

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Accessible label for the combobox reference node; substitutes for Base UI's `aria-labelledby` wiring, which has no gpui builder.

#### `.disabled(...)`

```rust
pub fn disabled(mut self, disabled: bool) -> Self
```

**Accepts**

- `disabled`: `bool`

When true, prevents user interaction with this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ComboboxInputStyleState<T>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ComboboxInputStyleState<T>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

#### `.input_style_with_state(...)`

```rust
pub fn input_style_with_state(mut self, style: impl Fn(InputStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(InputStyleState, Div) -> Div + 'static,`

Styling hook for the inner input primitive; composes with, not replaces, the Combobox-level `style_with_state`.

ComboboxInput also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ComboboxInputGroup`

Groups related child parts and coordinates their shared behavior.

[Source](../../src/combobox/layers/combobox_input_group.rs)

```rust
use base_gpui::combobox::ComboboxInputGroup;

ComboboxInputGroup::new()
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ComboboxInputGroup with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<ComboboxInputGroupChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<ComboboxInputGroupChild<T>>`

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
pub fn style_with_state(mut self, style: impl Fn(ComboboxInputGroupStyleState<T>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ComboboxInputGroupStyleState<T>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ComboboxInputGroup also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ComboboxItem`

Represents one interactive item in the component's collection.

[Source](../../src/combobox/layers/combobox_item.rs)

```rust
use base_gpui::combobox::ComboboxItem;

ComboboxItem::new()
    .id("example-id")
    .value(/* value: T */)
    .label("label")
    .disabled(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ComboboxItem with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<ComboboxItemChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<ComboboxItemChild<T>>`

Adds one typed child to this part.

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

#### `.value(...)`

```rust
pub fn value(mut self, value: T) -> Self
```

**Accepts**

- `value`: `T`

Sets the current controlled value or the value represented by this part, depending on the part's role.

#### `.label(...)`

```rust
pub fn label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Label used for display and filtering.

#### `.disabled(...)`

```rust
pub fn disabled(mut self, disabled: bool) -> Self
```

**Accepts**

- `disabled`: `bool`

When true, prevents user interaction with this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ComboboxItemStyleState<T>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ComboboxItemStyleState<T>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ComboboxItem also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ComboboxItemIndicator`

Visual indicator for an item's selected or checked state.

[Source](../../src/combobox/layers/combobox_item_indicator.rs)

```rust
use base_gpui::combobox::ComboboxItemIndicator;

ComboboxItemIndicator::new()
    .keep_mounted(true)
    .with_item_state(/* state: ComboboxItemStyleState<T> */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ComboboxItemIndicator with its default configuration.

#### `.keep_mounted(...)`

```rust
pub fn keep_mounted(mut self, keep_mounted: bool) -> Self
```

**Accepts**

- `keep_mounted`: `bool`

Keeps the part mounted when inactive or closed so child state can be preserved.

#### `.with_item_state(...)`

```rust
pub fn with_item_state(mut self, state: ComboboxItemStyleState<T>) -> Self
```

**Accepts**

- `state`: `ComboboxItemStyleState<T>`

Sets the with item state configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ComboboxItemIndicatorStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ComboboxItemIndicatorStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ComboboxItemIndicator also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ComboboxLabel`

Provides a visible label and associated accessibility semantics.

[Source](../../src/combobox/layers/combobox_label.rs)

```rust
use base_gpui::combobox::ComboboxLabel;

ComboboxLabel::new()
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ComboboxLabel with its default configuration.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ComboboxLabelStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ComboboxLabelStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ComboboxLabel also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ComboboxList`

Contains and coordinates the component's collection items.

[Source](../../src/combobox/layers/combobox_list.rs)

```rust
use base_gpui::combobox::ComboboxList;

ComboboxList::new()
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ComboboxList with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<ComboboxListChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<ComboboxListChild<T>>`

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
pub fn style_with_state(mut self, style: impl Fn(ComboboxListStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ComboboxListStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ComboboxList also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ComboboxSeparator`

Visual separator between neighboring items or groups.

[Source](../../src/combobox/layers/combobox_separator.rs)

```rust
use base_gpui::combobox::ComboboxSeparator;

ComboboxSeparator::new()
    .orientation(/* orientation: SeparatorOrientation */)
    .horizontal()
    .vertical()
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ComboboxSeparator with its default configuration.

#### `.orientation(...)`

```rust
pub fn orientation(mut self, orientation: SeparatorOrientation) -> Self
```

**Accepts**

- `orientation`: `SeparatorOrientation`

Sets the component's horizontal or vertical orientation and corresponding keyboard behavior.

#### `.horizontal(...)`

```rust
pub fn horizontal(self) -> Self
```

Sets the horizontal configuration for this part.

#### `.vertical(...)`

```rust
pub fn vertical(self) -> Self
```

Sets the vertical configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(SeparatorStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(SeparatorStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ComboboxSeparator also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ComboboxStatus`

Public renderable part of the Combobox Status component.

[Source](../../src/combobox/layers/combobox_status.rs)

```rust
use base_gpui::combobox::ComboboxStatus;

ComboboxStatus::new()
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ComboboxStatus with its default configuration.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ComboboxStatusStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ComboboxStatusStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ComboboxStatus also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `ComboboxValue`

Displays or edits the component's current value.

[Source](../../src/combobox/layers/combobox_value.rs)

```rust
use base_gpui::combobox::ComboboxValue;

ComboboxValue::new()
    .placeholder("placeholder")
    .formatter(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a ComboboxValue with its default configuration.

#### `.placeholder(...)`

```rust
pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self
```

**Accepts**

- `placeholder`: `impl Into<SharedString>`

Sets the content shown when the component has no current value.

#### `.formatter(...)`

```rust
pub fn formatter(mut self, formatter: impl Fn(&ComboboxValueStyleState<T>) -> SharedString + 'static,) -> Self
```

**Accepts**

- `formatter`: `impl Fn(&ComboboxValueStyleState<T>) -> SharedString + 'static,`

Rust-native formatter closure over the current selection.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(ComboboxValueStyleState<T>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(ComboboxValueStyleState<T>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

ComboboxValue also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
