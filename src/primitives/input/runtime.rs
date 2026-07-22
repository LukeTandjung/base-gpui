use std::ops::Range;

use gpui::{
    ClipboardItem, Context, EntityInputHandler, FocusHandle, MouseDownEvent, MouseMoveEvent,
    MouseUpEvent, Point, ShapedLine, SharedString, Subscription, UTF16Selection, Window,
};
use unicode_segmentation::UnicodeSegmentation;

use crate::primitives::input::{
    InputBackspace, InputBoundaryHandler, InputCopy, InputCut, InputDelete, InputEnd, InputEnter,
    InputEnterHandler, InputHome, InputLeft, InputPaste, InputPasteHandler, InputRight,
    InputSelectAll, InputSelectLeft, InputSelectRight, InputStyleState, InputValueChangeHandler,
};

pub struct InputRuntime {
    focus_handle: FocusHandle,
    value: SharedString,
    initial_value: SharedString,
    selected_range: Range<usize>,
    selection_reversed: bool,
    marked_range: Option<Range<usize>>,
    last_layout: Option<ShapedLine>,
    last_bounds: Option<gpui::Bounds<gpui::Pixels>>,
    last_scroll_offset: gpui::Pixels,
    ime_candidate_bounds: Option<gpui::Bounds<gpui::Pixels>>,
    selecting: bool,
    disabled: bool,
    read_only: bool,
    required: bool,
    controlled: bool,
    on_value_change: Option<InputValueChangeHandler>,
    on_enter: Option<InputEnterHandler>,
    on_home: Option<InputBoundaryHandler>,
    on_end: Option<InputBoundaryHandler>,
    on_paste: Option<InputPasteHandler>,
    on_edge_left: Option<InputBoundaryHandler>,
    on_edge_right: Option<InputBoundaryHandler>,
    on_backspace: Option<InputBoundaryHandler>,
    on_delete: Option<InputBoundaryHandler>,
    select_all_on_focus: bool,
    was_focused: bool,
    _subscriptions: Vec<Subscription>,
}

impl InputRuntime {
    pub fn new(value: SharedString, window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self::new_with_focus_handle(value, cx.focus_handle(), window, cx)
    }

    pub fn new_with_focus_handle(
        value: SharedString,
        focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscriptions = vec![
            cx.on_focus(&focus_handle, window, Self::on_focus),
            cx.on_blur(&focus_handle, window, Self::on_blur),
        ];
        let cursor = value.len();

        Self {
            focus_handle,
            value: value.clone(),
            initial_value: value,
            selected_range: cursor..cursor,
            selection_reversed: false,
            marked_range: None,
            last_layout: None,
            last_bounds: None,
            last_scroll_offset: gpui::px(0.0),
            ime_candidate_bounds: None,
            selecting: false,
            disabled: false,
            read_only: false,
            required: false,
            controlled: false,
            on_value_change: None,
            on_enter: None,
            on_home: None,
            on_end: None,
            on_paste: None,
            on_edge_left: None,
            on_edge_right: None,
            on_backspace: None,
            on_delete: None,
            select_all_on_focus: false,
            was_focused: false,
            _subscriptions: subscriptions,
        }
    }

    /// Syncs composite-container integration hooks: caret-edge boundary
    /// handlers consulted when a plain arrow press sits at the start/end of
    /// the text with no selection, and select-all-on-focus behavior used when
    /// roving focus enters the input. Containers such as the Toolbar call
    /// this every render; standalone inputs keep the defaults.
    pub fn sync_composite(
        &mut self,
        on_edge_left: Option<InputBoundaryHandler>,
        on_edge_right: Option<InputBoundaryHandler>,
        select_all_on_focus: bool,
    ) {
        self.on_edge_left = on_edge_left;
        self.on_edge_right = on_edge_right;
        self.select_all_on_focus = select_all_on_focus;
    }

    /// Observes the focus fact each render; when focus newly arrives and
    /// `select_all_on_focus` is set, the whole text is selected, matching
    /// composite-container focus behavior.
    pub fn sync_focus_observed(&mut self, focused: bool, cx: &mut Context<Self>) {
        if focused && !self.was_focused && self.select_all_on_focus {
            self.selected_range = 0..self.value.len();
            self.selection_reversed = false;
            cx.notify();
        }

        self.was_focused = focused;
    }

