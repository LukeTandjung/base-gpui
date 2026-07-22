use crate::fieldset::{FieldsetLegendStyleState, FieldsetProps, FieldsetRootStyleState};

#[derive(Clone, Debug, Default)]
pub struct FieldsetRuntime;

impl FieldsetRuntime {
    pub fn new() -> Self {
        Self
    }

    /// Returns the style state for the fieldset group root.
    pub fn root_state(&self, props: &FieldsetProps) -> FieldsetRootStyleState {
        FieldsetRootStyleState::new(props.disabled())
    }

    /// Returns the style state for the fieldset legend.
    pub fn legend_state(&self, props: &FieldsetProps) -> FieldsetLegendStyleState {
        FieldsetLegendStyleState::new(self.root_state(props))
    }
}
