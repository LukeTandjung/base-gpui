use gpui::{Bounds, FocusHandle, Pixels};

use crate::tabs::{
    TabsIndicatorStyleState, TabsListStyleState, TabsOrientation, TabsPanelStyleState,
    TabsRootStyleState, TabsTabStyleState,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TabsActivationDirection {
    #[default]
    None,
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone)]
pub struct TabsTabMetadata<T: Clone + Eq + 'static> {
    value: T,
    disabled: bool,
    index: usize,
}

impl<T: Clone + Eq + 'static> TabsTabMetadata<T> {
    pub fn new(value: T, disabled: bool, index: usize) -> Self {
        Self {
            value,
            disabled,
            index,
        }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TabsTabPosition {
    pub left: Pixels,
    pub right: Pixels,
    pub top: Pixels,
    pub bottom: Pixels,
}

impl TabsTabPosition {
    pub fn new(left: Pixels, right: Pixels, top: Pixels, bottom: Pixels) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TabsTabSize {
    pub width: Pixels,
    pub height: Pixels,
}

impl TabsTabSize {
    pub fn new(width: Pixels, height: Pixels) -> Self {
        Self { width, height }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Move {
    First,
    Last,
    Next,
    Previous,
}

pub struct SelectOutcome<T: Clone + Eq + 'static> {
    changed: bool,
    value: Option<T>,
}

impl<T: Clone + Eq + 'static> SelectOutcome<T> {
    fn new(changed: bool, value: Option<T>) -> Self {
        Self { changed, value }
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn value(&self) -> Option<&T> {
        self.value.as_ref()
    }
}

#[derive(Clone)]
pub struct TabsRuntime<T: Clone + Eq + 'static> {
    selected: Option<T>,
    tabs: Vec<TabsTabMetadata<T>>,
    highlighted_tab_index: Option<usize>,
    activation_direction: TabsActivationDirection,
    tab_bounds: Vec<(usize, Bounds<Pixels>)>,
    tab_focus_handles: Vec<(usize, FocusHandle)>,
    has_seeded_initial_focus: bool,
}

impl<T: Clone + Eq + 'static> Default for TabsRuntime<T> {
    fn default() -> Self {
        Self {
            selected: None,
            tabs: Vec::new(),
            highlighted_tab_index: None,
            activation_direction: TabsActivationDirection::None,
            tab_bounds: Vec::new(),
            tab_focus_handles: Vec::new(),
            has_seeded_initial_focus: false,
        }
    }
}

impl<T: Clone + Eq + 'static> TabsRuntime<T> {
    pub fn new(selected: Option<T>) -> Self {
        Self {
            selected,
            ..Self::default()
        }
    }

    pub fn selected_value(&self) -> Option<T> {
        self.selected.clone()
    }

    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    pub fn sync_children(
        &mut self,
        mut tabs: Vec<TabsTabMetadata<T>>,
        mut tab_focus_handles: Vec<(usize, FocusHandle)>,
    ) {
        let highlighted_value = self
            .highlighted_tab_index
            .and_then(|index| self.enabled_value_at_index(index).cloned());

        tabs.sort_by_key(TabsTabMetadata::index);
        tab_focus_handles.sort_by_key(|(index, _)| *index);

        self.tabs = tabs;
        self.tab_focus_handles = tab_focus_handles;
        self.highlighted_tab_index = highlighted_value
            .as_ref()
            .and_then(|value| self.index_of_enabled_value(value));
        self.tab_bounds
            .retain(|(index, _)| self.tabs.iter().any(|tab| tab.index() == *index));
    }

    pub fn reconcile(
        &mut self,
        observed_selected: Option<T>,
        allow_fallback: bool,
        orientation: TabsOrientation,
    ) {
        let previous = self.selected.clone();
        let needs_fallback =
            allow_fallback && !self.selected_is_enabled(observed_selected.as_ref());
        let selected = match needs_fallback {
            true => self.first_enabled_value(),
            false => observed_selected,
        };
        let selected_changed = previous != selected;

        if selected_changed {
            self.activation_direction = match needs_fallback {
                true => TabsActivationDirection::None,
                false => self.direction_between(previous.as_ref(), selected.as_ref(), orientation),
            };
        }

        self.selected = selected;

        if selected_changed || self.highlighted_tab_index.is_none() {
            self.sync_highlight_with_selected();
        }
    }

    pub fn set_tab_bounds(&mut self, bounds: Vec<(usize, Bounds<Pixels>)>) -> bool {
        if self.tab_bounds == bounds {
            return false;
        }

        self.tab_bounds = bounds;
        true
    }

    pub fn take_initial_focus_handle(&mut self) -> Option<FocusHandle> {
        if self.has_seeded_initial_focus {
            return None;
        }

        self.has_seeded_initial_focus = true;
        self.highlighted_focus_handle()
    }

    pub fn highlighted_value(&self) -> Option<T> {
        self.highlighted_tab_index
            .and_then(|index| self.enabled_value_at_index(index).cloned())
    }

    pub fn move_highlight(&mut self, direction: Move, loop_focus: bool) {
        match direction {
            Move::First => self.highlighted_tab_index = self.first_enabled_index(),
            Move::Last => self.highlighted_tab_index = self.last_enabled_index(),
            Move::Next => {
                if let Some(index) = self.next_enabled_index(self.highlighted_tab_index, loop_focus)
                {
                    self.highlighted_tab_index = Some(index);
                }
            }
            Move::Previous => {
                if let Some(index) =
                    self.previous_enabled_index(self.highlighted_tab_index, loop_focus)
                {
                    self.highlighted_tab_index = Some(index);
                }
            }
        }
    }

    pub fn select(&mut self, value: Option<T>, orientation: TabsOrientation) -> SelectOutcome<T> {
        let current = self.selected.clone();

        self.select_from(current, value, orientation, true)
    }

    pub fn select_from(
        &mut self,
        current: Option<T>,
        value: Option<T>,
        orientation: TabsOrientation,
        commit: bool,
    ) -> SelectOutcome<T> {
        let changed = current != value;

        self.activation_direction =
            self.direction_between(current.as_ref(), value.as_ref(), orientation);
        self.highlighted_tab_index = self.highlight_for_value(value.as_ref());

        if commit {
            self.selected = value.clone();
        } else {
            self.selected = current;
        }

        SelectOutcome::new(changed, value)
    }

    pub fn root_state(&self, orientation: TabsOrientation) -> TabsRootStyleState {
        TabsRootStyleState::new(orientation, self.activation_direction)
    }

    pub fn list_state(&self, orientation: TabsOrientation) -> TabsListStyleState {
        TabsListStyleState::new(orientation, self.activation_direction)
    }

    pub fn tab_state(
        &self,
        value: Option<&T>,
        disabled: bool,
        index: Option<usize>,
        orientation: TabsOrientation,
    ) -> TabsTabStyleState {
        let active = match (value, self.selected.as_ref()) {
            (Some(value), Some(selected)) => value == selected,
            _ => false,
        };

        TabsTabStyleState::new(
            active,
            disabled,
            self.highlighted_tab_index == index,
            orientation,
        )
    }

    pub fn panel_state(
        &self,
        value: Option<&T>,
        orientation: TabsOrientation,
    ) -> TabsPanelStyleState {
        let active = match (value, self.selected.as_ref()) {
            (Some(value), Some(selected)) => value == selected,
            _ => false,
        };

        TabsPanelStyleState::new(!active, orientation, self.activation_direction)
    }

    pub fn indicator_state(&self, orientation: TabsOrientation) -> TabsIndicatorStyleState {
        let selected = self.selected.as_ref();

        TabsIndicatorStyleState::new(
            selected.is_some(),
            self.active_tab_position(selected),
            self.active_tab_size(selected),
            orientation,
            self.activation_direction,
        )
    }

    fn highlighted_focus_handle(&self) -> Option<FocusHandle> {
        self.highlighted_tab_index
            .and_then(|index| self.focus_handle_at_index(index))
    }

    fn focus_handle_at_index(&self, index: usize) -> Option<FocusHandle> {
        self.tab_focus_handles
            .iter()
            .find(|(tab_index, _)| *tab_index == index)
            .map(|(_, focus_handle)| focus_handle.clone())
    }

    fn sync_highlight_with_selected(&mut self) {
        self.highlighted_tab_index = self.highlight_for_value(self.selected.as_ref());
    }

    fn highlight_for_value(&self, value: Option<&T>) -> Option<usize> {
        value
            .and_then(|value| self.index_of_enabled_value(value))
            .or_else(|| self.first_enabled_index())
    }

    fn selected_is_enabled(&self, selected: Option<&T>) -> bool {
        match selected {
            Some(value) => self.contains_enabled_value(value),
            None => false,
        }
    }

    fn direction_between(
        &self,
        previous: Option<&T>,
        next: Option<&T>,
        orientation: TabsOrientation,
    ) -> TabsActivationDirection {
        let Some(previous) = previous else {
            return TabsActivationDirection::None;
        };
        let Some(next) = next else {
            return TabsActivationDirection::None;
        };

        let Some(previous_index) = self.index_of_value(previous) else {
            return TabsActivationDirection::None;
        };
        let Some(next_index) = self.index_of_value(next) else {
            return TabsActivationDirection::None;
        };

        match previous_index.cmp(&next_index) {
            std::cmp::Ordering::Less => match orientation {
                TabsOrientation::Horizontal => TabsActivationDirection::Right,
                TabsOrientation::Vertical => TabsActivationDirection::Down,
            },
            std::cmp::Ordering::Greater => match orientation {
                TabsOrientation::Horizontal => TabsActivationDirection::Left,
                TabsOrientation::Vertical => TabsActivationDirection::Up,
            },
            std::cmp::Ordering::Equal => TabsActivationDirection::None,
        }
    }

    fn active_tab_position(&self, selected: Option<&T>) -> Option<TabsTabPosition> {
        let bounds = self.active_tab_bounds(selected)?;
        let origin = self.tab_bounds_origin()?;

        Some(TabsTabPosition::new(
            bounds.left() - origin.0,
            bounds.right() - origin.0,
            bounds.top() - origin.1,
            bounds.bottom() - origin.1,
        ))
    }

    fn active_tab_size(&self, selected: Option<&T>) -> Option<TabsTabSize> {
        let bounds = self.active_tab_bounds(selected)?;

        Some(TabsTabSize::new(bounds.size.width, bounds.size.height))
    }

    fn first_enabled_index(&self) -> Option<usize> {
        self.tabs
            .iter()
            .find(|tab| !tab.disabled())
            .map(TabsTabMetadata::index)
    }

    fn last_enabled_index(&self) -> Option<usize> {
        self.tabs
            .iter()
            .rev()
            .find(|tab| !tab.disabled())
            .map(TabsTabMetadata::index)
    }

    fn next_enabled_index(&self, current: Option<usize>, loop_focus: bool) -> Option<usize> {
        let current = current.or_else(|| self.first_enabled_index())?;

        self.tabs
            .iter()
            .find(|tab| !tab.disabled() && tab.index() > current)
            .map(TabsTabMetadata::index)
            .or_else(|| loop_focus.then(|| self.first_enabled_index()).flatten())
    }

    fn previous_enabled_index(&self, current: Option<usize>, loop_focus: bool) -> Option<usize> {
        let current = current.or_else(|| self.first_enabled_index())?;

        self.tabs
            .iter()
            .rev()
            .find(|tab| !tab.disabled() && tab.index() < current)
            .map(TabsTabMetadata::index)
            .or_else(|| loop_focus.then(|| self.last_enabled_index()).flatten())
    }

    fn enabled_value_at_index(&self, index: usize) -> Option<&T> {
        self.tabs
            .iter()
            .find(|tab| !tab.disabled() && tab.index() == index)
            .map(TabsTabMetadata::value)
    }

    fn index_of_value(&self, value: &T) -> Option<usize> {
        self.tabs
            .iter()
            .find(|tab| tab.value() == value)
            .map(TabsTabMetadata::index)
    }

    fn index_of_enabled_value(&self, value: &T) -> Option<usize> {
        self.tabs
            .iter()
            .find(|tab| !tab.disabled() && tab.value() == value)
            .map(TabsTabMetadata::index)
    }

    fn first_enabled_value(&self) -> Option<T> {
        self.tabs
            .iter()
            .find(|tab| !tab.disabled())
            .map(|tab| tab.value().clone())
    }

    fn contains_enabled_value(&self, value: &T) -> bool {
        self.tabs
            .iter()
            .any(|tab| !tab.disabled() && tab.value() == value)
    }

    fn active_tab_bounds(&self, selected: Option<&T>) -> Option<Bounds<Pixels>> {
        let selected = selected?;
        let index = self.index_of_value(selected)?;

        self.tab_bounds
            .iter()
            .find(|(tab_index, _)| *tab_index == index)
            .map(|(_, bounds)| *bounds)
    }

    fn tab_bounds_origin(&self) -> Option<(Pixels, Pixels)> {
        let first = self.tab_bounds.first()?.1;

        Some(
            self.tab_bounds
                .iter()
                .fold((first.left(), first.top()), |origin, (_, bounds)| {
                    (origin.0.min(bounds.left()), origin.1.min(bounds.top()))
                }),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Move, TabsRuntime, TabsTabMetadata};
    use crate::tabs::TabsOrientation;

    #[test]
    fn move_highlight_skips_disabled_tabs() {
        let mut runtime = TabsRuntime::new(Some(1));

        runtime.sync_children(
            vec![
                TabsTabMetadata::new(1, false, 0),
                TabsTabMetadata::new(2, true, 1),
                TabsTabMetadata::new(3, false, 2),
            ],
            Vec::new(),
        );
        runtime.move_highlight(Move::First, false);

        runtime.move_highlight(Move::Next, false);

        assert!(
            !runtime
                .tab_state(Some(&1), false, Some(0), TabsOrientation::Horizontal)
                .highlighted
        );
        assert!(
            runtime
                .tab_state(Some(&3), false, Some(2), TabsOrientation::Horizontal)
                .highlighted
        );
    }

    #[test]
    fn reconcile_applies_uncontrolled_fallback() {
        let mut runtime = TabsRuntime::new(Some(2));

        runtime.sync_children(
            vec![
                TabsTabMetadata::new(1, false, 0),
                TabsTabMetadata::new(2, true, 1),
            ],
            Vec::new(),
        );
        runtime.reconcile(runtime.selected_value(), true, TabsOrientation::Horizontal);

        assert!(
            runtime
                .tab_state(Some(&1), false, Some(0), TabsOrientation::Horizontal)
                .active
        );
        assert!(
            runtime
                .tab_state(Some(&1), false, Some(0), TabsOrientation::Horizontal)
                .highlighted
        );
        assert_eq!(
            runtime
                .root_state(TabsOrientation::Horizontal)
                .activation_direction,
            super::TabsActivationDirection::None
        );
    }

    #[test]
    fn reconcile_preserves_keyboard_highlight_when_selection_is_unchanged() {
        let mut runtime = TabsRuntime::new(Some(1));

        runtime.sync_children(
            vec![
                TabsTabMetadata::new(1, false, 0),
                TabsTabMetadata::new(2, false, 1),
            ],
            Vec::new(),
        );
        runtime.reconcile(runtime.selected_value(), true, TabsOrientation::Horizontal);
        runtime.move_highlight(Move::Next, false);
        runtime.reconcile(runtime.selected_value(), true, TabsOrientation::Horizontal);

        assert!(
            runtime
                .tab_state(Some(&2), false, Some(1), TabsOrientation::Horizontal)
                .highlighted
        );
    }
}
