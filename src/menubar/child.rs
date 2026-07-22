use gpui::{AnyElement, IntoElement};

use crate::menu::MenuRoot;
use crate::menubar::child_wiring::MenubarMenuErased;

/// Typed direct children of `Menubar`: hosted menus occupy roving trigger
/// slots; the `Any` escape hatch covers non-menu row content (Base UI demos
/// render plain elements between menus).
pub enum MenubarChild {
    Menu(Box<dyn MenubarMenuErased>),
    Any(AnyElement),
}

impl IntoElement for MenubarChild {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Menu(menu) => menu.into_any(),
            Self::Any(any) => any,
        }
    }
}

impl<P: Clone + 'static> From<MenuRoot<P>> for MenubarChild {
    fn from(value: MenuRoot<P>) -> Self {
        Self::Menu(Box::new(value))
    }
}
