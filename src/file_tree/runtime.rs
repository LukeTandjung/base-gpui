use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::{Hash, Hasher},
    ops::Range,
};

use super::{
    drag_and_drop::{resolve_dragged_paths_for_start, FileTreeDragSession, FileTreeDropTarget},
    interaction::{FileTreeRowClickPlan, FileTreeSelectionDirective},
    mutation::{
        list_count, remap_path, AppliedOperation, FileTreeAddEvent, FileTreeBatchEvent,
        FileTreeBatchOperation, FileTreeMoveEvent, FileTreeMutationError, FileTreeMutationEvent,
        FileTreeMutationInvalidation, FileTreeMutationSemanticEvent, FileTreeRemoveEvent,
        FileTreeResetEvent, MutationTransaction, PathMutation, PendingSemanticEvent,
    },
    prepared_input::strip_one_terminal_slash,
    presentation::{FileTreeItemKind, FileTreeVisibleRow},
    projection::{materialize_indices, materialize_range},
    rename::{
        public_rename_path, rename_destination, rename_leaf_name, FileTreeRenameCommit,
        FileTreeRenameError, FileTreeRenameEvent, FileTreeRenamePlan,
    },
    search::normalize_search_query,
    sticky::{layout_sticky_rows, FileTreeStickyCandidate, FileTreeStickyRow},
    store::FileTreeStore,
    FileTreeInitialExpansion, FileTreePreparedInput, FileTreeSearchMode,
};

#[derive(Clone, Debug, Eq, PartialEq)]
struct SourceKey {
    input_fingerprint: u64,
    initial_expansion: FileTreeInitialExpansion,
    initial_expanded_paths_fingerprint: u64,
    flatten_empty_directories: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FileTreeRuntimeUpdate {
    pub state_changed: bool,
    pub selection_changed: bool,
}

impl FileTreeRuntimeUpdate {
    fn state_changed() -> Self {
        Self {
            state_changed: true,
            selection_changed: false,
        }
    }

    fn selection_changed() -> Self {
        Self {
            state_changed: true,
            selection_changed: true,
        }
    }

    fn combine(self, other: Self) -> Self {
        Self {
            state_changed: self.state_changed || other.state_changed,
            selection_changed: self.selection_changed || other.selection_changed,
        }
    }
}

#[derive(Debug)]
struct SearchPathCache {
    listed_paths: Vec<String>,
    listed_paths_lowercase: Vec<String>,
    directory_paths: Vec<String>,
    directory_paths_lowercase: Vec<String>,
}

#[derive(Debug)]
pub struct FileTreeRuntime {
    source_key: SourceKey,
    current_input: FileTreePreparedInput,
    initial_expansion: FileTreeInitialExpansion,
    baseline_expanded_paths: Vec<String>,
    flatten_empty_directories: bool,
    has_imperative_input: bool,
    store: FileTreeStore,
    focused_path: Option<String>,
    focused_index: Option<usize>,
    selected_paths: Vec<String>,
    selected_path_set: HashSet<String>,
    selection_anchor_path: Option<String>,
    selection_version: u64,
    search_mode: FileTreeSearchMode,
    search_value: Option<String>,
    search_matching_paths: Vec<String>,
    search_match_path_set: HashSet<String>,
    search_previous_expanded_paths: Option<Vec<String>>,
    search_collapsed_overrides: HashSet<String>,
    search_visible_path_set: Option<HashSet<String>>,
    search_visible_paths: Option<Vec<String>>,
    search_visible_indices: Option<Vec<usize>>,
    search_visible_index_by_path: HashMap<String, usize>,
    search_path_cache: Option<SearchPathCache>,
    renaming_path: Option<String>,
    renaming_value: String,
    remove_renaming_path_if_canceled: bool,
    renaming_generation: u64,
    drag_session: Option<FileTreeDragSession>,
    drag_hover_generation: u64,
    drag_target_capture_generation: u64,
    drag_target_resolved_generation: u64,
    drag_scroll_generation: u64,
    drag_scroll_point: Option<(f32, f32, f32)>,
}

impl FileTreeRuntime {
    #[cfg(test)]
    pub fn new(
        input: &FileTreePreparedInput,
        initial_expansion: FileTreeInitialExpansion,
        initial_expanded_paths: &[String],
    ) -> Self {
        Self::new_with_configuration(
            input,
            initial_expansion,
            initial_expanded_paths,
            &[],
            FileTreeSearchMode::default(),
        )
    }

    #[cfg(test)]
    pub fn new_with_selection(
        input: &FileTreePreparedInput,
        initial_expansion: FileTreeInitialExpansion,
        initial_expanded_paths: &[String],
        initial_selected_paths: &[String],
    ) -> Self {
        Self::new_with_configuration(
            input,
            initial_expansion,
            initial_expanded_paths,
            initial_selected_paths,
            FileTreeSearchMode::default(),
        )
    }

    #[cfg(test)]
    pub fn new_with_configuration(
        input: &FileTreePreparedInput,
        initial_expansion: FileTreeInitialExpansion,
        initial_expanded_paths: &[String],
        initial_selected_paths: &[String],
        search_mode: FileTreeSearchMode,
    ) -> Self {
        Self::new_with_configuration_and_flattening(
            input,
            initial_expansion,
            initial_expanded_paths,
            initial_selected_paths,
            search_mode,
            false,
        )
    }

    pub fn new_with_configuration_and_flattening(
        input: &FileTreePreparedInput,
        initial_expansion: FileTreeInitialExpansion,
        initial_expanded_paths: &[String],
        initial_selected_paths: &[String],
        search_mode: FileTreeSearchMode,
        flatten_empty_directories: bool,
    ) -> Self {
        let source_key = source_key(
            input,
            initial_expansion,
            initial_expanded_paths,
            flatten_empty_directories,
        );
        let mut store = FileTreeStore::build_with_flattening(
            input,
            initial_expansion,
            initial_expanded_paths,
            flatten_empty_directories,
        )
        .expect("FileTreePreparedInput must satisfy store invariants");
        let mut selected_paths = Vec::new();
        let mut selected_path_set = HashSet::new();
        for path in initial_selected_paths {
            let Some(path) = store.canonical_path(path) else {
                continue;
            };
            if selected_path_set.insert(path.clone()) {
                selected_paths.push(path);
            }
        }
        let selection_anchor_path = selected_paths.last().cloned();
        let focused_index = selection_anchor_path
            .as_deref()
            .and_then(|path| store.nearest_visible_index_for_path(path))
            .or_else(|| (store.visible_count() > 0).then_some(0));
        let focused_path = focused_index.and_then(|index| store.path_at_visible_index(index));
        let selection_version = u64::from(!selected_paths.is_empty());

        Self {
            source_key,
            current_input: input.clone(),
            initial_expansion,
            baseline_expanded_paths: initial_expanded_paths.to_vec(),
            flatten_empty_directories,
            has_imperative_input: false,
            store,
            focused_path,
            focused_index,
            selected_paths,
            selected_path_set,
            selection_anchor_path,
            selection_version,
            search_mode,
            search_value: None,
            search_matching_paths: Vec::new(),
            search_match_path_set: HashSet::new(),
            search_previous_expanded_paths: None,
            search_collapsed_overrides: HashSet::new(),
            search_visible_path_set: None,
            search_visible_paths: None,
            search_visible_indices: None,
            search_visible_index_by_path: HashMap::new(),
            search_path_cache: None,
            renaming_path: None,
            renaming_value: String::new(),
            remove_renaming_path_if_canceled: false,
            renaming_generation: 0,
            drag_session: None,
            drag_target_capture_generation: 0,
            drag_target_resolved_generation: 0,
            drag_hover_generation: 0,
            drag_scroll_generation: 0,
            drag_scroll_point: None,
        }
    }

