use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use gpui::{App, ScrollStrategy, Window};

use super::{
    context::FileTreeContext,
    drag_and_drop::{
        build_drop_plan, is_self_or_descendant_drop, FileTreeDropContext, FileTreeDropError,
        FileTreeDropResult, FileTreeDropTarget, FileTreeDropTargetKind,
    },
    mutation::{
        FileTreeBatchOperation, FileTreeMoveOptions, FileTreeMutationError, FileTreeMutationEvent,
        FileTreeMutationEventType, FileTreeRemoveOptions, FileTreeResetOptions,
    },
    rename::{
        public_rename_path, FileTreeRenameCommit, FileTreeRenamingItem,
        FileTreeStartRenamingOptions,
    },
    runtime::{FileTreeRuntime, FileTreeRuntimeUpdate},
    FileTreeGitStatusEntry, FileTreeGitStatusPatch, FileTreePreparedInput,
};

/// Alignment used by [`FileTreeHandle::scroll_to_path`].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FileTreeScrollOffset {
    Top,
    Center,
    #[default]
    Nearest,
}

/// Options for a programmatic path scroll.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FileTreeScrollToPathOptions {
    pub focus: bool,
    pub offset: FileTreeScrollOffset,
}

impl Default for FileTreeScrollToPathOptions {
    fn default() -> Self {
        Self {
            focus: true,
            offset: FileTreeScrollOffset::Nearest,
        }
    }
}

/// An imperative, path-first handle for a rendered [`super::FileTree`].
pub struct FileTreeHandle(Rc<RefCell<FileTreeHandleState>>);

impl Clone for FileTreeHandle {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl Default for FileTreeHandle {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(FileTreeHandleState::default())))
    }
}

#[derive(Default)]
struct FileTreeHandleState {
    context: Option<FileTreeContext>,
    items: HashMap<String, FileTreeItemHandle>,
    mutation_listeners: Vec<FileTreeMutationListener>,
    next_mutation_listener_id: u64,
}

#[derive(Clone)]
struct FileTreeMutationListener {
    callback: Rc<dyn Fn(&FileTreeMutationEvent)>,
    event_type: Option<FileTreeMutationEventType>,
    id: u64,
}

/// An active mutation subscription. Drop it or call [`Self::unsubscribe`] to stop delivery.
pub struct FileTreeMutationSubscription {
    listener_id: Option<u64>,
    tree: Weak<RefCell<FileTreeHandleState>>,
}

impl FileTreeMutationSubscription {
    pub fn unsubscribe(&mut self) {
        let Some(listener_id) = self.listener_id.take() else {
            return;
        };
        if let Some(tree) = self.tree.upgrade() {
            tree.borrow_mut()
                .mutation_listeners
                .retain(|listener| listener.id != listener_id);
        }
    }
}

impl Drop for FileTreeMutationSubscription {
    fn drop(&mut self) {
        self.unsubscribe();
    }
}

impl FileTreeHandle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_item(&self, path: &str, cx: &mut App) -> Option<FileTreeItemHandle> {
        let context = self.context()?;
        let (path, is_directory) =
            context.update_runtime_cache(cx, |runtime| runtime.item_descriptor(path))?;

        if let Some(item) = self.0.borrow().items.get(&path) {
            if item.is_directory() == is_directory {
                return Some(item.clone());
            }
        }

