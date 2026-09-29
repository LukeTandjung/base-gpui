use std::{
    cmp::Ordering,
    collections::hash_map::DefaultHasher,
    error::Error,
    fmt,
    hash::{Hash, Hasher},
    sync::Arc,
};

pub struct SplitPath<'a> {
    pub segments: Vec<&'a str>,
    pub is_directory: bool,
}

pub fn strip_one_terminal_slash(path: &str) -> &str {
    path.strip_suffix('/').unwrap_or(path)
}

pub fn split_path(path: &str) -> SplitPath<'_> {
    let is_directory = path.ends_with('/');
    let path_without_directory_marker = if is_directory {
        strip_one_terminal_slash(path)
    } else {
        path
    };
    SplitPath {
        segments: path_without_directory_marker.split('/').collect(),
        is_directory,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedPath {
    pub path: String,
    pub segments: Box<[String]>,
    pub lower_segments: Box<[String]>,
    pub is_directory: bool,
}

impl PreparedPath {
    fn parse(path: String) -> Self {
        let SplitPath {
            segments: split_segments,
            is_directory,
        } = split_path(&path);
        let segments = split_segments
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let lower_segments = segments
            .iter()
            .map(|segment| segment.to_lowercase())
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Self {
            path,
            segments,
            lower_segments,
            is_directory,
        }
    }
}

type PathComparator = Arc<
    dyn for<'a> Fn(&FileTreePathSortEntry<'a>, &FileTreePathSortEntry<'a>) -> Ordering
        + Send
        + Sync,
>;

/// Borrowed path metadata passed to a custom file-tree sort comparator.
#[derive(Clone, Copy, Debug)]
pub struct FileTreePathSortEntry<'a> {
    /// Preserved input path, including a trailing slash for a directory.
    pub path: &'a str,
    /// Final path segment without a directory marker.
    pub basename: &'a str,
    /// Preserved path segments.
    pub segments: &'a [String],
    /// Number of preserved path segments.
    pub depth: usize,
    /// Whether the input path names a directory.
    pub is_directory: bool,
}

#[derive(Clone)]
pub enum PreparedInputKind {
    Prepared {
        paths: Arc<[PreparedPath]>,
        comparator: Option<PathComparator>,
    },
    Presorted(Arc<[String]>),
}

impl fmt::Debug for PreparedInputKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Prepared { paths, comparator } => formatter
                .debug_struct("Prepared")
                .field("paths", paths)
                .field("custom_comparator", &comparator.is_some())
                .finish(),
            Self::Presorted(paths) => formatter.debug_tuple("Presorted").field(paths).finish(),
        }
    }
}

/// Parsed or presorted file-tree input that can be reused across construction.
#[derive(Clone, Debug)]
pub struct FileTreePreparedInput {
    kind: PreparedInputKind,
    fingerprint: u64,
}