    #[cfg(test)]
    pub fn reconcile(
        &mut self,
        input: &FileTreePreparedInput,
        initial_expansion: FileTreeInitialExpansion,
        initial_expanded_paths: &[String],
        search_mode: FileTreeSearchMode,
    ) -> FileTreeRuntimeUpdate {
        self.reconcile_with_flattening(
            input,
            initial_expansion,
            initial_expanded_paths,
            search_mode,
            self.flatten_empty_directories,
        )
    }

    pub fn reconcile_with_flattening(
        &mut self,
        input: &FileTreePreparedInput,
        initial_expansion: FileTreeInitialExpansion,
        initial_expanded_paths: &[String],
        search_mode: FileTreeSearchMode,
        flatten_empty_directories: bool,
    ) -> FileTreeRuntimeUpdate {
        let mode_changed = self.search_mode != search_mode;
        if self.has_imperative_input {
            if !mode_changed {
                return FileTreeRuntimeUpdate::default();
            }
            self.search_mode = search_mode;
            let focus_candidate = self.refresh_active_search_state();
            self.repair_focus(focus_candidate.as_deref());
            return FileTreeRuntimeUpdate::state_changed();
        }

        let source_key = source_key(
            input,
            initial_expansion,
            initial_expanded_paths,
            flatten_empty_directories,
        );
        if self.source_key == source_key && !mode_changed {
            return FileTreeRuntimeUpdate::default();
        }
        if self.source_key == source_key {
            self.search_mode = search_mode;
            let focus_candidate = self.refresh_active_search_state();
            self.repair_focus(focus_candidate.as_deref());
            return FileTreeRuntimeUpdate::state_changed();
        }

        let focused_candidate = self.focused_path.clone();
        let selected_candidates = self.selected_paths.clone();
        let anchor_candidate = self.selection_anchor_path.clone();
        self.store = FileTreeStore::build_with_flattening(
            input,
            initial_expansion,
            initial_expanded_paths,
            flatten_empty_directories,
        )
        .expect("FileTreePreparedInput must satisfy store invariants");
        self.drag_session = None;
        self.drag_hover_generation = self.drag_hover_generation.wrapping_add(1);
        self.stop_drag_auto_scroll();
        self.source_key = source_key;
        self.current_input = input.clone();
        self.initial_expansion = initial_expansion;
        self.baseline_expanded_paths = initial_expanded_paths.to_vec();
        self.flatten_empty_directories = flatten_empty_directories;
        self.search_mode = search_mode;
        self.search_path_cache = None;
        if self
            .renaming_path
            .as_deref()
            .is_some_and(|path| self.store.canonical_path(path).is_none())
        {
            self.renaming_path = None;
            self.renaming_value.clear();
            self.remove_renaming_path_if_canceled = false;
        }

        let mut selected_paths = Vec::new();
        let mut selected_path_set = HashSet::new();
        for path in selected_candidates {
            let Some(path) = self.store.canonical_path(&path) else {
                continue;
            };
            if selected_path_set.insert(path.clone()) {
                selected_paths.push(path);
            }
        }
        let selection_changed = selected_path_set != self.selected_path_set;
        self.selected_paths = selected_paths;
        self.selected_path_set = selected_path_set;
        if selection_changed {
            self.selection_version += 1;
        }
        self.selection_anchor_path = anchor_candidate
            .as_deref()
            .and_then(|path| self.store.canonical_path(path));
        let search_focus_candidate = self.refresh_active_search_state();
        self.repair_focus(
            search_focus_candidate
                .as_deref()
                .or(focused_candidate.as_deref()),
        );

        FileTreeRuntimeUpdate {
            state_changed: true,
            selection_changed,
        }
    }

    pub fn apply_mutation(
        &mut self,
        operation: &FileTreeBatchOperation,
    ) -> Result<(FileTreeRuntimeUpdate, Option<FileTreeMutationEvent>), FileTreeMutationError> {
        self.apply_mutation_operations(std::slice::from_ref(operation), false)
    }

    pub fn apply_batch(
        &mut self,
        operations: &[FileTreeBatchOperation],
    ) -> Result<(FileTreeRuntimeUpdate, Option<FileTreeMutationEvent>), FileTreeMutationError> {
        self.apply_mutation_operations(operations, true)
    }

    fn apply_mutation_operations(
        &mut self,
        operations: &[FileTreeBatchOperation],
        batch: bool,
    ) -> Result<(FileTreeRuntimeUpdate, Option<FileTreeMutationEvent>), FileTreeMutationError> {
        let old_selected_set = self.selected_path_set.clone();
        let mut focused_candidate = self.focused_path.clone();
        let mut selected_candidates = self.selected_paths.clone();
        let mut anchor_candidate = self.selection_anchor_path.clone();
        let mut renaming_candidate = self.renaming_path.clone();
        let paths = self.current_input.paths();
        let mut transaction = MutationTransaction::new(&paths, self.store.expanded_paths());
        let mut semantic_events = Vec::new();
        let mut batch_visible_count_delta = 0;
        let mut batch_projection_changed = false;

        for operation in operations {
            let Some(applied) = transaction.apply_operation(operation)? else {
                continue;
            };
            focused_candidate = focused_candidate
                .as_deref()
                .and_then(|path| remap_path(path, &applied.mutation, true));
            selected_candidates = selected_candidates
                .iter()
                .filter_map(|path| remap_path(path, &applied.mutation, false))
                .collect();
            anchor_candidate = anchor_candidate
                .as_deref()
                .and_then(|path| remap_path(path, &applied.mutation, false));
            renaming_candidate = renaming_candidate
                .as_deref()
                .and_then(|path| remap_path(path, &applied.mutation, false));
            self.remap_search_collapsed_overrides(&applied.mutation);

            batch_visible_count_delta += applied.invalidation.visible_count_delta.unwrap_or(0);
            batch_projection_changed |= applied.invalidation.projection_changed;
            semantic_events.push(create_semantic_event(applied));
        }

        if semantic_events.is_empty() {
            let drag_aborted = self.drag_session.take().is_some();
            if drag_aborted {
                self.drag_hover_generation = self.drag_hover_generation.wrapping_add(1);
                self.stop_drag_auto_scroll();
            }
            let event = batch.then(|| {
                FileTreeMutationEvent::Batch(FileTreeBatchEvent {
                    invalidation: FileTreeMutationInvalidation {
                        canonical_changed: false,
                        projection_changed: false,
                        visible_count_delta: Some(0),
                    },
                    events: Vec::new(),
                })
            });
            let update = if drag_aborted {
                FileTreeRuntimeUpdate::state_changed()
            } else {
                FileTreeRuntimeUpdate::default()
            };
            return Ok((update, event));
        }

        self.drag_session = None;
        self.drag_hover_generation = self.drag_hover_generation.wrapping_add(1);
        self.stop_drag_auto_scroll();
        let input = transaction.prepare_input(&self.current_input)?;
        let expanded_paths = transaction.expanded_paths();
        let store = FileTreeStore::build_with_flattening(
            &input,
            FileTreeInitialExpansion::Closed,
            &expanded_paths,
            self.flatten_empty_directories,
        )?;
        self.current_input = input;
        self.store = store;
        self.has_imperative_input = true;
        self.search_path_cache = None;
        self.renaming_path = renaming_candidate;
        if self.renaming_path.is_none() {
            self.renaming_value.clear();
            self.remove_renaming_path_if_canceled = false;
        }
        self.restore_selection(selected_candidates, anchor_candidate);
        let refreshed_search_focus = self.refresh_active_search_state();
        let focus_candidate =
            self.focus_candidate_after_mutation(focused_candidate, refreshed_search_focus);
        self.repair_focus_after_mutation(focus_candidate.as_deref());
        let selection_changed = self.selected_path_set != old_selected_set;
        if selection_changed {
            self.selection_version += 1;
        }

        let final_invalidation = FileTreeMutationInvalidation {
            canonical_changed: true,
            projection_changed: batch_projection_changed,
            visible_count_delta: Some(batch_visible_count_delta),
        };
        let event = if batch {
            FileTreeMutationEvent::Batch(FileTreeBatchEvent {
                invalidation: final_invalidation,
                events: semantic_events,
            })
        } else {
            semantic_event_to_event(
                semantic_events
                    .pop()
                    .expect("one operation must produce one semantic event"),
            )
        };
        Ok((
            FileTreeRuntimeUpdate {
                state_changed: true,
                selection_changed,
            },
            Some(event),
        ))
    }

