use gpui::{
    px, App, Context, ElementId, Entity, FocusHandle, InteractiveElement as _, IntoElement,
    RenderOnce, SharedString, Styled as _, Window,
};

use crate::{
    file_tree::{context::FileTreeContext, FileTreeSearchBlurBehavior},
    primitives::input::{input, InputRuntime},
};

pub struct FileTreeSearchInteraction {
    blur_behavior: FileTreeSearchBlurBehavior,
    initialized: bool,
    input_interacted: bool,
    observed_open: bool,
    session_generation: usize,
}

pub struct FileTreeSearchRenderState {
    pub auto_focus: bool,
    pub session_generation: usize,
}

impl FileTreeSearchInteraction {
    pub fn new(blur_behavior: FileTreeSearchBlurBehavior) -> Self {
        Self {
            blur_behavior,
            initialized: false,
            input_interacted: false,
            observed_open: false,
            session_generation: 0,
        }
    }

    pub fn sync(
        &mut self,
        blur_behavior: FileTreeSearchBlurBehavior,
        open: bool,
    ) -> FileTreeSearchRenderState {
        self.blur_behavior = blur_behavior;

        let auto_focus = if self.initialized {
            open && !self.observed_open
        } else {
            self.initialized = true;
            open && blur_behavior == FileTreeSearchBlurBehavior::Close
        };
        if open && !self.observed_open {
            self.session_generation = self.session_generation.wrapping_add(1);
        }
        self.observed_open = open;

        FileTreeSearchRenderState {
            auto_focus,
            session_generation: self.session_generation,
        }
    }

    pub fn mark_interacted(&mut self) {
        self.input_interacted = true;
    }

    pub fn should_close_on_blur(&self) -> bool {
        self.blur_behavior == FileTreeSearchBlurBehavior::Close || self.input_interacted
    }
}

#[derive(IntoElement)]
pub struct FileTreeSearchInput {
    id: ElementId,
    auto_focus: bool,
    context: FileTreeContext,
    focus_handle: FocusHandle,
    interaction: Entity<FileTreeSearchInteraction>,
    value: SharedString,
}

impl FileTreeSearchInput {
    pub fn new(
        id: ElementId,
        auto_focus: bool,
        context: FileTreeContext,
        focus_handle: FocusHandle,
        interaction: Entity<FileTreeSearchInteraction>,
        value: SharedString,
    ) -> Self {
        Self {
            id,
            auto_focus,
            context,
            focus_handle,
            interaction,
            value,
        }
    }
}

impl RenderOnce for FileTreeSearchInput {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let change_context = self.context.clone();
        let enter_context = self.context.clone();
        let blur_context = self.context;
        let focus_interaction = self.interaction.clone();
        let blur_interaction = self.interaction;
        // Pinned GPUI exposes only a descendant-side active marker that requires
        // the focused element to be an ancestor. The input and tree rows are
        // siblings, so it cannot express Pierre's input-side relationship.
        input()
            .id(self.id)
            .value(self.value)
            .placeholder("Search…")
            .focus_handle(self.focus_handle)
            .auto_focus(self.auto_focus)
            .h(px(24.0))
            .w_full()
            .on_value_change_with_context(move |value, window, cx: &mut Context<InputRuntime>| {
                change_context.set_search(Some(value.as_ref()), window, cx);
            })
            .on_enter_with_context(move |_value, window, cx| {
                enter_context.select_focused_search_match(window, cx);
            })
            .on_focus_with_context(move |_window, cx| {
                focus_interaction.update(cx, |interaction, _| interaction.mark_interacted());
            })
            .on_blur_with_context(move |window, cx| {
                let should_close = blur_interaction
                    .update(cx, |interaction, _| interaction.should_close_on_blur());
                if should_close {
                    blur_context.close_search(window, cx);
                }
            })
            .style_with_state(|_state, input| {
                input.debug_selector(|| "file-tree-search-input".into())
            })
    }
}
