use crate::combobox::tests::support::single_runtime;
use crate::combobox::{ComboboxAutoHighlight, ComboboxMove};

#[test]
fn move_next_walks_filtered_items_and_loops_through_input_position() {
    let mut runtime = single_runtime(None, "", true);
    runtime.commit_input_value("b".into());
    // Matches: banana (1), blueberry (3).
    assert_eq!(runtime.filtered_indices(), vec![1, 3]);
    runtime.commit_open(true);

    runtime.move_highlight(ComboboxMove::Next, true);
    assert_eq!(runtime.highlighted_index(), Some(1));
    runtime.move_highlight(ComboboxMove::Next, true);
    assert_eq!(runtime.highlighted_index(), Some(3));
    // Loop passes through the input position (no highlight) between ends.
    runtime.move_highlight(ComboboxMove::Next, true);
    assert_eq!(runtime.highlighted_index(), None);
    runtime.move_highlight(ComboboxMove::Next, true);
    assert_eq!(runtime.highlighted_index(), Some(1));
}

#[test]
fn move_previous_from_input_position_goes_to_last() {
    let mut runtime = single_runtime(None, "", true);
    runtime.move_highlight(ComboboxMove::Previous, true);
    assert_eq!(runtime.highlighted_index(), Some(3));
}

#[test]
fn without_loop_navigation_clamps_at_the_ends() {
    let mut runtime = single_runtime(None, "", true);
    runtime.move_highlight(ComboboxMove::Last, false);
    assert_eq!(runtime.highlighted_index(), Some(3));
    runtime.move_highlight(ComboboxMove::Next, false);
    assert_eq!(runtime.highlighted_index(), Some(3));

    runtime.move_highlight(ComboboxMove::First, false);
    assert_eq!(runtime.highlighted_index(), Some(0));
    runtime.move_highlight(ComboboxMove::Previous, false);
    assert_eq!(runtime.highlighted_index(), Some(0));
}

#[test]
fn disabled_items_remain_highlightable_by_keyboard() {
    let mut runtime = single_runtime(None, "", true);
    runtime.move_highlight(ComboboxMove::Next, true);
    runtime.move_highlight(ComboboxMove::Next, true);
    runtime.move_highlight(ComboboxMove::Next, true);
    // Index 2 (cherry) is disabled but still roved over, matching Base UI.
    assert_eq!(runtime.highlighted_index(), Some(2));
}

#[test]
fn auto_highlight_off_clears_highlight_on_typing() {
    let mut runtime = single_runtime(None, "", true);
    runtime.move_highlight(ComboboxMove::First, true);
    runtime.commit_input_value("b".into());
    assert_eq!(runtime.highlighted_index(), None);
}

#[test]
fn auto_highlight_on_input_change_highlights_first_match() {
    let mut runtime = single_runtime(None, "", true);
    runtime.sync_filtering(
        None,
        false,
        None,
        None,
        ComboboxAutoHighlight::OnInputChange,
    );
    runtime.commit_input_value("b".into());
    assert_eq!(runtime.highlighted_index(), Some(1));
}

#[test]
fn auto_highlight_always_highlights_first_item_on_open() {
    let mut runtime = single_runtime(None, "", false);
    runtime.sync_filtering(None, false, None, None, ComboboxAutoHighlight::Always);
    runtime.commit_open(true);
    assert_eq!(runtime.highlighted_index(), Some(0));
}

#[test]
fn refilter_preserves_surviving_highlight_and_clears_lost_one() {
    let mut runtime = single_runtime(None, "", true);
    runtime.commit_input_value("b".into());
    runtime.move_highlight(ComboboxMove::Next, true);
    assert_eq!(runtime.highlighted_index(), Some(1));

    // banana survives the narrower query.
    runtime.commit_input_value("ban".into());
    // Off mode clears on typing; use highlight_item to emulate hover then refilter.
    runtime.highlight_item(Some(1), false);
    runtime.commit_input_value("banan".into());
    runtime.highlight_item(Some(1), false);
    runtime.refilter();
    assert_eq!(runtime.highlighted_index(), Some(1));

    // banana does not survive "blue"; highlight clears.
    runtime.sync_input_value_from_context("blue".into());
    assert_eq!(runtime.highlighted_index(), None);
}

#[test]
fn hover_highlight_respects_disabled_items() {
    let mut runtime = single_runtime(None, "", true);
    runtime.highlight_item(Some(2), true);
    assert_eq!(runtime.highlighted_index(), None);
    runtime.highlight_item(Some(1), false);
    assert_eq!(runtime.highlighted_index(), Some(1));
}

#[test]
fn pointer_leave_clears_highlight_unless_kept() {
    let mut runtime = single_runtime(None, "", true);
    runtime.highlight_item(Some(1), false);
    runtime.clear_highlight_unless_kept(true);
    assert_eq!(runtime.highlighted_index(), Some(1));
    runtime.clear_highlight_unless_kept(false);
    assert_eq!(runtime.highlighted_index(), None);
}

#[test]
fn highlight_transition_is_reported_exactly_once() {
    let mut runtime = single_runtime(None, "", true);
    assert_eq!(runtime.take_highlight_transition(), None);

    runtime.move_highlight(ComboboxMove::First, true);
    assert_eq!(runtime.take_highlight_transition(), Some(Some(0)));
    // Same highlight: no duplicate callback.
    assert_eq!(runtime.take_highlight_transition(), None);

    runtime.clear_highlight_unless_kept(false);
    assert_eq!(runtime.take_highlight_transition(), Some(None));
    assert_eq!(runtime.take_highlight_transition(), None);
}
