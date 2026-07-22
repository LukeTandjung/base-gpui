#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FieldsetRootStyleState {
    pub disabled: bool,
}

impl FieldsetRootStyleState {
    pub fn new(disabled: bool) -> Self {
        Self { disabled }
    }
}

impl Default for FieldsetRootStyleState {
    fn default() -> Self {
        Self::new(false)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FieldsetLegendStyleState {
    pub root: FieldsetRootStyleState,
    pub disabled: bool,
}

impl FieldsetLegendStyleState {
    pub fn new(root: FieldsetRootStyleState) -> Self {
        Self {
            disabled: root.disabled,
            root,
        }
    }
}

impl Default for FieldsetLegendStyleState {
    fn default() -> Self {
        Self::new(Default::default())
    }
}
