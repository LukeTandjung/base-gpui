use std::{cell::RefCell, rc::Rc, time::Duration};

use gpui::{
    point, px, size, Context, IntoElement, Modifiers, MouseButton, Render, TestAppContext,
    VisualTestContext, Window, WindowHandle,
};

use crate::file_tree::{
    drag_and_drop::{
        build_drop_plan, drag_edge_scroll_delta, is_self_or_descendant_drop,
        normalize_dragged_paths, FileTreeDropOperation,
    },
    runtime::FileTreeRuntime,
    FileTree, FileTreeDragAndDropConfig, FileTreeDropContext, FileTreeDropError,
    FileTreeDropResult, FileTreeDropTarget, FileTreeHandle, FileTreeInitialExpansion,
    FileTreeItemHandle, FileTreePreparedInput,
};

#[test]
fn dragged_path_normalization_filters_ancestors_without_reordering() {
    let paths = vec![
        "src/lib/a.rs".to_owned(),
        "README.md".to_owned(),
        "src/".to_owned(),
        "src/lib/".to_owned(),
        "README.md".to_owned(),
        "assets/icon.svg".to_owned(),
    ];
    assert_eq!(
        normalize_dragged_paths(&paths),
        vec![
            "README.md".to_owned(),
            "src/".to_owned(),
            "assets/icon.svg".to_owned(),
        ]
    );
}

#[test]
fn drop_validation_and_planning_preserve_order_and_target_forms() {
    let dragged = vec!["b.txt".to_owned(), "folder/a.txt".to_owned()];
    let directory = FileTreeDropTarget::directory("target/", "target/");
    let plan = build_drop_plan(&dragged, &directory).expect("directory drop should move");
    assert_eq!(plan.result.operation, FileTreeDropOperation::Batch);
    assert_eq!(plan.result.dragged_paths, dragged);
    assert_eq!(
        plan.operations,
        vec![
            crate::file_tree::FileTreeBatchOperation::move_path(
                "b.txt",
                "target/",
                Default::default(),
            ),
            crate::file_tree::FileTreeBatchOperation::move_path(
                "folder/a.txt",
                "target/",
                Default::default(),
            ),
        ]
    );

    let flattened = FileTreeDropTarget::flattened_directory("a/b/", "a/b/c/");
    assert_eq!(flattened.directory_path.as_deref(), Some("a/b/"));
    assert_eq!(flattened.flattened_segment_path.as_deref(), Some("a/b/"));

    assert!(is_self_or_descendant_drop(
        &["src/".to_owned()],
        &FileTreeDropTarget::directory("src/lib/", "src/lib/")
    ));
    assert!(build_drop_plan(
        &["src/a.txt".to_owned()],
        &FileTreeDropTarget::directory("src/", "src/b.txt")
    )
    .is_none());
    assert!(build_drop_plan(
        &["README.md".to_owned()],
        &FileTreeDropTarget::root("package.json")
    )
    .is_none());
}

#[test]
fn delayed_open_tick_is_generation_and_target_guarded() {
    let input = FileTreePreparedInput::prepare(["move.txt", "target/child.txt"]).unwrap();
    let mut runtime = FileTreeRuntime::new(&input, FileTreeInitialExpansion::Closed, &[]);
    let (primary, dragged) = runtime.prepare_drag_start("move.txt").unwrap();
    runtime.start_drag_session(primary, dragged);
    let target = FileTreeDropTarget::directory("target/", "target/");
    runtime.set_drag_target(Some(target.clone()));
    let generation = runtime.drag_session().unwrap().hover_generation;

    assert!(!runtime.is_expanded("target/"));
    assert!(
        !runtime
            .tick_drag_hover(generation.wrapping_add(1), &target)
            .state_changed
    );
    assert!(!runtime.is_expanded("target/"));
    assert!(runtime.tick_drag_hover(generation, &target).state_changed);
    assert!(runtime.is_expanded("target/"));
}