    pub fn reset_paths(
        &mut self,
        input: FileTreePreparedInput,
        initial_expanded_paths: Option<Vec<String>>,
        used_prepared_input: bool,
    ) -> Result<(FileTreeRuntimeUpdate, FileTreeMutationEvent), FileTreeMutationError> {
        let old_visible_paths = self.store.visible_paths(0..self.store.visible_count());
        let old_selected_set = self.selected_path_set.clone();
        let previous_path_count = list_count(&self.current_input.paths());
        let focused_candidate = self.focused_path.clone();
        let selected_candidates = self.selected_paths.clone();
        let anchor_candidate = self.selection_anchor_path.clone();

        let reset_expanded_paths = initial_expanded_paths
            .as_deref()
            .unwrap_or(&self.baseline_expanded_paths);
        let store = FileTreeStore::build_with_flattening(
            &input,
            self.initial_expansion,
            reset_expanded_paths,
            self.flatten_empty_directories,
        )?;
        self.drag_session = None;
        self.drag_hover_generation = self.drag_hover_generation.wrapping_add(1);
        self.stop_drag_auto_scroll();
        let new_visible_paths = store.visible_paths(0..store.visible_count());
        let path_count_after = input.paths().len();
        self.current_input = input;
        self.store = store;
        self.has_imperative_input = true;
        self.search_path_cache = None;
        if self
            .renaming_path
            .as_deref()
            .is_some_and(|path| self.store.canonical_path(path).is_none())
        {
            self.renaming_path = None;
            self.renaming_value.clear();
            self.remove_renaming_path_if_canceled = false;
        }
        self.restore_selection(selected_candidates, anchor_candidate);
        let refreshed_search_focus = self.refresh_active_search_state();
        let focus_candidate =
            self.focus_candidate_after_mutation(focused_candidate, refreshed_search_focus);
        self.repair_focus_after_mutation(focus_candidate.as_deref());
        let selection_changed = self.selected_path_set != old_selected_set;
        if selection_changed {
            self.selection_version += 1;
        }

        Ok((
            FileTreeRuntimeUpdate {
                state_changed: true,
                selection_changed,
            },
            FileTreeMutationEvent::Reset(FileTreeResetEvent {
                invalidation: FileTreeMutationInvalidation {
                    projection_changed: true,
                    ..mutation_invalidation(&old_visible_paths, &new_visible_paths)
                },
                path_count_after,
                path_count_before: previous_path_count,
                used_prepared_input,
            }),
        ))
    }

    fn restore_selection(
        &mut self,
        selected_candidates: Vec<String>,
        anchor_candidate: Option<String>,
    ) {
        self.selected_paths.clear();
        self.selected_path_set.clear();
        for path in selected_candidates {
            let Some(path) = self.store.canonical_path(&path) else {
                continue;
            };
            if self.selected_path_set.insert(path.clone()) {
                self.selected_paths.push(path);
            }
        }
        self.selection_anchor_path = anchor_candidate
            .as_deref()
            .and_then(|path| self.store.canonical_path(path));
    }

    fn focus_candidate_after_mutation(
        &self,
        mutation_candidate: Option<String>,
        refreshed_search_candidate: Option<String>,
    ) -> Option<String> {
        match self.search_value.as_deref() {
            Some(value) if !value.is_empty() => {
                let focused_is_filtered_visible = self.focused_path.as_ref().is_some_and(|path| {
                    self.search_visible_path_set
                        .as_ref()
                        .is_some_and(|visible_paths| visible_paths.contains(path))
                });
                if focused_is_filtered_visible {
                    self.focused_path.clone()
                } else {
                    refreshed_search_candidate
                }
            }
            Some("") => self.focused_path.clone(),
            None => mutation_candidate,
            Some(_) => unreachable!("nonempty search handled by the first match arm"),
        }
    }

    fn repair_focus_after_mutation(&mut self, candidate: Option<&str>) {
        let direct_or_ancestor = candidate.and_then(|path| {
            let mut candidate = path.to_owned();
            loop {
                if let Some(index) = self.nearest_visible_index_for_path(&candidate) {
                    return Some(index);
                }
                let bare = strip_one_terminal_slash(&candidate);
                let (parent, _) = bare.rsplit_once('/')?;
                candidate = format!("{parent}/");
            }
        });
        self.focused_index = direct_or_ancestor.or_else(|| (self.visible_count() > 0).then_some(0));
        self.focused_path = self
            .focused_index
            .and_then(|index| self.visible_path_at_index(index));
    }

    pub fn visible_count(&self) -> usize {
        self.search_visible_paths
            .as_ref()
            .map_or_else(|| self.store.visible_count(), Vec::len)
    }

    pub fn visible_rows(&self, range: Range<usize>) -> Vec<FileTreeVisibleRow> {
        let start = range.start.min(self.visible_count());
        let end = range.end.min(self.visible_count()).max(start);
        if let Some(indices) = self.search_visible_indices.as_ref() {
            return materialize_indices(
                &self.store,
                start,
                indices[start..end].iter().copied(),
                &self.selected_path_set,
                self.focused_path.as_deref(),
            );
        }
        materialize_range(
            &self.store,
            start..end,
            &self.selected_path_set,
            self.focused_path.as_deref(),
        )
    }

    pub fn sticky_rows(&mut self, scroll_top: f32, row_height: f32) -> Vec<FileTreeStickyRow> {
        if self.search_visible_indices.is_some()
            || self.store.visible_count() == 0
            || scroll_top <= 0.0
            || row_height <= 0.0
        {
            return Vec::new();
        }

        let visible_count = self.store.visible_count();
        let mut candidates = Vec::new();
        for slot_depth in 0..visible_count {
            let slot_top = scroll_top + slot_depth as f32 * row_height;
            let threshold = ((slot_top / row_height).floor() as usize).min(visible_count - 1);
            let candidate = self.sticky_candidate_at(threshold, slot_depth).or_else(|| {
                threshold
                    .checked_sub(1)
                    .and_then(|index| self.sticky_candidate_at(index, slot_depth))
            });
            let Some(candidate) = candidate else {
                break;
            };
            candidates.push(candidate);
        }

        layout_sticky_rows(candidates, visible_count, scroll_top, row_height)
    }

