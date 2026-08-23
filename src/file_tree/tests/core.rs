use crate::file_tree::{
    interaction::{
        compute_file_tree_row_click_plan, FileTreeRowClickMode, FileTreeSelectionDirective,
    },
    mutation::{FileTreeBatchOperation, FileTreeMoveOptions},
    prepared_input::PreparedInputKind,
    rename::{FileTreeRenameCommit, FileTreeRenameError},
    runtime::FileTreeRuntime,
    FileTreeInitialExpansion, FileTreeInputError, FileTreePreparedInput, FileTreeSearchMode,
};

#[test]
fn prepared_input_preserves_paths_and_sorts_with_pierre_order() {
    let input = FileTreePreparedInput::prepare([
        "file10",
        "src/z.rs",
        "file2",
        "tmp/",
        "src/a.rs",
        "../kept",
        r"dir\name",
    ])
    .unwrap();
    let PreparedInputKind::Prepared { paths, .. } = input.kind() else {
        panic!("raw input must produce parsed prepared paths");
    };

    assert!(paths.iter().any(|path| path.path == "../kept"));
    assert!(paths.iter().any(|path| path.path == r"dir\name"));
    assert_eq!(
        paths
            .iter()
            .map(|path| path.path.as_str())
            .collect::<Vec<_>>(),
        vec![
            "../kept",
            "src/a.rs",
            "src/z.rs",
            "tmp/",
            r"dir\name",
            "file2",
            "file10",
        ]
    );
}

#[test]
fn prepared_input_uses_javascript_utf16_string_order() {
    let input = FileTreePreparedInput::prepare(["\u{e000}", "\u{10000}"]).unwrap();
    let PreparedInputKind::Prepared { paths, .. } = input.kind() else {
        panic!("raw input must produce parsed prepared paths");
    };

    assert_eq!(
        paths
            .iter()
            .map(|path| path.path.as_str())
            .collect::<Vec<_>>(),
        vec!["\u{10000}", "\u{e000}"]
    );
}

#[test]
fn custom_path_comparator_survives_mutation_rebuilds() {
    let input = FileTreePreparedInput::prepare_with_comparator(["a.rs", "c.rs"], |left, right| {
        right.basename.cmp(left.basename)
    })
    .unwrap();
    let mut runtime = FileTreeRuntime::new(&input, FileTreeInitialExpansion::Closed, &[]);

    assert_eq!(paths(&runtime, 0..10), vec!["c.rs", "a.rs"]);
    runtime
        .apply_mutation(&FileTreeBatchOperation::add("b.rs"))
        .unwrap();
    assert_eq!(paths(&runtime, 0..10), vec!["c.rs", "b.rs", "a.rs"]);
}

#[test]
fn presorted_input_checks_adjacent_duplicates_and_file_prefix_collisions() {
    assert_eq!(
        FileTreePreparedInput::presorted(["a", "a"]).unwrap_err(),
        FileTreeInputError::DuplicatePath("a".to_owned())
    );
    assert!(matches!(
        FileTreePreparedInput::presorted(["a/", "a"]),
        Err(FileTreeInputError::PathCollidesWithFile { .. })
    ));
    assert!(matches!(
        FileTreePreparedInput::presorted(["a", "a/b"]),
        Err(FileTreeInputError::PathCollidesWithFile { .. })
    ));
}

#[test]
fn visible_projection_respects_closed_open_and_depth_expansion() {
    let input =
        FileTreePreparedInput::prepare(["README.md", "src/index.ts", "src/lib/util.ts", "tmp/"])
            .unwrap();

    let closed = FileTreeRuntime::new(&input, FileTreeInitialExpansion::Closed, &[]);
    assert_eq!(paths(&closed, 0..20), vec!["src/", "tmp/", "README.md"]);

    let open = FileTreeRuntime::new(&input, FileTreeInitialExpansion::Open, &[]);
    assert_eq!(
        paths(&open, 0..20),
        vec![
            "src/",
            "src/lib/",
            "src/lib/util.ts",
            "src/index.ts",
            "tmp/",
            "README.md",
        ]
    );
    assert_eq!(
        open.visible_rows(0..20)
            .into_iter()
            .map(|row| row.level)
            .collect::<Vec<_>>(),
        vec![0, 1, 2, 1, 0, 0]
    );

    let depth_one = FileTreeRuntime::new(&input, FileTreeInitialExpansion::Depth(1), &[]);
    assert_eq!(
        paths(&depth_one, 0..20),
        vec!["src/", "src/lib/", "src/index.ts", "tmp/", "README.md"]
    );
}

