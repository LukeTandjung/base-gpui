use std::rc::Rc;

use gpui::{
    div, prelude::FluentBuilder as _, AnyElement, App, ClickEvent, Div, ElementId, FocusHandle,
    InteractiveElement as _, IntoElement, ParentElement, RenderOnce, Role, SharedString,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Toggled, Window,
};

use crate::menu::{
    child_wiring::{part_focus_handle, MenuChildNode, MenuChildWiring},
    MenuContext, MenuItemActivation, MenuItemChangeDetails, MenuItemKind, MenuItemMetadata,
    MenuOpenChangeSource, MenuRadioItemChild, MenuRadioItemStyleState, MenuValueChangeHandler,
};

type MenuRadioItemStyle = Rc<dyn Fn(MenuRadioItemStyleState, Div) -> Div + 'static>;

pub struct MenuRadioItem<P: Clone + 'static, V: Clone + Eq + 'static> {
    id: ElementId,
    base: Div,
    children: Vec<MenuRadioItemChild<P, V>>,
    context: Option<MenuContext<P>>,
    value: Option<V>,
    label: Option<SharedString>,
    disabled: bool,
    close_on_click: bool,
    index: Option<usize>,
    group_index: Option<usize>,
    group_disabled: bool,
    controlled_checked: Option<bool>,
    group_controlled: bool,
    group_value: Option<Option<V>>,
    on_value_change: Option<MenuValueChangeHandler<V>>,
    focus_handle: Option<FocusHandle>,
    style_with_state: Option<MenuRadioItemStyle>,
}

impl<P: Clone + 'static, V: Clone + Eq + 'static> Default for MenuRadioItem<P, V> {
    fn default() -> Self {
        Self {
            id: ElementId::from("menu-radio-item"),
            base: div(),
            children: Vec::new(),
            context: None,
            value: None,
            label: None,
            disabled: false,
            close_on_click: false,
            index: None,
            group_index: None,
            group_disabled: false,
            controlled_checked: None,
            group_controlled: false,
            group_value: None,
            on_value_change: None,
            focus_handle: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static, V: Clone + Eq + 'static> Styled for MenuRadioItem<P, V> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static, V: Clone + Eq + 'static> IntoElement for MenuRadioItem<P, V> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        MenuRadioItemElement { item: self }.into_any_element()
    }
}

#[derive(IntoElement)]
struct MenuRadioItemElement<P: Clone + 'static, V: Clone + Eq + 'static> {
    item: MenuRadioItem<P, V>,
}

impl<P: Clone + 'static, V: Clone + Eq + 'static> RenderOnce for MenuRadioItemElement<P, V> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let item = self.item;
        let Some(context) = item.context.clone() else {
            return div().into_any_element();
        };
        let focus_handle = item
            .focus_handle
            .clone()
            .unwrap_or_else(|| part_focus_handle(&item.id, window, cx));
        let checked =
            item.controlled_checked
                .unwrap_or_else(|| match (item.group_index, item.index) {
                    (Some(group_index), Some(index)) => {
                        context.read(cx, |runtime, _| runtime.radio_selected(group_index))
                            == Some(index)
                    }
                    _ => false,
                });
        let item_disabled = item.disabled || item.group_disabled;
        let (state, tab_stop) = context.read(cx, |runtime, props| {
            let mut state = runtime.radio_item_state(item.index, item_disabled, checked);
            state.disabled = state.disabled || props.disabled();
            (state, runtime.item_is_tab_stop(item.index))
        });
        let disabled = state.disabled;
        let index = item.index;
        let hover_context = context.clone();
        let click_context = context.clone();
        let measure_context = context.clone();

        let children = item
            .children
            .into_iter()
            .map(|child| match child {
                MenuRadioItemChild::Indicator(indicator) => (*indicator)
                    .with_item_facts(checked, state.highlighted, state.disabled)
                    .into_any_element(),
                MenuRadioItemChild::Any(any) => any,
            })
            .collect::<Vec<AnyElement>>();

        let base = match item.style_with_state {
            Some(style_with_state) => style_with_state(state, item.base),
            None => item.base,
        };

