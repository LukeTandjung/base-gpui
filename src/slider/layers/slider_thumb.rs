use std::rc::Rc;

use gpui::{
    accesskit::ActionData, div, px, relative, AccessibleAction, App, Div, ElementId,
    InteractiveElement as _, IntoElement, MouseButton, Orientation, ParentElement, RenderOnce,
    Role, SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::slider::{
    SliderContext, SliderEnd, SliderHome, SliderKeyboardStep, SliderOrientation, SliderPageDown,
    SliderPageUp, SliderStepDown, SliderStepDownLarge, SliderStepLeft, SliderStepLeftLarge,
    SliderStepRight, SliderStepRightLarge, SliderStepUp, SliderStepUpLarge, SliderThumbStyleState,
    SLIDER_THUMB_KEY_CONTEXT,
};
use crate::utils::{current_direction, HorizontalArrowKey, HorizontalDirection, TextDirection};

#[derive(IntoElement)]
pub struct SliderThumb {
    id: Option<ElementId>,
    base: Div,
    children: Vec<gpui::AnyElement>,
    context: Option<SliderContext>,
    index: usize,
    disabled: bool,
    aria_label: Option<SharedString>,
    style_with_state: Option<Rc<dyn Fn(SliderThumbStyleState, Div) -> Div + 'static>>,
}

impl Default for SliderThumb {
    fn default() -> Self {
        Self {
            id: None,
            base: div(),
            children: Vec::from([]),
            context: None,
            index: 0,
            disabled: false,
            aria_label: None,
            style_with_state: None,
        }
    }
}

impl Styled for SliderThumb {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

fn arrow_step(
    direction: TextDirection,
    key: HorizontalArrowKey,
    large: bool,
) -> SliderKeyboardStep {
    match (direction.horizontal_arrow(key), large) {
        (HorizontalDirection::Next, false) => SliderKeyboardStep::Increment,
        (HorizontalDirection::Next, true) => SliderKeyboardStep::LargeIncrement,
        (HorizontalDirection::Previous, false) => SliderKeyboardStep::Decrement,
        (HorizontalDirection::Previous, true) => SliderKeyboardStep::LargeDecrement,
    }
}

impl RenderOnce for SliderThumb {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context = self
            .context
            .expect("SliderThumb must be rendered inside SliderControl");
        let index = self.index;
        let id = self
            .id
            .unwrap_or_else(|| context.child_id(format!("thumb-{index}")));
        let direction = current_direction();
        let focus_handle = context.read(cx, |runtime, _| runtime.thumb_focus_handle(index));

        if let Some(focus_handle) = focus_handle.as_ref() {
            context.sync_thumb_focused(index, focus_handle.is_focused(window), cx);
        }

        let style_state = context.read(cx, |runtime, props| runtime.thumb_state(index, props));
        let orientation = style_state.root.orientation;
        let disabled = style_state.disabled;
        let value = style_state.value;
        let min = style_state.root.min;
        let max = style_state.root.max;
        let thumb_count = context.read(cx, |runtime, _| runtime.thumb_count());
        let fraction = style_state.fraction as f32;
        let positioned = style_state.positioned;
        let half_thumb = style_state.half_thumb_main_axis as f32;
        let edge_offset = style_state.edge_offset;
        let z_index = style_state.z_index;

        let down_context = context.clone();
        let a11y_increment_context = context.clone();
        let a11y_decrement_context = context.clone();
        let a11y_set_value_context = context.clone();
        let key_contexts = std::iter::repeat_with(|| context.clone())
            .take(12)
            .collect::<Vec<_>>();

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        // Applied after `style_with_state` so the styling closure cannot drop
        // the accessibility wiring. The thumb is the value-bearing node,
        // replacing Base UI's hidden `<input type="range">`. Min/max are the
        // root bounds (matching Base UI's per-input `min`/`max`); neighbor
        // clamping stays a runtime behavior, not an exposed bound.
        // `aria-valuetext`/`aria-describedby`/disabled have no gpui builders;
        // position-in-set/size-of-set partially compensate for the missing
        // range-start/range-end phrasing, and disabled thumbs simply skip
        // registering a11y actions (AT sees an inert, not announced-as-
        // disabled, slider).
        let mut base = base
            .id(id)
            .role(Role::Slider)
            .aria_numeric_value(value)
            .aria_min_numeric_value(min)
            .aria_max_numeric_value(max)
            .aria_orientation(match orientation {
                SliderOrientation::Horizontal => Orientation::Horizontal,
                SliderOrientation::Vertical => Orientation::Vertical,
            })
            .aria_position_in_set(index + 1)
            .aria_size_of_set(thumb_count)
            .absolute()
            .occlude();
        if let Some(aria_label) = self.aria_label {
            base = base.aria_label(aria_label);
        }
        if !positioned {
            base = base.invisible();
        } else {
            base = match (orientation, edge_offset) {
                (SliderOrientation::Horizontal, Some(offset)) => {
                    if direction.is_rtl() {
                        base.right(px(offset as f32))
                    } else {
                        base.left(px(offset as f32))
                    }
                }
                (SliderOrientation::Horizontal, None) => {
                    if direction.is_rtl() {
                        base.right(relative(fraction)).mr(px(-half_thumb))
                    } else {
                        base.left(relative(fraction)).ml(px(-half_thumb))
                    }
                }
                (SliderOrientation::Vertical, Some(offset)) => base.bottom(px(offset as f32)),
                (SliderOrientation::Vertical, None) => {
                    base.bottom(relative(fraction)).mb(px(-half_thumb))
                }
            };
        }
        if z_index > 0 {
            base = base.occlude();
        }

        let mut base = base.key_context(SLIDER_THUMB_KEY_CONTEXT);
        if let Some(focus_handle) = focus_handle.as_ref() {
            base = base.track_focus(&focus_handle.clone().tab_stop(!disabled).tab_index(0));
        }

        let mut key_context_iter = key_contexts.into_iter();
        let mut next_key_context =
            move || key_context_iter.next().expect("twelve key contexts exist");
        let up_context = next_key_context();
        let down_action_context = next_key_context();
        let left_context = next_key_context();
        let right_context = next_key_context();
        let up_large_context = next_key_context();
        let down_large_context = next_key_context();
        let left_large_context = next_key_context();
        let right_large_context = next_key_context();
        let page_up_context = next_key_context();
        let page_down_context = next_key_context();
        let home_context = next_key_context();
        let end_context = next_key_context();

        let base = base
            .on_mouse_down(MouseButton::Left, move |event, window, cx| {
                cx.stop_propagation();
                down_context.press_thumb(index, event.position, direction, window, cx);
            })
            .on_action(move |_: &SliderStepUp, window, cx| {
                if disabled {
                    return;
                }
                up_context.keyboard_step(index, SliderKeyboardStep::Increment, window, cx);
            })
            .on_action(move |_: &SliderStepDown, window, cx| {
                if disabled {
                    return;
                }
                down_action_context.keyboard_step(index, SliderKeyboardStep::Decrement, window, cx);
            })
            .on_action(move |_: &SliderStepLeft, window, cx| {
                if disabled {
                    return;
                }
                left_context.keyboard_step(
                    index,
                    arrow_step(direction, HorizontalArrowKey::Left, false),
                    window,
                    cx,
                );
            })
            .on_action(move |_: &SliderStepRight, window, cx| {
                if disabled {
                    return;
                }
                right_context.keyboard_step(
                    index,
                    arrow_step(direction, HorizontalArrowKey::Right, false),
                    window,
                    cx,
                );
            })
            .on_action(move |_: &SliderStepUpLarge, window, cx| {
                if disabled {
                    return;
                }
                up_large_context.keyboard_step(
                    index,
                    SliderKeyboardStep::LargeIncrement,
                    window,
                    cx,
                );
            })
            .on_action(move |_: &SliderStepDownLarge, window, cx| {
                if disabled {
                    return;
                }
                down_large_context.keyboard_step(
                    index,
                    SliderKeyboardStep::LargeDecrement,
                    window,
                    cx,
                );
            })
            .on_action(move |_: &SliderStepLeftLarge, window, cx| {
                if disabled {
                    return;
                }
                left_large_context.keyboard_step(
                    index,
                    arrow_step(direction, HorizontalArrowKey::Left, true),
                    window,
                    cx,
                );
            })
            .on_action(move |_: &SliderStepRightLarge, window, cx| {
                if disabled {
                    return;
                }
                right_large_context.keyboard_step(
                    index,
                    arrow_step(direction, HorizontalArrowKey::Right, true),
                    window,
                    cx,
                );
            })
            .on_action(move |_: &SliderPageUp, window, cx| {
                if disabled {
                    return;
                }
                page_up_context.keyboard_step(
                    index,
                    SliderKeyboardStep::LargeIncrement,
                    window,
                    cx,
                );
            })
            .on_action(move |_: &SliderPageDown, window, cx| {
                if disabled {
                    return;
                }
                page_down_context.keyboard_step(
                    index,
                    SliderKeyboardStep::LargeDecrement,
                    window,
                    cx,
                );
            })
            .on_action(move |_: &SliderHome, window, cx| {
                if disabled {
                    return;
                }
                home_context.keyboard_step(index, SliderKeyboardStep::Home, window, cx);
            })
            .on_action(move |_: &SliderEnd, window, cx| {
                if disabled {
                    return;
                }
                end_context.keyboard_step(index, SliderKeyboardStep::End, window, cx);
            });

        // `Action::Focus` is auto-registered by `track_focus` above — do not
        // re-add it. Registration is skipped entirely while merged-disabled,
        // the fallback for the missing `aria-disabled` builder.
        let base = if disabled {
            base
        } else {
            base.on_a11y_action(AccessibleAction::Increment, move |_, window, cx| {
                a11y_increment_context.keyboard_step(
                    index,
                    SliderKeyboardStep::Increment,
                    window,
                    cx,
                );
            })
            .on_a11y_action(AccessibleAction::Decrement, move |_, window, cx| {
                a11y_decrement_context.keyboard_step(
                    index,
                    SliderKeyboardStep::Decrement,
                    window,
                    cx,
                );
            })
            .on_a11y_action(AccessibleAction::SetValue, move |data, window, cx| {
                if let Some(ActionData::NumericValue(value)) = data {
                    a11y_set_value_context.set_thumb_value(index, *value, window, cx);
                }
            })
        };

        base.children(self.children)
    }
}

impl SliderThumb {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_slider_context(mut self, context: SliderContext) -> Self {
        self.context = Some(context);
        self
    }

    pub fn with_thumb_index(mut self, index: usize) -> Self {
        self.index = index;
        self
    }

    pub fn thumb_index(&self) -> Option<usize> {
        if self.context.is_some() {
            Some(self.index)
        } else {
            None
        }
    }

    pub fn thumb_disabled(&self) -> bool {
        self.disabled
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Accessible label for this thumb (e.g. "Minimum" / "Maximum" on a
    /// range slider). A plain string per thumb replaces Base UI's optional
    /// `getAriaLabel(index)` closure.
    pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(SliderThumbStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