#[test]
fn flattened_empty_directories_share_one_visible_row_and_terminal_expansion() {
    let input = FileTreePreparedInput::prepare([
        "config/project/src/a.rs",
        "config/project/src/b.rs",
        "top.txt",
    ])
    .unwrap();
    let mut runtime = FileTreeRuntime::new_with_configuration_and_flattening(
        &input,
        FileTreeInitialExpansion::Closed,
        &[],
        &[],
        FileTreeSearchMode::default(),
        true,
    );

    assert_eq!(
        paths(&runtime, 0..10),
        vec!["config/project/src/", "top.txt"]
    );
    let row = runtime.visible_rows(0..1).remove(0);
    assert!(row.is_flattened);
    assert_eq!(row.depth, 1);
    assert_eq!(
        row.flattened_segments
            .iter()
            .map(|segment| (segment.path.as_str(), segment.is_terminal))
            .collect::<Vec<_>>(),
        vec![
            ("config/", false),
            ("config/project/", false),
            ("config/project/src/", true),
        ]
    );

    assert!(runtime.toggle("config/project/src/"));
    assert_eq!(
        paths(&runtime, 0..10),
        vec![
            "config/project/src/",
            "config/project/src/a.rs",
            "config/project/src/b.rs",
            "top.txt",
        ]
    );
}

#[test]
fn explicit_expansion_and_incremental_toggle_update_visible_counts() {
    let input = FileTreePreparedInput::prepare(["src/index.ts", "src/lib/util.ts"]).unwrap();
    let mut runtime = FileTreeRuntime::new(
        &input,
        FileTreeInitialExpansion::Closed,
        &["src/lib/".to_owned()],
    );

    assert_eq!(runtime.visible_count(), 1);
    assert!(runtime.toggle("src/"));
    assert_eq!(runtime.visible_count(), 4);
    assert_eq!(
        paths(&runtime, 0..10),
        vec!["src/", "src/lib/", "src/lib/util.ts", "src/index.ts"]
    );
    assert!(runtime.toggle("src/lib/"));
    assert_eq!(runtime.visible_count(), 3);
    assert_eq!(
        paths(&runtime, 0..10),
        vec!["src/", "src/lib/", "src/index.ts"]
    );
}

#[test]
fn expansion_keys_remove_only_one_terminal_slash() {
    let input = FileTreePreparedInput::prepare(["a//file.txt"]).unwrap();
    let mut runtime =
        FileTreeRuntime::new(&input, FileTreeInitialExpansion::Closed, &["a/".to_owned()]);

    assert_eq!(paths(&runtime, 0..10), vec!["a/", "a//"]);

    runtime.reconcile(
        &input,
        FileTreeInitialExpansion::Closed,
        &["a//".to_owned()],
        FileTreeSearchMode::default(),
    );
    assert_eq!(paths(&runtime, 0..10), vec!["a/"]);
    assert!(runtime.toggle("a/"));
    assert_eq!(paths(&runtime, 0..10), vec!["a/", "a//", "a//file.txt"]);
}

#[test]
fn range_materialization_only_returns_the_requested_rows() {
    let input = FileTreePreparedInput::prepare([
        "src/a.rs", "src/b.rs", "src/c.rs", "src/d.rs", "src/e.rs",
    ])
    .unwrap();
    let runtime = FileTreeRuntime::new(&input, FileTreeInitialExpansion::Open, &[]);

    let rows = runtime.visible_rows(2..4);
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].index, 2);
    assert_eq!(rows[0].path, "src/b.rs");
    assert_eq!(rows[1].path, "src/c.rs");
}

