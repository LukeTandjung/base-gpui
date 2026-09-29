/// Initial directory expansion policy.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum FileTreeInitialExpansion {
    /// Start with all directories closed.
    #[default]
    Closed,
    /// Start with all directories open.
    Open,
    /// Open directories whose depth is at most this value.
    Depth(usize),
}
