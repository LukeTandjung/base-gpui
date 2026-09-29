---
name: create-gpui-port-issue
description: Create a behavior-focused implementation issue for porting a web component or component family into base-gpui. Use when asked to study a React, DOM, Base UI, or other web implementation and plan a GPUI-native port. Port behavior and contracts rather than translating source code literally.
user-invocable: true
---

# Create a Web Component to GPUI Port Issue

Use this skill to research a web component and create one implementation-ready Markdown issue for `base-gpui`.

The deliverable belongs under:

```text
issues/port-<source>-<component>.md
```

Examples:

```text
issues/port-baseui-tabs.md
issues/port-pierre-file-tree.md
issues/port-radix-roving-focus.md
```

Use `issues/port-pierre-file-tree.md` as a reference for a large staged port. Adapt its depth to the real component rather than copying sections mechanically.

## Core principle

Port behavior and public contracts, not source code.

The source implementation is a behavioral reference. The GPUI result must use GPUI-native state, actions, focus, layout, rendering, and typed callback APIs.

Typical translations:

- React context and hooks become GPUI entities, component contexts, props, and runtime state.
- DOM registration and refs become stable typed IDs, runtime metadata, child routing, or host-owned model references.
- DOM measurement and `ResizeObserver` become GPUI layout, prepaint, bounds, and scroll-handle mechanisms.
- CSS variables and data attributes become typed `*StyleState` values passed to `style_with_state(...)`.
- DOM focus APIs become `FocusHandle`, key contexts, actions, and tab stops.
- Browser drag payloads become concrete typed GPUI payloads.
- Async browser or filesystem effects remain host-owned and communicate through typed requests, results, or controlled state.

## Required inputs

Before writing the issue, determine:

1. The component or component family to port.
2. The source project and source slug used in the issue filename.
3. The source checkout, repository URL, or exact files to study.
4. The source revision or commit used for research.
5. Whether an existing `base-gpui` implementation is a partial port, should be replaced, or should be ignored.
6. Any additional GPUI references the user named, such as Zed or `gpui-component`.

Prefer a local source checkout because source, tests, examples, and documentation together define behavior. If only a web URL is provided, inspect the repository and record the exact revision used.

For Base UI, default to `/home/luke/Projects/base-ui` only when it exists.

## Repository target

This skill runs for the `base-gpui` repository.

Default target paths:

```text
src/<component>/
docs/components/<component>.md
site/src/
issues/port-<source>-<component>.md
```

Read `Cargo.toml` before selecting Zed examples so references use the GPUI revision pinned by this repository.

## Research workflow

### 1. Audit the source family

Follow exports and index files to discover every public part.

Read:

- public component files,
- model, controller, reducer, or store files,
- public types and callbacks,
- unit tests,
- browser or integration tests,
- examples and documentation,
- styling and state exposure,
- accessibility behavior,
- performance or virtualization code when relevant.

Record exact source paths and the source revision.

### 2. Inspect the current target

Search `src/` for an existing component, shared primitive, action, context, or utility that affects the port.

Read:

```text
docs/component-architecture.md
CODE_STYLE.md
```

Also read component-local `AGENTS.md` files when they apply.

### 3. Inspect GPUI-native references

When useful, study:

- the Zed implementation at this repository's pinned GPUI revision,
- `gpui-component` implementations and documentation,
- existing `base-gpui` components with similar state or interaction behavior,
- GPUI examples for drag/drop, focus, actions, uniform lists, layout, or testing.

Use these as architectural references. Do not copy application-specific service coupling into a reusable component.

Secondary GPUI references may guide implementation mechanics but must not add behavior or feature scope unless the user explicitly requests it.

### 4. Separate behavior from platform mechanics

Classify each source feature as one of:

- direct GPUI behavior to preserve,
- behavior that needs a GPUI-native translation,
- web-only implementation detail to drop,
- application-owned behavior that needs a typed integration contract,
- follow-up behavior blocked by the pinned GPUI revision.

## Standard translation decisions

Use these defaults unless the user overrides them:

- Do not port React `render` props literally. Prefer typed GPUI components, builders, or narrowly scoped renderer callbacks.
- Do not port `className`.
- Do not port web `style` prop bags.
- Do not port CSS-variable APIs. Expose typed style state and normal GPUI styling methods.
- Do not port DOM data attributes as attributes. Map state into typed Rust values.
- Do not port SSR, hydration, prehydration scripts, or CSP nonce APIs.
- Do not port arbitrary JavaScript value semantics. Use explicit Rust types and bounds.
- Do not port browser event objects. Use typed source, reason, request, and details structures.
- Do not make a reusable component own application filesystem, network, project, Git, editor, or workspace services.
- Use AccessKit only through capabilities available in the pinned GPUI revision.

## `base-gpui` architecture

Use the current flat component layout:

```text
src/<component>/
  mod.rs
  actions.rs
  runtime.rs
  context.rs
  props.rs
  style_state.rs
  child.rs
  child_wiring.rs
  layers/
  tests/
```

Only include files the component needs.

Enforce these rules in the issue:

- `mod.rs` files are barrel exports only.
- Runtime behavior is implemented as inherent methods on `<Component>Runtime`.
- Controlled and uncontrolled mediation belongs in `<Component>Context`.
- Public configuration and callbacks belong in `props.rs`.
- Typed state exposed to styling uses `*StyleState` names in `style_state.rs`.
- Renderable GPUI parts belong under `layers/`.
- Typed child routing belongs in `child.rs`, not `layers/`.
- Keep component-specific helpers inside the component module.
- Add shared utilities only after repeated cross-component use is demonstrated.
- Preserve typed children and state until element erasure is necessary.