#[test]
fn row_capture_preserves_equal_hover_target_and_blank_capture_clears_it() {
    let input = FileTreePreparedInput::prepare(["move.txt", "target/child.txt"]).unwrap();
    let mut runtime = FileTreeRuntime::new(&input, FileTreeInitialExpansion::Closed, &[]);
    let (primary, dragged) = runtime.prepare_drag_start("move.txt").unwrap();
    runtime.start_drag_session(primary, dragged);
    let target = FileTreeDropTarget::directory("target/", "target/");
    runtime.set_drag_target(Some(target.clone()));
    let hover_generation = runtime.drag_session().unwrap().hover_generation;

    let capture = runtime.begin_drag_target_capture().unwrap();
    assert!(!runtime.set_drag_target(Some(target.clone())).state_changed);
    assert!(
        !runtime
            .clear_unresolved_drag_target_capture(capture)
            .state_changed
    );
    assert_eq!(
        runtime.drag_session().unwrap().hover_generation,
        hover_generation
    );
    assert_eq!(
        runtime.drag_session().unwrap().target.as_ref(),
        Some(&target)
    );

    let blank_capture = runtime.begin_drag_target_capture().unwrap();
    assert!(
        runtime
            .clear_unresolved_drag_target_capture(blank_capture)
            .state_changed
    );
    assert!(runtime.drag_session().unwrap().target.is_none());
}

#[test]
fn successful_external_mutation_aborts_the_drag_session() {
    let input = FileTreePreparedInput::prepare(["move.txt"]).unwrap();
    let mut runtime = FileTreeRuntime::new(&input, FileTreeInitialExpansion::Closed, &[]);
    let (primary, dragged) = runtime.prepare_drag_start("move.txt").unwrap();
    runtime.start_drag_session(primary, dragged);
    assert!(runtime.drag_session().is_some());

    runtime
        .apply_mutation(&crate::file_tree::FileTreeBatchOperation::add("new.txt"))
        .unwrap();
    assert!(runtime.drag_session().is_none());
}

#[test]
fn edge_auto_scroll_matches_pierre_threshold_and_speed() {
    assert_eq!(drag_edge_scroll_delta(100.0, 100.0, 300.0), -18.0);
    assert_eq!(drag_edge_scroll_delta(120.0, 100.0, 300.0), -9.0);
    assert_eq!(drag_edge_scroll_delta(200.0, 100.0, 300.0), 0.0);
    assert_eq!(drag_edge_scroll_delta(280.0, 100.0, 300.0), 9.0);
    assert_eq!(drag_edge_scroll_delta(300.0, 100.0, 300.0), 18.0);
}

struct DragDropTestView {
    config: FileTreeDragAndDropConfig,
    handle: FileTreeHandle,
    input: FileTreePreparedInput,
    initial_selected_paths: Vec<String>,
}

impl Render for DragDropTestView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        FileTree::new(self.input.clone())
            .initial_expansion(FileTreeInitialExpansion::Closed)
            .initial_selected_paths(self.initial_selected_paths.clone())
            .drag_and_drop_config(self.config.clone())
            .handle(self.handle.clone())
    }
}

fn open_tree(
    cx: &mut TestAppContext,
    config: FileTreeDragAndDropConfig,
    handle: FileTreeHandle,
    input: FileTreePreparedInput,
    initial_selected_paths: Vec<String>,
) -> WindowHandle<DragDropTestView> {
    cx.update(crate::file_tree::init);
    let window = cx.open_window(size(px(360.0), px(180.0)), move |_, _| DragDropTestView {
        config,
        handle,
        input,
        initial_selected_paths,
    });
    cx.run_until_parked();
    window
}

