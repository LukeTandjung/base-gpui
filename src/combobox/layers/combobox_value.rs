use std::rc::Rc;

use gpui::{
    div, App, Div, IntoElement, ParentElement, RenderOnce, SharedString, StyleRefinement, Styled,
    Window,
};

use crate::combobox::{
    child_wiring::ComboboxChildNode, ComboboxContext, ComboboxSelectionMode,
    ComboboxValueStyleState,
};

type ComboboxValueFormatter<T> = Rc<dyn Fn(&ComboboxValueStyleState<T>) -> SharedString + 'static>;

type ComboboxValueStyle<T> = Rc<dyn Fn(ComboboxValueStyleState<T>, Div) -> Div + 'static>;

/// Displays the selected value: placeholder when nothing is selected, the
/// selected label in single mode, joined labels (or a caller formatter) in
/// multiple mode.
#[derive(IntoElement)]
pub struct ComboboxValue<T: Clone + Eq + 'static> {
    base: Div,
    context: Option<ComboboxContext<T>>,
    placeholder: Option<SharedString>,
    formatter: Option<ComboboxValueFormatter<T>>,
    style_with_state: Option<ComboboxValueStyle<T>>,
}

impl<T: Clone + Eq + 'static> Default for ComboboxValue<T> {
    fn default() -> Self {
        Self {
            base: div(),
            context: None,
            placeholder: None,
            formatter: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for ComboboxValue<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for ComboboxValue<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, props| {
                    runtime.value_state(props, self.placeholder.clone())
                })
            })
            .unwrap_or_else(|| {
                ComboboxValueStyleState::new(
                    ComboboxSelectionMode::Single,
                    None,
                    Vec::new(),
                    Vec::new(),
                    true,
                    self.placeholder.clone().unwrap_or_default(),
                )
            });
        let display_text = self
            .formatter
            .as_ref()
            .map(|formatter| formatter(&state))
            .unwrap_or_else(|| state.display_text.clone());
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.child(display_text)
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxValue<T> {
    fn with_combobox_context(mut self, context: ComboboxContext<T>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<T: Clone + Eq + 'static> ComboboxValue<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Rust-native formatter closure over the current selection.
    pub fn formatter(
        mut self,
        formatter: impl Fn(&ComboboxValueStyleState<T>) -> SharedString + 'static,
    ) -> Self {
        self.formatter = Some(Rc::new(formatter));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ComboboxValueStyleState<T>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
