# File Tree

A path-first, fixed-row virtualized file tree based on Pierre's `@pierre/trees` behavior and performance model.

## Anatomy

`FileTree` is the only visual component. It owns the internal runtime and renders built-in rows. Public nonvisual types configure input, behavior, presentation, and imperative control.

```rust
use base_gpui::file_tree::{
    create_file_tree_handle, FileTree, FileTreeInitialExpansion,
    FileTreePreparedInput, FileTreeSearchMode,
};

let input = FileTreePreparedInput::prepare([
    "Cargo.toml",
    "src/lib.rs",
    "src/file_tree/mod.rs",
    "target/",
])?;
let handle = create_file_tree_handle();

let tree = FileTree::new(input)
    .id("project-files")
    .handle(handle.clone())
    .initial_expansion(FileTreeInitialExpansion::Depth(2))
    .search(true)
    .search_mode(FileTreeSearchMode::HideNonMatches)
    .renaming(true)
    .drag_and_drop(true)
    .sticky_folders(true);
# Ok::<_, base_gpui::file_tree::FileTreeInputError>(tree)
```

Call `base_gpui::init(cx)` before rendering. It registers the file-tree actions and key bindings with the other crate components.

## Path semantics

The tree treats each path as an in-memory identifier. It does not read, watch, or modify the filesystem.

- `/` is the only separator.
- A trailing `/` marks a directory. `src` is a file and `src/` is a directory.
- Segment spelling is preserved.
- The tree does not resolve `.`, `..`, repeated separators, or backslashes.
- Backslashes and `..` remain ordinary segment text.
- Public selection, focus, mutation, rename, drag/drop, and callback values use preserved paths.
- Directory lookup accepts the canonical trailing-slash path and the corresponding bare path where Pierre does.

Rename validation rejects an empty trimmed basename, a basename containing `/`, and a destination collision. It does not reject `..` or backslashes.

## Input and prepared data

### Raw input

Use `FileTree::from_paths(...)` for a small one-off tree. It parses and sorts with Pierre's default directory-first natural order.

### Reusable prepared input

Use `FileTreePreparedInput::prepare(...)` when several trees or resets share the same paths. The value owns shared parsed path and sort data and is cheap to clone.

Use `FileTreePreparedInput::prepare_with_comparator(...)` for custom ordering. The callback receives `FileTreePathSortEntry` values with `path`, `basename`, `segments`, `depth`, and `is_directory`. Return a stable `std::cmp::Ordering`. Keep every directory subtree contiguous so the depth-first store can ingest it.

```rust
use base_gpui::file_tree::FileTreePreparedInput;

let input = FileTreePreparedInput::prepare_with_comparator(
    ["a.rs", "c.rs", "b.rs"],
    |left, right| right.basename.cmp(left.basename),
)?;
# Ok::<_, base_gpui::file_tree::FileTreeInputError>(input)
```

Custom comparator order is retained when ordinary mutations rebuild prepared data.

### Presorted input

Use `FileTreePreparedInput::presorted(...)` or `FileTree::from_presorted_paths(...)` only when the caller already supplies Pierre's default depth-first order. The fast path trusts that order, as Pierre does. It checks adjacent duplicates and file/directory collisions but does not sort or detect a noncontiguous repeated subtree. Construction uses a previous-path, shared-prefix, and directory-stack builder. The path collection uses shared ownership after preparation.

### Initial expansion and compact chains

`FileTreeInitialExpansion` supports `Closed`, `Open`, and `Depth(n)`. Use `.initial_expanded_paths(...)` for an explicit baseline. `reset_paths` rebuilds expansion from that baseline or reset options, not from the current expansion state.

`.flatten_empty_directories(true)` folds eligible single-child directory chains into one row. Every visible segment keeps its canonical path for rename, context menu, and drop targeting.

## FileTree builders

| Builder | Purpose |
| --- | --- |
| `id` | Set the stable GPUI element ID. |
| `initial_expansion`, `initial_expanded_paths` | Set baseline expansion. |
| `flatten_empty_directories` | Enable compact directory chains. |
| `search`, `search_mode`, `initial_search_query`, `search_blur_behavior` | Configure inline search. |
| `initial_selected_paths`, `on_selection_change` | Configure path selection and change delivery. |
| `on_search_change` | Observe effective query changes. |
| `renaming`, `renaming_config` | Enable and configure inline rename. |
| `drag_and_drop`, `drag_and_drop_config` | Enable and configure internal path moves. |
| `sticky_folders` | Pin scrolled ancestor directories. |
| `handle` | Bind a public imperative handle. |
| `density`, `row_height` | Set fixed row layout. |
| `icons`, `git_status` | Configure built-in icon and Git lanes. |
| `composition`, `header`, `context_menu_config` | Add caller-owned composition. |
| `row_decoration` | Render a caller-owned row decoration lane. |
| `style_with_state`, `row_style_with_state` | Apply GPUI style from typed state. |
| `filename_max_chars` | Set the middle-truncation character budget. The minimum is 5. |

