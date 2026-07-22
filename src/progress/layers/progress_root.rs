use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, Role, SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled,
    Window,
};

use crate::progress::{ProgressChild, ProgressContext, ProgressProps, ProgressStyleState};

#[derive(IntoElement)]
pub struct ProgressRoot {
    id: ElementId,
    base: Div,
    children: Vec<ProgressChild>,
    value: Option<f64>,
    min: f64,
    max: f64,
    format: Option<Rc<dyn Fn(f64) -> String + 'static>>,
    label: Option<SharedString>,
    style_with_state: Option<Rc<dyn Fn(ProgressStyleState, Div) -> Div + 'static>>,
}

impl Default for ProgressRoot {
    fn default() -> Self {
        Self {
            id: ElementId::from("progress"),
            base: div(),
            children: Vec::from([]),
            value: None,
            min: 0.0,
            max: 100.0,
            format: None,
            label: None,
            style_with_state: None,
        }
    }
}

impl Styled for ProgressRoot {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for ProgressRoot {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let props = ProgressProps::new(self.value, self.min, self.max, self.format);
        let context = ProgressContext::new(self.id.clone(), &props);

        let children: Vec<AnyElement> = self
            .children
            .into_iter()
            .map(|child| match child {
                ProgressChild::Track(track) => track
                    .with_progress_context(context.clone())
                    .into_any_element(),
                ProgressChild::Value(value) => value
                    .with_progress_context(context.clone())
                    .into_any_element(),
                ProgressChild::Label(label) => label
                    .with_progress_context(context.clone())
                    .into_any_element(),
                ProgressChild::Any(any) => any,
            })
            .collect();

        let style_state = context.read(|runtime| runtime.state());
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state.clone(), self.base),
            None => self.base,
        };

        // Base UI's `role="progressbar"` + `aria-valuemin/max/now`. No
        // `.aria_valuetext` builder exists in this gpui revision, so the
        // formatted string / `"indeterminate progress"` default is omitted;
        // indeterminate is conveyed by omitting the numeric value.
        let base = base
            .id(self.id)
            .role(Role::ProgressIndicator)
            .aria_min_numeric_value(style_state.min)
            .aria_max_numeric_value(style_state.max);
        let base = match style_state.clamped_value {
            Some(value) => base.aria_numeric_value(value),
            None => base,
        };
        let base = match self.label {
            Some(label) => base.aria_label(label),
            None => base,
        };

        base.children(children)
    }
}

impl ProgressRoot {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn child(mut self, child: impl Into<ProgressChild>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<ProgressChild>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(ProgressChild::Any(child.into_any_element()));
        self
    }

    /// The task-completion value; `None` means indeterminate (Base UI's
    /// `@default null`).
    pub fn value(mut self, value: Option<f64>) -> Self {
        self.value = value;
        self
    }

    pub fn min(mut self, min: f64) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }

    /// Custom formatter receiving the raw (unclamped) value; never invoked
    /// when indeterminate.
    pub fn format(mut self, format: impl Fn(f64) -> String + 'static) -> Self {
        self.format = Some(Rc::new(format));
        self
    }

    /// Accessible label for the progress bar; the literal-string replacement
    /// for Base UI's `aria-labelledby` wiring to `ProgressLabel`. Pass the
    /// same string rendered inside `ProgressLabel`.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ProgressStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