    fn sticky_candidate_at(
        &mut self,
        visible_index: usize,
        slot_depth: usize,
    ) -> Option<FileTreeStickyCandidate> {
        let context_row = materialize_range(
            &self.store,
            visible_index..visible_index.saturating_add(1),
            &self.selected_path_set,
            self.focused_path.as_deref(),
        )
        .pop()?;
        let candidate_path = context_row
            .ancestor_paths
            .get(slot_depth)
            .cloned()
            .or_else(|| {
                (context_row.ancestor_paths.len() == slot_depth
                    && context_row.kind == FileTreeItemKind::Directory
                    && context_row.is_expanded)
                    .then(|| context_row.path.clone())
            })?;
        let candidate_index = self.store.visible_index_for_path(&candidate_path)?;
        let subtree_end_index = self
            .store
            .visible_subtree_end_index_for_path(&candidate_path)?;
        let row = materialize_range(
            &self.store,
            candidate_index..candidate_index.saturating_add(1),
            &self.selected_path_set,
            self.focused_path.as_deref(),
        )
        .pop()?;
        Some(FileTreeStickyCandidate {
            row,
            subtree_end_index,
        })
    }

    pub fn sticky_inset_rows_for_visible_index(&self, visible_index: usize) -> usize {
        if self.search_visible_indices.is_some() {
            return 0;
        }
        materialize_range(
            &self.store,
            visible_index..visible_index.saturating_add(1),
            &self.selected_path_set,
            self.focused_path.as_deref(),
        )
        .first()
        .map_or(0, |row| row.ancestor_paths.len())
    }

    pub fn search_value(&self) -> &str {
        self.search_value.as_deref().unwrap_or("")
    }

    pub fn search_value_option(&self) -> Option<&str> {
        self.search_value.as_deref()
    }

    pub fn is_search_open(&self) -> bool {
        self.search_value.is_some()
    }

    pub fn search_matching_paths(&self) -> &[String] {
        &self.search_matching_paths
    }

    pub fn set_search(&mut self, value: Option<&str>) -> FileTreeRuntimeUpdate {
        let normalized_value = value.map(normalize_search_query);
        if self.search_value == normalized_value {
            return FileTreeRuntimeUpdate::default();
        }

        if self.search_value.is_none() && normalized_value.is_some() {
            self.search_previous_expanded_paths = Some(self.store.expanded_paths());
        }
        self.search_value = normalized_value;
        let focused_candidate = self.focused_path.clone();

        match self.search_value.as_deref() {
            None => {
                self.search_collapsed_overrides.clear();
                self.restore_search_expanded_paths(true);
                self.search_previous_expanded_paths = None;
                self.clear_search_projection();
                self.repair_focus(focused_candidate.as_deref());
            }
            Some("") => {
                self.search_collapsed_overrides.clear();
                self.restore_search_expanded_paths(false);
                self.clear_search_projection();
                self.repair_focus(focused_candidate.as_deref());
            }
            Some(_) => {
                let focus_candidate = self.refresh_active_search_state();
                self.repair_focus(focus_candidate.as_deref());
            }
        }

        FileTreeRuntimeUpdate::state_changed()
    }

    pub fn focus_next_search_match(&mut self) -> FileTreeRuntimeUpdate {
        self.focus_relative_search_match(1)
    }

    pub fn focus_previous_search_match(&mut self) -> FileTreeRuntimeUpdate {
        self.focus_relative_search_match(-1)
    }

    pub fn focused_path(&self) -> Option<&str> {
        self.focused_path.as_deref()
    }

    pub fn focused_index(&self) -> Option<usize> {
        self.focused_index
    }

    pub fn selected_paths(&self) -> &[String] {
        &self.selected_paths
    }

    pub fn prepare_drag_start(&mut self, path: &str) -> Option<(String, Vec<String>)> {
        let path = self.store.canonical_path(path)?;
        let dragged_paths = resolve_dragged_paths_for_start(&path, &self.selected_paths);
        Some((path, dragged_paths))
    }

    pub fn start_drag_session(
        &mut self,
        primary_path: String,
        dragged_paths: Vec<String>,
    ) -> FileTreeRuntimeUpdate {
        let mut update = FileTreeRuntimeUpdate::default();
        if !self.selected_path_set.contains(&primary_path) {
            update = update.combine(self.select_only_path(&primary_path));
        }
        update = update.combine(self.focus_path(&primary_path));
        self.stop_drag_auto_scroll();
        self.drag_hover_generation = self.drag_hover_generation.wrapping_add(1);
        self.drag_session = Some(FileTreeDragSession {
            dragged_paths,
            primary_path,
            target: None,
            hover_generation: self.drag_hover_generation,
        });
        update.combine(FileTreeRuntimeUpdate::state_changed())
    }

    pub fn drag_session(&self) -> Option<&FileTreeDragSession> {
        self.drag_session.as_ref()
    }

    pub fn begin_drag_target_capture(&mut self) -> Option<u64> {
        self.drag_session.as_ref()?;
        self.drag_target_capture_generation = self.drag_target_capture_generation.wrapping_add(1);
        Some(self.drag_target_capture_generation)
    }

    pub fn clear_unresolved_drag_target_capture(
        &mut self,
        generation: u64,
    ) -> FileTreeRuntimeUpdate {
        if self.drag_target_capture_generation != generation
            || self.drag_target_resolved_generation == generation
        {
            return FileTreeRuntimeUpdate::default();
        }
        self.set_drag_target(None)
    }

    pub fn set_drag_target(&mut self, target: Option<FileTreeDropTarget>) -> FileTreeRuntimeUpdate {
        let Some(session) = self.drag_session.as_mut() else {
            return FileTreeRuntimeUpdate::default();
        };
        self.drag_target_resolved_generation = self.drag_target_capture_generation;
        if session.target == target {
            return FileTreeRuntimeUpdate::default();
        }
        self.drag_hover_generation = self.drag_hover_generation.wrapping_add(1);
        session.target = target;
        session.hover_generation = self.drag_hover_generation;
        FileTreeRuntimeUpdate::state_changed()
    }

    pub fn cancel_drag(&mut self) -> FileTreeRuntimeUpdate {
        if self.drag_session.take().is_none() {
            return FileTreeRuntimeUpdate::default();
        }
        self.drag_hover_generation = self.drag_hover_generation.wrapping_add(1);
        self.stop_drag_auto_scroll();
        FileTreeRuntimeUpdate::state_changed()
    }

    pub fn arm_drag_auto_scroll(&mut self, pointer_y: f32, top: f32, bottom: f32) -> Option<u64> {
        self.drag_session.as_ref()?;
        self.drag_scroll_generation = self.drag_scroll_generation.wrapping_add(1);
        self.drag_scroll_point = Some((pointer_y, top, bottom));
        Some(self.drag_scroll_generation)
    }

    pub fn drag_auto_scroll_point(&self, generation: u64) -> Option<(f32, f32, f32)> {
        (self.drag_session.is_some() && self.drag_scroll_generation == generation)
            .then_some(self.drag_scroll_point)
            .flatten()
    }

    pub fn stop_drag_auto_scroll(&mut self) {
        self.drag_scroll_generation = self.drag_scroll_generation.wrapping_add(1);
        self.drag_scroll_point = None;
    }

    pub fn tick_drag_hover(
        &mut self,
        generation: u64,
        target: &FileTreeDropTarget,
    ) -> FileTreeRuntimeUpdate {
        let Some(session) = self.drag_session.as_ref() else {
            return FileTreeRuntimeUpdate::default();
        };
        if session.hover_generation != generation || session.target.as_ref() != Some(target) {
            return FileTreeRuntimeUpdate::default();
        }
        let Some(path) = target.directory_path.as_deref() else {
            return FileTreeRuntimeUpdate::default();
        };
        self.expand(path)
    }

    #[cfg(test)]
    pub fn selection_anchor_path(&self) -> Option<&str> {
        self.selection_anchor_path.as_deref()
    }

    #[cfg(test)]
    pub fn selection_version(&self) -> u64 {
        self.selection_version
    }

