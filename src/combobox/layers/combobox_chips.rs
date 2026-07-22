use std::rc::Rc;

use gpui::{
    div, App, Div, InteractiveElement as _, IntoElement, Orientation, ParentElement, RenderOnce,
    Role, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::combobox::{
    child_wiring::ComboboxChildNode, ComboboxChip, ComboboxChipRemove, ComboboxChipsStyleState,
    ComboboxContext,
};

type ComboboxChipBuilder<T> = Rc<dyn Fn(&T, usize) -> ComboboxChip<T> + 'static>;

/// Renders one `ComboboxChip` per selected value in selection order; chips
/// compose as plain layers over runtime `Vec<T>` state. A builder closure can
/// replace the default chip rendering.
#[derive(IntoElement)]
pub struct ComboboxChips<T: Clone + Eq + 'static> {
    base: Div,
    context: Option<ComboboxContext<T>>,
    chip_builder: Option<ComboboxChipBuilder<T>>,
    style_with_state: Option<Rc<dyn Fn(ComboboxChipsStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for ComboboxChips<T> {
    fn default() -> Self {
        Self {
            base: div(),
            context: None,
            chip_builder: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for ComboboxChips<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for ComboboxChips<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context.clone() else {
            return div().into_any_element();
        };
        let (state, values) = context.read(cx, |runtime, _| {
            (runtime.chips_state(), runtime.selected_values())
        });
        let chips = values
            .iter()
            .enumerate()
            .map(|(index, value)| {
                let chip = match self.chip_builder.as_ref() {
                    Some(builder) => builder(value, index),
                    None => ComboboxChip::new()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1()
                        .child_any(context.read(cx, |runtime, _| runtime.chip_label(index)))
                        .child(ComboboxChipRemove::new()),
                };
                chip.index(index)
                    .with_combobox_context(context.clone())
                    .into_any_element()
            })
            .collect::<Vec<_>>();
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        // AccessKit: Base UI renders `role="toolbar"` only when selection
        // chips exist.
        if chips.is_empty() {
            base.into_any_element()
        } else {
            base.id("combobox-chips")
                .role(Role::Toolbar)
                .aria_orientation(Orientation::Horizontal)
                .children(chips)
                .into_any_element()
        }
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxChips<T> {
    fn with_combobox_context(mut self, context: ComboboxContext<T>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<T: Clone + Eq + 'static> ComboboxChips<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn chip_builder(
        mut self,
        builder: impl Fn(&T, usize) -> ComboboxChip<T> + 'static,
    ) -> Self {
        self.chip_builder = Some(Rc::new(builder));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ComboboxChipsStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