`FileTreeDensity` supports compact, default, relaxed, and explicit values. An explicit `.row_height(...)` overrides the density item height while the density still controls related spacing.

## Imperative handle

Create a `FileTreeHandle`, clone it into `.handle(...)`, and keep the other clone. An unbound handle returns empty query values or `false`; mutation calls return `FileTreeMutationError::NotBound`.

### Queries

- `get_item`, `get_focused_item`, `get_focused_path`, `get_focused_index`
- `get_selected_paths`, `get_visible_count`
- `get_search_value`, `is_search_open`, `get_search_matching_paths`
- `is_renaming`, `get_renaming_path`
- `is_drag_and_drop_enabled`, `dragged_paths`, `drop_target`

`get_item` caches canonical item handles. `FileTreeFileHandle` supports path, focus, and selection queries and commands. `FileTreeDirectoryHandle` adds `is_expanded`, `expand`, `collapse`, and `toggle`. Expanding a directory also expands collapsed ancestors. Focusing a hidden path repairs to its nearest visible ancestor.

### Navigation and scrolling

The handle supports `focus_path`, `focus_first_item`, `focus_last_item`, `focus_next_item`, `focus_previous_item`, and `focus_parent_item`.

`scroll_to_path` focuses by default. `scroll_to_path_with_options` accepts `FileTreeScrollToPathOptions` with `focus` and `FileTreeScrollOffset::{Top, Center, Nearest}`. Set `focus: false` to scroll without changing model or platform focus. Missing or search-hidden paths do not scroll.

Public focus commands do not request a programmatic scroll. Keyboard navigation uses nearest-row autoscroll.

## Mutations and subscriptions

The handle supports:

- `add(path)`
- `remove(path, FileTreeRemoveOptions)`
- `move_path(from, to, FileTreeMoveOptions)`
- `batch(&[FileTreeBatchOperation])`
- `reset_paths`, `reset_paths_with_options`, and `reset_prepared_input`

`FileTreeCollisionStrategy` supports `Error`, `Replace`, and `Skip`. Directory moves remap descendant paths. Ordinary mutations repair or remap expansion, focus, selection, active search, rename, drag state, and cached item handles.

A successful batch causes one projection invalidation and one notification pass. `FileTreeMutationEvent` includes operation details, canonical/projection flags, and a visible-count delta when known. Use `on_any_mutation` or filtered `on_mutation`; drop the returned `FileTreeMutationSubscription` to unsubscribe.

The current native mutation transaction validates a whole path index and rebuilds the compact arena once per successful batch. It does not rebuild once per operation. Group frequent changes into one `batch`. This is a deliberate performance gap from Pierre's fully incremental canonical store.

## Search

`FileTreeSearchMode` provides:

- `ExpandMatches`: expand ancestors of matching listed paths and known directories.
- `CollapseNonMatches`: collapse branches that do not contain matches.
- `HideNonMatches`: render only matching paths and required ancestors.

Use `set_search`, `open_search`, `open_search_with_value`, `close_search`, `focus_next_search_match`, and `focus_previous_search_match` on the handle. The tree preserves pre-search expansion, tracks manual collapses during search, keeps hidden selected paths selected, and repairs focus when filtering hides it. `FileTreeSearchBlurBehavior` selects close-on-blur or retain-on-blur behavior.

The runtime caches lowercase listed-path and directory-only corpora. Query changes reuse them. Mutations and resets invalidate them.

## Inline rename

Enable rename with `.renaming(true)` or pass `FileTreeRenamingConfig` with `can_rename`, `on_rename`, and `on_error` callbacks. Start it with `start_renaming`, `start_renaming_with_options`, or `start_renaming_focused`.

Starting rename expands and reveals the target, selects it, closes search, and focuses the inline input. Enter commits. Escape cancels. Blur follows Pierre's handoff behavior. A successful callback is followed by the synchronous internal move. Validation or move errors restore focus and leave the path model valid. `FileTreeStartRenamingOptions` supports placeholder removal on cancel. Compact directory segments can be renamed by their preserved segment path.

## Internal drag and drop

Enable drag/drop with `.drag_and_drop(true)` or `FileTreeDragAndDropConfig`. The concrete GPUI payload contains canonical paths. Drag starts from the current selection and removes redundant selected descendants.

The configuration supports `can_drag`, `can_drop`, `on_drop_complete`, `on_drop_error`, and delayed directory opening. Targets distinguish root, directory, hovered row, and compact segment. The planner rejects self and descendant drops, applies move or batch plans synchronously, supports edge auto-scroll, and clears drag state before mutation notifications.

Search filtering disables drag. Copy operations and native external-path drops are outside this port.

## Sticky ancestor rows

`.sticky_folders(true)` enables a separate GPUI overlay. Sticky slots derive from scroll position and row height. Each candidate tracks its canonical visible index and subtree end so the next row outside the subtree pushes it away at the directory boundary.

Only the small ancestor set is materialized. Sticky rows mirror canonical flow rows. Clicks return to the canonical path and reveal its flow row. Sticky inset participates in keyboard and programmatic scroll targeting. Disabling sticky folders skips sticky projection work.

