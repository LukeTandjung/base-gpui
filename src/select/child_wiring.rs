use gpui::{App, ElementId, FocusHandle, SharedString, Window};

use crate::select::{
    SelectChild, SelectContext, SelectGroupChild, SelectGroupMetadata, SelectItemChild,
    SelectItemMetadata, SelectListChild, SelectPopupChild, SelectPortalChild,
    SelectPositionerChild, SelectTriggerChild,
};

pub struct WiredSelectChildren<T: Clone + Eq + 'static> {
    pub items: Vec<SelectItemMetadata<T>>,
    pub groups: Vec<SelectGroupMetadata>,
    pub item_focus_handles: Vec<(usize, FocusHandle)>,
    pub trigger_focus_handle: Option<FocusHandle>,
    pub trigger_focused: bool,
    pub focused_item_index: Option<usize>,
    pub children: Vec<SelectChild<T>>,
}

pub trait SelectChildNode<T: Clone + Eq + 'static>: Sized {
    fn with_select_context(self, context: SelectContext<T>) -> Self;

    fn wire_select_child(
        self,
        _wiring: &mut SelectChildWiring<T>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        self
    }
}

pub struct SelectChildWiring<T: Clone + Eq + 'static> {
    root_id: ElementId,
    next_item_index: usize,
    next_group_index: usize,
    current_group_indices: Vec<usize>,
    current_list_child_indices: Vec<usize>,
    items: Vec<SelectItemMetadata<T>>,
    groups: Vec<SelectGroupMetadata>,
    item_focus_handles: Vec<(usize, FocusHandle)>,
    trigger_focus_handle: Option<FocusHandle>,
    trigger_focused: bool,
    focused_item_index: Option<usize>,
}

