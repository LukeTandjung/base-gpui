use std::{
    collections::{BTreeSet, HashMap, HashSet},
    error::Error,
    fmt,
};

use super::{prepared_input::strip_one_terminal_slash, FileTreeInputError, FileTreePreparedInput};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FileTreeRemoveOptions {
    pub recursive: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FileTreeCollisionStrategy {
    #[default]
    Error,
    Replace,
    Skip,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FileTreeMoveOptions {
    pub collision: FileTreeCollisionStrategy,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FileTreeBatchOperation {
    Add {
        path: String,
    },
    Remove {
        path: String,
        options: FileTreeRemoveOptions,
    },
    Move {
        from: String,
        to: String,
        options: FileTreeMoveOptions,
    },
}

impl FileTreeBatchOperation {
    pub fn add(path: impl Into<String>) -> Self {
        Self::Add { path: path.into() }
    }

    pub fn remove(path: impl Into<String>, options: FileTreeRemoveOptions) -> Self {
        Self::Remove {
            path: path.into(),
            options,
        }
    }

    pub fn move_path(
        from: impl Into<String>,
        to: impl Into<String>,
        options: FileTreeMoveOptions,
    ) -> Self {
        Self::Move {
            from: from.into(),
            to: to.into(),
            options,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FileTreeMutationEventType {
    Add,
    Remove,
    Move,
    Batch,
    Reset,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FileTreeMutationInvalidation {
    pub canonical_changed: bool,
    pub projection_changed: bool,
    pub visible_count_delta: Option<isize>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileTreeAddEvent {
    pub invalidation: FileTreeMutationInvalidation,
    pub path: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileTreeRemoveEvent {
    pub invalidation: FileTreeMutationInvalidation,
    pub path: String,
    pub recursive: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileTreeMoveEvent {
    pub invalidation: FileTreeMutationInvalidation,
    pub from: String,
    pub to: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FileTreeMutationSemanticEvent {
    Add(FileTreeAddEvent),
    Remove(FileTreeRemoveEvent),
    Move(FileTreeMoveEvent),
    Reset(FileTreeResetEvent),
}

impl FileTreeMutationSemanticEvent {
    pub fn event_type(&self) -> FileTreeMutationEventType {
        match self {
            Self::Add(_) => FileTreeMutationEventType::Add,
            Self::Remove(_) => FileTreeMutationEventType::Remove,
            Self::Move(_) => FileTreeMutationEventType::Move,
            Self::Reset(_) => FileTreeMutationEventType::Reset,
        }
    }

    pub fn invalidation(&self) -> FileTreeMutationInvalidation {
        match self {
            Self::Add(event) => event.invalidation,
            Self::Remove(event) => event.invalidation,
            Self::Move(event) => event.invalidation,
            Self::Reset(event) => event.invalidation,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileTreeBatchEvent {
    pub invalidation: FileTreeMutationInvalidation,
    pub events: Vec<FileTreeMutationSemanticEvent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileTreeResetEvent {
    pub invalidation: FileTreeMutationInvalidation,
    pub path_count_after: usize,
    pub path_count_before: usize,
    pub used_prepared_input: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FileTreeMutationEvent {
    Add(FileTreeAddEvent),
    Remove(FileTreeRemoveEvent),
    Move(FileTreeMoveEvent),
    Batch(FileTreeBatchEvent),
    Reset(FileTreeResetEvent),
}

impl FileTreeMutationEvent {
    pub fn event_type(&self) -> FileTreeMutationEventType {
        match self {
            Self::Add(_) => FileTreeMutationEventType::Add,
            Self::Remove(_) => FileTreeMutationEventType::Remove,
            Self::Move(_) => FileTreeMutationEventType::Move,
            Self::Batch(_) => FileTreeMutationEventType::Batch,
            Self::Reset(_) => FileTreeMutationEventType::Reset,
        }
    }

    pub fn invalidation(&self) -> FileTreeMutationInvalidation {
        match self {
            Self::Add(event) => event.invalidation,
            Self::Remove(event) => event.invalidation,
            Self::Move(event) => event.invalidation,
            Self::Batch(event) => event.invalidation,
            Self::Reset(event) => event.invalidation,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct FileTreeResetOptions {
    pub prepared_input: Option<FileTreePreparedInput>,
    pub initial_expanded_paths: Option<Vec<String>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FileTreeMutationError {
    NotBound,
    InvalidInput(FileTreeInputError),
    PathAlreadyExists(String),
    PathDoesNotExist(String),
    NonEmptyDirectoryRequiresRecursive(String),
    SourcePathDoesNotExist(String),
    DestinationParentDoesNotExist(String),
    DestinationAlreadyExists(String),
    CannotMoveDirectoryIntoDescendant,
    ReplaceRequiresSameKind,
    ReplaceDoesNotSupportNonEmptyDirectory,
    ResetInputMismatch,
}

impl fmt::Display for FileTreeMutationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotBound => write!(
                formatter,
                "file-tree handle is not bound to a rendered tree"
            ),
            Self::InvalidInput(error) => error.fmt(formatter),
            Self::PathAlreadyExists(path) => write!(formatter, "path already exists: {path:?}"),
            Self::PathDoesNotExist(path) => write!(formatter, "path does not exist: {path:?}"),
            Self::NonEmptyDirectoryRequiresRecursive(path) => write!(
                formatter,
                "cannot remove a non-empty directory without recursive: {path:?}"
            ),
            Self::SourcePathDoesNotExist(path) => {
                write!(formatter, "source path does not exist: {path:?}")
            }
            Self::DestinationParentDoesNotExist(path) => {
                write!(formatter, "destination parent does not exist: {path:?}")
            }
            Self::DestinationAlreadyExists(path) => {
                write!(formatter, "destination already exists: {path:?}")
            }
            Self::CannotMoveDirectoryIntoDescendant => {
                write!(
                    formatter,
                    "cannot move a directory into one of its descendants"
                )
            }
            Self::ReplaceRequiresSameKind => {
                write!(
                    formatter,
                    "replace collision requires the same source and destination kinds"
                )
            }
            Self::ReplaceDoesNotSupportNonEmptyDirectory => write!(
                formatter,
                "replace collision does not support non-empty directories"
            ),
            Self::ResetInputMismatch => write!(
                formatter,
                "file-tree reset received paths and prepared input for different path lists"
            ),
        }
    }
}

impl Error for FileTreeMutationError {}

impl From<FileTreeInputError> for FileTreeMutationError {
    fn from(error: FileTreeInputError) -> Self {
        Self::InvalidInput(error)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum NodeKind {
    Directory,
    File,
}

#[derive(Clone, Copy, Debug)]
struct NodeInfo {
    child_count: usize,
    explicit: bool,
    kind: NodeKind,
}

#[derive(Clone, Debug)]
pub enum PathMutation {
    Add,
    Remove {
        canonical_path: String,
    },
    Move {
        from: String,
        replaced_destination: bool,
        to: String,
    },
}

#[derive(Clone, Debug)]
pub struct AppliedOperation {
    pub invalidation: FileTreeMutationInvalidation,
    pub mutation: PathMutation,
    pub semantic: PendingSemanticEvent,
}

#[derive(Clone, Debug)]
pub enum PendingSemanticEvent {
    Add { path: String },
    Remove { path: String, recursive: bool },
    Move { from: String, to: String },
}

#[derive(Default)]
struct MutationImpact {
    before_visible: HashSet<String>,
    touched: HashSet<String>,
}

impl MutationImpact {
    fn touch(&mut self, path: &str, visible_paths: &HashSet<String>) {
        if self.touched.insert(path.to_owned()) && visible_paths.contains(path) {
            self.before_visible.insert(path.to_owned());
        }
    }
}

struct MoveTarget {
    collision_path: Option<String>,
    path: String,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct MutationTransactionMetrics {
    full_index_builds: usize,
    full_prepares: usize,
}

/// Private mutable path index used for one atomic mutation transaction.
///
/// The index is built once. Each operation updates only its ancestor chains and
/// affected subtrees. Pierre sorting and input preparation run once at commit.
pub struct MutationTransaction {
    expanded_paths: BTreeSet<String>,
    explicit_paths: BTreeSet<String>,
    node_paths: BTreeSet<String>,
    nodes: HashMap<String, NodeInfo>,
    visible_paths: HashSet<String>,
    #[cfg(test)]
    metrics: MutationTransactionMetrics,
}

impl MutationTransaction {
    pub fn new(paths: &[String], expanded_paths: Vec<String>) -> Self {
        let mut transaction = Self {
            expanded_paths: expanded_paths.into_iter().collect(),
            explicit_paths: BTreeSet::new(),
            node_paths: BTreeSet::new(),
            nodes: HashMap::new(),
            visible_paths: HashSet::new(),
            #[cfg(test)]
            metrics: MutationTransactionMetrics {
                full_index_builds: 1,
                full_prepares: 0,
            },
        };
        for path in paths {
            transaction.insert_explicit_unchecked(path, None);
        }
        transaction.rebuild_initial_visibility();
        transaction
    }

    pub fn apply_operation(
        &mut self,
        operation: &FileTreeBatchOperation,
    ) -> Result<Option<AppliedOperation>, FileTreeMutationError> {
        match operation {
            FileTreeBatchOperation::Add { path } => self.add_path(path),
            FileTreeBatchOperation::Remove { path, options } => self.remove_path(path, *options),
            FileTreeBatchOperation::Move { from, to, options } => {
                self.move_path(from, to, *options)
            }
        }
    }

    pub fn expanded_paths(&self) -> Vec<String> {
        self.expanded_paths.iter().cloned().collect()
    }

    pub fn prepare_input(
        &mut self,
        source: &FileTreePreparedInput,
    ) -> Result<FileTreePreparedInput, FileTreeMutationError> {
        #[cfg(test)]
        {
            self.metrics.full_prepares += 1;
        }
        source
            .prepare_updated_paths(self.explicit_paths.iter().cloned())
            .map_err(Into::into)
    }

    #[cfg(test)]
    fn metrics(&self) -> MutationTransactionMetrics {
        self.metrics
    }

    fn add_path(&mut self, path: &str) -> Result<Option<AppliedOperation>, FileTreeMutationError> {
        let requested_kind = if path.ends_with('/') {
            NodeKind::Directory
        } else {
            NodeKind::File
        };
        if let Some((canonical, node)) = self.resolve_node(path) {
            if requested_kind == NodeKind::Directory
                && node.kind == NodeKind::Directory
                && !node.explicit
            {
                self.explicit_paths.insert(canonical.clone());
                self.nodes
                    .get_mut(&canonical)
                    .expect("resolved node must remain indexed")
                    .explicit = true;
                return Ok(Some(AppliedOperation {
                    invalidation: unchanged_projection_invalidation(),
                    mutation: PathMutation::Add,
                    semantic: PendingSemanticEvent::Add {
                        path: path.to_owned(),
                    },
                }));
            }
            return Err(FileTreeMutationError::PathAlreadyExists(path.to_owned()));
        }

        self.validate_parent_chain(path)?;
        let mut impact = MutationImpact::default();
        self.insert_explicit_unchecked(path, Some(&mut impact));
        let invalidation = self.finish_impact(impact, false);
        Ok(Some(AppliedOperation {
            invalidation,
            mutation: PathMutation::Add,
            semantic: PendingSemanticEvent::Add {
                path: path.to_owned(),
            },
        }))
    }

    fn remove_path(
        &mut self,
        path: &str,
        options: FileTreeRemoveOptions,
    ) -> Result<Option<AppliedOperation>, FileTreeMutationError> {
        let Some((canonical, node)) = self.resolve_node(path) else {
            return Err(FileTreeMutationError::PathDoesNotExist(path.to_owned()));
        };
        if node.kind == NodeKind::Directory && node.child_count > 0 && !options.recursive {
            return Err(FileTreeMutationError::NonEmptyDirectoryRequiresRecursive(
                path.to_owned(),
            ));
        }

        let parent = parent_directory(&canonical);
        let mut impact = MutationImpact::default();
        self.remove_subtree(&canonical, &mut impact);
        self.promote_empty_parent(parent, &mut impact);
        let mutation = PathMutation::Remove {
            canonical_path: canonical,
        };
        self.remap_expanded_paths(&mutation);
        let invalidation = self.finish_impact(impact, false);
        Ok(Some(AppliedOperation {
            invalidation,
            mutation,
            semantic: PendingSemanticEvent::Remove {
                path: path.to_owned(),
                recursive: options.recursive,
            },
        }))
    }

    fn move_path(
        &mut self,
        from_path: &str,
        to_path: &str,
        options: FileTreeMoveOptions,
    ) -> Result<Option<AppliedOperation>, FileTreeMutationError> {
        let Some((from, source)) = self.resolve_node(from_path) else {
            return Err(FileTreeMutationError::SourcePathDoesNotExist(
                from_path.to_owned(),
            ));
        };

        let move_target = self.resolve_move_target(&from, source.kind, to_path)?;
        let to = move_target.path;
        if from == to {
            return Ok(None);
        }
        if source.kind == NodeKind::Directory && is_descendant(&to, &from) {
            return Err(FileTreeMutationError::CannotMoveDirectoryIntoDescendant);
        }

        let replaced_destination = move_target.collision_path.is_some()
            && options.collision == FileTreeCollisionStrategy::Replace;
        if let Some(collision_path) = move_target.collision_path.as_deref() {
            let collision = self
                .nodes
                .get(collision_path)
                .copied()
                .expect("resolved collision path must exist");
            match options.collision {
                FileTreeCollisionStrategy::Skip => return Ok(None),
                FileTreeCollisionStrategy::Error => {
                    return Err(FileTreeMutationError::DestinationAlreadyExists(
                        collision_path.to_owned(),
                    ));
                }
                FileTreeCollisionStrategy::Replace => {
                    if collision.kind != source.kind {
                        return Err(FileTreeMutationError::ReplaceRequiresSameKind);
                    }
                    if collision.kind == NodeKind::Directory && collision.child_count > 0 {
                        return Err(FileTreeMutationError::ReplaceDoesNotSupportNonEmptyDirectory);
                    }
                }
            }
        }

        let moved_paths = self
            .explicit_subtree_paths(&from)
            .into_iter()
            .map(|path| {
                if path == from {
                    remap_exact_path(&path, &to)
                } else {
                    remap_descendant_path(&path, &from, &to)
                }
            })
            .collect::<Vec<_>>();
        let old_parent = parent_directory(&from);
        let mut impact = MutationImpact::default();
        if let Some(collision_path) = move_target.collision_path.as_deref() {
            if options.collision == FileTreeCollisionStrategy::Replace {
                self.remove_subtree(collision_path, &mut impact);
            }
        }
        self.remove_subtree(&from, &mut impact);
        for path in moved_paths {
            self.insert_explicit_unchecked(&path, Some(&mut impact));
        }
        self.promote_empty_parent(old_parent, &mut impact);

        let mutation = PathMutation::Move {
            from: from.clone(),
            replaced_destination,
            to: to.clone(),
        };
        self.remap_expanded_paths(&mutation);
        let invalidation = self.finish_impact(impact, replaced_destination);
        Ok(Some(AppliedOperation {
            invalidation,
            mutation,
            semantic: PendingSemanticEvent::Move {
                from: from_path.to_owned(),
                to,
            },
        }))
    }

    fn resolve_move_target(
        &self,
        from: &str,
        source_kind: NodeKind,
        to_path: &str,
    ) -> Result<MoveTarget, FileTreeMutationError> {
        let bare_destination = strip_one_terminal_slash(to_path);
        if self
            .nodes
            .get(bare_destination)
            .is_some_and(|node| node.kind == NodeKind::File)
        {
            return Ok(MoveTarget {
                collision_path: Some(bare_destination.to_owned()),
                path: canonical_for_kind(to_path, source_kind),
            });
        }

        let directory_destination = format!("{bare_destination}/");
        if self
            .nodes
            .get(&directory_destination)
            .is_some_and(|node| node.kind == NodeKind::Directory)
        {
            let target = format!(
                "{directory_destination}{}{}",
                basename(from),
                directory_suffix(source_kind)
            );
            return Ok(MoveTarget {
                collision_path: self.nodes.contains_key(&target).then(|| target.clone()),
                path: target,
            });
        }

        let target = canonical_for_kind(to_path, source_kind);
        let Some(parent) = parent_directory(&target) else {
            return Ok(MoveTarget {
                collision_path: None,
                path: target,
            });
        };
        if self
            .nodes
            .get(&parent)
            .is_none_or(|node| node.kind != NodeKind::Directory)
        {
            return Err(FileTreeMutationError::DestinationParentDoesNotExist(
                to_path.to_owned(),
            ));
        }
        Ok(MoveTarget {
            collision_path: self.nodes.contains_key(&target).then(|| target.clone()),
            path: target,
        })
    }

    fn resolve_node(&self, path: &str) -> Option<(String, NodeInfo)> {
        if path.ends_with('/') {
            return self
                .nodes
                .get(path)
                .copied()
                .map(|node| (path.to_owned(), node));
        }
        if let Some(node) = self.nodes.get(path).copied() {
            return Some((path.to_owned(), node));
        }
        let directory = format!("{path}/");
        self.nodes
            .get(&directory)
            .copied()
            .map(|node| (directory, node))
    }

    fn validate_parent_chain(&self, path: &str) -> Result<(), FileTreeMutationError> {
        let mut parent = parent_directory(path);
        while let Some(directory) = parent {
            let bare = strip_one_terminal_slash(&directory);
            if self
                .nodes
                .get(bare)
                .is_some_and(|node| node.kind == NodeKind::File)
            {
                return Err(FileTreeMutationError::InvalidInput(
                    FileTreeInputError::PathCollidesWithFile {
                        file: bare.to_owned(),
                        path: path.to_owned(),
                    },
                ));
            }
            parent = parent_directory(&directory);
        }
        Ok(())
    }

    fn insert_explicit_unchecked(&mut self, path: &str, mut impact: Option<&mut MutationImpact>) {
        let ancestry = node_ancestry(path);
        for (index, (canonical, kind)) in ancestry.iter().enumerate() {
            if !self.nodes.contains_key(canonical) {
                if let Some(impact) = impact.as_deref_mut() {
                    impact.touch(canonical, &self.visible_paths);
                }
                self.nodes.insert(
                    canonical.clone(),
                    NodeInfo {
                        child_count: 0,
                        explicit: false,
                        kind: *kind,
                    },
                );
                self.node_paths.insert(canonical.clone());
                if index > 0 {
                    let parent = &ancestry[index - 1].0;
                    self.nodes
                        .get_mut(parent)
                        .expect("new node parent must be indexed")
                        .child_count += 1;
                }
            }
        }

        let canonical = ancestry
            .last()
            .map(|(canonical, _)| canonical)
            .expect("paths must contain one segment");
        self.nodes
            .get_mut(canonical)
            .expect("inserted terminal node must be indexed")
            .explicit = true;
        self.explicit_paths.insert(path.to_owned());
    }

    fn remove_subtree(&mut self, canonical: &str, impact: &mut MutationImpact) {
        let nodes = self.node_subtree_paths(canonical);
        for path in &nodes {
            impact.touch(path, &self.visible_paths);
        }
        for path in self.explicit_subtree_paths(canonical) {
            self.explicit_paths.remove(&path);
        }
        for path in nodes.into_iter().rev() {
            self.remove_node(&path);
        }
        self.prune_empty_implicit_ancestors(parent_directory(canonical), impact);
    }

    fn remove_node(&mut self, path: &str) {
        if self.nodes.remove(path).is_none() {
            return;
        }
        self.node_paths.remove(path);
        if let Some(parent) = parent_directory(path) {
            if let Some(parent_node) = self.nodes.get_mut(&parent) {
                parent_node.child_count = parent_node.child_count.saturating_sub(1);
            }
        }
    }

    fn prune_empty_implicit_ancestors(
        &mut self,
        mut parent: Option<String>,
        impact: &mut MutationImpact,
    ) {
        while let Some(path) = parent {
            let Some(node) = self.nodes.get(&path).copied() else {
                parent = parent_directory(&path);
                continue;
            };
            if node.explicit || node.child_count > 0 {
                break;
            }
            impact.touch(&path, &self.visible_paths);
            parent = parent_directory(&path);
            self.remove_node(&path);
        }
    }

    fn promote_empty_parent(&mut self, parent: Option<String>, impact: &mut MutationImpact) {
        let Some(parent) = parent else {
            return;
        };
        if self
            .nodes
            .get(&parent)
            .is_some_and(|node| node.child_count > 0 || node.explicit)
        {
            return;
        }
        self.insert_explicit_unchecked(&parent, Some(impact));
    }

    fn explicit_subtree_paths(&self, canonical: &str) -> Vec<String> {
        subtree_paths(&self.explicit_paths, canonical)
    }

    fn node_subtree_paths(&self, canonical: &str) -> Vec<String> {
        subtree_paths(&self.node_paths, canonical)
    }

    fn rebuild_initial_visibility(&mut self) {
        self.visible_paths = self
            .node_paths
            .iter()
            .filter(|path| path_is_visible(path, &self.expanded_paths))
            .cloned()
            .collect();
    }

    fn finish_impact(
        &mut self,
        impact: MutationImpact,
        force_projection_changed: bool,
    ) -> FileTreeMutationInvalidation {
        let mut after_visible = HashSet::new();
        for path in &impact.touched {
            self.visible_paths.remove(path);
            if self.nodes.contains_key(path) && path_is_visible(path, &self.expanded_paths) {
                self.visible_paths.insert(path.clone());
                after_visible.insert(path.clone());
            }
        }
        FileTreeMutationInvalidation {
            canonical_changed: true,
            projection_changed: force_projection_changed || impact.before_visible != after_visible,
            visible_count_delta: Some(
                after_visible.len() as isize - impact.before_visible.len() as isize,
            ),
        }
    }

    fn remap_expanded_paths(&mut self, mutation: &PathMutation) {
        match mutation {
            PathMutation::Add => {}
            PathMutation::Remove { canonical_path } => {
                for path in subtree_paths(&self.expanded_paths, canonical_path) {
                    self.expanded_paths.remove(&path);
                }
            }
            PathMutation::Move {
                from,
                replaced_destination,
                to,
            } => {
                if *replaced_destination {
                    for path in subtree_paths(&self.expanded_paths, to) {
                        self.expanded_paths.remove(&path);
                    }
                }
                let moved = subtree_paths(&self.expanded_paths, from);
                for path in &moved {
                    self.expanded_paths.remove(path);
                }
                for path in moved {
                    let remapped = if path == *from {
                        to.clone()
                    } else {
                        remap_descendant_path(&path, from, to)
                    };
                    self.expanded_paths.insert(remapped);
                }
            }
        }
    }
}

fn unchanged_projection_invalidation() -> FileTreeMutationInvalidation {
    FileTreeMutationInvalidation {
        canonical_changed: true,
        projection_changed: false,
        visible_count_delta: Some(0),
    }
}

fn node_ancestry(path: &str) -> Vec<(String, NodeKind)> {
    let canonical = canonical_explicit_path(path);
    let without_slash = strip_one_terminal_slash(&canonical);
    let segments = without_slash.split('/').collect::<Vec<_>>();
    let mut ancestry = Vec::with_capacity(segments.len());
    let mut prefix = String::new();
    for (index, segment) in segments.iter().enumerate() {
        if index > 0 {
            prefix.push('/');
        }
        prefix.push_str(segment);
        let terminal = index + 1 == segments.len();
        let kind = if terminal && !canonical.ends_with('/') {
            NodeKind::File
        } else {
            NodeKind::Directory
        };
        let path = if kind == NodeKind::Directory {
            format!("{prefix}/")
        } else {
            prefix.clone()
        };
        ancestry.push((path, kind));
    }
    ancestry
}

fn subtree_paths(paths: &BTreeSet<String>, canonical: &str) -> Vec<String> {
    let mut result = Vec::new();
    if paths.contains(canonical) {
        result.push(canonical.to_owned());
    }
    let prefix = if canonical.ends_with('/') {
        canonical.to_owned()
    } else {
        format!("{canonical}/")
    };
    result.extend(
        paths
            .range(prefix.clone()..)
            .take_while(|path| path.starts_with(&prefix))
            .filter(|path| path.as_str() != canonical)
            .cloned(),
    );
    result
}

fn path_is_visible(path: &str, expanded: &BTreeSet<String>) -> bool {
    let mut parent = parent_directory(path);
    while let Some(directory) = parent {
        if !expanded.contains(directory.as_str()) {
            return false;
        }
        parent = parent_directory(&directory);
    }
    true
}

fn canonical_explicit_path(path: &str) -> String {
    path.to_owned()
}

fn canonical_for_kind(path: &str, kind: NodeKind) -> String {
    let bare = strip_one_terminal_slash(path);
    match kind {
        NodeKind::Directory => format!("{bare}/"),
        NodeKind::File => bare.to_owned(),
    }
}

fn parent_directory(path: &str) -> Option<String> {
    let bare = strip_one_terminal_slash(path);
    let (parent, _) = bare.rsplit_once('/')?;
    Some(format!("{parent}/"))
}

fn basename(path: &str) -> &str {
    strip_one_terminal_slash(path)
        .rsplit_once('/')
        .map_or(strip_one_terminal_slash(path), |(_, basename)| basename)
}

fn directory_suffix(kind: NodeKind) -> &'static str {
    match kind {
        NodeKind::Directory => "/",
        NodeKind::File => "",
    }
}

fn is_descendant(candidate: &str, directory: &str) -> bool {
    let prefix = if directory.ends_with('/') {
        directory.to_owned()
    } else {
        format!("{directory}/")
    };
    candidate != directory && candidate.starts_with(&prefix)
}

fn remap_exact_path(path: &str, to: &str) -> String {
    if path.ends_with('/') {
        canonical_for_kind(to, NodeKind::Directory)
    } else {
        canonical_for_kind(to, NodeKind::File)
    }
}

fn remap_descendant_path(path: &str, from: &str, to: &str) -> String {
    let from_prefix = if from.ends_with('/') {
        from.to_owned()
    } else {
        format!("{from}/")
    };
    let to_prefix = if to.ends_with('/') {
        to.to_owned()
    } else {
        format!("{to}/")
    };
    format!("{to_prefix}{}", &path[from_prefix.len()..])
}

pub fn remap_path(path: &str, mutation: &PathMutation, preserve_removed: bool) -> Option<String> {
    match mutation {
        PathMutation::Add => Some(path.to_owned()),
        PathMutation::Remove { canonical_path } => {
            if path == canonical_path || is_descendant(path, canonical_path) {
                preserve_removed.then(|| path.to_owned())
            } else {
                Some(path.to_owned())
            }
        }
        PathMutation::Move { from, to, .. } => {
            if path == from {
                Some(to.clone())
            } else if is_descendant(path, from) {
                Some(remap_descendant_path(path, from, to))
            } else {
                Some(path.to_owned())
            }
        }
    }
}

pub fn list_count(paths: &[String]) -> usize {
    let transaction = MutationTransaction::new(paths, Vec::new());
    transaction
        .explicit_paths
        .iter()
        .filter(|path| {
            transaction
                .nodes
                .get(*path)
                .is_some_and(|node| node.kind == NodeKind::File || node.child_count == 0)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn large_batch_builds_one_index_and_prepares_once() {
        let paths = (0..2_000)
            .map(|index| format!("src/existing-{index}.rs"))
            .collect::<Vec<_>>();
        let source = FileTreePreparedInput::prepare(paths.clone()).unwrap();
        let mut transaction = MutationTransaction::new(&paths, vec!["src/".to_owned()]);

        for index in 0..1_000 {
            transaction
                .apply_operation(&FileTreeBatchOperation::add(format!(
                    "src/added-{index}.rs"
                )))
                .unwrap();
        }
        let prepared = transaction.prepare_input(&source).unwrap();

        assert_eq!(prepared.paths().len(), 3_000);
        assert_eq!(
            transaction.metrics(),
            MutationTransactionMetrics {
                full_index_builds: 1,
                full_prepares: 1,
            }
        );
    }
}
