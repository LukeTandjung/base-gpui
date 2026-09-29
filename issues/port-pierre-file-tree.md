# Port Pierre File Tree to GPUI

## Problem

Pierre's `@pierre/trees` implements a path-first file tree with a deliberately optimized hierarchy store, range-based visible projection, fixed-row virtualization, sticky ancestors, search, selection, rename, internal drag and drop, incremental mutations, and composition surfaces.

The primary goal is to reimplement Pierre's behavior and performance techniques in GPUI. Zed and `gpui-component` are references for how to express specific mechanics with GPUI APIs, such as `uniform_list`, actions, focus handles, scrolling, and typed drag payloads. They do not define additional feature scope for this port.

The port should preserve the algorithmic properties that make Pierre scale. It should not translate JavaScript typed arrays, React rendering, or browser scheduling literally when Rust and GPUI have better native representations.

`FileTree` owns an in-memory path model and interaction state. It does not traverse or mutate the real filesystem. Applications feed it paths and call its mutation methods when their filesystem model changes.

## Repository and target

This issue is tracked and implemented in [`base-gpui`](https://github.com/LukeTandjung/base-gpui).

The crate currently pins Zed/GPUI commit `59b2ebf10351b5c0b5cd4403f01ed0460eeec06d`.

Proposed module shape:

```text
src/file_tree/
  mod.rs
  actions.rs
  context.rs
  prepared_input.rs
  projection.rs
  props.rs
  runtime.rs
  store.rs
  style_state.rs
  layers/
    mod.rs
    file_tree.rs
    file_tree_row.rs
    sticky_file_tree_rows.rs
  tests/
```

The exact private split may change, but:

- `FileTree` is the single public visual component.
- `FileTreeRuntime` owns all tree behavior and interaction state.
- `FileTreeContext` injects the runtime, props, and callbacks into internal renderers.
- `store.rs`, `projection.rs`, and `prepared_input.rs` are private implementation modules owned by the runtime.
- Renderable GPUI elements belong under `layers/`.
- No `child.rs` or `child_wiring.rs` is needed because rows are data-driven and virtualized, not public compound children.
- `mod.rs` files remain barrel exports only.

## Primary source

Research snapshot: [`pierrecomputer/pierre@55a9419`](https://github.com/pierrecomputer/pierre/tree/55a941914056af44c78c4ba607b37130f189fb70)

### Public file-tree behavior

- `packages/trees/README.md`
- `packages/trees/src/model/FileTreeController.ts`
- `packages/trees/src/model/publicTypes.ts`
- `packages/trees/src/model/virtualization.ts`
- `packages/trees/src/model/layout.ts`
- `packages/trees/src/model/density.ts`
- `packages/trees/src/model/searchHelpers.ts`
- `packages/trees/src/model/dragAndDrop.ts`
- `packages/trees/src/model/gitStatus.ts`
- `packages/trees/src/model/mutationEvents.ts`
- `packages/trees/src/preparedInput.ts`
- `packages/trees/src/render/FileTree.ts`
- `packages/trees/src/render/FileTreeView.tsx`
- `packages/trees/src/render/controllerSnapshotSubscription.ts`
- `packages/trees/src/render/rowClickPlan.ts`
- `packages/trees/src/render/scrollTarget.ts`
- `packages/trees/src/render/RenameInput.tsx`

### Path-store and projection performance

- `packages/path-store/src/builder.ts`
- `packages/path-store/src/child-index.ts`
- `packages/path-store/src/events.ts`
- `packages/path-store/src/internal-types.ts`
- `packages/path-store/src/projection.ts`
- `packages/path-store/src/segments.ts`
- `packages/path-store/src/soa-node-store.ts`
- `packages/path-store/src/sort.ts`
- `packages/path-store/src/state.ts`
- `packages/path-store/src/store.ts`
- `packages/path-store/src/visible-tree-projection.ts`
- `packages/path-store/src/cleanup.ts`

### Tests and benchmarks

- `packages/trees/test/file-tree-controller-focus.test.ts`
- `packages/trees/test/file-tree-keyboard-selection.test.ts`
- `packages/trees/test/file-tree-public-navigation.test.ts`
- `packages/trees/test/file-tree-row-click-plan.test.ts`
- `packages/trees/test/file-tree-row-interactions.test.ts`
- `packages/trees/test/file-tree-layout.test.ts`
- `packages/trees/test/file-tree-virtualization-window.test.ts`
- `packages/trees/test/file-tree-sticky-rows.test.ts`
- `packages/trees/test/file-tree-search.test.ts`
- `packages/trees/test/file-tree-search-manual-collapse.test.ts`
- `packages/trees/test/file-tree-renaming.test.ts`
- `packages/trees/test/file-tree-rename-handoff.test.ts`
- `packages/trees/test/file-tree-drag-and-drop.test.ts`
- `packages/trees/test/file-tree-dynamic-files-mutation-api.test.ts`
- `packages/trees/test/file-tree-git-status.test.ts`
- `packages/trees/test/file-tree-icon-config.test.ts`
- `packages/trees/test/file-tree-density.test.ts`
- `packages/trees/test/file-tree-composition-surfaces.test.ts`
- `packages/path-store/scripts/benchmark.ts`
- `packages/path-store/scripts/benchmarkNodeStorageSoA.ts`
- `packages/path-store/scripts/benchmarkVisibleTreeProjection.ts`
- `packages/path-store/scripts/profileVisibleTreeProjection.ts`
- `packages/trees/scripts/benchmarkFileTreeGetItem.ts`
- `packages/trees/scripts/profileFileTree.ts`

## Secondary GPUI references

These references guide GPUI implementation mechanics only.

### Zed

Use the GPUI revision pinned by `base-gpui`:

- [`crates/project_panel/src/project_panel.rs`](https://github.com/zed-industries/zed/blob/59b2ebf10351b5c0b5cd4403f01ed0460eeec06d/crates/project_panel/src/project_panel.rs)
- `crates/gpui/src/elements/uniform_list.rs`
- `crates/gpui/examples/drag_drop.rs`

Relevant ideas:

- render requested list ranges through `uniform_list`,
- use `FocusHandle`, key contexts, and actions,
- render sticky ancestors separately from the flow list,
- use concrete typed GPUI drag payloads.

Do not add Zed project-panel features solely because Zed implements them.

### gpui-component

Research snapshot: [`longbridge/gpui-component@5a5e2ab`](https://github.com/longbridge/gpui-component/blob/5a5e2abc837b6d927e2b38e2097d2fbf39ebee77/crates/base/src/tree.rs)

Relevant ideas:

- one public tree component backed by an entity,
- caller-configured tree state,
- GPUI action wiring,
- reveal and scroll helpers,
- uniform-list row rendering.

Do not inherit its index-based persistent selection or eager full nested-item projection.

## Port boundary

### Port from Pierre

- Path-first public identity.
- Raw, prepared, and presorted path input.
- Initial expansion modes and explicit expanded paths.
- Optional compact single-child directory chains.
- Multi-selection, range selection, focus, and navigation.
- Fixed-row virtualization, overscan, and reusable render windows.
- Sticky ancestor rows.
- Search and all three search modes.
- Inline rename.
- Internal path drag and drop.
- Add, remove, move, batch, and reset mutations.
- Mutation events and subscription semantics.
- Density, icons, Git status, row decorations, header, and context-menu composition.

### Translate rather than copy

- Private JavaScript numeric IDs become a private Rust `NodeId` or arena index.
- JavaScript segment tables and typed arrays become compact Rust vectors, arenas, and interners. A structure-of-arrays layout is only an optional native benchmark candidate, not the required primary runtime layout.
- React subscriptions become GPUI entity notifications and stable query APIs.
- DOM focus becomes `FocusHandle` and actions.
- Browser virtualization math becomes GPUI `uniform_list` range rendering plus Pierre-equivalent window reuse where useful.
- CSS sticky positioning becomes a separate GPUI sticky-row overlay.
- DOM data attributes and CSS variables become typed style-state structs.
- Browser drag events become one concrete internal GPUI drag payload containing canonical paths.
- Pierre's ARIA-derived row metadata becomes typed row metadata and AccessKit properties where the pinned GPUI revision supports them.

### Do not port

- React hooks, JSX, providers, or lifecycle mechanics.
- Web components and shadow DOM.
- SSR, hydration, preloaded HTML, or fake browser focus.
- `HTMLElement`, `DOMRect`, portal containment attributes, or DOM refs.
- CSS classes, CSS variables, raw CSS, or web `style` prop bags.
- Browser `DataTransfer`, requestAnimationFrame scheduling, or browser-specific drag preview workarounds.
- Actual filesystem traversal, file watching, Git queries, editor integration, trash, or undo history.
- Zed-only stable public entry IDs, project/worktree services, copy/cut/paste, diagnostics, lazy filesystem loading, background projection, or external-path drops.
- Variable-height rows. Pierre's model and GPUI `uniform_list` both rely on fixed row height.

## Public API direction

Expected public types include:

- `FileTree`
- `FileTreeRuntime`
- `FileTreeContext`
- `FileTreeProps`
- `FileTreePath`
- `FileTreePreparedInput`
- `FileTreeVisibleRow`
- `FileTreeVisibleSegment`
- `FileTreeItemHandle`
- `FileTreeSearchMode`
- `FileTreeDensity`
- `FileTreeMutation`
- `FileTreeMutationEvent`
- `FileTreeDropTarget`
- `FileTreeDropContext`
- `FileTreeDropResult`
- `FileTreeRenamingConfig`
- `FileTreeRenameEvent`
- `FileTreeStyleState`
- `FileTreeRowStyleState`

Names may be refined during implementation.

Public identity must remain path-first. Private numeric node IDs are an implementation detail and must not appear in selection, focus, mutation, rename, drag/drop, or callback APIs.

The Rust path representation must preserve Pierre's input-string semantics without filesystem I/O: split on `/`, preserve segment spelling, and use a trailing `/` to encode directory kind. Do not add lexical normalization or reject `..` or backslash characters beyond Pierre's own rename rules.

## Suggested implementation order

1. Compact path store, prepared input, visible counts, range projection, expansion, and basic `uniform_list` rendering.
2. Focus, selection, keyboard and pointer behavior, search, mutations, rename, and internal drag/drop.
3. Sticky rows, icons, Git status, decorations, context menus, benchmarks, and performance hardening.

Every stage remains Pierre parity. Zed and `gpui-component` only help choose GPUI primitives.

## Acceptance Criteria

### Module and architecture

- [ ] Add a public `file_tree` module and re-export the intended public API from `src/lib.rs`.
- [ ] Keep `FileTree` as the single public visual component.
- [ ] Keep all tree state and behavior in `FileTreeRuntime` and its private store/projection implementation.
- [ ] Use `FileTreeContext` as the dependency-injection boundary for the runtime, props, callbacks, and internal renderers.
- [ ] Keep `FileTree` and internal row renderers thin: translate GPUI events to runtime commands and runtime queries to elements/styles.
- [ ] Keep private numeric node identity out of public APIs.
- [ ] Keep all `mod.rs` files as barrel exports only.
- [ ] Register file-tree actions from `base_gpui::init`.
- [ ] Add rustdoc and `docs/components/file_tree.md`.
- [ ] Add a site example using a realistically large path set.

### Path-first input and prepared data

- [ ] Accept raw path strings with Pierre's exact file/directory representation, including trailing `/` for directories.
- [ ] Split raw paths on `/` once during build or reset without lexically normalizing segment contents.
- [ ] Support Pierre-equivalent `initial_expansion`: closed, open, depth-limited, and explicit expanded paths.
- [ ] Support `flatten_empty_directories`.
- [ ] Support the default directory/file sort and a custom path comparator.
- [ ] Add `FileTreePreparedInput` that reuses parsed path segments and sort metadata across tree construction or reset.
- [ ] Add a presorted prepared-input path that bypasses sorting.
- [ ] For presorted input, retain the previous path and a directory stack, find the shared segment prefix, and open or close only the changed suffix.
- [ ] Detect duplicates in presorted input by checking adjacent paths rather than allocating a whole-input duplicate set.
- [ ] Avoid copying a large presorted path collection when the Rust ownership model permits borrowing or shared ownership.
- [ ] Ingest shared path prefixes without repeatedly parsing or allocating each ancestor.
- [ ] Validate that prepared input and any accompanying raw paths describe the same preserved path set, including trailing-slash directory kind.

### Compact internal store

- [ ] Use a private compact numeric `NodeId` or equivalent arena index.
- [ ] Intern repeated path segments instead of storing a complete path string on every node.
- [ ] Store parent relationships and sorted child IDs in compact indexed structures.
- [ ] Maintain subtree node counts and visible subtree counts needed for range selection.
- [ ] Build per-directory `childIdByNameId` and child-position indexes lazily, then invalidate or repair only affected directories after mutation.
- [ ] Use chunked visible-child summaries, a Fenwick tree, or an equivalent order-statistics structure for very wide directories so visible-index lookup does not linearly scan every sibling.
- [ ] Keep public paths lazily materialized or cached rather than eagerly duplicating every complete path.
- [ ] Version or invalidate path and ancestor caches after mutations.
- [ ] Treat structure-of-arrays as an optional native benchmark candidate against an idiomatic Rust arena/AoS layout, not as the required primary runtime representation.
- [ ] Choose any non-default layout only with native benchmark and allocation evidence.
- [ ] Do not require a full visible-row vector as the canonical projection.

### Visible projection and range materialization

- [ ] Expose a cached total visible row count.
- [ ] Resolve the row at a visible index using subtree-visible counts or equivalent order-statistics metadata.
- [ ] Permit Pierre's compact O(n) reverse sweep to initialize `visibleSubtreeCount`; the initial-path constraint is about avoiding eager projected objects and indexes, not forbidding this count pass.
- [ ] Materialize only a requested visible range into `FileTreeVisibleRow` values.
- [ ] Make virtualized range rendering proportional to the requested window and tree depth, not total visible rows.
- [ ] Avoid constructing row, complete-path, ancestor-path, or accessibility metadata objects for offscreen entries during initial mount and ordinary scrolling.
- [ ] Preserve Pierre row metadata: path, name, kind, depth, level, ancestor paths, expanded, selected, focused, flattened segments, `pos_in_set`, and `set_size`.
- [ ] Materialize ancestor rows and ancestor paths lazily when a caller requests them.
- [ ] Build a full visible projection or path-to-visible-index map only when an operation needs it, cache it by projection version, and invalidate it after relevant changes.
- [ ] Bound initial direct range materialization so opening a very large fully expanded tree allocates only the first requested window plus required metadata.
- [ ] Keep projection bookkeeping reusable across render, keyboard navigation, sticky rows, and scrolling.

### Expansion and compact directory chains

- [ ] Support Pierre's path-targeted expand, collapse, and toggle behavior.
- [ ] Keep expansion keyed by preserved path strings at the public boundary.
- [ ] Update visible subtree counts incrementally when expansion changes.
- [ ] Preserve or remap expansion across ordinary mutations where applicable.
- [ ] Make `reset_paths` construct fresh expansion from the baseline `initial_expansion` options or explicit `initial_expanded_paths`; do not seed reset expansion from the current tree.
- [ ] Fold eligible single-child directory chains into one visible row when enabled.
- [ ] Preserve every folded segment's path for rename, context-menu, and drop targeting.
- [ ] Compute folded chains inside projection logic rather than creating synthetic public entries.
- [ ] Keep empty directories identifiable as directories.

### Focus, selection, and navigation

- [ ] Preserve Pierre's distinction between focused and selected paths.
- [ ] Support single selection, additive toggle, contiguous range, additive range, and select-all-visible.
- [ ] Keep a path-keyed selection anchor.
- [ ] Preserve hidden selected paths unless the underlying path is removed.
- [ ] Repair focus to a visible ancestor or nearby row after collapse, search, or removal.
- [ ] Expose path-first item handles and programmatic navigation methods equivalent to Pierre's controller.
- [ ] Implement first, last, next, previous, parent, and direct-path focus commands.
- [ ] Implement top, center, and nearest scroll-to-path alignment.
- [ ] Cache item handles and ancestor calculations where this avoids repeated allocation.
- [ ] Emit selection notifications only when the selected path set changes.

### Keyboard and pointer behavior

- [ ] Use GPUI actions and a file-tree key context.
- [ ] Match Pierre's Up, Down, Left, Right, Home, End, PageUp, and PageDown behavior.
- [ ] Match Pierre's Enter and Space selection/directory behavior.
- [ ] Support Shift range extension and Cmd/Ctrl additive selection.
- [ ] Keep inline search and rename fields from also dispatching tree navigation actions.
- [ ] Match Pierre's plain, modified, shift, right-click, and sticky-row click plans.
- [ ] Toggle directories on unmodified row clicks without treating files as directories.
- [ ] Preserve existing multi-selection when opening a context menu on an already selected row.
- [ ] Keep keyboard and pointer focus synchronized with the same runtime commands.

### Virtualization and reusable windows

- [ ] Keep rows fixed-height and derive row height from density or explicit configuration.
- [ ] Render the flow list through GPUI `uniform_list` or an equivalent fixed-height virtual list.
- [ ] Compute the visible viewport range with constant-time row-height arithmetic.
- [ ] Add configurable overscan with Pierre's default behavior as the reference.
- [ ] Retain the current overscanned window while the viewport remains inside it instead of rebuilding the mounted range on every small scroll.
- [ ] Fetch the mounted range directly from the indexed projection.
- [ ] Keep total height, viewport offset, mounted window, and sticky inset calculations explicit and unit-tested.
- [ ] Preserve scroll position or intentionally repair it when projection changes above the viewport.
- [ ] Prove with instrumentation that scrolling a large tree does not materialize or clone all visible rows.

### Sticky ancestor rows

- [ ] Derive each sticky slot from the current scroll position plus that slot's row-height offset, not from focus.
- [ ] Track each sticky directory's visible row index and subtree end index so the next row outside the subtree pushes it out at the directory boundary.
- [ ] Materialize only the small sticky ancestor set.
- [ ] Render sticky rows in a separate GPUI overlay rather than copying browser CSS sticky mechanics.
- [ ] Keep sticky rows as mirrors of canonical flow rows, not duplicate persistent identities.
- [ ] Hand sticky-row interaction back to the canonical path and reveal the corresponding flow row.
- [ ] Include sticky height when resolving scroll-to-path and keyboard autoscroll.
- [ ] Keep context-menu anchoring stable while the flow list scrolls.
- [ ] Skip sticky projection work entirely when sticky folders are disabled.

### Search and filtered projection

- [ ] Support Pierre's `expand-matches`, `collapse-non-matches`, and `hide-non-matches` modes.
- [ ] Support open, set, clear, close, next-match, and previous-match commands.
- [ ] Preserve and restore pre-search expansion state.
- [ ] Preserve Pierre's manual-collapse behavior while search is active.
- [ ] Keep selected hidden paths selected.
- [ ] Repair focus and scrolling when the focused path stops matching.
- [ ] Keep close-on-blur and retain-on-blur behavior configurable.
- [ ] Cache lowercase search corpora for all listed paths and for known directory paths; use the directory-only corpus for `expand-matches`.
- [ ] Invalidate both search corpora after mutations and resets.
- [ ] Store filtered visible indexes or equivalent compact mapping rather than rebuilding full row objects.
- [ ] Coalesce contiguous underlying visible ranges before materializing filtered rows.
- [ ] Avoid one complete projection traversal or one path lookup per rendered filtered row.
- [ ] Emit search notifications only when the effective query changes.

### Mutations, invalidation, and subscriptions

- [ ] Support `add`, recursive `remove`, `move`, `batch`, and `reset_paths`.
- [ ] Support collision policies equivalent to error, replace, and skip.
- [ ] Update descendant paths when a directory moves.
- [ ] Preserve or remap expansion, focus, selection, search, rename, and drag state by path after ordinary mutations.
- [ ] Recompute only affected child indexes, subtree counts, visible counts, and caches where practical.
- [ ] Batch multiple operations into one projection invalidation and one notification pass.
- [ ] Emit Pierre-equivalent mutation events with operation details, canonical/projection change flags, and visible-count delta where known.
- [ ] Keep whole-tree reset as an explicit coarse operation, allow prepared input, and rebuild expansion from reset options rather than current expansion.
- [ ] Expose one stable subscription API for consumers and selector-style integrations.
- [ ] Avoid duplicate initial notifications when a renderer or selector resubscribes.

### Inline rename

- [ ] Support Pierre's `renaming` boolean/config surface with `can_rename`, `on_rename`, and `on_error`.
- [ ] Expand collapsed ancestors and reveal the row before starting rename.
- [ ] Select the renaming path, close search, and focus the inline editor without inventing a text-selection range contract.
- [ ] Commit on Enter and cancel on Escape according to Pierre behavior.
- [ ] Reject only an empty trimmed basename, a basename containing `/`, and destination collisions; treat an unchanged basename as a no-op and do not reject `..` or backslashes.
- [ ] Invoke `on_rename` with source path, destination path, and folder flag.
- [ ] Apply the corresponding internal path move synchronously after a successful rename callback, matching Pierre's controller contract.
- [ ] Support placeholder removal on cancel where Pierre's `removeIfCanceled` behavior is used.
- [ ] Restore focus and emit `on_error` without corrupting the path model when validation or move fails.
- [ ] Support renaming a segment in a compact directory chain.

### Internal drag and drop

- [ ] Use one concrete GPUI payload containing the dragged canonical paths.
- [ ] Start a drag from the current selection and remove redundant selected descendants.
- [ ] Disable drag while search filtering is active, matching Pierre.
- [ ] Support `can_drag`, `can_drop`, `on_drop_complete`, `on_drop_error`, and delayed directory opening.
- [ ] Distinguish root, directory, hovered-row, and compact-segment targets.
- [ ] Reject self and descendant drops.
- [ ] Build Pierre-equivalent move or batch plans with collision handling.
- [ ] Apply the internal move or batch synchronously and report completion/error callbacks.
- [ ] Support edge auto-scroll during drag.
- [ ] Clear drag state before applying mutations so subscribers observe a closed drag session.
- [ ] Do not add copy semantics or native external-path drops in this port.

### Rendering, composition, and styling

- [ ] Render built-in file-tree rows rather than exposing public compound item children.
- [ ] Support configurable file, folder, open-folder, disclosure, search, rename, and context-menu icons equivalent to Pierre's icon configuration.
- [ ] Support Git statuses: added, deleted, ignored, modified, renamed, and untracked.
- [ ] Support caller-provided row decorations alongside Git and action lanes.
- [ ] Support a caller-owned header.
- [ ] Support Pierre's right-click, button, and combined context-menu trigger modes.
- [ ] Support context-menu button visibility modes and focus restoration.
- [ ] Support compact, default, relaxed, and explicit row-density configuration.
- [ ] Truncate long filenames predictably while preserving extension visibility where practical.
- [ ] Add `FileTreeStyleState` and `FileTreeRowStyleState` covering Pierre-visible interaction and presentation state.
- [ ] Expose styling through normal GPUI builders and `style_with_state(...)`, not CSS variables or DOM attributes.

### Accessibility translation

- [ ] Preserve level, position-in-set, set-size, expanded, selected, and item-kind metadata in visible rows.
- [ ] Map Pierre's tree/tree-item accessibility semantics into AccessKit where the pinned GPUI revision supports them.
- [ ] Ensure sticky mirror rows are not exposed as duplicate accessible items.
- [ ] Document unsupported AccessKit properties without adding DOM-like APIs.

### Correctness tests

- [ ] Port behavior-level tests for path preservation, trailing-slash directory semantics, sorting, expansion, compact chains, selection, focus repair, and navigation.
- [ ] Port virtualization window, window-reuse, layout, scroll-target, and sticky-row tests, including nested sticky transitions and subtree-end displacement.
- [ ] Port all three search modes and manual-collapse tests.
- [ ] Port rename, rename-handoff, cancellation, validation, unchanged-name, and path-remapping tests.
- [ ] Port internal drag/drop planning, validation, delayed-open, auto-scroll, completion, and error tests.
- [ ] Port add, remove, move, collision, batch, reset, invalidation, and subscription tests.
- [ ] Port density, icon, Git status, header, context-menu, and row-decoration tests.
- [ ] Unit-test the runtime and private store/projection without a GPUI window where possible.
- [ ] Add rendered GPUI tests only for event wiring, focus, list ranges, sticky overlays, and visual-state exposure.

### Performance tests and benchmarks

- [ ] Benchmark raw, prepared, and presorted construction separately.
- [ ] Benchmark at representative sizes such as 10K, 100K, and several hundred thousand paths.
- [ ] Benchmark the presorted previous-path/directory-stack fast path, shared-prefix reuse, and adjacent duplicate checking.
- [ ] Include very wide-directory cases that exercise lazy child maps, child-position indexes, and chunked or Fenwick-equivalent visible-child summaries.
- [ ] Measure construction time, allocations, retained memory, visible-count calculation, range materialization, expand/collapse, path lookup, search, and mutation batches.
- [ ] Verify that initial mount of a very large open tree may perform the compact count sweep but does bounded direct range materialization and does not allocate all projected rows, paths, path indexes, or accessibility metadata.
- [ ] Verify that ordinary scrolling materializes only the mounted range plus overscan and sticky ancestors.
- [ ] Verify that `visible_count` does not require materializing visible rows.
- [ ] Verify that prepared input avoids repeated parsing and sorting without changing preserved path strings.
- [ ] Verify that presorted prepared input bypasses sorting and avoids unnecessary large input copies.
- [ ] Benchmark repeated search query changes and confirm the lowercase all-path and directory-only corpora are reused until mutation invalidates them.
- [ ] Verify that a small expansion or mutation does not rebuild unrelated path indexes or row objects.
- [ ] Profile an idiomatic Rust arena/AoS primary layout and, optionally, a structure-of-arrays candidate before selecting a different internal node layout.
- [ ] Keep benchmark instrumentation capable of attributing time to build, sort, count initialization, projection, materialization, search, and mutation phases.
- [ ] Document native benchmark results and any deliberate divergence from Pierre's browser-oriented representation.

### Verification

- [ ] Run `cargo fmt --check`.
- [ ] Run `cargo check`.
- [ ] Run `cargo test file_tree`.
- [ ] Run the full `cargo test` suite.
- [ ] Run `cargo clippy --all-targets`.
- [ ] Run `ast-grep scan src`.
- [ ] When the site example changes, run `cargo fmt --check --manifest-path site/Cargo.toml` and `cargo check --manifest-path site/Cargo.toml`.

## Completion notes

Completion means Pierre's public file-tree behavior and performance model have GPUI-native equivalents. Zed and `gpui-component` are successful references only when they help express Pierre's behavior with GPUI primitives without expanding the feature set.
