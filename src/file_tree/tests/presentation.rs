use std::{cell::RefCell, rc::Rc};

use gpui::{
    div, px, size, Context, InteractiveElement as _, IntoElement, Modifiers, MouseButton, Render,
    TestAppContext, VisualTestContext, Window,
};

use crate::file_tree::{
    FileTree, FileTreeContextMenuButtonVisibility, FileTreeContextMenuClose,
    FileTreeContextMenuComposition, FileTreeContextMenuSource, FileTreeGitStatus,
    FileTreeGitStatusEntry, FileTreeGitStatusPatch, FileTreeHandle, FileTreeIcon,
    FileTreeIconConfig, FileTreeIconSlot, FileTreePreparedInput, FileTreeRowDecorationContext,
    FileTreeRowStyleState, FileTreeStyleState,
};

struct PresentationTestView {
    close: Rc<RefCell<Option<FileTreeContextMenuClose>>>,
    decoration_states: Rc<RefCell<Vec<FileTreeRowDecorationContext>>>,
    handle: FileTreeHandle,
    header_renders: Rc<RefCell<usize>>,
    icon_slots: Rc<RefCell<Vec<FileTreeIconSlot>>>,
    input: FileTreePreparedInput,
    menu_opens: Rc<RefCell<Vec<(String, FileTreeContextMenuSource)>>>,
    row_states: Rc<RefCell<Vec<FileTreeRowStyleState>>>,
    tree_states: Rc<RefCell<Vec<FileTreeStyleState>>>,
}

impl Render for PresentationTestView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let header_renders = Rc::clone(&self.header_renders);
        let decoration_states = Rc::clone(&self.decoration_states);
        let icon_slots = Rc::clone(&self.icon_slots);
        let search_icon_slots = Rc::clone(&icon_slots);
        let file_icon_slots = Rc::clone(&icon_slots);
        let menu_opens = Rc::clone(&self.menu_opens);
        let close = Rc::clone(&self.close);
        let tree_states = Rc::clone(&self.tree_states);
        let row_states = Rc::clone(&self.row_states);

        let icons = FileTreeIconConfig::new()
            .remap(
                FileTreeIconSlot::Search,
                FileTreeIcon::Render(Rc::new(move |context, _, _| {
                    search_icon_slots.borrow_mut().push(context.slot);
                    div()
                        .debug_selector(|| "file-tree-test-search-icon".to_owned())
                        .into_any_element()
                })),
            )
            .remap(
                FileTreeIconSlot::File,
                FileTreeIcon::Render(Rc::new(move |context, _, _| {
                    file_icon_slots.borrow_mut().push(context.slot);
                    div()
                        .debug_selector(|| "file-tree-test-file-icon".to_owned())
                        .into_any_element()
                })),
            );
        let context_menu = FileTreeContextMenuComposition::new()
            .button_visibility(FileTreeContextMenuButtonVisibility::Always)
            .on_open(move |context, _, _| {
                menu_opens
                    .borrow_mut()
                    .push((context.item.path.clone(), context.source));
                *close.borrow_mut() = Some(context.close);
            })
            .render(|_, _, _| {
                Some(
                    div()
                        .debug_selector(|| "file-tree-test-context-menu".to_owned())
                        .into_any_element(),
                )
            });

        FileTree::new(self.input.clone())
            .search(true)
            .initial_search_query("README")
            .initial_selected_paths(["README.md"])
            .handle(self.handle.clone())
            .icons(icons)
            .git_status([FileTreeGitStatusEntry::new(
                "README.md",
                FileTreeGitStatus::Modified,
            )])
            .header(move |_, _| {
                *header_renders.borrow_mut() += 1;
                div()
                    .debug_selector(|| "file-tree-test-header".to_owned())
                    .into_any_element()
            })
            .row_decoration(move |context, _, _| {
                decoration_states.borrow_mut().push(context);
                Some(
                    div()
                        .debug_selector(|| "file-tree-test-decoration".to_owned())
                        .into_any_element(),
                )
            })
            .context_menu_config(context_menu)
            .style_with_state(move |state, base| {
                tree_states.borrow_mut().push(state);
                base
            })
            .row_style_with_state(move |state, base| {
                row_states.borrow_mut().push(state);
                base
            })
    }
}

