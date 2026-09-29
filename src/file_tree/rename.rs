use std::{error::Error, fmt, rc::Rc};

use gpui::{App, Window};

use super::{mutation::FileTreeMutationError, prepared_input::strip_one_terminal_slash};

/// A file-tree item offered to a rename permission callback.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileTreeRenamingItem {
    pub path: String,
    pub is_directory: bool,
}

/// A completed rename request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileTreeRenameEvent {
    pub source_path: String,
    pub destination_path: String,
    pub is_directory: bool,
}

/// An inline rename failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FileTreeRenameError {
    EmptyName,
    NameContainsSlash,
    DestinationAlreadyExists(String),
    SourceDoesNotExist { path: String, is_directory: bool },
    Mutation(FileTreeMutationError),
}

impl fmt::Display for FileTreeRenameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(formatter, "Name cannot be empty."),
            Self::NameContainsSlash => write!(formatter, "Name cannot include \"/\"."),
            Self::DestinationAlreadyExists(path) => {
                write!(formatter, "\"{path}\" already exists.")
            }
            Self::SourceDoesNotExist { is_directory, .. } => write!(
                formatter,
                "Could not find the selected {} to rename.",
                if *is_directory { "folder" } else { "file" }
            ),
            Self::Mutation(error) => error.fmt(formatter),
        }
    }
}

impl Error for FileTreeRenameError {}

impl From<FileTreeMutationError> for FileTreeRenameError {
    fn from(error: FileTreeMutationError) -> Self {
        Self::Mutation(error)
    }
}

type CanRenameHandler = Rc<dyn Fn(&FileTreeRenamingItem) -> bool>;
type RenameHandler = Rc<dyn Fn(&FileTreeRenameEvent, &mut Window, &mut App)>;
type RenameErrorHandler = Rc<dyn Fn(&FileTreeRenameError, &mut Window, &mut App)>;

/// Callbacks and permission checks for inline renaming.
#[derive(Clone, Default)]
pub struct FileTreeRenamingConfig {
    can_rename: Option<CanRenameHandler>,
    on_rename: Option<RenameHandler>,
    on_error: Option<RenameErrorHandler>,
}

impl FileTreeRenamingConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn can_rename(mut self, handler: impl Fn(&FileTreeRenamingItem) -> bool + 'static) -> Self {
        self.can_rename = Some(Rc::new(handler));
        self
    }

    pub fn on_rename(
        mut self,
        handler: impl Fn(&FileTreeRenameEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_rename = Some(Rc::new(handler));
        self
    }

    pub fn on_error(
        mut self,
        handler: impl Fn(&FileTreeRenameError, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_error = Some(Rc::new(handler));
        self
    }

    pub fn allows(&self, item: &FileTreeRenamingItem) -> bool {
        self.can_rename
            .as_ref()
            .is_none_or(|can_rename| can_rename(item))
    }

    pub fn emit_rename(&self, event: &FileTreeRenameEvent, window: &mut Window, cx: &mut App) {
        if let Some(on_rename) = self.on_rename.as_ref() {
            on_rename(event, window, cx);
        }
    }

    pub fn emit_error(&self, error: &FileTreeRenameError, window: &mut Window, cx: &mut App) {
        if let Some(on_error) = self.on_error.as_ref() {
            on_error(error, window, cx);
        }
    }
}

/// Options for starting an inline rename.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FileTreeStartRenamingOptions {
    pub remove_if_canceled: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileTreeRenamePlan {
    pub source_canonical_path: String,
    pub destination_canonical_path: String,
    pub event: FileTreeRenameEvent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FileTreeRenameCommit {
    Remove {
        canonical_path: String,
        is_directory: bool,
    },
    Unchanged,
    Move(FileTreeRenamePlan),
    Error(FileTreeRenameError),
}

pub fn public_rename_path(canonical_path: &str) -> String {
    strip_one_terminal_slash(canonical_path).to_owned()
}

pub fn rename_leaf_name(canonical_path: &str) -> String {
    strip_one_terminal_slash(canonical_path)
        .rsplit('/')
        .next()
        .unwrap_or_default()
        .to_owned()
}

pub fn rename_destination(
    canonical_source_path: &str,
    value: &str,
) -> Result<(String, String), FileTreeRenameError> {
    let trimmed_name = value.trim();
    if trimmed_name.is_empty() {
        return Err(FileTreeRenameError::EmptyName);
    }
    if trimmed_name.contains('/') {
        return Err(FileTreeRenameError::NameContainsSlash);
    }

    let is_directory = canonical_source_path.ends_with('/');
    let source_path = public_rename_path(canonical_source_path);
    let parent = source_path
        .rsplit_once('/')
        .map_or("", |(parent, _)| parent);
    let destination_path = if parent.is_empty() {
        trimmed_name.to_owned()
    } else {
        format!("{parent}/{trimmed_name}")
    };
    let destination_canonical_path = if is_directory {
        format!("{destination_path}/")
    } else {
        destination_path.clone()
    };

    Ok((destination_path, destination_canonical_path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destination_validates_and_preserves_directory_kind() {
        assert_eq!(
            rename_destination("src/lib/", " renamed ").unwrap(),
            ("src/renamed".to_owned(), "src/renamed/".to_owned())
        );
        assert_eq!(
            rename_destination("README.md", " README.md ").unwrap(),
            ("README.md".to_owned(), "README.md".to_owned())
        );
        assert_eq!(
            rename_destination("README.md", " "),
            Err(FileTreeRenameError::EmptyName)
        );
        assert_eq!(
            rename_destination("README.md", "nested/name"),
            Err(FileTreeRenameError::NameContainsSlash)
        );
    }
}