    pub fn is_selected(&self, path: &str) -> bool {
        self.selected_path_set.contains(path)
    }

    #[cfg(test)]
    pub fn is_directory(&mut self, path: &str) -> bool {
        self.store.is_directory_path(path)
    }

    pub fn is_expanded(&mut self, path: &str) -> bool {
        self.store.is_expanded_path(path)
    }

    pub fn item_descriptor(&mut self, path: &str) -> Option<(String, bool)> {
        let path = self.store.canonical_path(path)?;
        let is_directory = self.store.is_directory_path(&path);
        Some((path, is_directory))
    }

    pub fn renaming_path(&self) -> Option<&str> {
        self.renaming_path.as_deref()
    }

    pub fn renaming_value(&self) -> &str {
        &self.renaming_value
    }

    pub fn is_renaming(&self) -> bool {
        self.renaming_path.is_some()
    }

    pub fn renaming_generation(&self) -> u64 {
        self.renaming_generation
    }

    pub fn start_renaming(
        &mut self,
        path: &str,
        remove_if_canceled: bool,
    ) -> FileTreeRuntimeUpdate {
        let Some(canonical_path) = self.store.canonical_path(path) else {
            return FileTreeRuntimeUpdate::default();
        };

        let mut update = self.set_search(None);
        let mut expansion_changed = false;
        for ancestor in ancestor_directory_paths(&canonical_path) {
            expansion_changed |= self.store.set_expanded(&ancestor, true);
        }
        if expansion_changed {
            self.sync_search_visibility();
            update = update.combine(FileTreeRuntimeUpdate::state_changed());
        }
        update = update.combine(self.select_only_path(&canonical_path));
        update = update.combine(self.focus_path(&canonical_path));

        let next_value = rename_leaf_name(&canonical_path);
        if self.renaming_path.as_deref() != Some(canonical_path.as_str())
            || self.renaming_value != next_value
            || self.remove_renaming_path_if_canceled != remove_if_canceled
        {
            self.renaming_path = Some(canonical_path);
            self.renaming_value = next_value;
            self.remove_renaming_path_if_canceled = remove_if_canceled;
            self.renaming_generation = self.renaming_generation.wrapping_add(1);
            update = update.combine(FileTreeRuntimeUpdate::state_changed());
        }
        update
    }

    pub fn set_renaming_value(&mut self, value: &str) -> FileTreeRuntimeUpdate {
        if self.renaming_path.is_none() || self.renaming_value == value {
            return FileTreeRuntimeUpdate::default();
        }
        self.renaming_value.clear();
        self.renaming_value.push_str(value);
        FileTreeRuntimeUpdate::state_changed()
    }

    pub fn cancel_renaming(&mut self) -> (FileTreeRuntimeUpdate, Option<(String, bool)>) {
        let Some(path) = self.renaming_path.take() else {
            return (FileTreeRuntimeUpdate::default(), None);
        };
        let remove = self.remove_renaming_path_if_canceled;
        self.renaming_value.clear();
        self.remove_renaming_path_if_canceled = false;
        let mut update = FileTreeRuntimeUpdate::state_changed();
        if !remove {
            update = update.combine(self.focus_path(&path));
        }
        (
            update,
            remove.then_some((path.clone(), path.ends_with('/'))),
        )
    }

    pub fn prepare_rename_commit(
        &mut self,
    ) -> (FileTreeRuntimeUpdate, Option<FileTreeRenameCommit>) {
        let Some(source_canonical_path) = self.renaming_path.take() else {
            return (FileTreeRuntimeUpdate::default(), None);
        };
        let value = std::mem::take(&mut self.renaming_value);
        let remove_if_canceled = std::mem::take(&mut self.remove_renaming_path_if_canceled);
        let is_directory = source_canonical_path.ends_with('/');
        let mut update = FileTreeRuntimeUpdate::state_changed();

        if remove_if_canceled && value.trim().is_empty() {
            return (
                update,
                Some(FileTreeRenameCommit::Remove {
                    canonical_path: source_canonical_path,
                    is_directory,
                }),
            );
        }

        if self.store.canonical_path(&source_canonical_path).is_none() {
            return (
                update,
                Some(FileTreeRenameCommit::Error(
                    FileTreeRenameError::SourceDoesNotExist {
                        path: public_rename_path(&source_canonical_path),
                        is_directory,
                    },
                )),
            );
        }

        let (destination_path, destination_canonical_path) =
            match rename_destination(&source_canonical_path, &value) {
                Ok(destination) => destination,
                Err(error) => {
                    update = update.combine(self.focus_path(&source_canonical_path));
                    return (update, Some(FileTreeRenameCommit::Error(error)));
                }
            };
        let source_path = public_rename_path(&source_canonical_path);
        if source_path == destination_path {
            update = update.combine(self.focus_path(&source_canonical_path));
            return (update, Some(FileTreeRenameCommit::Unchanged));
        }
        if self.store.canonical_path(&destination_path).is_some() {
            update = update.combine(self.focus_path(&source_canonical_path));
            return (
                update,
                Some(FileTreeRenameCommit::Error(
                    FileTreeRenameError::DestinationAlreadyExists(destination_path),
                )),
            );
        }

        (
            update,
            Some(FileTreeRenameCommit::Move(FileTreeRenamePlan {
                source_canonical_path,
                destination_canonical_path,
                event: FileTreeRenameEvent {
                    source_path,
                    destination_path,
                    is_directory,
                },
            })),
        )
    }

    pub fn visible_index_for_path(&mut self, path: &str) -> Option<usize> {
        let path = self.store.canonical_path(path)?;
        self.exact_visible_index_for_path(&path)
    }

    fn ensure_search_path_cache(&mut self) {
        if self.search_path_cache.is_some() {
            return;
        }

        let listed_paths = self.current_input.paths();
        let listed_paths_lowercase = listed_paths
            .iter()
            .map(|path| path.to_lowercase())
            .collect();
        let mut directory_paths = self.store.directory_paths();
        directory_paths.sort_by(|left, right| left.encode_utf16().cmp(right.encode_utf16()));
        let directory_paths_lowercase = directory_paths
            .iter()
            .map(|path| path.to_lowercase())
            .collect();
        self.search_path_cache = Some(SearchPathCache {
            listed_paths,
            listed_paths_lowercase,
            directory_paths,
            directory_paths_lowercase,
        });
    }

