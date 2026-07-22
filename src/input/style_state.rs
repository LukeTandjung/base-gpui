use gpui::SharedString;

use crate::{
    field::FieldRootStyleState, primitives::input::InputStyleState as PrimitiveInputStyleState,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputStyleState {
    pub value: SharedString,
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub focused: bool,
    pub empty: bool,
    pub filled: bool,
    pub dirty: bool,
    pub touched: bool,
    pub controlled: bool,
    pub valid: Option<bool>,
    pub invalid: bool,
}

impl InputStyleState {
    pub fn new(primitive: PrimitiveInputStyleState, field: Option<FieldRootStyleState>) -> Self {
        let valid = field.map_or(primitive.valid, |field| field.valid);
        let filled = field.map_or(primitive.filled, |field| field.filled);
        let dirty = field.map_or(primitive.dirty, |field| field.dirty);
        let focused = field.map_or(primitive.focused, |field| field.focused);
        let touched = field.map_or(false, |field| field.touched);

        Self {
            value: primitive.value,
            disabled: primitive.disabled,
            read_only: primitive.read_only,
            required: primitive.required,
            focused,
            empty: !filled,
            filled,
            dirty,
            touched,
            controlled: primitive.controlled,
            valid,
            invalid: valid == Some(false),
        }
    }
}

impl Default for InputStyleState {
    fn default() -> Self {
        Self::new(PrimitiveInputStyleState::default(), None)
    }
}
