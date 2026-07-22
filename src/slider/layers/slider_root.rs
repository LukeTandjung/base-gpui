use std::{rc::Rc, sync::Arc};

use gpui::{
    div, App, Div, ElementId, Entity, FocusHandle, InteractiveElement as _, IntoElement,
    Orientation, ParentElement, RenderOnce, Role, SharedString, StatefulInteractiveElement as _,
    StyleRefinement, Styled, Window,
};

use crate::{
    field::{
        current_field_context, current_field_item_disabled, FieldContext, FieldControlRegistration,
        FieldValue,
    },
    fieldset::current_fieldset_disabled,
    slider::{
        child_wiring::wire_children, SliderChild, SliderContext, SliderFormatHandler,
        SliderOrientation, SliderProps, SliderRootStyleState, SliderThumbAlignment,
        SliderThumbCollisionBehavior, SliderThumbMeta, SliderValueChangeDetails,
        SliderValueChangeHandler, SliderValueCommitDetails, SliderValueCommitHandler, SliderValues,
    },
};

#[derive(IntoElement)]
pub struct SliderRoot {
    id: ElementId,
    base: Div,
    children: Vec<SliderChild>,
    field_context: Option<FieldContext>,
    name: Option<SharedString>,
    default_value: Option<SliderValues>,
    value: Option<SliderValues>,
    min: f64,
    max: f64,
    step: f64,
    large_step: f64,
    min_steps_between_values: f64,
    orientation: SliderOrientation,
    thumb_collision_behavior: SliderThumbCollisionBehavior,
    thumb_alignment: SliderThumbAlignment,
    disabled: bool,
    aria_label: Option<SharedString>,
    format: Option<SliderFormatHandler>,
    on_value_change: Option<SliderValueChangeHandler>,
    on_value_committed: Option<SliderValueCommitHandler>,
    style_with_state: Option<Rc<dyn Fn(SliderRootStyleState, Div) -> Div + 'static>>,
}

impl Default for SliderRoot {
    fn default() -> Self {
        Self {
            id: ElementId::from("slider"),
            base: div(),
            children: Vec::from([]),
            field_context: None,
            name: None,
            default_value: None,
            value: None,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            large_step: 10.0,
            min_steps_between_values: 0.0,
            orientation: SliderOrientation::default(),
            thumb_collision_behavior: SliderThumbCollisionBehavior::default(),
            thumb_alignment: SliderThumbAlignment::default(),
            disabled: false,
            aria_label: None,
            format: None,
            on_value_change: None,
            on_value_committed: None,
            style_with_state: None,
        }
    }
}

impl Styled for SliderRoot {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for SliderRoot {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let field_context = self.field_context.or_else(current_field_context);
        let field_state = field_context
            .as_ref()
            .map(|context| context.read(cx, |runtime, props| runtime.root_state(props)));
        let field_disabled = field_state.map(|state| state.disabled).unwrap_or(false);
        let item_disabled = current_field_item_disabled();
        let fieldset_disabled = current_fieldset_disabled();
        let disabled = self.disabled || field_disabled || item_disabled || fieldset_disabled;
        let id = self.id.clone();
        let name = self.name.clone();

        let context = SliderContext::new(
            self.id.clone(),
            cx,
            window,
            self.value,
            self.default_value,
            SliderProps::new(
                self.name,
                self.min,
                self.max,
                self.step,
                self.large_step,
                self.min_steps_between_values,
                self.orientation,
                self.thumb_collision_behavior,
                self.thumb_alignment,
                disabled,
                self.format,
                self.on_value_change,
                self.on_value_committed,
            ),
        );

        let wired = wire_children(self.children, context.clone());

        let mut thumbs = Vec::from([]);
        for (index, thumb_disabled) in wired.thumb_disabled.iter().copied().enumerate() {
            let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
                ElementId::NamedChild(
                    Arc::new(id.clone()),
                    SharedString::from(format!("thumb-focus-{index}")),
                ),
                cx,
                |_, cx| cx.focus_handle(),
            );
            thumbs.push(SliderThumbMeta {
                disabled: thumb_disabled,
                focus_handle: focus_handle_entity.read(cx).clone(),
            });
        }

