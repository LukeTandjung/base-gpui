use gpui::KeyDownEvent;

pub fn typeahead_text(event: &KeyDownEvent) -> Option<String> {
    if event.keystroke.modifiers.control
        || event.keystroke.modifiers.alt
        || event.keystroke.modifiers.platform
        || event.keystroke.modifiers.function
    {
        return None;
    }

    event
        .keystroke
        .key_char
        .as_ref()
        .or(Some(&event.keystroke.key))
        .filter(|text| text.chars().count() == 1)
        .filter(|text| text.chars().all(|ch| !ch.is_control()))
        .cloned()
}
