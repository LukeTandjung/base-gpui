use std::rc::Rc;

use gpui::{App, Window};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TabsOrientation {
    Horizontal,
    Vertical,
}

pub type TabsValueChangeHandler<T> = Rc<dyn Fn(Option<&T>, &mut Window, &mut App) + 'static>;

pub struct TabsProps<T: Clone + Eq + 'static> {
    orientation: TabsOrientation,
    on_value_change: Option<TabsValueChangeHandler<T>>,
}

impl<T: Clone + Eq + 'static> Clone for TabsProps<T> {
    fn clone(&self) -> Self {
        Self {
            orientation: self.orientation,
            on_value_change: self.on_value_change.clone(),
        }
    }
}

impl<T: Clone + Eq + 'static> TabsProps<T> {
    pub fn new(
        orientation: TabsOrientation,
        on_value_change: Option<TabsValueChangeHandler<T>>,
    ) -> Self {
        Self {
            orientation,
            on_value_change,
        }
    }

    pub fn orientation(&self) -> TabsOrientation {
        self.orientation
    }

    pub fn on_value_change(&self) -> Option<&TabsValueChangeHandler<T>> {
        self.on_value_change.as_ref()
    }
}
