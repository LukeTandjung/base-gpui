use std::time::Duration;

use gpui::{px, Bounds, Pixels, Point};

pub fn pt(x: f32, y: f32) -> Point<Pixels> {
    Point::new(px(x), px(y))
}

pub fn rect(x: f32, y: f32, width: f32, height: f32) -> Bounds<Pixels> {
    Bounds::new(pt(x, y), gpui::Size::new(px(width), px(height)))
}

pub fn ms(value: u64) -> Duration {
    Duration::from_millis(value)
}
