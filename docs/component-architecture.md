# `base_gpui` Component Architecture

This document describes the architecture for compound GPUI components in `crates/base_gpui`.

The public API surface of each component mirrors Base UI: compound part composition
(`Root` / `List` / `Tab` / ...), `value` / `default_value` / `on_value_change`
controlled-uncontrolled props, and state-aware styling via `style_with_state`.
The internals do **not** mirror Base UI's React internals — React idioms such as
effect-based prop diffing are compensations for React's render model and must not be
ported into GPUI's immediate-mode render.

## Design rule

Modules are carved by **knowledge**, not by widget. All knowledge in a compound
component (what children exist, what is selected, what is highlighted, how
transitions are derived) is one entangled body of knowledge, so it lives in **one
deep module per component**. The renderable parts are peers — views of the same
runtime — arranged hub-and-spoke around it, not in layers.

Per component there are exactly three kinds of module:

| Module | Depth | Responsibility |
|---|---|---|
| `<Component>Runtime<T>` | deep | all state, all business logic |
| `<Component>Context<T>` | thin | injection vehicle: entity plumbing + controlled/uncontrolled rule |
| Parts (`Root`, `List`, ...) | thin | GPUI adapters: input events → commands, queries → styles |

```text
parts (peers, thin)          injection (thin)         business logic (deep)
ComponentRoot ─────┐
ComponentList ─────┤
ComponentPart ─────┼──→  ComponentContext<T>  ──→  ComponentRuntime<T>
ComponentPanel ────┘     Entity<Runtime>             all state, all rules
                         + Rc<Props>
                         + controlled marker
```

## Directory shape

```text
crates/base_gpui/src/<component>/
  mod.rs
  actions.rs        # optional GPUI key dispatch actions/bindings
  runtime.rs        # ComponentRuntime + metadata structs + command enums/outcomes
  context.rs        # ComponentContext
  props.rs          # injected props/callbacks/config
  style_state.rs    # style-state structs (one per part that draws)
  child.rs          # typed child enums
  child_wiring.rs   # optional private child traversal/indexing/context attachment
  layers/           # renderable GPUI parts only
  tests/
```

Do not nest `child/context/{props,runtime,state}/` taxonomies. One file per concept,
directly under the component folder.

Do not introduce shared generic primitives for tiny plumbing. If a helper is trivially small, inline it per-component; a generic abstraction that only saves a setter usually creates more ontology than it removes.

## The runtime (deep module)

`ComponentRuntime<T>` is one struct that owns **all** component state: child
metadata, uncontrolled selection, highlight, derived transition state (e.g.
activation direction), measured bounds, focus handles. It uses plain `&mut self` /
`&self` methods, no GPUI entity types, and is unit-testable without a window.

Its interface has exactly two vocabularies:

**Commands** — one method per thing-that-can-happen, named in domain language:

```rust
fn sync_children(&mut self, metadata: Vec<ChildMeta<T>>, focus_handles: Vec<(usize, FocusHandle)>);
fn reconcile(&mut self, observed: Option<T>, allow_fallback: bool);
fn select(&mut self, value: Option<T>) -> SelectOutcome<T>;
fn move_highlight(&mut self, direction: Move, loop_focus: bool);
fn set_bounds(&mut self, bounds: Vec<(usize, Bounds<Pixels>)>) -> bool;
```

**Queries** — one method per part-that-draws, returning style-state structs:

```rust
fn root_state(&self) -> ComponentRootStyleState;
fn part_state(&self, value: Option<&T>, /* part-local facts */) -> ComponentPartStyleState;
```

Rules:

- No getter/setter pairs. A getter/setter pair is state escaping the module.
- No query answers "what is the highlighted index?" — only "am I highlighted?".
  Parts ask part-shaped questions and emit event-shaped commands.
- Every transition (direction, highlight sync, fallback) is computed inside the
  runtime, once. Do not keep shadow copies of values to detect changes by diffing;
  `reconcile` is the single transition-resolution point.
- `select` returns an outcome describing what changed so the caller can fire
  callbacks; the runtime itself never calls user callbacks.

## The context (injection vehicle)

```rust
pub struct ComponentContext<T: Clone + Eq + 'static> {
    runtime: Entity<ComponentRuntime<T>>,  // keyed GPUI state
    props: Rc<ComponentProps<T>>,
    controlled: Rc<Option<Option<T>>>,
}
```

Methods, exactly three shapes:

```rust
fn read<O>(&self, cx: &App, f: impl FnOnce(&ComponentRuntime<T>, &ComponentProps<T>) -> O) -> O;
fn update<O>(&self, cx: &mut App, f: impl FnOnce(&mut ComponentRuntime<T>) -> O) -> O; // notifies
fn select(&self, value: Option<T>, window: &mut Window, cx: &mut App); // or toggle for boolean components
```

The value-changing method (`select`, `toggle`, etc.) is the one non-trivial method:
it resolves controlled vs uncontrolled, calls the runtime command, and fires the
props callback based on the outcome. The controlled/uncontrolled rule lives here
(and in what value is passed to `reconcile`) — nowhere else.

The context must never grow component vocabulary (`register_tab`,
`highlight_next_tab`, ...). Those names belong on the runtime, where they take
`&mut self` and related state sits behind one borrow.

## Parts (renderable layers)

Files under `layers/` are GPUI renderable pieces. A part's `render` does only two
things:

1. translate GPUI input events into context commands
   (`context.update(cx, |m| m.move_highlight(...))`),
