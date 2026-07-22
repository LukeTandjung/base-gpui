use std::time::Duration;

use gpui::{px, Pixels};

/// Tuning knobs for the safe-polygon tracker, defaulting to Floating UI's
/// constants.
///
/// - `polygon_buffer` widens the quadrilateral around the exit point and pulls
///   its far edge slightly inside the popup. Larger values are more forgiving
///   of wobbly diagonal paths but keep the popup open over more neighboring
///   space (risking stuck-open feel over sibling items).
/// - `cursor_speed_threshold` (px/ms) is the minimum cursor speed that counts
///   as travel intent inside the quadrilateral. Lower values tolerate slower,
///   more hesitant pointers; higher values close sooner when the user parks
///   mid-flight.
/// - `inside_grace` is how long the consumer should reschedule the pending
///   close after each `Inside` verdict. Shorter values close faster once
///   movement stops; longer values keep the popup open through brief pauses.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SafePolygonConfig {
    pub polygon_buffer: Pixels,
    pub cursor_speed_threshold: f32,
    pub inside_grace: Duration,
}

impl Default for SafePolygonConfig {
    fn default() -> Self {
        Self {
            polygon_buffer: px(0.5),
            cursor_speed_threshold: 0.1,
            inside_grace: Duration::from_millis(40),
        }
    }
}
