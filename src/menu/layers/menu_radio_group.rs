use std::rc::Rc;

use gpui::{
    div, prelude::FluentBuilder as _, AnyElement, App, Div, ElementId, InteractiveElement as _,
    IntoElement, ParentElement, Role, SharedString, StatefulInteractiveElement as _,
    StyleRefinement, Styled, Window,
};

use crate::menu::{
    child_wiring::{MenuChildNode, MenuChildWiring, MenuRadioGroupErased},
    MenuContext, MenuRadioGroupChild, MenuRadioGroupStyleState, MenuValueChangeHandler,
};

type MenuRadioGroupStyle = Rc<dyn Fn(MenuRadioGroupStyleState, Div) -> Div + 'static>;

pub struct MenuRadioGroup<P: Clone + 'static, V: Clone + Eq + 'static> {
    id: ElementId,
    base: Div,
    aria_label: Option<SharedString>,
    children: Vec<MenuRadioGroupChild<P, V>>,
    context: Option<MenuContext<P>>,
    value: Option<Option<V>>,
    default_value: Option<V>,
    disabled: bool,
    on_value_change: Option<MenuValueChangeHandler<V>>,
    group_index: Option<usize>,
    style_with_state: Option<MenuRadioGroupStyle>,
}

impl<P: Clone + 'static, V: Clone + Eq + 'static> Default for MenuRadioGroup<P, V> {
    fn default() -> Self {
        Self {
            id: ElementId::from("menu-radio-group"),
            base: div(),
            aria_label: None,
            children: Vec::new(),
            context: None,
            value: None,
            default_value: None,
            disabled: false,
            on_value_change: None,
            group_index: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static, V: Clone + Eq + 'static> Styled for MenuRadioGroup<P, V> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static, V: Clone + Eq + 'static> IntoElement for MenuRadioGroup<P, V> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let state = MenuRadioGroupStyleState::new(self.disabled);
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        // `aria-labelledby` has no gpui builder (documented gap): the group
        // label registered by `MenuGroupLabel` is surfaced here as a literal
        // `.aria_label(...)` instead. `disabled` is not announced
        // (no `.aria_disabled` builder); disabled groups stay inert through
        // withheld tab stops and activation no-ops.
        base.id(self.id)
            .role(Role::Group)
            .when_some(self.aria_label, |this, label| this.aria_label(label))
            .children(
                self.children
                    .into_iter()
                    .map(IntoElement::into_element)
                    .collect::<Vec<_>>(),
            )
            .into_any_element()
    }
}

impl<P: Clone + 'static, V: Clone + Eq + 'static> MenuRadioGroupErased<P> for MenuRadioGroup<P, V> {
    fn wire_erased(
        self: Box<Self>,
        wiring: &mut MenuChildWiring<P>,
        context: &MenuContext<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Box<dyn MenuRadioGroupErased<P>> {
        Box::new(self.wire(wiring, context, window, cx))
    }

    fn into_any(self: Box<Self>) -> AnyElement {
        (*self).into_any_element()
    }
}

impl<P: Clone + 'static, V: Clone + Eq + 'static> MenuRadioGroup<P, V> {
    fn wire(
        mut self,
        wiring: &mut MenuChildWiring<P>,
        context: &MenuContext<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let group_index = wiring.next_radio_group_index();
        self.group_index = Some(group_index);
        if self.aria_label.is_none() {
            self.aria_label = self.children.iter().find_map(|child| match child {
                MenuRadioGroupChild::GroupLabel(label) => label.label_value(),
                _ => None,
            });
        }
        self.id = wiring.scope_child_id(&ElementId::from(SharedString::from(format!(
            "{}-{group_index}",
            self.id
        ))));

        let controlled_value = self.value.clone();
        let default_value = self.default_value.clone();
        let mut default_selected: Option<usize> = None;
        let group_disabled = self.disabled;
        let handler = self.on_value_change.clone();

        self.children = self
            .children
            .into_iter()
            .map(|child| match child {
                MenuRadioGroupChild::RadioItem(item) => {
                    let item = (*item).prepare_for_group(
                        group_index,
                        group_disabled,
                        controlled_value.clone(),
                        handler.clone(),
                    );
                    let item = item.wire_menu_child(wiring, context, window, cx);
                    if let (Some(default_value), Some(index), Some(item_value)) = (
                        default_value.as_ref(),
                        item.wired_index(),
                        item.item_value(),
                    ) {
                        if default_value == item_value {
                            default_selected = Some(index);
                        }
                    }
                    MenuRadioGroupChild::RadioItem(Box::new(item))
                }
                other => other.wire_menu_child(wiring, context, window, cx),
            })
            .collect();

        wiring.register_radio_default(group_index, default_selected);
        self.context = Some(context.clone());
        self
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    /// Accessible group label. Defaults to the label registered by a
    /// `MenuGroupLabel` child.
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn child(mut self, child: impl Into<MenuRadioGroupChild<P, V>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(MenuRadioGroupChild::Any(child.into_any_element()));
        self
    }

    pub fn value(mut self, value: Option<V>) -> Self {
        self.value = Some(value);
        self
    }

    pub fn default_value(mut self, default_value: Option<V>) -> Self {
        self.default_value = default_value;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_value_change(
        mut self,
        on_value_change: impl Fn(V, &mut crate::menu::MenuItemChangeDetails, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(on_value_change));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(MenuRadioGroupStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
