# Select

GPUI port of Base UI Select.

[Base UI reference](https://base-ui.com/react/components/select) · [Source](../../src/select/mod.rs)

## Anatomy

Import the component's parts and compose them under its root:

```rust
use base_gpui::select::{
    SelectArrow,
    SelectBackdrop,
    SelectGroup,
    SelectGroupLabel,
    SelectIcon,
    SelectItem,
    SelectItemIndicator,
    SelectItemText,
    SelectLabel,
    SelectList,
    SelectPopup,
    SelectPortal,
    SelectPositioner,
    SelectRoot,
    SelectScrollDownArrow,
    SelectScrollUpArrow,
    SelectSeparator,
    SelectTrigger,
    SelectValue,
};

SelectRoot::new()
    .child(
        SelectLabel::new(),
    )
    .child(
        SelectPortal::new()
                .child(
                    SelectBackdrop::new(),
                )
                .child(
                    SelectPositioner::new()
                                .child(
                                    SelectArrow::new(),
                                )
                                .child(
                                    SelectList::new()
                                                    .child(
                                                        SelectGroup::new()
                                                                            .child(
                                                                                SelectGroupLabel::new(),
                                                                            )
                                                                            .child(
                                                                                SelectItem::new()
                                                                                                        .child(
                                                                                                            SelectItemIndicator::new(),
                                                                                                        )
                                                                                                        .child(
                                                                                                            SelectItemText::new(),
                                                                                                        ),
                                                                            )
                                                                            .child(
                                                                                SelectSeparator::new(),
                                                                            ),
                                                    ),
                                )
                                .child(
                                    SelectPopup::new()
                                                    .child(
                                                        SelectScrollDownArrow::new(),
                                                    )
                                                    .child(
                                                        SelectScrollUpArrow::new(),
                                                    ),
                                ),
                ),
    )
    .child(
        SelectTrigger::new()
                .child(
                    SelectIcon::new(),
                )
                .child(
                    SelectValue::new(),
                ),
    );
```

> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.

## `SelectRoot`

Coordinates the component's state and supplies context to its child parts.

[Source](../../src/select/layers/select_root.rs)

```rust
use base_gpui::select::SelectRoot;

SelectRoot::new()
    .id("example-id")
    .name("name")
    .form(/* form: impl Into<SharedString> */)
    .default_value(None)
    .value(None)
    .on_value_change(|/* callback arguments */| { /* handle change */ })
    .default_open(true)
    .open(true)
    .on_open_change(|/* callback arguments */| { /* handle change */ })
    .multiple(true)
    .default_values(/* default_values: Vec<T> */)
    .values(/* values: Vec<T> */)
    .on_values_change(|/* callback arguments */| { /* handle change */ })
    .disabled(true)
    .read_only(true)
    .required(true)
    .highlight_item_on_hover(true)
    .modal(true)
    .label_resolver(|/* callback arguments */| { /* handle change */ })
    .value_serializer(|/* callback arguments */| { /* handle change */ })
    .item_to_string_value(|/* callback arguments */| { /* handle change */ })
    .value_comparator(|/* callback arguments */| { /* handle change */ })
    .multiple_value_formatter(|/* callback arguments */| { /* handle change */ })
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a SelectRoot with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<SelectChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<SelectChild<T>>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<SelectChild<T>>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<SelectChild<T>>>`

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

#### `.form(...)`

```rust
pub fn form(mut self, form: impl Into<SharedString>) -> Self
```

**Accepts**

- `form`: `impl Into<SharedString>`

Sets the form configuration for this part.

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
pub fn on_value_change(mut self, on_value_change: impl Fn(Option<&T>, &mut SelectValueChangeDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_value_change`: `impl Fn(Option<&T>, &mut SelectValueChangeDetails, &mut Window, &mut App)
            + 'static`

Registers a callback invoked when value change occurs.

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
pub fn on_open_change(mut self, on_open_change: impl Fn(bool, &mut SelectOpenChangeDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_open_change`: `impl Fn(bool, &mut SelectOpenChangeDetails, &mut Window, &mut App) + 'static`

Registers a callback invoked when open change occurs.

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
pub fn on_values_change(mut self, on_values_change: impl Fn(&[T], &mut SelectValueChangeDetails, &mut Window, &mut App) + 'static,) -> Self
```

**Accepts**

- `on_values_change`: `impl Fn(&[T], &mut SelectValueChangeDetails, &mut Window, &mut App) + 'static`

Registers a callback invoked when values change occurs.

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

#### `.highlight_item_on_hover(...)`

```rust
pub fn highlight_item_on_hover(mut self, highlight_item_on_hover: bool) -> Self
```

**Accepts**

- `highlight_item_on_hover`: `bool`

Controls whether highlight item on hover behavior is enabled.

#### `.modal(...)`

```rust
pub fn modal(mut self, modal: bool) -> Self
```

**Accepts**

- `modal`: `bool`

Controls whether the overlay behaves modally and blocks interaction outside it.

#### `.label_resolver(...)`

```rust
pub fn label_resolver(mut self, resolver: impl Fn(&T) -> SharedString + 'static) -> Self
```

**Accepts**

- `resolver`: `impl Fn(&T) -> SharedString + 'static`

Sets the label resolver configuration for this part.

#### `.value_serializer(...)`

```rust
pub fn value_serializer(mut self, serializer: impl Fn(&T) -> SharedString + 'static) -> Self
```

**Accepts**

- `serializer`: `impl Fn(&T) -> SharedString + 'static`

Sets the value serializer configuration for this part.

#### `.item_to_string_value(...)`

```rust
pub fn item_to_string_value(self, serializer: impl Fn(&T) -> SharedString + 'static) -> Self
```

**Accepts**

- `serializer`: `impl Fn(&T) -> SharedString + 'static`

Sets the item to string value configuration for this part.

#### `.value_comparator(...)`

```rust
pub fn value_comparator(mut self, comparator: impl Fn(&T, &T) -> bool + 'static) -> Self
```

**Accepts**

- `comparator`: `impl Fn(&T, &T) -> bool + 'static`

Sets the value comparator configuration for this part.

#### `.multiple_value_formatter(...)`

```rust
pub fn multiple_value_formatter(mut self, formatter: impl Fn(&[SharedString], &[T]) -> SharedString + 'static,) -> Self
```

**Accepts**

- `formatter`: `impl Fn(&[SharedString], &[T]) -> SharedString + 'static,`

Sets the multiple value formatter configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(SelectRootStyleState<T>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(SelectRootStyleState<T>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

SelectRoot also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `SelectTrigger`

Interactive control that opens, closes, or activates the component.

[Source](../../src/select/layers/select_trigger.rs)

```rust
use base_gpui::select::SelectTrigger;

SelectTrigger::new()
    .id("example-id")
    .aria_label("label")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a SelectTrigger with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<SelectTriggerChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<SelectTriggerChild<T>>`

Adds one typed child to this part.

#### `.children(...)`

```rust
pub fn children(mut self, children: impl IntoIterator<Item = impl Into<SelectTriggerChild<T>>>,) -> Self
```

**Accepts**

- `children`: `impl IntoIterator<Item = impl Into<SelectTriggerChild<T>>>`

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

#### `.aria_label(...)`

```rust
pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self
```

**Accepts**

- `label`: `impl Into<SharedString>`

Sets the accessible name announced for the trigger. Base UI relies on `aria-labelledby`, which has no gpui equivalent, so the literal string is the substitute. When the visible trigger content already matches the label, render that content with `Text::new_inaccessible(...)` to avoid double-announcing.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(SelectTriggerStyleState<T>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(SelectTriggerStyleState<T>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

SelectTrigger also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `SelectPortal`

Hosts overlay content outside the normal child layout.

[Source](../../src/select/layers/select_portal.rs)

```rust
use base_gpui::select::SelectPortal;

SelectPortal::new()
    .force_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a SelectPortal with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<SelectPortalChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<SelectPortalChild<T>>`

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
pub fn style_with_state(mut self, style: impl Fn(SelectPortalStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(SelectPortalStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

SelectPortal also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `SelectPositioner`

Measures the anchor and positions floating content.

[Source](../../src/select/layers/select_positioner.rs)

```rust
use base_gpui::select::SelectPositioner;

SelectPositioner::new()
    .side("example-id")
    .align(/* align: SelectAlign */)
    .side_offset("example-id")
    .align_offset(/* align_offset: Pixels */)
    .collision_padding(/* collision_padding: Pixels */)
    .align_item_with_trigger(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a SelectPositioner with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<SelectPositionerChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<SelectPositionerChild<T>>`

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
pub fn side(mut self, side: SelectSide) -> Self
```

**Accepts**

- `side`: `SelectSide`

Sets the preferred side of the anchor on which floating content appears.

#### `.align(...)`

```rust
pub fn align(mut self, align: SelectAlign) -> Self
```

**Accepts**

- `align`: `SelectAlign`

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

#### `.align_item_with_trigger(...)`

```rust
pub fn align_item_with_trigger(mut self, align_item_with_trigger: bool) -> Self
```

**Accepts**

- `align_item_with_trigger`: `bool`

Sets the align item with trigger configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(SelectPositionerStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(SelectPositionerStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

SelectPositioner also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `SelectPopup`

Contains the floating interactive content.

[Source](../../src/select/layers/select_popup.rs)

```rust
use base_gpui::select::SelectPopup;

SelectPopup::new()
    .side("example-id")
    .align(/* align: SelectAlign */)
    .force_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a SelectPopup with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<SelectPopupChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<SelectPopupChild<T>>`

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
pub fn side(mut self, side: SelectSide) -> Self
```

**Accepts**

- `side`: `SelectSide`

Sets the preferred side of the anchor on which floating content appears.

#### `.align(...)`

```rust
pub fn align(mut self, align: SelectAlign) -> Self
```

**Accepts**

- `align`: `SelectAlign`

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
pub fn style_with_state(mut self, style: impl Fn(SelectPopupStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(SelectPopupStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

SelectPopup also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `SelectArrow`

Optional arrow that visually points toward the overlay anchor.

[Source](../../src/select/layers/select_arrow.rs)

```rust
use base_gpui::select::SelectArrow;

SelectArrow::new()
    .side("example-id")
    .align(/* align: SelectAlign */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a SelectArrow with its default configuration.

#### `.side(...)`

```rust
pub fn side(mut self, side: SelectSide) -> Self
```

**Accepts**

- `side`: `SelectSide`

Sets the preferred side of the anchor on which floating content appears.

#### `.align(...)`

```rust
pub fn align(mut self, align: SelectAlign) -> Self
```

**Accepts**

- `align`: `SelectAlign`

Sets floating content alignment along the selected side.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(SelectArrowStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(SelectArrowStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

SelectArrow also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `SelectBackdrop`

Covers content behind a modal overlay and handles outside interaction.

[Source](../../src/select/layers/select_backdrop.rs)

```rust
use base_gpui::select::SelectBackdrop;

SelectBackdrop::new()
    .force_mounted(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a SelectBackdrop with its default configuration.

#### `.force_mounted(...)`

```rust
pub fn force_mounted(mut self, force_mounted: bool) -> Self
```

**Accepts**

- `force_mounted`: `bool`

Controls whether force mounted behavior is enabled.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(SelectBackdropStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(SelectBackdropStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

SelectBackdrop also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `SelectGroup`

Groups related child parts and coordinates their shared behavior.

[Source](../../src/select/layers/select_group.rs)

```rust
use base_gpui::select::SelectGroup;

SelectGroup::new()
    .id("example-id")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a SelectGroup with its default configuration.

#### `.id(...)`

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self
```

**Accepts**

- `id`: `impl Into<ElementId>`

Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<SelectGroupChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<SelectGroupChild<T>>`

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
pub fn style_with_state(mut self, style: impl Fn(SelectGroupStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(SelectGroupStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

SelectGroup also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `SelectGroupLabel`

Provides a visible label and associated accessibility semantics.

[Source](../../src/select/layers/select_group_label.rs)

```rust
use base_gpui::select::SelectGroupLabel;

SelectGroupLabel::new()
    .label("label")
    .text("label")
    .registration_label()
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a SelectGroupLabel with its default configuration.

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

#### `.registration_label(...)`

```rust
pub fn registration_label(&self) -> Option<SharedString>
```

Sets the registration label configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(SelectGroupLabelStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(SelectGroupLabelStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

SelectGroupLabel also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `SelectIcon`

Optional visual icon for the component.

[Source](../../src/select/layers/select_icon.rs)

```rust
use base_gpui::select::SelectIcon;

SelectIcon::new()
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a SelectIcon with its default configuration.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(SelectIconStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(SelectIconStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

SelectIcon also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `SelectItem`

Represents one interactive item in the component's collection.

[Source](../../src/select/layers/select_item.rs)

```rust
use base_gpui::select::SelectItem;

SelectItem::new()
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

Creates a SelectItem with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<SelectItemChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<SelectItemChild<T>>`

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

Assigns the item value that participates in selection.  Values are compared with `Eq`, so callers should prefer values that are unique within a `SelectRoot`. If duplicate values are registered, the selected state is value-based for every matching item, while value-label lookup and typeahead use the first matching item in render order.  An item without a value is rendered but is not registered as selectable; use `T = Option<U>` when a selectable null-like value is needed.

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

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(SelectItemStyleState<T>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(SelectItemStyleState<T>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

SelectItem also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `SelectItemIndicator`

Visual indicator for an item's selected or checked state.

[Source](../../src/select/layers/select_item_indicator.rs)

```rust
use base_gpui::select::SelectItemIndicator;

SelectItemIndicator::new()
    .keep_mounted(true)
    .with_item_state(/* state: SelectItemStyleState<T> */)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a SelectItemIndicator with its default configuration.

#### `.keep_mounted(...)`

```rust
pub fn keep_mounted(mut self, keep_mounted: bool) -> Self
```

**Accepts**

- `keep_mounted`: `bool`

Keeps the part mounted when inactive or closed so child state can be preserved.

#### `.with_item_state(...)`

```rust
pub fn with_item_state(mut self, state: SelectItemStyleState<T>) -> Self
```

**Accepts**

- `state`: `SelectItemStyleState<T>`

Sets the with item state configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(SelectItemIndicatorStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(SelectItemIndicatorStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

SelectItemIndicator also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `SelectItemText`

Public renderable part of the Select Item Text component.

[Source](../../src/select/layers/select_item_text.rs)

```rust
use base_gpui::select::SelectItemText;

SelectItemText::new()
    .label("label")
    .text("label")
    .with_item_state(/* state: SelectItemStyleState<T> */)
    .registration_label()
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a SelectItemText with its default configuration.

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

#### `.with_item_state(...)`

```rust
pub fn with_item_state(mut self, state: SelectItemStyleState<T>) -> Self
```

**Accepts**

- `state`: `SelectItemStyleState<T>`

Sets the with item state configuration for this part.

#### `.registration_label(...)`

```rust
pub fn registration_label(&self) -> Option<SharedString>
```

Sets the registration label configuration for this part.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(SelectItemTextStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(SelectItemTextStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

SelectItemText also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `SelectLabel`

Provides a visible label and associated accessibility semantics.

[Source](../../src/select/layers/select_label.rs)

```rust
use base_gpui::select::SelectLabel;

SelectLabel::new()
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a SelectLabel with its default configuration.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn((), Div) -> Div + 'static) -> Self
```

**Accepts**

- `style`: `impl Fn((), Div) -> Div + 'static`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

SelectLabel also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `SelectList`

Contains and coordinates the component's collection items.

[Source](../../src/select/layers/select_list.rs)

```rust
use base_gpui::select::SelectList;

SelectList::new()
    .loop_focus(true)
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a SelectList with its default configuration.

#### `.child(...)`

```rust
pub fn child(mut self, child: impl Into<SelectListChild<T>>) -> Self
```

**Accepts**

- `child`: `impl Into<SelectListChild<T>>`

Adds one typed child to this part.

#### `.child_any(...)`

```rust
pub fn child_any(mut self, child: impl IntoElement) -> Self
```

**Accepts**

- `child`: `impl IntoElement`

Adds one arbitrary renderable child to this part.

#### `.loop_focus(...)`

```rust
pub fn loop_focus(mut self, loop_focus: bool) -> Self
```

**Accepts**

- `loop_focus`: `bool`

Controls whether keyboard focus wraps from the last enabled item to the first and vice versa.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(SelectListStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(SelectListStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

SelectList also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `SelectScrollDownArrow`

Optional arrow that visually points toward the overlay anchor.

[Source](../../src/select/layers/select_scroll_down_arrow.rs)

```rust
use base_gpui::select::SelectScrollDownArrow;

SelectScrollDownArrow::new()
    .keep_mounted(true)
    .side("example-id")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a SelectScrollDownArrow with its default configuration.

#### `.keep_mounted(...)`

```rust
pub fn keep_mounted(mut self, keep_mounted: bool) -> Self
```

**Accepts**

- `keep_mounted`: `bool`

Keeps the part mounted when inactive or closed so child state can be preserved.

#### `.side(...)`

```rust
pub fn side(mut self, side: SelectSide) -> Self
```

**Accepts**

- `side`: `SelectSide`

Sets the preferred side of the anchor on which floating content appears.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(SelectScrollArrowStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(SelectScrollArrowStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

SelectScrollDownArrow also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `SelectScrollUpArrow`

Optional arrow that visually points toward the overlay anchor.

[Source](../../src/select/layers/select_scroll_up_arrow.rs)

```rust
use base_gpui::select::SelectScrollUpArrow;

SelectScrollUpArrow::new()
    .keep_mounted(true)
    .side("example-id")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a SelectScrollUpArrow with its default configuration.

#### `.keep_mounted(...)`

```rust
pub fn keep_mounted(mut self, keep_mounted: bool) -> Self
```

**Accepts**

- `keep_mounted`: `bool`

Keeps the part mounted when inactive or closed so child state can be preserved.

#### `.side(...)`

```rust
pub fn side(mut self, side: SelectSide) -> Self
```

**Accepts**

- `side`: `SelectSide`

Sets the preferred side of the anchor on which floating content appears.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(SelectScrollArrowStyleState, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(SelectScrollArrowStyleState, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

SelectScrollUpArrow also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `SelectSeparator`

Visual separator between neighboring items or groups.

[Source](../../src/select/layers/select_separator.rs)

```rust
use base_gpui::select::SelectSeparator;

SelectSeparator::new()
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

Creates a SelectSeparator with its default configuration.

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

SelectSeparator also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## `SelectValue`

Displays or edits the component's current value.

[Source](../../src/select/layers/select_value.rs)

```rust
use base_gpui::select::SelectValue;

SelectValue::new()
    .placeholder("placeholder")
    .style_with_state(|/* callback arguments */| { /* handle change */ });
```

### Builders

#### `.new(...)`

```rust
pub fn new() -> Self
```

Creates a SelectValue with its default configuration.

#### `.placeholder(...)`

```rust
pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self
```

**Accepts**

- `placeholder`: `impl Into<SharedString>`

Sets the content shown when the component has no current value.

#### `.style_with_state(...)`

```rust
pub fn style_with_state(mut self, style: impl Fn(SelectValueStyleState<T>, Div) -> Div + 'static,) -> Self
```

**Accepts**

- `style`: `impl Fn(SelectValueStyleState<T>, Div) -> Div + 'static,`

Styles the part from its current behavioral state while preserving separation between behavior and visual design.

SelectValue also supports the GPUI traits implemented in its source, such as standard styling or child composition.

## Accessibility

Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.

## Stability

Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.
