use crate::utils::PresenceState;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AccordionOrientation {
    Horizontal,
    #[default]
    Vertical,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AccordionRootStyleState<T: Clone + Eq + 'static> {
    pub values: Vec<T>,
    pub disabled: bool,
    pub multiple: bool,
    pub orientation: AccordionOrientation,
}

impl<T: Clone + Eq + 'static> AccordionRootStyleState<T> {
    pub fn new(
        values: Vec<T>,
        disabled: bool,
        multiple: bool,
        orientation: AccordionOrientation,
    ) -> Self {
        Self {
            values,
            disabled,
            multiple,
            orientation,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AccordionItemStyleState<T: Clone + Eq + 'static> {
    pub value: T,
    pub values: Vec<T>,
    pub open: bool,
    pub closed: bool,
    pub hidden: bool,
    pub disabled: bool,
    pub index: usize,
    pub orientation: AccordionOrientation,
}

impl<T: Clone + Eq + 'static> AccordionItemStyleState<T> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        value: T,
        values: Vec<T>,
        open: bool,
        disabled: bool,
        index: usize,
        orientation: AccordionOrientation,
    ) -> Self {
        Self {
            value,
            values,
            open,
            closed: !open,
            hidden: !open,
            disabled,
            index,
            orientation,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AccordionHeaderStyleState<T: Clone + Eq + 'static> {
    pub item: AccordionItemStyleState<T>,
}

impl<T: Clone + Eq + 'static> AccordionHeaderStyleState<T> {
    pub fn new(item: AccordionItemStyleState<T>) -> Self {
        Self { item }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AccordionTriggerStyleState<T: Clone + Eq + 'static> {
    pub item: AccordionItemStyleState<T>,
    pub panel_open: bool,
    pub focused: bool,
}

impl<T: Clone + Eq + 'static> AccordionTriggerStyleState<T> {
    pub fn new(item: AccordionItemStyleState<T>, focused: bool) -> Self {
        Self {
            panel_open: item.open,
            item,
            focused,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AccordionPanelStyleState<T: Clone + Eq + 'static> {
    pub item: AccordionItemStyleState<T>,
    pub mounted: bool,
    pub present: bool,
    pub transitioning: bool,
}

impl<T: Clone + Eq + 'static> AccordionPanelStyleState<T> {
    pub fn new(item: AccordionItemStyleState<T>, keep_mounted: bool) -> Self {
        let presence = PresenceState::new(item.open, keep_mounted);

        Self {
            item,
            mounted: presence.present,
            present: presence.present,
            transitioning: presence.transitioning,
        }
    }
}
