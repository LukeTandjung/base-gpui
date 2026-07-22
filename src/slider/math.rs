//! Pure slider value math ported from Base UI's `slider/utils/`.
//!
//! These are slider-specific ports of `roundValueToStep`, `getSliderValue`,
//! `resolveThumbCollision`, `getPushedThumbValues`, and
//! `validateMinimumDistance`. The decimal-precision rule intentionally mirrors
//! Base UI's `toFixed`-based cleanup rather than
//! `number_field/number.rs::clean_floating_point_noise`, because the slider
//! grid rule rounds to an explicit precision derived from `step`/`min`.

/// Thumb collision behaviors for pointer drags in range mode.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SliderThumbCollisionBehavior {
    #[default]
    Push,
    Swap,
    None,
}

/// Outcome of resolving a proposed pointer value against neighboring thumbs.
#[derive(Clone, Debug, PartialEq)]
pub struct SliderCollisionResult {
    pub values: Vec<f64>,
    pub thumb_index: usize,
    pub did_swap: bool,
}

/// Port of Base UI `getDecimalPrecision`.
pub fn get_decimal_precision(value: f64) -> u32 {
    if value == 0.0 || !value.is_finite() {
        return 0;
    }

    if value.abs() < 1.0 {
        let formatted = format!("{value:e}");
        let (mantissa, exponent) = formatted
            .split_once('e')
            .expect("exponential formatting always contains an exponent");
        let exponent = exponent.parse::<i32>().unwrap_or(0);
        let mantissa_decimals = mantissa
            .split_once('.')
            .map(|(_, decimals)| decimals.len() as i32)
            .unwrap_or(0);
        return (mantissa_decimals - exponent).max(0) as u32;
    }

    let formatted = value.to_string();
    formatted
        .split_once('.')
        .map(|(_, decimals)| decimals.len() as u32)
        .unwrap_or(0)
}

/// Rounds a value to the given number of decimal places (JS `toFixed` equivalent).
pub fn round_to_precision(value: f64, digits: u32) -> f64 {
    let factor = 10f64.powi(digits.min(15) as i32);
    (value * factor).round() / factor
}

/// Port of Base UI `roundValueToStep`: nearest step multiple with `min` as the
/// grid origin, rounded to `max(precision(step), precision(min))` decimals.
pub fn round_value_to_step(value: f64, step: f64, min: f64) -> f64 {
    let nearest = ((value - min) / step).round() * step + min;
    round_to_precision(
        nearest,
        get_decimal_precision(step).max(get_decimal_precision(min)),
    )
}

/// Clamps a value into `[min, max]`.
pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}

/// Port of Base UI `valueToPercent`, expressed as a `[0, 1]` fraction.
pub fn value_to_fraction(value: f64, min: f64, max: f64) -> f64 {
    if max <= min {
        return 0.0;
    }
    clamp((value - min) / (max - min), 0.0, 1.0)
}

/// Port of Base UI `getSliderValue`: clamps a per-thumb keyboard/programmatic
/// change to `[min, max]` and, in range mode, to the hard bounds of the
/// neighboring values. Never pushes or swaps.
pub fn get_slider_value(
    value_input: f64,
    index: usize,
    min: f64,
    max: f64,
    values: &[f64],
) -> Vec<f64> {
    let mut new_value = clamp(value_input, min, max);

    if values.len() > 1 {
        let lower = if index > 0 {
            values[index - 1]
        } else {
            f64::NEG_INFINITY
        };
        let upper = values.get(index + 1).copied().unwrap_or(f64::INFINITY);
        new_value = clamp(new_value, lower, upper);
    }

    let mut next = values.to_vec();
    if index < next.len() {
        next[index] = new_value;
    }
    next
}

/// Port of Base UI `validateMinimumDistance`.
pub fn validate_minimum_distance(values: &[f64], step: f64, min_steps_between_values: f64) -> bool {
    if values.len() < 2 {
        return true;
    }

    let min_distance = values
        .windows(2)
        .map(|pair| (pair[0] - pair[1]).abs())
        .fold(f64::INFINITY, f64::min);
    min_distance >= step * min_steps_between_values
}