#[gpui::test]
fn rendered_drag_moves_selection_and_reports_synchronous_completion(cx: &mut TestAppContext) {
    let completed = Rc::new(RefCell::new(Vec::<FileTreeDropResult>::new()));
    let observed_drop = Rc::new(RefCell::new(Vec::<FileTreeDropContext>::new()));
    let complete_out = Rc::clone(&completed);
    let drop_out = Rc::clone(&observed_drop);
    let config = FileTreeDragAndDropConfig::new()
        .can_drop(move |context| {
            drop_out.borrow_mut().push(context.clone());
            true
        })
        .on_drop_complete(move |result, _, _| complete_out.borrow_mut().push(result.clone()));
    let handle = FileTreeHandle::new();
    let input =
        FileTreePreparedInput::prepare(["a.txt", "b.txt", "source/c.txt", "target/keep.txt"])
            .unwrap();
    let window = open_tree(
        cx,
        config,
        handle.clone(),
        input,
        vec!["a.txt".to_owned(), "source/c.txt".to_owned()],
    );

    window
        .update(cx, |_view, window, cx| {
            assert!(handle.start_drag("a.txt", window, cx));
            assert_eq!(
                handle.dragged_paths(cx),
                vec!["a.txt".to_owned(), "source/c.txt".to_owned()]
            );
            assert!(handle.set_drop_target(
                Some(FileTreeDropTarget::directory("target/", "target/")),
                window,
                cx,
            ));
            let result = handle.complete_drag(window, cx).unwrap();
            assert_eq!(result.operation, FileTreeDropOperation::Batch);
            assert!(handle.get_item("target/a.txt", cx).is_some());
            assert!(handle.get_item("target/c.txt", cx).is_some());
            assert!(handle.get_item("a.txt", cx).is_none());
            assert_eq!(
                handle.get_selected_paths(cx),
                vec!["target/a.txt".to_owned(), "target/c.txt".to_owned()]
            );
            assert_eq!(handle.get_focused_path(cx).as_deref(), Some("target/"));

            assert!(handle.start_drag("b.txt", window, cx));
            assert_eq!(handle.dragged_paths(cx), vec!["b.txt".to_owned()]);
            assert_eq!(handle.get_selected_paths(cx), vec!["b.txt".to_owned()]);
            assert_eq!(handle.get_focused_path(cx).as_deref(), Some("b.txt"));
            handle.cancel_drag(window, cx);

            handle.set_search(Some("target"), window, cx);
            assert!(!handle.start_drag("b.txt", window, cx));
        })
        .unwrap();

    assert!(!observed_drop.borrow().is_empty());
    assert_eq!(completed.borrow().len(), 1);
}

#[gpui::test]
fn rendered_drop_error_is_atomic_and_delayed_open_uses_pinned_timer(cx: &mut TestAppContext) {
    let errors = Rc::new(RefCell::new(
        Vec::<(FileTreeDropError, FileTreeDropContext)>::new(),
    ));
    let errors_out = Rc::clone(&errors);
    let config = FileTreeDragAndDropConfig::new()
        .open_on_drop_delay(Duration::from_millis(10))
        .on_drop_error(move |error, context, _, _| {
            errors_out
                .borrow_mut()
                .push((error.clone(), context.clone()));
        });
    let handle = FileTreeHandle::new();
    let input =
        FileTreePreparedInput::prepare(["a.txt", "b.txt", "closed/child.txt", "target/a.txt"])
            .unwrap();
    let window = open_tree(
        cx,
        config,
        handle.clone(),
        input,
        vec!["a.txt".to_owned(), "b.txt".to_owned()],
    );

    window
        .update(cx, |_view, window, cx| {
            assert!(handle.start_drag("a.txt", window, cx));
            assert!(handle.set_drop_target(
                Some(FileTreeDropTarget::directory("closed/", "closed/")),
                window,
                cx,
            ));
            let FileTreeItemHandle::Directory(closed) =
                handle.get_item("closed/", cx).expect("closed should exist")
            else {
                panic!("closed should be a directory");
            };
            assert!(!closed.is_expanded(cx));
        })
        .unwrap();
    cx.executor().advance_clock(Duration::from_millis(10));
    cx.run_until_parked();

    window
        .update(cx, |_view, window, cx| {
            let FileTreeItemHandle::Directory(closed) =
                handle.get_item("closed/", cx).expect("closed should exist")
            else {
                panic!("closed should be a directory");
            };
            assert!(closed.is_expanded(cx));
            handle.cancel_drag(window, cx);

            assert!(handle.start_drag("a.txt", window, cx));
            assert!(handle.set_drop_target(
                Some(FileTreeDropTarget::directory("target/", "target/")),
                window,
                cx,
            ));
            assert!(matches!(
                handle.complete_drag(window, cx),
                Err(FileTreeDropError::Mutation(_))
            ));
            assert!(handle.get_item("a.txt", cx).is_some());
            assert!(handle.get_item("b.txt", cx).is_some());
            assert!(handle.get_item("target/a.txt", cx).is_some());
            assert!(handle.get_item("target/b.txt", cx).is_none());
        })
        .unwrap();

    assert_eq!(errors.borrow().len(), 1);
}

