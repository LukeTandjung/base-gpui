//! Private child wiring: the one place that knows which Root children are
//! which part and attaches the shared context before erasure. Scroll Area
//! needs no indices or metadata collection.

use crate::scroll_area::{ScrollAreaChild, ScrollAreaContext, ScrollAreaViewportChild};

pub trait ScrollAreaChildNode: Sized {
    fn with_scroll_area_context(self, context: ScrollAreaContext) -> Self;
}

impl ScrollAreaChildNode for ScrollAreaChild {
    fn with_scroll_area_context(self, context: ScrollAreaContext) -> Self {
        match self {
            Self::Viewport(viewport) => Self::Viewport(viewport.with_scroll_area_context(context)),
            Self::Scrollbar(scrollbar) => {
                Self::Scrollbar(scrollbar.with_scroll_area_context(context))
            }
            Self::Corner(corner) => Self::Corner(corner.with_scroll_area_context(context)),
        }
    }
}

impl ScrollAreaChildNode for ScrollAreaViewportChild {
    fn with_scroll_area_context(self, context: ScrollAreaContext) -> Self {
        match self {
            Self::Content(content) => Self::Content(content.with_scroll_area_context(context)),
            Self::Any(element) => Self::Any(element),
        }
    }
}

pub fn wire_children(
    children: Vec<ScrollAreaChild>,
    context: ScrollAreaContext,
) -> Vec<ScrollAreaChild> {
    children
        .into_iter()
        .map(|child| child.with_scroll_area_context(context.clone()))
        .collect()
}