impl<T: Clone + Eq + 'static> SelectChildWiring<T> {
    pub fn new(root_id: ElementId) -> Self {
        Self {
            root_id,
            next_item_index: 0,
            next_group_index: 0,
            current_group_indices: Vec::new(),
            current_list_child_indices: Vec::new(),
            items: Vec::new(),
            groups: Vec::new(),
            item_focus_handles: Vec::new(),
            trigger_focus_handle: None,
            trigger_focused: false,
            focused_item_index: None,
        }
    }

    pub fn scope_child_id(&self, id: &ElementId) -> ElementId {
        ElementId::from((self.root_id.clone(), SharedString::from(id.to_string())))
    }

    pub fn wire_trigger_children(
        &mut self,
        children: Vec<SelectTriggerChild<T>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<SelectTriggerChild<T>> {
        children
            .into_iter()
            .map(|child| child.wire_select_child(self, window, cx))
            .collect()
    }

    pub fn wire_portal_children(
        &mut self,
        children: Vec<SelectPortalChild<T>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<SelectPortalChild<T>> {
        children
            .into_iter()
            .map(|child| child.wire_select_child(self, window, cx))
            .collect()
    }

    pub fn wire_positioner_children(
        &mut self,
        children: Vec<SelectPositionerChild<T>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<SelectPositionerChild<T>> {
        children
            .into_iter()
            .map(|child| child.wire_select_child(self, window, cx))
            .collect()
    }

    pub fn wire_popup_children(
        &mut self,
        children: Vec<SelectPopupChild<T>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<SelectPopupChild<T>> {
        children
            .into_iter()
            .map(|child| child.wire_select_child(self, window, cx))
            .collect()
    }

    pub fn wire_list_children(
        &mut self,
        children: Vec<SelectListChild<T>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<SelectListChild<T>> {
        children
            .into_iter()
            .enumerate()
            .map(|(child_index, child)| {
                self.current_list_child_indices.push(child_index);
                let child = child.wire_select_child(self, window, cx);
                self.current_list_child_indices.pop();
                child
            })
            .collect()
    }

    pub fn wire_group_children(
        &mut self,
        children: Vec<SelectGroupChild<T>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<SelectGroupChild<T>> {
        children
            .into_iter()
            .map(|child| child.wire_select_child(self, window, cx))
            .collect()
    }

    pub fn wire_item_children(
        &mut self,
        children: Vec<SelectItemChild<T>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<SelectItemChild<T>> {
        children
            .into_iter()
            .map(|child| child.wire_select_child(self, window, cx))
            .collect()
    }

    pub fn register_trigger(&mut self, focus_handle: FocusHandle, focused: bool) {
        self.trigger_focus_handle = Some(focus_handle);
        self.trigger_focused = focused;
    }

    pub fn begin_group(&mut self) -> usize {
        let index = self.next_group_index;
        self.next_group_index += 1;
        self.groups.push(SelectGroupMetadata::new(index));
        self.current_group_indices.push(index);
        index
    }

    pub fn end_group(&mut self) {
        self.current_group_indices.pop();
    }

    pub fn register_group_label(&mut self, label: Option<gpui::SharedString>) {
        let Some(group_index) = self.current_group_indices.last().copied() else {
            return;
        };
        let Some(group) = self
            .groups
            .iter_mut()
            .find(|group| group.index() == group_index)
        else {
            return;
        };

        group.set_label(label);
    }

    pub fn register_item(
        &mut self,
        value: Option<T>,
        label: Option<gpui::SharedString>,
        disabled: bool,
        focus_handle: FocusHandle,
        focused: bool,
    ) -> usize {
        let index = self.next_item_index;
        self.next_item_index += 1;

        if let Some(value) = value {
            self.items.push(
                SelectItemMetadata::new(value, label, disabled, index)
                    .with_group_index(self.current_group_indices.last().copied())
                    .with_list_child_index(self.current_list_child_indices.last().copied()),
            );
        }

        if focused {
            self.focused_item_index = Some(index);
        }

        self.item_focus_handles.push((index, focus_handle));
        index
    }

    fn finish(self, children: Vec<SelectChild<T>>) -> WiredSelectChildren<T> {
        WiredSelectChildren {
            items: self.items,
            groups: self.groups,
            item_focus_handles: self.item_focus_handles,
            trigger_focus_handle: self.trigger_focus_handle,
            trigger_focused: self.trigger_focused,
            focused_item_index: self.focused_item_index,
            children,
        }
    }
}

pub fn wire_children<T: Clone + Eq + 'static>(
    children: Vec<SelectChild<T>>,
    context: SelectContext<T>,
    window: &mut Window,
    cx: &mut App,
) -> WiredSelectChildren<T> {
    let mut wiring = SelectChildWiring::new(context.root_id());
    let children = children
        .into_iter()
        .map(|child| child.wire_select_child(&mut wiring, window, cx))
        .map(|child| child.with_select_context(context.clone()))
        .collect();

    wiring.finish(children)
}

impl<T: Clone + Eq + 'static> SelectChildNode<T> for SelectChild<T> {
    fn with_select_context(self, context: SelectContext<T>) -> Self {
        match self {
            Self::Label(label) => Self::Label(Box::new(label.with_select_context(context))),
            Self::Trigger(trigger) => Self::Trigger(Box::new(trigger.with_select_context(context))),
            Self::Portal(portal) => Self::Portal(Box::new(portal.with_select_context(context))),
            Self::Backdrop(backdrop) => {
                Self::Backdrop(Box::new(backdrop.with_select_context(context)))
            }
            Self::Positioner(positioner) => {
                Self::Positioner(Box::new(positioner.with_select_context(context)))
            }
            Self::Popup(popup) => Self::Popup(Box::new(popup.with_select_context(context))),
            Self::List(list) => Self::List(Box::new(list.with_select_context(context))),
            Self::Group(group) => Self::Group(Box::new(group.with_select_context(context))),
            Self::GroupLabel(label) => {
                Self::GroupLabel(Box::new(label.with_select_context(context)))
            }
            Self::Item(item) => Self::Item(Box::new(item.with_select_context(context))),
            Self::Separator(separator) => Self::Separator(separator),
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_select_child(
        self,
        wiring: &mut SelectChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Trigger(trigger) => {
                Self::Trigger(Box::new(trigger.wire_select_child(wiring, window, cx)))
            }
            Self::Portal(portal) => {
                Self::Portal(Box::new(portal.wire_select_child(wiring, window, cx)))
            }
            Self::Positioner(positioner) => {
                Self::Positioner(Box::new(positioner.wire_select_child(wiring, window, cx)))
            }
            Self::Popup(popup) => {
                Self::Popup(Box::new(popup.wire_select_child(wiring, window, cx)))
            }
            Self::List(list) => Self::List(Box::new(list.wire_select_child(wiring, window, cx))),
            Self::Group(group) => {
                Self::Group(Box::new(group.wire_select_child(wiring, window, cx)))
            }
            Self::Item(item) => Self::Item(Box::new(item.wire_select_child(wiring, window, cx))),
            other => other,
        }
    }
}

impl<T: Clone + Eq + 'static> SelectChildNode<T> for SelectTriggerChild<T> {
    fn with_select_context(self, context: SelectContext<T>) -> Self {
        match self {
            Self::Value(value) => Self::Value(Box::new(value.with_select_context(context))),
            Self::Icon(icon) => Self::Icon(Box::new(icon.with_select_context(context))),
            Self::Any(any) => Self::Any(any),
        }
    }
}

impl<T: Clone + Eq + 'static> SelectChildNode<T> for SelectPortalChild<T> {
    fn with_select_context(self, context: SelectContext<T>) -> Self {
        match self {
            Self::Backdrop(backdrop) => {
                Self::Backdrop(Box::new(backdrop.with_select_context(context)))
            }
            Self::Positioner(positioner) => {
                Self::Positioner(Box::new(positioner.with_select_context(context)))
            }
            Self::Popup(popup) => Self::Popup(Box::new(popup.with_select_context(context))),
            Self::List(list) => Self::List(Box::new(list.with_select_context(context))),
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_select_child(
        self,
        wiring: &mut SelectChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Positioner(positioner) => {
                Self::Positioner(Box::new(positioner.wire_select_child(wiring, window, cx)))
            }
            Self::Popup(popup) => {
                Self::Popup(Box::new(popup.wire_select_child(wiring, window, cx)))
            }
            Self::List(list) => Self::List(Box::new(list.wire_select_child(wiring, window, cx))),
            other => other,
        }
    }
}

impl<T: Clone + Eq + 'static> SelectChildNode<T> for SelectPositionerChild<T> {
    fn with_select_context(self, context: SelectContext<T>) -> Self {
        match self {
            Self::Popup(popup) => Self::Popup(Box::new(popup.with_select_context(context))),
            Self::Arrow(arrow) => Self::Arrow(Box::new(arrow.with_select_context(context))),
            Self::List(list) => Self::List(Box::new(list.with_select_context(context))),
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_select_child(
        self,
        wiring: &mut SelectChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Popup(popup) => {
                Self::Popup(Box::new(popup.wire_select_child(wiring, window, cx)))
            }
            Self::List(list) => Self::List(Box::new(list.wire_select_child(wiring, window, cx))),
            other => other,
        }
    }
}

impl<T: Clone + Eq + 'static> SelectChildNode<T> for SelectPopupChild<T> {
    fn with_select_context(self, context: SelectContext<T>) -> Self {
        match self {
            Self::List(list) => Self::List(Box::new(list.with_select_context(context))),
            Self::Arrow(arrow) => Self::Arrow(Box::new(arrow.with_select_context(context))),
            Self::ScrollUpArrow(arrow) => {
                Self::ScrollUpArrow(Box::new(arrow.with_select_context(context)))
            }
            Self::ScrollDownArrow(arrow) => {
                Self::ScrollDownArrow(Box::new(arrow.with_select_context(context)))
            }
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_select_child(
        self,
        wiring: &mut SelectChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::List(list) => Self::List(Box::new(list.wire_select_child(wiring, window, cx))),
            other => other,
        }
    }
}

impl<T: Clone + Eq + 'static> SelectChildNode<T> for SelectListChild<T> {
    fn with_select_context(self, context: SelectContext<T>) -> Self {
        match self {
            Self::Item(item) => Self::Item(Box::new(item.with_select_context(context))),
            Self::Group(group) => Self::Group(Box::new(group.with_select_context(context))),
            Self::GroupLabel(label) => {
                Self::GroupLabel(Box::new(label.with_select_context(context)))
            }
            Self::ScrollUpArrow(arrow) => {
                Self::ScrollUpArrow(Box::new(arrow.with_select_context(context)))
            }
            Self::ScrollDownArrow(arrow) => {
                Self::ScrollDownArrow(Box::new(arrow.with_select_context(context)))
            }
            Self::Separator(separator) => Self::Separator(separator),
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_select_child(
        self,
        wiring: &mut SelectChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Item(item) => Self::Item(Box::new(item.wire_select_child(wiring, window, cx))),
            Self::Group(group) => {
                Self::Group(Box::new(group.wire_select_child(wiring, window, cx)))
            }
            Self::GroupLabel(label) => {
                Self::GroupLabel(Box::new(label.wire_select_child(wiring, window, cx)))
            }
            other => other,
        }
    }
}

impl<T: Clone + Eq + 'static> SelectChildNode<T> for SelectGroupChild<T> {
    fn with_select_context(self, context: SelectContext<T>) -> Self {
        match self {
            Self::Item(item) => Self::Item(Box::new(item.with_select_context(context))),
            Self::GroupLabel(label) => {
                Self::GroupLabel(Box::new(label.with_select_context(context)))
            }
            Self::Separator(separator) => Self::Separator(separator),
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_select_child(
        self,
        wiring: &mut SelectChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Item(item) => Self::Item(Box::new(item.wire_select_child(wiring, window, cx))),
            Self::GroupLabel(label) => {
                Self::GroupLabel(Box::new(label.wire_select_child(wiring, window, cx)))
            }
            other => other,
        }
    }
}

impl<T: Clone + Eq + 'static> SelectChildNode<T> for SelectItemChild<T> {
    fn with_select_context(self, context: SelectContext<T>) -> Self {
        match self {
            Self::Text(text) => Self::Text(Box::new(text.with_select_context(context))),
            Self::Indicator(indicator) => {
                Self::Indicator(Box::new(indicator.with_select_context(context)))
            }
            Self::Any(any) => Self::Any(any),
        }
    }
}
