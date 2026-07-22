use std::{rc::Rc, sync::Arc};

use gpui::{
    div, App, Div, ElementId, Entity, FocusHandle, InteractiveElement as _, IntoElement,
    ParentElement, RenderOnce, Role, SharedString, StatefulInteractiveElement as _,
    StyleRefinement, Styled, Window,
};

use crate::navigation_menu::{
    child_wiring::{scoped_value_id, NavigationMenuChildNode, NavigationMenuChildWiring},
    NavigationMenuContext, NavigationMenuItemChild, NavigationMenuItemMetadata,
    NavigationMenuItemStyleState, NavigationMenuListEntry,
};

type NavigationMenuItemStyle = Rc<dyn Fn(NavigationMenuItemStyleState, Div) -> Div + 'static>;

/// One menu item keyed by a required value (Base UI's auto-generated
/// fallback id is not portable to a generic `T`). Hosts the trigger; the
/// item's content is routed to the shared popup viewport by child wiring and
/// never rendered in place.
#[derive(IntoElement)]
pub struct NavigationMenuItem<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<NavigationMenuItemChild<T>>,
    context: Option<NavigationMenuContext<T>>,
    value: Option<T>,
    order: usize,
    style_with_state: Option<NavigationMenuItemStyle>,
}

impl<T: Clone + Eq + 'static> Default for NavigationMenuItem<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            value: None,
            order: 0,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for NavigationMenuItem<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for NavigationMenuItem<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (state, item_count) = self
            .context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, _| {
                    (runtime.item_state(), runtime.item_count())
                })
            })
            .unwrap_or_default();

        let mut base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        }
        .id(("navigation-menu-item", self.order));

        // Base UI renders `<li>`; wired items report "item i of n" from the
        // order/count child wiring assigned.
        if self.context.is_some() {
            base = base
                .role(Role::ListItem)
                .aria_position_in_set(self.order + 1)
                .aria_size_of_set(item_count);
        }

        base.children(self.children.into_iter().filter_map(|child| match child {
            NavigationMenuItemChild::Trigger(trigger) => Some((*trigger).into_any_element()),
            // Content is routed to the viewport by wiring; an unwired
            // content (root without a viewport) is dropped, never rendered
            // in place.
            NavigationMenuItemChild::Content(_) => None,
            NavigationMenuItemChild::Any(any) => Some(any),
        }))
    }
}

impl<T: Clone + Eq + 'static> NavigationMenuChildNode<T> for NavigationMenuItem<T> {
    fn with_navigation_menu_context(mut self, context: NavigationMenuContext<T>) -> Self {
        self.context = Some(context.clone());
        self.children = self
            .children
            .into_iter()
            .map(|child| match child {
                NavigationMenuItemChild::Trigger(trigger) => NavigationMenuItemChild::Trigger(
                    Box::new(trigger.with_navigation_menu_context(context.clone())),
                ),
                other => other,
            })
            .collect();
        self
    }

    fn wire_navigation_menu_child(
        mut self,
        wiring: &mut NavigationMenuChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let Some(value) = self.value.clone() else {
            return self;
        };

        let disabled = self.children.iter().any(|child| match child {
            NavigationMenuItemChild::Trigger(trigger) => trigger.is_disabled(),
            _ => false,
        });
        let order = wiring.next_item_order();
        let focus_handle = item_trigger_focus_handle(
            &scoped_value_id(&wiring.root_id(), "navigation-menu-trigger", order),
            window,
            cx,
        );
        let order = wiring.register_item(NavigationMenuItemMetadata::new(
            value.clone(),
            disabled,
            Some(focus_handle.clone()),
            order,
        ));
        let entry_index = wiring.register_list_entry(NavigationMenuListEntry::new(
            Some(focus_handle.clone()),
            Some(value.clone()),
            disabled,
        ));
        self.order = order;

        self.children = self
            .children
            .into_iter()
            .filter_map(|child| match child {
                NavigationMenuItemChild::Trigger(trigger) => {
                    Some(NavigationMenuItemChild::Trigger(Box::new(trigger.wired(
                        value.clone(),
                        focus_handle.clone(),
                        entry_index,
                        order,
                    ))))
                }
                NavigationMenuItemChild::Content(content) => {
                    wiring.collect_content(content.wired(value.clone()));
                    None
                }
                other => Some(other),
            })
            .collect();
        self
    }
}

pub fn item_trigger_focus_handle(id: &ElementId, window: &mut Window, cx: &mut App) -> FocusHandle {
    let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
        ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("focus")),
        cx,
        |_, cx| cx.focus_handle(),
    );

    focus_handle_entity.read(cx).clone()
}

impl<T: Clone + Eq + 'static> NavigationMenuItem<T> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Required: the item's value. Items without a value are not registered.
    pub fn value(mut self, value: T) -> Self {
        self.value = Some(value);
        self
    }

    pub fn child(mut self, child: impl Into<NavigationMenuItemChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(NavigationMenuItemChild::Any(child.into_any_element()));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(NavigationMenuItemStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