#[test]
fn presorted_build_keeps_child_maps_lazy_and_summarizes_wide_directories() {
    let paths = (0..300)
        .map(|index| format!("wide/file{index}.txt"))
        .collect::<Vec<_>>();
    let input = FileTreePreparedInput::presorted(paths).unwrap();
    let runtime = FileTreeRuntime::new(&input, FileTreeInitialExpansion::Open, &[]);

    assert!(runtime.store().directory_maps_are_lazy("wide/"));
    assert!(runtime.store().has_wide_directory_summary("wide/"));
    assert_eq!(runtime.visible_count(), 301);
    let rows = runtime.visible_rows(250..260);
    assert_eq!(rows.len(), 10);
    assert_eq!(rows[0].path, "wide/file249.txt");
    assert_eq!(rows[9].path, "wide/file258.txt");
}

#[test]
fn closed_construction_keeps_complete_paths_lazy() {
    let paths = (0..200)
        .map(|index| format!("dir{index}/"))
        .collect::<Vec<_>>();
    let input = FileTreePreparedInput::presorted(paths).unwrap();
    let runtime = FileTreeRuntime::new(&input, FileTreeInitialExpansion::Closed, &[]);

    assert_eq!(runtime.store().cached_path_count(), 2);
    assert_eq!(runtime.visible_rows(50..55).len(), 5);
    assert_eq!(runtime.store().cached_path_count(), 7);
}

#[test]
fn initial_selection_canonicalizes_paths_and_focuses_the_last_valid_item() {
    let input = FileTreePreparedInput::prepare(["README.md", "src/foo/bar.ts"]).unwrap();
    let runtime = FileTreeRuntime::new_with_selection(
        &input,
        FileTreeInitialExpansion::Open,
        &[],
        &["missing.ts".to_owned(), "src/foo".to_owned()],
    );

    assert_eq!(runtime.selected_paths(), &["src/foo/".to_owned()]);
    assert_eq!(runtime.selection_anchor_path(), Some("src/foo/"));
    assert_eq!(runtime.focused_path(), Some("src/foo/"));
    assert_eq!(runtime.focused_index(), Some(1));
    assert_eq!(runtime.selection_version(), 1);

    let rows = runtime.visible_rows(0..runtime.visible_count());
    assert_eq!(rows.iter().filter(|row| row.is_focused).count(), 1);
    assert!(rows
        .iter()
        .any(|row| row.path == "src/foo/" && row.is_selected));
}

#[test]
fn focus_navigation_uses_visible_order_and_canonical_parent_paths() {
    let input =
        FileTreePreparedInput::prepare(["README.md", "src/index.ts", "src/lib/util.ts"]).unwrap();
    let mut runtime = FileTreeRuntime::new(&input, FileTreeInitialExpansion::Open, &[]);

    assert_eq!(runtime.focused_path(), Some("src/"));
    assert!(runtime.focus_next_item().state_changed);
    assert_eq!(runtime.focused_path(), Some("src/lib/"));
    assert_eq!(runtime.focused_index(), Some(1));

    runtime.focus_last_item();
    assert_eq!(runtime.focused_path(), Some("README.md"));
    assert!(!runtime.focus_next_item().state_changed);
    runtime.focus_previous_item();
    assert_eq!(runtime.focused_path(), Some("src/index.ts"));
    runtime.focus_parent_item();
    assert_eq!(runtime.focused_path(), Some("src/"));

    runtime.focus_path("src/lib/util.ts");
    assert_eq!(runtime.focused_path(), Some("src/lib/util.ts"));
    runtime.focus_parent_item();
    assert_eq!(runtime.focused_path(), Some("src/lib/"));
    runtime.focus_first_item();
    assert_eq!(runtime.focused_path(), Some("src/"));
}

