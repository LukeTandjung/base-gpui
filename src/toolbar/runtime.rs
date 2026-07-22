use gpui::FocusHandle;

use crate::toolbar::{ToolbarProps, ToolbarRootStyleState};

/// Per-item registration metadata, the Base UI `ToolbarRoot.ItemMetadata`
/// equivalent. This is the shared registration channel for every roving item:
/// buttons, links, inputs today, and future Toggle/ToggleGroup children,
/// which register `{ disabled, focusable_when_disabled: false }` per toggle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ToolbarItemMetadata {
    disabled: bool,
    focusable_when_disabled: bool,
}

impl ToolbarItemMetadata {
    pub fn new(disabled: bool, focusable_when_disabled: bool) -> Self {
        Self {
            disabled,
            focusable_when_disabled,
        }
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn focusable_when_disabled(&self) -> bool {
        self.focusable_when_disabled
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ToolbarMove {
    Next,
    Previous,
}

/// The single owner of toolbar business state: registered item metadata in
/// flattened source order, roving focus handles, and the roving tab-stop
/// index. The skip set (`disabled && !focusable_when_disabled`) is computed
/// internally and never escapes; parts ask item-shaped questions only.
#[derive(Clone, Default)]
pub struct ToolbarRuntime {
    items: Vec<ToolbarItemMetadata>,
    highlighted_index: Option<usize>,
    focus_handles: Vec<(usize, FocusHandle)>,
}

impl ToolbarRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    /// Replaces the wired item metadata and focus handles. Item index is the
    /// position in `items`, assigned by child wiring in flattened source
    /// order.
    pub fn sync_children(
        &mut self,
        items: Vec<ToolbarItemMetadata>,
        mut focus_handles: Vec<(usize, FocusHandle)>,
    ) {
        focus_handles.sort_by_key(|(index, _)| *index);

        self.items = items;
        self.focus_handles = focus_handles;
    }

    /// Resolves the roving tab stop after a children sync: an invalid or
    /// skipped highlight falls back to the first item outside the skip set,
    /// so a disabled, non-focusable first item never becomes the initial tab
    /// stop.
    pub fn reconcile(&mut self) {
        let highlight_valid = self
            .highlighted_index
            .map(|index| self.is_focusable_index(index))
            .unwrap_or(false);

        if !highlight_valid {
            self.highlighted_index = self.first_focusable_index();
        }
    }

    /// Makes the item that currently owns real focus the roving tab stop, so
    /// tabbing away and back returns to it. Click-focus routes through here
    /// on the next render.
    pub fn sync_focused_index(&mut self, focused_index: Option<usize>) {
        if let Some(index) = focused_index {
            if self.is_focusable_index(index) {
                self.highlighted_index = Some(index);
            }
        }
    }

    /// Moves the roving highlight across items outside the skip set, wrapping
    /// only when `loop_focus` is set and clamping at the ends otherwise.
    pub fn move_highlight(&mut self, direction: ToolbarMove, loop_focus: bool) {
        let target = match direction {
            ToolbarMove::Next => self.next_focusable_index(loop_focus),
            ToolbarMove::Previous => self.previous_focusable_index(loop_focus),
        };

        if let Some(index) = target {
            self.highlighted_index = Some(index);
        }
    }

    /// Returns the focus handle for the current roving tab stop, when
    /// registered.
    pub fn highlighted_focus_handle(&self) -> Option<FocusHandle> {
        let index = self.highlighted_index?;

        self.focus_handles
            .iter()
            .find(|(handle_index, _)| *handle_index == index)
            .map(|(_, focus_handle)| focus_handle.clone())
    }

    /// Answers whether the item at `index` owns the toolbar's single roving
    /// tab stop.
    pub fn is_tab_stop(&self, index: usize) -> bool {
        self.highlighted_index == Some(index)
    }

    /// Returns the style state for the `ToolbarRoot` layer.
    pub fn root_state(&self, props: &ToolbarProps) -> ToolbarRootStyleState {
        ToolbarRootStyleState::new(props.disabled(), props.orientation())
    }

    fn is_focusable_index(&self, index: usize) -> bool {
        self.items
            .get(index)
            .map(|item| !item.disabled() || item.focusable_when_disabled())
            .unwrap_or(false)
    }

    fn first_focusable_index(&self) -> Option<usize> {
        (0..self.items.len()).find(|index| self.is_focusable_index(*index))
    }

    fn last_focusable_index(&self) -> Option<usize> {
        (0..self.items.len()).rfind(|index| self.is_focusable_index(*index))
    }

    fn next_focusable_index(&self, loop_focus: bool) -> Option<usize> {
        let current = self
            .highlighted_index
            .or_else(|| self.first_focusable_index())?;

        ((current + 1)..self.items.len())
            .find(|index| self.is_focusable_index(*index))
            .or_else(|| match loop_focus {
                true => self.first_focusable_index(),
                false => None,
            })
    }

    fn previous_focusable_index(&self, loop_focus: bool) -> Option<usize> {
        let current = self
            .highlighted_index
            .or_else(|| self.first_focusable_index())?;

        (0..current)
            .rfind(|index| self.is_focusable_index(*index))
            .or_else(|| match loop_focus {
                true => self.last_focusable_index(),
                false => None,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::{ToolbarItemMetadata, ToolbarMove, ToolbarRuntime};

    fn runtime(items: Vec<ToolbarItemMetadata>) -> ToolbarRuntime {
        let mut runtime = ToolbarRuntime::new();
        runtime.sync_children(items, Vec::new());
        runtime.reconcile();
        runtime
    }

    #[test]
    fn initial_tab_stop_skips_disabled_non_focusable_first_item() {
        let runtime = runtime(vec![
            ToolbarItemMetadata::new(true, false),
            ToolbarItemMetadata::new(false, true),
        ]);

        assert!(!runtime.is_tab_stop(0));
        assert!(runtime.is_tab_stop(1));
    }

    #[test]
    fn disabled_but_focusable_items_stay_in_the_roving_order() {
        let mut runtime = runtime(vec![
            ToolbarItemMetadata::new(false, true),
            ToolbarItemMetadata::new(true, true),
            ToolbarItemMetadata::new(false, true),
        ]);

        runtime.move_highlight(ToolbarMove::Next, true);
        assert!(runtime.is_tab_stop(1));
    }

    #[test]
    fn move_highlight_skips_the_skip_set_and_clamps_without_loop() {
        let mut runtime = runtime(vec![
            ToolbarItemMetadata::new(false, true),
            ToolbarItemMetadata::new(true, false),
            ToolbarItemMetadata::new(false, true),
        ]);

        runtime.move_highlight(ToolbarMove::Next, false);
        assert!(runtime.is_tab_stop(2));

        runtime.move_highlight(ToolbarMove::Next, false);
        assert!(runtime.is_tab_stop(2));

        runtime.move_highlight(ToolbarMove::Next, true);
        assert!(runtime.is_tab_stop(0));

        runtime.move_highlight(ToolbarMove::Previous, false);
        assert!(runtime.is_tab_stop(0));
    }

    #[test]
    fn sync_focused_index_moves_the_tab_stop_only_to_focusable_items() {
        let mut runtime = runtime(vec![
            ToolbarItemMetadata::new(false, true),
            ToolbarItemMetadata::new(true, false),
            ToolbarItemMetadata::new(false, true),
        ]);

        runtime.sync_focused_index(Some(2));
        assert!(runtime.is_tab_stop(2));

        runtime.sync_focused_index(Some(1));
        assert!(runtime.is_tab_stop(2));

        runtime.sync_focused_index(None);
        assert!(runtime.is_tab_stop(2));
    }
}