## Issue filename and title

Use a short lowercase source slug and kebab-case component name:

```text
issues/port-<source>-<component>.md
```

Title:

```md
# Port <Source> <Component> to GPUI
```

If several sources jointly define the desired behavior, use the primary source in the filename and list the others as architectural references.

## Issue structure

Create sections in this order.

### Problem

Explain:

- the behavior provided by the source,
- what `base-gpui` currently lacks,
- why the behavior belongs in a reusable component,
- that the goal is GPUI-native behavioral parity,
- important type, ownership, scaling, or platform constraints.

### Repository and target

List:

- target module and expected files,
- current GPUI revision when reference compatibility matters,
- consumer integration that remains outside the shared component.

### Reference sources

List exact source paths and pinned repository links.

Separate:

- primary behavioral source,
- GPUI-native architectural references,
- relevant tests and examples.

### Scope

List the intended public Rust types, renderable parts, callbacks, events, and runtime responsibilities.

For complicated controlled or async components, include a small signature-level sketch that settles ownership and event flow. Do not fill the issue with implementation code.

### Web-only translation and out of scope

List only details that are genuinely being dropped or moved to the host application.

Be precise. Do not call something out of scope when GPUI has a meaningful equivalent that should be implemented.

### Delivery phases

Split large ports into mergeable phases.

Typical shape:

1. Core data model, state, rendering, focus, and basic interaction.
2. Source parity features such as search, editing, drag/drop, composition, and advanced styling.
3. Optional desktop extensions, performance work, or accessibility follow-ups.

Label acceptance sections and tests by phase. Later-phase criteria must not block an earlier phase.

### Acceptance Criteria

Use unchecked checkbox subsections.

Include only sections that apply, such as:

- module and API surface,
- ownership and runtime architecture,
- identity and hierarchy,
- controlled/uncontrolled state,
- selection and activation,
- keyboard and focus,
- pointer behavior,
- layout and measurement,
- virtualization and scrolling,
- popup or context-menu behavior,
- search and filtering,
- inline editing,
- drag/drop,
- dynamic mutations,
- styling and state exposure,
- loading, empty, and error states,
- accessibility follow-up,
- tests and verification.

## Behavioral checklist rules

### Controlled and uncontrolled state

Define:

- which data is host-controlled,
- which interaction state is runtime-owned by default,
- controlled-value precedence,
- default-value semantics,
- callback timing,
- cancelable transitions,
- invalid external value repair,
- ordering for concurrent or asynchronous updates.

If host operations are asynchronous, define request IDs, acceptance/rejection, stale-result behavior, and how accepted results relate to controlled state revisions.

### Identity

Do not use visible list indexes as persistent identity.

Define:

- stable IDs,
- user-facing values or paths,
- behavior across sorting, filtering, rename, move, and replacement,
- duplicate handling,
- fallback when an active item disappears.

### Keyboard and focus

Prefer GPUI actions and key contexts.

Specify:

- directional navigation,
- Home/End/Page behavior,
- wrapping or clamping,
- parent/child semantics for hierarchical widgets,
- selection extension,
- activation behavior,
- focus repair after filtering, collapse, removal, or unmount,
- interaction between editing fields and component actions.

Keep focused, selected, active, highlighted, context-targeted, and drag-targeted states distinct when relevant.

### Measurement and virtualization

State:

- which bounds or row sizes are required,
- whether rows must be uniform height,
- the GPUI list or scroll primitive,
- reveal and alignment behavior,
- how projection changes affect scroll position,
- which work must stay outside render,
- thread-safety constraints for background derivation.

### Styling

Require component-specific `*StyleState` values for state-aware styling.

Do not expose DOM attributes, CSS selectors, CSS variables, or raw web-style objects.

### Drag/drop

GPUI dispatches drops by concrete payload type. Name each supported payload and register separate handlers when native payloads such as `gpui::ExternalPaths` are required.

Define:

- drag source identity,
- destination types,
- invalid target rules,
- hover expansion and auto-scroll,
- copy versus move semantics,
- host request and result flow.

## Tests and verification

Every issue must include tests tailored to the behavior.

Prefer state-transition and interaction tests over wording or markup tests.

Typical verification commands:

```sh
cargo fmt --check
cargo check
cargo test <component-filter>
cargo test
cargo clippy --all-targets
ast-grep scan src
```

When the separate site workspace changes:

```sh
cargo fmt --check --manifest-path site/Cargo.toml
cargo check --manifest-path site/Cargo.toml
```

If a command cannot run in the current environment, record the concrete toolchain failure. Do not silently claim it passed.

## Writing style

- Be concrete and implementation-ready.
- Use exact file paths and pinned source links.
- Use checked boxes only for behavior already implemented and verified.
- Avoid generic filler and speculative abstractions.
- Explain deliberate deviations from the source.
- Keep application-specific behavior behind typed host integration contracts.
- Keep the issue useful as a living implementation checklist.

## Deliverable

Write one issue under:

```text
issues/port-<source>-<component>.md
```

Then summarize:

- issue path,
- source revisions and files referenced,
- GPUI references used,
- major behavior areas covered,
- phase boundaries,
- unresolved decisions requiring human confirmation,
- validation performed.

Respect the user's Git instructions. Do not commit when the user asks you not to commit.