#[gpui::test]
fn presentation_callbacks_context_menu_and_git_patches_compose(cx: &mut TestAppContext) {
    cx.update(crate::file_tree::init);
    let handle = FileTreeHandle::new();
    let close = Rc::new(RefCell::new(None));
    let decoration_states = Rc::new(RefCell::new(Vec::new()));
    let header_renders = Rc::new(RefCell::new(0));
    let icon_slots = Rc::new(RefCell::new(Vec::new()));
    let menu_opens = Rc::new(RefCell::new(Vec::new()));
    let row_states = Rc::new(RefCell::new(Vec::new()));
    let tree_states = Rc::new(RefCell::new(Vec::new()));
    let input = FileTreePreparedInput::prepare(["README.md", "src/lib.rs"]).unwrap();
    let window = cx.open_window(size(px(360.0), px(180.0)), {
        let close = Rc::clone(&close);
        let decoration_states = Rc::clone(&decoration_states);
        let handle = handle.clone();
        let header_renders = Rc::clone(&header_renders);
        let icon_slots = Rc::clone(&icon_slots);
        let menu_opens = Rc::clone(&menu_opens);
        let row_states = Rc::clone(&row_states);
        let tree_states = Rc::clone(&tree_states);
        move |_, _| PresentationTestView {
            close,
            decoration_states,
            handle,
            header_renders,
            icon_slots,
            input,
            menu_opens,
            row_states,
            tree_states,
        }
    });
    cx.run_until_parked();

    let mut visual = VisualTestContext::from_window(window.into(), cx);
    assert!(visual.debug_bounds("file-tree-test-header").is_some());
    assert!(visual.debug_bounds("file-tree-search-wrapper").is_some());
    assert!(visual.debug_bounds("file-tree-test-search-icon").is_some());
    assert!(visual.debug_bounds("file-tree-test-file-icon").is_some());
    assert!(visual.debug_bounds("file-tree-test-decoration").is_some());
    let row_bounds = visual
        .debug_bounds("file-tree-row-0")
        .expect("the filtered README row should render");
    visual.simulate_mouse_down(
        row_bounds.center(),
        MouseButton::Right,
        Modifiers::default(),
    );
    visual.simulate_mouse_up(
        row_bounds.center(),
        MouseButton::Right,
        Modifiers::default(),
    );
    visual.run_until_parked();
    drop(visual);
    cx.run_until_parked();

    assert!(*header_renders.borrow() > 0);
    assert!(icon_slots.borrow().contains(&FileTreeIconSlot::Search));
    assert!(icon_slots.borrow().contains(&FileTreeIconSlot::File));
    assert!(decoration_states.borrow().iter().any(|state| {
        state.item.path == "README.md"
            && state.row.path == "README.md"
            && state.row.position_in_set == 2
            && state.row.set_size == 2
    }));
    assert!(tree_states.borrow().iter().any(|state| state.search_open));
    assert!(row_states.borrow().iter().any(|state| {
        state.row.path == "README.md" && state.git_status == Some(FileTreeGitStatus::Modified)
    }));
    assert_eq!(
        menu_opens.borrow().as_slice(),
        &[(
            "README.md".to_owned(),
            FileTreeContextMenuSource::RightClick
        )]
    );
    assert_eq!(
        window
            .update(cx, |_, _, cx| handle.get_focused_path(cx))
            .unwrap()
            .as_deref(),
        Some("README.md")
    );

    cx.simulate_keystrokes(window.into(), "down");
    cx.run_until_parked();
    assert_eq!(
        window
            .update(cx, |_, _, cx| handle.get_focused_path(cx))
            .unwrap()
            .as_deref(),
        Some("README.md"),
        "tree key actions must be blocked while the menu is open"
    );

    let close_menu = close
        .borrow()
        .as_ref()
        .cloned()
        .expect("the menu open context must supply a close callback");
    window
        .update(cx, |_, window, cx| close_menu(true, window, cx))
        .unwrap();
    cx.run_until_parked();
    drop(close_menu);
    *close.borrow_mut() = None;

    window
        .update(cx, |_, window, cx| {
            let patch = FileTreeGitStatusPatch::new().with_set([FileTreeGitStatusEntry::new(
                "README.md",
                FileTreeGitStatus::Added,
            )]);
            assert!(handle.apply_git_status_patch(&patch, window, cx,));
        })
        .unwrap();
    cx.run_until_parked();
    assert!(row_states.borrow().iter().rev().any(|state| {
        state.row.path == "README.md" && state.git_status == Some(FileTreeGitStatus::Added)
    }));
}