    fn refresh_active_search_state(&mut self) -> Option<String> {
        let Some(search_value) = self
            .search_value
            .as_ref()
            .filter(|value| !value.is_empty())
            .cloned()
        else {
            self.clear_search_projection();
            return self.focused_path.clone();
        };

        self.prune_stale_search_collapsed_overrides();
        self.ensure_search_path_cache();
        let (matching_paths, matching_path_set) = {
            let cache = self
                .search_path_cache
                .as_ref()
                .expect("search cache must exist after initialization");
            let mut matching_paths = Vec::new();
            let mut matching_path_set = HashSet::new();
            for (path, lowercase_path) in cache
                .listed_paths
                .iter()
                .zip(cache.listed_paths_lowercase.iter())
            {
                if lowercase_path.contains(&search_value) && matching_path_set.insert(path.clone())
                {
                    matching_paths.push(path.clone());
                }
            }
            for (path, lowercase_path) in cache
                .directory_paths
                .iter()
                .zip(cache.directory_paths_lowercase.iter())
            {
                if lowercase_path.contains(&search_value) && matching_path_set.insert(path.clone())
                {
                    matching_paths.push(path.clone());
                }
            }
            (matching_paths, matching_path_set)
        };

        let focus_candidate = matching_paths
            .first()
            .cloned()
            .or_else(|| self.focused_path.clone());
        self.search_matching_paths = matching_paths;
        self.search_match_path_set = matching_path_set;
        let mut search_visible_path_set = (self.search_mode == FileTreeSearchMode::HideNonMatches
            && !self.search_matching_paths.is_empty())
        .then(HashSet::new);
        let mut expanded_paths = if self.search_mode == FileTreeSearchMode::ExpandMatches {
            self.search_previous_expanded_paths
                .clone()
                .unwrap_or_default()
                .into_iter()
                .collect::<HashSet<_>>()
        } else {
            HashSet::new()
        };

        for matching_path in &self.search_matching_paths {
            if let Some(visible_paths) = search_visible_path_set.as_mut() {
                visible_paths.insert(matching_path.clone());
            }
            if matching_path.ends_with('/') {
                expanded_paths.insert(matching_path.clone());
            }
            for ancestor_path in ancestor_directory_paths(matching_path) {
                expanded_paths.insert(ancestor_path.clone());
                if let Some(visible_paths) = search_visible_path_set.as_mut() {
                    visible_paths.insert(ancestor_path);
                }
            }
        }

        for collapsed_path in &self.search_collapsed_overrides {
            expanded_paths.remove(collapsed_path);
            if let Some(visible_paths) = search_visible_path_set.as_mut() {
                visible_paths.insert(collapsed_path.clone());
                visible_paths.extend(ancestor_directory_paths(collapsed_path));
            }
        }

        self.search_visible_path_set = search_visible_path_set;
        self.set_expanded_paths(&expanded_paths);
        self.sync_search_visibility();
        focus_candidate
    }

    fn clear_search_projection(&mut self) {
        self.search_match_path_set.clear();
        self.search_matching_paths.clear();
        self.search_visible_path_set = None;
        self.search_visible_paths = None;
        self.search_visible_indices = None;
        self.search_visible_index_by_path.clear();
    }

    fn restore_search_expanded_paths(&mut self, keep_selected_open: bool) {
        let mut expanded_paths = self
            .search_previous_expanded_paths
            .clone()
            .unwrap_or_default()
            .into_iter()
            .collect::<HashSet<_>>();
        if keep_selected_open {
            for selected_path in &self.selected_paths {
                expanded_paths.extend(ancestor_directory_paths(selected_path));
            }
        }
        self.set_expanded_paths(&expanded_paths);
    }

    fn set_expanded_paths(&mut self, expanded_paths: &HashSet<String>) {
        self.ensure_search_path_cache();
        let directory_paths = self
            .search_path_cache
            .as_ref()
            .expect("search cache must exist after initialization")
            .directory_paths
            .clone();
        for directory_path in directory_paths {
            self.store
                .set_expanded(&directory_path, expanded_paths.contains(&directory_path));
        }
    }

    fn sync_search_visibility(&mut self) {
        if self.search_mode != FileTreeSearchMode::HideNonMatches
            || self.search_value.as_deref().is_none_or(str::is_empty)
            || self.search_match_path_set.is_empty()
        {
            self.search_visible_paths = None;
            self.search_visible_indices = None;
            self.search_visible_index_by_path.clear();
            return;
        }

        let visible_path_set = self
            .search_visible_path_set
            .as_ref()
            .expect("filtered search must have a visible path set");
        let projection_paths = self.store.visible_paths(0..self.store.visible_count());
        let mut visible_paths = Vec::new();
        let mut visible_indices = Vec::new();
        let mut visible_index_by_path = HashMap::new();
        for (projection_index, path) in projection_paths.into_iter().enumerate() {
            if !visible_path_set.contains(&path) {
                continue;
            }
            visible_index_by_path.insert(path.clone(), visible_paths.len());
            visible_indices.push(projection_index);
            visible_paths.push(path);
        }
        self.search_visible_paths = Some(visible_paths);
        self.search_visible_indices = Some(visible_indices);
        self.search_visible_index_by_path = visible_index_by_path;
    }

    fn focus_relative_search_match(&mut self, direction: isize) -> FileTreeRuntimeUpdate {
        if self.search_matching_paths.is_empty() {
            return FileTreeRuntimeUpdate::default();
        }
        let current_index = self.focused_path.as_ref().and_then(|path| {
            self.search_matching_paths
                .iter()
                .position(|match_path| match_path == path)
        });
        let next_index = match current_index {
            Some(index) if direction < 0 => index.saturating_sub(1),
            Some(index) => (index + 1).min(self.search_matching_paths.len() - 1),
            None if direction < 0 => self.search_matching_paths.len() - 1,
            None => 0,
        };
        let path = self.search_matching_paths[next_index].clone();
        self.focus_path(&path)
    }

    fn visible_path_at_index(&self, index: usize) -> Option<String> {
        self.search_visible_paths
            .as_ref()
            .and_then(|paths| paths.get(index).cloned())
            .or_else(|| {
                self.search_visible_paths
                    .is_none()
                    .then(|| self.store.path_at_visible_index(index))
                    .flatten()
            })
    }

    fn visible_paths(&self, range: Range<usize>) -> Vec<String> {
        let start = range.start.min(self.visible_count());
        let end = range.end.min(self.visible_count()).max(start);
        self.search_visible_paths.as_ref().map_or_else(
            || self.store.visible_paths(start..end),
            |paths| paths[start..end].to_vec(),
        )
    }

    fn exact_visible_index_for_path(&mut self, path: &str) -> Option<usize> {
        if self.search_visible_paths.is_some() {
            self.search_visible_index_by_path.get(path).copied()
        } else {
            self.store.visible_index_for_path(path)
        }
    }

    fn nearest_visible_index_for_path(&mut self, path: &str) -> Option<usize> {
        let mut candidate = self.store.canonical_path(path)?;
        loop {
            if let Some(index) = self.exact_visible_index_for_path(&candidate) {
                return Some(index);
            }
            let Some(parent) = self.store.parent_path(&candidate) else {
                return (self.visible_count() > 0).then_some(0);
            };
            candidate = parent;
        }
    }

    fn prune_stale_search_collapsed_overrides(&mut self) {
        let overrides = std::mem::take(&mut self.search_collapsed_overrides);
        for path in overrides {
            let Some(canonical_path) = self.store.canonical_path(&path) else {
                continue;
            };
            if self.store.is_directory_path(&canonical_path) {
                self.search_collapsed_overrides.insert(canonical_path);
            }
        }
    }

    fn remap_search_collapsed_overrides(&mut self, mutation: &PathMutation) {
        self.search_collapsed_overrides = self
            .search_collapsed_overrides
            .iter()
            .filter_map(|path| remap_path(path, mutation, false))
            .collect();
    }

    pub fn focus_visible_index(&mut self, index: usize) -> FileTreeRuntimeUpdate {
        self.set_focused_index(index)
    }

    pub fn focus_first_item(&mut self) -> FileTreeRuntimeUpdate {
        self.set_focused_index(0)
    }

    pub fn focus_last_item(&mut self) -> FileTreeRuntimeUpdate {
        let Some(index) = self.visible_count().checked_sub(1) else {
            return FileTreeRuntimeUpdate::default();
        };
        self.set_focused_index(index)
    }

    pub fn focus_next_item(&mut self) -> FileTreeRuntimeUpdate {
        self.move_focus(1)
    }

    pub fn focus_previous_item(&mut self) -> FileTreeRuntimeUpdate {
        self.move_focus(-1)
    }