#[test]
fn selection_ranges_use_the_last_explicit_anchor() {
    let input = FileTreePreparedInput::prepare(["a.ts", "b.ts", "c.ts"]).unwrap();
    let mut runtime = FileTreeRuntime::new_with_selection(
        &input,
        FileTreeInitialExpansion::Closed,
        &[],
        &["a.ts".to_owned(), "c.ts".to_owned()],
    );

    let update = runtime.select_path_range("b.ts", false);
    assert!(update.selection_changed);
    assert_eq!(
        runtime.selected_paths(),
        &["b.ts".to_owned(), "c.ts".to_owned()]
    );
    assert_eq!(runtime.selection_anchor_path(), Some("c.ts"));

    runtime.toggle_path_selection("a.ts");
    assert_eq!(runtime.selection_anchor_path(), Some("a.ts"));
    runtime.select_path_range("b.ts", true);
    assert_eq!(
        runtime.selected_paths(),
        &["b.ts".to_owned(), "c.ts".to_owned(), "a.ts".to_owned()]
    );
    runtime.select_all_visible_paths();
    assert_eq!(
        runtime.selected_paths(),
        &["a.ts".to_owned(), "b.ts".to_owned(), "c.ts".to_owned()]
    );
}

#[test]
fn shift_extension_reverses_without_duplicate_selection_updates() {
    let input = FileTreePreparedInput::prepare(["a.ts", "b.ts", "c.ts"]).unwrap();
    let mut runtime = FileTreeRuntime::new(&input, FileTreeInitialExpansion::Closed, &[]);

    let first = runtime.extend_selection_from_focused(1);
    assert!(first.selection_changed);
    assert_eq!(runtime.focused_path(), Some("b.ts"));
    assert_eq!(runtime.selected_paths(), &["b.ts".to_owned()]);
    assert_eq!(runtime.selection_anchor_path(), Some("a.ts"));

    runtime.extend_selection_from_focused(1);
    assert_eq!(runtime.focused_path(), Some("c.ts"));
    assert_eq!(
        runtime.selected_paths(),
        &["b.ts".to_owned(), "c.ts".to_owned()]
    );
    let version = runtime.selection_version();

    runtime.extend_selection_from_focused(-1);
    assert_eq!(runtime.focused_path(), Some("b.ts"));
    assert_eq!(runtime.selected_paths(), &["b.ts".to_owned()]);
    assert_eq!(runtime.selection_version(), version + 1);
    assert!(runtime.extend_selection_from_focused(-1).selection_changed);
    assert_eq!(runtime.focused_path(), Some("a.ts"));
    assert!(!runtime.extend_selection_from_focused(-1).state_changed);
}

#[test]
fn collapse_repairs_focus_to_visible_ancestor_and_preserves_hidden_selection() {
    let input = FileTreePreparedInput::prepare(["README.md", "src/lib/util.ts"]).unwrap();
    let mut runtime = FileTreeRuntime::new_with_selection(
        &input,
        FileTreeInitialExpansion::Open,
        &[],
        &["src/lib/util.ts".to_owned()],
    );

    assert!(runtime.collapse("src/").state_changed);
    assert_eq!(runtime.focused_path(), Some("src/"));
    assert_eq!(runtime.focused_index(), Some(0));
    assert!(runtime.is_selected("src/lib/util.ts"));
    assert_eq!(runtime.visible_count(), 2);
    assert!(runtime.is_directory("src"));
    assert!(!runtime.is_expanded("src/"));
}

#[test]
fn row_click_plan_matches_pierre_modifier_precedence() {
    let plain_directory =
        compute_file_tree_row_click_plan(false, false, false, FileTreeRowClickMode::Flow, true);
    assert_eq!(
        plain_directory.selection,
        FileTreeSelectionDirective::Single
    );
    assert!(plain_directory.toggle_directory);
    assert!(!plain_directory.close_search);
    assert!(!plain_directory.reveal_canonical);

    let additive_range =
        compute_file_tree_row_click_plan(true, true, false, FileTreeRowClickMode::Sticky, true);
    assert_eq!(
        additive_range.selection,
        FileTreeSelectionDirective::Range { additive: true }
    );
    assert!(!additive_range.toggle_directory);
    assert!(additive_range.reveal_canonical);

    let toggle =
        compute_file_tree_row_click_plan(false, false, true, FileTreeRowClickMode::Flow, false);
    assert_eq!(toggle.selection, FileTreeSelectionDirective::Toggle);
    assert!(!toggle.toggle_directory);
}