        div()
            .on_children_prepainted(move |bounds, window, cx| {
                let (Some(bounds), Some(index)) = (bounds.first().copied(), index) else {
                    return;
                };
                let changed =
                    measure_context.update(cx, |runtime| runtime.set_item_bounds(index, bounds));
                if changed {
                    window.request_animation_frame();
                }
            })
            .child(
                base.id(item.id)
                    .role(Role::MenuItemRadio)
                    .aria_toggled(match checked {
                        true => Toggled::True,
                        false => Toggled::False,
                    })
                    .when_some(item.label.clone(), |this, label| this.aria_label(label))
                    .track_focus(
                        &focus_handle
                            .tab_stop(tab_stop && !disabled)
                            .tab_index(if tab_stop && !disabled { 0 } else { -1 }),
                    )
                    .focusable()
                    .on_mouse_move(move |_event, _window, cx| {
                        let Some(index) = index else {
                            return;
                        };
                        let should_highlight = hover_context
                            .read(cx, |_runtime, props| props.highlight_item_on_hover());
                        if !should_highlight || disabled {
                            return;
                        }
                        hover_context.update(cx, |runtime| {
                            runtime.highlight_item_from_pointer(index);
                        });
                    })
                    .on_click(move |event, window, cx| {
                        if !matches!(event, ClickEvent::Mouse(_)) {
                            return;
                        }
                        let Some(index) = index else {
                            return;
                        };
                        if disabled {
                            return;
                        }
                        click_context.activate_item(
                            index,
                            MenuOpenChangeSource::Pointer,
                            window,
                            cx,
                        );
                    })
                    .children(children),
            )
            .into_any_element()
    }
}

impl<P: Clone + 'static, V: Clone + Eq + 'static> MenuChildNode<P> for MenuRadioItem<P, V> {
    fn wire_menu_child(
        mut self,
        wiring: &mut MenuChildWiring<P>,
        context: &MenuContext<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let index = wiring.next_item_index();
        let scoped_id = wiring.scope_child_id(&ElementId::from(SharedString::from(format!(
            "{}-{index}",
            self.id
        ))));
        let focus_handle = part_focus_handle(&scoped_id, window, cx);
        let item_disabled = self.disabled || self.group_disabled;

        self.controlled_checked = self
            .group_value
            .as_ref()
            .map(|group_value| group_value.as_ref() == self.value.as_ref());

        let activation: Option<MenuItemActivation> = match (self.value.clone(), self.group_index) {
            (Some(value), Some(group_index)) => {
                let select_context = context.clone();
                let handler = self.on_value_change.clone();
                let group_controlled = self.group_controlled;
                Some(Rc::new(move |window: &mut Window, cx: &mut App| {
                    let mut details = MenuItemChangeDetails::new();
                    if let Some(handler) = handler.as_ref() {
                        handler(value.clone(), &mut details, window, cx);
                    }
                    if details.is_canceled() {
                        return;
                    }
                    if !group_controlled {
                        select_context
                            .update(cx, |runtime| runtime.commit_radio(group_index, index));
                    }
                }))
            }
            _ => None,
        };

        let mut metadata = MenuItemMetadata::new(
            index,
            MenuItemKind::RadioItem,
            self.label.clone(),
            item_disabled,
            self.close_on_click,
        )
        .with_focus_handle(focus_handle.clone());
        if let Some(activation) = activation {
            metadata = metadata.with_activation(activation);
        }
        wiring.register_item(metadata);
        self.id = scoped_id;
        self.index = Some(index);
        self.focus_handle = Some(focus_handle);
        self.context = Some(context.clone());
        self
    }
}

impl<P: Clone + 'static, V: Clone + Eq + 'static> MenuRadioItem<P, V> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Called by `MenuRadioGroup` wiring before item wiring: injects group
    /// facts so radio state resolution stays with the group.
    pub fn prepare_for_group(
        mut self,
        group_index: usize,
        group_disabled: bool,
        group_value: Option<Option<V>>,
        on_value_change: Option<MenuValueChangeHandler<V>>,
    ) -> Self {
        self.group_index = Some(group_index);
        self.group_disabled = group_disabled;
        self.group_controlled = group_value.is_some();
        self.group_value = group_value;
        self.on_value_change = on_value_change;
        self
    }

    pub fn wired_index(&self) -> Option<usize> {
        self.index
    }

    pub fn item_value(&self) -> Option<&V> {
        self.value.as_ref()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn child(mut self, child: impl Into<MenuRadioItemChild<P, V>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(MenuRadioItemChild::Any(child.into_any_element()));
        self
    }

    pub fn value(mut self, value: V) -> Self {
        self.value = Some(value);
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn close_on_click(mut self, close_on_click: bool) -> Self {
        self.close_on_click = close_on_click;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(MenuRadioItemStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
