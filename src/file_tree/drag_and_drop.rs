use std::{error::Error, fmt, rc::Rc, time::Duration};

use gpui::{App, Window};

use super::{
    mutation::{FileTreeBatchOperation, FileTreeMoveOptions, FileTreeMutationError},
    prepared_input::strip_one_terminal_slash,
};

/// The kind of internal file-tree drop target.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileTreeDropTargetKind {
    Directory,
    Root,
}

/// A path-first internal file-tree drop target.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileTreeDropTarget {
    pub directory_path: Option<String>,
    pub flattened_segment_path: Option<String>,
    pub hovered_path: Option<String>,
    pub kind: FileTreeDropTargetKind,
}

impl FileTreeDropTarget {
    pub fn root(hovered_path: impl Into<String>) -> Self {
        Self {
            directory_path: None,
            flattened_segment_path: None,
            hovered_path: Some(hovered_path.into()),
            kind: FileTreeDropTargetKind::Root,
        }
    }

    pub fn directory(directory_path: impl Into<String>, hovered_path: impl Into<String>) -> Self {
        Self {
            directory_path: Some(directory_path.into()),
            flattened_segment_path: None,
            hovered_path: Some(hovered_path.into()),
            kind: FileTreeDropTargetKind::Directory,
        }
    }

    pub fn flattened_directory(
        directory_path: impl Into<String>,
        hovered_path: impl Into<String>,
    ) -> Self {
        let directory_path = directory_path.into();
        Self {
            flattened_segment_path: Some(directory_path.clone()),
            directory_path: Some(directory_path),
            hovered_path: Some(hovered_path.into()),
            kind: FileTreeDropTargetKind::Directory,
        }
    }
}

/// The paths and target supplied to `can_drop` and error callbacks.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileTreeDropContext {
    pub dragged_paths: Vec<String>,
    pub target: FileTreeDropTarget,
}

/// The mutation form used to complete an internal drop.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileTreeDropOperation {
    Batch,
    Move,
}

/// A successful internal file-tree drop.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileTreeDropResult {
    pub dragged_paths: Vec<String>,
    pub operation: FileTreeDropOperation,
    pub target: FileTreeDropTarget,
}

/// A failed internal file-tree drop.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FileTreeDropError {
    NotBound,
    Disabled,
    InvalidSourcePath(String),
    SearchActive,
    DragRejected,
    NoActiveDrag,
    InvalidTarget,
    NoMoveRequired,
    Mutation(FileTreeMutationError),
}

impl fmt::Display for FileTreeDropError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotBound => write!(
                formatter,
                "file-tree handle is not bound to a rendered tree"
            ),
            Self::Disabled => write!(formatter, "file-tree drag and drop is disabled"),
            Self::InvalidSourcePath(path) => {
                write!(formatter, "drag source does not exist: {path:?}")
            }
            Self::SearchActive => write!(
                formatter,
                "drag is disabled while search filtering is active"
            ),
            Self::DragRejected => write!(formatter, "drag was rejected by can_drag"),
            Self::NoActiveDrag => write!(formatter, "no file-tree drag is active"),
            Self::InvalidTarget => {
                write!(formatter, "drop target is not valid for the active drag")
            }
            Self::NoMoveRequired => write!(formatter, "drop target does not move any dragged path"),
            Self::Mutation(error) => error.fmt(formatter),
        }
    }
}

impl Error for FileTreeDropError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Mutation(error) => Some(error),
            _ => None,
        }
    }
}

impl From<FileTreeMutationError> for FileTreeDropError {
    fn from(error: FileTreeMutationError) -> Self {
        Self::Mutation(error)
    }
}

pub type FileTreeCanDragHandler = Rc<dyn Fn(&[String]) -> bool + 'static>;
pub type FileTreeCanDropHandler = Rc<dyn Fn(&FileTreeDropContext) -> bool + 'static>;
pub type FileTreeDropCompleteHandler =
    Rc<dyn Fn(&FileTreeDropResult, &mut Window, &mut App) + 'static>;
pub type FileTreeDropErrorHandler =
    Rc<dyn Fn(&FileTreeDropError, &FileTreeDropContext, &mut Window, &mut App) + 'static>;

/// Configuration for Pierre-compatible internal file-tree drag and drop.
#[derive(Clone)]
pub struct FileTreeDragAndDropConfig {
    can_drag: Option<FileTreeCanDragHandler>,
    can_drop: Option<FileTreeCanDropHandler>,
    on_drop_complete: Option<FileTreeDropCompleteHandler>,
    on_drop_error: Option<FileTreeDropErrorHandler>,
    open_on_drop_delay: Duration,
}

impl Default for FileTreeDragAndDropConfig {
    fn default() -> Self {
        Self {
            can_drag: None,
            can_drop: None,
            on_drop_complete: None,
            on_drop_error: None,
            open_on_drop_delay: Duration::from_millis(800),
        }
    }
}

