use gpui::{AnyElement, App, Empty, IntoElement, Window};

use crate::menu::{
    child_wiring::MenuRadioGroupErased, MenuArrow, MenuBackdrop, MenuCheckboxItem,
    MenuCheckboxItemIndicator, MenuContext, MenuGroup, MenuGroupLabel, MenuItem, MenuLinkItem,
    MenuPopup, MenuPortal, MenuPositioner, MenuRadioGroup, MenuRadioItem, MenuRadioItemIndicator,
    MenuSeparator, MenuSubmenuRoot, MenuSubmenuTrigger, MenuTrigger,
};

/// Context-area seam: builds an element with the wired menu context. Used by
/// the Context Menu trigger area, which is not a button trigger but still
/// needs the context to command cursor-anchored opens.
pub type MenuContextAreaBuild<P> =
    Box<dyn FnOnce(&MenuContext<P>, &mut Window, &mut App) -> AnyElement>;

pub enum MenuChild<P: Clone + 'static> {
    Trigger(Box<MenuTrigger<P>>),
    Portal(Box<MenuPortal<P>>),
    ContextArea(MenuContextAreaBuild<P>),
    Any(AnyElement),
}

impl<P: Clone + 'static> IntoElement for MenuChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Trigger(trigger) => (*trigger).into_any_element(),
            Self::Portal(portal) => (*portal).into_any_element(),
            // A context area only renders once wired with a context.
            Self::ContextArea(_) => Empty.into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<P: Clone + 'static> From<MenuTrigger<P>> for MenuChild<P> {
    fn from(value: MenuTrigger<P>) -> Self {
        Self::Trigger(Box::new(value))
    }
}

impl<P: Clone + 'static> From<MenuPortal<P>> for MenuChild<P> {
    fn from(value: MenuPortal<P>) -> Self {
        Self::Portal(Box::new(value))
    }
}

pub enum MenuPortalChild<P: Clone + 'static> {
    Backdrop(Box<MenuBackdrop<P>>),
    Positioner(Box<MenuPositioner<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> IntoElement for MenuPortalChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Backdrop(backdrop) => (*backdrop).into_any_element(),
            Self::Positioner(positioner) => (*positioner).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<P: Clone + 'static> From<MenuBackdrop<P>> for MenuPortalChild<P> {
    fn from(value: MenuBackdrop<P>) -> Self {
        Self::Backdrop(Box::new(value))
    }
}

impl<P: Clone + 'static> From<MenuPositioner<P>> for MenuPortalChild<P> {
    fn from(value: MenuPositioner<P>) -> Self {
        Self::Positioner(Box::new(value))
    }
}

impl<P: Clone + 'static> MenuPortalChild<P> {
    pub fn keep_mounted_from_portal(self) -> Self {
        match self {
            Self::Backdrop(backdrop) => Self::Backdrop(Box::new(backdrop.keep_mounted(true))),
            Self::Positioner(positioner) => {
                Self::Positioner(Box::new(positioner.keep_mounted_from_portal()))
            }
            Self::Any(any) => Self::Any(any),
        }
    }
}

pub enum MenuPositionerChild<P: Clone + 'static> {
    Popup(Box<MenuPopup<P>>),
    Arrow(Box<MenuArrow<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> IntoElement for MenuPositionerChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Popup(popup) => (*popup).into_any_element(),
            Self::Arrow(arrow) => (*arrow).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<P: Clone + 'static> From<MenuPopup<P>> for MenuPositionerChild<P> {
    fn from(value: MenuPopup<P>) -> Self {
        Self::Popup(Box::new(value))
    }
}

impl<P: Clone + 'static> From<MenuArrow<P>> for MenuPositionerChild<P> {
    fn from(value: MenuArrow<P>) -> Self {
        Self::Arrow(Box::new(value))
    }
}

impl<P: Clone + 'static> MenuPositionerChild<P> {
    pub fn keep_mounted_from_portal(self) -> Self {
        match self {
            Self::Popup(popup) => Self::Popup(Box::new(popup.keep_mounted(true))),
            Self::Arrow(arrow) => Self::Arrow(arrow),
            Self::Any(any) => Self::Any(any),
        }
    }
}