    pub fn sync_props(
        &mut self,
        controlled_value: Option<SharedString>,
        disabled: bool,
        read_only: bool,
        required: bool,
        on_value_change: Option<InputValueChangeHandler>,
        on_enter: Option<InputEnterHandler>,
        on_home: Option<InputBoundaryHandler>,
        on_end: Option<InputBoundaryHandler>,
        on_paste: Option<InputPasteHandler>,
        on_backspace: Option<InputBoundaryHandler>,
        on_delete: Option<InputBoundaryHandler>,
        cx: &mut Context<Self>,
    ) {
        let mut changed = false;
        self.controlled = controlled_value.is_some();
        if let Some(value) = controlled_value {
            if self.value != value {
                self.value = value;
                self.clamp_selection_to_value();
                self.marked_range = None;
                changed = true;
            }
        }
        changed |= self.disabled != disabled;
        changed |= self.read_only != read_only;
        changed |= self.required != required;
        self.disabled = disabled;
        self.read_only = read_only;
        self.required = required;
        self.on_value_change = on_value_change;
        self.on_enter = on_enter;
        self.on_home = on_home;
        self.on_end = on_end;
        self.on_paste = on_paste;
        self.on_backspace = on_backspace;
        self.on_delete = on_delete;

        if changed {
            cx.notify();
        }
    }

    pub fn focus_handle(&self) -> FocusHandle {
        self.focus_handle.clone()
    }

    pub fn value(&self) -> SharedString {
        self.value.clone()
    }

    pub fn is_focused(&self, window: &Window) -> bool {
        self.focus_handle.is_focused(window)
    }

    pub fn style_state(&self, window: &Window, valid: Option<bool>) -> InputStyleState {
        InputStyleState::new(
            self.value.clone(),
            self.disabled,
            self.read_only,
            self.required,
            self.focus_handle.is_focused(window),
            self.dirty(),
            self.controlled,
            valid,
        )
    }

    pub fn dirty(&self) -> bool {
        self.value != self.initial_value
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn read_only(&self) -> bool {
        self.read_only
    }

    pub fn required(&self) -> bool {
        self.required
    }

    pub fn selected_range(&self) -> Range<usize> {
        self.selected_range.clone()
    }

    pub fn marked_range(&self) -> Option<Range<usize>> {
        self.marked_range.clone()
    }

    pub fn cursor_offset(&self) -> usize {
        if self.selection_reversed {
            self.selected_range.start
        } else {
            self.selected_range.end
        }
    }

    pub fn set_last_layout(
        &mut self,
        layout: ShapedLine,
        bounds: gpui::Bounds<gpui::Pixels>,
        scroll_offset: gpui::Pixels,
        _cx: &mut Context<Self>,
    ) {
        self.last_layout = Some(layout);
        self.last_bounds = Some(bounds);
        self.last_scroll_offset = scroll_offset;
    }

    /// Overrides the IME candidate-window anchor used by `bounds_for_range`.
    ///
    /// Segmented displays (e.g. OTP fields) have no single shaped line; they anchor
    /// the IME candidate window to their active segment's bounds instead.
    pub fn set_ime_candidate_bounds(
        &mut self,
        bounds: Option<gpui::Bounds<gpui::Pixels>>,
        _cx: &mut Context<Self>,
    ) {
        self.ime_candidate_bounds = bounds;
    }

    pub fn backspace(&mut self, _: &InputBackspace, window: &mut Window, cx: &mut Context<Self>) {
        if !self.can_edit() {
            return;
        }
        // Composite containers (Combobox chips) may consume the press before
        // any text edit happens, e.g. to remove a highlighted chip or the
        // last selected value when the input is empty.
        if let Some(on_backspace) = self.on_backspace.clone() {
            if on_backspace(self.value.clone(), window, cx) {
                return;
            }
        }

        if self.selected_range.is_empty() {
            let previous = self.previous_boundary(self.cursor_offset());
            if previous == self.cursor_offset() {
                window.play_system_bell();
                return;
            }
            self.select_to(previous, cx);
        }
        self.replace_text_in_range(None, "", window, cx);
    }

    pub fn delete(&mut self, _: &InputDelete, window: &mut Window, cx: &mut Context<Self>) {
        if !self.can_edit() {
            return;
        }
        if let Some(on_delete) = self.on_delete.clone() {
            if on_delete(self.value.clone(), window, cx) {
                return;
            }
        }

        if self.selected_range.is_empty() {
            let next = self.next_boundary(self.cursor_offset());
            if next == self.cursor_offset() {
                window.play_system_bell();
                return;
            }
            self.select_to(next, cx);
        }
        self.replace_text_in_range(None, "", window, cx);
    }

    pub fn left(&mut self, _: &InputLeft, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            if self.cursor_offset() == 0 {
                if let Some(on_edge_left) = self.on_edge_left.clone() {
                    if on_edge_left(self.value.clone(), window, cx) {
                        return;
                    }
                }
            }

            self.move_to(self.previous_boundary(self.cursor_offset()), cx);
        } else {
            self.move_to(self.selected_range.start, cx);
        }
    }