impl FileTreeDragAndDropConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn can_drag(mut self, handler: impl Fn(&[String]) -> bool + 'static) -> Self {
        self.can_drag = Some(Rc::new(handler));
        self
    }

    pub fn can_drop(mut self, handler: impl Fn(&FileTreeDropContext) -> bool + 'static) -> Self {
        self.can_drop = Some(Rc::new(handler));
        self
    }

    pub fn on_drop_complete(
        mut self,
        handler: impl Fn(&FileTreeDropResult, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_drop_complete = Some(Rc::new(handler));
        self
    }

    pub fn on_drop_error(
        mut self,
        handler: impl Fn(&FileTreeDropError, &FileTreeDropContext, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_drop_error = Some(Rc::new(handler));
        self
    }

    pub fn open_on_drop_delay(mut self, delay: Duration) -> Self {
        self.open_on_drop_delay = delay;
        self
    }

    pub fn open_on_drop_delay_value(&self) -> Duration {
        self.open_on_drop_delay
    }

    pub fn allows_drag(&self, paths: &[String]) -> bool {
        self.can_drag.as_ref().is_none_or(|handler| handler(paths))
    }

    pub fn allows_drop(&self, context: &FileTreeDropContext) -> bool {
        self.can_drop
            .as_ref()
            .is_none_or(|handler| handler(context))
    }

    pub fn emit_complete(&self, result: &FileTreeDropResult, window: &mut Window, cx: &mut App) {
        if let Some(handler) = self.on_drop_complete.as_ref() {
            handler(result, window, cx);
        }
    }

    pub fn emit_error(
        &self,
        error: &FileTreeDropError,
        context: &FileTreeDropContext,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(handler) = self.on_drop_error.as_ref() {
            handler(error, context, window, cx);
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileTreeDragSession {
    pub dragged_paths: Vec<String>,
    pub primary_path: String,
    pub target: Option<FileTreeDropTarget>,
    pub hover_generation: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileTreeDropPlan {
    pub operations: Vec<FileTreeBatchOperation>,
    pub result: FileTreeDropResult,
}

pub fn normalize_dragged_paths(paths: &[String]) -> Vec<String> {
    let mut unique = Vec::new();
    for path in paths {
        if !unique.contains(path) {
            unique.push(path.clone());
        }
    }

    unique
        .iter()
        .filter(|path| {
            !unique.iter().any(|ancestor| {
                ancestor != *path && ancestor.ends_with('/') && path.starts_with(ancestor.as_str())
            })
        })
        .cloned()
        .collect()
}

pub fn resolve_dragged_paths_for_start(path: &str, selected_paths: &[String]) -> Vec<String> {
    if selected_paths.iter().any(|selected| selected == path) {
        normalize_dragged_paths(selected_paths)
    } else {
        vec![path.to_owned()]
    }
}

pub fn is_self_or_descendant_drop(dragged_paths: &[String], target: &FileTreeDropTarget) -> bool {
    if target.kind != FileTreeDropTargetKind::Directory {
        return false;
    }
    let Some(directory_path) = target.directory_path.as_deref() else {
        return false;
    };
    dragged_paths.iter().any(|path| {
        path.ends_with('/') && (directory_path == path || directory_path.starts_with(path))
    })
}

pub fn build_drop_plan(
    dragged_paths: &[String],
    target: &FileTreeDropTarget,
) -> Option<FileTreeDropPlan> {
    let operations = dragged_paths
        .iter()
        .filter_map(|path| {
            let destination = match target.kind {
                FileTreeDropTargetKind::Root => basename(path),
                FileTreeDropTargetKind::Directory => target.directory_path.clone()?,
            };
            let final_path = if destination.ends_with('/') {
                format!("{}{}", destination, basename(path))
            } else {
                destination.clone()
            };
            (final_path != *path).then(|| {
                FileTreeBatchOperation::move_path(
                    path.clone(),
                    destination,
                    FileTreeMoveOptions::default(),
                )
            })
        })
        .collect::<Vec<_>>();
    if operations.is_empty() {
        return None;
    }
    Some(FileTreeDropPlan {
        result: FileTreeDropResult {
            dragged_paths: dragged_paths.to_vec(),
            operation: if operations.len() == 1 {
                FileTreeDropOperation::Move
            } else {
                FileTreeDropOperation::Batch
            },
            target: target.clone(),
        },
        operations,
    })
}

pub fn drag_edge_scroll_delta(pointer_y: f32, top: f32, bottom: f32) -> f32 {
    const THRESHOLD: f32 = 40.0;
    const MAX_SPEED: f32 = 18.0;
    let top_distance = pointer_y - top;
    if top_distance < THRESHOLD {
        let distance = top_distance.max(0.0);
        return -(((THRESHOLD - distance) / THRESHOLD) * MAX_SPEED).ceil();
    }
    let bottom_distance = bottom - pointer_y;
    if bottom_distance < THRESHOLD {
        let distance = bottom_distance.max(0.0);
        return (((THRESHOLD - distance) / THRESHOLD) * MAX_SPEED).ceil();
    }
    0.0
}

fn basename(path: &str) -> String {
    let directory = path.ends_with('/');
    let path = strip_one_terminal_slash(path);
    let name = path.rsplit('/').next().unwrap_or(path);
    if directory {
        format!("{name}/")
    } else {
        name.to_owned()
    }
}