pub enum MenuPopupChild<P: Clone + 'static> {
    Item(Box<MenuItem<P>>),
    LinkItem(Box<MenuLinkItem<P>>),
    CheckboxItem(Box<MenuCheckboxItem<P>>),
    RadioGroup(Box<dyn MenuRadioGroupErased<P>>),
    Group(Box<MenuGroup<P>>),
    GroupLabel(Box<MenuGroupLabel<P>>),
    Separator(Box<MenuSeparator>),
    SubmenuRoot(Box<MenuSubmenuRoot<P>>),
    Arrow(Box<MenuArrow<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> IntoElement for MenuPopupChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Item(item) => (*item).into_any_element(),
            Self::LinkItem(item) => (*item).into_any_element(),
            Self::CheckboxItem(item) => (*item).into_any_element(),
            Self::RadioGroup(group) => group.into_any(),
            Self::Group(group) => (*group).into_any_element(),
            Self::GroupLabel(label) => (*label).into_any_element(),
            Self::Separator(separator) => (*separator).into_any_element(),
            Self::SubmenuRoot(submenu) => (*submenu).into_any_element(),
            Self::Arrow(arrow) => (*arrow).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<P: Clone + 'static> From<MenuItem<P>> for MenuPopupChild<P> {
    fn from(value: MenuItem<P>) -> Self {
        Self::Item(Box::new(value))
    }
}

impl<P: Clone + 'static> From<MenuLinkItem<P>> for MenuPopupChild<P> {
    fn from(value: MenuLinkItem<P>) -> Self {
        Self::LinkItem(Box::new(value))
    }
}

impl<P: Clone + 'static> From<MenuCheckboxItem<P>> for MenuPopupChild<P> {
    fn from(value: MenuCheckboxItem<P>) -> Self {
        Self::CheckboxItem(Box::new(value))
    }
}

impl<P: Clone + 'static, V: Clone + Eq + 'static> From<MenuRadioGroup<P, V>> for MenuPopupChild<P> {
    fn from(value: MenuRadioGroup<P, V>) -> Self {
        Self::RadioGroup(Box::new(value))
    }
}

impl<P: Clone + 'static> From<MenuGroup<P>> for MenuPopupChild<P> {
    fn from(value: MenuGroup<P>) -> Self {
        Self::Group(Box::new(value))
    }
}

impl<P: Clone + 'static> From<MenuGroupLabel<P>> for MenuPopupChild<P> {
    fn from(value: MenuGroupLabel<P>) -> Self {
        Self::GroupLabel(Box::new(value))
    }
}

impl<P: Clone + 'static> From<MenuSeparator> for MenuPopupChild<P> {
    fn from(value: MenuSeparator) -> Self {
        Self::Separator(Box::new(value))
    }
}

impl<P: Clone + 'static> From<MenuSubmenuRoot<P>> for MenuPopupChild<P> {
    fn from(value: MenuSubmenuRoot<P>) -> Self {
        Self::SubmenuRoot(Box::new(value))
    }
}

impl<P: Clone + 'static> From<MenuArrow<P>> for MenuPopupChild<P> {
    fn from(value: MenuArrow<P>) -> Self {
        Self::Arrow(Box::new(value))
    }
}

pub enum MenuGroupChild<P: Clone + 'static> {
    GroupLabel(Box<MenuGroupLabel<P>>),
    Item(Box<MenuItem<P>>),
    LinkItem(Box<MenuLinkItem<P>>),
    CheckboxItem(Box<MenuCheckboxItem<P>>),
    Separator(Box<MenuSeparator>),
    Any(AnyElement),
}

impl<P: Clone + 'static> IntoElement for MenuGroupChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::GroupLabel(label) => (*label).into_any_element(),
            Self::Item(item) => (*item).into_any_element(),
            Self::LinkItem(item) => (*item).into_any_element(),
            Self::CheckboxItem(item) => (*item).into_any_element(),
            Self::Separator(separator) => (*separator).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<P: Clone + 'static> From<MenuGroupLabel<P>> for MenuGroupChild<P> {
    fn from(value: MenuGroupLabel<P>) -> Self {
        Self::GroupLabel(Box::new(value))
    }
}

