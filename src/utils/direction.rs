//! Ambient text-direction support.
//!
//! `DirectionProvider` scopes behavior-only direction to its element subtree. It intentionally
//! does not set text layout, shaping, or styling direction. Single-child providers delegate directly
//! to that child. GPUI currently needs a wrapper element for multiple children, so that case renders
//! a default `div` around the children.
//!
//! Future portal or overlay components that render outside the provider's element subtree should
//! explicitly bridge direction by capturing `current_direction()` while still rendering inside the
//! source subtree and wrapping the detached content in `DirectionProvider::new().direction(captured)`.
//! The provider stack is render-scoped, so detached rendering should not rely on implicit ambient
//! inheritance.

use std::cell::RefCell;

use gpui::{
    div, AnyElement, App, Bounds, Element, ElementId, GlobalElementId, InspectorElementId,
    IntoElement, LayoutId, ParentElement, Pixels, RenderOnce, Window,
};

#[cfg(test)]
mod tests;

/// Ambient text direction for behavior that depends on horizontal reading order.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TextDirection {
    #[default]
    Ltr,
    Rtl,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HorizontalDirection {
    Previous,
    Next,
}

impl TextDirection {
    pub fn is_ltr(self) -> bool {
        self == Self::Ltr
    }

    pub fn is_rtl(self) -> bool {
        self == Self::Rtl
    }

    pub fn horizontal_arrow(self, key: HorizontalArrowKey) -> HorizontalDirection {
        match (self, key) {
            (Self::Ltr, HorizontalArrowKey::Left) | (Self::Rtl, HorizontalArrowKey::Right) => {
                HorizontalDirection::Previous
            }
            (Self::Ltr, HorizontalArrowKey::Right) | (Self::Rtl, HorizontalArrowKey::Left) => {
                HorizontalDirection::Next
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HorizontalArrowKey {
    Left,
    Right,
}

thread_local! {
    static DIRECTION_STACK: RefCell<Vec<TextDirection>> = const { RefCell::new(Vec::new()) };
}

pub fn current_direction() -> TextDirection {
    DIRECTION_STACK
        .with(|stack| stack.borrow().last().copied())
        .unwrap_or_default()
}

fn with_direction<Output>(direction: TextDirection, f: impl FnOnce() -> Output) -> Output {
    struct DirectionGuard;

    impl Drop for DirectionGuard {
        fn drop(&mut self) {
            DIRECTION_STACK.with(|stack| {
                stack.borrow_mut().pop();
            });
        }
    }

    DIRECTION_STACK.with(|stack| {
        stack.borrow_mut().push(direction);
    });
    let _guard = DirectionGuard;

    f()
}

#[derive(IntoElement)]
pub struct DirectionProvider {
    direction: TextDirection,
    children: Vec<AnyElement>,
}

impl Default for DirectionProvider {
    fn default() -> Self {
        Self {
            direction: TextDirection::Ltr,
            children: Vec::new(),
        }
    }
}

impl RenderOnce for DirectionProvider {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut children = self.children;
        let inner = if children.len() == 1 {
            children.pop().expect("single direction child should exist")
        } else {
            div().children(children).into_any_element()
        };

        DirectionProviderElement {
            direction: self.direction,
            inner,
        }
    }
}

impl ParentElement for DirectionProvider {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl DirectionProvider {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn direction(mut self, direction: TextDirection) -> Self {
        self.direction = direction;
        self
    }
}

struct DirectionProviderElement {
    direction: TextDirection,
    inner: AnyElement,
}

impl IntoElement for DirectionProviderElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for DirectionProviderElement {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let layout_id = with_direction(self.direction, || self.inner.request_layout(window, cx));

        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        with_direction(self.direction, || {
            self.inner.prepaint(window, cx);
        });
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        with_direction(self.direction, || {
            self.inner.paint(window, cx);
        });
    }
}
