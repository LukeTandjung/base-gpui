use gpui::{
    fill, hsla, point, px, rgba, size, App, Bounds, ContentMask, Element, ElementId,
    ElementInputHandler, Entity, GlobalElementId, InspectorElementId, IntoElement, LayoutId,
    PaintQuad, Pixels, ShapedLine, SharedString, Style, TextAlign, TextRun, UnderlineStyle, Window,
};

use crate::primitives::input::InputRuntime;

pub struct InputTextElement {
    state: Entity<InputRuntime>,
    placeholder: SharedString,
}

impl InputTextElement {
    pub fn new(state: Entity<InputRuntime>, placeholder: SharedString) -> Self {
        Self { state, placeholder }
    }
}

pub struct InputTextPrepaintState {
    line: Option<ShapedLine>,
    cursor: Option<PaintQuad>,
    selection: Option<PaintQuad>,
    scroll_offset: Pixels,
    #[cfg(test)]
    display_text: SharedString,
    #[cfg(test)]
    has_cursor: bool,
    #[cfg(test)]
    has_selection: bool,
}

impl IntoElement for InputTextElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for InputTextElement {
    type RequestLayoutState = ();
    type PrepaintState = InputTextPrepaintState;

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
        style.size.width = gpui::relative(1.0).into();
        style.size.height = window.line_height().into();

        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let input = self.state.read(cx);
        let value = input.value();
        let selected_range = input.selected_range();
        let marked_range = input.marked_range();
        let cursor = input.cursor_offset();
        let style = window.text_style();
        let disabled = input.disabled();

        let (display_text, text_color) = if value.is_empty() {
            (self.placeholder.clone(), hsla(0.0, 0.0, 0.0, 0.35))
        } else {
            (value.clone(), style.color)
        };
        let text_color = if disabled {
            text_color.opacity(0.5)
        } else {
            text_color
        };

        let run = TextRun {
            len: display_text.len(),
            font: style.font(),
            color: text_color,
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let runs = if !value.is_empty() {
            if let Some(marked_range) = marked_range.as_ref() {
                vec![
                    TextRun {
                        len: marked_range.start,
                        ..run.clone()
                    },
                    TextRun {
                        len: marked_range.end - marked_range.start,
                        underline: Some(UnderlineStyle {
                            color: Some(run.color),
                            thickness: px(1.0),
                            wavy: false,
                        }),
                        ..run.clone()
                    },
                    TextRun {
                        len: display_text.len() - marked_range.end,
                        ..run.clone()
                    },
                ]
                .into_iter()
                .filter(|run| run.len > 0)
                .collect()
            } else {
                vec![run]
            }
        } else {
            vec![run]
        };

        let font_size = style.font_size.to_pixels(window.rem_size());
        #[cfg(test)]
        let debug_display_text = display_text.clone();
        let line = window
            .text_system()
            .shape_line(display_text, font_size, &runs, None);
        let cursor_x = line.x_for_index(cursor);
        // Keep the cursor visible when the shaped line is wider than the
        // element: shift painting left so the cursor stays inside bounds.
        let visible_width = bounds.right() - bounds.left();
        let scroll_offset = if cursor_x + px(2.0) > visible_width {
            visible_width - cursor_x - px(2.0)
        } else {
            px(0.0)
        };
        let (selection, cursor) = if !value.is_empty() && !selected_range.is_empty() {
            (
                Some(fill(
                    Bounds::from_corners(
                        point(
                            bounds.left() + scroll_offset + line.x_for_index(selected_range.start),
                            bounds.top(),
                        ),
                        point(
                            bounds.left() + scroll_offset + line.x_for_index(selected_range.end),
                            bounds.bottom(),
                        ),
                    ),
                    rgba(0x335b9dff),
                )),
                None,
            )
        } else {
            (
                None,
                Some(fill(
                    Bounds::new(
                        point(bounds.left() + scroll_offset + cursor_x, bounds.top()),
                        size(px(1.0), bounds.bottom() - bounds.top()),
                    ),
                    style.color,
                )),
            )
        };

        #[cfg(test)]
        let has_cursor = cursor.is_some();
        #[cfg(test)]
        let has_selection = selection.is_some();

        InputTextPrepaintState {
            line: Some(line),
            cursor,
            selection,
            scroll_offset,
            #[cfg(test)]
            display_text: debug_display_text,
            #[cfg(test)]
            has_cursor,
            #[cfg(test)]
            has_selection,
        }
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
        let focus_handle = self.state.read(cx).focus_handle();
        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.state.clone()),
            cx,
        );

        let scroll_offset = prepaint.scroll_offset;
        let selection = prepaint.selection.take();
        let line = prepaint
            .line
            .take()
            .expect("input text should be shaped during prepaint");
        let cursor = prepaint.cursor.take();
        let line_for_paint = line.clone();

        // Clip painting to the element so overflowing text never draws over
        // adjacent siblings (clear buttons, icons, group borders).
        window.with_content_mask(Some(ContentMask { bounds }), |window| {
            if let Some(selection) = selection {
                window.paint_quad(selection);
            }

            line_for_paint
                .paint(
                    bounds.origin + point(scroll_offset, px(0.0)),
                    window.line_height(),
                    TextAlign::Left,
                    None,
                    window,
                    cx,
                )
                .expect("input text should paint");

            if focus_handle.is_focused(window) {
                if let Some(cursor) = cursor {
                    window.paint_quad(cursor);
                }
            }
        });

        self.state.update(cx, |input, cx| {
            input.set_last_layout(line, bounds, scroll_offset, cx);
        });
    }
}

