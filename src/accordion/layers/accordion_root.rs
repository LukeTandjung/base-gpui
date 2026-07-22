use std::rc::Rc;

use gpui::{
    div, App, Div, ElementId, InteractiveElement as _, IntoElement, Orientation, ParentElement,
    RenderOnce, Role, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::accordion::{
    child_wiring::wire_children, AccordionContext, AccordionOrientation, AccordionProps,
    AccordionRootChild, AccordionRootStyleState, AccordionValueChangeDetails,
    AccordionValueChangeHandler,
};

#[derive(IntoElement)]
pub struct AccordionRoot<T: Clone + Eq + 'static> {
    id: ElementId,
    base: Div,
    children: Vec<AccordionRootChild<T>>,
    default_value: Vec<T>,
    value: Option<Vec<T>>,
    disabled: bool,
    multiple: bool,
    keep_mounted: bool,
    orientation: AccordionOrientation,
    on_value_change: Option<AccordionValueChangeHandler<T>>,
    style_with_state: Option<Rc<dyn Fn(AccordionRootStyleState<T>, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for AccordionRoot<T> {
    fn default() -> Self {
        Self {
            id: ElementId::from("accordion"),
            base: div(),
            children: Vec::new(),
            default_value: Vec::new(),
            value: None,
            disabled: false,
            multiple: false,
            keep_mounted: false,
            orientation: AccordionOrientation::Vertical,
            on_value_change: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for AccordionRoot<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for AccordionRoot<T> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context = AccordionContext::new(
            self.id.clone(),
            cx,
            window,
            self.value,
            self.default_value,
            AccordionProps::new(
                self.disabled,
                self.multiple,
                self.keep_mounted,
                self.orientation,
                self.on_value_change,
            ),
        );
        let wired_children = wire_children(self.children, context.clone(), window, cx);
        let children = wired_children.children;

        context.update(cx, |runtime| {
            runtime.sync_children(wired_children.items);
        });

        let style_state = context.read(cx, |runtime, props| runtime.root_state(props));
        let orientation = match style_state.orientation {
            AccordionOrientation::Horizontal => Orientation::Horizontal,
            AccordionOrientation::Vertical => Orientation::Vertical,
        };
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        base.id(self.id)
            .role(Role::Group)
            .aria_orientation(orientation)
            .children(children)
    }
}

impl<T: Clone + Eq + 'static> AccordionRoot<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<AccordionRootChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<AccordionRootChild<T>>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
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

    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    pub fn orientation(mut self, orientation: AccordionOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn on_value_change(
        mut self,
        on_value_change: impl Fn(&[T], &mut AccordionValueChangeDetails, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(on_value_change));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(AccordionRootStyleState<T>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
