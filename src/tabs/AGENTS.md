# Tabs Implementation Notes

This file captures Tabs-specific implementation details for `crates/base_gpui/src/tabs`.
Generic component architecture belongs in `docs/base-gpui-component-architecture.md`; keep this file focused on Tabs behavior and local contracts.

## Component family

The Tabs port exposes these explicit component names:

- `TabsRoot<T>`
- `TabsList<T>`
- `TabsTab<T>`
- `TabsPanel<T>`
- `TabsIndicator<T>`

Tab values are generic:

```rust
T: Clone + Eq + 'static
```

Do not hard-code string or index values into the component model.

## Public props and API surface

`TabsRoot<T>` is the only part with root-level Tabs configuration:

- `.value(...)`
- `.default_value(...)`
- `.on_value_change(...)`
- `.orientation(...)`

Do not add orientation props or setters to `TabsList`, `TabsTab`, `TabsPanel`, or `TabsIndicator`. Those parts receive orientation through injected `TabsContext<T>` / `TabsProps<T>` when deriving style state or handling keyboard behavior.

Part-local props stay on their parts:

- `TabsList<T>`: `activate_on_focus`, `loop_focus`
- `TabsTab<T>`: `value`, `disabled`
- `TabsPanel<T>`: `value`, `keep_mounted`

## Controlled value semantics

`TabsRoot<T>` uses `Option<Option<T>>` for the controlled value prop:

- `None` = uncontrolled prop absent
- `Some(None)` = controlled empty selection
- `Some(Some(value))` = controlled selected value

This distinction is important. Do not collapse it.

Behavior:

- Controlled mode: caller passes `.value(...)`; interactions call `on_value_change(...)` but do not mutate the source of truth.
- Uncontrolled mode: caller omits `.value(...)`; interactions call `on_value_change(...)` and mutate runtime selection.
- `default_value(...)` initializes uncontrolled selection.
- If no uncontrolled default is provided, fallback selects the first enabled tab.
- Controlled mode preserves externally supplied values even if disabled or missing.
- Uncontrolled fallback uses walked tab metadata and falls back to first enabled tab or `None`.

## File layout

Tabs uses the runtime/context split from the component architecture:

```text
crates/base_gpui/src/tabs/
  actions.rs
  child.rs            # TabsChild and TabsListChild typed routing
  child_wiring.rs     # private child traversal, indexing, and context attachment
  context.rs          # TabsContext: entity plumbing + controlled/uncontrolled rule
  props.rs            # TabsOrientation, TabsProps, callback type
  style_state.rs      # style-state structs for all drawing parts
  runtime.rs          # TabsRuntime: selected value, tab metadata, transitions, queries
  layers/
    tabs_indicator.rs
    tabs_list.rs
    tabs_panel.rs
    tabs_root.rs
    tabs_tab.rs
```

There is no `TabsState<T>`, no `GenericContext`, and no shared generic child abstraction in Tabs. The selected value lives in `TabsRuntime<T>`.

## Context, props, runtime

`TabsContext<T>` owns exactly the injection/plumbing facts:

```rust
runtime: Entity<TabsRuntime<T>>
props: Rc<TabsProps<T>>
controlled: Rc<Option<Option<T>>>
```

It exposes three method shapes:

- `read(...)`
- `update(...)`
- `select(...)`

Do not grow Tabs vocabulary on `TabsContext<T>` (`register_tab`, `highlight_next_tab`, etc.). Tabs behavior belongs on `TabsRuntime<T>`.

`TabsProps<T>` owns stable root props:

- orientation,
- `on_value_change`.

`TabsRuntime<T>` owns Tabs-specific runtime facts:

- uncontrolled selected value,
- walked tab metadata,
- highlighted tab index,
- selection reconciliation and uncontrolled fallback,
- activation direction bookkeeping,
- measured tab bounds for indicator state,
- tab focus handles for roving focus,
- initial-focus seeding.

Tabs intentionally does **not** keep panel metadata. Panel visibility derives from the panel's local `value` and the runtime selected value.

## Typed child routing and wiring

`TabsRoot<T>` keeps typed `TabsChild<T>` children before GPUI erases to `AnyElement`.

`TabsChild<T>` currently routes:

- `TabsList<T>`
- `TabsPanel<T>`
- `TabsIndicator<T>`

`TabsList<T>` keeps typed `TabsListChild<T>` children so the list can accept both tabs and indicators while preserving tab-only metadata collection and measurement.

`TabsListChild<T>` currently routes:

- `TabsTab<T>`
- `TabsIndicator<T>`

Typed child-routing enums belong in `tabs/child.rs`, not `tabs/layers/`.

`tabs/child_wiring.rs` owns the private Tabs child ontology. It wires the typed child hierarchy before GPUI erases children to `AnyElement`:

