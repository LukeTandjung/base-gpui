use std::rc::Rc;

use gpui::{
    div, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    Role, SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::toggle_group::{
    child_wiring::wire_children, ToggleGroupChild, ToggleGroupContext, ToggleGroupOrientation,
    ToggleGroupProps, ToggleGroupStyleState, ToggleGroupValueChangeDetails,
    ToggleGroupValueChangeHandler,
};

/// Accessibility: the root renders with `Role::Group` (Base UI's `role="group"`);
/// pass `.aria_label(...)` to name the group for assistive technology. Base UI
/// deliberately renders no `aria-orientation` on the group, so none is set here.
/// The pinned gpui revision has no `aria_disabled` builder, so a disabled group is
/// not *announced* as disabled to AT; disabled toggles are still action-inert
/// (activation guards and tab-stop removal already apply).
#[derive(IntoElement)]
pub struct ToggleGroup<T: Clone + Eq + 'static> {
    id: ElementId,
    base: Div,
    aria_label: Option<SharedString>,
    children: Vec<ToggleGroupChild<T>>,
    default_value: Vec<T>,
    value: Option<Vec<T>>,
    disabled: bool,
    orientation: ToggleGroupOrientation,
    multiple: bool,
    loop_focus: bool,
    on_value_change: Option<ToggleGroupValueChangeHandler<T>>,
    style_with_state: Option<Rc<dyn Fn(ToggleGroupStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for ToggleGroup<T> {
    fn default() -> Self {
        Self {
            id: ElementId::from("toggle-group"),
            base: div(),
            aria_label: None,
            children: Vec::new(),
            default_value: Vec::new(),
            value: None,
            disabled: false,
            orientation: ToggleGroupOrientation::Horizontal,
            multiple: false,
            loop_focus: true,
            on_value_change: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for ToggleGroup<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for ToggleGroup<T> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let controlled = self.value.clone();
        let value_initialized = self.value.is_some() || !self.default_value.is_empty();
        let context = ToggleGroupContext::new(
            self.id.clone(),
            cx,
            window,
            self.value,
            self.default_value,
            ToggleGroupProps::new(
                self.disabled,
                self.orientation,
                self.multiple,
                self.loop_focus,
                self.on_value_change,
            ),
        );

        let wired_children = wire_children(
            self.children,
            context.clone(),
            self.disabled,
            value_initialized,
            window,
            cx,
        );
        let toggles = wired_children.toggles;
        let focus_handles = wired_children.focus_handles;
        let focused_index = wired_children.focused_index;
        let children = wired_children.children;

        context.update(cx, |runtime| {
            runtime.sync_children(toggles, focus_handles);
            runtime.sync_focused_index(focused_index);

            let observed_value = controlled.unwrap_or_else(|| runtime.value_vec());
            runtime.reconcile(observed_value);
        });

        let style_state = context.read(cx, |runtime, props| runtime.group_state(props));
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        let mut base = base.id(self.id.clone()).role(Role::Group);
        if let Some(aria_label) = self.aria_label.clone() {
            base = base.aria_label(aria_label);
        }

        base.children(children)
    }
}

impl<T: Clone + Eq + 'static> ToggleGroup<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<ToggleGroupChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<ToggleGroupChild<T>>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn default_value(mut self, default_value: Vec<T>) -> Self {
        self.default_value = default_value;
        self
    }

    pub fn value(mut self, value: Vec<T>) -> Self {
        self.value = Some(value);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn orientation(mut self, orientation: ToggleGroupOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    pub fn loop_focus(mut self, loop_focus: bool) -> Self {
        self.loop_focus = loop_focus;
        self
    }

    pub fn on_value_change(
        mut self,
        on_value_change: impl Fn(&[T], &mut ToggleGroupValueChangeDetails, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(on_value_change));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ToggleGroupStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
