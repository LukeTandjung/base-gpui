use gpui::{
    div, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled, Window,
};
use std::rc::Rc;

use crate::combobox::{
    child_wiring::{ComboboxChildNode, ComboboxChildWiring},
    ComboboxContext, ComboboxItem,
};

type ComboboxCollectionBuilder<T> = Rc<dyn Fn(&T, usize) -> ComboboxItem<T> + 'static>;

/// Data-driven item rendering from a `Vec<T>` plus a per-item builder
/// closure. Items materialize at wiring time so registered metadata is the
/// single registry filtering runs against, exactly like statically declared
/// items.
#[derive(IntoElement)]
pub struct ComboboxCollection<T: Clone + Eq + 'static> {
    base: Div,
    items: Vec<T>,
    builder: Option<ComboboxCollectionBuilder<T>>,
    wired_items: Vec<ComboboxItem<T>>,
}

impl<T: Clone + Eq + 'static> Default for ComboboxCollection<T> {
    fn default() -> Self {
        Self {
            base: div(),
            items: Vec::new(),
            builder: None,
            wired_items: Vec::new(),
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for ComboboxCollection<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for ComboboxCollection<T> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.base.children(
            self.wired_items
                .into_iter()
                .map(IntoElement::into_any_element)
                .collect::<Vec<_>>(),
        )
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxCollection<T> {
    fn with_combobox_context(mut self, context: ComboboxContext<T>) -> Self {
        self.wired_items = self
            .wired_items
            .into_iter()
            .map(|item| item.with_combobox_context(context.clone()))
            .collect();
        self
    }

    fn wire_combobox_child(
        mut self,
        wiring: &mut ComboboxChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let Some(builder) = self.builder.clone() else {
            return self;
        };
        self.wired_items = self
            .items
            .iter()
            .enumerate()
            .map(|(position, value)| {
                builder(value, position)
                    .id(("combobox-collection-item", position))
                    .wire_combobox_child(wiring, window, cx)
            })
            .collect();
        self
    }
}

impl<T: Clone + Eq + 'static> ComboboxCollection<T> {
    pub fn new(items: Vec<T>, builder: impl Fn(&T, usize) -> ComboboxItem<T> + 'static) -> Self {
        Self {
            base: div(),
            items,
            builder: Some(Rc::new(builder)),
            wired_items: Vec::new(),
        }
    }
}