/// Port of Base UI `getPushedThumbValues`: moving the thumb at `index` beyond
/// its neighbours pushes them while respecting `minStepsBetweenValues`, and
/// pushed neighbours restore toward `initial_values` without overshooting.
pub fn get_pushed_thumb_values(
    values: &[f64],
    index: usize,
    next_value: f64,
    min: f64,
    max: f64,
    step: f64,
    min_steps_between_values: f64,
    initial_values: Option<&[f64]>,
) -> Vec<f64> {
    if values.is_empty() {
        return Vec::from([]);
    }

    let mut next_values = values.to_vec();
    let min_value_difference = step * min_steps_between_values;
    let last_index = next_values.len() - 1;
    let base_initial_values = initial_values.unwrap_or(values);

    let index_min = min + index as f64 * min_value_difference;
    let index_max = max - (last_index - index) as f64 * min_value_difference;
    next_values[index] = clamp(next_value, index_min, index_max);

    for i in (index + 1)..=last_index {
        let min_allowed = next_values[i - 1] + min_value_difference;
        let max_allowed = max - (last_index - i) as f64 * min_value_difference;
        let initial_value = base_initial_values
            .get(i)
            .copied()
            .unwrap_or(next_values[i]);
        let mut candidate = next_values[i].max(min_allowed);

        if initial_value < candidate {
            candidate = initial_value.max(min_allowed);
        }

        next_values[i] = clamp(candidate, min_allowed, max_allowed);
    }

    for i in (0..index).rev() {
        let max_allowed = next_values[i + 1] - min_value_difference;
        let min_allowed = min + i as f64 * min_value_difference;
        let initial_value = base_initial_values
            .get(i)
            .copied()
            .unwrap_or(next_values[i]);
        let mut candidate = next_values[i].min(max_allowed);

        if initial_value > candidate {
            candidate = initial_value.min(max_allowed);
        }

        next_values[i] = clamp(candidate, min_allowed, max_allowed);
    }

    for value in next_values.iter_mut() {
        *value = round_to_precision(*value, 12);
    }

    next_values
}

/// Port of Base UI `resolveThumbCollision`: resolves a proposed pointer value
/// for the pressed thumb against the configured collision behavior.
pub fn resolve_thumb_collision(
    behavior: SliderThumbCollisionBehavior,
    values: &[f64],
    initial_values: Option<&[f64]>,
    pressed_index: usize,
    next_value: f64,
    min: f64,
    max: f64,
    step: f64,
    min_steps_between_values: f64,
) -> SliderCollisionResult {
    let baseline_values = initial_values.unwrap_or(values);

    if values.len() < 2 {
        return SliderCollisionResult {
            values: Vec::from([next_value]),
            thumb_index: 0,
            did_swap: false,
        };
    }

    let min_value_difference = step * min_steps_between_values;

    match behavior {
        SliderThumbCollisionBehavior::Swap => {
            let pressed_initial_value = values[pressed_index];
            let epsilon = 1e-7;
            let mut candidate_values = values.to_vec();
            let previous_neighbor = if pressed_index > 0 {
                Some(candidate_values[pressed_index - 1])
            } else {
                None
            };
            let next_neighbor = candidate_values.get(pressed_index + 1).copied();

            let lower_bound = previous_neighbor
                .map(|neighbor| neighbor + min_value_difference)
                .unwrap_or(min);
            let upper_bound = next_neighbor
                .map(|neighbor| neighbor - min_value_difference)
                .unwrap_or(max);

            let constrained_value = clamp(next_value, lower_bound, upper_bound);
            let pressed_value_after_clamp = round_to_precision(constrained_value, 12);
            candidate_values[pressed_index] = pressed_value_after_clamp;

            let moving_forward = next_value > pressed_initial_value;
            let moving_backward = next_value < pressed_initial_value;

            let should_swap_forward = moving_forward
                && next_neighbor
                    .map(|neighbor| next_value >= neighbor - epsilon)
                    .unwrap_or(false);
            let should_swap_backward = moving_backward
                && previous_neighbor
                    .map(|neighbor| next_value <= neighbor + epsilon)
                    .unwrap_or(false);

            if !should_swap_forward && !should_swap_backward {
                return SliderCollisionResult {
                    values: candidate_values,
                    thumb_index: pressed_index,
                    did_swap: false,
                };
            }

            let target_index = if should_swap_forward {
                pressed_index + 1
            } else {
                pressed_index - 1
            };

            let initial_values_for_push = candidate_values
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    if index == pressed_index {
                        pressed_value_after_clamp
                    } else {
                        baseline_values.get(index).copied().unwrap_or(*value)
                    }
                })
                .collect::<Vec<_>>();

            let next_value_for_target = if should_swap_forward {
                next_value.max(candidate_values[target_index])
            } else {
                next_value.min(candidate_values[target_index])
            };

            let mut adjusted_values = get_pushed_thumb_values(
                &candidate_values,
                target_index,
                next_value_for_target,
                min,
                max,
                step,
                min_steps_between_values,
                Some(&initial_values_for_push),
            );

            let neighbor_index = if should_swap_forward {
                target_index as i64 - 1
            } else {
                target_index as i64 + 1
            };

            if neighbor_index >= 0 && (neighbor_index as usize) < adjusted_values.len() {
                let neighbor_index = neighbor_index as usize;
                let previous_value = if neighbor_index > 0 {
                    Some(adjusted_values[neighbor_index - 1])
                } else {
                    None
                };
                let next_value_after = adjusted_values.get(neighbor_index + 1).copied();

                let mut neighbor_lower_bound = previous_value
                    .map(|value| value + min_value_difference)
                    .unwrap_or(min);
                neighbor_lower_bound =
                    neighbor_lower_bound.max(min + neighbor_index as f64 * min_value_difference);

                let mut neighbor_upper_bound = next_value_after
                    .map(|value| value - min_value_difference)
                    .unwrap_or(max);
                neighbor_upper_bound = neighbor_upper_bound.min(
                    max - (adjusted_values.len() - 1 - neighbor_index) as f64
                        * min_value_difference,
                );

                let restored_value = clamp(
                    pressed_value_after_clamp,
                    neighbor_lower_bound,
                    neighbor_upper_bound,
                );
                adjusted_values[neighbor_index] = round_to_precision(restored_value, 12);
            }

            SliderCollisionResult {
                values: adjusted_values,
                thumb_index: target_index,
                did_swap: true,
            }
        }
        SliderThumbCollisionBehavior::Push => SliderCollisionResult {
            values: get_pushed_thumb_values(
                values,
                pressed_index,
                next_value,
                min,
                max,
                step,
                min_steps_between_values,
                None,
            ),
            thumb_index: pressed_index,
            did_swap: false,
        },
        SliderThumbCollisionBehavior::None => {
            let mut candidate_values = values.to_vec();
            let previous_neighbor = if pressed_index > 0 {
                Some(candidate_values[pressed_index - 1])
            } else {
                None
            };
            let next_neighbor = candidate_values.get(pressed_index + 1).copied();

            let lower_bound = previous_neighbor
                .map(|neighbor| neighbor + min_value_difference)
                .unwrap_or(min);
            let upper_bound = next_neighbor
                .map(|neighbor| neighbor - min_value_difference)
                .unwrap_or(max);

            let constrained_value = clamp(next_value, lower_bound, upper_bound);
            candidate_values[pressed_index] = round_to_precision(constrained_value, 12);

            SliderCollisionResult {
                values: candidate_values,
                thumb_index: pressed_index,
                did_swap: false,
            }
        }
    }
}

