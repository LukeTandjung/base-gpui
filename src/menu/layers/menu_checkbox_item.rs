use std::rc::Rc;

use gpui::{
    div, prelude::FluentBuilder as _, AnyElement, App, ClickEvent, Div, ElementId, FocusHandle,
    InteractiveElement as _, IntoElement, ParentElement, RenderOnce, Role, SharedString,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Toggled, Window,
};

use crate::menu::{
    child_wiring::{part_focus_handle, MenuChildNode, MenuChildWiring},
    MenuCheckboxItemChild, MenuCheckboxItemStyleState, MenuCheckedChangeHandler, MenuContext,
    MenuItemActivation, MenuItemChangeDetails, MenuItemKind, MenuItemMetadata,
    MenuOpenChangeSource,
};

type MenuCheckboxItemStyle = Rc<dyn Fn(MenuCheckboxItemStyleState, Div) -> Div + 'static>;

#[derive(IntoElement)]
pub struct MenuCheckboxItem<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<MenuCheckboxItemChild<P>>,
    context: Option<MenuContext<P>>,
    label: Option<SharedString>,
    disabled: bool,
    close_on_click: bool,
    checked: Option<bool>,
    default_checked: bool,
    on_checked_change: Option<MenuCheckedChangeHandler>,
    index: Option<usize>,
    focus_handle: Option<FocusHandle>,
    style_with_state: Option<MenuCheckboxItemStyle>,
}

impl<P: Clone + 'static> Default for MenuCheckboxItem<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("menu-checkbox-item"),
            base: div(),
            children: Vec::new(),
            context: None,
            label: None,
            disabled: false,
            close_on_click: false,
            checked: None,
            default_checked: false,
            on_checked_change: None,
            index: None,
            focus_handle: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for MenuCheckboxItem<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for MenuCheckboxItem<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context.clone() else {
            return div().into_any_element();
        };
        let focus_handle = self
            .focus_handle
            .clone()
            .unwrap_or_else(|| part_focus_handle(&self.id, window, cx));
        let checked = self.checked.unwrap_or_else(|| {
            self.index
                .map(|index| context.read(cx, |runtime, _| runtime.checkbox_checked(index)))
                .unwrap_or(self.default_checked)
        });
        let (state, tab_stop) = context.read(cx, |runtime, props| {
            let mut state = runtime.checkbox_item_state(self.index, self.disabled, checked);
            state.disabled = state.disabled || props.disabled();
            (state, runtime.item_is_tab_stop(self.index))
        });
        let disabled = state.disabled;
        let index = self.index;
        let hover_context = context.clone();
        let click_context = context.clone();
        let measure_context = context.clone();

        let children = self
            .children
            .into_iter()
            .map(|child| match child {
                MenuCheckboxItemChild::Indicator(indicator) => (*indicator)
                    .with_item_facts(checked, state.highlighted, state.disabled)
                    .into_any_element(),
                MenuCheckboxItemChild::Any(any) => any,
            })
            .collect::<Vec<AnyElement>>();

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
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
                base.id(self.id)
                    .role(Role::MenuItemCheckBox)
                    .aria_toggled(match checked {
                        true => Toggled::True,
                        false => Toggled::False,
                    })
                    .when_some(self.label.clone(), |this, label| this.aria_label(label))
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

impl<P: Clone + 'static> MenuChildNode<P> for MenuCheckboxItem<P> {
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
        wiring.register_checkbox_default(index, self.default_checked);

        let toggle_context = context.clone();
        let controlled = self.checked;
        let handler = self.on_checked_change.clone();
        let activation: MenuItemActivation = Rc::new(move |window, cx| {
            let current = controlled.unwrap_or_else(|| {
                toggle_context.read(cx, |runtime, _| runtime.checkbox_checked(index))
            });
            let next = !current;
            let mut details = MenuItemChangeDetails::new();
            if let Some(handler) = handler.as_ref() {
                handler(next, &mut details, window, cx);
            }
            if details.is_canceled() {
                return;
            }
            if controlled.is_none() {
                toggle_context.update(cx, |runtime| runtime.commit_checkbox(index, next));
            }
        });

        wiring.register_item(
            MenuItemMetadata::new(
                index,
                MenuItemKind::CheckboxItem,
                self.label.clone(),
                self.disabled,
                self.close_on_click,
            )
            .with_focus_handle(focus_handle.clone())
            .with_activation(activation),
        );
        self.id = scoped_id;
        self.index = Some(index);
        self.focus_handle = Some(focus_handle);
        self.context = Some(context.clone());
        self
    }
}

impl<P: Clone + 'static> MenuCheckboxItem<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn child(mut self, child: impl Into<MenuCheckboxItemChild<P>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(MenuCheckboxItemChild::Any(child.into_any_element()));
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

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    pub fn default_checked(mut self, default_checked: bool) -> Self {
        self.default_checked = default_checked;
        self
    }

    pub fn on_checked_change(
        mut self,
        on_checked_change: impl Fn(bool, &mut MenuItemChangeDetails, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_checked_change = Some(Rc::new(on_checked_change));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(MenuCheckboxItemStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
