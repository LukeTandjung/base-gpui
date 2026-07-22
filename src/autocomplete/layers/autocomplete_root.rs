use std::rc::Rc;

use gpui::{
    div, App, Div, ElementId, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Styled, Window,
};

use crate::{
    autocomplete::AutocompleteChild,
    combobox::{
        child_wiring::wire_children, ComboboxAutoHighlight, ComboboxChangeDetails,
        ComboboxChangeReason, ComboboxChangeSource, ComboboxChild, ComboboxContext,
        ComboboxItemHighlightDetails, ComboboxProps, ComboboxRootStyleState, ComboboxSelectionMode,
    },
    field::{
        current_field_context, current_field_item_disabled, FieldControlRegistration, FieldValue,
    },
    fieldset::current_fieldset_disabled,
};

type AutocompleteRootStyle<T> = Rc<dyn Fn(ComboboxRootStyleState<T>, Div) -> Div + 'static>;

/// Base UI `Autocomplete.Root` `mode` prop: whether the list filters against
/// the typed query (`List`/`Both`) and whether keyboard-highlighting an item
/// inline-autocompletes the input (`Both`/`Inline`).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AutocompleteMode {
    #[default]
    List,
    Both,
    Inline,
    None,
}

/// Resolves a mode to the Combobox runtime knobs:
/// `(filter_disabled, inline_autocomplete)`.
pub fn resolve_mode(mode: AutocompleteMode) -> (bool, bool) {
    match mode {
        AutocompleteMode::List => (false, false),
        AutocompleteMode::Both => (false, true),
        AutocompleteMode::Inline => (true, true),
        AutocompleteMode::None => (true, false),
    }
}

/// The Autocomplete root: the Combobox core configured with
/// `selection_mode = None` and `fill_input_on_item_press = true` — the
/// component's value axis is the input text. Mirrors Base UI, where
/// `AutocompleteRoot` renders `AriaCombobox` directly rather than
/// `ComboboxRoot`. No selection props, no `.multiple`, no fill-on-press knob.
///
/// Accessibility gaps (no gpui builders in this revision, blocked upstream):
/// Base UI's `aria-autocomplete` — literally the [`AutocompleteMode`] — has no
/// accesskit exposure, so the `mode` is not conveyed to assistive technology.
/// Likewise the disabled/read-only state is not announced (the runtime merely
/// ignores interactions), and the status/empty live regions
/// (`aria-live="polite"`) plus the `Both`/`Inline` inline overlay text swap
/// are silent to AT for now.
#[derive(IntoElement)]
pub struct AutocompleteRoot<T: Clone + Eq + 'static> {
    id: ElementId,
    base: Div,
    children: Vec<AutocompleteChild<T>>,
    props: ComboboxProps<T>,
    mode: AutocompleteMode,
    default_value: Option<SharedString>,
    value: Option<SharedString>,
    default_open: bool,
    open: Option<bool>,
    style_with_state: Option<AutocompleteRootStyle<T>>,
}

impl<T: Clone + Eq + 'static> Default for AutocompleteRoot<T> {
    fn default() -> Self {
        // Autocomplete defaults `openOnInputClick` to false (Combobox: true).
        let props = ComboboxProps {
            selection_mode: ComboboxSelectionMode::None,
            fill_input_on_item_press: true,
            open_on_input_click: false,
            ..ComboboxProps::default()
        };

        Self {
            id: ElementId::from("autocomplete"),
            base: div(),
            children: Vec::new(),
            props,
            mode: AutocompleteMode::default(),
            default_value: None,
            value: None,
            default_open: false,
            open: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for AutocompleteRoot<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for AutocompleteRoot<T> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let field_context = current_field_context();
        let field_state = field_context
            .as_ref()
            .map(|context| context.read(cx, |runtime, props| runtime.root_state(props)));
        let field_disabled = field_state.map(|state| state.disabled).unwrap_or(false);
        let field_valid = field_state.and_then(|state| state.valid);

        let mut props = self.props;
        props.disabled = props.disabled
            || field_disabled
            || current_field_item_disabled()
            || current_fieldset_disabled();
        // Thin-variant invariants: regardless of caller input.
        props.selection_mode = ComboboxSelectionMode::None;
        props.fill_input_on_item_press = true;
        let (filter_disabled, inline_autocomplete) = resolve_mode(self.mode);
        props.filter_disabled = filter_disabled;
        props.inline_autocomplete = inline_autocomplete;

        let id = self.id.clone();
        let name = props.name.clone();
        let required = props.required;

        let controlled_input = self.value.clone();
        let controlled_open = self.open;

        let context = ComboboxContext::new(
            self.id.clone(),
            cx,
            window,
            ComboboxSelectionMode::None,
            None,
            None,
            None,
            Vec::new(),
            controlled_input.clone(),
            self.default_value,
            controlled_open,
            self.default_open,
            props,
        );

        // Route Value parts through the context, everything else through the
        // existing Combobox child wiring, preserving child order.
        let children: Vec<ComboboxChild<T>> = self
            .children
            .into_iter()
            .map(|child| match child {
                AutocompleteChild::Combobox(child) => child,
                AutocompleteChild::Value(value) => {
                    ComboboxChild::Any(value.with_context(context.clone()).into_any_element())
                }
            })
            .collect();

        let wired_children = wire_children(children, context.clone(), window, cx);
        let items = wired_children.items;
        let groups = wired_children.groups;
        let input_focus_handle = wired_children.input_focus_handle;
        let input_focused = wired_children.input_focused;
        let children = wired_children.children;

        let close_for_focus_out = context.update(cx, |runtime| {
            runtime.sync_groups(groups);
            runtime.sync_children(items, input_focus_handle.clone(), input_focused);
            runtime.reconcile_single(None, false);
            runtime.reconcile_input_value(controlled_input.clone());
            if let Some(open) = controlled_open {
                runtime.sync_open_from_context(open);
            }
            runtime.refilter();
            runtime.take_focus_out_close_request()
        });
        if close_for_focus_out {
            context.set_open(
                false,
                ComboboxChangeReason::FocusOut,
                ComboboxChangeSource::None,
                window,
                cx,
            );
        }

        let mut style_state = context.read(cx, |runtime, props| runtime.root_state(props));
        if let Some(field_valid) = field_valid {
            style_state.valid = Some(field_valid);
            style_state.invalid = !field_valid;
        }

        if let Some(field_context) = field_context.as_ref() {
            // None-mode Field serialization: the input text is the value.
            let field_value = match style_state.input_value.is_empty() {
                true => FieldValue::Empty,
                false => FieldValue::Text(style_state.input_value.clone()),
            };
            let value_missing = required && !field_value.filled();
            let mut registration = FieldControlRegistration::new(id.to_string())
                .value(field_value)
                .disabled(style_state.disabled)
                .focused(style_state.focused)
                .required(required)
                .value_missing(value_missing);
            if let Some(focus_handle) = context.read(cx, |runtime, _| runtime.input_focus_handle())
            {
                registration = registration.focus_handle(focus_handle);
            }
            if let Some(name) = name {
                registration = registration.name(name);
            }
            field_context.register_control(registration, cx);
        }

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        base.children(children)
    }
}

