use gpui::{
    fill, point, px, relative, size, App, Bounds, Element, ElementId, ElementInputHandler,
    GlobalElementId, InspectorElementId, IntoElement, LayoutId, PaintQuad, Pixels, ShapedLine,
    SharedString, Style, TextAlign, TextRun, Window,
};

use crate::otp_field::OTPFieldContext;

/// Component-local custom element for one OTP slot. Paints the slot's character
/// (or mask character), a caret in the active slot while the group is focused,
/// records the slot's bounds, and — for the active slot only — installs the one
/// platform input handler for the whole OTP field and anchors the IME candidate
/// window to this slot's bounds.
pub struct OtpSlotElement {
    context: OTPFieldContext,
    index: usize,
    character: SharedString,
    masked: bool,
    active: bool,
    disabled: bool,
}

impl OtpSlotElement {
    pub fn new(
        context: OTPFieldContext,
        index: usize,
        character: SharedString,
        masked: bool,
        active: bool,
        disabled: bool,
    ) -> Self {
        Self {
            context,
            index,
            character,
            masked,
            active,
            disabled,
        }
    }
}

pub struct OtpSlotPrepaintState {
    line: Option<ShapedLine>,
    cursor: Option<PaintQuad>,
}

impl IntoElement for OtpSlotElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for OtpSlotElement {
    type RequestLayoutState = ();
    type PrepaintState = OtpSlotPrepaintState;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = relative(1.0).into();
        style.size.height = relative(1.0).into();

        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
        let style = window.text_style();
        let display: SharedString = if self.character.is_empty() {
            SharedString::default()
        } else if self.masked {
            SharedString::from("•")
        } else {
            self.character.clone()
        };
        let text_color = if self.disabled {
            style.color.opacity(0.5)
        } else {
            style.color
        };

        let line = if display.is_empty() {
            None
        } else {
            let run = TextRun {
                len: display.len(),
                font: style.font(),
                color: text_color,
                background_color: None,
                underline: None,
                strikethrough: None,
            };
            let font_size = style.font_size.to_pixels(window.rem_size());
            Some(
                window
                    .text_system()
                    .shape_line(display, font_size, &[run], None),
            )
        };

        let focused = self.context.focus_handle().is_focused(window);
        let cursor = if self.active && focused && line.is_none() {
            Some(fill(
                Bounds::new(
                    point(
                        bounds.center().x,
                        bounds.top() + (bounds.size.height - window.line_height()) / 2.0,
                    ),
                    size(px(1.0), window.line_height()),
                ),
                text_color,
            ))
        } else if self.active && focused {
            let width = line.as_ref().map(|line| line.width).unwrap_or(px(0.0));
            Some(fill(
                Bounds::new(
                    point(
                        bounds.center().x + width / 2.0 + px(1.0),
                        bounds.top() + (bounds.size.height - window.line_height()) / 2.0,
                    ),
                    size(px(1.0), window.line_height()),
                ),
                text_color,
            ))
        } else {
            None
        };

        OtpSlotPrepaintState { line, cursor }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.context.focus_handle();

        if self.active {
            // Exactly one platform input handler per OTP field: only the active
            // slot installs it, gated on the shared group focus handle.
            window.handle_input(
                &focus_handle,
                ElementInputHandler::new(bounds, self.context.input_runtime()),
                cx,
            );
            self.context.input_runtime().update(cx, |input, cx| {
                input.set_ime_candidate_bounds(Some(bounds), cx);
            });
        }

        self.context.record_slot_bounds(self.index, bounds, cx);

        if let Some(line) = prepaint.line.take() {
            let origin = point(
                bounds.left() + (bounds.size.width - line.width) / 2.0,
                bounds.top() + (bounds.size.height - window.line_height()) / 2.0,
            );
            line.paint(
                origin,
                window.line_height(),
                TextAlign::Left,
                None,
                window,
                cx,
            )
            .expect("otp slot text should paint");
        }

        if focus_handle.is_focused(window) {
            if let Some(cursor) = prepaint.cursor.take() {
                window.paint_quad(cursor);
            }
        }
    }
}