impl FileTreePreparedInput {
    /// Parse and sort raw paths with Pierre's default path order.
    pub fn prepare(
        paths: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Self, FileTreeInputError> {
        Self::prepare_inner(paths, None)
    }

    /// Parse and sort raw paths with a caller-provided path comparator.
    ///
    /// The comparator receives preserved path metadata. It must produce a stable
    /// depth-first order that keeps each directory subtree contiguous.
    pub fn prepare_with_comparator<F>(
        paths: impl IntoIterator<Item = impl Into<String>>,
        comparator: F,
    ) -> Result<Self, FileTreeInputError>
    where
        F: for<'a> Fn(&FileTreePathSortEntry<'a>, &FileTreePathSortEntry<'a>) -> Ordering
            + Send
            + Sync
            + 'static,
    {
        Self::prepare_inner(paths, Some(Arc::new(comparator)))
    }

    fn prepare_inner(
        paths: impl IntoIterator<Item = impl Into<String>>,
        comparator: Option<PathComparator>,
    ) -> Result<Self, FileTreeInputError> {
        let mut paths = paths
            .into_iter()
            .map(Into::into)
            .map(PreparedPath::parse)
            .collect::<Vec<_>>();
        match comparator.as_ref() {
            Some(comparator) => {
                paths.sort_by(|left, right| comparator(&sort_entry(left), &sort_entry(right)))
            }
            None => paths.sort_by(compare_prepared_paths),
        }
        validate_sorted_prepared_paths(&paths)?;
        let fingerprint = fingerprint(paths.iter().map(|path| path.path.as_str()));

        Ok(Self {
            kind: PreparedInputKind::Prepared {
                paths: paths.into(),
                comparator,
            },
            fingerprint,
        })
    }

    /// Keep caller-provided depth-first order and check adjacent path conflicts.
    ///
    /// This fast path trusts ordering, as Pierre does. Use [`Self::prepare`] when
    /// the caller cannot guarantee default depth-first order.
    pub fn presorted(
        paths: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Self, FileTreeInputError> {
        let paths = paths.into_iter().map(Into::into).collect::<Vec<_>>();
        validate_presorted_paths(&paths)?;
        let fingerprint = fingerprint(paths.iter().map(String::as_str));

        Ok(Self {
            kind: PreparedInputKind::Presorted(paths.into()),
            fingerprint,
        })
    }

    pub fn fingerprint(&self) -> u64 {
        self.fingerprint
    }

    pub fn kind(&self) -> &PreparedInputKind {
        &self.kind
    }

    pub fn paths(&self) -> Vec<String> {
        match &self.kind {
            PreparedInputKind::Prepared { paths, .. } => {
                paths.iter().map(|path| path.path.clone()).collect()
            }
            PreparedInputKind::Presorted(paths) => paths.iter().cloned().collect(),
        }
    }

    pub fn prepare_updated_paths(
        &self,
        paths: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Self, FileTreeInputError> {
        match &self.kind {
            PreparedInputKind::Prepared { comparator, .. } => {
                Self::prepare_inner(paths, comparator.clone())
            }
            PreparedInputKind::Presorted(_) => Self::prepare(paths),
        }
    }
}

fn sort_entry(path: &PreparedPath) -> FileTreePathSortEntry<'_> {
    FileTreePathSortEntry {
        path: &path.path,
        basename: path.segments.last().map(String::as_str).unwrap_or(""),
        segments: &path.segments,
        depth: path.segments.len(),
        is_directory: path.is_directory,
    }
}

/// Invalid file-tree input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FileTreeInputError {
    DuplicatePath(String),
    PathCollidesWithFile { file: String, path: String },
}

impl fmt::Display for FileTreeInputError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicatePath(path) => write!(formatter, "duplicate path: {path:?}"),
            Self::PathCollidesWithFile { file, path } => {
                write!(formatter, "path {path:?} collides with file {file:?}")
            }
        }
    }
}

impl Error for FileTreeInputError {}

pub fn compare_prepared_paths(left: &PreparedPath, right: &PreparedPath) -> Ordering {
    let shared_depth = left.segments.len().min(right.segments.len());

    for depth in 0..shared_depth {
        if left.segments[depth] == right.segments[depth] {
            continue;
        }

        let left_directory = kind_at_depth(left, depth);
        let right_directory = kind_at_depth(right, depth);
        if left_directory != right_directory {
            return if left_directory {
                Ordering::Less
            } else {
                Ordering::Greater
            };
        }

        let ordering =
            compare_natural_lower(&left.lower_segments[depth], &right.lower_segments[depth]);
        if ordering != Ordering::Equal {
            return ordering;
        }

        let ordering = compare_utf16(&left.segments[depth], &right.segments[depth]);
        if ordering != Ordering::Equal {
            return ordering;
        }
    }

    left.segments
        .len()
        .cmp(&right.segments.len())
        .then_with(|| right.is_directory.cmp(&left.is_directory))
}

fn kind_at_depth(path: &PreparedPath, depth: usize) -> bool {
    depth < path.segments.len().saturating_sub(1) || path.is_directory
}

fn compare_utf16(left: &str, right: &str) -> Ordering {
    left.encode_utf16().cmp(right.encode_utf16())
}

