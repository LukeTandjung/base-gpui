use gpui::SharedString;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NumberParseError {
    Invalid,
    NonFinite,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NumberFieldStep {
    Amount(f64),
    Any,
}

impl NumberFieldStep {
    pub fn amount(value: f64) -> Self {
        if value.is_finite() && value > 0.0 {
            Self::Amount(value)
        } else {
            Self::Amount(1.0)
        }
    }

    pub fn any() -> Self {
        Self::Any
    }

    pub fn interactive_amount(self) -> f64 {
        match self {
            Self::Amount(value) => value,
            Self::Any => 1.0,
        }
    }

    pub fn validates_step(self) -> bool {
        matches!(self, Self::Amount(_))
    }
}

impl Default for NumberFieldStep {
    fn default() -> Self {
        Self::Amount(1.0)
    }
}

pub fn parse_number(text: &str) -> Result<Option<f64>, NumberParseError> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    let value = trimmed
        .parse::<f64>()
        .map_err(|_| NumberParseError::Invalid)?;
    if !value.is_finite() {
        return Err(NumberParseError::NonFinite);
    }

    Ok(Some(clean_floating_point_noise(value)))
}

pub fn format_number(value: Option<f64>) -> SharedString {
    value
        .map(clean_floating_point_noise)
        .map(|value| SharedString::from(value.to_string()))
        .unwrap_or_default()
}

pub fn clamp_value(value: f64, min: Option<f64>, max: Option<f64>) -> f64 {
    let mut clamped = value;
    if let Some(min) = min.filter(|value| value.is_finite()) {
        clamped = clamped.max(min);
    }
    if let Some(max) = max.filter(|value| value.is_finite()) {
        clamped = clamped.min(max);
    }

    clean_floating_point_noise(clamped)
}

pub fn step_value(
    current: Option<f64>,
    direction: i8,
    amount: f64,
    min: Option<f64>,
    max: Option<f64>,
    snap_on_step: bool,
) -> f64 {
    let direction = if direction < 0 { -1.0 } else { 1.0 };
    let amount = if amount.is_finite() && amount > 0.0 {
        amount
    } else {
        1.0
    };
    let base = current.unwrap_or(0.0);
    let stepped = base + amount * direction;
    let snapped = if snap_on_step {
        snap_to_step_grid(stepped, amount, min, direction)
    } else {
        stepped
    };

    clamp_value(clean_floating_point_noise(snapped), min, max)
}

pub fn snap_to_step_grid(value: f64, step: f64, min: Option<f64>, direction: f64) -> f64 {
    let step = if step.is_finite() && step > 0.0 {
        step
    } else {
        1.0
    };
    let origin = min.filter(|value| value.is_finite()).unwrap_or(0.0);
    let scaled = (value - origin) / step;
    let nearest = scaled.round();
    let units = if (scaled - nearest).abs() < 1e-10 {
        nearest
    } else if direction < 0.0 {
        scaled.floor()
    } else {
        scaled.ceil()
    };

    clean_floating_point_noise(origin + units * step)
}

pub fn clean_floating_point_noise(value: f64) -> f64 {
    if !value.is_finite() {
        return value;
    }

    let scale = 1_000_000_000_000.0;
    let rounded = (value * scale).round() / scale;
    if (value - rounded).abs() < 1e-12 {
        rounded
    } else {
        value
    }
}

pub fn option_values_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => {
            clean_floating_point_noise(left) == clean_floating_point_noise(right)
        }
        (None, None) => true,
        _ => false,
    }
}

pub fn normalize_optional_value(value: Option<f64>) -> Option<f64> {
    value
        .filter(|value| value.is_finite())
        .map(clean_floating_point_noise)
}

#[cfg(test)]
mod tests {
    use super::{
        clean_floating_point_noise, format_number, parse_number, step_value, NumberParseError,
    };

    #[test]
    fn empty_text_parses_to_none() {
        assert_eq!(parse_number(""), Ok(None));
        assert_eq!(parse_number("  \t"), Ok(None));
    }

    #[test]
    fn ascii_integer_decimal_and_signs_parse() {
        assert_eq!(parse_number("42"), Ok(Some(42.0)));
        assert_eq!(parse_number("-4.25"), Ok(Some(-4.25)));
        assert_eq!(parse_number("+0.5"), Ok(Some(0.5)));
        assert_eq!(parse_number("1e3"), Ok(Some(1000.0)));
    }

    #[test]
    fn surrounding_whitespace_is_ignored() {
        assert_eq!(parse_number("  12.5\n"), Ok(Some(12.5)));
    }

    #[test]
    fn non_finite_values_are_rejected() {
        assert_eq!(parse_number("NaN"), Err(NumberParseError::NonFinite));
        assert_eq!(parse_number("inf"), Err(NumberParseError::NonFinite));
        assert_eq!(parse_number("-inf"), Err(NumberParseError::NonFinite));
    }

    #[test]
    fn invalid_text_is_rejected() {
        assert_eq!(parse_number("abc"), Err(NumberParseError::Invalid));
        assert_eq!(parse_number("1.2.3"), Err(NumberParseError::Invalid));
    }

    #[test]
    fn formats_none_as_empty_text() {
        assert_eq!(format_number(None), SharedStringLike::from(""));
    }

    #[test]
    fn cleans_common_floating_point_noise() {
        assert_eq!(clean_floating_point_noise(0.1 + 0.2), 0.3);
        assert_eq!(step_value(Some(0.1), 1, 0.2, None, None, false), 0.3);
    }

    struct SharedStringLike;

    impl SharedStringLike {
        fn from(value: &str) -> gpui::SharedString {
            gpui::SharedString::from(value)
        }
    }
}
