use std::{rc::Rc, sync::Arc};

use gpui::{
    div, AccessibleAction, App, Div, ElementId, Entity, FocusHandle, InteractiveElement as _,
    IntoElement, ParentElement, RenderOnce, Role, SharedString, StatefulInteractiveElement as _,
    StyleRefinement, Styled, Window,
};

use crate::{
    combobox::{
        child_wiring::{ComboboxChildNode, ComboboxChildWiring},
        ComboboxChangeReason, ComboboxChangeSource, ComboboxChipMoveOutcome, ComboboxContext,
        ComboboxEscape, ComboboxInputStyleState, ComboboxMove, ComboboxMoveNext,
        ComboboxMovePrevious, ComboboxSide, COMBOBOX_KEY_CONTEXT,
    },
    primitives::input::{Input, InputStyleState},
};

type ComboboxInputStyle<T> = Rc<dyn Fn(ComboboxInputStyleState<T>, Div) -> Div + 'static>;
type InnerInputStyle = Rc<dyn Fn(InputStyleState, Div) -> Div + 'static>;

/// The text field: composes the `primitives/input` `Input` (no second
/// text-editing implementation) and only adds Combobox wiring — input value
/// sync, open-on-type, chip navigation hand-off, and key dispatch for list
/// navigation.
///
/// Key composition: text-editing keys stay with `INPUT_KEY_CONTEXT`
/// (printable characters always type; Home/End move the caret and never jump
/// the highlight). ArrowUp/ArrowDown/Escape bubble to the Combobox context;
/// Enter and Backspace-with-chips are handled through key-down observation
/// because the input primitive claims those bindings first.
#[derive(IntoElement)]
pub struct ComboboxInput<T: Clone + Eq + 'static> {
    id: ElementId,
    base: Div,
    context: Option<ComboboxContext<T>>,
    placeholder: SharedString,
    aria_label: Option<SharedString>,
    disabled: bool,
    focus_handle: Option<FocusHandle>,
    style_with_state: Option<ComboboxInputStyle<T>>,
    input_style_with_state: Option<InnerInputStyle>,
}