    pub fn right(&mut self, _: &InputRight, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            if self.cursor_offset() == self.value.len() {
                if let Some(on_edge_right) = self.on_edge_right.clone() {
                    if on_edge_right(self.value.clone(), window, cx) {
                        return;
                    }
                }
            }

            self.move_to(self.next_boundary(self.cursor_offset()), cx);
        } else {
            self.move_to(self.selected_range.end, cx);
        }
    }

    pub fn select_left(&mut self, _: &InputSelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.previous_boundary(self.cursor_offset()), cx);
    }

    pub fn select_right(&mut self, _: &InputSelectRight, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.next_boundary(self.cursor_offset()), cx);
    }

    pub fn select_all(&mut self, _: &InputSelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.selected_range = 0..self.value.len();
        self.selection_reversed = false;
        cx.notify();
    }

    pub fn home(&mut self, _: &InputHome, window: &mut Window, cx: &mut Context<Self>) {
        if self
            .on_home
            .as_ref()
            .is_some_and(|on_home| on_home(self.value.clone(), window, cx))
        {
            return;
        }

        self.move_to(0, cx);
    }

    pub fn end(&mut self, _: &InputEnd, window: &mut Window, cx: &mut Context<Self>) {
        if self
            .on_end
            .as_ref()
            .is_some_and(|on_end| on_end(self.value.clone(), window, cx))
        {
            return;
        }

        self.move_to(self.value.len(), cx);
    }

    pub fn copy(&mut self, _: &InputCopy, _: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.value[self.selected_range.clone()].to_string(),
            ));
        }
    }

    pub fn cut(&mut self, _: &InputCut, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            return;
        }
        cx.write_to_clipboard(ClipboardItem::new_string(
            self.value[self.selected_range.clone()].to_string(),
        ));
        if self.can_edit() {
            self.replace_text_in_range(None, "", window, cx);
        }
    }

    pub fn paste(&mut self, _: &InputPaste, window: &mut Window, cx: &mut Context<Self>) {
        if !self.can_edit() {
            return;
        }
        if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
            if self
                .on_paste
                .clone()
                .is_some_and(|on_paste| on_paste(SharedString::from(text.clone()), window, cx))
            {
                return;
            }
            self.replace_text_in_range(None, &normalize_single_line(&text), window, cx);
        }
    }

    pub fn enter(&mut self, _: &InputEnter, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(on_enter) = self.on_enter.clone() {
            on_enter(self.value.clone(), window, cx);
        }
    }

    pub fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.disabled {
            return;
        }

        self.focus_handle.focus(window, cx);
        self.selecting = true;
        let offset = self.index_for_mouse_position(event.position);
        if event.modifiers.shift {
            self.select_to(offset, cx);
        } else {
            self.move_to(offset, cx);
        }
    }

    pub fn on_mouse_up(&mut self, _: &MouseUpEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.selecting = false;
        cx.notify();
    }

    pub fn on_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.selecting {
            self.select_to(self.index_for_mouse_position(event.position), cx);
        }
    }

    fn on_focus(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        if self.select_all_on_focus {
            self.selected_range = 0..self.value.len();
            self.selection_reversed = false;
        }

        cx.notify();
    }

    fn on_blur(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.selecting = false;
        cx.notify();
    }

    fn can_edit(&self) -> bool {
        !self.disabled && !self.read_only
    }

    fn move_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        let offset = self.clamp_offset(offset);
        self.selected_range = offset..offset;
        self.selection_reversed = false;
        cx.notify();
    }

    fn select_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        let offset = self.clamp_offset(offset);
        if self.selection_reversed {
            self.selected_range.start = offset;
        } else {
            self.selected_range.end = offset;
        }
        if self.selected_range.end < self.selected_range.start {
            self.selection_reversed = !self.selection_reversed;
            self.selected_range = self.selected_range.end..self.selected_range.start;
        }
        cx.notify();
    }

    fn index_for_mouse_position(&self, position: Point<gpui::Pixels>) -> usize {
        if self.value.is_empty() {
            return 0;
        }

        let (Some(bounds), Some(line)) = (self.last_bounds.as_ref(), self.last_layout.as_ref())
        else {
            return self.value.len();
        };

        if position.x <= bounds.left() {
            return 0;
        }
        if position.x >= bounds.right() && self.last_scroll_offset == gpui::px(0.0) {
            return self.value.len();
        }
        line.closest_index_for_x(position.x - bounds.left() - self.last_scroll_offset)
    }

    fn previous_boundary(&self, offset: usize) -> usize {
        self.value
            .grapheme_indices(true)
            .rev()
            .find_map(|(index, _)| (index < offset).then_some(index))
            .unwrap_or(0)
    }

    fn next_boundary(&self, offset: usize) -> usize {
        self.value
            .grapheme_indices(true)
            .find_map(|(index, _)| (index > offset).then_some(index))
            .unwrap_or(self.value.len())
    }

    fn offset_from_utf16(&self, offset: usize) -> usize {
        let mut utf8_offset = 0;
        let mut utf16_count = 0;

        for ch in self.value.chars() {
            if utf16_count >= offset {
                break;
            }
            utf16_count += ch.len_utf16();
            utf8_offset += ch.len_utf8();
        }

        utf8_offset
    }

    fn offset_to_utf16(&self, offset: usize) -> usize {
        let mut utf16_offset = 0;
        let mut utf8_count = 0;

        for ch in self.value.chars() {
            if utf8_count >= offset {
                break;
            }
            utf8_count += ch.len_utf8();
            utf16_offset += ch.len_utf16();
        }

        utf16_offset
    }

    fn range_from_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range.start)..self.offset_from_utf16(range.end)
    }

    fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
    }

    fn clamp_offset(&self, offset: usize) -> usize {
        let mut offset = offset.min(self.value.len());
        while !self.value.is_char_boundary(offset) {
            offset -= 1;
        }
        offset
    }

    fn clamp_selection_to_value(&mut self) {
        let start = self.clamp_offset(self.selected_range.start);
        let end = self.clamp_offset(self.selected_range.end);
        self.selected_range = start.min(end)..start.max(end);
    }

    fn replacement_range(&self, range_utf16: Option<Range<usize>>) -> Range<usize> {
        range_utf16
            .as_ref()
            .map(|range| self.range_from_utf16(range))
            .or_else(|| self.marked_range.clone())
            .unwrap_or_else(|| self.selected_range.clone())
    }

    fn replace_selected_text(
        &mut self,
        range: Range<usize>,
        new_text: &str,
        mark: Option<Option<Range<usize>>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let new_text = normalize_single_line(new_text);
        let mut next = String::new();
        next.push_str(&self.value[..range.start]);
        next.push_str(&new_text);
        next.push_str(&self.value[range.end..]);
        let next_value = SharedString::from(next);
        let next_offset = range.start + new_text.len();

        if let Some(on_value_change) = self.on_value_change.as_ref() {
            on_value_change(next_value.clone(), window, cx);
        }

        self.value = next_value;
        match mark {
            Some(new_selected_range_utf16) => {
                if new_text.is_empty() {
                    self.marked_range = None;
                    self.selected_range = range.start..range.start;
                } else {
                    let marked_range = range.start..range.start + new_text.len();
                    self.marked_range = Some(marked_range.clone());
                    self.selected_range = new_selected_range_utf16
                        .as_ref()
                        .map(|range_utf16| self.range_from_utf16(range_utf16))
                        .map(|new_range| {
                            new_range.start + marked_range.start..new_range.end + marked_range.start
                        })
                        .unwrap_or_else(|| marked_range.end..marked_range.end);
                }
            }
            None => {
                self.marked_range = None;
                self.selected_range = next_offset..next_offset;
            }
        }
        self.selection_reversed = false;
        cx.notify();
    }
}

