use std::rc::Rc;

pub type ProgressFormatHandler = Rc<dyn Fn(f64) -> String + 'static>;

/// Public configuration for Progress: nullable value, range bounds, and an
/// optional formatting callback replacing Base UI's `Intl.NumberFormat`.
pub struct ProgressProps {
    value: Option<f64>,
    min: f64,
    max: f64,
    format: Option<ProgressFormatHandler>,
}

impl ProgressProps {
    pub fn new(
        value: Option<f64>,
        min: f64,
        max: f64,
        format: Option<ProgressFormatHandler>,
    ) -> Self {
        Self {
            value,
            min,
            max,
            format,
        }
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn format(&self) -> Option<&ProgressFormatHandler> {
        self.format.as_ref()
    }
}
