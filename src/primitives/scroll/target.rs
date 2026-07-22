//! `ScrollTarget`: the trait boundary between the scrollbar primitive and
//! whatever actually scrolls the content.
//!
//! The scrollbar never owns scroll position; it reads and writes it through
//! this trait only. Implementations are provided for GPUI's [`ScrollHandle`]
//! (plain `.overflow_scroll()` divs), [`UniformListScrollHandle`]
//! (`uniform_list`), and [`ListState`] (`list`). Consumers with custom scroll
//! containers can implement it themselves.

use gpui::{size, ListState, Pixels, Point, ScrollHandle, Size, UniformListScrollHandle};

/// Abstraction over a scrollable container the scrollbar reads from and
/// writes to.
///
/// Offsets follow GPUI's convention: zero at content start, growing
/// *negative* as the content scrolls; the valid range on an axis is
/// `-(content_len - viewport_len)..=0`.
pub trait ScrollTarget: 'static {
    /// Current scroll offset (zero-or-negative).
    fn offset(&self) -> Point<Pixels>;

    /// Move the scroll position.
    fn set_offset(&self, offset: Point<Pixels>);

    /// Full scrollable content size, including the part outside the viewport.
    fn content_size(&self) -> Size<Pixels>;

    /// Visible container size.
    fn viewport_size(&self) -> Size<Pixels>;

    /// Called when a scrollbar thumb drag starts. Virtualized targets
    /// ([`ListState`]) use this to keep measurement stable during the drag.
    fn drag_started(&self) {}

    /// Called when a scrollbar thumb drag ends.
    fn drag_ended(&self) {}
}

impl ScrollTarget for ScrollHandle {
    fn offset(&self) -> Point<Pixels> {
        self.offset()
    }

    fn set_offset(&self, offset: Point<Pixels>) {
        self.set_offset(offset);
    }

    fn content_size(&self) -> Size<Pixels> {
        let max_offset = self.max_offset();
        let viewport = self.bounds().size;
        size(
            max_offset.x + viewport.width,
            max_offset.y + viewport.height,
        )
    }

    fn viewport_size(&self) -> Size<Pixels> {
        self.bounds().size
    }
}

impl ScrollTarget for UniformListScrollHandle {
    fn offset(&self) -> Point<Pixels> {
        self.0.borrow().base_handle.offset()
    }

    fn set_offset(&self, offset: Point<Pixels>) {
        self.0.borrow().base_handle.set_offset(offset);
    }

    fn content_size(&self) -> Size<Pixels> {
        let state = self.0.borrow();
        let max_offset = state.base_handle.max_offset();
        let viewport = state.base_handle.bounds().size;
        size(
            max_offset.x + viewport.width,
            max_offset.y + viewport.height,
        )
    }

    fn viewport_size(&self) -> Size<Pixels> {
        self.0.borrow().base_handle.bounds().size
    }
}

impl ScrollTarget for ListState {
    fn offset(&self) -> Point<Pixels> {
        self.scroll_px_offset_for_scrollbar()
    }

    fn set_offset(&self, offset: Point<Pixels>) {
        self.set_offset_from_scrollbar(offset);
    }

    fn content_size(&self) -> Size<Pixels> {
        let max_offset = self.max_offset_for_scrollbar();
        let viewport = self.viewport_bounds().size;
        size(
            max_offset.x + viewport.width,
            max_offset.y + viewport.height,
        )
    }

    fn viewport_size(&self) -> Size<Pixels> {
        self.viewport_bounds().size
    }

    fn drag_started(&self) {
        self.scrollbar_drag_started();
    }

    fn drag_ended(&self) {
        self.scrollbar_drag_ended();
    }
}
