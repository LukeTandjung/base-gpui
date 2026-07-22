use std::rc::Rc;

/// Optional idempotent user hook applied during normalization.
pub type OTPFieldNormalizeValueHandler = Rc<dyn Fn(String) -> String + 'static>;

/// Character filter applied to every incoming OTP value.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum OTPFieldValidationType {
    #[default]
    Numeric,
    Alpha,
    Alphanumeric,
    None,
}

impl OTPFieldValidationType {
    fn accepts(self, character: char) -> bool {
        match self {
            Self::Numeric => character.is_ascii_digit(),
            Self::Alpha => character.is_ascii_alphabetic(),
            Self::Alphanumeric => character.is_ascii_alphanumeric(),
            Self::None => true,
        }
    }
}

/// The result of normalizing an OTP value or fragment.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OTPNormalizedValue {
    pub value: String,
    pub rejected: bool,
}

/// The result of splicing a fragment into an OTP value at a slot index.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OTPReplaceOutcome {
    pub value: String,
    pub rejected: bool,
    /// Number of fragment code points that survived filtering and clamping.
    pub accepted: usize,
}

/// Strips all Unicode whitespace from the input.
pub fn strip_whitespace(input: &str) -> String {
    input.chars().filter(|c| !c.is_whitespace()).collect()
}

/// Keeps only the characters accepted by the validation type.
pub fn filter_validation(input: &str, validation_type: OTPFieldValidationType) -> String {
    input
        .chars()
        .filter(|c| validation_type.accepts(*c))
        .collect()
}

/// Clamps the value to `length` Unicode code points.
pub fn clamp_code_points(input: &str, length: usize) -> String {
    input.chars().take(length).collect()
}

/// Counts Unicode code points.
pub fn code_point_count(input: &str) -> usize {
    input.chars().count()
}

/// Normalizes an incoming OTP value: strips whitespace, filters by validation
/// type, applies the optional custom hook (whose output is stripped, re-filtered,
/// and clamped again), and clamps to `length` code points. Reports whether any
/// characters were rejected at any stage.
pub fn normalize_otp_value(
    input: &str,
    validation_type: OTPFieldValidationType,
    normalize_value: Option<&OTPFieldNormalizeValueHandler>,
    length: usize,
) -> OTPNormalizedValue {
    let stripped = strip_whitespace(input);
    let filtered = filter_validation(&stripped, validation_type);
    let mut rejected = code_point_count(&filtered) < code_point_count(&stripped);

    let hooked = match normalize_value {
        Some(hook) => {
            let output = hook(filtered.clone());
            let refiltered = filter_validation(&strip_whitespace(&output), validation_type);
            rejected |= code_point_count(&refiltered) < code_point_count(&filtered);
            rejected |= code_point_count(&refiltered) < code_point_count(&output);
            refiltered
        }
        None => filtered,
    };

    let clamped = clamp_code_points(&hooked, length);
    rejected |= code_point_count(&clamped) < code_point_count(&hooked);

    OTPNormalizedValue {
        value: clamped,
        rejected,
    }
}

/// Splices a typed or pasted fragment into `value` starting at slot `index`,
/// overwriting forward, then re-normalizes the final value. The fragment is
/// normalized first so paste and multi-character edits stay contiguous.
pub fn replace_at_index(
    value: &str,
    index: usize,
    fragment: &str,
    validation_type: OTPFieldValidationType,
    normalize_value: Option<&OTPFieldNormalizeValueHandler>,
    length: usize,
) -> OTPReplaceOutcome {
    let normalized_fragment =
        normalize_otp_value(fragment, validation_type, normalize_value, length);
    let fragment_count = code_point_count(&normalized_fragment.value);

    let prefix: String = value.chars().take(index).collect();
    let suffix: String = value.chars().skip(index + fragment_count).collect();
    let spliced = format!("{prefix}{}{suffix}", normalized_fragment.value);

    let normalized = normalize_otp_value(&spliced, validation_type, normalize_value, length);
    let accepted = fragment_count.min(length.saturating_sub(index));

    OTPReplaceOutcome {
        rejected: normalized_fragment.rejected || normalized.rejected,
        value: normalized.value,
        accepted,
    }
}

