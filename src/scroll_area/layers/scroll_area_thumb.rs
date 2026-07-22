//! Scroll Area Thumb: styling configuration for the composed scrollbar
//! primitive's thumb. The thumb owns no drag handlers or geometry — the
//! primitive (`primitives/scroll`) performs dragging, track clicks, and
//! thumb sizing; this part only maps Scroll Area facts to a
//! [`ScrollbarStyle`].

use std::rc::Rc;

use crate::primitives::scroll::ScrollbarStyle;
use crate::scroll_area::ScrollAreaThumbStyleState;

type ThumbStyleFn = dyn Fn(ScrollAreaThumbStyleState, ScrollbarStyle) -> ScrollbarStyle + 'static;

#[derive(Default)]
pub struct ScrollAreaThumb {
    style_with_state: Option<Rc<ThumbStyleFn>>,
}

impl ScrollAreaThumb {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adjust the primitive's thumb/track appearance from the thumb's
    /// Scroll Area facts. Receives [`ScrollbarStyle::default`] and returns
    /// the style to use for the current frame.
    pub fn style_with_state(
        mut self,
        style: impl Fn(ScrollAreaThumbStyleState, ScrollbarStyle) -> ScrollbarStyle + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }

    /// The configured style callback, consumed by the Scrollbar part when
    /// it builds the primitive.
    pub fn take_style(self) -> Option<Rc<ThumbStyleFn>> {
        self.style_with_state
    }
}