/// Keyboard stepping: rounds the current value to the step grid, applies the
/// signed increment, cleans decimal precision (max of value/increment/min
/// precisions), then clamps to `[min, max]`.
pub fn get_new_value(current: f64, increment: f64, step: f64, min: f64, max: f64) -> f64 {
    let rounded = round_value_to_step(current, step, min);
    let raw = rounded + increment;
    let precision = get_decimal_precision(step)
        .max(get_decimal_precision(min))
        .max(get_decimal_precision(increment));
    clamp(round_to_precision(raw, precision), min, max)
}

/// Converts a pointer main-axis position to a `[0, 1]` fraction of the control,
/// with an `inset` on both ends (edge alignment's half-thumb inset) and an
/// optional reversed axis (RTL horizontal, or vertical measured from bottom).
pub fn position_to_fraction(
    position: f64,
    control_start: f64,
    control_size: f64,
    inset: f64,
    reversed: bool,
) -> f64 {
    let effective_size = control_size - 2.0 * inset;
    if effective_size <= 0.0 {
        return 0.0;
    }

    let mut fraction = (position - control_start - inset) / effective_size;
    if reversed {
        fraction = 1.0 - fraction;
    }
    clamp(fraction, 0.0, 1.0)
}

/// Scales a `[0, 1]` fraction into the `[min, max]` value domain.
pub fn fraction_to_value(fraction: f64, min: f64, max: f64) -> f64 {
    (max - min) * fraction + min
}

#[cfg(test)]
mod tests {
    use super::{
        clamp, fraction_to_value, get_decimal_precision, get_new_value, get_pushed_thumb_values,
        get_slider_value, position_to_fraction, resolve_thumb_collision, round_value_to_step,
        validate_minimum_distance, SliderThumbCollisionBehavior,
    };