impl<T: Clone + Eq + 'static> AutocompleteRoot<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<AutocompleteChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<AutocompleteChild<T>>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(AutocompleteChild::Combobox(ComboboxChild::Any(
                child.into_any_element(),
            )));
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn name(mut self, name: impl Into<SharedString>) -> Self {
        self.props.name = Some(name.into());
        self
    }

    pub fn mode(mut self, mode: AutocompleteMode) -> Self {
        self.mode = mode;
        self
    }

    /// The Autocomplete value axis is the input text (Base UI maps
    /// `defaultValue` to Combobox `defaultInputValue`).
    pub fn default_value(mut self, default_value: impl Into<SharedString>) -> Self {
        self.default_value = Some(default_value.into());
        self
    }

    /// Controlled input text (Base UI `value` → Combobox `inputValue`).
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn on_value_change(
        mut self,
        on_value_change: impl Fn(&SharedString, &mut ComboboxChangeDetails, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.props.on_input_value_change = Some(Rc::new(on_value_change));
        self
    }

    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn on_open_change(
        mut self,
        on_open_change: impl Fn(bool, &mut ComboboxChangeDetails, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.props.on_open_change = Some(Rc::new(on_open_change));
        self
    }

    pub fn on_item_highlighted(
        mut self,
        on_item_highlighted: impl Fn(Option<&T>, &ComboboxItemHighlightDetails, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.props.on_item_highlighted = Some(Rc::new(on_item_highlighted));
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.props.disabled = disabled;
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.props.read_only = read_only;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.props.required = required;
        self
    }

    pub fn open_on_input_click(mut self, open_on_input_click: bool) -> Self {
        self.props.open_on_input_click = open_on_input_click;
        self
    }

    pub fn auto_highlight(mut self, auto_highlight: ComboboxAutoHighlight) -> Self {
        self.props.auto_highlight = auto_highlight;
        self
    }

    pub fn keep_highlight(mut self, keep_highlight: bool) -> Self {
        self.props.keep_highlight = keep_highlight;
        self
    }

    pub fn highlight_item_on_hover(mut self, highlight_item_on_hover: bool) -> Self {
        self.props.highlight_item_on_hover = highlight_item_on_hover;
        self
    }

    /// Documented no-op hook until Form exposes programmatic submit.
    pub fn submit_on_item_click(mut self, submit_on_item_click: bool) -> Self {
        self.props.submit_on_item_click = submit_on_item_click;
        self
    }

    pub fn loop_focus(mut self, loop_focus: bool) -> Self {
        self.props.loop_focus = loop_focus;
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.props.limit = Some(limit);
        self
    }

    /// Custom filter replacing the default case-insensitive contains match
    /// (only applies in `List`/`Both` modes).
    pub fn filter(
        mut self,
        filter: impl Fn(&T, Option<&SharedString>, &str) -> bool + 'static,
    ) -> Self {
        self.props.filter = Some(Rc::new(filter));
        self
    }

    /// Feeds display, filtering, fill-on-press, and Field serialization
    /// (Base UI `itemToStringValue` → Combobox `itemToStringLabel`).
    pub fn item_to_string_value(mut self, resolver: impl Fn(&T) -> SharedString + 'static) -> Self {
        let resolver: Rc<dyn Fn(&T) -> SharedString + 'static> = Rc::new(resolver);
        self.props.label_resolver = Some(Rc::clone(&resolver));
        self.props.value_serializer = Some(resolver);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ComboboxRootStyleState<T>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