        context.update(cx, |runtime| {
            runtime.sync_thumbs(thumbs);
            runtime.sync_disabled(disabled);
        });

        let style_state = context.read(cx, |runtime, props| runtime.root_state(props));

        if let Some(field_context) = field_context.as_ref() {
            let (formatted, focused, focus_handle) = context.read(cx, |runtime, props| {
                let value_state = runtime.value_state(props);
                let joined = value_state
                    .formatted_values
                    .iter()
                    .map(SharedString::as_ref)
                    .collect::<Vec<_>>()
                    .join(" \u{2013} ");
                (
                    SharedString::from(joined),
                    runtime.any_thumb_focused(),
                    runtime.single_thumb_focus_handle(),
                )
            });
            let mut registration = FieldControlRegistration::new(id.to_string())
                .value(FieldValue::Text(formatted))
                .disabled(style_state.disabled)
                .focused(focused);
            if let Some(focus_handle) = focus_handle {
                registration = registration.focus_handle(focus_handle);
            }
            if let Some(name) = name {
                registration = registration.name(name);
            }
            field_context.register_control(registration, cx);
        }

        let orientation = style_state.orientation;
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        // Applied after `style_with_state` so the styling closure cannot drop
        // the accessibility wiring. Base UI renders the root as
        // `role="group"` with `aria-labelledby` pointing at `SliderLabel`;
        // gpui has no id-reference builder, so the label is a literal
        // `.aria_label(...)` string instead.
        let mut base = base
            .id(id)
            .role(Role::Group)
            .aria_orientation(match orientation {
                SliderOrientation::Horizontal => Orientation::Horizontal,
                SliderOrientation::Vertical => Orientation::Vertical,
            });
        if let Some(aria_label) = self.aria_label {
            base = base.aria_label(aria_label);
        }

        base.children(wired.children)
    }
}

impl SliderRoot {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_field_context(mut self, context: FieldContext) -> Self {
        self.field_context = Some(context);
        self
    }

    pub fn child(mut self, child: impl Into<SliderChild>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<SliderChild>>) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(SliderChild::Any(child.into_any_element()));
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn name(mut self, name: impl Into<SharedString>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn default_value(mut self, default_value: SliderValues) -> Self {
        self.default_value = Some(default_value);
        self
    }

    pub fn value(mut self, value: SliderValues) -> Self {
        self.value = Some(value);
        self
    }

    pub fn min(mut self, min: f64) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }

    pub fn step(mut self, step: f64) -> Self {
        self.step = step;
        self
    }

    pub fn large_step(mut self, large_step: f64) -> Self {
        self.large_step = large_step;
        self
    }

    pub fn min_steps_between_values(mut self, min_steps_between_values: f64) -> Self {
        self.min_steps_between_values = min_steps_between_values;
        self
    }

    pub fn orientation(mut self, orientation: SliderOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn thumb_collision_behavior(mut self, behavior: SliderThumbCollisionBehavior) -> Self {
        self.thumb_collision_behavior = behavior;
        self
    }

    pub fn thumb_alignment(mut self, thumb_alignment: SliderThumbAlignment) -> Self {
        self.thumb_alignment = thumb_alignment;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Accessible label for the slider group, mirroring what `SliderLabel`
    /// displays. Base UI links the label by id via `aria-labelledby`; gpui
    /// has no id-reference builder, so the text is supplied literally.
    /// Callers who set this should render the visible `SliderLabel` text
    /// with `Text::new_inaccessible(...)` to avoid double-announcing.
    pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn format(mut self, format: impl Fn(f64) -> SharedString + 'static) -> Self {
        self.format = Some(Rc::new(format));
        self
    }

    pub fn on_value_change(
        mut self,
        on_value_change: impl Fn(SliderValues, &mut SliderValueChangeDetails, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(on_value_change));
        self
    }

    pub fn on_value_committed(
        mut self,
        on_value_committed: impl Fn(SliderValues, SliderValueCommitDetails, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.on_value_committed = Some(Rc::new(on_value_committed));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(SliderRootStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
