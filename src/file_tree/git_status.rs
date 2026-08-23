use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

/// Git status attached to a file-tree path.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FileTreeGitStatus {
    Added,
    Deleted,
    Ignored,
    Modified,
    Renamed,
    Untracked,
}

impl FileTreeGitStatus {
    /// Return the stable Pierre status name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Added => "added",
            Self::Deleted => "deleted",
            Self::Ignored => "ignored",
            Self::Modified => "modified",
            Self::Renamed => "renamed",
            Self::Untracked => "untracked",
        }
    }
}

/// One Git status value keyed by a file or directory path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileTreeGitStatusEntry {
    pub path: String,
    pub status: FileTreeGitStatus,
}

impl FileTreeGitStatusEntry {
    pub fn new(path: impl Into<String>, status: FileTreeGitStatus) -> Self {
        Self {
            path: path.into(),
            status,
        }
    }
}

/// An incremental Git status update.
///
/// Removals run before sets, as in Pierre. A directory path must end in `/`.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FileTreeGitStatusPatch {
    pub remove: Vec<String>,
    pub set: Vec<FileTreeGitStatusEntry>,
}

impl FileTreeGitStatusPatch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_remove(mut self, paths: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.remove.extend(paths.into_iter().map(Into::into));
        self
    }

    pub fn with_set(mut self, entries: impl IntoIterator<Item = FileTreeGitStatusEntry>) -> Self {
        self.set.extend(entries);
        self
    }

    pub fn is_empty(&self) -> bool {
        self.remove.is_empty() && self.set.is_empty()
    }
}

/// Resolved Git status indexes keyed by canonical file-tree paths.
///
/// Callers normally keep this state behind `Arc`. Resolver and patch no-ops
/// return the same allocation so render caches can use pointer identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileTreeGitStatusState {
    change_count_by_directory_path: BTreeMap<String, usize>,
    directories_with_changes: BTreeSet<String>,
    ignored_directory_paths: BTreeSet<String>,
    signature: String,
    status_by_path: BTreeMap<String, FileTreeGitStatus>,
}

impl FileTreeGitStatusState {
    /// Return the explicit status for a canonicalized path.
    pub fn status_for_path(&self, path: &str) -> Option<FileTreeGitStatus> {
        let path = normalize_git_status_path(path)?;
        self.status_by_path.get(&path.canonical_path).copied()
    }

    /// Return an explicit status first, then inherited ignored status.
    pub fn resolved_status_for_path(&self, path: &str) -> Option<FileTreeGitStatus> {
        let path = normalize_git_status_path(path)?;
        if let Some(status) = self.status_by_path.get(&path.canonical_path) {
            return Some(*status);
        }

        ancestor_directory_paths(&path.canonical_path)
            .into_iter()
            .rev()
            .any(|ancestor| self.ignored_directory_paths.contains(ancestor))
            .then_some(FileTreeGitStatus::Ignored)
    }

    /// Return true when a directory has a changed descendant.
    pub fn directory_has_changes(&self, path: &str) -> bool {
        self.directory_change_count(path) > 0
    }

    /// Return the number of explicitly marked descendants of a directory.
    pub fn directory_change_count(&self, path: &str) -> usize {
        let Some(path) = normalize_git_status_path(path) else {
            return 0;
        };
        if !path.is_directory {
            return 0;
        }

        self.change_count_by_directory_path
            .get(&path.canonical_path)
            .copied()
            .unwrap_or(0)
    }

    /// Return true when this exact directory supplies ignored inheritance.
    pub fn is_ignored_directory(&self, path: &str) -> bool {
        let Some(path) = normalize_git_status_path(path) else {
            return false;
        };
        path.is_directory && self.ignored_directory_paths.contains(&path.canonical_path)
    }

    pub fn len(&self) -> usize {
        self.status_by_path.len()
    }

    pub fn is_empty(&self) -> bool {
        self.status_by_path.is_empty()
    }
}

/// Resolve a full Git status input.
///
/// Invalid paths are ignored. Empty or wholly invalid input resolves to `None`.
/// When the raw input signature is unchanged, this returns `previous`.
pub fn resolve_file_tree_git_status_state(
    entries: &[FileTreeGitStatusEntry],
    previous: Option<&Arc<FileTreeGitStatusState>>,
) -> Option<Arc<FileTreeGitStatusState>> {
    if entries.is_empty() {
        return None;
    }

    let signature = input_signature(entries);
    if previous.is_some_and(|state| state.signature == signature) {
        return previous.cloned();
    }

    let mut state = FileTreeGitStatusState {
        change_count_by_directory_path: BTreeMap::new(),
        directories_with_changes: BTreeSet::new(),
        ignored_directory_paths: BTreeSet::new(),
        signature,
        status_by_path: BTreeMap::new(),
    };

    for entry in entries {
        let Some(path) = normalize_git_status_path(&entry.path) else {
            continue;
        };
        set_git_status_path(&mut state, &path, entry.status);
    }

    (!state.is_empty()).then(|| Arc::new(state))
}