#[gpui::test]
fn hooks_reject_drag_and_drop_before_mutation(cx: &mut TestAppContext) {
    let config = FileTreeDragAndDropConfig::new()
        .can_drag(|paths| paths != ["blocked.txt"])
        .can_drop(|_| false);
    let handle = FileTreeHandle::new();
    let input =
        FileTreePreparedInput::prepare(["blocked.txt", "ok.txt", "target/item.txt"]).unwrap();
    let window = open_tree(cx, config, handle.clone(), input, Vec::new());

    window
        .update(cx, |_view, window, cx| {
            assert!(!handle.start_drag("blocked.txt", window, cx));
            assert!(handle.start_drag("ok.txt", window, cx));
            assert!(!handle.set_drop_target(
                Some(FileTreeDropTarget::directory("target/", "target/")),
                window,
                cx,
            ));
            assert!(handle.drop_target(cx).is_none());
            assert!(matches!(
                handle.complete_drag(window, cx),
                Err(FileTreeDropError::InvalidTarget)
            ));
            assert!(handle.get_item("ok.txt", cx).is_some());
        })
        .unwrap();
}

#[gpui::test]
fn native_blank_space_release_clears_target_and_drag_session(cx: &mut TestAppContext) {
    let handle = FileTreeHandle::new();
    let input =
        FileTreePreparedInput::prepare(["a.txt", "b.txt", "closed/child.txt", "target/item.txt"])
            .unwrap();
    let window = open_tree(
        cx,
        FileTreeDragAndDropConfig::new(),
        handle.clone(),
        input,
        Vec::new(),
    );
    {
        let mut visual = VisualTestContext::from_window(window.into(), cx);
        visual.simulate_mouse_down(
            point(px(20.0), px(12.0)),
            MouseButton::Left,
            Modifiers::default(),
        );
        visual.simulate_mouse_move(
            point(px(30.0), px(12.0)),
            Some(MouseButton::Left),
            Modifiers::default(),
        );
        visual.run_until_parked();
    }
    window
        .update(cx, |_view, _window, cx| {
            assert!(!handle.dragged_paths(cx).is_empty());
        })
        .unwrap();

    {
        let mut visual = VisualTestContext::from_window(window.into(), cx);
        visual.simulate_mouse_move(
            point(px(30.0), px(84.0)),
            Some(MouseButton::Left),
            Modifiers::default(),
        );
        visual.run_until_parked();
    }
    window
        .update(cx, |_view, _window, cx| {
            assert!(handle.drop_target(cx).is_some());
        })
        .unwrap();

    {
        let mut visual = VisualTestContext::from_window(window.into(), cx);
        visual.simulate_mouse_move(
            point(px(30.0), px(160.0)),
            Some(MouseButton::Left),
            Modifiers::default(),
        );
        visual.run_until_parked();
    }
    window
        .update(cx, |_view, _window, cx| {
            assert!(handle.drop_target(cx).is_none());
        })
        .unwrap();

    {
        let mut visual = VisualTestContext::from_window(window.into(), cx);
        visual.simulate_mouse_up(
            point(px(30.0), px(160.0)),
            MouseButton::Left,
            Modifiers::default(),
        );
        visual.run_until_parked();
    }
    window
        .update(cx, |_view, _window, cx| {
            assert!(handle.dragged_paths(cx).is_empty());
            assert!(handle.get_item("closed/", cx).is_some());
        })
        .unwrap();
}