    pub fn focus_parent_item(&mut self) -> FileTreeRuntimeUpdate {
        let Some(focused_path) = self.focused_path.clone() else {
            return FileTreeRuntimeUpdate::default();
        };
        let Some(parent_path) = self.store.parent_path(&focused_path) else {
            return FileTreeRuntimeUpdate::default();
        };
        self.focus_path(&parent_path)
    }

    pub fn focus_path(&mut self, path: &str) -> FileTreeRuntimeUpdate {
        let Some(path) = self.store.canonical_path(path) else {
            return FileTreeRuntimeUpdate::default();
        };
        let Some(index) = self.nearest_visible_index_for_path(&path) else {
            return FileTreeRuntimeUpdate::default();
        };
        self.set_focused_index(index)
    }

    #[cfg(test)]
    pub fn toggle(&mut self, path: &str) -> bool {
        self.toggle_update(path).state_changed
    }

    pub fn toggle_update(&mut self, path: &str) -> FileTreeRuntimeUpdate {
        if !self.store.is_directory_path(path) {
            return FileTreeRuntimeUpdate::default();
        }
        if self.store.is_expanded_path(path) {
            self.collapse(path)
        } else {
            self.expand(path)
        }
    }

    pub fn expand(&mut self, path: &str) -> FileTreeRuntimeUpdate {
        let Some(path) = self.store.canonical_path(path) else {
            return FileTreeRuntimeUpdate::default();
        };
        if !self.store.is_directory_path(&path) {
            return FileTreeRuntimeUpdate::default();
        }

        if self
            .search_value
            .as_deref()
            .is_some_and(|value| !value.is_empty())
        {
            self.search_collapsed_overrides.remove(&path);
        }
        let focused_candidate = self.focused_path.clone();
        let mut expansion_paths = vec![path.clone()];
        let mut current_path = path;
        while let Some(parent_path) = self.store.parent_path(&current_path) {
            expansion_paths.push(parent_path.clone());
            current_path = parent_path;
        }
        expansion_paths.reverse();

        let mut state_changed = false;
        for path in expansion_paths {
            state_changed |= self.store.set_expanded(&path, true);
        }
        if !state_changed {
            return FileTreeRuntimeUpdate::default();
        }
        self.sync_search_visibility();
        self.repair_focus(focused_candidate.as_deref());
        FileTreeRuntimeUpdate::state_changed()
    }

    pub fn collapse(&mut self, path: &str) -> FileTreeRuntimeUpdate {
        let Some(path) = self.store.canonical_path(path) else {
            return FileTreeRuntimeUpdate::default();
        };
        if self
            .search_value
            .as_deref()
            .is_some_and(|value| !value.is_empty())
            && self.store.is_expanded_path(&path)
        {
            self.search_collapsed_overrides.insert(path.clone());
        }
        self.set_expanded(&path, false)
    }

    pub fn focus_right(&mut self) -> FileTreeRuntimeUpdate {
        let Some(path) = self.focused_path.clone() else {
            return FileTreeRuntimeUpdate::default();
        };
        if self.store.is_directory_path(&path) && !self.store.is_expanded_path(&path) {
            self.expand(&path)
        } else {
            self.focus_next_item()
        }
    }

    pub fn focus_left(&mut self) -> FileTreeRuntimeUpdate {
        let Some(path) = self.focused_path.clone() else {
            return FileTreeRuntimeUpdate::default();
        };
        if self.store.is_directory_path(&path) && self.store.is_expanded_path(&path) {
            self.collapse(&path)
        } else {
            self.focus_parent_item()
        }
    }

    pub fn activate_focused(&mut self) -> FileTreeRuntimeUpdate {
        let Some(path) = self.focused_path.clone() else {
            return FileTreeRuntimeUpdate::default();
        };
        let update = self.select_only_path(&path);
        if self.store.is_directory_path(&path) {
            update.combine(self.toggle_update(&path))
        } else {
            update
        }
    }

    pub fn apply_row_click(
        &mut self,
        path: &str,
        plan: FileTreeRowClickPlan,
    ) -> FileTreeRuntimeUpdate {
        let Some(path) = self.store.canonical_path(path) else {
            return FileTreeRuntimeUpdate::default();
        };
        let Some(index) = self.exact_visible_index_for_path(&path) else {
            return FileTreeRuntimeUpdate::default();
        };
        let mut update = self.set_focused_index(index);
        update = update.combine(match plan.selection {
            FileTreeSelectionDirective::Single => self.select_only_path(&path),
            FileTreeSelectionDirective::Toggle => self.toggle_path_selection(&path),
            FileTreeSelectionDirective::Range { additive } => {
                self.select_path_range(&path, additive)
            }
        });
        if plan.toggle_directory {
            update = update.combine(self.toggle_update(&path));
        }
        update
    }

    pub fn apply_context_click(&mut self, path: &str) -> FileTreeRuntimeUpdate {
        let Some(path) = self.store.canonical_path(path) else {
            return FileTreeRuntimeUpdate::default();
        };
        let Some(index) = self.exact_visible_index_for_path(&path) else {
            return FileTreeRuntimeUpdate::default();
        };
        self.set_focused_index(index)
    }

    pub fn select_all_visible_paths(&mut self) -> FileTreeRuntimeUpdate {
        let paths = self.visible_paths(0..self.visible_count());
        let anchor = self
            .focused_path
            .clone()
            .or_else(|| self.selection_anchor_path.clone());
        self.apply_selection(paths, anchor)
    }

    pub fn select_only_path(&mut self, path: &str) -> FileTreeRuntimeUpdate {
        let Some(path) = self.store.canonical_path(path) else {
            return FileTreeRuntimeUpdate::default();
        };
        self.apply_selection(vec![path.clone()], Some(path))
    }

    pub fn select_path(&mut self, path: &str) -> FileTreeRuntimeUpdate {
        let Some(path) = self.store.canonical_path(path) else {
            return FileTreeRuntimeUpdate::default();
        };
        if self.selected_path_set.contains(&path) {
            return FileTreeRuntimeUpdate::default();
        }
        let mut selected_paths = self.selected_paths.clone();
        selected_paths.push(path);
        self.apply_selection(selected_paths, self.selection_anchor_path.clone())
    }

    pub fn deselect_path(&mut self, path: &str) -> FileTreeRuntimeUpdate {
        let Some(path) = self.store.canonical_path(path) else {
            return FileTreeRuntimeUpdate::default();
        };
        if !self.selected_path_set.contains(&path) {
            return FileTreeRuntimeUpdate::default();
        }
        let selected_paths = self
            .selected_paths
            .iter()
            .filter(|selected_path| **selected_path != path)
            .cloned()
            .collect();
        self.apply_selection(selected_paths, self.selection_anchor_path.clone())
    }

    pub fn toggle_focused_selection(&mut self) -> FileTreeRuntimeUpdate {
        let Some(path) = self.focused_path.clone() else {
            return FileTreeRuntimeUpdate::default();
        };
        self.toggle_path_selection(&path)
    }

    pub fn toggle_path_selection(&mut self, path: &str) -> FileTreeRuntimeUpdate {
        let Some(path) = self.store.canonical_path(path) else {
            return FileTreeRuntimeUpdate::default();
        };
        let update = if self.selected_path_set.contains(&path) {
            self.deselect_path(&path)
        } else {
            self.select_path(&path)
        };
        update.combine(self.apply_selection(self.selected_paths.clone(), Some(path)))
    }

