use std::{rc::Rc, time::Duration};

use gpui::{
    div, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    Role, SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::navigation_menu::{
    child_wiring::wire_children, layers::navigation_menu_trigger::evaluate_safe_polygon_move,
    NavigationMenuChild, NavigationMenuContext, NavigationMenuOpenChangeCompleteHandler,
    NavigationMenuOrientation, NavigationMenuProps, NavigationMenuRootStyleState,
    NavigationMenuValueChangeDetails, NavigationMenuValueChangeHandler,
    DEFAULT_NAVIGATION_MENU_CLOSE_DELAY, DEFAULT_NAVIGATION_MENU_DELAY,
};

type NavigationMenuRootStyle = Rc<dyn Fn(NavigationMenuRootStyleState, Div) -> Div + 'static>;

/// Value-driven root: open is derived from `value: Option<T>` (`None` =
/// closed). Base UI's `actionsRef.unmount` manual-unmount escape hatch is
/// deferred alongside transition infrastructure.
///
/// Accessibility: a non-nested root enters the AccessKit tree as
/// `Role::Navigation` (Base UI's `<nav>`); pass `.aria_label(...)` so
/// multiple navigation landmarks are distinguishable. A nested root gets no
/// role (plain container, Base UI's nested `<div>`). Live announcements for
/// content switches have no gpui API and are omitted (blocked pending gpui
/// upstream).
#[derive(IntoElement)]
pub struct NavigationMenuRoot<T: Clone + Eq + 'static> {
    id: ElementId,
    base: Div,
    children: Vec<NavigationMenuChild<T>>,
    default_value: Option<T>,
    value: Option<Option<T>>,
    delay: Duration,
    close_delay: Duration,
    orientation: NavigationMenuOrientation,
    nested: bool,
    on_value_change: Option<NavigationMenuValueChangeHandler<T>>,
    on_open_change_complete: Option<NavigationMenuOpenChangeCompleteHandler<T>>,
    aria_label: Option<SharedString>,
    style_with_state: Option<NavigationMenuRootStyle>,
}

impl<T: Clone + Eq + 'static> Default for NavigationMenuRoot<T> {
    fn default() -> Self {
        Self {
            id: ElementId::from("navigation-menu"),
            base: div(),
            children: Vec::new(),
            default_value: None,
            value: None,
            delay: DEFAULT_NAVIGATION_MENU_DELAY,
            close_delay: DEFAULT_NAVIGATION_MENU_CLOSE_DELAY,
            orientation: NavigationMenuOrientation::Horizontal,
            nested: false,
            on_value_change: None,
            on_open_change_complete: None,
            aria_label: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for NavigationMenuRoot<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for NavigationMenuRoot<T> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context = NavigationMenuContext::new(
            self.id.clone(),
            cx,
            window,
            self.value.clone(),
            self.default_value.clone(),
            NavigationMenuProps::new(
                self.delay,
                self.close_delay,
                self.orientation,
                self.on_value_change,
                self.on_open_change_complete,
            ),
        );

        let wired_children = wire_children(self.children, context.clone(), window, cx);
        let items = wired_children.items;
        let list_entries = wired_children.list_entries;
        let children = wired_children.children;

        let state = context.update(cx, |runtime| {
            runtime.sync_children(items, list_entries);
            runtime.root_state(self.nested)
        });

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        let mut base = base.id(self.id.clone());
        if !self.nested {
            base = base.role(Role::Navigation);
            if let Some(aria_label) = self.aria_label.clone() {
                base = base.aria_label(aria_label);
            }
        }

        let move_context = context.clone();
        base.on_mouse_move(move |event, window, cx| {
            evaluate_safe_polygon_move(&move_context, event.position, window, cx);
        })
        .children(children)
    }
}

impl<T: Clone + Eq + 'static> NavigationMenuRoot<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn child(mut self, child: impl Into<NavigationMenuChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<NavigationMenuChild<T>>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(NavigationMenuChild::Any(child.into_any_element()));
        self
    }

    pub fn default_value(mut self, default_value: Option<T>) -> Self {
        self.default_value = default_value;
        self
    }

    pub fn value(mut self, value: Option<T>) -> Self {
        self.value = Some(value);
        self
    }

    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    pub fn close_delay(mut self, close_delay: Duration) -> Self {
        self.close_delay = close_delay;
        self
    }

    pub fn orientation(mut self, orientation: NavigationMenuOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Marks this root as nested inside another navigation menu's content
    /// (renders inline; style state reports `nested`).
    pub fn nested(mut self, nested: bool) -> Self {
        self.nested = nested;
        self
    }

    /// Accessible label for the navigation landmark (non-nested roots only;
    /// nested roots carry no role).
    pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn on_value_change(
        mut self,
        on_value_change: impl Fn(Option<T>, &mut NavigationMenuValueChangeDetails, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(on_value_change));
        self
    }

    pub fn on_open_change_complete(
        mut self,
        on_open_change_complete: impl Fn(Option<T>, &NavigationMenuValueChangeDetails, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.on_open_change_complete = Some(Rc::new(on_open_change_complete));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(NavigationMenuRootStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
