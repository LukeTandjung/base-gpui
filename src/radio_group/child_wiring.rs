use gpui::{App, FocusHandle, Window};

use crate::radio_group::{
    RadioGroupChild, RadioGroupContext, RadioGroupRadioChild, RadioGroupRadioMetadata,
    RadioGroupRadioStyleState,
};

pub struct WiredRadioGroupChildren<T: Clone + Eq + 'static> {
    pub radios: Vec<RadioGroupRadioMetadata<T>>,
    pub radio_focus_handles: Vec<(usize, FocusHandle)>,
    pub focused_radio_index: Option<usize>,
    pub children: Vec<RadioGroupChild<T>>,
}

pub trait RadioGroupChildNode<T: Clone + Eq + 'static>: Sized {
    fn with_radio_group_context(self, context: RadioGroupContext<T>) -> Self;

    fn wire_radio_group_child(
        self,
        _wiring: &mut RadioGroupChildWiring<T>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        self
    }
}

pub trait RadioGroupRadioChildNode: Sized {
    fn with_radio_state(self, state: RadioGroupRadioStyleState) -> Self;
}

pub struct RadioGroupChildWiring<T: Clone + Eq + 'static> {
    next_radio_index: usize,
    radios: Vec<RadioGroupRadioMetadata<T>>,
    radio_focus_handles: Vec<(usize, FocusHandle)>,
    focused_radio_index: Option<usize>,
}

impl<T: Clone + Eq + 'static> RadioGroupChildWiring<T> {
    pub fn new() -> Self {
        Self {
            next_radio_index: 0,
            radios: Vec::new(),
            radio_focus_handles: Vec::new(),
            focused_radio_index: None,
        }
    }

    pub fn register_radio(
        &mut self,
        value: Option<T>,
        disabled: bool,
        read_only: bool,
        required: bool,
        focus_handle: FocusHandle,
        focused: bool,
    ) -> usize {
        let index = self.next_radio_index;
        self.next_radio_index += 1;

        if let Some(value) = value {
            self.radios.push(RadioGroupRadioMetadata::new(
                value, disabled, read_only, required, index,
            ));
        }

        if focused {
            self.focused_radio_index = Some(index);
        }

        self.radio_focus_handles.push((index, focus_handle));
        index
    }

    fn finish(self, children: Vec<RadioGroupChild<T>>) -> WiredRadioGroupChildren<T> {
        WiredRadioGroupChildren {
            radios: self.radios,
            radio_focus_handles: self.radio_focus_handles,
            focused_radio_index: self.focused_radio_index,
            children,
        }
    }
}

pub fn wire_children<T: Clone + Eq + 'static>(
    children: Vec<RadioGroupChild<T>>,
    context: RadioGroupContext<T>,
    window: &mut Window,
    cx: &mut App,
) -> WiredRadioGroupChildren<T> {
    let mut wiring = RadioGroupChildWiring::new();
    let children = children
        .into_iter()
        .map(|child| child.wire_radio_group_child(&mut wiring, window, cx))
        .map(|child| child.with_radio_group_context(context.clone()))
        .collect();

    wiring.finish(children)
}

impl<T: Clone + Eq + 'static> RadioGroupChildNode<T> for RadioGroupChild<T> {
    fn with_radio_group_context(self, context: RadioGroupContext<T>) -> Self {
        match self {
            Self::Radio(radio) => Self::Radio(radio.with_radio_group_context(context)),
            Self::Indicator(indicator) => Self::Indicator(indicator),
        }
    }

    fn wire_radio_group_child(
        self,
        wiring: &mut RadioGroupChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Radio(radio) => Self::Radio(radio.wire_radio_group_child(wiring, window, cx)),
            Self::Indicator(indicator) => Self::Indicator(indicator),
        }
    }
}

impl RadioGroupRadioChildNode for RadioGroupRadioChild {
    fn with_radio_state(self, state: RadioGroupRadioStyleState) -> Self {
        match self {
            Self::Indicator(indicator) => Self::Indicator(indicator.with_radio_state(state)),
        }
    }
}
