use std::rc::Rc;
use std::time::Duration;

use gpui::{
    div, AnyElement, App, Div, Empty, IntoElement, ParentElement, RenderOnce, StyleRefinement,
    Styled, Window,
};

use crate::avatar::{child_wiring::AvatarChildNode, AvatarContext, AvatarFallbackStyleState};

/// Fallback content shown while the avatar image is not loaded.
///
/// Accessibility: this part carries no role or aria props — it is a visual
/// substitute for the labeled root node. When `AvatarRoot` has an
/// `.aria_label(...)`, build visible initials with
/// `Text::new_inaccessible(...)` instead of `text!(...)`, so the initials are
/// not announced alongside the root's label. Children arrive as
/// `AnyElement`s, so this is a caller convention the layer cannot enforce.
#[derive(IntoElement)]
pub struct AvatarFallback {
    base: Div,
    children: Vec<AnyElement>,
    delay: Option<Duration>,
    context: Option<AvatarContext>,
    style_with_state: Option<Rc<dyn Fn(AvatarFallbackStyleState, Div) -> Div + 'static>>,
}

impl Default for AvatarFallback {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            delay: None,
            context: None,
            style_with_state: None,
        }
    }
}

impl ParentElement for AvatarFallback {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for AvatarFallback {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for AvatarFallback {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context else {
            return self.base.children(self.children).into_any_element();
        };

        if let Some(delay) = context.update(cx, |runtime| runtime.arm_fallback_delay(self.delay)) {
            let timer_context = context.clone();
            window
                .spawn(cx, async move |cx| {
                    cx.background_executor().timer(delay).await;
                    cx.update(|_window, cx| {
                        timer_context.update(cx, |runtime| runtime.fallback_delay_elapsed());
                    })
                    .ok();
                })
                .detach();
        }

        let (visible, state) = context.read(cx, |runtime| {
            (runtime.fallback_visible(), runtime.fallback_state())
        });

        if !visible {
            return Empty.into_any_element();
        }

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.children(self.children).into_any_element()
    }
}

impl AvatarChildNode for AvatarFallback {
    fn with_avatar_context(mut self, context: AvatarContext) -> Self {
        self.context = Some(context);
        self
    }
}

impl AvatarFallback {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = Some(delay);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(AvatarFallbackStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