#[cfg(test)]
mod tests {
    use gpui::{
        div, point, px, size, AppContext as _, Entity, IntoElement, Render, SharedString,
        TestAppContext, VisualTestContext,
    };

    use crate::primitives::input::{InputRuntime, InputSelectAll};

    use super::InputTextElement;

    struct TextElementHarness {
        input: Entity<InputRuntime>,
    }

    impl Render for TextElementHarness {
        fn render(
            &mut self,
            _window: &mut gpui::Window,
            _cx: &mut gpui::Context<Self>,
        ) -> impl IntoElement {
            div()
        }
    }

    fn open_harness(
        cx: &mut TestAppContext,
        value: SharedString,
    ) -> gpui::WindowHandle<TextElementHarness> {
        cx.open_window(size(px(180.0), px(48.0)), |window, cx| TextElementHarness {
            input: cx.new(|cx| InputRuntime::new(value, window, cx)),
        })
    }

    #[gpui::test]
    fn placeholder_text_is_shaped_when_value_is_empty(cx: &mut TestAppContext) {
        let window = open_harness(cx, SharedString::default());
        let input = window
            .update(cx, |harness, _window, _cx| harness.input.clone())
            .expect("text element test window should be open");
        let mut visual = VisualTestContext::from_window(window.into(), cx);

        let (_, prepaint) = visual.draw(
            point(px(0.0), px(0.0)),
            size(px(160.0), px(24.0)),
            |_, _| InputTextElement::new(input.clone(), SharedString::from("Placeholder")),
        );

        assert_eq!(prepaint.display_text, SharedString::from("Placeholder"));
    }

    #[gpui::test]
    fn cursor_is_prepared_when_selection_is_collapsed(cx: &mut TestAppContext) {
        let window = open_harness(cx, SharedString::from("abc"));
        let input = window
            .update(cx, |harness, window, cx| {
                harness.input.read(cx).focus_handle().focus(window, cx);
                harness.input.clone()
            })
            .expect("text element test window should be open");
        let mut visual = VisualTestContext::from_window(window.into(), cx);

        let (_, prepaint) = visual.draw(
            point(px(0.0), px(0.0)),
            size(px(160.0), px(24.0)),
            |_, _| InputTextElement::new(input.clone(), SharedString::default()),
        );

        assert!(prepaint.has_cursor);
        assert!(!prepaint.has_selection);
    }

    #[gpui::test]
    fn selection_highlight_is_prepared_when_selection_is_non_empty(cx: &mut TestAppContext) {
        let window = open_harness(cx, SharedString::from("abc"));
        let input = window
            .update(cx, |harness, window, cx| {
                harness.input.update(cx, |input, cx| {
                    input.select_all(&InputSelectAll, window, cx);
                });
                harness.input.clone()
            })
            .expect("text element test window should be open");
        let mut visual = VisualTestContext::from_window(window.into(), cx);

        let (_, prepaint) = visual.draw(
            point(px(0.0), px(0.0)),
            size(px(160.0), px(24.0)),
            |_, _| InputTextElement::new(input.clone(), SharedString::default()),
        );

        assert!(prepaint.has_selection);
        assert!(!prepaint.has_cursor);
    }
}