1. `TabsRoot<T>` creates `TabsContext<T>`.
2. `child_wiring::wire_children(...)` walks typed children, assigns tab indices, collects tab metadata, collects stable keyed tab `FocusHandle`s, attaches `TabsContext<T>`, and returns wired children for rendering.
3. `TabsRoot<T>` updates `TabsRuntime<T>` with `sync_children(...)` and `reconcile(...)` in one root render update.
4. Layers render from the wired children.

`TabsPanel<T>` does not register runtime metadata. Panel visibility derives from its local `value` and the runtime selected value.

Do not move child indexing or runtime metadata collection back into leaf render paths. Do not generalize child wiring into a shared child abstraction; metadata shapes differ by component. Keep child-wiring traits in the private `child_wiring` module so layer internals do not become public helper methods.

## Selection and fallback

Selection behavior:

- Enabled inactive tab click selects that tab.
- Clicking active tab is a no-op.
- Clicking disabled tab is a no-op.
- User-initiated selection computes activation direction from walked tab order.
- Automatic fallback has activation direction `None`.
- Disabled or removed selected tabs trigger fallback only in uncontrolled mode.

`TabsRuntime<T>` owns this behavior through methods such as:

- `sync_children(...)`
- `reconcile(...)`
- `select(...)`
- `move_highlight(...)`

`TabsContext<T>::select(...)` is responsible for controlled/uncontrolled resolution and firing `on_value_change` after the runtime update returns its outcome.

## Keyboard and focus

Tabs uses GPUI actions/key dispatch, not raw key-down handlers.

`tabs/actions.rs` defines:

- `TabsSelectLeft`
- `TabsSelectRight`
- `TabsSelectUp`
- `TabsSelectDown`
- `TabsSelectFirst`
- `TabsSelectLast`
- `TabsActivateHighlighted`

`TABS_LIST_KEY_CONTEXT` scopes key bindings to the Tabs list.

Keyboard behavior:

- Horizontal tabs use left/right.
- Vertical tabs use up/down.
- Home moves highlight to first enabled tab.
- End moves highlight to last enabled tab.
- `loop_focus = true` wraps at edges.
- `loop_focus = false` clamps at edges.
- `activate_on_focus = true` selects during navigation.
- `activate_on_focus = false` only moves highlight; Enter/Space activate highlighted tab.
- Disabled tabs are skipped and never activated.

Known GPUI caveat:

- Initial highlighted state is not necessarily actual GPUI focus.
- Keyboard actions require focus to be inside the relevant key context.
- Be careful when moving actual GPUI focus during action dispatch; it can affect subsequent action routing.

## Style-state styling

Tabs exposes state-aware styling through `style_with_state(...)` on:

- `TabsRoot<T>`
- `TabsList<T>`
- `TabsTab<T>`
- `TabsPanel<T>`
- `TabsIndicator<T>`

Style-state structs:

- `TabsRootStyleState`: orientation and activation direction.
- `TabsListStyleState`: orientation and activation direction.
- `TabsTabStyleState`: active, disabled, highlighted, and orientation.
- `TabsPanelStyleState`: hidden, orientation, and activation direction.
- `TabsIndicatorStyleState`: selected state, active tab position/size, orientation, and activation direction.

Do not add DOM-style data attributes or CSS variables unless they become useful GPUI API.

## Indicator measurement

`TabsList<T>` uses `Div::on_children_prepainted(...)` to capture child bounds after layout.

Important details:

- `TabsList<T>` filters bounds through `TabsListChild<T>` so only tab child bounds enter runtime.
- Bounds are registered through `TabsRuntime<T>` via `TabsContext<T>::update(...)`.
- `TabsRuntime<T>` derives active tab position and size.
- `TabsIndicator<T>` exposes those values through `TabsIndicatorStyleState`.

Do not port Base UI indicator CSS variable names directly.

## Panels

Panel behavior:

- Panel is visible when its value equals selected value.
- `keep_mounted = false`: inactive panels are omitted.
- `keep_mounted = true`: inactive panels remain mounted but hidden.
- Panels receive hidden/orientation/activation direction via `TabsPanelStyleState`.
- Panels do not register metadata in runtime.

## Base UI differences / intentionally dropped web details

Do not port:

- ARIA roles/attributes,
- DOM id linking (`aria-controls`, `aria-labelledby`),
- React hooks/context details,
- SSR/hydration/prehydration scripts,
- CSS variable API,
- `ResizeObserver` directly,
- DOM transition attributes,
- arbitrary JS value semantics.

Translate behavior into GPUI-native state, runtime, measurement, focus handles, and actions.

## Tests / verification

Rendered behavior tests live under `crates/base_gpui/src/tabs/tests/` and cover:

- controlled and uncontrolled selection,
- disabled and removed-tab fallback,
- click activation,
- keyboard navigation,
- `activate_on_focus`,
- `loop_focus`,
- panel visibility and `keep_mounted`,
- indicator movement.

`TabsRuntime<T>` also has window-free unit tests for runtime-only behavior.