impl<P: Clone + 'static> From<MenuItem<P>> for MenuGroupChild<P> {
    fn from(value: MenuItem<P>) -> Self {
        Self::Item(Box::new(value))
    }
}

impl<P: Clone + 'static> From<MenuLinkItem<P>> for MenuGroupChild<P> {
    fn from(value: MenuLinkItem<P>) -> Self {
        Self::LinkItem(Box::new(value))
    }
}

impl<P: Clone + 'static> From<MenuCheckboxItem<P>> for MenuGroupChild<P> {
    fn from(value: MenuCheckboxItem<P>) -> Self {
        Self::CheckboxItem(Box::new(value))
    }
}

impl<P: Clone + 'static> From<MenuSeparator> for MenuGroupChild<P> {
    fn from(value: MenuSeparator) -> Self {
        Self::Separator(Box::new(value))
    }
}

/// Radio items are only representable inside a `MenuRadioGroup`, making a
/// radio item outside a group unrepresentable at the type level.
pub enum MenuRadioGroupChild<P: Clone + 'static, V: Clone + Eq + 'static> {
    RadioItem(Box<MenuRadioItem<P, V>>),
    GroupLabel(Box<MenuGroupLabel<P>>),
    Separator(Box<MenuSeparator>),
    Any(AnyElement),
}

impl<P: Clone + 'static, V: Clone + Eq + 'static> IntoElement for MenuRadioGroupChild<P, V> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::RadioItem(item) => (*item).into_any_element(),
            Self::GroupLabel(label) => (*label).into_any_element(),
            Self::Separator(separator) => (*separator).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<P: Clone + 'static, V: Clone + Eq + 'static> From<MenuRadioItem<P, V>>
    for MenuRadioGroupChild<P, V>
{
    fn from(value: MenuRadioItem<P, V>) -> Self {
        Self::RadioItem(Box::new(value))
    }
}

impl<P: Clone + 'static, V: Clone + Eq + 'static> From<MenuGroupLabel<P>>
    for MenuRadioGroupChild<P, V>
{
    fn from(value: MenuGroupLabel<P>) -> Self {
        Self::GroupLabel(Box::new(value))
    }
}

impl<P: Clone + 'static, V: Clone + Eq + 'static> From<MenuSeparator>
    for MenuRadioGroupChild<P, V>
{
    fn from(value: MenuSeparator) -> Self {
        Self::Separator(Box::new(value))
    }
}

pub enum MenuCheckboxItemChild<P: Clone + 'static> {
    Indicator(Box<MenuCheckboxItemIndicator<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> From<MenuCheckboxItemIndicator<P>> for MenuCheckboxItemChild<P> {
    fn from(value: MenuCheckboxItemIndicator<P>) -> Self {
        Self::Indicator(Box::new(value))
    }
}

pub enum MenuRadioItemChild<P: Clone + 'static, V: Clone + Eq + 'static> {
    Indicator(Box<MenuRadioItemIndicator<P, V>>),
    Any(AnyElement),
}

impl<P: Clone + 'static, V: Clone + Eq + 'static> From<MenuRadioItemIndicator<P, V>>
    for MenuRadioItemChild<P, V>
{
    fn from(value: MenuRadioItemIndicator<P, V>) -> Self {
        Self::Indicator(Box::new(value))
    }
}

pub enum MenuSubmenuRootChild<P: Clone + 'static> {
    SubmenuTrigger(Box<MenuSubmenuTrigger<P>>),
    Portal(Box<MenuPortal<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> IntoElement for MenuSubmenuRootChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::SubmenuTrigger(trigger) => (*trigger).into_any_element(),
            Self::Portal(portal) => (*portal).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<P: Clone + 'static> From<MenuSubmenuTrigger<P>> for MenuSubmenuRootChild<P> {
    fn from(value: MenuSubmenuTrigger<P>) -> Self {
        Self::SubmenuTrigger(Box::new(value))
    }
}

impl<P: Clone + 'static> From<MenuPortal<P>> for MenuSubmenuRootChild<P> {
    fn from(value: MenuPortal<P>) -> Self {
        Self::Portal(Box::new(value))
    }
}