    pub fn select_path_range(
        &mut self,
        path: &str,
        union_selection: bool,
    ) -> FileTreeRuntimeUpdate {
        let Some(path) = self.store.canonical_path(path) else {
            return FileTreeRuntimeUpdate::default();
        };
        let anchor_path = self.selection_anchor_path.clone();
        let anchor_index = match anchor_path.as_deref() {
            Some(anchor) => self.exact_visible_index_for_path(anchor),
            None => None,
        };
        let target_index = self.exact_visible_index_for_path(&path);
        let (Some(anchor_index), Some(target_index)) = (anchor_index, target_index) else {
            let mut selected_paths = if union_selection {
                self.selected_paths.clone()
            } else {
                Vec::new()
            };
            selected_paths.push(path.clone());
            return self.apply_selection(selected_paths, Some(path));
        };

        let start = anchor_index.min(target_index);
        let end = anchor_index.max(target_index) + 1;
        let range_paths = self.visible_paths(start..end);
        let selected_paths = if union_selection {
            let mut selected_paths = self.selected_paths.clone();
            selected_paths.extend(range_paths);
            selected_paths
        } else {
            range_paths
        };
        self.apply_selection(selected_paths, anchor_path)
    }

    pub fn extend_selection_from_focused(&mut self, offset: isize) -> FileTreeRuntimeUpdate {
        if !matches!(offset, -1 | 1) {
            return FileTreeRuntimeUpdate::default();
        }
        let Some(focused_index) = self.focused_index else {
            return FileTreeRuntimeUpdate::default();
        };
        let next_index = if offset < 0 {
            focused_index.saturating_sub(1)
        } else {
            (focused_index + 1).min(self.visible_count().saturating_sub(1))
        };
        if next_index == focused_index {
            return FileTreeRuntimeUpdate::default();
        }
        let Some(current_path) = self.visible_path_at_index(focused_index) else {
            return FileTreeRuntimeUpdate::default();
        };
        let Some(next_path) = self.visible_path_at_index(next_index) else {
            return FileTreeRuntimeUpdate::default();
        };

        let mut selected_paths = self.selected_paths.clone();
        if self.selected_path_set.contains(&current_path)
            && self.selected_path_set.contains(&next_path)
        {
            selected_paths.retain(|path| path != &current_path);
        } else if !self.selected_path_set.contains(&next_path) {
            selected_paths.push(next_path);
        }
        let anchor = self.selection_anchor_path.clone().or(Some(current_path));
        self.apply_selection(selected_paths, anchor)
            .combine(self.set_focused_index(next_index))
    }

    fn move_focus(&mut self, offset: isize) -> FileTreeRuntimeUpdate {
        if self.visible_count() == 0 {
            return FileTreeRuntimeUpdate::default();
        }
        let current_index = self.focused_index.unwrap_or(0);
        let next_index = if offset < 0 {
            current_index.saturating_sub(offset.unsigned_abs())
        } else {
            current_index
                .saturating_add(offset as usize)
                .min(self.visible_count() - 1)
        };
        if next_index == current_index && self.focused_index.is_some() {
            return FileTreeRuntimeUpdate::default();
        }
        self.set_focused_index(next_index)
    }

    fn set_focused_index(&mut self, index: usize) -> FileTreeRuntimeUpdate {
        let Some(path) = self.visible_path_at_index(index) else {
            return FileTreeRuntimeUpdate::default();
        };
        if self.focused_index == Some(index) && self.focused_path.as_deref() == Some(path.as_str())
        {
            return FileTreeRuntimeUpdate::default();
        }
        self.focused_index = Some(index);
        self.focused_path = Some(path);
        FileTreeRuntimeUpdate::state_changed()
    }

    fn set_expanded(&mut self, path: &str, expanded: bool) -> FileTreeRuntimeUpdate {
        let focused_candidate = self.focused_path.clone();
        if !self.store.set_expanded(path, expanded) {
            return FileTreeRuntimeUpdate::default();
        }
        self.sync_search_visibility();
        self.repair_focus(focused_candidate.as_deref());
        FileTreeRuntimeUpdate::state_changed()
    }

    fn repair_focus(&mut self, candidate: Option<&str>) {
        self.focused_index = candidate
            .and_then(|path| self.nearest_visible_index_for_path(path))
            .or_else(|| (self.visible_count() > 0).then_some(0));
        self.focused_path = self
            .focused_index
            .and_then(|index| self.visible_path_at_index(index));
    }

    fn apply_selection(
        &mut self,
        next_selected_paths: Vec<String>,
        next_anchor_path: Option<String>,
    ) -> FileTreeRuntimeUpdate {
        let mut unique_paths = Vec::with_capacity(next_selected_paths.len());
        let mut unique_path_set = HashSet::with_capacity(next_selected_paths.len());
        for path in next_selected_paths {
            if unique_path_set.insert(path.clone()) {
                unique_paths.push(path);
            }
        }

        let selection_changed = unique_path_set != self.selected_path_set;
        let anchor_changed = next_anchor_path != self.selection_anchor_path;
        if !selection_changed && !anchor_changed {
            return FileTreeRuntimeUpdate::default();
        }

        self.selected_paths = unique_paths;
        self.selected_path_set = unique_path_set;
        self.selection_anchor_path = next_anchor_path;
        if selection_changed {
            self.selection_version += 1;
            FileTreeRuntimeUpdate::selection_changed()
        } else {
            FileTreeRuntimeUpdate::state_changed()
        }
    }

    #[cfg(test)]
    pub fn store(&self) -> &FileTreeStore {
        &self.store
    }
}

fn ancestor_directory_paths(path: &str) -> Vec<String> {
    let bare_path = strip_one_terminal_slash(path);
    let mut ancestors = Vec::new();
    for (index, _) in bare_path.match_indices('/') {
        ancestors.push(format!("{}/", &bare_path[..index]));
    }
    ancestors
}

fn mutation_invalidation(before: &[String], after: &[String]) -> FileTreeMutationInvalidation {
    FileTreeMutationInvalidation {
        canonical_changed: true,
        projection_changed: before != after,
        visible_count_delta: Some(after.len() as isize - before.len() as isize),
    }
}

fn create_semantic_event(applied: AppliedOperation) -> FileTreeMutationSemanticEvent {
    let invalidation = applied.invalidation;
    match applied.semantic {
        PendingSemanticEvent::Add { path } => {
            FileTreeMutationSemanticEvent::Add(FileTreeAddEvent { invalidation, path })
        }
        PendingSemanticEvent::Remove { path, recursive } => {
            FileTreeMutationSemanticEvent::Remove(FileTreeRemoveEvent {
                invalidation,
                path,
                recursive,
            })
        }
        PendingSemanticEvent::Move { from, to } => {
            FileTreeMutationSemanticEvent::Move(FileTreeMoveEvent {
                invalidation,
                from,
                to,
            })
        }
    }
}

fn semantic_event_to_event(event: FileTreeMutationSemanticEvent) -> FileTreeMutationEvent {
    match event {
        FileTreeMutationSemanticEvent::Add(event) => FileTreeMutationEvent::Add(event),
        FileTreeMutationSemanticEvent::Remove(event) => FileTreeMutationEvent::Remove(event),
        FileTreeMutationSemanticEvent::Move(event) => FileTreeMutationEvent::Move(event),
        FileTreeMutationSemanticEvent::Reset(event) => FileTreeMutationEvent::Reset(event),
    }
}

fn source_key(
    input: &FileTreePreparedInput,
    initial_expansion: FileTreeInitialExpansion,
    initial_expanded_paths: &[String],
    flatten_empty_directories: bool,
) -> SourceKey {
    let mut hasher = DefaultHasher::new();
    for path in initial_expanded_paths {
        strip_one_terminal_slash(path).hash(&mut hasher);
    }

    SourceKey {
        input_fingerprint: input.fingerprint(),
        initial_expansion,
        initial_expanded_paths_fingerprint: hasher.finish(),
        flatten_empty_directories,
    }
}