#[test]
fn row_and_context_clicks_share_path_first_focus_and_selection_commands() {
    let input =
        FileTreePreparedInput::prepare(["other.ts", "src/file.ts", "unselected.ts"]).unwrap();
    let mut runtime = FileTreeRuntime::new(&input, FileTreeInitialExpansion::Open, &[]);
    let plain_directory =
        compute_file_tree_row_click_plan(false, false, false, FileTreeRowClickMode::Flow, true);

    let update = runtime.apply_row_click("src", plain_directory);
    assert!(update.selection_changed);
    assert_eq!(runtime.focused_path(), Some("src/"));
    assert_eq!(runtime.selected_paths(), &["src/".to_owned()]);
    assert!(!runtime.is_expanded("src/"));

    let toggle_file =
        compute_file_tree_row_click_plan(false, true, false, FileTreeRowClickMode::Flow, false);
    runtime.apply_row_click("other.ts", toggle_file);
    assert_eq!(
        runtime.selected_paths(),
        &["src/".to_owned(), "other.ts".to_owned()]
    );

    let selected_paths = runtime.selected_paths().to_vec();
    let version = runtime.selection_version();
    let context_update = runtime.apply_context_click("unselected.ts");
    assert!(!context_update.selection_changed);
    assert_eq!(runtime.focused_path(), Some("unselected.ts"));
    assert_eq!(runtime.selected_paths(), selected_paths);
    assert_eq!(runtime.selection_version(), version);
}

#[test]
fn rename_state_expands_ancestors_closes_search_and_prepares_parity_results() {
    let input =
        FileTreePreparedInput::prepare(["README.md", "src/index.ts", "src/lib/util.ts"]).unwrap();
    let mut runtime = FileTreeRuntime::new_with_configuration(
        &input,
        FileTreeInitialExpansion::Closed,
        &[],
        &["README.md".to_owned()],
        FileTreeSearchMode::HideNonMatches,
    );
    runtime.set_search(Some("util"));

    let update = runtime.start_renaming("src/lib/util.ts", false);
    assert!(update.state_changed);
    assert!(update.selection_changed);
    assert_eq!(runtime.search_value_option(), None);
    assert_eq!(runtime.focused_path(), Some("src/lib/util.ts"));
    assert_eq!(runtime.selected_paths(), &["src/lib/util.ts".to_owned()]);
    assert!(runtime.is_expanded("src/"));
    assert!(runtime.is_expanded("src/lib/"));
    assert_eq!(runtime.renaming_value(), "util.ts");

    let (_, unchanged) = runtime.prepare_rename_commit();
    assert_eq!(unchanged, Some(FileTreeRenameCommit::Unchanged));
    assert_eq!(runtime.focused_path(), Some("src/lib/util.ts"));

    runtime.start_renaming("src/lib/util.ts", false);
    runtime.set_renaming_value("nested/name");
    let (_, invalid) = runtime.prepare_rename_commit();
    assert_eq!(
        invalid,
        Some(FileTreeRenameCommit::Error(
            FileTreeRenameError::NameContainsSlash
        ))
    );
    assert_eq!(runtime.focused_path(), Some("src/lib/util.ts"));
}