2. call one query and feed the result to `style_with_state`.

Parts never see runtime internals — style-state structs and commands only.

The root is the single mutation site outside event handlers:

```rust
fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
    let context = ComponentContext::new(...);

    let wired_children = wire_children(self.children, context.clone(), window, cx);
    let initial_focus = context.update(cx, |runtime| {
        runtime.sync_children(wired_children.metadata, wired_children.focus_handles);
        runtime.reconcile(
            /* controlled value or uncontrolled current */,
            /* allow uncontrolled fallback */,
        );
        runtime.take_initial_focus_handle()
    });
    if let Some(focus_handle) = initial_focus {
        focus_handle.focus(window, cx);
    }

    // pure rendering from here: style via root_state and render wired children
}
```

`wire_children` (in `child_wiring.rs` when a component needs non-trivial child wiring) is the only function that knows which children count as which part and assigns indices. It returns both the metadata for `sync_children` and the wired children for rendering. No index counter may be threaded through child enums or recomputed in a part. Keep child-wiring traits in a private module; do not expose helper methods on public layer types just to satisfy traversal or context attachment.

## Typed child enums

Compound roots keep typed children before GPUI erases elements to `AnyElement`:

```rust
pub enum ComponentChild<T: Clone + Eq + 'static> {
    PartA(ComponentPartA<T>),
    PartB(ComponentPartB<T>),
}
```

Their only public job is typed composition. Private component-local child wiring may attach context, walk descendants, assign indices, and collect metadata before erasing to `AnyElement`. They must not expose registration traversal or index bookkeeping as public helper methods. Nested compound layers may define their own constrained child enums in `child.rs`.

## Props

Props hold stable public configuration and callbacks: orientation, behavior flags,
controlled callback handlers. Props never own runtime state or metadata — those
belong to the runtime.

## Style-state structs

Style-state structs are component-specific public API, modeling the same
information Base UI exposes through state-aware `className` / `style` / `render`
callbacks, adapted to GPUI. They are the return types of runtime queries and the
input to `style_with_state`:

```rust
ComponentPart::new()
    .style_with_state(|state, part| if state.active { part.bg(...) } else { part })
```

Do not port DOM data attributes or CSS variable APIs.

## Measurement and layout-derived state

Translate Base UI DOM measurement into GPUI-native mechanisms
(`Div::on_children_prepainted(...)`). Measured facts go into the runtime via a
command (`set_bounds`) that returns whether anything changed, and come out through
style-state queries.

## Keyboard dispatch

Use GPUI key dispatch instead of raw `on_key_down`:

1. `actions.rs` defines actions,
2. component `init(cx)` binds keys (registered from `base_gpui::init(cx)`),
3. the relevant layer sets `key_context(...)`,
4. `on_action(...)` handlers translate actions into runtime commands via the context.

```rust
div()
    .key_context(COMPONENT_KEY_CONTEXT)
    .on_action(move |_: &ComponentMoveNext, window, cx| {
        context.update(cx, |runtime| runtime.move_highlight(Move::Next, loop_focus));
    })
```

## Invariants

1. State mutates in exactly two places: the top of root render
   (`sync_children` + `reconcile`) and event handlers (via context commands).
   Nowhere else.
2. Every transition is computed inside the runtime, once. No sync-by-diffing, no
   shadow previous-value fields outside the runtime.
3. Knowledge of child indexing lives only in component-local child wiring.
4. Parts never see runtime internals — style-state structs and commands only.
5. The context never grows component vocabulary.

## Implementation checklist for a new component

1. `runtime.rs` — write the runtime **comments-first**: sketch the signature block
   (commands + queries) with doc comments before any bodies. If a method's comment
   is hard to write in pure component-domain language, the design is wrong — that
   knowledge belongs inside the runtime, not on its interface.
2. `props.rs` — injected props/callbacks/config.
3. `style_state.rs` — one style-state struct per part that draws.
4. `context.rs` — `read` / `update` / `select`, nothing else.
5. `child.rs` — typed child enum(s); add private `child_wiring.rs` when traversal/indexing/context attachment needs an internal layer-wiring trait.
6. `layers/` — root creates the context, wires children once, calls
   `sync_children` + `reconcile`; other parts are event/query adapters.
7. `actions.rs` if the component has keyboard behavior.
8. Unit-test the runtime directly (no window needed) plus rendered behavior tests
   under `tests/`.
9. Re-export from `mod.rs`.

## Rules of thumb

- Decompose by knowledge, not by widget. Entangled knowledge = one module.
- The deep module is the runtime; everything else stays thin.
- Hide decisions, not just mechanics. Hiding entity plumbing while index
  assignment or transition detection escapes into parts is the cheap kind of
  hiding and buys nothing.
- Same tier of abstraction = same module's clients, not layers to invent.
- A small runtime (e.g. checkbox) is expected and fine — depth is about the
  interface-to-knowledge ratio, not line count.
- Avoid shared generic primitives unless they describe a deep, repeated concept.
- Keep component-specific code under the component folder. Truly shared, repeated
  cross-component helpers may live under `utils/` as flat named files.
- `mod.rs` files are barrel exports only: module declarations, re-exports, and
  test module declarations. No structs, enums, traits, functions, type aliases,
  constants/statics, impl blocks, or macros in `mod.rs`.
- Do not add `runtime_control.rs` or similar trait-boundary files for normal
  component behavior. Put behavior on inherent runtime methods unless there is a
  concrete polymorphism boundary.
