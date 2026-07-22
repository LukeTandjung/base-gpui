use std::rc::Rc;

use gpui::{
    div, App, Div, InteractiveElement as _, IntoElement, MouseButton, ParentElement, RenderOnce,
    StyleRefinement, Styled, Window,
};

use crate::combobox::{
    child_wiring::{ComboboxChildNode, ComboboxChildWiring},
    ComboboxChangeReason, ComboboxChangeSource, ComboboxContext, ComboboxInputGroupChild,
    ComboboxInputGroupStyleState, ComboboxSide,
};

type ComboboxInputGroupStyle<T> = Rc<dyn Fn(ComboboxInputGroupStyleState<T>, Div) -> Div + 'static>;

/// Wraps the input and adjacent controls (clear, trigger, chips). Pressing
/// its non-interactive area focuses the input and opens the popup when
/// `open_on_input_click` (Base UI `handleInputPress`). It is also the
/// preferred positioner anchor when present.
#[derive(IntoElement)]
pub struct ComboboxInputGroup<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<ComboboxInputGroupChild<T>>,
    context: Option<ComboboxContext<T>>,
    style_with_state: Option<ComboboxInputGroupStyle<T>>,
}

impl<T: Clone + Eq + 'static> Default for ComboboxInputGroup<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for ComboboxInputGroup<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for ComboboxInputGroup<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context.clone() else {
            return div();
        };
        let state = context.read(cx, |runtime, props| {
            runtime.input_group_state(props, ComboboxSide::Bottom)
        });
        let disabled = state.root.disabled;
        let read_only = state.root.read_only;
        let open_on_input_click = context.props().open_on_input_click;
        let press_context = context.clone();
        let measure_context = context.clone();
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        let group = base
            .on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                if disabled {
                    return;
                }
                press_context.focus_input(window, cx);
                if open_on_input_click && !read_only {
                    press_context.set_open(
                        true,
                        ComboboxChangeReason::None,
                        ComboboxChangeSource::Pointer,
                        window,
                        cx,
                    );
                }
            })
            .children(
                self.children
                    .into_iter()
                    .map(IntoElement::into_element)
                    .collect::<Vec<_>>(),
            );

        // The measuring wrapper must span the available width so the group's
        // own percentage sizing (`w_full`) has something to resolve against.
        div()
            .w_full()
            .on_children_prepainted(move |bounds, window, cx| {
                let Some(bounds) = bounds.first().copied() else {
                    return;
                };
                if measure_context.record_input_group_bounds(bounds, cx) {
                    window.request_animation_frame();
                }
            })
            .child(group)
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxInputGroup<T> {
    fn with_combobox_context(mut self, context: ComboboxContext<T>) -> Self {
        self.context = Some(context.clone());
        self.children = self
            .children
            .into_iter()
            .map(|child| child.with_combobox_context(context.clone()))
            .collect();
        self
    }

    fn wire_combobox_child(
        mut self,
        wiring: &mut ComboboxChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        self.children = wiring.wire_input_group_children(self.children, window, cx);
        self
    }
}

impl<T: Clone + Eq + 'static> ComboboxInputGroup<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<ComboboxInputGroupChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(ComboboxInputGroupChild::Any(child.into_any_element()));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ComboboxInputGroupStyleState<T>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