impl<T: Clone + Eq + 'static> Default for ComboboxInput<T> {
    fn default() -> Self {
        Self {
            id: ElementId::from("combobox-input"),
            base: div(),
            context: None,
            placeholder: SharedString::default(),
            aria_label: None,
            disabled: false,
            focus_handle: None,
            style_with_state: None,
            input_style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for ComboboxInput<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for ComboboxInput<T> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context.clone() else {
            return div();
        };
        let focus_handle = self
            .focus_handle
            .unwrap_or_else(|| input_focus_handle(&self.id, window, cx));
        let (state, display_value, suppress_placeholder) = context.read(cx, |runtime, props| {
            (
                runtime.input_state(props, ComboboxSide::Bottom),
                runtime.display_value(),
                // With chips present, a multi-select field is not empty even
                // though the text input is, so the placeholder must not show.
                props.selection_mode == crate::combobox::ComboboxSelectionMode::Multiple
                    && !runtime.selected_values().is_empty(),
            )
        });
        let disabled = self.disabled || state.root.disabled;
        let read_only = state.root.read_only;
        let open = state.root.open;
        let open_on_input_click = context.props().open_on_input_click;

        let typed_context = context.clone();
        let click_context = context.clone();
        let enter_context = context.clone();
        let backspace_context = context.clone();
        let delete_context = context.clone();
        let next_context = context.clone();
        let previous_context = context.clone();
        let escape_context = context.clone();
        let edge_left_context = context.clone();
        let edge_right_context = context.clone();
        let expand_context = context.clone();
        let collapse_context = context.clone();
        let measure_context = context.clone();

        let mut input = Input::new()
            .id(self.id.clone())
            .value(display_value)
            .placeholder(if suppress_placeholder {
                SharedString::default()
            } else {
                self.placeholder.clone()
            })
            .disabled(disabled)
            .read_only(read_only)
            .focus_handle(focus_handle)
            .on_value_change_with_context(move |value, window, cx| {
                typed_context.input_typed(value, window, cx);
            })
            .on_edge_left(move |_value, window, cx| {
                // Caret at start: hand ArrowLeft off to chip navigation.
                let has_chips =
                    edge_left_context.read(cx, |runtime, _| !runtime.selected_values().is_empty());
                if !has_chips {
                    return false;
                }
                let outcome = edge_left_context.move_chip_highlight(ComboboxMove::Previous, cx);
                let _ = window;
                !matches!(outcome, ComboboxChipMoveOutcome::NoChips)
            })
            .on_edge_right(move |_value, window, cx| {
                let highlighted = edge_right_context
                    .read(cx, |runtime, _| runtime.highlighted_chip_index().is_some());
                if !highlighted {
                    return false;
                }
                let outcome = edge_right_context.move_chip_highlight(ComboboxMove::Next, cx);
                let _ = window;
                !matches!(outcome, ComboboxChipMoveOutcome::NoChips)
            })
            // Enter/Backspace/Delete are claimed by the input primitive's key
            // bindings before any ancestor `on_key_down` runs, so Combobox
            // behavior hooks into the primitive's handlers directly.
            .on_enter_with_context(move |_value, window, cx| {
                enter_context.activate_highlighted(ComboboxChangeSource::Keyboard, window, cx);
            })
            .on_backspace(move |value, window, cx| {
                if backspace_context.remove_highlighted_chip(window, cx) {
                    return true;
                }
                if value.is_empty() {
                    backspace_context.remove_last_value(window, cx);
                    return true;
                }
                false
            })
            .on_delete(move |_value, window, cx| {
                delete_context.remove_highlighted_chip(window, cx)
            });
        if let Some(input_style_with_state) = self.input_style_with_state.clone() {
            input = input.style_with_state(move |state, base| input_style_with_state(state, base));
        }

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        // User sizing styles (e.g. `flex_1`) live on `base`, so `base` must be
        // the element that participates in the surrounding flex layout; an
        // unstyled outer div would auto-size a flex-basis-0 child to zero
        // width and collapse the input.
        // AccessKit: the wrapper is the `Role::ComboBox` reference node.
        // Base UI's `aria-controls`/`aria-haspopup`/`aria-autocomplete` and
        // `aria-activedescendant` have no gpui builders and are documented
        // gaps in `combobox/mod.rs`. Focus is tracked by the wrapped input
        // primitive, so `Action::Focus` is auto-registered there.
        let mut wrapper = div()
            .w_full()
            .id((self.id, "combobox-input-wrapper"))
            .role(Role::ComboBox)
            .aria_expanded(open);
        if let Some(aria_label) = self.aria_label.clone() {
            wrapper = wrapper.aria_label(aria_label);
        }
        if !disabled && !read_only {
            // Same change-reason plumbing as the trigger press path so
            // open-change callbacks stay consistent for AT-driven changes.
            wrapper = wrapper
                .on_a11y_action(AccessibleAction::Expand, move |_data, window, cx| {
                    expand_context.set_open(
                        true,
                        ComboboxChangeReason::TriggerPress,
                        ComboboxChangeSource::Keyboard,
                        window,
                        cx,
                    );
                })
                .on_a11y_action(AccessibleAction::Collapse, move |_data, window, cx| {
                    collapse_context.set_open(
                        false,
                        ComboboxChangeReason::TriggerPress,
                        ComboboxChangeSource::Keyboard,
                        window,
                        cx,
                    );
                });
        }
        let wrapper = wrapper
            .key_context(COMBOBOX_KEY_CONTEXT)
            .on_action(move |_: &ComboboxMoveNext, window, cx| {
                next_context.navigate_list(ComboboxMove::Next, window, cx);
            })
            .on_action(move |_: &ComboboxMovePrevious, window, cx| {
                previous_context.navigate_list(ComboboxMove::Previous, window, cx);
            })
            .on_action(move |_: &ComboboxEscape, window, cx| {
                escape_context.escape_pressed(window, cx);
            })
            .on_mouse_down(gpui::MouseButton::Left, move |_event, window, cx| {
                // Pressing the input opens the popup without toggling it
                // closed when `open_on_input_click`.
                if open_on_input_click && !disabled && !read_only {
                    click_context.set_open(
                        true,
                        crate::combobox::ComboboxChangeReason::None,
                        ComboboxChangeSource::Pointer,
                        window,
                        cx,
                    );
                }
            })
            .child(input);

        base.on_children_prepainted(move |bounds, window, cx| {
            let Some(bounds) = bounds.first().copied() else {
                return;
            };
            if measure_context.record_input_bounds(bounds, cx) {
                window.request_animation_frame();
            }
        })
        .child(wrapper)
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxInput<T> {
    fn with_combobox_context(mut self, context: ComboboxContext<T>) -> Self {
        self.context = Some(context);
        self
    }

    fn wire_combobox_child(
        mut self,
        wiring: &mut ComboboxChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let scoped_id = wiring.scope_child_id(&self.id);
        let focus_handle = input_focus_handle(&scoped_id, window, cx);
        wiring.register_input(focus_handle.clone(), focus_handle.is_focused(window));
        self.id = scoped_id;
        self.focus_handle = Some(focus_handle);
        self
    }
}

impl<T: Clone + Eq + 'static> ComboboxInput<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Accessible label for the combobox reference node; substitutes for Base
    /// UI's `aria-labelledby` wiring, which has no gpui builder.
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ComboboxInputStyleState<T>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }

    /// Styling hook for the inner input primitive; composes with, not
    /// replaces, the Combobox-level `style_with_state`.
    pub fn input_style_with_state(
        mut self,
        style: impl Fn(InputStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.input_style_with_state = Some(Rc::new(style));
        self
    }
}

fn input_focus_handle(id: &ElementId, window: &mut Window, cx: &mut App) -> FocusHandle {
    let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
        ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("focus")),
        cx,
        |_, cx| cx.focus_handle(),
    );

    focus_handle_entity.read(cx).clone()
}