/// Removes exactly one code point at `index`; out-of-bounds indices leave the
/// value unchanged.
pub fn remove_at_index(value: &str, index: usize) -> String {
    value
        .chars()
        .enumerate()
        .filter(|(char_index, _)| *char_index != index)
        .map(|(_, character)| character)
        .collect()
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::{
        clamp_code_points, filter_validation, normalize_otp_value, remove_at_index,
        replace_at_index, strip_whitespace, OTPFieldNormalizeValueHandler, OTPFieldValidationType,
    };

    fn hook(f: impl Fn(String) -> String + 'static) -> OTPFieldNormalizeValueHandler {
        Rc::new(f)
    }

    #[test]
    fn strips_all_unicode_whitespace_including_empty() {
        assert_eq!(strip_whitespace(""), "");
        assert_eq!(strip_whitespace(" 1 2\t3\n4\u{00a0}5 "), "12345");
    }

    #[test]
    fn numeric_filters_to_ascii_digits() {
        assert_eq!(
            filter_validation("a1b2é3", OTPFieldValidationType::Numeric),
            "123"
        );
    }

    #[test]
    fn alpha_filters_to_ascii_letters() {
        assert_eq!(
            filter_validation("a1b2c3", OTPFieldValidationType::Alpha),
            "abc"
        );
    }

    #[test]
    fn alphanumeric_filters_to_ascii_letters_and_digits() {
        assert_eq!(
            filter_validation("a-1_b!2", OTPFieldValidationType::Alphanumeric),
            "a1b2"
        );
    }

    #[test]
    fn none_applies_no_character_filter_and_supports_custom_normalization() {
        let normalize = hook(|value| value.to_uppercase());
        let outcome =
            normalize_otp_value("ab💝", OTPFieldValidationType::None, Some(&normalize), 6);

        assert_eq!(outcome.value, "AB💝");
        assert!(!outcome.rejected);
    }

    #[test]
    fn custom_normalization_runs_after_filtering_and_is_refiltered() {
        let normalize = hook(|value| format!("x{value}"));
        let outcome =
            normalize_otp_value("12", OTPFieldValidationType::Numeric, Some(&normalize), 6);

        assert_eq!(outcome.value, "12");
        assert!(outcome.rejected);
    }

    #[test]
    fn clamping_applies_after_custom_normalization() {
        let normalize = hook(|value| format!("{value}{value}"));
        let outcome =
            normalize_otp_value("123", OTPFieldValidationType::Numeric, Some(&normalize), 4);

        assert_eq!(outcome.value, "1231");
        assert!(outcome.rejected);
    }

    #[test]
    fn zero_length_clamps_to_empty() {
        let outcome = normalize_otp_value("123", OTPFieldValidationType::Numeric, None, 0);

        assert_eq!(outcome.value, "");
        assert!(outcome.rejected);
    }

    #[test]
    fn replace_at_first_middle_and_last_slot() {
        let first = replace_at_index("123456", 0, "9", OTPFieldValidationType::Numeric, None, 6);
        assert_eq!(first.value, "923456");
        assert_eq!(first.accepted, 1);

        let middle = replace_at_index("123456", 2, "9", OTPFieldValidationType::Numeric, None, 6);
        assert_eq!(middle.value, "129456");

        let last = replace_at_index("123456", 5, "9", OTPFieldValidationType::Numeric, None, 6);
        assert_eq!(last.value, "123459");
    }

    #[test]
    fn replace_preserves_suffix_when_normalization_shrinks_the_fragment() {
        let outcome = replace_at_index("123456", 2, "x9", OTPFieldValidationType::Numeric, None, 6);

        assert_eq!(outcome.value, "129456");
        assert!(outcome.rejected);
        assert_eq!(outcome.accepted, 1);
    }

    #[test]
    fn replace_reports_rejections_from_final_clamp() {
        let outcome = replace_at_index("1234", 3, "99", OTPFieldValidationType::Numeric, None, 4);

        assert_eq!(outcome.value, "1239");
        assert!(outcome.rejected);
        assert_eq!(outcome.accepted, 1);
    }

    #[test]
    fn remove_at_first_last_and_out_of_bounds_index() {
        assert_eq!(remove_at_index("123", 0), "23");
        assert_eq!(remove_at_index("123", 2), "12");
        assert_eq!(remove_at_index("123", 3), "123");
        assert_eq!(remove_at_index("", 0), "");
    }

    #[test]
    fn clamping_counts_code_points_and_never_splits_multibyte_characters() {
        assert_eq!(clamp_code_points("💝💝💝", 2), "💝💝");
        let outcome = normalize_otp_value("💝💝💝", OTPFieldValidationType::None, None, 2);
        assert_eq!(outcome.value, "💝💝");
        assert!(outcome.rejected);
    }

    #[test]
    fn rejection_details_are_reported_per_stage() {
        let clean = normalize_otp_value("123", OTPFieldValidationType::Numeric, None, 6);
        assert!(!clean.rejected);

        let filtered = normalize_otp_value("1a3", OTPFieldValidationType::Numeric, None, 6);
        assert!(filtered.rejected);

        let whitespace_only =
            normalize_otp_value(" 1 2 ", OTPFieldValidationType::Numeric, None, 6);
        assert!(!whitespace_only.rejected);
    }
}
