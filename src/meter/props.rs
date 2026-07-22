use std::rc::Rc;

pub type MeterFormatHandler = Rc<dyn Fn(f64) -> String + 'static>;

/// Public configuration for Meter: the value, range bounds, and an optional
/// formatting callback replacing Base UI's `Intl.NumberFormat`.
pub struct MeterProps {
    value: f64,
    min: f64,
    max: f64,
    format: Option<MeterFormatHandler>,
}

impl MeterProps {
    pub fn new(value: f64, min: f64, max: f64, format: Option<MeterFormatHandler>) -> Self {
        Self {
            value,
            min,
            max,
            format,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn format(&self) -> Option<&MeterFormatHandler> {
        self.format.as_ref()
    }
}
