use std::rc::Rc;

use gpui::{
    div, prelude::FluentBuilder as _, AnyElement, App, ClickEvent, Div, ElementId, FocusHandle,
    InteractiveElement as _, IntoElement, MouseButton, ParentElement, RenderOnce, Role,
    SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::menu::{
    child_wiring::{part_focus_handle, MenuChildNode, MenuChildWiring},
    MenuActivationHandler, MenuContext, MenuItemKind, MenuItemMetadata, MenuItemStyleState,
    MenuOpenChangeReason, MenuOpenChangeSource, MenuParentKind,
};

type MenuItemStyle = Rc<dyn Fn(MenuItemStyleState, Div) -> Div + 'static>;

#[derive(IntoElement)]
pub struct MenuItem<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<AnyElement>,
    context: Option<MenuContext<P>>,
    label: Option<SharedString>,
    disabled: bool,
    close_on_click: bool,
    on_click: Option<MenuActivationHandler>,
    index: Option<usize>,
    focus_handle: Option<FocusHandle>,
    style_with_state: Option<MenuItemStyle>,
}

impl<P: Clone + 'static> Default for MenuItem<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("menu-item"),
            base: div(),
            children: Vec::new(),
            context: None,
            label: None,
            disabled: false,
            close_on_click: true,
            on_click: None,
            index: None,
            focus_handle: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> ParentElement for MenuItem<P> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<P: Clone + 'static> Styled for MenuItem<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for MenuItem<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context.clone() else {
            return div().children(self.children).into_any_element();
        };
        let focus_handle = self
            .focus_handle
            .clone()
            .unwrap_or_else(|| part_focus_handle(&self.id, window, cx));
        let (state, tab_stop, item_count) = context.read(cx, |runtime, props| {
            let mut state = runtime.item_state(self.index, self.disabled);
            state.disabled = state.disabled || props.disabled();
            (
                state,
                runtime.item_is_tab_stop(self.index),
                runtime.item_count(),
            )
        });
        let disabled = state.disabled;
        let index = self.index;
        let hover_context = context.clone();
        let click_context = context.clone();
        let measure_context = context.clone();
        let tree_root = context.tree_root().clone();
        let context_menu_tree =
            tree_root.read(cx, |runtime, _| runtime.parent_kind()) == MenuParentKind::ContextMenu;
        let release_context = context.clone();

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        div()
            .on_children_prepainted(move |bounds, window, cx| {
                let Some(bounds) = bounds.first().copied() else {
                    return;
                };
                let Some(index) = index else {
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
                    // `aria-disabled` has no gpui builder (documented gap):
                    // disabled items stay inert via withheld tab stops and
                    // activation no-ops but are not announced as disabled.
                    .role(Role::MenuItem)
                    .when_some(self.label.clone(), |this, label| this.aria_label(label))
                    .when_some(index, |this, index| this.aria_position_in_set(index + 1))
                    .aria_size_of_set(item_count)
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
                    .when(context_menu_tree, |base| {
                        // Right-button drag-release activation for Context
                        // Menu trees: release over an enabled item activates
                        // it, unless the release lands within ±1px of the
                        // initial cursor point of the opening gesture.
                        base.on_mouse_up(MouseButton::Right, move |event, window, cx| {
                            let Some(index) = index else {
                                return;
                            };
                            if disabled {
                                return;
                            }
                            let suppressed = tree_root.update(cx, |runtime| {
                                runtime.consume_context_menu_initial_point(event.position)
                            });
                            if suppressed {
                                return;
                            }
                            let activated = release_context.activate_item(
                                index,
                                MenuOpenChangeSource::Pointer,
                                window,
                                cx,
                            );
                            // Right-button drag-release closes the whole tree
                            // even when the item opts out of close-on-click.
                            if activated {
                                release_context.close_tree(
                                    MenuOpenChangeReason::ItemPress,
                                    MenuOpenChangeSource::Pointer,
                                    window,
                                    cx,
                                );
                            }
                        })
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
                    .children(self.children),
            )
            .into_any_element()
    }
}

impl<P: Clone + 'static> MenuChildNode<P> for MenuItem<P> {
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
        let mut metadata = MenuItemMetadata::new(
            index,
            MenuItemKind::Item,
            self.label.clone(),
            self.disabled,
            self.close_on_click,
        )
        .with_focus_handle(focus_handle.clone());
        if let Some(on_click) = self.on_click.clone() {
            metadata = metadata.with_activation(on_click);
        }
        wiring.register_item(metadata);
        self.id = scoped_id;
        self.index = Some(index);
        self.focus_handle = Some(focus_handle);
        self.context = Some(context.clone());
        self
    }
}

impl<P: Clone + 'static> MenuItem<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
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

    pub fn on_click(mut self, on_click: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(on_click));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(MenuItemStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
