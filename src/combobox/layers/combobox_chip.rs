use std::rc::Rc;

use gpui::{
    div, AccessibleAction, App, Div, InteractiveElement as _, IntoElement, MouseButton,
    ParentElement, RenderOnce, Role, StatefulInteractiveElement as _, StyleRefinement, Styled,
    Window,
};

use crate::combobox::{
    child_wiring::ComboboxChildNode, ComboboxChipChild, ComboboxChipStyleState, ComboboxContext,
};

/// One selected value in multiple mode. The chip highlight is virtual
/// runtime state driven from the input's keyboard.
#[derive(IntoElement)]
pub struct ComboboxChip<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<ComboboxChipChild<T>>,
    context: Option<ComboboxContext<T>>,
    index: Option<usize>,
    style_with_state: Option<Rc<dyn Fn(ComboboxChipStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for ComboboxChip<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            index: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for ComboboxChip<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for ComboboxChip<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context.clone() else {
            return div().into_any_element();
        };
        let index = self.index.unwrap_or(0);
        let state = context.read(cx, |runtime, props| runtime.chip_state(index, props));
        let (chip_label, set_size) = context.read(cx, |runtime, _| {
            (runtime.chip_label(index), runtime.selected_values().len())
        });
        let press_context = context.clone();
        let a11y_context = context.clone();
        let children = self
            .children
            .into_iter()
            .map(|child| match child {
                ComboboxChipChild::Remove(remove) => (*remove)
                    .index(index)
                    .with_combobox_context(context.clone())
                    .into_any_element(),
                ComboboxChipChild::Any(any) => any,
            })
            .collect::<Vec<_>>();
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        // AccessKit: the chip is `.on_mouse_down`-driven, so `Action::Click`
        // is NOT auto-registered — add it explicitly, routed through the same
        // highlight-then-focus path the pointer runs.
        base.id(("combobox-chip", index))
            .role(Role::ListItem)
            .aria_label(chip_label)
            .aria_position_in_set(index + 1)
            .aria_size_of_set(set_size)
            .on_a11y_action(AccessibleAction::Click, move |_data, window, cx| {
                a11y_context.update(cx, |runtime| runtime.highlight_chip(Some(index)));
                a11y_context.focus_input(window, cx);
            })
            .on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                press_context.update(cx, |runtime| runtime.highlight_chip(Some(index)));
                press_context.focus_input(window, cx);
            })
            .children(children)
            .into_any_element()
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxChip<T> {
    fn with_combobox_context(mut self, context: ComboboxContext<T>) -> Self {
        self.context = Some(context.clone());
        self.children = self
            .children
            .into_iter()
            .map(|child| child.with_combobox_context(context.clone()))
            .collect();
        self
    }
}

impl<T: Clone + Eq + 'static> ComboboxChip<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<ComboboxChipChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(ComboboxChipChild::Any(child.into_any_element()));
        self
    }

    /// Selection-order position of this chip; assigned by `ComboboxChips`.
    pub fn index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ComboboxChipStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