    #[test]
    fn decimal_precision_handles_small_magnitudes() {
        assert_eq!(get_decimal_precision(0.0), 0);
        assert_eq!(get_decimal_precision(1.0), 0);
        assert_eq!(get_decimal_precision(0.1), 1);
        assert_eq!(get_decimal_precision(0.25), 2);
        assert_eq!(get_decimal_precision(0.000_000_01), 8);
        assert_eq!(get_decimal_precision(1.55), 2);
    }

    #[test]
    fn round_value_to_step_uses_min_as_origin() {
        assert_eq!(round_value_to_step(5.4, 1.0, 0.0), 5.0);
        assert_eq!(round_value_to_step(5.6, 1.0, 0.0), 6.0);
        assert_eq!(round_value_to_step(0.25, 0.1, 0.0), 0.3);
        assert_eq!(round_value_to_step(7.0, 3.0, 1.0), 7.0);
        assert_eq!(round_value_to_step(8.0, 3.0, 1.0), 7.0);
        assert_eq!(round_value_to_step(0.121, 0.01, 0.005), 0.125);
    }

    #[test]
    fn round_value_to_step_stays_precision_clean_for_decimal_steps() {
        assert_eq!(round_value_to_step(0.1 + 0.2, 0.1, 0.0), 0.3);
    }

    #[test]
    fn get_slider_value_clamps_to_range_bounds() {
        assert_eq!(get_slider_value(150.0, 0, 0.0, 100.0, &[50.0]), [100.0]);
        assert_eq!(get_slider_value(-10.0, 0, 0.0, 100.0, &[50.0]), [0.0]);
    }

    #[test]
    fn get_slider_value_clamps_to_neighbors_in_range_mode() {
        assert_eq!(
            get_slider_value(80.0, 0, 0.0, 100.0, &[20.0, 60.0]),
            [60.0, 60.0]
        );
        assert_eq!(
            get_slider_value(5.0, 1, 0.0, 100.0, &[20.0, 60.0]),
            [20.0, 20.0]
        );
        assert_eq!(
            get_slider_value(40.0, 1, 0.0, 100.0, &[20.0, 60.0, 80.0]),
            [20.0, 40.0, 80.0]
        );
    }

    #[test]
    fn validate_minimum_distance_accepts_and_rejects() {
        assert!(validate_minimum_distance(&[10.0], 1.0, 5.0));
        assert!(validate_minimum_distance(&[10.0, 20.0], 1.0, 10.0));
        assert!(!validate_minimum_distance(&[10.0, 15.0], 1.0, 10.0));
        assert!(validate_minimum_distance(&[10.0, 20.0, 30.0], 1.0, 10.0));
        assert!(!validate_minimum_distance(&[10.0, 20.0, 25.0], 1.0, 10.0));
    }

    #[test]
    fn pushed_values_push_neighbors_forward_and_backward() {
        assert_eq!(
            get_pushed_thumb_values(&[20.0, 40.0], 0, 50.0, 0.0, 100.0, 1.0, 0.0, None),
            [50.0, 50.0]
        );
        assert_eq!(
            get_pushed_thumb_values(&[20.0, 40.0], 1, 10.0, 0.0, 100.0, 1.0, 0.0, None),
            [10.0, 10.0]
        );
    }

    #[test]
    fn pushed_values_respect_min_distance_and_budgets() {
        assert_eq!(
            get_pushed_thumb_values(&[20.0, 40.0], 0, 50.0, 0.0, 100.0, 1.0, 10.0, None),
            [50.0, 60.0]
        );
        // Pressed thumb budget at max: index 0 of 2 thumbs cannot exceed max - distance.
        assert_eq!(
            get_pushed_thumb_values(&[20.0, 40.0], 0, 99.0, 0.0, 100.0, 1.0, 10.0, None),
            [90.0, 100.0]
        );
        // Budget at min.
        assert_eq!(
            get_pushed_thumb_values(&[60.0, 80.0], 1, 2.0, 0.0, 100.0, 1.0, 10.0, None),
            [0.0, 10.0]
        );
    }

    #[test]
    fn pushed_values_restore_toward_initial_values() {
        let initial = [20.0, 40.0];
        // Push forward past the neighbor, then retreat: neighbor restores to its
        // initial value without overshooting.
        let pushed =
            get_pushed_thumb_values(&[45.0, 45.0], 0, 30.0, 0.0, 100.0, 1.0, 0.0, Some(&initial));
        assert_eq!(pushed, [30.0, 40.0]);
    }

