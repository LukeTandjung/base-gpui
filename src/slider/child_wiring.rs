//! Private child traversal: the only place that assigns thumb indices (in
//! declaration order) and attaches the slider context.

use crate::slider::{SliderChild, SliderContext, SliderControlChild, SliderTrackChild};

pub struct WiredSliderChildren {
    pub children: Vec<SliderChild>,
    /// Per-thumb `disabled` flags, in declaration (index) order.
    pub thumb_disabled: Vec<bool>,
}

pub fn wire_children(children: Vec<SliderChild>, context: SliderContext) -> WiredSliderChildren {
    let mut next_thumb_index = 0usize;
    let mut thumb_disabled = Vec::from([]);

    let children = children
        .into_iter()
        .map(|child| match child {
            SliderChild::Control(control) => {
                let control = control.with_slider_context(context.clone());
                let control = control.map_children(|children| {
                    children
                        .into_iter()
                        .map(|child| match child {
                            SliderControlChild::Track(track) => {
                                let track = track.with_slider_context(context.clone());
                                let track = track.map_children(|children| {
                                    children
                                        .into_iter()
                                        .map(|child| match child {
                                            SliderTrackChild::Indicator(indicator) => {
                                                SliderTrackChild::Indicator(Box::new(
                                                    indicator.with_slider_context(context.clone()),
                                                ))
                                            }
                                            SliderTrackChild::Any(any) => {
                                                SliderTrackChild::Any(any)
                                            }
                                        })
                                        .collect()
                                });
                                SliderControlChild::Track(Box::new(track))
                            }
                            SliderControlChild::Thumb(thumb) => {
                                let index = next_thumb_index;
                                next_thumb_index += 1;
                                thumb_disabled.push(thumb.thumb_disabled());
                                SliderControlChild::Thumb(Box::new(
                                    thumb
                                        .with_slider_context(context.clone())
                                        .with_thumb_index(index),
                                ))
                            }
                            SliderControlChild::Any(any) => SliderControlChild::Any(any),
                        })
                        .collect()
                });
                SliderChild::Control(Box::new(control))
            }
            SliderChild::Value(value) => {
                SliderChild::Value(Box::new(value.with_slider_context(context.clone())))
            }
            SliderChild::Label(label) => {
                SliderChild::Label(Box::new(label.with_slider_context(context.clone())))
            }
            SliderChild::Any(any) => SliderChild::Any(any),
        })
        .collect();

    WiredSliderChildren {
        children,
        thumb_disabled,
    }
}
