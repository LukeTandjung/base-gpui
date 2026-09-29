use crate::file_tree::{
    mutation::{FileTreeBatchOperation, FileTreeMoveOptions},
    runtime::FileTreeRuntime,
    FileTreeInitialExpansion, FileTreePreparedInput, FileTreeSearchMode,
};

fn runtime(
    paths: &[&str],
    initial_expansion: FileTreeInitialExpansion,
    initial_expanded_paths: &[String],
    initial_selected_paths: &[String],
    search_mode: FileTreeSearchMode,
) -> FileTreeRuntime {
    let input = FileTreePreparedInput::prepare(paths.iter().copied()).unwrap();
    FileTreeRuntime::new_with_configuration(
        &input,
        initial_expansion,
        initial_expanded_paths,
        initial_selected_paths,
        search_mode,
    )
}

fn visible_paths(runtime: &FileTreeRuntime) -> Vec<String> {
    runtime
        .visible_rows(0..runtime.visible_count())
        .into_iter()
        .map(|row| row.path)
        .collect()
}

#[test]
fn search_normalizes_full_path_substrings_and_orders_listed_paths_before_directories() {
    let mut runtime = runtime(
        &["src/worker/deep/file.ts", "src/utils/Other.ts"],
        FileTreeInitialExpansion::Open,
        &[],
        &[],
        FileTreeSearchMode::HideNonMatches,
    );

    runtime.set_search(Some("  SRC\\WORKER  "));
    assert_eq!(runtime.search_value(), "src/worker");
    assert_eq!(
        runtime.search_matching_paths(),
        ["src/worker/deep/file.ts", "src/worker/", "src/worker/deep/"]
    );
    assert_eq!(
        visible_paths(&runtime),
        vec![
            "src/",
            "src/worker/",
            "src/worker/deep/",
            "src/worker/deep/file.ts"
        ]
    );

    runtime.set_search(Some("srwk"));
    assert!(runtime.search_matching_paths().is_empty());
}

#[test]
fn search_modes_apply_their_distinct_expansion_and_filter_rules() {
    let paths = [
        "README.md",
        "src/components/Button.tsx",
        "src/utils/stream.ts",
        "src/utils/worker.ts",
    ];
    let initial_expanded = ["src/components/".to_owned()];

    let mut expand = runtime(
        &paths,
        FileTreeInitialExpansion::Closed,
        &initial_expanded,
        &[],
        FileTreeSearchMode::ExpandMatches,
    );
    expand.set_search(Some("worker"));
    let visible = visible_paths(&expand);
    assert!(visible.contains(&"README.md".to_owned()));
    assert!(visible.contains(&"src/components/Button.tsx".to_owned()));
    assert!(visible.contains(&"src/utils/worker.ts".to_owned()));

    let mut collapse = runtime(
        &paths,
        FileTreeInitialExpansion::Closed,
        &initial_expanded,
        &[],
        FileTreeSearchMode::CollapseNonMatches,
    );
    collapse.set_search(Some("worker"));
    let visible = visible_paths(&collapse);
    assert!(visible.contains(&"README.md".to_owned()));
    assert!(!visible.contains(&"src/components/Button.tsx".to_owned()));
    assert!(visible.contains(&"src/utils/worker.ts".to_owned()));

    let mut hide = runtime(
        &paths,
        FileTreeInitialExpansion::Open,
        &[],
        &[],
        FileTreeSearchMode::HideNonMatches,
    );
    hide.set_search(Some("worker"));
    assert_eq!(
        visible_paths(&hide),
        vec!["src/", "src/utils/", "src/utils/worker.ts"]
    );
}

#[test]
fn search_match_navigation_clamps_without_wrapping() {
    let mut runtime = runtime(
        &["a-worker.ts", "b-worker.ts", "other.ts"],
        FileTreeInitialExpansion::Open,
        &[],
        &[],
        FileTreeSearchMode::HideNonMatches,
    );

    runtime.set_search(Some("worker"));
    assert_eq!(runtime.focused_path(), Some("a-worker.ts"));
    assert!(!runtime.focus_previous_search_match().state_changed);
    assert_eq!(runtime.focused_path(), Some("a-worker.ts"));

    assert!(runtime.focus_next_search_match().state_changed);
    assert_eq!(runtime.focused_path(), Some("b-worker.ts"));
    assert!(!runtime.focus_next_search_match().state_changed);
    assert_eq!(runtime.focused_path(), Some("b-worker.ts"));
}