    #[test]
    fn collision_push_behavior_pushes_neighbors() {
        let result = resolve_thumb_collision(
            SliderThumbCollisionBehavior::Push,
            &[20.0, 40.0],
            None,
            0,
            50.0,
            0.0,
            100.0,
            1.0,
            0.0,
        );
        assert_eq!(result.values, [50.0, 50.0]);
        assert_eq!(result.thumb_index, 0);
        assert!(!result.did_swap);
    }

    #[test]
    fn collision_swap_behavior_swaps_past_neighbor() {
        let result = resolve_thumb_collision(
            SliderThumbCollisionBehavior::Swap,
            &[20.0, 40.0],
            None,
            0,
            50.0,
            0.0,
            100.0,
            1.0,
            0.0,
        );
        assert!(result.did_swap);
        assert_eq!(result.thumb_index, 1);
        assert_eq!(result.values, [40.0, 50.0]);
    }

    #[test]
    fn collision_swap_epsilon_boundary_does_not_swap_below_threshold() {
        let result = resolve_thumb_collision(
            SliderThumbCollisionBehavior::Swap,
            &[20.0, 40.0],
            None,
            0,
            40.0 - 5e-8,
            0.0,
            100.0,
            1.0,
            0.0,
        );
        // Within epsilon of the neighbor: swaps.
        assert!(result.did_swap);

        let result = resolve_thumb_collision(
            SliderThumbCollisionBehavior::Swap,
            &[20.0, 40.0],
            None,
            0,
            39.0,
            0.0,
            100.0,
            1.0,
            0.0,
        );
        assert!(!result.did_swap);
        assert_eq!(result.values, [39.0, 40.0]);
    }

    #[test]
    fn collision_none_behavior_clamps_to_neighbors() {
        let result = resolve_thumb_collision(
            SliderThumbCollisionBehavior::None,
            &[20.0, 40.0],
            None,
            0,
            50.0,
            0.0,
            100.0,
            1.0,
            0.0,
        );
        assert_eq!(result.values, [40.0, 40.0]);
        assert!(!result.did_swap);

        let result = resolve_thumb_collision(
            SliderThumbCollisionBehavior::None,
            &[20.0, 40.0],
            None,
            0,
            50.0,
            0.0,
            100.0,
            1.0,
            5.0,
        );
        assert_eq!(result.values, [35.0, 40.0]);
    }

    #[test]
    fn collision_single_value_bypasses_resolution() {
        let result = resolve_thumb_collision(
            SliderThumbCollisionBehavior::Push,
            &[20.0],
            None,
            0,
            150.0,
            0.0,
            100.0,
            1.0,
            0.0,
        );
        assert_eq!(result.values, [150.0]);
        assert_eq!(result.thumb_index, 0);
    }

    #[test]
    fn keyboard_new_value_is_grid_rounded_and_precision_clean() {
        assert_eq!(get_new_value(0.3, 0.1, 0.1, 0.0, 1.0), 0.4);
        assert_eq!(get_new_value(0.300_000_000_04, 0.1, 0.1, 0.0, 1.0), 0.4);
        assert_eq!(get_new_value(99.5, 1.0, 1.0, 0.0, 100.0), 100.0);
        assert_eq!(get_new_value(0.0, -1.0, 1.0, 0.0, 100.0), 0.0);
    }

    #[test]
    fn position_to_fraction_maps_axes() {
        assert_eq!(position_to_fraction(50.0, 0.0, 100.0, 0.0, false), 0.5);
        assert_eq!(position_to_fraction(25.0, 0.0, 100.0, 0.0, true), 0.75);
        assert_eq!(position_to_fraction(-10.0, 0.0, 100.0, 0.0, false), 0.0);
        assert_eq!(position_to_fraction(150.0, 0.0, 100.0, 0.0, false), 1.0);
        // Edge alignment inset of 10 on each end.
        assert_eq!(position_to_fraction(50.0, 0.0, 100.0, 10.0, false), 0.5);
        assert_eq!(position_to_fraction(10.0, 0.0, 100.0, 10.0, false), 0.0);
        assert_eq!(position_to_fraction(90.0, 0.0, 100.0, 10.0, false), 1.0);
    }

    #[test]
    fn fraction_to_value_scales_to_domain() {
        assert_eq!(fraction_to_value(0.5, 0.0, 100.0), 50.0);
        assert_eq!(fraction_to_value(0.0, -50.0, 50.0), -50.0);
        assert_eq!(
            clamp(fraction_to_value(1.0, -50.0, 50.0), -50.0, 50.0),
            50.0
        );
    }
}
