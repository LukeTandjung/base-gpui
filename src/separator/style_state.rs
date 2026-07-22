#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SeparatorOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SeparatorStyleState {
    pub orientation: SeparatorOrientation,
}

impl SeparatorStyleState {
    pub fn new(orientation: SeparatorOrientation) -> Self {
        Self { orientation }
    }
}

impl Default for SeparatorStyleState {
    fn default() -> Self {
        Self::new(SeparatorOrientation::Horizontal)
    }
}