        let core = Rc::new(FileTreeItemHandleCore {
            tree: Rc::downgrade(&self.0),
            path: path.clone(),
        });
        let item = if is_directory {
            FileTreeItemHandle::Directory(FileTreeDirectoryHandle(core))
        } else {
            FileTreeItemHandle::File(FileTreeFileHandle(core))
        };
        self.0.borrow_mut().items.insert(path, item.clone());
        Some(item)
    }

    pub fn get_focused_item(&self, cx: &mut App) -> Option<FileTreeItemHandle> {
        let path = self.get_focused_path(cx)?;
        self.get_item(&path, cx)
    }

    pub fn get_focused_path(&self, cx: &App) -> Option<String> {
        self.context()?
            .read_runtime(cx, |runtime| runtime.focused_path().map(str::to_owned))
    }

    pub fn get_focused_index(&self, cx: &App) -> Option<usize> {
        self.context()?
            .read_runtime(cx, FileTreeRuntime::focused_index)
    }

    pub fn get_selected_paths(&self, cx: &App) -> Vec<String> {
        self.context()
            .map(|context| context.read_runtime(cx, |runtime| runtime.selected_paths().to_vec()))
            .unwrap_or_default()
    }

    pub fn get_visible_count(&self, cx: &App) -> usize {
        self.context()
            .map(|context| context.read_runtime(cx, FileTreeRuntime::visible_count))
            .unwrap_or(0)
    }

    pub fn get_search_value(&self, cx: &App) -> String {
        self.context()
            .map(|context| context.read_runtime(cx, |runtime| runtime.search_value().to_owned()))
            .unwrap_or_default()
    }

    pub fn is_search_open(&self, cx: &App) -> bool {
        self.context()
            .is_some_and(|context| context.read_runtime(cx, FileTreeRuntime::is_search_open))
    }

    pub fn get_search_matching_paths(&self, cx: &App) -> Vec<String> {
        self.context()
            .map(|context| {
                context.read_runtime(cx, |runtime| runtime.search_matching_paths().to_vec())
            })
            .unwrap_or_default()
    }

    pub fn set_git_status(
        &self,
        entries: impl IntoIterator<Item = FileTreeGitStatusEntry>,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let Some(context) = self.context() else {
            return false;
        };
        context.set_git_status(&entries.into_iter().collect::<Vec<_>>(), window, cx)
    }

    pub fn apply_git_status_patch(
        &self,
        patch: &FileTreeGitStatusPatch,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        self.context()
            .is_some_and(|context| context.apply_git_status_patch(patch, window, cx))
    }

    pub fn set_search(&self, value: Option<&str>, window: &mut Window, cx: &mut App) -> bool {
        self.update_runtime(window, cx, |runtime| runtime.set_search(value))
    }

    pub fn open_search(&self, window: &mut Window, cx: &mut App) -> bool {
        self.set_search(Some(""), window, cx)
    }

    pub fn open_search_with_value(
        &self,
        initial_value: &str,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        self.set_search(Some(initial_value), window, cx)
    }

    pub fn close_search(&self, window: &mut Window, cx: &mut App) -> bool {
        self.set_search(None, window, cx)
    }

    pub fn focus_next_search_match(&self, window: &mut Window, cx: &mut App) -> bool {
        self.update_runtime(window, cx, FileTreeRuntime::focus_next_search_match)
    }

    pub fn focus_previous_search_match(&self, window: &mut Window, cx: &mut App) -> bool {
        self.update_runtime(window, cx, FileTreeRuntime::focus_previous_search_match)
    }

    pub fn is_renaming(&self, cx: &App) -> bool {
        self.context()
            .is_some_and(|context| context.read_runtime(cx, FileTreeRuntime::is_renaming))
    }

    pub fn get_renaming_path(&self, cx: &App) -> Option<String> {
        self.context()?.read_runtime(cx, |runtime| {
            runtime.renaming_path().map(public_rename_path)
        })
    }

    pub fn start_renaming(&self, path: &str, window: &mut Window, cx: &mut App) -> bool {
        self.start_renaming_with_options(path, FileTreeStartRenamingOptions::default(), window, cx)
    }

    pub fn start_renaming_with_options(
        &self,
        path: &str,
        options: FileTreeStartRenamingOptions,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let Some(context) = self.context() else {
            return false;
        };
        let Some(config) = context.renaming_config() else {
            return false;
        };
        let Some((canonical_path, is_directory)) =
            context.update_runtime_cache(cx, |runtime| runtime.item_descriptor(path))
        else {
            return false;
        };
        let item = FileTreeRenamingItem {
            path: public_rename_path(&canonical_path),
            is_directory,
        };
        if !config.allows(&item) {
            return false;
        }

        context.update_runtime(window, cx, |runtime| {
            runtime.start_renaming(&canonical_path, options.remove_if_canceled)
        });
        if let Some(index) = context.update_runtime_cache(cx, |runtime| {
            runtime.visible_index_for_path(&canonical_path)
        }) {
            context.scroll_to_visible_index(index, ScrollStrategy::Nearest, false, window, cx);
        }
        window.refresh();
        true
    }

    pub fn start_renaming_focused(&self, window: &mut Window, cx: &mut App) -> bool {
        let Some(path) = self.get_focused_path(cx) else {
            return false;
        };
        self.start_renaming(&path, window, cx)
    }

    fn cancel_renaming(&self, window: &mut Window, cx: &mut App) -> bool {
        let Some(context) = self.context() else {
            return false;
        };
        let mut remove = None;
        let update = context.update_runtime(window, cx, |runtime| {
            let (update, removal) = runtime.cancel_renaming();
            remove = removal;
            update
        });
        let was_active = update.state_changed;
        if let Some((path, is_directory)) = remove {
            let operation = FileTreeBatchOperation::remove(
                path,
                FileTreeRemoveOptions {
                    recursive: is_directory,
                },
            );
            let _ = self.run_mutation(window, cx, move |runtime| {
                runtime.apply_mutation(&operation)
            });
        }
        if was_active {
            context.focus_tree(window, cx);
        }
        was_active
    }

    fn commit_renaming(&self, window: &mut Window, cx: &mut App) -> bool {
        let Some(context) = self.context() else {
            return false;
        };
        let Some(config) = context.renaming_config() else {
            return false;
        };
        let mut commit = None;
        context.update_runtime(window, cx, |runtime| {
            let (update, next_commit) = runtime.prepare_rename_commit();
            commit = next_commit;
            update
        });
        let Some(commit) = commit else {
            return false;
        };

        match commit {
            FileTreeRenameCommit::Remove {
                canonical_path,
                is_directory,
            } => {
                let operation = FileTreeBatchOperation::remove(
                    canonical_path,
                    FileTreeRemoveOptions {
                        recursive: is_directory,
                    },
                );
                if let Err(error) = self.run_mutation(window, cx, move |runtime| {
                    runtime.apply_mutation(&operation)
                }) {
                    config.emit_error(&error.into(), window, cx);
                }
            }
            FileTreeRenameCommit::Unchanged => {}
            FileTreeRenameCommit::Move(plan) => {
                config.emit_rename(&plan.event, window, cx);
                let operation = FileTreeBatchOperation::move_path(
                    plan.source_canonical_path,
                    plan.destination_canonical_path,
                    FileTreeMoveOptions::default(),
                );
                if let Err(error) = self.run_mutation(window, cx, move |runtime| {
                    runtime.apply_mutation(&operation)
                }) {
                    config.emit_error(&error.into(), window, cx);
                }
            }
            FileTreeRenameCommit::Error(error) => {
                config.emit_error(&error, window, cx);
            }
        }
        context.focus_tree(window, cx);
        true
    }

    fn set_renaming_value(&self, value: &str, window: &mut Window, cx: &mut App) -> bool {
        self.update_runtime(window, cx, |runtime| runtime.set_renaming_value(value))
    }

    pub fn is_drag_and_drop_enabled(&self) -> bool {
        self.context()
            .is_some_and(|context| context.drag_and_drop_config().is_some())
    }

    pub fn dragged_paths(&self, cx: &App) -> Vec<String> {
        self.context()
            .and_then(|context| {
                context.read_runtime(cx, |runtime| {
                    runtime
                        .drag_session()
                        .map(|session| session.dragged_paths.clone())
                })
            })
            .unwrap_or_default()
    }

    pub fn drop_target(&self, cx: &App) -> Option<FileTreeDropTarget> {
        self.context()?.read_runtime(cx, |runtime| {
            runtime
                .drag_session()
                .and_then(|session| session.target.clone())
        })
    }

    pub fn start_drag(&self, path: &str, window: &mut Window, cx: &mut App) -> bool {
        self.try_start_drag(path, window, cx).is_ok()
    }

    pub fn try_start_drag(
        &self,
        path: &str,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<(), FileTreeDropError> {
        let Some(context) = self.context() else {
            return Err(FileTreeDropError::NotBound);
        };
        let Some(config) = context.drag_and_drop_config() else {
            return Err(FileTreeDropError::Disabled);
        };
        if context.read_runtime(cx, |runtime| !runtime.search_value().is_empty()) {
            return Err(FileTreeDropError::SearchActive);
        }
        let Some((primary_path, dragged_paths)) =
            context.update_runtime_cache(cx, |runtime| runtime.prepare_drag_start(path))
        else {
            return Err(FileTreeDropError::InvalidSourcePath(path.to_owned()));
        };
        if !config.allows_drag(&dragged_paths) {
            return Err(FileTreeDropError::DragRejected);
        }
        context.update_runtime(window, cx, |runtime| {
            runtime.start_drag_session(primary_path, dragged_paths)
        });
        context.focus_tree(window, cx);
        Ok(())
    }

    pub fn set_drop_target(
        &self,
        target: Option<FileTreeDropTarget>,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let Some(context) = self.context() else {
            return false;
        };
        let Some(config) = context.drag_and_drop_config() else {
            return false;
        };
        let Some(session) = context.read_runtime(cx, |runtime| runtime.drag_session().cloned())
        else {
            return false;
        };
        let target = target.and_then(|target| {
            let normalized = self.normalize_drop_target(target, &context, cx)?;
            let drop_context = FileTreeDropContext {
                dragged_paths: session.dragged_paths.clone(),
                target: normalized.clone(),
            };
            (!is_self_or_descendant_drop(&session.dragged_paths, &normalized)
                && build_drop_plan(&session.dragged_paths, &normalized).is_some()
                && config.allows_drop(&drop_context))
            .then_some(normalized)
        });
        let accepted = target.is_some();
        let update = context.update_runtime(window, cx, |runtime| {
            runtime.set_drag_target(target.clone())
        });
        if update.state_changed {
            if let Some((generation, target)) = context.read_runtime(cx, |runtime| {
                runtime.drag_session().and_then(|session| {
                    session
                        .target
                        .clone()
                        .map(|target| (session.hover_generation, target))
                })
            }) {
                context.schedule_drag_hover_open(generation, target, window, cx);
            }
        }
        accepted
    }

    pub fn cancel_drag(&self, window: &mut Window, cx: &mut App) -> bool {
        self.update_runtime(window, cx, FileTreeRuntime::cancel_drag)
    }

    pub fn complete_drag(
        &self,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<FileTreeDropResult, FileTreeDropError> {
        let Some(context) = self.context() else {
            return Err(FileTreeDropError::NotBound);
        };
        let Some(config) = context.drag_and_drop_config() else {
            return Err(FileTreeDropError::Disabled);
        };
        let Some(session) = context.read_runtime(cx, |runtime| runtime.drag_session().cloned())
        else {
            return Err(FileTreeDropError::NoActiveDrag);
        };
        context.update_runtime(window, cx, FileTreeRuntime::cancel_drag);
        let Some(target) = session.target.clone() else {
            return Err(FileTreeDropError::InvalidTarget);
        };
        let drop_context = FileTreeDropContext {
            dragged_paths: session.dragged_paths.clone(),
            target: target.clone(),
        };
        if is_self_or_descendant_drop(&session.dragged_paths, &target)
            || !config.allows_drop(&drop_context)
        {
            return Err(FileTreeDropError::InvalidTarget);
        }
        let Some(plan) = build_drop_plan(&session.dragged_paths, &target) else {
            return Err(FileTreeDropError::NoMoveRequired);
        };
        let operations = plan.operations.clone();
        let mutation = if operations.len() == 1 {
            let operation = operations
                .into_iter()
                .next()
                .expect("one drop operation must exist");
            self.run_mutation(window, cx, move |runtime| {
                runtime.apply_mutation(&operation)
            })
        } else {
            self.run_mutation(window, cx, move |runtime| runtime.apply_batch(&operations))
        };
        if let Err(error) = mutation {
            let error = FileTreeDropError::Mutation(error);
            config.emit_error(&error, &drop_context, window, cx);
            return Err(error);
        }
        config.emit_complete(&plan.result, window, cx);
        Ok(plan.result)
    }

    fn normalize_drop_target(
        &self,
        mut target: FileTreeDropTarget,
        context: &FileTreeContext,
        cx: &mut App,
    ) -> Option<FileTreeDropTarget> {
        target.hovered_path = match target.hovered_path.as_deref() {
            Some(path) => Some(
                context
                    .update_runtime_cache(cx, |runtime| runtime.item_descriptor(path))?
                    .0,
            ),
            None => None,
        };
        match target.kind {
            FileTreeDropTargetKind::Root => {
                target.directory_path = None;
                target.flattened_segment_path = None;
            }
            FileTreeDropTargetKind::Directory => {
                let path = target.directory_path.as_deref()?;
                let (path, is_directory) =
                    context.update_runtime_cache(cx, |runtime| runtime.item_descriptor(path))?;
                if !is_directory {
                    return None;
                }
                target.directory_path = Some(path.clone());
                if target.flattened_segment_path.is_some() {
                    target.flattened_segment_path = Some(path);
                }
            }
        }
        Some(target)
    }

    pub fn focus_path(&self, path: &str, window: &mut Window, cx: &mut App) -> bool {
        self.update_runtime(window, cx, |runtime| runtime.focus_path(path))
    }

    pub fn focus_first_item(&self, window: &mut Window, cx: &mut App) -> bool {
        self.update_runtime(window, cx, FileTreeRuntime::focus_first_item)
    }

    pub fn focus_last_item(&self, window: &mut Window, cx: &mut App) -> bool {
        self.update_runtime(window, cx, FileTreeRuntime::focus_last_item)
    }

    pub fn focus_next_item(&self, window: &mut Window, cx: &mut App) -> bool {
        self.update_runtime(window, cx, FileTreeRuntime::focus_next_item)
    }

    pub fn focus_previous_item(&self, window: &mut Window, cx: &mut App) -> bool {
        self.update_runtime(window, cx, FileTreeRuntime::focus_previous_item)
    }

    pub fn focus_parent_item(&self, window: &mut Window, cx: &mut App) -> bool {
        self.update_runtime(window, cx, FileTreeRuntime::focus_parent_item)
    }

    pub fn scroll_to_path(&self, path: &str, window: &mut Window, cx: &mut App) -> bool {
        self.scroll_to_path_with_options(path, FileTreeScrollToPathOptions::default(), window, cx)
    }

    pub fn scroll_to_path_with_options(
        &self,
        path: &str,
        options: FileTreeScrollToPathOptions,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let Some(context) = self.context() else {
            return false;
        };
        let Some(index) =
            context.update_runtime_cache(cx, |runtime| runtime.visible_index_for_path(path))
        else {
            return false;
        };

        if options.focus {
            context.update_runtime(window, cx, |runtime| runtime.focus_visible_index(index));
        }
        let (strategy, strict) = match options.offset {
            FileTreeScrollOffset::Top => (ScrollStrategy::Top, true),
            FileTreeScrollOffset::Center => (ScrollStrategy::Center, true),
            FileTreeScrollOffset::Nearest => (ScrollStrategy::Nearest, false),
        };
        context.scroll_to_visible_index(index, strategy, strict, window, cx);
        window.refresh();
        true
    }

    pub fn add(
        &self,
        path: impl Into<String>,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<(), FileTreeMutationError> {
        let operation = FileTreeBatchOperation::add(path);
        self.run_mutation(window, cx, move |runtime| {
            runtime.apply_mutation(&operation)
        })
    }

    pub fn remove(
        &self,
        path: impl Into<String>,
        options: FileTreeRemoveOptions,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<(), FileTreeMutationError> {
        let operation = FileTreeBatchOperation::remove(path, options);
        self.run_mutation(window, cx, move |runtime| {
            runtime.apply_mutation(&operation)
        })
    }

    pub fn move_path(
        &self,
        from: impl Into<String>,
        to: impl Into<String>,
        options: FileTreeMoveOptions,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<(), FileTreeMutationError> {
        let operation = FileTreeBatchOperation::move_path(from, to, options);
        self.run_mutation(window, cx, move |runtime| {
            runtime.apply_mutation(&operation)
        })
    }

    pub fn batch(
        &self,
        operations: &[FileTreeBatchOperation],
        window: &mut Window,
        cx: &mut App,
    ) -> Result<(), FileTreeMutationError> {
        let operations = operations.to_vec();
        self.run_mutation(window, cx, move |runtime| runtime.apply_batch(&operations))
    }

    pub fn reset_paths(
        &self,
        paths: impl IntoIterator<Item = impl Into<String>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<(), FileTreeMutationError> {
        self.reset_paths_with_options(paths, FileTreeResetOptions::default(), window, cx)
    }

    pub fn reset_paths_with_options(
        &self,
        paths: impl IntoIterator<Item = impl Into<String>>,
        options: FileTreeResetOptions,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<(), FileTreeMutationError> {
        let raw_input = FileTreePreparedInput::prepare(paths)?;
        let used_prepared_input = options.prepared_input.is_some();
        let input = if let Some(prepared_input) = options.prepared_input {
            if raw_input.paths() != prepared_input.paths() {
                return Err(FileTreeMutationError::ResetInputMismatch);
            }
            prepared_input
        } else {
            raw_input
        };
        self.run_reset(
            input,
            options.initial_expanded_paths,
            used_prepared_input,
            window,
            cx,
        )
    }

    pub fn reset_prepared_input(
        &self,
        input: FileTreePreparedInput,
        initial_expanded_paths: Option<Vec<String>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<(), FileTreeMutationError> {
        self.run_reset(input, initial_expanded_paths, true, window, cx)
    }

    pub fn on_mutation(
        &self,
        event_type: FileTreeMutationEventType,
        handler: impl Fn(&FileTreeMutationEvent) + 'static,
    ) -> FileTreeMutationSubscription {
        self.add_mutation_listener(Some(event_type), Rc::new(handler))
    }

    pub fn on_any_mutation(
        &self,
        handler: impl Fn(&FileTreeMutationEvent) + 'static,
    ) -> FileTreeMutationSubscription {
        self.add_mutation_listener(None, Rc::new(handler))
    }

    fn add_mutation_listener(
        &self,
        event_type: Option<FileTreeMutationEventType>,
        callback: Rc<dyn Fn(&FileTreeMutationEvent)>,
    ) -> FileTreeMutationSubscription {
        let mut state = self.0.borrow_mut();
        let id = state.next_mutation_listener_id;
        state.next_mutation_listener_id += 1;
        state.mutation_listeners.push(FileTreeMutationListener {
            callback,
            event_type,
            id,
        });
        FileTreeMutationSubscription {
            listener_id: Some(id),
            tree: Rc::downgrade(&self.0),
        }
    }

    fn run_reset(
        &self,
        input: FileTreePreparedInput,
        initial_expanded_paths: Option<Vec<String>>,
        used_prepared_input: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<(), FileTreeMutationError> {
        self.run_mutation(window, cx, move |runtime| {
            runtime
                .reset_paths(input, initial_expanded_paths, used_prepared_input)
                .map(|(update, event)| (update, Some(event)))
        })
    }

    fn run_mutation(
        &self,
        window: &mut Window,
        cx: &mut App,
        mutation: impl FnOnce(
            &mut FileTreeRuntime,
        ) -> Result<
            (FileTreeRuntimeUpdate, Option<FileTreeMutationEvent>),
            FileTreeMutationError,
        >,
    ) -> Result<(), FileTreeMutationError> {
        let Some(context) = self.context() else {
            return Err(FileTreeMutationError::NotBound);
        };
        let mut result = None;
        context.update_runtime(window, cx, |runtime| match mutation(runtime) {
            Ok((update, event)) => {
                result = Some(Ok(event));
                update
            }
            Err(error) => {
                result = Some(Err(error));
                FileTreeRuntimeUpdate::default()
            }
        });
        let event = result.expect("mutation closure must set its result")?;
        if let Some(index) = context.update_runtime_cache(cx, |runtime| {
            let path = runtime.renaming_path()?.to_owned();
            runtime.visible_index_for_path(&path)
        }) {
            context.scroll_to_visible_index(index, ScrollStrategy::Nearest, false, window, cx);
            window.refresh();
        }
        if let Some(event) = event {
            if event.invalidation().canonical_changed {
                self.0.borrow_mut().items.clear();
            }
            self.emit_mutation(&event);
        }
        Ok(())
    }

    fn emit_mutation(&self, event: &FileTreeMutationEvent) {
        let event_type = event.event_type();
        let callbacks = {
            let state = self.0.borrow();
            state
                .mutation_listeners
                .iter()
                .filter(|listener| listener.event_type == Some(event_type))
                .chain(
                    state
                        .mutation_listeners
                        .iter()
                        .filter(|listener| listener.event_type.is_none()),
                )
                .map(|listener| Rc::clone(&listener.callback))
                .collect::<Vec<_>>()
        };
        for callback in callbacks {
            callback(event);
        }
    }

    fn bind(&self, context: FileTreeContext) {
        self.0.borrow_mut().context = Some(context);
    }

    fn context(&self) -> Option<FileTreeContext> {
        self.0.borrow().context.clone()
    }

    fn update_runtime(
        &self,
        window: &mut Window,
        cx: &mut App,
        update: impl FnOnce(&mut FileTreeRuntime) -> FileTreeRuntimeUpdate,
    ) -> bool {
        self.context()
            .map(|context| context.update_runtime(window, cx, update).state_changed)
            .unwrap_or(false)
    }
}

pub fn create_file_tree_handle() -> FileTreeHandle {
    FileTreeHandle::new()
}

pub fn bind_file_tree_handle(handle: &FileTreeHandle, context: FileTreeContext) {
    handle.bind(context);
}

pub fn file_tree_handle_identity(handle: &FileTreeHandle) -> usize {
    Rc::as_ptr(&handle.0) as usize
}

pub fn prepared_file_tree_dragged_paths(
    handle: &FileTreeHandle,
    path: &str,
    cx: &mut App,
) -> Option<Vec<String>> {
    let context = handle.context()?;
    let config = context.drag_and_drop_config()?;
    if !context.read_runtime(cx, |runtime| runtime.search_value().is_empty()) {
        return None;
    }
    let (_, paths) =
        context.update_runtime_cache(cx, |runtime| runtime.prepare_drag_start(path))?;
    config.allows_drag(&paths).then_some(paths)
}

pub fn cancel_file_tree_renaming(
    handle: &FileTreeHandle,
    window: &mut Window,
    cx: &mut App,
) -> bool {
    handle.cancel_renaming(window, cx)
}

pub fn commit_file_tree_renaming(
    handle: &FileTreeHandle,
    window: &mut Window,
    cx: &mut App,
) -> bool {
    handle.commit_renaming(window, cx)
}

pub fn set_file_tree_renaming_value(
    handle: &FileTreeHandle,
    value: &str,
    window: &mut Window,
    cx: &mut App,
) -> bool {
    handle.set_renaming_value(value, window, cx)
}

struct FileTreeItemHandleCore {
    tree: Weak<RefCell<FileTreeHandleState>>,
    path: String,
}

impl FileTreeItemHandleCore {
    fn context(&self) -> Option<FileTreeContext> {
        self.tree.upgrade()?.borrow().context.clone()
    }

    fn is_focused(&self, cx: &App) -> bool {
        self.context().is_some_and(|context| {
            context.read_runtime(cx, |runtime| {
                runtime.focused_path() == Some(self.path.as_str())
            })
        })
    }

    fn is_selected(&self, cx: &App) -> bool {
        self.context().is_some_and(|context| {
            context.read_runtime(cx, |runtime| runtime.is_selected(&self.path))
        })
    }

    fn update_runtime(
        &self,
        window: &mut Window,
        cx: &mut App,
        update: impl FnOnce(&mut FileTreeRuntime, &str) -> FileTreeRuntimeUpdate,
    ) -> bool {
        self.context()
            .map(|context| {
                context
                    .update_runtime(window, cx, |runtime| update(runtime, &self.path))
                    .state_changed
            })
            .unwrap_or(false)
    }

    fn is_expanded(&self, cx: &mut App) -> bool {
        self.context().is_some_and(|context| {
            context.update_runtime_cache(cx, |runtime| runtime.is_expanded(&self.path))
        })
    }
}

/// A cached path-first handle to a file-tree item.
#[derive(Clone)]
pub enum FileTreeItemHandle {
    Directory(FileTreeDirectoryHandle),
    File(FileTreeFileHandle),
}

impl FileTreeItemHandle {
    pub fn get_path(&self) -> &str {
        self.core().path.as_str()
    }

    pub fn is_directory(&self) -> bool {
        matches!(self, Self::Directory(_))
    }

    pub fn is_focused(&self, cx: &App) -> bool {
        self.core().is_focused(cx)
    }

    pub fn is_selected(&self, cx: &App) -> bool {
        self.core().is_selected(cx)
    }

    pub fn focus(&self, window: &mut Window, cx: &mut App) -> bool {
        self.core()
            .update_runtime(window, cx, FileTreeRuntime::focus_path)
    }

    pub fn select(&self, window: &mut Window, cx: &mut App) -> bool {
        self.core()
            .update_runtime(window, cx, FileTreeRuntime::select_path)
    }

    pub fn deselect(&self, window: &mut Window, cx: &mut App) -> bool {
        self.core()
            .update_runtime(window, cx, FileTreeRuntime::deselect_path)
    }

    pub fn toggle_select(&self, window: &mut Window, cx: &mut App) -> bool {
        self.core()
            .update_runtime(window, cx, FileTreeRuntime::toggle_path_selection)
    }

    fn core(&self) -> &Rc<FileTreeItemHandleCore> {
        match self {
            Self::Directory(handle) => &handle.0,
            Self::File(handle) => &handle.0,
        }
    }
}

macro_rules! impl_common_item_handle {
    ($handle:ident, $is_directory:expr) => {
        impl $handle {
            pub fn get_path(&self) -> &str {
                self.0.path.as_str()
            }

            pub fn is_directory(&self) -> bool {
                $is_directory
            }

            pub fn is_focused(&self, cx: &App) -> bool {
                self.0.is_focused(cx)
            }

            pub fn is_selected(&self, cx: &App) -> bool {
                self.0.is_selected(cx)
            }

            pub fn focus(&self, window: &mut Window, cx: &mut App) -> bool {
                self.0
                    .update_runtime(window, cx, FileTreeRuntime::focus_path)
            }

            pub fn select(&self, window: &mut Window, cx: &mut App) -> bool {
                self.0
                    .update_runtime(window, cx, FileTreeRuntime::select_path)
            }

            pub fn deselect(&self, window: &mut Window, cx: &mut App) -> bool {
                self.0
                    .update_runtime(window, cx, FileTreeRuntime::deselect_path)
            }

            pub fn toggle_select(&self, window: &mut Window, cx: &mut App) -> bool {
                self.0
                    .update_runtime(window, cx, FileTreeRuntime::toggle_path_selection)
            }
        }
    };
}

/// A path-first handle to a directory item.
#[derive(Clone)]
pub struct FileTreeDirectoryHandle(Rc<FileTreeItemHandleCore>);

impl_common_item_handle!(FileTreeDirectoryHandle, true);

impl FileTreeDirectoryHandle {
    pub fn is_expanded(&self, cx: &mut App) -> bool {
        self.0.is_expanded(cx)
    }

    pub fn expand(&self, window: &mut Window, cx: &mut App) -> bool {
        self.0.update_runtime(window, cx, FileTreeRuntime::expand)
    }

    pub fn collapse(&self, window: &mut Window, cx: &mut App) -> bool {
        self.0.update_runtime(window, cx, FileTreeRuntime::collapse)
    }

    pub fn toggle(&self, window: &mut Window, cx: &mut App) -> bool {
        self.0
            .update_runtime(window, cx, FileTreeRuntime::toggle_update)
    }
}

/// A path-first handle to a file item.
#[derive(Clone)]
pub struct FileTreeFileHandle(Rc<FileTreeItemHandleCore>);

impl_common_item_handle!(FileTreeFileHandle, false);

#[cfg(test)]
mod tests {
    use gpui::{
        div, point, px, size, Bounds, Context, FocusHandle, InteractiveElement as _, IntoElement,
        Modifiers, ParentElement as _, Pixels, Render, StatefulInteractiveElement as _,
        Styled as _, TestAppContext, VisualTestContext, Window, WindowHandle,
    };
    use std::time::Duration;

    use super::*;
    use crate::file_tree::{
        FileTree, FileTreeCollisionStrategy, FileTreeInitialExpansion, FileTreePreparedInput,
        FileTreeRenameError, FileTreeRenameEvent, FileTreeRenamingConfig,
        FileTreeStartRenamingOptions,
    };

    struct FileTreeHandleTestView {
        handle: FileTreeHandle,
        input: FileTreePreparedInput,
        initial_expansion: FileTreeInitialExpansion,
        row_height: Pixels,
    }

    impl Render for FileTreeHandleTestView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            FileTree::new(self.input.clone())
                .initial_expansion(self.initial_expansion)
                .row_height(self.row_height)
                .handle(self.handle.clone())
        }
    }

    struct FileTreeStickyTestView {
        handle: FileTreeHandle,
        input: FileTreePreparedInput,
    }

    impl Render for FileTreeStickyTestView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            FileTree::new(self.input.clone())
                .initial_expansion(FileTreeInitialExpansion::Open)
                .sticky_folders(true)
                .drag_and_drop(true)
                .row_height(px(30.0))
                .handle(self.handle.clone())
        }
    }

    fn open_tree(
        cx: &mut TestAppContext,
        handle: FileTreeHandle,
        input: FileTreePreparedInput,
        initial_expansion: FileTreeInitialExpansion,
        row_height: Pixels,
    ) -> WindowHandle<FileTreeHandleTestView> {
        cx.update(crate::file_tree::init);
        let window = cx.open_window(size(px(320.0), px(120.0)), move |_, _| {
            FileTreeHandleTestView {
                handle,
                input,
                initial_expansion,
                row_height,
            }
        });
        cx.run_until_parked();
        window
    }

    #[gpui::test]
    fn sticky_overlay_renders_separately_disables_for_search_and_reveals_clicks(
        cx: &mut TestAppContext,
    ) {
        cx.update(crate::file_tree::init);
        let handle = create_file_tree_handle();
        let input = FileTreePreparedInput::prepare(
            ["a/b/c/file.ts".to_owned()]
                .into_iter()
                .chain((0..10).map(|index| format!("z{index}.ts"))),
        )
        .unwrap();
        let window = cx.open_window(size(px(320.0), px(120.0)), {
            let handle = handle.clone();
            move |_, _| FileTreeStickyTestView { handle, input }
        });
        cx.run_until_parked();
        window
            .update(cx, |_view, _window, cx| {
                let context = handle.context().expect("handle should be bound");
                assert!(context.sticky_rows(cx).is_empty());
                let (preview_rows, visible) = context.sticky_overlay_rows(cx);
                assert!(!visible);
                assert_eq!(preview_rows.len(), 3);
            })
            .expect("sticky test window should be open");

        window
            .update(cx, |_view, window, _cx| {
                let context = handle.context().expect("handle should be bound");
                let base_handle = context.scroll_handle().0.borrow().base_handle.clone();
                base_handle.set_offset(point(px(0.0), px(-1.0)));
                context.schedule_sticky_refresh(window, _cx);
                window.refresh();
            })
            .expect("sticky test window should be open");
        cx.run_until_parked();
        window
            .update(cx, |_view, _window, cx| {
                let context = handle.context().expect("handle should be bound");
                assert_eq!(context.scroll_top(), 1.0);
                assert_eq!(context.sticky_rows(cx).len(), 3);
            })
            .expect("sticky test window should be open");

        let mut visual = VisualTestContext::from_window(window.into(), cx);
        assert!(visual.debug_bounds("file-tree-sticky-overlay").is_some());
        assert_eq!(
            f32::from(
                visual
                    .debug_bounds("file-tree-sticky-row-0")
                    .expect("top sticky row should render")
                    .top(),
            ),
            0.0
        );
        let second_sticky_bounds = visual
            .debug_bounds("file-tree-sticky-row-1")
            .expect("nested sticky row should render");
        assert_eq!(f32::from(second_sticky_bounds.top()), 30.0);
        drop(visual);

        window
            .update(cx, |_view, window, cx| {
                assert!(handle.set_search(Some("file"), window, cx));
            })
            .expect("sticky test window should be open");
        cx.run_until_parked();
        assert!(VisualTestContext::from_window(window.into(), cx)
            .debug_bounds("file-tree-sticky-overlay")
            .is_none());

        window
            .update(cx, |_view, window, cx| {
                assert!(handle.close_search(window, cx));
                let context = handle.context().expect("handle should be bound");
                context
                    .scroll_handle()
                    .0
                    .borrow()
                    .base_handle
                    .clone()
                    .set_offset(point(px(0.0), px(-1.0)));
                context.schedule_sticky_refresh(window, cx);
                window.refresh();
            })
            .expect("sticky test window should be open");
        cx.run_until_parked();
        let mut visual = VisualTestContext::from_window(window.into(), cx);
        let second_sticky_bounds = visual
            .debug_bounds("file-tree-sticky-row-1")
            .expect("sticky rows should return after search closes");
        visual.simulate_click(second_sticky_bounds.center(), Modifiers::default());
        visual.run_until_parked();
        drop(visual);
        cx.run_until_parked();

        window
            .update(cx, |_view, _window, cx| {
                assert_eq!(handle.get_focused_path(cx).as_deref(), Some("a/b/"));
                let context = handle.context().expect("handle should be bound");
                assert_eq!(context.scroll_top(), 0.0);
                assert!(context.sticky_rows(cx).is_empty());
                let (preview_rows, visible) = context.sticky_overlay_rows(cx);
                assert!(!visible);
                assert_eq!(
                    preview_rows
                        .iter()
                        .map(|entry| entry.row.path.as_str())
                        .collect::<Vec<_>>(),
                    vec!["a/"]
                );
                let FileTreeItemHandle::Directory(directory) =
                    handle.get_item("a/b/", cx).expect("directory should exist")
                else {
                    panic!("a/b must be a directory");
                };
                assert!(!directory.is_expanded(cx));
            })
            .expect("sticky test window should be open");
        cx.run_until_parked();
        let mut visual = VisualTestContext::from_window(window.into(), cx);
        assert!(visual.debug_bounds("file-tree-sticky-overlay").is_some());
        assert_eq!(
            f32::from(
                visual
                    .debug_bounds("file-tree-row-1")
                    .expect("canonical collapsed row should be revealed")
                    .top(),
            ),
            30.0
        );
    }

    #[gpui::test]
    fn sticky_scroll_to_path_places_the_canonical_row_below_all_pinned_parents(
        cx: &mut TestAppContext,
    ) {
        cx.update(crate::file_tree::init);
        let handle = create_file_tree_handle();
        let input = FileTreePreparedInput::prepare(
            ["a/file.ts".to_owned(), "m/n/o/file.ts".to_owned()]
                .into_iter()
                .chain((0..10).map(|index| format!("z{index}.ts"))),
        )
        .unwrap();
        let window = cx.open_window(size(px(320.0), px(120.0)), {
            let handle = handle.clone();
            move |_, _| FileTreeStickyTestView { handle, input }
        });
        cx.run_until_parked();

        window
            .update(cx, |_view, window, cx| {
                assert_eq!(handle.get_focused_path(cx).as_deref(), Some("a/"));
                assert!(handle.scroll_to_path_with_options(
                    "m/n/o/file.ts",
                    FileTreeScrollToPathOptions {
                        focus: false,
                        offset: FileTreeScrollOffset::Top,
                    },
                    window,
                    cx,
                ));
            })
            .expect("sticky scroll test window should be open");
        cx.run_until_parked();
        cx.run_until_parked();

        window
            .update(cx, |_view, _window, cx| {
                assert_eq!(handle.get_focused_path(cx).as_deref(), Some("a/"));
                let context = handle.context().expect("handle should be bound");
                assert_eq!(context.scroll_top(), 60.0);
                assert_eq!(
                    context
                        .sticky_rows(cx)
                        .iter()
                        .map(|entry| entry.row.path.as_str())
                        .collect::<Vec<_>>(),
                    vec!["m/", "m/n/", "m/n/o/"]
                );
            })
            .expect("sticky scroll test window should be open");
        let mut visual = VisualTestContext::from_window(window.into(), cx);
        assert_eq!(
            f32::from(
                visual
                    .debug_bounds("file-tree-row-5")
                    .expect("target row should stay in the flow list")
                    .top(),
            ),
            90.0
        );
    }

    #[gpui::test]
    fn overlapping_sticky_rows_route_clicks_to_the_outer_row(cx: &mut TestAppContext) {
        cx.update(crate::file_tree::init);
        let handle = create_file_tree_handle();
        let input = FileTreePreparedInput::prepare(
            ["a/b/c/file.ts".to_owned()]
                .into_iter()
                .chain((0..10).map(|index| format!("z{index}.ts"))),
        )
        .unwrap();
        let window = cx.open_window(size(px(320.0), px(120.0)), {
            let handle = handle.clone();
            move |_, _| FileTreeStickyTestView { handle, input }
        });
        cx.run_until_parked();

        window
            .update(cx, |_view, window, cx| {
                let context = handle.context().expect("handle should be bound");
                context
                    .scroll_handle()
                    .0
                    .borrow()
                    .base_handle
                    .clone()
                    .set_offset(point(px(0.0), px(-61.0)));
                context.schedule_sticky_refresh(window, cx);
                window.refresh();
            })
            .expect("sticky overlap test window should be open");
        cx.run_until_parked();

        window
            .update(cx, |_view, _window, cx| {
                let context = handle.context().expect("handle should be bound");
                let sticky_rows = context.sticky_rows(cx);
                assert_eq!(
                    sticky_rows
                        .iter()
                        .map(|entry| (entry.row.path.as_str(), entry.top))
                        .collect::<Vec<_>>(),
                    vec![("a/", 0.0), ("a/b/", 29.0), ("a/b/c/", 29.0)]
                );
            })
            .expect("sticky overlap test window should be open");

        let mut visual = VisualTestContext::from_window(window.into(), cx);
        let outer_bounds = visual
            .debug_bounds("file-tree-sticky-row-1")
            .expect("outer overlapping sticky row should render");
        let inner_bounds = visual
            .debug_bounds("file-tree-sticky-row-2")
            .expect("inner overlapping sticky row should render");
        assert_eq!(outer_bounds, inner_bounds);
        visual.simulate_click(outer_bounds.center(), Modifiers::default());
        visual.run_until_parked();
        drop(visual);
        cx.run_until_parked();

        window
            .update(cx, |_view, _window, cx| {
                assert_eq!(handle.get_focused_path(cx).as_deref(), Some("a/b/"));
                let FileTreeItemHandle::Directory(directory) = handle
                    .get_item("a/b/", cx)
                    .expect("outer overlapping directory should exist")
                else {
                    panic!("a/b must be a directory");
                };
                assert!(!directory.is_expanded(cx));
            })
            .expect("sticky overlap test window should be open");
    }

    #[gpui::test]
    fn continuous_drag_auto_scroll_refreshes_sticky_push_off(cx: &mut TestAppContext) {
        cx.update(crate::file_tree::init);
        let handle = create_file_tree_handle();
        let input = FileTreePreparedInput::prepare(
            ["a/file.ts".to_owned(), "b/file.ts".to_owned()]
                .into_iter()
                .chain((0..10).map(|index| format!("z{index}.ts"))),
        )
        .unwrap();
        let window = cx.open_window(size(px(320.0), px(120.0)), {
            let handle = handle.clone();
            move |_, _| FileTreeStickyTestView { handle, input }
        });
        cx.run_until_parked();

        window
            .update(cx, |_view, window, cx| {
                let context = handle.context().expect("handle should be bound");
                context
                    .scroll_handle()
                    .0
                    .borrow()
                    .base_handle
                    .clone()
                    .set_offset(point(px(0.0), px(-1.0)));
                context.schedule_sticky_refresh(window, cx);
                assert!(handle.start_drag("z0.ts", window, cx));
                assert!(context.drag_auto_scroll(
                    px(120.0),
                    Bounds {
                        origin: point(px(0.0), px(0.0)),
                        size: size(px(320.0), px(120.0)),
                    },
                    window,
                    cx,
                ));
            })
            .expect("sticky drag-scroll test window should be open");
        cx.run_until_parked();
        let mut visual = VisualTestContext::from_window(window.into(), cx);
        assert_eq!(
            f32::from(
                visual
                    .debug_bounds("file-tree-sticky-row-0")
                    .expect("sticky row should render during drag scrolling")
                    .top(),
            ),
            0.0
        );
        drop(visual);

        cx.executor().advance_clock(Duration::from_millis(16));
        cx.run_until_parked();
        cx.run_until_parked();
        window
            .update(cx, |_view, window, cx| {
                let context = handle.context().expect("handle should be bound");
                assert_eq!(context.scroll_top(), 37.0);
                context.stop_drag_auto_scroll(cx);
                handle.cancel_drag(window, cx);
            })
            .expect("sticky drag-scroll test window should be open");
        let mut visual = VisualTestContext::from_window(window.into(), cx);
        assert_eq!(
            f32::from(
                visual
                    .debug_bounds("file-tree-sticky-row-0")
                    .expect("sticky row should refresh after the timer tick")
                    .top(),
            ),
            -7.0
        );
    }

    #[gpui::test]
    fn public_handle_exposes_cached_path_items_and_live_commands(cx: &mut TestAppContext) {
        let handle = create_file_tree_handle();
        let input =
            FileTreePreparedInput::prepare(["README.md", "src/index.ts", "src/lib/util.ts"])
                .unwrap();
        let window = open_tree(
            cx,
            handle.clone(),
            input,
            FileTreeInitialExpansion::Depth(1),
            px(30.0),
        );

        window
            .update(cx, |_view, window, cx| {
                let src = handle.get_item("src", cx).expect("src should exist");
                let src_again = handle.get_item("src/", cx).expect("src should exist");
                assert!(Rc::ptr_eq(src.core(), src_again.core()));
                assert_eq!(src.get_path(), "src/");
                assert!(src.is_directory());
                assert!(src.is_focused(cx));
                assert!(!src.is_selected(cx));
                assert!(handle.get_item("missing.ts", cx).is_none());

                let readme = handle
                    .get_item("README.md", cx)
                    .expect("README should exist");
                assert!(!readme.is_directory());
                assert!(readme.focus(window, cx));
                assert_eq!(handle.get_focused_path(cx).as_deref(), Some("README.md"));
                assert!(readme.select(window, cx));
                assert!(src.select(window, cx));
                assert_eq!(
                    handle.get_selected_paths(cx),
                    vec!["README.md".to_owned(), "src/".to_owned()]
                );
                assert!(src.toggle_select(window, cx));
                assert_eq!(handle.get_selected_paths(cx), vec!["README.md".to_owned()]);
                assert!(readme.deselect(window, cx));
                assert!(handle.get_selected_paths(cx).is_empty());

                let FileTreeItemHandle::Directory(src_directory) = src else {
                    panic!("src should be a directory handle");
                };
                let FileTreeItemHandle::Directory(lib_directory) = handle
                    .get_item("src/lib", cx)
                    .expect("src/lib should exist")
                else {
                    panic!("src/lib should be a directory handle");
                };
                assert!(src_directory.collapse(window, cx));
                assert!(lib_directory.focus(window, cx));
                assert_eq!(handle.get_focused_path(cx).as_deref(), Some("src/"));
                assert!(!handle.scroll_to_path("src/lib/util.ts", window, cx));
                assert!(!handle.scroll_to_path("missing.ts", window, cx));
                assert_eq!(handle.get_focused_path(cx).as_deref(), Some("src/"));
                assert!(lib_directory.expand(window, cx));
                assert!(src_directory.is_expanded(cx));
                assert!(lib_directory.is_expanded(cx));
                assert_eq!(handle.get_visible_count(cx), 5);
                assert!(lib_directory.focus(window, cx));
                assert_eq!(handle.get_focused_path(cx).as_deref(), Some("src/lib/"));

                assert!(handle.focus_last_item(window, cx));
                assert_eq!(handle.get_focused_path(cx).as_deref(), Some("README.md"));
                assert!(!handle.focus_next_item(window, cx));
                assert!(handle.focus_previous_item(window, cx));
                assert_eq!(handle.get_focused_path(cx).as_deref(), Some("src/index.ts"));
                assert!(handle.focus_parent_item(window, cx));
                assert_eq!(handle.get_focused_path(cx).as_deref(), Some("src/"));
                assert!(!handle.focus_first_item(window, cx));
            })
            .expect("file-tree test window should be open");
    }

    #[gpui::test]
    fn scroll_to_path_supports_focus_control_and_all_offsets(cx: &mut TestAppContext) {
        let handle = create_file_tree_handle();
        let input =
            FileTreePreparedInput::prepare((0..120).map(|index| format!("item{index:03}.ts")))
                .unwrap();
        let window = open_tree(
            cx,
            handle.clone(),
            input,
            FileTreeInitialExpansion::Closed,
            px(30.0),
        );

        window
            .update(cx, |_view, window, cx| {
                let tree_focus = handle
                    .context()
                    .expect("handle should be bound")
                    .focus_handle()
                    .clone();
                assert!(!tree_focus.is_focused(window));
                assert!(handle.scroll_to_path_with_options(
                    "item080.ts",
                    FileTreeScrollToPathOptions {
                        focus: false,
                        offset: FileTreeScrollOffset::Center,
                    },
                    window,
                    cx,
                ));
                assert_eq!(handle.get_focused_path(cx).as_deref(), Some("item000.ts"));
                assert!(!tree_focus.is_focused(window));
            })
            .expect("file-tree test window should be open");
        cx.run_until_parked();
        assert_eq!(scroll_top(&handle), px(2355.0));

        window
            .update(cx, |_view, window, cx| {
                assert!(handle.focus_path("item010.ts", window, cx));
                assert_eq!(handle.get_focused_path(cx).as_deref(), Some("item010.ts"));
            })
            .expect("file-tree test window should be open");
        cx.run_until_parked();
        assert_eq!(scroll_top(&handle), px(2355.0));

        window
            .update(cx, |_view, window, cx| {
                assert!(handle.scroll_to_path_with_options(
                    "item010.ts",
                    FileTreeScrollToPathOptions {
                        focus: true,
                        offset: FileTreeScrollOffset::Top,
                    },
                    window,
                    cx,
                ));
                assert_eq!(handle.get_focused_path(cx).as_deref(), Some("item010.ts"));
            })
            .expect("file-tree test window should be open");
        cx.run_until_parked();
        assert_eq!(scroll_top(&handle), px(300.0));

        window
            .update(cx, |_view, window, cx| {
                assert!(handle.scroll_to_path_with_options(
                    "item012.ts",
                    FileTreeScrollToPathOptions {
                        focus: false,
                        offset: FileTreeScrollOffset::Nearest,
                    },
                    window,
                    cx,
                ));
            })
            .expect("file-tree test window should be open");
        cx.run_until_parked();
        assert_eq!(scroll_top(&handle), px(300.0));

        window
            .update(cx, |_view, window, cx| {
                assert!(handle.scroll_to_path_with_options(
                    "item119.ts",
                    FileTreeScrollToPathOptions {
                        focus: true,
                        offset: FileTreeScrollOffset::Center,
                    },
                    window,
                    cx,
                ));
            })
            .expect("file-tree test window should be open");
        cx.run_until_parked();
        assert_eq!(scroll_top(&handle), px(3480.0));

        window
            .update(cx, |_view, window, cx| {
                assert!(handle.scroll_to_path("item000.ts", window, cx));
                assert_eq!(handle.get_focused_path(cx).as_deref(), Some("item000.ts"));
            })
            .expect("file-tree test window should be open");
        cx.run_until_parked();
        assert_eq!(scroll_top(&handle), px(0.0));
    }

    #[gpui::test]
    fn mutations_repair_state_keep_new_directories_closed_and_invalidate_handles(
        cx: &mut TestAppContext,
    ) {
        let handle = create_file_tree_handle();
        let input = FileTreePreparedInput::prepare(["README.md", "src/index.ts"]).unwrap();
        let window = open_tree(
            cx,
            handle.clone(),
            input,
            FileTreeInitialExpansion::Open,
            px(30.0),
        );

        window
            .update(cx, |_view, window, cx| {
                let FileTreeItemHandle::Directory(src) =
                    handle.get_item("src", cx).expect("src should exist")
                else {
                    panic!("src should be a directory");
                };
                assert!(src.collapse(window, cx));
                handle.add("src/utils.ts", window, cx).unwrap();
                assert_eq!(handle.get_visible_count(cx), 2);
                assert!(src.expand(window, cx));
                assert_eq!(handle.get_visible_count(cx), 4);

                let old_utils = handle
                    .get_item("src/utils.ts", cx)
                    .expect("added file should exist");
                assert!(old_utils.focus(window, cx));
                assert!(old_utils.select(window, cx));
                handle
                    .move_path(
                        "src/utils.ts",
                        "src/helpers.ts",
                        FileTreeMoveOptions::default(),
                        window,
                        cx,
                    )
                    .unwrap();
                assert_eq!(
                    handle.get_focused_path(cx).as_deref(),
                    Some("src/helpers.ts")
                );
                assert_eq!(handle.get_selected_paths(cx), vec!["src/helpers.ts"]);
                assert!(handle.get_item("src/utils.ts", cx).is_none());
                let new_helpers = handle
                    .get_item("src/helpers.ts", cx)
                    .expect("moved file should exist");
                assert!(!Rc::ptr_eq(old_utils.core(), new_helpers.core()));

                handle.add("z.ts", window, cx).unwrap();
                let z = handle.get_item("z.ts", cx).unwrap();
                assert!(z.focus(window, cx));
                handle
                    .remove("z.ts", FileTreeRemoveOptions::default(), window, cx)
                    .unwrap();
                assert_eq!(handle.get_focused_path(cx).as_deref(), Some("src/"));

                handle
                    .remove("src", FileTreeRemoveOptions { recursive: true }, window, cx)
                    .unwrap();
                assert!(handle.get_selected_paths(cx).is_empty());
                assert_eq!(handle.get_focused_path(cx).as_deref(), Some("README.md"));
                assert!(handle.get_item("src/", cx).is_none());
            })
            .expect("file-tree test window should be open");
    }

    #[gpui::test]
    fn move_supports_destination_directories_collisions_and_descendant_remapping(
        cx: &mut TestAppContext,
    ) {
        let handle = create_file_tree_handle();
        let input = FileTreePreparedInput::prepare([
            "README.md",
            "docs/readme.md",
            "src/index.ts",
            "src/utils.ts",
            "tmp/",
        ])
        .unwrap();
        let window = open_tree(
            cx,
            handle.clone(),
            input,
            FileTreeInitialExpansion::Open,
            px(30.0),
        );

        window
            .update(cx, |_view, window, cx| {
                assert!(handle.focus_path("docs/readme.md", window, cx));
                handle
                    .get_item("docs/readme.md", cx)
                    .unwrap()
                    .select(window, cx);
                handle
                    .move_path(
                        "docs/readme.md",
                        "tmp/",
                        FileTreeMoveOptions::default(),
                        window,
                        cx,
                    )
                    .unwrap();
                assert_eq!(
                    handle.get_focused_path(cx).as_deref(),
                    Some("tmp/readme.md")
                );
                assert_eq!(handle.get_selected_paths(cx), vec!["tmp/readme.md"]);

                assert!(handle.focus_path("src/index.ts", window, cx));
                handle
                    .get_item("src/index.ts", cx)
                    .unwrap()
                    .select(window, cx);
                handle
                    .move_path("src", "lib/", FileTreeMoveOptions::default(), window, cx)
                    .unwrap();
                assert_eq!(handle.get_focused_path(cx).as_deref(), Some("lib/index.ts"));
                assert!(handle
                    .get_selected_paths(cx)
                    .contains(&"lib/index.ts".to_owned()));

                handle.add("target.ts", window, cx).unwrap();
                handle
                    .move_path(
                        "lib/",
                        "target.ts/",
                        FileTreeMoveOptions {
                            collision: FileTreeCollisionStrategy::Skip,
                        },
                        window,
                        cx,
                    )
                    .unwrap();
                assert!(handle.get_item("lib/", cx).is_some());
                assert_eq!(
                    handle
                        .move_path(
                            "lib/",
                            "target.ts",
                            FileTreeMoveOptions::default(),
                            window,
                            cx,
                        )
                        .unwrap_err(),
                    FileTreeMutationError::DestinationAlreadyExists("target.ts".to_owned())
                );
                assert_eq!(
                    handle
                        .move_path(
                            "lib/",
                            "target.ts",
                            FileTreeMoveOptions {
                                collision: FileTreeCollisionStrategy::Replace,
                            },
                            window,
                            cx,
                        )
                        .unwrap_err(),
                    FileTreeMutationError::ReplaceRequiresSameKind
                );
                handle
                    .move_path(
                        "README.md",
                        "target.ts",
                        FileTreeMoveOptions {
                            collision: FileTreeCollisionStrategy::Skip,
                        },
                        window,
                        cx,
                    )
                    .unwrap();
                assert!(handle.get_item("README.md", cx).is_some());
                handle
                    .move_path(
                        "README.md",
                        "target.ts",
                        FileTreeMoveOptions {
                            collision: FileTreeCollisionStrategy::Replace,
                        },
                        window,
                        cx,
                    )
                    .unwrap();
                assert!(handle.get_item("README.md", cx).is_none());
                assert!(handle.get_item("target.ts", cx).is_some());
            })
            .expect("file-tree test window should be open");
    }

    #[gpui::test]
    fn batch_is_atomic_and_subscriptions_are_filtered_ordered_and_synchronous(
        cx: &mut TestAppContext,
    ) {
        let handle = create_file_tree_handle();
        let input = FileTreePreparedInput::prepare(["README.md", "src/old.ts"]).unwrap();
        let window = open_tree(
            cx,
            handle.clone(),
            input,
            FileTreeInitialExpansion::Open,
            px(30.0),
        );
        let calls = Rc::new(RefCell::new(Vec::new()));
        let wildcard_calls = Rc::clone(&calls);
        let _wildcard = handle.on_any_mutation(move |event| {
            wildcard_calls
                .borrow_mut()
                .push(format!("wildcard:{:?}", event.event_type()));
        });
        let batch_events = Rc::new(RefCell::new(Vec::new()));
        let captured_batch_events = Rc::clone(&batch_events);
        let typed_calls = Rc::clone(&calls);
        let _typed = handle.on_mutation(FileTreeMutationEventType::Batch, move |event| {
            typed_calls
                .borrow_mut()
                .push(format!("typed:{:?}", event.event_type()));
            captured_batch_events.borrow_mut().push(event.clone());
        });

        window
            .update(cx, |_view, window, cx| {
                handle
                    .batch(
                        &[
                            FileTreeBatchOperation::add("src/new.ts"),
                            FileTreeBatchOperation::move_path(
                                "src/new.ts",
                                "src/renamed.ts",
                                FileTreeMoveOptions::default(),
                            ),
                            FileTreeBatchOperation::remove(
                                "src/old.ts",
                                FileTreeRemoveOptions::default(),
                            ),
                        ],
                        window,
                        cx,
                    )
                    .unwrap();
                assert_eq!(calls.borrow().as_slice(), ["typed:Batch", "wildcard:Batch"]);
                assert!(handle.get_item("src/renamed.ts", cx).is_some());
                assert!(handle.get_item("src/old.ts", cx).is_none());

                let before_noop_batch = handle.get_item("src/renamed.ts", cx).unwrap();
                handle.batch(&[], window, cx).unwrap();
                assert_eq!(
                    calls.borrow().as_slice(),
                    [
                        "typed:Batch",
                        "wildcard:Batch",
                        "typed:Batch",
                        "wildcard:Batch"
                    ]
                );
                let after_noop_batch = handle.get_item("src/renamed.ts", cx).unwrap();
                assert!(Rc::ptr_eq(
                    before_noop_batch.core(),
                    after_noop_batch.core()
                ));

                let error = handle
                    .batch(
                        &[
                            FileTreeBatchOperation::add("src/transient.ts"),
                            FileTreeBatchOperation::remove(
                                "missing.ts",
                                FileTreeRemoveOptions::default(),
                            ),
                        ],
                        window,
                        cx,
                    )
                    .unwrap_err();
                assert_eq!(
                    error,
                    FileTreeMutationError::PathDoesNotExist("missing.ts".to_owned())
                );
                assert!(handle.get_item("src/transient.ts", cx).is_none());
                assert_eq!(calls.borrow().len(), 4);

                handle
                    .batch(
                        &[
                            FileTreeBatchOperation::add("src/transient.ts"),
                            FileTreeBatchOperation::remove(
                                "src/transient.ts",
                                FileTreeRemoveOptions::default(),
                            ),
                        ],
                        window,
                        cx,
                    )
                    .unwrap();
                let events = batch_events.borrow();
                let FileTreeMutationEvent::Batch(event) = events.last().unwrap() else {
                    panic!("expected batch event");
                };
                assert!(event.invalidation.projection_changed);
                assert_eq!(event.invalidation.visible_count_delta, Some(0));
            })
            .expect("file-tree test window should be open");
    }

    #[gpui::test]
    fn reset_accepts_prepared_input_repairs_state_and_emits_metadata(cx: &mut TestAppContext) {
        let handle = create_file_tree_handle();
        let input = FileTreePreparedInput::prepare(["README.md", "src/index.ts"]).unwrap();
        let window = open_tree(
            cx,
            handle.clone(),
            input,
            FileTreeInitialExpansion::Closed,
            px(30.0),
        );
        let events = Rc::new(RefCell::new(Vec::new()));
        let event_log = Rc::clone(&events);
        let _subscription = handle.on_mutation(FileTreeMutationEventType::Reset, move |event| {
            event_log.borrow_mut().push(event.clone());
        });

        window
            .update(cx, |_view, window, cx| {
                let FileTreeItemHandle::Directory(src_directory) =
                    handle.get_item("src/", cx).unwrap()
                else {
                    panic!("expected src directory");
                };
                assert!(src_directory.expand(window, cx));
                let src = handle.get_item("src/index.ts", cx).unwrap();
                assert!(src.focus(window, cx));
                assert!(src.select(window, cx));
                let prepared =
                    FileTreePreparedInput::prepare(["README.md", "docs/guide.md"]).unwrap();
                handle
                    .reset_paths_with_options(
                        ["docs/guide.md", "README.md"],
                        FileTreeResetOptions {
                            prepared_input: Some(prepared),
                            initial_expanded_paths: Some(vec!["docs/".to_owned()]),
                        },
                        window,
                        cx,
                    )
                    .unwrap();
                assert!(handle.get_item("src/index.ts", cx).is_none());
                assert!(handle.get_item("docs/guide.md", cx).is_some());
                assert!(handle.get_selected_paths(cx).is_empty());
                assert_eq!(handle.get_focused_path(cx).as_deref(), Some("docs/"));
                {
                    let event = events.borrow();
                    let FileTreeMutationEvent::Reset(event) = &event[0] else {
                        panic!("expected reset event");
                    };
                    assert_eq!(event.path_count_before, 2);
                    assert_eq!(event.path_count_after, 2);
                    assert!(event.used_prepared_input);
                    assert!(event.invalidation.projection_changed);
                }

                let prepared =
                    FileTreePreparedInput::prepare(["README.md", "docs/guide.md"]).unwrap();
                handle
                    .reset_prepared_input(prepared, None, window, cx)
                    .unwrap();
                let FileTreeItemHandle::Directory(docs) = handle.get_item("docs/", cx).unwrap()
                else {
                    panic!("expected docs directory");
                };
                assert!(!docs.is_expanded(cx));
                assert_eq!(handle.get_visible_count(cx), 2);
                assert_eq!(events.borrow().len(), 2);
            })
            .expect("file-tree test window should be open");
        cx.run_until_parked();
        window
            .update(cx, |_view, _window, cx| {
                assert!(handle.get_item("docs/guide.md", cx).is_some());
            })
            .expect("file-tree test window should be open");
    }

    struct FileTreeSearchTestView {
        blur_behavior: super::super::FileTreeSearchBlurBehavior,
        changes: Rc<RefCell<Vec<Option<String>>>>,
        handle: FileTreeHandle,
        initial_query: Option<String>,
        input: FileTreePreparedInput,
        outside_focus: FocusHandle,
        search_enabled: bool,
    }

    impl Render for FileTreeSearchTestView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            let changes = Rc::clone(&self.changes);
            let mut tree = FileTree::new(self.input.clone())
                .id("file-tree-search-test")
                .initial_expansion(FileTreeInitialExpansion::Open)
                .search(self.search_enabled)
                .search_blur_behavior(self.blur_behavior)
                .on_search_change(move |value, _window, _cx| {
                    changes.borrow_mut().push(value.map(str::to_owned));
                })
                .handle(self.handle.clone());
            if let Some(query) = self.initial_query.clone() {
                tree = tree.initial_search_query(query);
            }
            div().size_full().child(tree).child(
                div()
                    .id("file-tree-search-outside")
                    .track_focus(&self.outside_focus)
                    .focusable(),
            )
        }
    }

    fn open_search_tree(
        cx: &mut TestAppContext,
        handle: FileTreeHandle,
        search_enabled: bool,
        initial_query: Option<&str>,
        blur_behavior: super::super::FileTreeSearchBlurBehavior,
        changes: Rc<RefCell<Vec<Option<String>>>>,
    ) -> WindowHandle<FileTreeSearchTestView> {
        cx.update(crate::file_tree::init);
        cx.update(crate::primitives::input::init);
        let input = FileTreePreparedInput::prepare([
            "README.md",
            "src/utils/stream.ts",
            "src/utils/worker-a.ts",
            "src/utils/worker-b.ts",
        ])
        .unwrap();
        let initial_query = initial_query.map(str::to_owned);
        let window = cx.open_window(size(px(320.0), px(160.0)), move |_, cx| {
            FileTreeSearchTestView {
                blur_behavior,
                changes,
                handle,
                initial_query,
                input,
                outside_focus: cx.focus_handle(),
                search_enabled,
            }
        });
        cx.run_until_parked();
        cx.run_until_parked();
        window
    }

    fn search_input_is_rendered(
        cx: &mut TestAppContext,
        window: WindowHandle<FileTreeSearchTestView>,
    ) -> bool {
        VisualTestContext::from_window(window.into(), cx)
            .debug_bounds("file-tree-search-input")
            .is_some()
    }

    #[gpui::test]
    fn built_in_search_hotkey_controls_input_navigation_selection_and_escape(
        cx: &mut TestAppContext,
    ) {
        let handle = create_file_tree_handle();
        let changes = Rc::new(RefCell::new(Vec::new()));
        let window = open_search_tree(
            cx,
            handle.clone(),
            true,
            None,
            super::super::FileTreeSearchBlurBehavior::Close,
            Rc::clone(&changes),
        );

        window
            .update(cx, |_view, window, cx| {
                handle
                    .context()
                    .expect("handle should be bound")
                    .focus_tree(window, cx);
            })
            .expect("file-tree test window should be open");
        cx.simulate_keystrokes(window.into(), "w");
        cx.run_until_parked();
        cx.run_until_parked();

        assert!(search_input_is_rendered(cx, window));
        window
            .update(cx, |_view, window, cx| {
                let context = handle.context().expect("handle should be bound");
                assert!(context.search_focus_handle().is_focused(window));
                assert_eq!(handle.get_search_value(cx), "w");
            })
            .expect("file-tree test window should be open");

        cx.simulate_keystrokes(window.into(), "o r k e r");
        cx.run_until_parked();
        cx.run_until_parked();
        window
            .update(cx, |_view, window, cx| {
                let context = handle.context().expect("handle should be bound");
                assert_eq!(handle.get_search_value(cx), "worker");
                assert_eq!(
                    handle.get_focused_path(cx).as_deref(),
                    Some("src/utils/worker-a.ts")
                );
                assert!(context.search_focus_handle().is_focused(window));
            })
            .expect("file-tree test window should be open");

        window
            .update(cx, |_view, window, cx| {
                assert!(handle.set_search(Some("stream"), window, cx));
            })
            .expect("file-tree test window should be open");
        cx.run_until_parked();
        cx.simulate_keystrokes(window.into(), "x");
        cx.run_until_parked();
        window
            .update(cx, |_view, window, cx| {
                assert_eq!(handle.get_search_value(cx), "streamx");
                assert!(handle.set_search(Some("worker"), window, cx));
            })
            .expect("file-tree test window should be open");
        cx.run_until_parked();

        cx.simulate_keystrokes(window.into(), "down");
        cx.run_until_parked();
        window
            .update(cx, |_view, window, cx| {
                let context = handle.context().expect("handle should be bound");
                assert_eq!(
                    handle.get_focused_path(cx).as_deref(),
                    Some("src/utils/worker-b.ts")
                );
                assert!(context.search_focus_handle().is_focused(window));
            })
            .expect("file-tree test window should be open");
        cx.simulate_keystrokes(window.into(), "up");
        cx.run_until_parked();
        window
            .update(cx, |_view, window, cx| {
                let context = handle.context().expect("handle should be bound");
                assert_eq!(
                    handle.get_focused_path(cx).as_deref(),
                    Some("src/utils/worker-a.ts")
                );
                assert!(context.search_focus_handle().is_focused(window));
            })
            .expect("file-tree test window should be open");
        cx.simulate_keystrokes(window.into(), "down enter");
        cx.run_until_parked();
        cx.run_until_parked();
        window
            .update(cx, |_view, window, cx| {
                let context = handle.context().expect("handle should be bound");
                assert_eq!(
                    handle.get_focused_path(cx).as_deref(),
                    Some("src/utils/worker-b.ts")
                );
                assert_eq!(
                    handle.get_selected_paths(cx),
                    vec!["src/utils/worker-b.ts".to_owned()]
                );
                assert!(handle.is_search_open(cx));
                assert!(context.search_focus_handle().is_focused(window));
            })
            .expect("file-tree test window should be open");

        cx.simulate_keystrokes(window.into(), "escape");
        cx.run_until_parked();
        cx.run_until_parked();
        assert!(!search_input_is_rendered(cx, window));
        window
            .update(cx, |_view, window, cx| {
                let context = handle.context().expect("handle should be bound");
                assert!(!handle.is_search_open(cx));
                assert!(context.focus_handle().is_focused(window));
            })
            .expect("file-tree test window should be open");
        assert_eq!(changes.borrow().first(), Some(&Some("w".to_owned())));
        assert!(changes.borrow().contains(&Some("stream".to_owned())));
        assert!(changes.borrow().contains(&Some("streamx".to_owned())));
        assert_eq!(changes.borrow().last(), Some(&None));
    }

    #[gpui::test]
    fn search_visibility_programmatic_updates_and_disabled_hotkey_match_configuration(
        cx: &mut TestAppContext,
    ) {
        let handle = create_file_tree_handle();
        let changes = Rc::new(RefCell::new(Vec::new()));
        let window = open_search_tree(
            cx,
            handle.clone(),
            false,
            None,
            super::super::FileTreeSearchBlurBehavior::Close,
            Rc::clone(&changes),
        );

        window
            .update(cx, |_view, window, cx| {
                handle
                    .context()
                    .expect("handle should be bound")
                    .focus_tree(window, cx);
            })
            .expect("file-tree test window should be open");
        cx.simulate_keystrokes(window.into(), "w");
        cx.run_until_parked();
        window
            .update(cx, |_view, _window, cx| {
                assert!(!handle.is_search_open(cx));
            })
            .expect("file-tree test window should be open");
        assert!(!search_input_is_rendered(cx, window));

        window
            .update(cx, |_view, window, cx| {
                assert!(handle.open_search_with_value("worker", window, cx));
                assert_eq!(handle.get_visible_count(cx), 4);
            })
            .expect("file-tree test window should be open");
        cx.run_until_parked();
        assert!(!search_input_is_rendered(cx, window));
        assert_eq!(changes.borrow().as_slice(), &[Some("worker".to_owned())]);
    }

    #[gpui::test]
    fn initial_query_suppresses_callback_and_blur_behavior_is_interaction_aware(
        cx: &mut TestAppContext,
    ) {
        let close_handle = create_file_tree_handle();
        let close_changes = Rc::new(RefCell::new(Vec::new()));
        let close_window = open_search_tree(
            cx,
            close_handle.clone(),
            true,
            Some("worker"),
            super::super::FileTreeSearchBlurBehavior::Close,
            Rc::clone(&close_changes),
        );
        assert!(close_changes.borrow().is_empty());
        close_window
            .update(cx, |view, window, cx| {
                let context = close_handle.context().expect("handle should be bound");
                assert!(context.search_focus_handle().is_focused(window));
                view.outside_focus.focus(window, cx);
            })
            .expect("file-tree test window should be open");
        cx.run_until_parked();
        cx.run_until_parked();
        close_window
            .update(cx, |_view, _window, cx| {
                assert!(!close_handle.is_search_open(cx));
            })
            .expect("file-tree test window should be open");
        assert_eq!(close_changes.borrow().as_slice(), &[None]);

        let retain_handle = create_file_tree_handle();
        let retain_changes = Rc::new(RefCell::new(Vec::new()));
        let retain_window = open_search_tree(
            cx,
            retain_handle.clone(),
            true,
            Some("worker"),
            super::super::FileTreeSearchBlurBehavior::Retain,
            Rc::clone(&retain_changes),
        );
        retain_window
            .update(cx, |_view, window, cx| {
                let context = retain_handle.context().expect("handle should be bound");
                assert!(!context.search_focus_handle().is_focused(window));
                context.focus_tree(window, cx);
                assert!(retain_handle.is_search_open(cx));
                context.search_focus_handle().focus(window, cx);
            })
            .expect("file-tree test window should be open");
        cx.run_until_parked();
        retain_window
            .update(cx, |view, window, cx| {
                view.outside_focus.focus(window, cx);
            })
            .expect("file-tree test window should be open");
        cx.run_until_parked();
        cx.run_until_parked();
        retain_window
            .update(cx, |_view, _window, cx| {
                assert!(!retain_handle.is_search_open(cx));
            })
            .expect("file-tree test window should be open");
        assert_eq!(retain_changes.borrow().as_slice(), &[None]);
    }

    struct FileTreeRenameTestView {
        config: FileTreeRenamingConfig,
        handle: FileTreeHandle,
        input: FileTreePreparedInput,
    }

    impl Render for FileTreeRenameTestView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            FileTree::new(self.input.clone())
                .initial_expansion(FileTreeInitialExpansion::Closed)
                .renaming_config(self.config.clone())
                .handle(self.handle.clone())
        }
    }

    fn open_rename_tree(
        cx: &mut TestAppContext,
        handle: FileTreeHandle,
        input: FileTreePreparedInput,
        config: FileTreeRenamingConfig,
    ) -> WindowHandle<FileTreeRenameTestView> {
        cx.update(crate::file_tree::init);
        cx.update(crate::primitives::input::init);
        let window = cx.open_window(size(px(320.0), px(160.0)), move |_, _| {
            FileTreeRenameTestView {
                config,
                handle,
                input,
            }
        });
        cx.run_until_parked();
        cx.run_until_parked();
        window
    }

    #[gpui::test]
    fn inline_rename_focus_handoff_selects_the_name_and_enter_commits(cx: &mut TestAppContext) {
        let handle = create_file_tree_handle();
        let events = Rc::new(RefCell::new(Vec::<FileTreeRenameEvent>::new()));
        let event_sink = Rc::clone(&events);
        let config = FileTreeRenamingConfig::new().on_rename(move |event, _, _| {
            event_sink.borrow_mut().push(event.clone());
        });
        let input = FileTreePreparedInput::prepare(["README.md", "src/lib/util.ts"]).unwrap();
        let window = open_rename_tree(cx, handle.clone(), input, config);

        window
            .update(cx, |_view, window, cx| {
                assert!(handle.focus_path("README.md", window, cx));
                handle
                    .context()
                    .expect("handle should be bound")
                    .focus_tree(window, cx);
            })
            .expect("file-tree rename test window should be open");
        cx.simulate_keystrokes(window.into(), "f2");
        cx.run_until_parked();
        cx.run_until_parked();
        window
            .update(cx, |_view, _window, cx| {
                assert_eq!(handle.get_renaming_path(cx).as_deref(), Some("README.md"));
            })
            .expect("file-tree rename test window should be open");
        cx.simulate_keystrokes(window.into(), "x enter");
        cx.run_until_parked();
        cx.run_until_parked();

        window
            .update(cx, |_view, window, cx| {
                assert!(!handle.is_renaming(cx));
                assert!(handle.get_item("x", cx).is_some());
                assert!(handle.get_item("README.md", cx).is_none());
                assert_eq!(handle.get_selected_paths(cx), vec!["x".to_owned()]);
                let context = handle.context().expect("handle should be bound");
                assert!(context.focus_handle().is_focused(window));
            })
            .expect("file-tree rename test window should be open");
        assert_eq!(
            events.borrow().as_slice(),
            &[FileTreeRenameEvent {
                source_path: "README.md".to_owned(),
                destination_path: "x".to_owned(),
                is_directory: false,
            }]
        );
    }

    #[gpui::test]
    fn rename_permissions_errors_unchanged_and_remove_if_canceled_match_pierre(
        cx: &mut TestAppContext,
    ) {
        let handle = create_file_tree_handle();
        let offered = Rc::new(RefCell::new(Vec::new()));
        let errors = Rc::new(RefCell::new(Vec::<FileTreeRenameError>::new()));
        let events = Rc::new(RefCell::new(Vec::<FileTreeRenameEvent>::new()));
        let offered_sink = Rc::clone(&offered);
        let error_sink = Rc::clone(&errors);
        let event_sink = Rc::clone(&events);
        let config = FileTreeRenamingConfig::new()
            .can_rename(move |item| {
                offered_sink.borrow_mut().push(item.clone());
                item.path != "denied.ts"
            })
            .on_error(move |error, _, _| error_sink.borrow_mut().push(error.clone()))
            .on_rename(move |event, _, _| event_sink.borrow_mut().push(event.clone()));
        let input = FileTreePreparedInput::prepare([
            "denied.ts",
            "placeholder.ts",
            "src/lib/util.ts",
            "src/lib/worker.ts",
            "src/target",
        ])
        .unwrap();
        let window = open_rename_tree(cx, handle.clone(), input, config);

        window
            .update(cx, |_view, window, cx| {
                assert!(!handle.start_renaming("denied.ts", window, cx));
                assert!(handle.start_renaming("src/lib", window, cx));
                assert_eq!(handle.get_renaming_path(cx).as_deref(), Some("src/lib"));
                assert!(handle
                    .get_item("src", cx)
                    .and_then(|item| match item {
                        FileTreeItemHandle::Directory(directory) => Some(directory),
                        FileTreeItemHandle::File(_) => None,
                    })
                    .expect("src should be a directory")
                    .is_expanded(cx));
                assert!(handle
                    .get_item("src/lib", cx)
                    .and_then(|item| match item {
                        FileTreeItemHandle::Directory(directory) => Some(directory),
                        FileTreeItemHandle::File(_) => None,
                    })
                    .expect("src/lib should be a directory")
                    .is_selected(cx));
                set_file_tree_renaming_value(&handle, "lib", window, cx);
                assert!(commit_file_tree_renaming(&handle, window, cx));
                assert!(events.borrow().is_empty());
                assert!(errors.borrow().is_empty());

                assert!(handle.start_renaming("src/lib/util.ts", window, cx));
                set_file_tree_renaming_value(&handle, "worker.ts", window, cx);
                assert!(commit_file_tree_renaming(&handle, window, cx));
                assert_eq!(
                    errors.borrow().as_slice(),
                    &[FileTreeRenameError::DestinationAlreadyExists(
                        "src/lib/worker.ts".to_owned()
                    )]
                );
                assert!(events.borrow().is_empty());
                assert!(!handle.is_renaming(cx));

                assert!(handle.start_renaming("src/lib/worker.ts", window, cx));
                set_file_tree_renaming_value(&handle, "nested/name", window, cx);
                assert!(commit_file_tree_renaming(&handle, window, cx));
                assert_eq!(
                    errors.borrow().as_slice(),
                    &[
                        FileTreeRenameError::DestinationAlreadyExists(
                            "src/lib/worker.ts".to_owned(),
                        ),
                        FileTreeRenameError::NameContainsSlash,
                    ]
                );

                assert!(handle.start_renaming("src/lib", window, cx));
                set_file_tree_renaming_value(&handle, "target", window, cx);
                assert!(commit_file_tree_renaming(&handle, window, cx));
                assert_eq!(
                    errors.borrow().last(),
                    Some(&FileTreeRenameError::DestinationAlreadyExists(
                        "src/target".to_owned(),
                    ))
                );
                assert!(events.borrow().is_empty());
                assert!(handle.get_item("src/lib/util.ts", cx).is_some());

                assert!(handle.start_renaming("src/lib", window, cx));
                set_file_tree_renaming_value(&handle, "core", window, cx);
                assert!(commit_file_tree_renaming(&handle, window, cx));
                assert!(handle.get_item("src/lib", cx).is_none());
                assert!(handle.get_item("src/core/util.ts", cx).is_some());
                assert!(handle.get_item("src/core/worker.ts", cx).is_some());
                assert_eq!(
                    events.borrow().as_slice(),
                    &[FileTreeRenameEvent {
                        source_path: "src/lib".to_owned(),
                        destination_path: "src/core".to_owned(),
                        is_directory: true,
                    }]
                );

                assert!(handle.start_renaming_with_options(
                    "placeholder.ts",
                    FileTreeStartRenamingOptions {
                        remove_if_canceled: true,
                    },
                    window,
                    cx,
                ));
                assert!(cancel_file_tree_renaming(&handle, window, cx));
                assert!(handle.get_item("placeholder.ts", cx).is_none());
            })
            .expect("file-tree rename test window should be open");

        assert!(offered
            .borrow()
            .iter()
            .any(|item| item.path == "src/lib" && item.is_directory));
    }

    #[gpui::test]
    fn blur_commits_inline_rename_and_escape_cancels_it(cx: &mut TestAppContext) {
        let handle = create_file_tree_handle();
        let input = FileTreePreparedInput::prepare(["README.md", "other.ts"]).unwrap();
        let window = open_rename_tree(cx, handle.clone(), input, FileTreeRenamingConfig::default());

        window
            .update(cx, |_view, window, cx| {
                assert!(handle.start_renaming("README.md", window, cx));
            })
            .expect("file-tree rename test window should be open");
        cx.run_until_parked();
        cx.run_until_parked();
        cx.simulate_keystrokes(window.into(), "escape");
        cx.run_until_parked();
        cx.run_until_parked();
        window
            .update(cx, |_view, window, cx| {
                assert!(!handle.is_renaming(cx));
                assert!(handle.get_item("README.md", cx).is_some());
                assert!(handle.start_renaming("README.md", window, cx));
            })
            .expect("file-tree rename test window should be open");
        cx.run_until_parked();
        cx.run_until_parked();
        cx.simulate_keystrokes(window.into(), "z");
        cx.run_until_parked();
        window
            .update(cx, |_view, window, cx| {
                handle
                    .context()
                    .expect("handle should be bound")
                    .focus_tree(window, cx);
            })
            .expect("file-tree rename test window should be open");
        cx.run_until_parked();
        cx.run_until_parked();
        window
            .update(cx, |_view, _window, cx| {
                assert!(!handle.is_renaming(cx));
                assert!(handle.get_item("z", cx).is_some());
            })
            .expect("file-tree rename test window should be open");
    }

    #[gpui::test]
    fn offscreen_rename_scrolls_the_canonical_row_into_view_before_focus_handoff(
        cx: &mut TestAppContext,
    ) {
        let handle = create_file_tree_handle();
        let mut paths = (0..80)
            .map(|index| format!("item-{index:03}.ts"))
            .collect::<Vec<_>>();
        paths.push("zz-target.ts".to_owned());
        let input = FileTreePreparedInput::prepare(paths).unwrap();
        let window = open_rename_tree(cx, handle.clone(), input, FileTreeRenamingConfig::default());

        window
            .update(cx, |_view, window, cx| {
                assert!(handle.start_renaming("zz-target.ts", window, cx));
            })
            .expect("file-tree rename test window should be open");
        cx.run_until_parked();
        cx.run_until_parked();
        cx.simulate_keystrokes(window.into(), "q enter");
        cx.run_until_parked();
        cx.run_until_parked();
        window
            .update(cx, |_view, _window, cx| {
                assert!(!handle.is_renaming(cx));
                assert!(handle.get_item("q", cx).is_some());
                assert!(handle.get_item("zz-target.ts", cx).is_none());
            })
            .expect("file-tree rename test window should be open");
    }

    #[gpui::test]
    fn mutation_remap_reveals_the_active_rename_input_at_its_new_path(cx: &mut TestAppContext) {
        let handle = create_file_tree_handle();
        let mut paths = vec!["a-target.ts".to_owned()];
        paths.extend((0..80).map(|index| format!("item-{index:03}.ts")));
        let input = FileTreePreparedInput::prepare(paths).unwrap();
        let window = open_rename_tree(cx, handle.clone(), input, FileTreeRenamingConfig::default());

        window
            .update(cx, |_view, window, cx| {
                assert!(handle.start_renaming("a-target.ts", window, cx));
            })
            .expect("file-tree rename test window should be open");
        cx.run_until_parked();
        cx.run_until_parked();
        window
            .update(cx, |_view, window, cx| {
                handle
                    .move_path(
                        "a-target.ts",
                        "zz-target.ts",
                        FileTreeMoveOptions::default(),
                        window,
                        cx,
                    )
                    .unwrap();
                assert_eq!(
                    handle.get_renaming_path(cx).as_deref(),
                    Some("zz-target.ts")
                );
            })
            .expect("file-tree rename test window should be open");
        cx.run_until_parked();
        cx.run_until_parked();
        cx.simulate_keystrokes(window.into(), "q enter");
        cx.run_until_parked();
        cx.run_until_parked();
        window
            .update(cx, |_view, _window, cx| {
                assert!(!handle.is_renaming(cx));
                assert!(handle.get_item("q", cx).is_some());
                assert!(handle.get_item("zz-target.ts", cx).is_none());
            })
            .expect("file-tree rename test window should be open");
    }

    fn scroll_top(handle: &FileTreeHandle) -> Pixels {
        let context = handle.context().expect("handle should be bound");
        let scroll_handle = context.scroll_handle().clone();
        let offset = scroll_handle.0.borrow().base_handle.offset();
        -offset.y
    }
}
