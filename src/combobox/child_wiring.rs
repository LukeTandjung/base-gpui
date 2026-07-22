use gpui::{App, ElementId, FocusHandle, SharedString, Window};

use crate::combobox::{
    ComboboxChild, ComboboxChipChild, ComboboxContext, ComboboxGroupChild, ComboboxGroupMetadata,
    ComboboxInputGroupChild, ComboboxItemChild, ComboboxItemMetadata, ComboboxListChild,
    ComboboxPopupChild, ComboboxPortalChild, ComboboxPositionerChild,
};

pub struct WiredComboboxChildren<T: Clone + Eq + 'static> {
    pub items: Vec<ComboboxItemMetadata<T>>,
    pub groups: Vec<ComboboxGroupMetadata>,
    pub input_focus_handle: Option<FocusHandle>,
    pub input_focused: bool,
    pub children: Vec<ComboboxChild<T>>,
}

pub trait ComboboxChildNode<T: Clone + Eq + 'static>: Sized {
    fn with_combobox_context(self, context: ComboboxContext<T>) -> Self;

    fn wire_combobox_child(
        self,
        _wiring: &mut ComboboxChildWiring<T>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        self
    }
}

pub struct ComboboxChildWiring<T: Clone + Eq + 'static> {
    root_id: ElementId,
    next_item_index: usize,
    next_group_index: usize,
    current_group_indices: Vec<usize>,
    current_list_child_indices: Vec<usize>,
    items: Vec<ComboboxItemMetadata<T>>,
    groups: Vec<ComboboxGroupMetadata>,
    input_focus_handle: Option<FocusHandle>,
    input_focused: bool,
}

