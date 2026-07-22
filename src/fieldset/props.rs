#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldsetProps {
    disabled: bool,
}

impl FieldsetProps {
    pub fn new(disabled: bool) -> Self {
        Self { disabled }
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }
}
