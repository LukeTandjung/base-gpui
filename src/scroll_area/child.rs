//! Typed Scroll Area child enums: which parts each compound layer accepts,
//! kept typed until the root/viewport render erases them to elements.

use gpui::{AnyElement, IntoElement};

use crate::scroll_area::{
    ScrollAreaContent, ScrollAreaCorner, ScrollAreaScrollbar, ScrollAreaThumb, ScrollAreaViewport,
};

/// Children the Root accepts.
pub enum ScrollAreaChild {
    Viewport(ScrollAreaViewport),
    Scrollbar(ScrollAreaScrollbar),
    Corner(ScrollAreaCorner),
}

impl IntoElement for ScrollAreaChild {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Viewport(viewport) => viewport.into_any_element(),
            Self::Scrollbar(scrollbar) => scrollbar.into_any_element(),
            Self::Corner(corner) => corner.into_any_element(),
        }
    }
}

impl From<ScrollAreaViewport> for ScrollAreaChild {
    fn from(value: ScrollAreaViewport) -> Self {
        Self::Viewport(value)
    }
}

impl From<ScrollAreaScrollbar> for ScrollAreaChild {
    fn from(value: ScrollAreaScrollbar) -> Self {
        Self::Scrollbar(value)
    }
}

impl From<ScrollAreaCorner> for ScrollAreaChild {
    fn from(value: ScrollAreaCorner) -> Self {
        Self::Corner(value)
    }
}

/// Children the Viewport accepts: the optional Content wrapper or arbitrary
/// content directly (Content is optional for vertical-only use, as in
/// Base UI).
pub enum ScrollAreaViewportChild {
    Content(ScrollAreaContent),
    Any(AnyElement),
}

impl IntoElement for ScrollAreaViewportChild {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Content(content) => content.into_any_element(),
            Self::Any(element) => element,
        }
    }
}

impl From<ScrollAreaContent> for ScrollAreaViewportChild {
    fn from(value: ScrollAreaContent) -> Self {
        Self::Content(value)
    }
}

impl From<AnyElement> for ScrollAreaViewportChild {
    fn from(value: AnyElement) -> Self {
        Self::Any(value)
    }
}

/// Children a Scrollbar accepts: its Thumb.
pub enum ScrollAreaScrollbarChild {
    Thumb(ScrollAreaThumb),
}

impl From<ScrollAreaThumb> for ScrollAreaScrollbarChild {
    fn from(value: ScrollAreaThumb) -> Self {
        Self::Thumb(value)
    }
}