## Composition and presentation

`FileTreeComposition` combines a caller-owned header with `FileTreeContextMenuComposition`. Context menus support right-click, button, and combined triggers, configurable button visibility, and focus restoration. Row decorations render beside Git and action lanes.

`FileTreeIcons` and related resolver/rule types configure file, folder, open-folder, disclosure, search, rename, and context-menu icons. Git state supports added, deleted, ignored, modified, renamed, and untracked. `FileTreeStyleState` and `FileTreeRowStyleState` expose typed interaction and presentation state to normal GPUI style closures.

Long names use deterministic middle truncation. The implementation tries to retain the extension inside the configured character budget.

## Keyboard and pointer behavior

| Input | Result |
| --- | --- |
| Up / Down | Focus the previous or next visible row. |
| Left | Collapse an open directory, otherwise focus its parent. |
| Right | Expand a closed directory, otherwise focus the next row. |
| Home / End | Focus the first or last visible row. |
| PageUp / PageDown | Scroll one viewport without changing controller focus. |
| Enter | Select the focused item and toggle a directory. |
| Space | Toggle focused selection. |
| Shift navigation | Extend the contiguous path range. |
| Cmd/Ctrl selection | Add or toggle without clearing the other selected paths. |
| F2 | Start rename when enabled. |

Inline search and rename inputs consume their own editing keys and do not also dispatch tree navigation.

## Accessibility

The root exposes GPUI tree semantics and owns platform focus. Canonical flow rows expose tree-item role, label, one-based level, position in set, set size, selected state, directory expansion, and active-descendant state. Item kind is included in the accessible label.

The GPUI revision pinned by this repository does not provide builders for:

- `aria-hidden` on sticky mirrors,
- `aria-haspopup` on context-menu triggers,
- a separate AccessKit item-kind property.

Sticky mirrors therefore omit `Role::TreeItem`, which avoids duplicate accessible tree items. The visible context-menu action button exposes menu availability. The component does not add DOM-like attributes to emulate unsupported properties.

## Performance model

The private runtime uses:

- numeric arena node IDs and interned path segments,
- parent links and sorted child ID vectors,
- lazy child lookup and child-position maps,
- subtree node and visible-subtree counts,
- chunked visible-child summaries for wide directories,
- lazy complete-path caching,
- direct visible-index cursor lookup,
- requested-range row materialization,
- fixed-height `uniform_list` rendering,
- a small independent sticky projection,
- reusable lowercase search corpora.

Initial open-tree construction performs a compact visible-count sweep. It does not create a complete `FileTreeVisibleRow` vector. Ordinary rendering materializes the mounted range plus sticky ancestors. `visible_count` reads cached count metadata and does not materialize rows.

### Native benchmark harness

Run the bounded 10K-path suite:

```sh
cargo test --release file_tree::tests::benchmarks::file_tree_performance_harness -- --ignored --nocapture
```

Run 10K, 100K, and 300K hierarchical construction plus a 100K-wide directory:

```sh
FILE_TREE_BENCH_FULL=1 cargo test --release file_tree::tests::benchmarks::file_tree_performance_harness -- --ignored --nocapture
```

The harness measures raw, prepared, and presorted construction; cached `visible_count`; 64-row range materialization; collapse/expand; repeated search query changes; 100-operation add/remove batches; and wide-directory lookup/materialization. It uses only the standard library, so the crate adds no benchmark dependency.

A local x86-64 release run on 2026-08-23 produced these bounded 10K-path samples. Treat them as implementation evidence, not a cross-machine threshold.

| Operation | Sample |
| --- | ---: |
| Raw prepare plus open runtime construction | 19.4 ms |
| Reused prepared open runtime construction | 2.57 ms |
| Presorted open runtime construction | 2.40 ms |
| Cached `visible_count` | 0.41 ns/read |
| Materialize 64 rows near the middle | 15.9 µs |
| Collapse plus expand the top directory | 0.75 µs |
| Three search queries plus clear | 1.48 ms |
| Add and remove two 100-operation batches | 51.7 ms |
| Materialize the last 64 rows of a 10K-wide directory | 10.3 µs |

Allocation counts and retained-memory attribution are not built into the harness. Use a native allocator profiler around the same command when those values are needed. The primary layout is the idiomatic Rust arena/AoS form. No SoA layout replaces it without separate native allocation evidence.

## Deliberate limits

- The component owns an in-memory model only. It does no filesystem, watcher, Git query, editor, trash, or undo work.
- Rows have fixed height. Variable-height rows are outside the model.
- Pinned GPUI `uniform_list` owns its exact mounted range and exposes no overscan control, so `FileTree` has no separate overscan builder.
- Drag/drop is internal move only. It has no copy or external native-path mode.
- A successful mutation batch rebuilds the private canonical arena once. Batch related changes to bound this cost.
- The benchmark harness reports time. It does not directly count allocations or retained bytes.
- Accessibility follows the builders available in pinned GPUI, as listed above.
