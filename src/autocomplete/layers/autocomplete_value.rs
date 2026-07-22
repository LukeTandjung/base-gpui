use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Styled, Text, Window,
};

use crate::combobox::ComboboxContext;

type AutocompleteValueFormatter = Rc<dyn Fn(&str) -> SharedString + 'static>;

/// Displays the current displayed input value — including a live inline
/// overlay — mirroring Base UI's `Autocomplete.Value`, which reads the
/// resolved input value context. Static children take precedence, then the
/// formatter closure, then the raw value.
///
/// Accessibility: this part is a purely visual mirror of the input's value —
/// the input itself is the accessible value surface — so the resolved string
/// is rendered with `Text::new_inaccessible(...)` and the container carries no
/// role, keeping the value from being announced twice.
#[derive(IntoElement)]
pub struct AutocompleteValue<T: Clone + Eq + 'static> {
    base: Div,
    context: Option<ComboboxContext<T>>,
    formatter: Option<AutocompleteValueFormatter>,
    children: Vec<AnyElement>,
}

impl<T: Clone + Eq + 'static> Default for AutocompleteValue<T> {
    fn default() -> Self {
        Self {
            base: div(),
            context: None,
            formatter: None,
            children: Vec::new(),
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for AutocompleteValue<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for AutocompleteValue<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        if !self.children.is_empty() {
            return self.base.children(self.children);
        }

        let value = self
            .context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.display_value()))
            .unwrap_or_default();
        let display = self
            .formatter
            .as_ref()
            .map(|formatter| formatter(&value))
            .unwrap_or(value);

        self.base.child(Text::new_inaccessible(display))
    }
}

impl<T: Clone + Eq + 'static> AutocompleteValue<T> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Attaches the root context; called by `AutocompleteRoot` during child
    /// routing.
    pub fn with_context(mut self, context: ComboboxContext<T>) -> Self {
        self.context = Some(context);
        self
    }

    /// Formatter closure over the current value; ignored when static
    /// children are present.
    pub fn formatter(mut self, formatter: impl Fn(&str) -> SharedString + 'static) -> Self {
        self.formatter = Some(Rc::new(formatter));
        self
    }

    /// Static children take precedence over the raw value (Base UI children
    /// fallback order).
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}