impl<T: Clone + Eq + 'static> ComboboxChildWiring<T> {
    pub fn new(root_id: ElementId) -> Self {
        Self {
            root_id,
            next_item_index: 0,
            next_group_index: 0,
            current_group_indices: Vec::new(),
            current_list_child_indices: Vec::new(),
            items: Vec::new(),
            groups: Vec::new(),
            input_focus_handle: None,
            input_focused: false,
        }
    }

    pub fn scope_child_id(&self, id: &ElementId) -> ElementId {
        ElementId::from((self.root_id.clone(), SharedString::from(id.to_string())))
    }

    pub fn register_input(&mut self, focus_handle: FocusHandle, focused: bool) {
        self.input_focus_handle = Some(focus_handle);
        self.input_focused = focused;
    }

    pub fn begin_group(&mut self) -> usize {
        let index = self.next_group_index;
        self.next_group_index += 1;
        self.groups.push(ComboboxGroupMetadata::new(index));
        self.current_group_indices.push(index);
        index
    }

    pub fn end_group(&mut self) {
        self.current_group_indices.pop();
    }

    pub fn register_group_label(&mut self, label: Option<SharedString>) {
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

    /// Registers item metadata; separators, group labels, status and empty
    /// parts never call this, so they cannot corrupt item indices.
    pub fn register_item(
        &mut self,
        value: Option<T>,
        label: Option<SharedString>,
        disabled: bool,
    ) -> usize {
        let index = self.next_item_index;
        self.next_item_index += 1;

        if let Some(value) = value {
            self.items.push(
                ComboboxItemMetadata::new(value, label, disabled, index)
                    .with_group_index(self.current_group_indices.last().copied())
                    .with_list_child_index(self.current_list_child_indices.last().copied()),
            );
        }

        index
    }

    pub fn wire_input_group_children(
        &mut self,
        children: Vec<ComboboxInputGroupChild<T>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<ComboboxInputGroupChild<T>> {
        children
            .into_iter()
            .map(|child| child.wire_combobox_child(self, window, cx))
            .collect()
    }

    pub fn wire_portal_children(
        &mut self,
        children: Vec<ComboboxPortalChild<T>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<ComboboxPortalChild<T>> {
        children
            .into_iter()
            .map(|child| child.wire_combobox_child(self, window, cx))
            .collect()
    }

    pub fn wire_positioner_children(
        &mut self,
        children: Vec<ComboboxPositionerChild<T>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<ComboboxPositionerChild<T>> {
        children
            .into_iter()
            .map(|child| child.wire_combobox_child(self, window, cx))
            .collect()
    }

    pub fn wire_popup_children(
        &mut self,
        children: Vec<ComboboxPopupChild<T>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<ComboboxPopupChild<T>> {
        children
            .into_iter()
            .map(|child| child.wire_combobox_child(self, window, cx))
            .collect()
    }

    pub fn wire_list_children(
        &mut self,
        children: Vec<ComboboxListChild<T>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<ComboboxListChild<T>> {
        children
            .into_iter()
            .enumerate()
            .map(|(child_index, child)| {
                self.current_list_child_indices.push(child_index);
                let child = child.wire_combobox_child(self, window, cx);
                self.current_list_child_indices.pop();
                child
            })
            .collect()
    }

    pub fn wire_group_children(
        &mut self,
        children: Vec<ComboboxGroupChild<T>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<ComboboxGroupChild<T>> {
        children
            .into_iter()
            .map(|child| child.wire_combobox_child(self, window, cx))
            .collect()
    }

    fn finish(self, children: Vec<ComboboxChild<T>>) -> WiredComboboxChildren<T> {
        WiredComboboxChildren {
            items: self.items,
            groups: self.groups,
            input_focus_handle: self.input_focus_handle,
            input_focused: self.input_focused,
            children,
        }
    }
}

pub fn wire_children<T: Clone + Eq + 'static>(
    children: Vec<ComboboxChild<T>>,
    context: ComboboxContext<T>,
    window: &mut Window,
    cx: &mut App,
) -> WiredComboboxChildren<T> {
    let mut wiring = ComboboxChildWiring::new(context.root_id());
    let children = children
        .into_iter()
        .map(|child| child.wire_combobox_child(&mut wiring, window, cx))
        .map(|child| child.with_combobox_context(context.clone()))
        .collect();

    wiring.finish(children)
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxChild<T> {
    fn with_combobox_context(self, context: ComboboxContext<T>) -> Self {
        match self {
            Self::Label(label) => Self::Label(Box::new(label.with_combobox_context(context))),
            Self::Value(value) => Self::Value(Box::new(value.with_combobox_context(context))),
            Self::Input(input) => Self::Input(Box::new(input.with_combobox_context(context))),
            Self::InputGroup(group) => {
                Self::InputGroup(Box::new(group.with_combobox_context(context)))
            }
            Self::Trigger(trigger) => {
                Self::Trigger(Box::new(trigger.with_combobox_context(context)))
            }
            Self::Chips(chips) => Self::Chips(Box::new(chips.with_combobox_context(context))),
            Self::Clear(clear) => Self::Clear(Box::new(clear.with_combobox_context(context))),
            Self::Portal(portal) => Self::Portal(Box::new(portal.with_combobox_context(context))),
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_combobox_child(
        self,
        wiring: &mut ComboboxChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Input(input) => {
                Self::Input(Box::new(input.wire_combobox_child(wiring, window, cx)))
            }
            Self::InputGroup(group) => {
                Self::InputGroup(Box::new(group.wire_combobox_child(wiring, window, cx)))
            }
            Self::Portal(portal) => {
                Self::Portal(Box::new(portal.wire_combobox_child(wiring, window, cx)))
            }
            other => other,
        }
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxInputGroupChild<T> {
    fn with_combobox_context(self, context: ComboboxContext<T>) -> Self {
        match self {
            Self::Input(input) => Self::Input(Box::new(input.with_combobox_context(context))),
            Self::Trigger(trigger) => {
                Self::Trigger(Box::new(trigger.with_combobox_context(context)))
            }
            Self::Clear(clear) => Self::Clear(Box::new(clear.with_combobox_context(context))),
            Self::Chips(chips) => Self::Chips(Box::new(chips.with_combobox_context(context))),
            Self::Icon(icon) => Self::Icon(Box::new(icon.with_combobox_context(context))),
            Self::Value(value) => Self::Value(Box::new(value.with_combobox_context(context))),
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_combobox_child(
        self,
        wiring: &mut ComboboxChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Input(input) => {
                Self::Input(Box::new(input.wire_combobox_child(wiring, window, cx)))
            }
            other => other,
        }
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxPortalChild<T> {
    fn with_combobox_context(self, context: ComboboxContext<T>) -> Self {
        match self {
            Self::Backdrop(backdrop) => {
                Self::Backdrop(Box::new(backdrop.with_combobox_context(context)))
            }
            Self::Positioner(positioner) => {
                Self::Positioner(Box::new(positioner.with_combobox_context(context)))
            }
            Self::Popup(popup) => Self::Popup(Box::new(popup.with_combobox_context(context))),
            Self::List(list) => Self::List(Box::new(list.with_combobox_context(context))),
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_combobox_child(
        self,
        wiring: &mut ComboboxChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Positioner(positioner) => {
                Self::Positioner(Box::new(positioner.wire_combobox_child(wiring, window, cx)))
            }
            Self::Popup(popup) => {
                Self::Popup(Box::new(popup.wire_combobox_child(wiring, window, cx)))
            }
            Self::List(list) => Self::List(Box::new(list.wire_combobox_child(wiring, window, cx))),
            other => other,
        }
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxPositionerChild<T> {
    fn with_combobox_context(self, context: ComboboxContext<T>) -> Self {
        match self {
            Self::Popup(popup) => Self::Popup(Box::new(popup.with_combobox_context(context))),
            Self::Arrow(arrow) => Self::Arrow(Box::new(arrow.with_combobox_context(context))),
            Self::List(list) => Self::List(Box::new(list.with_combobox_context(context))),
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_combobox_child(
        self,
        wiring: &mut ComboboxChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Popup(popup) => {
                Self::Popup(Box::new(popup.wire_combobox_child(wiring, window, cx)))
            }
            Self::List(list) => Self::List(Box::new(list.wire_combobox_child(wiring, window, cx))),
            other => other,
        }
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxPopupChild<T> {
    fn with_combobox_context(self, context: ComboboxContext<T>) -> Self {
        match self {
            Self::List(list) => Self::List(Box::new(list.with_combobox_context(context))),
            Self::Arrow(arrow) => Self::Arrow(Box::new(arrow.with_combobox_context(context))),
            Self::Status(status) => Self::Status(Box::new(status.with_combobox_context(context))),
            Self::Empty(empty) => Self::Empty(Box::new(empty.with_combobox_context(context))),
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_combobox_child(
        self,
        wiring: &mut ComboboxChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::List(list) => Self::List(Box::new(list.wire_combobox_child(wiring, window, cx))),
            other => other,
        }
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxListChild<T> {
    fn with_combobox_context(self, context: ComboboxContext<T>) -> Self {
        match self {
            Self::Item(item) => Self::Item(Box::new(item.with_combobox_context(context))),
            Self::Group(group) => Self::Group(Box::new(group.with_combobox_context(context))),
            Self::GroupLabel(label) => {
                Self::GroupLabel(Box::new(label.with_combobox_context(context)))
            }
            Self::Collection(collection) => {
                Self::Collection(Box::new(collection.with_combobox_context(context)))
            }
            Self::Empty(empty) => Self::Empty(Box::new(empty.with_combobox_context(context))),
            Self::Status(status) => Self::Status(Box::new(status.with_combobox_context(context))),
            Self::Separator(separator) => Self::Separator(separator),
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_combobox_child(
        self,
        wiring: &mut ComboboxChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Item(item) => Self::Item(Box::new(item.wire_combobox_child(wiring, window, cx))),
            Self::Group(group) => {
                Self::Group(Box::new(group.wire_combobox_child(wiring, window, cx)))
            }
            Self::GroupLabel(label) => {
                Self::GroupLabel(Box::new(label.wire_combobox_child(wiring, window, cx)))
            }
            Self::Collection(collection) => {
                Self::Collection(Box::new(collection.wire_combobox_child(wiring, window, cx)))
            }
            other => other,
        }
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxGroupChild<T> {
    fn with_combobox_context(self, context: ComboboxContext<T>) -> Self {
        match self {
            Self::Item(item) => Self::Item(Box::new(item.with_combobox_context(context))),
            Self::GroupLabel(label) => {
                Self::GroupLabel(Box::new(label.with_combobox_context(context)))
            }
            Self::Separator(separator) => Self::Separator(separator),
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_combobox_child(
        self,
        wiring: &mut ComboboxChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Item(item) => Self::Item(Box::new(item.wire_combobox_child(wiring, window, cx))),
            Self::GroupLabel(label) => {
                Self::GroupLabel(Box::new(label.wire_combobox_child(wiring, window, cx)))
            }
            other => other,
        }
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxItemChild<T> {
    fn with_combobox_context(self, context: ComboboxContext<T>) -> Self {
        match self {
            Self::Indicator(indicator) => {
                Self::Indicator(Box::new(indicator.with_combobox_context(context)))
            }
            Self::Any(any) => Self::Any(any),
        }
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxChipChild<T> {
    fn with_combobox_context(self, context: ComboboxContext<T>) -> Self {
        match self {
            Self::Remove(remove) => Self::Remove(Box::new(remove.with_combobox_context(context))),
            Self::Any(any) => Self::Any(any),
        }
    }
}
