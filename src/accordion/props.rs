use std::rc::Rc;

use gpui::{App, Window};

use crate::accordion::{
    AccordionItemOpenChangeDetails, AccordionOrientation, AccordionValueChangeDetails,
};

pub type AccordionValueChangeHandler<T> =
    Rc<dyn Fn(&[T], &mut AccordionValueChangeDetails, &mut Window, &mut App) + 'static>;

pub type AccordionItemOpenChangeHandler =
    Rc<dyn Fn(bool, &mut AccordionItemOpenChangeDetails, &mut Window, &mut App) + 'static>;

pub struct AccordionProps<T: Clone + Eq + 'static> {
    disabled: bool,
    multiple: bool,
    keep_mounted: bool,
    orientation: AccordionOrientation,
    on_value_change: Option<AccordionValueChangeHandler<T>>,
}

impl<T: Clone + Eq + 'static> Clone for AccordionProps<T> {
    fn clone(&self) -> Self {
        Self {
            disabled: self.disabled,
            multiple: self.multiple,
            keep_mounted: self.keep_mounted,
            orientation: self.orientation,
            on_value_change: self.on_value_change.clone(),
        }
    }
}

impl<T: Clone + Eq + 'static> AccordionProps<T> {
    pub fn new(
        disabled: bool,
        multiple: bool,
        keep_mounted: bool,
        orientation: AccordionOrientation,
        on_value_change: Option<AccordionValueChangeHandler<T>>,
    ) -> Self {
        Self {
            disabled,
            multiple,
            keep_mounted,
            orientation,
            on_value_change,
        }
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn multiple(&self) -> bool {
        self.multiple
    }

    pub fn keep_mounted(&self) -> bool {
        self.keep_mounted
    }

    pub fn orientation(&self) -> AccordionOrientation {
        self.orientation
    }

    pub fn on_value_change(&self) -> Option<&AccordionValueChangeHandler<T>> {
        self.on_value_change.as_ref()
    }
}