/// Apply an incremental Git status patch.
///
/// Invalid paths are ignored. A no-op returns the same `Arc`; removing the last
/// status returns `None`.
pub fn apply_file_tree_git_status_patch(
    previous: Option<&Arc<FileTreeGitStatusState>>,
    patch: &FileTreeGitStatusPatch,
) -> Option<Arc<FileTreeGitStatusState>> {
    if patch.is_empty() {
        return previous.cloned();
    }

    let mut state = previous
        .map(|state| state.as_ref().clone())
        .unwrap_or_else(|| FileTreeGitStatusState {
            change_count_by_directory_path: BTreeMap::new(),
            directories_with_changes: BTreeSet::new(),
            ignored_directory_paths: BTreeSet::new(),
            signature: String::new(),
            status_by_path: BTreeMap::new(),
        });
    let mut changed = false;

    for path in &patch.remove {
        let Some(path) = normalize_git_status_path(path) else {
            continue;
        };
        changed |= remove_git_status_path(&mut state, &path.canonical_path);
    }

    for entry in &patch.set {
        let Some(path) = normalize_git_status_path(&entry.path) else {
            continue;
        };
        changed |= set_git_status_path(&mut state, &path, entry.status);
    }

    if !changed {
        return previous.cloned();
    }
    if state.is_empty() {
        return None;
    }

    state.signature = state_signature(&state.status_by_path);
    Some(Arc::new(state))
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NormalizedGitStatusPath {
    canonical_path: String,
    is_directory: bool,
}

fn normalize_git_status_path(path: &str) -> Option<NormalizedGitStatusPath> {
    let is_directory = path.ends_with('/');
    let normalized = path
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .join("/");
    if normalized.is_empty() {
        return None;
    }

    let canonical_path = if is_directory {
        format!("{normalized}/")
    } else {
        normalized
    };
    Some(NormalizedGitStatusPath {
        canonical_path,
        is_directory,
    })
}

fn ancestor_directory_paths(path: &str) -> Vec<&str> {
    let path = path.strip_suffix('/').unwrap_or(path);
    path.match_indices('/')
        .map(|(index, _)| &path[..=index])
        .collect()
}

fn remove_git_status_path(state: &mut FileTreeGitStatusState, canonical_path: &str) -> bool {
    let Some(previous_status) = state.status_by_path.remove(canonical_path) else {
        return false;
    };

    if previous_status == FileTreeGitStatus::Ignored && canonical_path.ends_with('/') {
        state.ignored_directory_paths.remove(canonical_path);
    }
    decrement_ancestor_change_counts(state, canonical_path);
    true
}

fn set_git_status_path(
    state: &mut FileTreeGitStatusState,
    path: &NormalizedGitStatusPath,
    status: FileTreeGitStatus,
) -> bool {
    let previous_status = state.status_by_path.get(&path.canonical_path).copied();
    if previous_status == Some(status) {
        return false;
    }

    if previous_status.is_none() {
        increment_ancestor_change_counts(state, &path.canonical_path);
    }
    state
        .status_by_path
        .insert(path.canonical_path.clone(), status);

    if status == FileTreeGitStatus::Ignored && path.is_directory {
        state
            .ignored_directory_paths
            .insert(path.canonical_path.clone());
    } else if path.is_directory {
        state.ignored_directory_paths.remove(&path.canonical_path);
    }
    true
}

fn increment_ancestor_change_counts(state: &mut FileTreeGitStatusState, path: &str) {
    for ancestor in ancestor_directory_paths(path) {
        let count = state
            .change_count_by_directory_path
            .entry(ancestor.to_owned())
            .or_insert(0);
        *count += 1;
        state.directories_with_changes.insert(ancestor.to_owned());
    }
}

fn decrement_ancestor_change_counts(state: &mut FileTreeGitStatusState, path: &str) {
    for ancestor in ancestor_directory_paths(path) {
        let next_count = state
            .change_count_by_directory_path
            .get(ancestor)
            .copied()
            .unwrap_or(0)
            .saturating_sub(1);
        if next_count == 0 {
            state.change_count_by_directory_path.remove(ancestor);
            state.directories_with_changes.remove(ancestor);
        } else {
            state
                .change_count_by_directory_path
                .insert(ancestor.to_owned(), next_count);
        }
    }
}

fn input_signature(entries: &[FileTreeGitStatusEntry]) -> String {
    let mut signature = entries.len().to_string();
    for entry in entries {
        signature.push('\0');
        signature.push_str(&entry.path);
        signature.push('\0');
        signature.push_str(entry.status.as_str());
    }
    signature
}

fn state_signature(status_by_path: &BTreeMap<String, FileTreeGitStatus>) -> String {
    let mut signature = status_by_path.len().to_string();
    for (path, status) in status_by_path {
        signature.push('\0');
        signature.push_str(path);
        signature.push('\0');
        signature.push_str(status.as_str());
    }
    signature
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(path: &str, status: FileTreeGitStatus) -> FileTreeGitStatusEntry {
        FileTreeGitStatusEntry::new(path, status)
    }

    #[test]
    fn later_directory_status_clears_stale_ignored_inheritance() {
        let state = resolve_file_tree_git_status_state(
            &[
                entry("src/", FileTreeGitStatus::Ignored),
                entry("src/", FileTreeGitStatus::Modified),
            ],
            None,
        )
        .expect("expected Git status state");

        assert_eq!(
            state.status_for_path("src/"),
            Some(FileTreeGitStatus::Modified)
        );
        assert!(!state.is_ignored_directory("src/"));
        assert_eq!(state.resolved_status_for_path("src/file.rs"), None);
    }

    #[test]
    fn incremental_patch_updates_only_changed_status_entries() {
        let initial = resolve_file_tree_git_status_state(
            &[
                entry("src/index.ts", FileTreeGitStatus::Modified),
                entry("src/components/Button.tsx", FileTreeGitStatus::Added),
            ],
            None,
        )
        .expect("expected initial Git status state");

        let no_op = apply_file_tree_git_status_patch(
            Some(&initial),
            &FileTreeGitStatusPatch::new()
                .with_set([entry("src/index.ts", FileTreeGitStatus::Modified)]),
        )
        .expect("expected unchanged Git status state");
        assert!(Arc::ptr_eq(&initial, &no_op));

        let one_removed = apply_file_tree_git_status_patch(
            Some(&initial),
            &FileTreeGitStatusPatch::new().with_remove(["src/index.ts"]),
        )
        .expect("expected remaining Git status state");
        assert_eq!(one_removed.status_for_path("src/index.ts"), None);
        assert_eq!(
            one_removed.status_for_path("src/components/Button.tsx"),
            Some(FileTreeGitStatus::Added)
        );
        assert!(one_removed.directory_has_changes("src/"));
        assert!(one_removed.directory_has_changes("src/components/"));
        assert_eq!(one_removed.directory_change_count("src/"), 1);

        let empty = apply_file_tree_git_status_patch(
            Some(&one_removed),
            &FileTreeGitStatusPatch::new().with_remove(["src/components/Button.tsx"]),
        );
        assert!(empty.is_none());
    }

    #[test]
    fn incremental_patch_updates_ignored_directory_inheritance() {
        let initial =
            resolve_file_tree_git_status_state(&[entry("src/", FileTreeGitStatus::Ignored)], None)
                .expect("expected initial Git status state");
        assert!(initial.is_ignored_directory("src/"));
        assert_eq!(
            initial.resolved_status_for_path("src/components/Card.tsx"),
            Some(FileTreeGitStatus::Ignored)
        );

        let modified = apply_file_tree_git_status_patch(
            Some(&initial),
            &FileTreeGitStatusPatch::new().with_set([entry("src/", FileTreeGitStatus::Modified)]),
        )
        .expect("expected modified Git status state");
        assert_eq!(
            modified.status_for_path("src/"),
            Some(FileTreeGitStatus::Modified)
        );
        assert!(!modified.is_ignored_directory("src/"));
        assert!(!modified.directory_has_changes("src/"));

        let empty = apply_file_tree_git_status_patch(
            Some(&modified),
            &FileTreeGitStatusPatch::new().with_remove(["src/"]),
        );
        assert!(empty.is_none());
    }

    #[test]
    fn paths_are_normalized_and_preserve_directory_markers() {
        let state = resolve_file_tree_git_status_state(
            &[
                entry("/src//components///", FileTreeGitStatus::Ignored),
                entry("//src//index.rs", FileTreeGitStatus::Modified),
                entry("///", FileTreeGitStatus::Added),
            ],
            None,
        )
        .expect("expected normalized Git status state");

        assert_eq!(state.len(), 2);
        assert!(state.is_ignored_directory("src/components/"));
        assert_eq!(
            state.status_for_path("src//index.rs"),
            Some(FileTreeGitStatus::Modified)
        );
        assert_eq!(state.status_for_path("src/components"), None);
        assert_eq!(state.directory_change_count("src/"), 2);
    }

    #[test]
    fn exact_child_status_wins_over_inherited_ignored() {
        let state = resolve_file_tree_git_status_state(
            &[
                entry("src/", FileTreeGitStatus::Ignored),
                entry("src/kept.rs", FileTreeGitStatus::Modified),
            ],
            None,
        )
        .expect("expected Git status state");

        assert_eq!(
            state.resolved_status_for_path("src/kept.rs"),
            Some(FileTreeGitStatus::Modified)
        );
        assert_eq!(
            state.resolved_status_for_path("src/other.rs"),
            Some(FileTreeGitStatus::Ignored)
        );
    }

    #[test]
    fn unchanged_full_input_reuses_previous_state() {
        let entries = [entry("src/index.rs", FileTreeGitStatus::Modified)];
        let initial = resolve_file_tree_git_status_state(&entries, None)
            .expect("expected initial Git status state");
        let resolved = resolve_file_tree_git_status_state(&entries, Some(&initial))
            .expect("expected reused Git status state");

        assert!(Arc::ptr_eq(&initial, &resolved));
        assert!(apply_file_tree_git_status_patch(
            Some(&initial),
            &FileTreeGitStatusPatch::new().with_remove(["///"]),
        )
        .is_some_and(|state| Arc::ptr_eq(&initial, &state)));
    }
}