#[test]
fn rename_state_remaps_through_mutations_and_supports_placeholder_removal() {
    let input = FileTreePreparedInput::prepare(["src/lib/util.ts", "src/main.ts"]).unwrap();
    let mut runtime = FileTreeRuntime::new(&input, FileTreeInitialExpansion::Open, &[]);

    runtime.start_renaming("src/lib/util.ts", false);
    runtime.set_renaming_value("worker.ts");
    runtime
        .apply_mutation(&FileTreeBatchOperation::move_path(
            "src",
            "app",
            FileTreeMoveOptions::default(),
        ))
        .unwrap();
    assert_eq!(runtime.renaming_path(), Some("app/lib/util.ts"));
    assert_eq!(runtime.renaming_value(), "worker.ts");

    let (_, commit) = runtime.prepare_rename_commit();
    let Some(FileTreeRenameCommit::Move(plan)) = commit else {
        panic!("renamed file must produce a move plan");
    };
    assert_eq!(plan.source_canonical_path, "app/lib/util.ts");
    assert_eq!(plan.destination_canonical_path, "app/lib/worker.ts");
    assert_eq!(plan.event.source_path, "app/lib/util.ts");
    assert_eq!(plan.event.destination_path, "app/lib/worker.ts");

    runtime.start_renaming("app/main.ts", true);
    runtime.set_renaming_value("  ");
    let (_, removal) = runtime.prepare_rename_commit();
    assert_eq!(
        removal,
        Some(FileTreeRenameCommit::Remove {
            canonical_path: "app/main.ts".to_owned(),
            is_directory: false,
        })
    );
}

#[test]
fn sticky_projection_keeps_nested_slot_continuity_and_pushes_at_subtree_boundaries() {
    let input = FileTreePreparedInput::prepare([
        "a/b/c/file.ts",
        "a/b/d/file.ts",
        "a/e/file.ts",
        "x/file.ts",
    ])
    .unwrap();
    let mut runtime = FileTreeRuntime::new(&input, FileTreeInitialExpansion::Open, &[]);

    let nested = runtime.sticky_rows(90.0, 30.0);
    assert_eq!(
        nested
            .iter()
            .map(|entry| (entry.row.path.as_str(), entry.top))
            .collect::<Vec<_>>(),
        vec![("a/", 0.0), ("a/b/", 30.0), ("a/b/d/", 60.0)]
    );

    let displaced = runtime.sticky_rows(121.0, 30.0);
    assert_eq!(
        displaced
            .iter()
            .map(|entry| (entry.row.path.as_str(), entry.top))
            .collect::<Vec<_>>(),
        vec![("a/", 0.0), ("a/b/", 29.0), ("a/b/d/", 29.0)]
    );

    let after_boundary = runtime.sticky_rows(151.0, 30.0);
    assert_eq!(
        after_boundary
            .iter()
            .map(|entry| entry.row.path.as_str())
            .collect::<Vec<_>>(),
        vec!["a/", "a/e/"]
    );
}

#[test]
fn sticky_projection_disables_for_search_and_uses_compact_visible_chains() {
    let input =
        FileTreePreparedInput::prepare(["a/b/c/file.ts", "a/b/c/peer.ts", "z/file.ts"]).unwrap();
    let mut runtime = FileTreeRuntime::new_with_configuration_and_flattening(
        &input,
        FileTreeInitialExpansion::Open,
        &[],
        &[],
        FileTreeSearchMode::HideNonMatches,
        true,
    );

    let sticky = runtime.sticky_rows(1.0, 30.0);
    assert_eq!(sticky.len(), 1);
    assert_eq!(sticky[0].row.path, "a/b/c/");
    assert!(sticky[0].row.is_flattened);
    assert_eq!(
        sticky[0]
            .row
            .flattened_segments
            .iter()
            .map(|segment| segment.path.as_str())
            .collect::<Vec<_>>(),
        vec!["a/", "a/b/", "a/b/c/"]
    );
    assert_eq!(runtime.sticky_inset_rows_for_visible_index(1), 1);

    runtime.set_search(Some("file"));
    assert!(runtime.sticky_rows(30.0, 30.0).is_empty());
    assert_eq!(runtime.sticky_inset_rows_for_visible_index(0), 0);
}

fn paths(runtime: &FileTreeRuntime, range: std::ops::Range<usize>) -> Vec<String> {
    runtime
        .visible_rows(range)
        .into_iter()
        .map(|row| row.path)
        .collect()
}