fn compare_natural_lower(left: &str, right: &str) -> Ordering {
    let mut left = left.encode_utf16().peekable();
    let mut right = right.encode_utf16().peekable();

    while let (Some(left_unit), Some(right_unit)) = (left.peek().copied(), right.peek().copied()) {
        let left_digit = is_ascii_digit(left_unit);
        let right_digit = is_ascii_digit(right_unit);

        if left_digit && right_digit {
            let (left_run_length, left_significant_start) = digit_run_metadata(&left);
            let (right_run_length, right_significant_start) = digit_run_metadata(&right);
            let left_significant_length = left_run_length - left_significant_start;
            let right_significant_length = right_run_length - right_significant_start;

            let ordering = left_significant_length
                .cmp(&right_significant_length)
                .then_with(|| {
                    left.clone()
                        .skip(left_significant_start)
                        .take(left_significant_length)
                        .cmp(
                            right
                                .clone()
                                .skip(right_significant_start)
                                .take(right_significant_length),
                        )
                });
            if ordering != Ordering::Equal {
                return ordering;
            }

            for _ in 0..left_run_length {
                left.next();
            }
            for _ in 0..right_run_length {
                right.next();
            }
            continue;
        }

        let ordering = left_unit.cmp(&right_unit);
        if ordering != Ordering::Equal {
            return ordering;
        }
        left.next();
        right.next();
    }

    match (left.peek().is_none(), right.peek().is_none()) {
        (true, true) => Ordering::Equal,
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        (false, false) => unreachable!("natural comparison loop stops only at an input end"),
    }
}

fn digit_run_metadata(units: &std::iter::Peekable<std::str::EncodeUtf16<'_>>) -> (usize, usize) {
    let mut units = units.clone();
    let mut length = 0;
    let mut leading_zeroes = 0;

    while let Some(unit) = units.peek().copied().filter(|unit| is_ascii_digit(*unit)) {
        if unit == u16::from(b'0') && leading_zeroes == length {
            leading_zeroes += 1;
        }
        length += 1;
        units.next();
    }

    (length, leading_zeroes.min(length.saturating_sub(1)))
}

fn is_ascii_digit(unit: u16) -> bool {
    (u16::from(b'0')..=u16::from(b'9')).contains(&unit)
}

fn validate_presorted_paths(paths: &[String]) -> Result<(), FileTreeInputError> {
    for pair in paths.windows(2) {
        let previous = pair[0].as_str();
        let current = pair[1].as_str();
        if previous == current {
            return Err(FileTreeInputError::DuplicatePath(current.to_owned()));
        }

        if let Some(directory_path) = previous.strip_suffix('/') {
            if directory_path == current {
                return Err(FileTreeInputError::PathCollidesWithFile {
                    file: current.to_owned(),
                    path: previous.to_owned(),
                });
            }
        } else if current.len() > previous.len()
            && current.starts_with(previous)
            && current.as_bytes()[previous.len()] == b'/'
        {
            return Err(FileTreeInputError::PathCollidesWithFile {
                file: previous.to_owned(),
                path: current.to_owned(),
            });
        }
    }

    Ok(())
}

fn validate_sorted_prepared_paths(paths: &[PreparedPath]) -> Result<(), FileTreeInputError> {
    for pair in paths.windows(2) {
        validate_adjacent_paths(&pair[0], &pair[1])?;
    }
    Ok(())
}

fn validate_adjacent_paths(
    previous: &PreparedPath,
    current: &PreparedPath,
) -> Result<(), FileTreeInputError> {
    if previous.path == current.path {
        return Err(FileTreeInputError::DuplicatePath(current.path.clone()));
    }

    if previous.segments == current.segments && previous.is_directory != current.is_directory {
        let file = if previous.is_directory {
            &current.path
        } else {
            &previous.path
        };
        return Err(FileTreeInputError::PathCollidesWithFile {
            file: file.clone(),
            path: current.path.clone(),
        });
    }

    if !current.is_directory
        && current.segments.len() < previous.segments.len()
        && previous.segments.starts_with(current.segments.as_ref())
    {
        return Err(FileTreeInputError::PathCollidesWithFile {
            file: current.path.clone(),
            path: previous.path.clone(),
        });
    }

    if !previous.is_directory
        && previous.segments.len() < current.segments.len()
        && current.segments.starts_with(previous.segments.as_ref())
    {
        return Err(FileTreeInputError::PathCollidesWithFile {
            file: previous.path.clone(),
            path: current.path.clone(),
        });
    }

    Ok(())
}

fn fingerprint<'a>(paths: impl Iterator<Item = &'a str>) -> u64 {
    let mut hasher = DefaultHasher::new();
    for path in paths {
        path.hash(&mut hasher);
    }
    hasher.finish()
}