impl EntityInputHandler for InputRuntime {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        adjusted_range: &mut Option<Range<usize>>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<String> {
        let range = self.range_from_utf16(&range_utf16);
        adjusted_range.replace(self.range_to_utf16(&range));
        Some(self.value[range].to_string())
    }

    fn selected_text_range(
        &mut self,
        ignore_disabled_input: bool,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        if !ignore_disabled_input && self.disabled {
            return None;
        }

        Some(UTF16Selection {
            range: self.range_to_utf16(&self.selected_range),
            reversed: self.selection_reversed,
        })
    }

    fn marked_text_range(&self, _: &mut Window, _: &mut Context<Self>) -> Option<Range<usize>> {
        self.marked_range
            .as_ref()
            .map(|range| self.range_to_utf16(range))
    }

    fn unmark_text(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.marked_range = None;
        cx.notify();
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        text: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.can_edit() {
            return;
        }

        let range = self.replacement_range(range_utf16);
        self.replace_selected_text(range, text, None, window, cx);
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range: Option<Range<usize>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.can_edit() {
            return;
        }

        let range = self.replacement_range(range_utf16);
        self.replace_selected_text(range, new_text, Some(new_selected_range), window, cx);
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        element_bounds: gpui::Bounds<gpui::Pixels>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<gpui::Bounds<gpui::Pixels>> {
        if let Some(bounds) = self.ime_candidate_bounds {
            return Some(bounds);
        }

        let layout = self.last_layout.as_ref()?;
        let range = self.range_from_utf16(&range_utf16);

        Some(gpui::Bounds::from_corners(
            gpui::point(
                element_bounds.left() + layout.x_for_index(range.start),
                element_bounds.top(),
            ),
            gpui::point(
                element_bounds.left() + layout.x_for_index(range.end),
                element_bounds.bottom(),
            ),
        ))
    }

    fn character_index_for_point(
        &mut self,
        point: Point<gpui::Pixels>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<usize> {
        let bounds = self.last_bounds?;
        let layout = self.last_layout.as_ref()?;
        if !bounds.contains(&point) {
            return None;
        }

        let utf8_index = layout.index_for_x(point.x - bounds.left())?;
        Some(self.offset_to_utf16(utf8_index))
    }

    fn accepts_text_input(&self, _: &mut Window, _: &mut Context<Self>) -> bool {
        self.can_edit()
    }
}

fn normalize_single_line(text: &str) -> String {
    text.replace(['\n', '\r'], " ")
}

#[cfg(test)]
mod unit_tests {
    use gpui::{
        div, px, size, AppContext as _, Entity, IntoElement, Render, SharedString, TestAppContext,
    };

    use super::{normalize_single_line, EntityInputHandler, InputRuntime};

    struct RuntimeHarness {
        input: Entity<InputRuntime>,
    }

    impl Render for RuntimeHarness {
        fn render(
            &mut self,
            _window: &mut gpui::Window,
            _cx: &mut gpui::Context<Self>,
        ) -> impl IntoElement {
            div()
        }
    }

    #[test]
    fn normalizes_line_breaks() {
        assert_eq!(normalize_single_line("a\nb\rc"), "a b c");
    }

    #[gpui::test]
    fn utf16_ranges_replace_whole_emoji_without_splitting(cx: &mut TestAppContext) {
        let window = cx.open_window(size(px(120.0), px(40.0)), |window, cx| RuntimeHarness {
            input: cx.new(|cx| InputRuntime::new(SharedString::from("💝a"), window, cx)),
        });

        window
            .update(cx, |harness, window, cx| {
                harness.input.update(cx, |input, cx| {
                    EntityInputHandler::replace_text_in_range(input, Some(0..2), "x", window, cx);
                    assert_eq!(input.value(), SharedString::from("xa"));
                });
            })
            .expect("runtime test window should be open");
    }

    #[gpui::test]
    fn ime_marked_range_is_stored_and_reported(cx: &mut TestAppContext) {
        let window = cx.open_window(size(px(120.0), px(40.0)), |window, cx| RuntimeHarness {
            input: cx.new(|cx| InputRuntime::new(SharedString::from("a"), window, cx)),
        });

        window
            .update(cx, |harness, window, cx| {
                harness.input.update(cx, |input, cx| {
                    EntityInputHandler::replace_and_mark_text_in_range(
                        input,
                        None,
                        "é",
                        Some(0..1),
                        window,
                        cx,
                    );
                    assert_eq!(input.value(), SharedString::from("aé"));
                    assert_eq!(input.marked_range(), Some(1..3));
                    assert_eq!(
                        EntityInputHandler::marked_text_range(input, window, cx),
                        Some(1..2)
                    );
                });
            })
            .expect("runtime test window should be open");
    }
}
