//! Pierre-compatible path-first file tree for GPUI.
//!
//! [`FileTree`] is the only public visual component. Build it from raw paths,
//! a reusable [`FileTreePreparedInput`], or validated presorted paths. Attach a
//! [`FileTreeHandle`] when the application needs imperative queries or commands.
//!
//! # Path semantics
//!
//! Paths are in-memory identifiers. The component does no filesystem I/O.
//! It splits paths only on `/`, preserves segment spelling, and does no lexical
//! normalization. A trailing `/` marks a directory. Thus `src` is a file and
//! `src/` is a directory. `..` and backslashes remain ordinary segment text.
//!
//! # Input and construction
//!
//! [`FileTreePreparedInput::prepare`] parses and applies Pierre's default
//! directory-first natural order. [`FileTreePreparedInput::prepare_with_comparator`]
//! accepts a custom depth-first comparator. [`FileTreePreparedInput::presorted`]
//! trusts caller order, checks adjacent path conflicts, and lets the private store
//! use its previous-path and directory-stack ingest path. Prepared values use
//! shared storage and are cheap to clone for repeated construction or reset.
//!
//! # Handles and mutations
//!
//! Create a handle with [`create_file_tree_handle`] or [`FileTreeHandle::new`],
//! then pass a clone to [`FileTree::handle`]. Item lookup is canonical and cached.
//! File and directory handles expose focus and selection commands. Directory
//! handles also expose expansion commands. The tree handle provides path-first
//! navigation, scrolling, search, rename, Git-state, drag/drop, and mutation APIs.
//!
//! Mutations use [`FileTreeBatchOperation`] and support add, recursive remove,
//! move, collision policy, batches, and reset. [`FileTreeMutationEvent`] reports
//! canonical and projection invalidation. Subscribe through
//! [`FileTreeHandle::on_any_mutation`] or [`FileTreeHandle::on_mutation`].
//!
//! # Search, rename, and drag/drop
//!
//! [`FileTreeSearchMode`] selects expand-matches, collapse-non-matches, or
//! hide-non-matches behavior. Search keeps path selection separate from filtered
//! visibility and restores the pre-search expansion state when it closes.
//! [`FileTreeRenamingConfig`] controls inline rename validation and callbacks.
//! [`FileTreeDragAndDropConfig`] controls typed internal path moves, validation,
//! delayed folder opening, completion, and error callbacks. External filesystem
//! drops and copy semantics are outside this component.
//!
//! # Sticky rows and composition
//!
//! [`FileTree::sticky_folders`] renders a small overlay of scrolled ancestors.
//! Sticky rows mirror canonical flow rows and do not create persistent item
//! identity. [`FileTreeComposition`] configures a caller-owned header and context
//! menu. Icons, Git status, row decorations, density, and state-aware GPUI styles
//! are configured through the related public types and `FileTree` builders.
//!
//! # Accessibility
//!
//! The root uses tree semantics. Canonical rows expose tree-item role, label,
//! level, position, set size, selection, expansion, and active-descendant state.
//! Sticky mirrors omit tree-item role to avoid duplicate accessible items. The
//! pinned GPUI revision has no builders for `aria-hidden`, `aria-haspopup`, or a
//! separate item-kind property. Item kind is included in the accessible label,
//! and context-menu availability is exposed by its visible action button.
//!
//! # Performance model
//!
//! The private store uses numeric node IDs, interned segments, lazy path caches,
//! subtree counts, chunked wide-directory summaries, and range materialization.
//! Rendering uses a fixed-height `uniform_list`; it materializes the mounted
//! range plus sticky ancestors instead of a complete row vector. Search reuses
//! lowercase path corpora until mutation invalidates them. The ignored Rust
//! performance harness documents construction, projection, expansion, search,
//! and mutation costs. See `docs/components/file_tree.md` for commands and the
//! current deliberate limits.

mod actions;
mod context;
mod controller;
mod drag_and_drop;
mod git_status;
mod interaction;
mod layers;
mod mutation;
mod prepared_input;
mod presentation;
mod projection;
mod props;
mod rename;
mod runtime;
mod search;
mod sticky;
mod store;

#[cfg(test)]
mod tests;

pub use actions::init;
pub use controller::{
    create_file_tree_handle, FileTreeDirectoryHandle, FileTreeFileHandle, FileTreeHandle,
    FileTreeItemHandle, FileTreeMutationSubscription, FileTreeScrollOffset,
    FileTreeScrollToPathOptions,
};
pub use drag_and_drop::{
    FileTreeDragAndDropConfig, FileTreeDropContext, FileTreeDropError, FileTreeDropOperation,
    FileTreeDropResult, FileTreeDropTarget, FileTreeDropTargetKind,
};
pub use git_status::{
    apply_file_tree_git_status_patch, resolve_file_tree_git_status_state, FileTreeGitStatus,
    FileTreeGitStatusEntry, FileTreeGitStatusPatch, FileTreeGitStatusState,
};
pub use layers::FileTree;
pub use mutation::{
    FileTreeAddEvent, FileTreeBatchEvent, FileTreeBatchOperation, FileTreeCollisionStrategy,
    FileTreeMoveEvent, FileTreeMoveOptions, FileTreeMutationError, FileTreeMutationEvent,
    FileTreeMutationEventType, FileTreeMutationInvalidation, FileTreeMutationSemanticEvent,
    FileTreeRemoveEvent, FileTreeRemoveOptions, FileTreeResetEvent, FileTreeResetOptions,
};
pub use prepared_input::{FileTreeInputError, FileTreePathSortEntry, FileTreePreparedInput};
pub use presentation::{
    middle_truncate_file_name, normalize_file_tree_icons, resolve_file_tree_density,
    split_file_name_for_middle_truncation, FileTreeBuiltInFileIcon, FileTreeBuiltInIconSet,
    FileTreeComposition, FileTreeCompositionItem, FileTreeContextMenuButtonVisibility,
    FileTreeContextMenuClose, FileTreeContextMenuCloseHandler, FileTreeContextMenuComposition,
    FileTreeContextMenuOpenContext, FileTreeContextMenuOpenHandler, FileTreeContextMenuRenderer,
    FileTreeContextMenuSource, FileTreeContextMenuTriggerMode, FileTreeDensity,
    FileTreeDensityPreset, FileTreeDropState, FileTreeHeaderComposition, FileTreeHeaderRenderer,
    FileTreeIcon, FileTreeIconConfig, FileTreeIconRenderContext, FileTreeIconRenderer,
    FileTreeIconResolver, FileTreeIconRule, FileTreeIconSet, FileTreeIconSlot, FileTreeIcons,
    FileTreeItemKind, FileTreeMiddleTruncation, FileTreeResolvedIcon, FileTreeResolvedIconSource,
    FileTreeRowDecorationContext, FileTreeRowDecorationRenderer, FileTreeRowStyleState,
    FileTreeRowStyleWithState, FileTreeStyleState, FileTreeStyleWithState, FileTreeVisibleRow,
    FileTreeVisibleSegment, NormalizedFileTreeIconConfig,
};
pub use props::FileTreeInitialExpansion;
pub use rename::{
    FileTreeRenameError, FileTreeRenameEvent, FileTreeRenamingConfig, FileTreeRenamingItem,
    FileTreeStartRenamingOptions,
};
pub use search::{FileTreeSearchBlurBehavior, FileTreeSearchMode};
