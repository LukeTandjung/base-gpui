#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FormSubmitReason {
    #[default]
    Programmatic,
    Action,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FormSubmitDetails {
    pub reason: FormSubmitReason,
}

impl FormSubmitDetails {
    pub fn new(reason: FormSubmitReason) -> Self {
        Self { reason }
    }
}
