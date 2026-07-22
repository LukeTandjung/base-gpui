use crate::autocomplete::layers::resolve_mode;
use crate::autocomplete::AutocompleteMode;

#[test]
fn list_mode_filters_without_inline_autocompletion() {
    assert_eq!(resolve_mode(AutocompleteMode::List), (false, false));
}

#[test]
fn both_mode_filters_with_inline_autocompletion() {
    assert_eq!(resolve_mode(AutocompleteMode::Both), (false, true));
}

#[test]
fn inline_mode_is_static_with_inline_autocompletion() {
    assert_eq!(resolve_mode(AutocompleteMode::Inline), (true, true));
}

#[test]
fn none_mode_is_static_without_inline_autocompletion() {
    assert_eq!(resolve_mode(AutocompleteMode::None), (true, false));
}

#[test]
fn default_mode_is_list() {
    assert_eq!(AutocompleteMode::default(), AutocompleteMode::List);
}
