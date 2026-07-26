use base_gpui::slider::{
    SliderControl, SliderIndicator, SliderRoot, SliderThumb, SliderTrack, SliderValues,
};
use gpui::prelude::*;
use gpui::px;

use crate::theme;

pub fn render() -> impl IntoElement {
    SliderRoot::new()
        .id("demo-slider")
        .name("volume")
        .default_value(SliderValues::Single(40.))
        .min(0.)
        .max(100.)
        .step(1.)
        .w(px(240.))
        .child(
            SliderControl::new()
                .id("demo-slider-control")
                .w_full()
                .h(px(20.))
                .child(
                    // SliderControl lays its children out in an internal
                    // full-size wrapper, so styles on the control itself can't
                    // center them; the track and thumb each center against the
                    // 20px control height by hand (track 4px -> mt 8, thumb
                    // 16px -> top 2).
                    SliderTrack::new()
                        .id("demo-slider-track")
                        .w_full()
                        .mt(px(8.))
                        .h(px(4.))
                        .rounded_full()
                        .bg(theme::control())
                        .child(
                            SliderIndicator::new()
                                .id("demo-slider-indicator")
                                .h(px(4.))
                                .rounded_full()
                                .bg(theme::accent()),
                        ),
                )
                .child(
                    SliderThumb::new()
                        .id("demo-slider-thumb")
                        .aria_label("Volume")
                        // The thumb is absolutely positioned by the library on
                        // the main axis only; vertically it sits at its static
                        // position, flush under the track. Pull it up by half
                        // its height plus half the track height to center it.
                        .mt(px(-10.))
                        .w(px(16.))
                        .h(px(16.))
                        .rounded_full()
                        .bg(theme::background())
                        .border_2()
                        .border_color(theme::accent())
                        .style_with_state(|state, thumb| {
                            if state.focused {
                                thumb.border_color(theme::focus_ring())
                            } else {
                                thumb
                            }
                        }),
                ),
        )
}