#[test]
fn search_restores_expansion_preserves_hidden_selection_and_repairs_focus() {
    let mut runtime = runtime(
        &[
            "src/components/Button.tsx",
            "src/utils/stream.ts",
            "src/utils/worker.ts",
        ],
        FileTreeInitialExpansion::Closed,
        &["src/components/".to_owned()],
        &["src/components/Button.tsx".to_owned()],
        FileTreeSearchMode::HideNonMatches,
    );

    runtime.set_search(Some("worker"));
    assert_eq!(runtime.selected_paths(), ["src/components/Button.tsx"]);
    assert!(!visible_paths(&runtime).contains(&"src/components/Button.tsx".to_owned()));
    assert_eq!(runtime.focused_path(), Some("src/utils/worker.ts"));

    runtime.collapse("src/utils/");
    assert_eq!(runtime.focused_path(), Some("src/utils/"));
    assert!(!visible_paths(&runtime).contains(&"src/utils/worker.ts".to_owned()));

    runtime.set_search(None);
    let visible = visible_paths(&runtime);
    assert!(visible.contains(&"src/components/Button.tsx".to_owned()));
    assert!(!visible.contains(&"src/utils/worker.ts".to_owned()));
    assert_eq!(runtime.selected_paths(), ["src/components/Button.tsx"]);
}

#[test]
fn empty_or_closed_search_clears_manual_collapse_overrides() {
    let mut runtime = runtime(
        &["README.md", "src/utils/stream.ts", "src/utils/worker.ts"],
        FileTreeInitialExpansion::Open,
        &[],
        &[],
        FileTreeSearchMode::HideNonMatches,
    );

    runtime.set_search(Some("worker.ts"));
    runtime.collapse("src/utils/");
    assert!(!visible_paths(&runtime).contains(&"src/utils/worker.ts".to_owned()));

    runtime.set_search(Some(""));
    assert!(runtime.is_search_open());
    runtime.set_search(Some("worker.ts"));
    assert!(visible_paths(&runtime).contains(&"src/utils/worker.ts".to_owned()));

    runtime.collapse("src/utils/");
    runtime.set_search(None);
    runtime.set_search(Some("worker.ts"));
    assert!(visible_paths(&runtime).contains(&"src/utils/worker.ts".to_owned()));
}

#[test]
fn active_search_cache_refreshes_after_mutation_and_reset() {
    let mut runtime = runtime(
        &["src/utils/worker.ts", "src/utils/stream.ts"],
        FileTreeInitialExpansion::Open,
        &[],
        &[],
        FileTreeSearchMode::HideNonMatches,
    );
    runtime.set_search(Some("worker"));
    assert_eq!(runtime.search_matching_paths(), ["src/utils/worker.ts"]);
    assert_eq!(runtime.focused_path(), Some("src/utils/worker.ts"));

    runtime
        .apply_mutation(&FileTreeBatchOperation::add("src/utils/worker-new.ts"))
        .unwrap();
    assert_eq!(
        runtime.search_matching_paths(),
        ["src/utils/worker-new.ts", "src/utils/worker.ts"]
    );
    assert_eq!(runtime.focused_path(), Some("src/utils/worker.ts"));

    runtime
        .apply_mutation(&FileTreeBatchOperation::move_path(
            "src/utils/worker.ts",
            "src/utils/archive.ts",
            FileTreeMoveOptions::default(),
        ))
        .unwrap();
    assert_eq!(runtime.search_matching_paths(), ["src/utils/worker-new.ts"]);
    assert_eq!(runtime.focused_path(), Some("src/utils/worker-new.ts"));

    let reset = FileTreePreparedInput::prepare(["new/worker-reset.ts", "new/other.ts"]).unwrap();
    runtime.reset_paths(reset, None, true).unwrap();
    assert_eq!(runtime.search_matching_paths(), ["new/worker-reset.ts"]);
    assert_eq!(runtime.focused_path(), Some("new/worker-reset.ts"));
    assert_eq!(visible_paths(&runtime), vec!["new/", "new/worker-reset.ts"]);
}
