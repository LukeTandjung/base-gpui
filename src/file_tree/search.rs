/// Controls how an active search changes the tree projection.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FileTreeSearchMode {
    /// Preserve the pre-search expansion and open every matching branch.
    ExpandMatches,
    /// Start from a closed tree and open every matching branch.
    CollapseNonMatches,
    /// Open matching branches and hide rows outside those branches.
    #[default]
    HideNonMatches,
}

/// Controls what happens when the built-in search input loses focus.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FileTreeSearchBlurBehavior {
    /// Close the active search session.
    #[default]
    Close,
    /// Keep an initial search open until the user interacts with the input.
    Retain,
}

pub fn normalize_search_query(value: &str) -> String {
    value.trim().replace('\\', "/").to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::normalize_search_query;

    #[test]
    fn normalizes_search_queries() {
        assert_eq!(
            normalize_search_query("  SRC\\Utils\\Worker.TS  "),
            "src/utils/worker.ts"
        );
        assert_eq!(normalize_search_query("  \t\n"), "");
    }
}
