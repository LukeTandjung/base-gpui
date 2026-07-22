use std::{
    collections::HashMap,
    rc::Rc,
    time::{Duration, Instant},
};

use gpui::{
    px, App, Bounds, ElementId, FocusHandle, Pixels, Point, ScrollHandle, SharedString, Size,
    Window,
};

use crate::menu::{
    MenuAlign, MenuArrowStyleState, MenuBackdropStyleState, MenuCheckboxItemIndicatorStyleState,
    MenuCheckboxItemStyleState, MenuGroupLabelStyleState, MenuGroupStyleState, MenuItemStyleState,
    MenuLinkItemStyleState, MenuPopupStyleState, MenuPortalStyleState, MenuPositionerStyleState,
    MenuProps, MenuRadioGroupStyleState, MenuRadioItemIndicatorStyleState, MenuRadioItemStyleState,
    MenuRootStyleState, MenuSide, MenuSubmenuTriggerStyleState, MenuTriggerStyleState,
};
use crate::primitives::safe_polygon::{
    SafePolygon, SafePolygonConfig, SafePolygonSide, SafePolygonVerdict,
};

const TYPEAHEAD_RESET_AFTER: Duration = Duration::from_millis(500);

/// Post-open mouseup grace window for Context Menu parents: a window mouse-up
/// before this deadline is the tail of the opening right-click and is inert.
pub const CONTEXT_MENU_GRACE: Duration = Duration::from_millis(500);

/// Tolerance around the initial cursor point within which a mouse-up never
/// activates the item that happened to spawn under the cursor.
pub const CONTEXT_MENU_INITIAL_POINT_TOLERANCE: f32 = 1.0;

/// Outcome of a window mouse-up observed while a Context Menu tree is open.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuContextMenuMouseUp {
    /// Within the post-open grace window: the release of the opening gesture.
    InertGrace,
    /// Within ±1px of the initial cursor point (consumed on first check).
    SuppressedInitialPoint,
    /// Inside the open tree: item drag-release handling applies.
    InsideTree,
    /// Outside the open tree after the grace deadline: close with `CancelOpen`.
    CloseCancelOpen,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MenuOpenChangeReason {
    TriggerHover,
    TriggerFocus,
    TriggerPress,
    OutsidePress,
    FocusOut,
    ListNavigation,
    EscapeKey,
    ItemPress,
    ClosePress,
    SiblingOpen,
    CancelOpen,
    ImperativeAction,
    #[default]
    None,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MenuOpenChangeSource {
    Pointer,
    Touch,
    Keyboard,
    Focus,
    Imperative,
    Unknown,
    #[default]
    None,
}

/// Instant (non-transitioned) classification for the latest open transition.
/// `Group` is reserved for the Menubar issue.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MenuInstantKind {
    #[default]
    None,
    Click,
    Dismiss,
    TriggerChange,
    Group,
}

/// Parent typing seam for the Menu family. `Menubar` and `ContextMenu` are
/// constructible but their behavior is implemented by the Menubar and Context
/// Menu issues; every runtime branch site consults this enum.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MenuParentKind {
    #[default]
    None,
    Submenu,
    /// Reserved: implemented by the Menubar issue.
    Menubar,
    /// Reserved: implemented by the Context Menu issue.
    ContextMenu,
}

#[derive(Clone)]
pub struct MenuOpenChangeDetails<P: Clone + 'static> {
    reason: MenuOpenChangeReason,
    source: MenuOpenChangeSource,
    trigger_id: Option<ElementId>,
    payload: Option<P>,
    cancelable: bool,
    canceled: bool,
    prevent_unmount_on_close: bool,
}

impl<P: Clone + 'static> MenuOpenChangeDetails<P> {
    pub fn new(
        reason: MenuOpenChangeReason,
        source: MenuOpenChangeSource,
        trigger_id: Option<ElementId>,
        payload: Option<P>,
        cancelable: bool,
    ) -> Self {
        Self {
            reason,
            source,
            trigger_id,
            payload,
            cancelable,
            canceled: false,
            prevent_unmount_on_close: false,
        }
    }

    pub fn reason(&self) -> MenuOpenChangeReason {
        self.reason
    }

    pub fn source(&self) -> MenuOpenChangeSource {
        self.source
    }

    pub fn trigger_id(&self) -> Option<&ElementId> {
        self.trigger_id.as_ref()
    }

    pub fn payload(&self) -> Option<&P> {
        self.payload.as_ref()
    }

    pub fn cancelable(&self) -> bool {
        self.cancelable
    }

    pub fn cancel(&mut self) {
        if self.cancelable {
            self.canceled = true;
        }
    }

    pub fn is_canceled(&self) -> bool {
        self.canceled
    }

    pub fn prevent_unmount_on_close(&mut self) {
        self.prevent_unmount_on_close = true;
    }

    pub fn prevents_unmount_on_close(&self) -> bool {
        self.prevent_unmount_on_close
    }
}

/// Cancelable details for checkbox/radio item value changes.
#[derive(Clone, Copy, Debug, Default)]
pub struct MenuItemChangeDetails {
    canceled: bool,
}

impl MenuItemChangeDetails {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cancel(&mut self) {
        self.canceled = true;
    }

    pub fn is_canceled(&self) -> bool {
        self.canceled
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuItemKind {
    Item,
    LinkItem,
    CheckboxItem,
    RadioItem,
    SubmenuTrigger,
}

pub type MenuItemActivation = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

#[derive(Clone)]
pub struct MenuItemMetadata {
    index: usize,
    kind: MenuItemKind,
    label: Option<SharedString>,
    disabled: bool,
    close_on_click: bool,
    focus_handle: Option<FocusHandle>,
    bounds: Option<Bounds<Pixels>>,
    activate: Option<MenuItemActivation>,
}

impl MenuItemMetadata {
    pub fn new(
        index: usize,
        kind: MenuItemKind,
        label: Option<SharedString>,
        disabled: bool,
        close_on_click: bool,
    ) -> Self {
        Self {
            index,
            kind,
            label,
            disabled,
            close_on_click,
            focus_handle: None,
            bounds: None,
            activate: None,
        }
    }

    pub fn with_focus_handle(mut self, focus_handle: FocusHandle) -> Self {
        self.focus_handle = Some(focus_handle);
        self
    }

    pub fn with_activation(mut self, activate: MenuItemActivation) -> Self {
        self.activate = Some(activate);
        self
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn kind(&self) -> MenuItemKind {
        self.kind
    }

    pub fn label(&self) -> Option<&SharedString> {
        self.label.as_ref()
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn close_on_click(&self) -> bool {
        self.close_on_click
    }

    pub fn focus_handle(&self) -> Option<FocusHandle> {
        self.focus_handle.clone()
    }

    pub fn bounds(&self) -> Option<Bounds<Pixels>> {
        self.bounds
    }

    pub fn activation(&self) -> Option<MenuItemActivation> {
        self.activate.clone()
    }
}

#[derive(Clone)]
pub struct MenuTriggerMetadata<P: Clone + 'static> {
    id: ElementId,
    disabled: bool,
    open_on_hover: bool,
    delay: Duration,
    close_delay: Duration,
    payload: Option<P>,
    focus_handle: Option<FocusHandle>,
    bounds: Option<Bounds<Pixels>>,
}

impl<P: Clone + 'static> MenuTriggerMetadata<P> {
    pub fn new(
        id: ElementId,
        disabled: bool,
        open_on_hover: bool,
        delay: Duration,
        close_delay: Duration,
        payload: Option<P>,
        focus_handle: Option<FocusHandle>,
    ) -> Self {
        Self {
            id,
            disabled,
            open_on_hover,
            delay,
            close_delay,
            payload,
            focus_handle,
            bounds: None,
        }
    }

    pub fn id(&self) -> &ElementId {
        &self.id
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn open_on_hover(&self) -> bool {
        self.open_on_hover
    }

    pub fn delay(&self) -> Duration {
        self.delay
    }

    pub fn close_delay(&self) -> Duration {
        self.close_delay
    }

    pub fn payload(&self) -> Option<&P> {
        self.payload.as_ref()
    }

    pub fn focus_handle(&self) -> Option<FocusHandle> {
        self.focus_handle.clone()
    }

    pub fn bounds(&self) -> Option<Bounds<Pixels>> {
        self.bounds
    }
}

pub type MenuSubmenuCloseFn = Rc<dyn Fn(MenuOpenChangeReason, &mut Window, &mut App) + 'static>;
pub type MenuSubmenuOpenFn = Rc<dyn Fn(&mut Window, &mut App) + 'static>;
pub type MenuSubmenuIsOpenFn = Rc<dyn Fn(&App) -> bool + 'static>;
pub type MenuSubmenuContainsFn = Rc<dyn Fn(Point<Pixels>, &App) -> bool + 'static>;
pub type MenuSubmenuBoundsFn = Rc<dyn Fn(&App) -> Option<Bounds<Pixels>> + 'static>;

/// Explicit tree link from a parent menu runtime to one child submenu,
/// replacing Base UI's `FloatingTree` events.
#[derive(Clone)]
pub struct MenuSubmenuLink {
    item_index: usize,
    close_delay: Duration,
    close: MenuSubmenuCloseFn,
    open: MenuSubmenuOpenFn,
    is_open: MenuSubmenuIsOpenFn,
    tree_contains: MenuSubmenuContainsFn,
    popup_bounds: MenuSubmenuBoundsFn,
}

impl MenuSubmenuLink {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        item_index: usize,
        close_delay: Duration,
        close: MenuSubmenuCloseFn,
        open: MenuSubmenuOpenFn,
        is_open: MenuSubmenuIsOpenFn,
        tree_contains: MenuSubmenuContainsFn,
        popup_bounds: MenuSubmenuBoundsFn,
    ) -> Self {
        Self {
            item_index,
            close_delay,
            close,
            open,
            is_open,
            tree_contains,
            popup_bounds,
        }
    }

    pub fn item_index(&self) -> usize {
        self.item_index
    }

    pub fn close_delay(&self) -> Duration {
        self.close_delay
    }

    pub fn close(&self, reason: MenuOpenChangeReason, window: &mut Window, cx: &mut App) {
        (self.close)(reason, window, cx);
    }

    pub fn open(&self, window: &mut Window, cx: &mut App) {
        (self.open)(window, cx);
    }

    pub fn is_open(&self, cx: &App) -> bool {
        (self.is_open)(cx)
    }

    pub fn tree_contains(&self, point: Point<Pixels>, cx: &App) -> bool {
        (self.tree_contains)(point, cx)
    }

    pub fn popup_bounds(&self, cx: &App) -> Option<Bounds<Pixels>> {
        (self.popup_bounds)(cx)
    }
}

pub type MenuMenubarOpenFn =
    Rc<dyn Fn(MenuOpenChangeReason, MenuOpenChangeSource, &mut Window, &mut App) + 'static>;
pub type MenuMenubarRegisterFn = Rc<
    dyn Fn(usize, bool, Option<FocusHandle>, MenuMenubarOpenFn, MenuMenubarOpenFn, &mut App)
        + 'static,
>;
pub type MenuMenubarNoteOpenChangeFn =
    Rc<dyn Fn(usize, bool, MenuOpenChangeReason, &mut App) + 'static>;
pub type MenuMenubarCloseSiblingFn =
    Rc<dyn Fn(usize, MenuOpenChangeReason, MenuOpenChangeSource, &mut Window, &mut App) + 'static>;
pub type MenuMenubarRelayFn = Rc<dyn Fn(usize, MenuMove, &mut Window, &mut App) + 'static>;
pub type MenuMenubarHighlightFn = Rc<dyn Fn(usize, &mut App) + 'static>;
pub type MenuMenubarQueryFn = Rc<dyn Fn(&App) -> bool + 'static>;
pub type MenuMenubarIndexQueryFn = Rc<dyn Fn(usize, &App) -> bool + 'static>;
pub type MenuMenubarBoundsFn = Rc<dyn Fn(&App) -> Option<Bounds<Pixels>> + 'static>;

/// Explicit link from a menubar-hosted menu to the menubar runtime,
/// replacing Base UI's `MenubarContext` + `CompositeRoot` event delegation.
/// Built by the Menubar issue's child wiring and injected into the child
/// `MenuRoot`.
#[derive(Clone)]
pub struct MenuMenubarLink {
    index: usize,
    disabled: bool,
    modal: bool,
    horizontal: bool,
    has_submenu_open: MenuMenubarQueryFn,
    is_tab_stop: MenuMenubarIndexQueryFn,
    register_trigger: MenuMenubarRegisterFn,
    note_open_change: MenuMenubarNoteOpenChangeFn,
    close_sibling: MenuMenubarCloseSiblingFn,
    relay: MenuMenubarRelayFn,
    highlight: MenuMenubarHighlightFn,
    menubar_bounds: MenuMenubarBoundsFn,
}

impl MenuMenubarLink {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        index: usize,
        disabled: bool,
        modal: bool,
        horizontal: bool,
        has_submenu_open: MenuMenubarQueryFn,
        is_tab_stop: MenuMenubarIndexQueryFn,
        register_trigger: MenuMenubarRegisterFn,
        note_open_change: MenuMenubarNoteOpenChangeFn,
        close_sibling: MenuMenubarCloseSiblingFn,
        relay: MenuMenubarRelayFn,
        highlight: MenuMenubarHighlightFn,
        menubar_bounds: MenuMenubarBoundsFn,
    ) -> Self {
        Self {
            index,
            disabled,
            modal,
            horizontal,
            has_submenu_open,
            is_tab_stop,
            register_trigger,
            note_open_change,
            close_sibling,
            relay,
            highlight,
            menubar_bounds,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    /// Menubar-wide disabled fact combined into every child trigger.
    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn modal(&self) -> bool {
        self.modal
    }

    pub fn horizontal(&self) -> bool {
        self.horizontal
    }

    pub fn has_submenu_open(&self, cx: &App) -> bool {
        (self.has_submenu_open)(cx)
    }

    pub fn is_tab_stop(&self, index: usize, cx: &App) -> bool {
        (self.is_tab_stop)(index, cx)
    }

    pub fn register_trigger(
        &self,
        disabled: bool,
        focus_handle: Option<FocusHandle>,
        open: MenuMenubarOpenFn,
        close: MenuMenubarOpenFn,
        cx: &mut App,
    ) {
        (self.register_trigger)(self.index, disabled, focus_handle, open, close, cx);
    }

    pub fn note_open_change(&self, open: bool, reason: MenuOpenChangeReason, cx: &mut App) {
        (self.note_open_change)(self.index, open, reason, cx);
    }

    pub fn close_sibling(
        &self,
        reason: MenuOpenChangeReason,
        source: MenuOpenChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) {
        (self.close_sibling)(self.index, reason, source, window, cx);
    }

    /// Typed keyboard-relay command (menu → menubar): moves the menubar
    /// roving highlight and hands the open menu off to the neighbor.
    pub fn relay(&self, direction: MenuMove, window: &mut Window, cx: &mut App) {
        (self.relay)(self.index, direction, window, cx);
    }

    pub fn highlight(&self, cx: &mut App) {
        (self.highlight)(self.index, cx);
    }

    pub fn menubar_bounds(&self, cx: &App) -> Option<Bounds<Pixels>> {
        (self.menubar_bounds)(cx)
    }
}

pub struct MenuOpenChangeOutcome<P: Clone + 'static> {
    changed: bool,
    open: bool,
    trigger_id: Option<ElementId>,
    payload: Option<P>,
}

impl<P: Clone + 'static> MenuOpenChangeOutcome<P> {
    fn new(changed: bool, open: bool, trigger_id: Option<ElementId>, payload: Option<P>) -> Self {
        Self {
            changed,
            open,
            trigger_id,
            payload,
        }
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn open(&self) -> bool {
        self.open
    }

    pub fn trigger_id(&self) -> Option<&ElementId> {
        self.trigger_id.as_ref()
    }

    pub fn into_parts(self) -> (bool, Option<ElementId>, Option<P>) {
        (self.open, self.trigger_id, self.payload)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuMove {
    Next,
    Previous,
    First,
    Last,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuHoverTarget {
    Open,
    Close,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuTypeaheadOutcome {
    Moved,
    NoMatch,
}

/// What the popup should do about the open child branch after a hover-driven
/// highlight change.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuChildHoverDirective {
    None,
    CancelClose,
    ScheduleClose {
        item_index: usize,
        close_delay: Duration,
        generation: u64,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct MenuPendingHover {
    generation: u64,
    target: MenuHoverTarget,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct MenuPendingChildClose {
    generation: u64,
    item_index: usize,
}

/// Per-menu deep runtime. Each `MenuRoot` and each `MenuSubmenuRoot` owns one,
/// linked into a tree through [`MenuSubmenuLink`]s rather than one god-runtime.
pub struct MenuRuntime<P: Clone + 'static> {
    open: bool,
    parent_kind: MenuParentKind,
    /// Cursor anchor point for Context Menu parents: the menu anchors at this
    /// zero-size virtual rect instead of measured trigger bounds.
    context_menu_anchor: Option<Point<Pixels>>,
    /// Grace deadline armed on every Context Menu open gesture.
    context_menu_grace_deadline: Option<Instant>,
    /// Initial cursor point of the opening gesture; consumed by the first
    /// mouse-up proximity check.
    context_menu_initial_point: Option<Point<Pixels>>,
    last_open_reason: MenuOpenChangeReason,
    last_open_source: MenuOpenChangeSource,
    instant: MenuInstantKind,
    prevent_unmount_on_close: bool,
    trigger: Option<MenuTriggerMetadata<P>>,
    items: Vec<MenuItemMetadata>,
    group_labels: Vec<SharedString>,
    highlighted_index: Option<usize>,
    typeahead: SharedString,
    last_typeahead_at: Option<Instant>,
    allow_mouse_enter: bool,
    hover_enabled: bool,
    stick_if_open: bool,
    checkbox_checked: HashMap<usize, bool>,
    radio_selected: HashMap<usize, Option<usize>>,
    popup_bounds: Option<Bounds<Pixels>>,
    arrow_bounds: Option<Bounds<Pixels>>,
    available_size: Option<Size<Pixels>>,
    effective_side: Option<MenuSide>,
    effective_align: Option<MenuAlign>,
    popup_focus_handle: Option<FocusHandle>,
    popup_scroll_handle: ScrollHandle,
    focus_popup_on_open: bool,
    was_focused: bool,
    trigger_was_focused: bool,
    focus_out_close_requested: bool,
    hover_generation: u64,
    pending_hover: Option<MenuPendingHover>,
    submenu_links: Vec<MenuSubmenuLink>,
    open_submenu_item: Option<usize>,
    child_close_generation: u64,
    pending_child_close: Option<MenuPendingChildClose>,
    child_polygon: SafePolygon,
    child_polygon_item: Option<usize>,
    /// Armed when a menubar trigger press-down opens this menu; the mouse-up
    /// click of that same press must not toggle the menu closed again.
    opened_by_current_press: bool,
}

impl<P: Clone + 'static> MenuRuntime<P> {
    /// Creates runtime state with the observed initial open value and the
    /// parent typing seam fixed for this menu's lifetime.
    pub fn new(open: bool, parent_kind: MenuParentKind) -> Self {
        Self {
            open,
            parent_kind,
            context_menu_anchor: None,
            context_menu_grace_deadline: None,
            context_menu_initial_point: None,
            last_open_reason: MenuOpenChangeReason::None,
            last_open_source: MenuOpenChangeSource::None,
            instant: MenuInstantKind::None,
            prevent_unmount_on_close: false,
            trigger: None,
            items: Vec::new(),
            group_labels: Vec::new(),
            highlighted_index: None,
            typeahead: SharedString::default(),
            last_typeahead_at: None,
            allow_mouse_enter: false,
            hover_enabled: true,
            stick_if_open: false,
            checkbox_checked: HashMap::new(),
            radio_selected: HashMap::new(),
            popup_bounds: None,
            arrow_bounds: None,
            available_size: None,
            effective_side: None,
            effective_align: None,
            popup_focus_handle: None,
            popup_scroll_handle: ScrollHandle::new(),
            focus_popup_on_open: false,
            was_focused: false,
            trigger_was_focused: false,
            focus_out_close_requested: false,
            hover_generation: 0,
            pending_hover: None,
            submenu_links: Vec::new(),
            open_submenu_item: None,
            child_close_generation: 0,
            pending_child_close: None,
            child_polygon: SafePolygon::new(SafePolygonConfig::default()),
            child_polygon_item: None,
            opened_by_current_press: false,
        }
    }

    /// Arms or clears the press-open marker on trigger mouse-down.
    pub fn set_opened_by_current_press(&mut self, opened: bool) {
        self.opened_by_current_press = opened;
    }

    /// Consumes the press-open marker; true when the current click's
    /// mouse-down is the one that opened the menu.
    pub fn take_opened_by_current_press(&mut self) -> bool {
        std::mem::take(&mut self.opened_by_current_press)
    }

    pub fn open_value(&self) -> bool {
        self.open
    }

    pub fn parent_kind(&self) -> MenuParentKind {
        self.parent_kind
    }

    /// Reserved seam consumed by the Context Menu issue: anchor placement at
    /// the pointer instead of the trigger.
    pub fn set_context_menu_anchor(&mut self, anchor: Option<Point<Pixels>>) {
        self.context_menu_anchor = anchor;
    }

    pub fn context_menu_anchor(&self) -> Option<Point<Pixels>> {
        self.context_menu_anchor
    }

    /// Records one Context Menu open gesture: re-anchors at the cursor,
    /// stores the initial cursor point, and arms the post-open grace window.
    /// Gesture-source-agnostic so a touch long-press can drive it later.
    pub fn open_context_menu_at(&mut self, point: Point<Pixels>, now: Instant) {
        self.context_menu_anchor = Some(point);
        self.context_menu_initial_point = Some(point);
        self.context_menu_grace_deadline = Some(now + CONTEXT_MENU_GRACE);
    }

    pub fn context_menu_grace_active(&self, now: Instant) -> bool {
        self.context_menu_grace_deadline
            .map(|deadline| now < deadline)
            .unwrap_or(false)
    }

    pub fn context_menu_initial_point(&self) -> Option<Point<Pixels>> {
        self.context_menu_initial_point
    }

    /// First-mouse-up proximity check against the initial cursor point:
    /// returns true when the release lands within ±1px of the point that
    /// spawned the menu. The recorded point is consumed either way.
    pub fn consume_context_menu_initial_point(&mut self, position: Point<Pixels>) -> bool {
        let Some(initial) = self.context_menu_initial_point.take() else {
            return false;
        };
        let tolerance = px(CONTEXT_MENU_INITIAL_POINT_TOLERANCE);
        (position.x - initial.x).abs() <= tolerance && (position.y - initial.y).abs() <= tolerance
    }

    /// Classifies a window mouse-up while a Context Menu tree is open.
    pub fn context_menu_mouse_up(
        &mut self,
        position: Point<Pixels>,
        inside_tree: bool,
        now: Instant,
    ) -> MenuContextMenuMouseUp {
        if self.context_menu_grace_active(now) {
            if self.consume_context_menu_initial_point(position) {
                return MenuContextMenuMouseUp::SuppressedInitialPoint;
            }
            if !inside_tree {
                return MenuContextMenuMouseUp::InertGrace;
            }
            return MenuContextMenuMouseUp::InsideTree;
        }
        if self.consume_context_menu_initial_point(position) {
            return MenuContextMenuMouseUp::SuppressedInitialPoint;
        }
        match inside_tree {
            true => MenuContextMenuMouseUp::InsideTree,
            false => MenuContextMenuMouseUp::CloseCancelOpen,
        }
    }

    /// Clears the per-gesture Context Menu facts; called when the menu closes
    /// so a subsequent open re-arms deterministically.
    pub fn clear_context_menu_gesture(&mut self) {
        self.context_menu_grace_deadline = None;
        self.context_menu_initial_point = None;
    }

    pub fn mounted_value(&self, keep_mounted: bool) -> bool {
        self.open || keep_mounted || self.prevent_unmount_on_close
    }

    pub fn last_open_reason(&self) -> MenuOpenChangeReason {
        self.last_open_reason
    }

    pub fn instant_kind(&self) -> MenuInstantKind {
        self.instant
    }

    pub fn active_payload(&self) -> Option<P> {
        self.trigger
            .as_ref()
            .and_then(|trigger| trigger.payload.clone())
    }

    /// Reconciles the runtime with an externally controlled open value.
    pub fn sync_open_from_context(&mut self, open: bool) {
        self.open = open;
        if open {
            self.prevent_unmount_on_close = false;
        }
    }

    /// Replaces walked trigger metadata, preserving measured bounds.
    pub fn sync_trigger(&mut self, trigger: Option<MenuTriggerMetadata<P>>) {
        let previous_bounds = self.trigger.as_ref().and_then(MenuTriggerMetadata::bounds);
        self.trigger = trigger.map(|mut trigger| {
            if trigger.bounds.is_none() {
                trigger.bounds = previous_bounds;
            }
            trigger
        });
    }

    /// Replaces walked item metadata, preserving measured bounds and clamping
    /// the highlight to the new item count.
    pub fn sync_items(&mut self, items: Vec<MenuItemMetadata>) {
        let previous_bounds = self
            .items
            .iter()
            .map(|item| (item.index, item.bounds))
            .collect::<HashMap<_, _>>();
        self.items = items
            .into_iter()
            .map(|mut item| {
                if item.bounds.is_none() {
                    item.bounds = previous_bounds.get(&item.index).copied().flatten();
                }
                item
            })
            .collect();
        if let Some(highlighted) = self.highlighted_index {
            if highlighted >= self.items.len() {
                self.highlighted_index = None;
            }
        }
    }

    /// Replaces walked group-label metadata kept for the AccessKit follow-up.
    pub fn sync_group_labels(&mut self, labels: Vec<SharedString>) {
        self.group_labels = labels;
    }

    pub fn group_labels(&self) -> &[SharedString] {
        &self.group_labels
    }

    /// Registers an uncontrolled checkbox item's default checked value once.
    pub fn register_checkbox_default(&mut self, index: usize, default_checked: bool) {
        self.checkbox_checked
            .entry(index)
            .or_insert(default_checked);
    }

    /// Returns the uncontrolled checked value for a checkbox item.
    pub fn checkbox_checked(&self, index: usize) -> bool {
        self.checkbox_checked.get(&index).copied().unwrap_or(false)
    }

    /// Commits an accepted uncontrolled checkbox toggle.
    pub fn commit_checkbox(&mut self, index: usize, checked: bool) {
        self.checkbox_checked.insert(index, checked);
    }

    /// Registers an uncontrolled radio group's default selection once.
    pub fn register_radio_default(&mut self, group_index: usize, selected_item: Option<usize>) {
        self.radio_selected
            .entry(group_index)
            .or_insert(selected_item);
    }

    /// Returns the uncontrolled selected item index for a radio group.
    pub fn radio_selected(&self, group_index: usize) -> Option<usize> {
        self.radio_selected.get(&group_index).copied().flatten()
    }

    /// Commits an accepted uncontrolled radio selection.
    pub fn commit_radio(&mut self, group_index: usize, item_index: usize) {
        self.radio_selected.insert(group_index, Some(item_index));
    }

    /// Requests an open-state transition, deduplicating redundant requests.
    pub fn request_open_change(&self, current: bool, next: bool) -> MenuOpenChangeOutcome<P> {
        let trigger_disabled = self
            .trigger
            .as_ref()
            .map(MenuTriggerMetadata::disabled)
            .unwrap_or(false);
        if next
            && trigger_disabled
            && matches!(
                self.parent_kind,
                MenuParentKind::None | MenuParentKind::Menubar
            )
        {
            return MenuOpenChangeOutcome::new(false, current, None, None);
        }

        let mounted_only_close = !next && !current && self.prevent_unmount_on_close;
        if current == next && !mounted_only_close {
            return MenuOpenChangeOutcome::new(
                false,
                current,
                self.trigger.as_ref().map(|trigger| trigger.id.clone()),
                self.active_payload(),
            );
        }

        MenuOpenChangeOutcome::new(
            true,
            next,
            self.trigger.as_ref().map(|trigger| trigger.id.clone()),
            self.active_payload(),
        )
    }

    /// Commits an accepted open-state transition.
    pub fn commit_open(
        &mut self,
        open: bool,
        prevent_unmount_on_close: bool,
        commit_open_state: bool,
    ) {
        if commit_open_state {
            self.open = open;
        }
        if open {
            self.prevent_unmount_on_close = false;
            self.allow_mouse_enter = false;
            self.hover_enabled = true;
            self.stick_if_open = self.last_open_reason == MenuOpenChangeReason::TriggerHover;
        } else {
            self.clear_context_menu_gesture();
            self.stick_if_open = false;
            self.prevent_unmount_on_close = prevent_unmount_on_close;
            self.focus_popup_on_open = false;
            self.highlighted_index = None;
            self.open_submenu_item = None;
            self.reset_typeahead();
            self.cancel_child_close();
            self.disarm_child_polygon();
        }
    }

    /// Records the reason/source of an accepted transition and classifies the
    /// `instant` transition kind.
    pub fn record_open_change(
        &mut self,
        reason: MenuOpenChangeReason,
        source: MenuOpenChangeSource,
    ) {
        if reason != MenuOpenChangeReason::None {
            self.last_open_reason = reason;
        }
        if source != MenuOpenChangeSource::None {
            self.last_open_source = source;
        }
        // Menubar-parent menus record `group` instants for focus/hover/handoff
        // driven transitions (`MenuRoot.tsx` menubar branch).
        if self.parent_kind == MenuParentKind::Menubar
            && matches!(
                reason,
                MenuOpenChangeReason::TriggerFocus
                    | MenuOpenChangeReason::FocusOut
                    | MenuOpenChangeReason::TriggerHover
                    | MenuOpenChangeReason::ListNavigation
                    | MenuOpenChangeReason::SiblingOpen
            )
        {
            self.instant = MenuInstantKind::Group;
            return;
        }
        self.instant = match (reason, source) {
            (MenuOpenChangeReason::TriggerPress, MenuOpenChangeSource::Keyboard)
            | (MenuOpenChangeReason::ItemPress, MenuOpenChangeSource::Keyboard) => {
                MenuInstantKind::Click
            }
            (
                MenuOpenChangeReason::EscapeKey
                | MenuOpenChangeReason::OutsidePress
                | MenuOpenChangeReason::FocusOut
                | MenuOpenChangeReason::None,
                _,
            ) => MenuInstantKind::Dismiss,
            _ => MenuInstantKind::None,
        };
    }

    /// Records whether this open transition came from a hover interaction, so
    /// modal/backdrop capture can be skipped for hover opens.
    pub fn opened_by_hover(&self) -> bool {
        self.last_open_reason == MenuOpenChangeReason::TriggerHover
    }

    pub fn request_popup_focus_on_open(
        &mut self,
        reason: MenuOpenChangeReason,
        source: MenuOpenChangeSource,
    ) {
        // Submenus never steal initial focus from the parent for hover opens;
        // keyboard/press opens still move focus into the child popup.
        self.focus_popup_on_open = matches!(
            reason,
            MenuOpenChangeReason::TriggerPress | MenuOpenChangeReason::ListNavigation
        ) && matches!(
            source,
            MenuOpenChangeSource::Pointer | MenuOpenChangeSource::Keyboard
        );
    }

    pub fn sync_popup_focus_handle(&mut self, focus_handle: Option<FocusHandle>) {
        self.popup_focus_handle = focus_handle;
    }

    pub fn take_popup_focus_on_open(&mut self) -> Option<FocusHandle> {
        if !self.focus_popup_on_open || !self.open {
            return None;
        }
        self.focus_popup_on_open = false;
        self.popup_focus_handle.clone()
    }

    pub fn popup_focus_handle(&self) -> Option<FocusHandle> {
        self.popup_focus_handle.clone()
    }

    pub fn trigger_focus_handle(&self) -> Option<FocusHandle> {
        self.trigger
            .as_ref()
            .and_then(MenuTriggerMetadata::focus_handle)
    }

    pub fn item_focus_handle(&self, index: usize) -> Option<FocusHandle> {
        self.item(index).and_then(MenuItemMetadata::focus_handle)
    }

    /// Synchronizes focus containment for focus-out dismissal.
    pub fn sync_focus(&mut self, focused: bool) {
        if self.was_focused && !focused && self.open {
            self.focus_out_close_requested = true;
        }
        self.was_focused = focused;
    }

    /// Records the trigger's focus level and reports the rising edge, so
    /// menubar focus-open gating fires once per focus gain rather than every
    /// render while focused.
    pub fn trigger_focus_edge(&mut self, focused: bool) -> bool {
        let rising = focused && !self.trigger_was_focused;
        self.trigger_was_focused = focused;
        rising
    }

    pub fn take_focus_out_close_request(&mut self) -> bool {
        let requested = self.focus_out_close_requested;
        self.focus_out_close_requested = false;
        requested
    }

    // --- Highlight / typeahead -------------------------------------------------

    /// Moves the highlight through registered items (disabled items included,
    /// matching Base UI roving behavior), wrapping or clamping per `loop_focus`.
    pub fn move_highlight(&mut self, direction: MenuMove, loop_focus: bool) {
        if self.items.is_empty() {
            return;
        }
        let last = self.items.len() - 1;
        self.highlighted_index = match direction {
            MenuMove::First => Some(0),
            MenuMove::Last => Some(last),
            MenuMove::Next => match self.highlighted_index {
                None => Some(0),
                Some(index) if index >= last => Some(if loop_focus { 0 } else { last }),
                Some(index) => Some(index + 1),
            },
            MenuMove::Previous => match self.highlighted_index {
                None => Some(last),
                Some(0) => Some(if loop_focus { last } else { 0 }),
                Some(index) => Some(index - 1),
            },
        };
        self.scroll_highlight_into_view();
    }

    /// Highlights an item from pointer hover, gated on real pointer movement
    /// since open (`allow_mouse_enter`).
    pub fn highlight_item_from_pointer(&mut self, index: usize) -> bool {
        if !self.allow_mouse_enter {
            return false;
        }
        if self.highlighted_index == Some(index) {
            return false;
        }
        self.highlighted_index = Some(index);
        true
    }

    /// Records that the pointer actually moved inside the popup since open.
    pub fn note_pointer_moved(&mut self) {
        self.allow_mouse_enter = true;
    }

    pub fn allow_mouse_enter(&self) -> bool {
        self.allow_mouse_enter
    }

    /// Clears the highlight (pointer leaving the item region).
    pub fn clear_highlight(&mut self) {
        self.highlighted_index = None;
    }

    /// Returns the highlighted item's activation facts for keyboard activation.
    pub fn highlighted_activation(
        &self,
    ) -> Option<(usize, MenuItemKind, bool, bool, Option<MenuItemActivation>)> {
        let index = self.highlighted_index?;
        let item = self.item(index)?;
        Some((
            index,
            item.kind,
            item.disabled,
            item.close_on_click,
            item.activate.clone(),
        ))
    }

    /// Returns activation facts for one item index (pointer activation).
    pub fn item_activation(
        &self,
        index: usize,
    ) -> Option<(MenuItemKind, bool, bool, Option<MenuItemActivation>)> {
        let item = self.item(index)?;
        Some((
            item.kind,
            item.disabled,
            item.close_on_click,
            item.activate.clone(),
        ))
    }

    /// Applies typed text to the typeahead buffer, moving the highlight to the
    /// next matching enabled item (matching never activates).
    pub fn apply_typeahead(&mut self, text: &str, now: Instant) -> MenuTypeaheadOutcome {
        let text = if text == " " { text } else { text.trim() };
        if text.is_empty() {
            return MenuTypeaheadOutcome::NoMatch;
        }

        if self
            .last_typeahead_at
            .map(|last| now.duration_since(last) > TYPEAHEAD_RESET_AFTER)
            .unwrap_or(true)
        {
            self.typeahead = SharedString::default();
        }
        self.last_typeahead_at = Some(now);

        let next_text = format!("{}{}", self.typeahead, text);
        let repeated = next_text
            .chars()
            .next()
            .map(|first| next_text.chars().all(|ch| ch.eq_ignore_ascii_case(&first)))
            .unwrap_or(false);
        let search = if repeated { text } else { next_text.as_str() };

        if !self.highlight_by_typeahead(search) {
            return MenuTypeaheadOutcome::NoMatch;
        }

        self.typeahead = next_text.into();
        MenuTypeaheadOutcome::Moved
    }

    /// Returns whether a typeahead session is in progress (Space contributes
    /// to the match instead of activating).
    pub fn typeahead_active(&self, now: Instant) -> bool {
        !self.typeahead.is_empty()
            && self
                .last_typeahead_at
                .map(|last| now.duration_since(last) <= TYPEAHEAD_RESET_AFTER)
                .unwrap_or(false)
    }

    pub fn reset_typeahead(&mut self) {
        self.typeahead = SharedString::default();
        self.last_typeahead_at = None;
    }

    fn highlight_by_typeahead(&mut self, text: &str) -> bool {
        let text = text.trim_start().to_lowercase();
        if text.is_empty() {
            return false;
        }
        let len = self.items.len();
        if len == 0 {
            return false;
        }
        let current = self.highlighted_index;
        for offset in 1..=len {
            let position = match current {
                Some(current) => (current + offset) % len,
                None => offset - 1,
            };
            let item = &self.items[position];
            if item.disabled {
                continue;
            }
            if item
                .label
                .as_ref()
                .map(|label| label.to_lowercase().starts_with(&text))
                .unwrap_or(false)
            {
                self.highlighted_index = Some(position);
                self.scroll_highlight_into_view();
                return true;
            }
        }
        false
    }

    // --- Hover timers ------------------------------------------------------------

    /// Schedules a delayed hover-open or hover-close transition for the trigger.
    pub fn schedule_hover(&mut self, target: MenuHoverTarget) -> u64 {
        self.hover_generation = self.hover_generation.wrapping_add(1);
        self.pending_hover = Some(MenuPendingHover {
            generation: self.hover_generation,
            target,
        });
        self.hover_generation
    }

    /// Clears any delayed hover transition and invalidates pending timer tasks.
    pub fn cancel_hover(&mut self) {
        self.hover_generation = self.hover_generation.wrapping_add(1);
        self.pending_hover = None;
    }

    /// Consumes a pending delayed hover transition if it is still current.
    pub fn take_scheduled_hover(&mut self, generation: u64, target: MenuHoverTarget) -> bool {
        let matches = self
            .pending_hover
            .as_ref()
            .map(|pending| pending.generation == generation && pending.target == target)
            .unwrap_or(false);
        if matches {
            self.pending_hover = None;
        }
        matches
    }

    // --- Submenu tree ---------------------------------------------------------

    /// Registers or refreshes the tree link for one child submenu.
    pub fn register_submenu_link(&mut self, link: MenuSubmenuLink) {
        if let Some(existing) = self
            .submenu_links
            .iter_mut()
            .find(|existing| existing.item_index == link.item_index)
        {
            *existing = link;
            return;
        }
        self.submenu_links.push(link);
    }

    /// Records that the child submenu anchored at an item opened; parent
    /// hover-driven closing is disabled while a child branch is open.
    pub fn note_submenu_opened(&mut self, item_index: usize) {
        self.open_submenu_item = Some(item_index);
        self.hover_enabled = false;
    }

    /// Records that the child submenu anchored at an item closed.
    pub fn note_submenu_closed(&mut self, item_index: usize) {
        if self.open_submenu_item == Some(item_index) {
            self.open_submenu_item = None;
            self.hover_enabled = true;
        }
        if self.child_polygon_item == Some(item_index) {
            self.disarm_child_polygon();
        }
    }

    pub fn open_submenu_item(&self) -> Option<usize> {
        self.open_submenu_item
    }

    pub fn hover_enabled(&self) -> bool {
        self.hover_enabled
    }

    /// Reserved seam: patient-click stickiness for hover-opened menus is
    /// deferred alongside the Popover patient-click precedent.
    pub fn stick_if_open(&self) -> bool {
        self.stick_if_open
    }

    /// Returns the link for the currently open child branch, if any.
    pub fn open_child_link(&self) -> Option<MenuSubmenuLink> {
        let item_index = self.open_submenu_item?;
        self.submenu_link(item_index)
    }

    pub fn submenu_link(&self, item_index: usize) -> Option<MenuSubmenuLink> {
        self.submenu_links
            .iter()
            .find(|link| link.item_index == item_index)
            .cloned()
    }

    /// Returns sibling submenu links other than the one at `item_index`, so
    /// opening one branch can close the others (`SiblingOpen`).
    pub fn sibling_links_except(&self, item_index: usize) -> Vec<MenuSubmenuLink> {
        self.submenu_links
            .iter()
            .filter(|link| link.item_index != item_index)
            .cloned()
            .collect()
    }

    /// Returns every registered child link (parent-close cascade).
    pub fn all_submenu_links(&self) -> Vec<MenuSubmenuLink> {
        self.submenu_links.clone()
    }

    /// Schedules the open child branch to close after its `close_delay`
    /// because a different parent item was highlighted.
    pub fn schedule_child_close(&mut self, item_index: usize) -> u64 {
        self.child_close_generation = self.child_close_generation.wrapping_add(1);
        self.pending_child_close = Some(MenuPendingChildClose {
            generation: self.child_close_generation,
            item_index,
        });
        self.child_close_generation
    }

    /// Cancels a pending delayed child close (re-hovering the same submenu
    /// trigger).
    pub fn cancel_child_close(&mut self) {
        self.child_close_generation = self.child_close_generation.wrapping_add(1);
        self.pending_child_close = None;
    }

    /// Consumes a pending delayed child close if it is still current.
    pub fn take_scheduled_child_close(&mut self, generation: u64, item_index: usize) -> bool {
        let matches = self
            .pending_child_close
            .as_ref()
            .map(|pending| pending.generation == generation && pending.item_index == item_index)
            .unwrap_or(false);
        if matches {
            self.pending_child_close = None;
        }
        matches
    }

    /// Reconciles the pending delayed child close with the current highlight:
    /// highlighting a different parent item schedules the open child branch to
    /// close after its `close_delay`; re-highlighting the submenu trigger
    /// cancels the pending close.
    pub fn reconcile_child_hover(&mut self) -> MenuChildHoverDirective {
        let Some(open_item) = self.open_submenu_item else {
            return MenuChildHoverDirective::None;
        };
        match self.highlighted_index {
            Some(highlighted) if highlighted == open_item => {
                if self.pending_child_close.is_some() {
                    self.cancel_child_close();
                    return MenuChildHoverDirective::CancelClose;
                }
                MenuChildHoverDirective::None
            }
            Some(_) => {
                if self.pending_child_close.is_some() || self.child_polygon.is_armed() {
                    return MenuChildHoverDirective::None;
                }
                let close_delay = self
                    .submenu_link(open_item)
                    .map(|link| link.close_delay())
                    .unwrap_or(Duration::ZERO);
                let generation = self.schedule_child_close(open_item);
                MenuChildHoverDirective::ScheduleClose {
                    item_index: open_item,
                    close_delay,
                    generation,
                }
            }
            None => MenuChildHoverDirective::None,
        }
    }

    pub fn pending_child_close_item(&self) -> Option<usize> {
        self.pending_child_close
            .as_ref()
            .map(|pending| pending.item_index)
    }

    // --- Safe-polygon hover intent ---------------------------------------------

    /// Arms the safe-polygon tracker when the pointer leaves the submenu
    /// trigger of the open child branch.
    pub fn arm_child_polygon(
        &mut self,
        item_index: usize,
        exit_point: Point<Pixels>,
        trigger_bounds: Bounds<Pixels>,
        popup_bounds: Bounds<Pixels>,
        side: SafePolygonSide,
    ) {
        self.child_polygon_item = Some(item_index);
        self.child_polygon
            .arm(exit_point, trigger_bounds, popup_bounds, side);
    }

    /// Evaluates the pointer against the armed child polygon; `None` when the
    /// tracker is not armed.
    pub fn evaluate_child_polygon(
        &mut self,
        pointer: Point<Pixels>,
        now: Duration,
    ) -> Option<(usize, SafePolygonVerdict)> {
        let item_index = self.child_polygon_item?;
        if !self.child_polygon.is_armed() {
            return None;
        }
        let verdict = self.child_polygon.evaluate(pointer, now);
        if matches!(
            verdict,
            SafePolygonVerdict::LandedPopup | SafePolygonVerdict::LandedTrigger
        ) {
            self.child_polygon_item = None;
        }
        Some((item_index, verdict))
    }

    pub fn disarm_child_polygon(&mut self) {
        self.child_polygon.disarm();
        self.child_polygon_item = None;
    }

    // --- Bounds / placement ------------------------------------------------------

    pub fn set_trigger_bounds(&mut self, bounds: Bounds<Pixels>) -> bool {
        let Some(trigger) = self.trigger.as_mut() else {
            return false;
        };
        if trigger.bounds == Some(bounds) {
            return false;
        }
        trigger.bounds = Some(bounds);
        true
    }

    pub fn set_popup_bounds(&mut self, bounds: Bounds<Pixels>) -> bool {
        if self.popup_bounds == Some(bounds) {
            return false;
        }
        self.popup_bounds = Some(bounds);
        true
    }

    pub fn set_arrow_bounds(&mut self, bounds: Bounds<Pixels>) -> bool {
        if self.arrow_bounds == Some(bounds) {
            return false;
        }
        self.arrow_bounds = Some(bounds);
        true
    }

    pub fn set_item_bounds(&mut self, index: usize, bounds: Bounds<Pixels>) -> bool {
        let Some(item) = self.items.iter_mut().find(|item| item.index == index) else {
            return false;
        };
        if item.bounds == Some(bounds) {
            return false;
        }
        item.bounds = Some(bounds);
        true
    }

    pub fn set_available_size(&mut self, size: Size<Pixels>) -> bool {
        if self.available_size == Some(size) {
            return false;
        }
        self.available_size = Some(size);
        true
    }

    pub fn set_effective_placement(&mut self, side: MenuSide, align: MenuAlign) -> bool {
        if self.effective_side == Some(side) && self.effective_align == Some(align) {
            return false;
        }
        self.effective_side = Some(side);
        self.effective_align = Some(align);
        true
    }

    pub fn trigger_bounds(&self) -> Option<Bounds<Pixels>> {
        // Context Menu parents anchor at a zero-size virtual rect at the
        // cursor instead of measured trigger bounds.
        if self.parent_kind == MenuParentKind::ContextMenu {
            return self
                .context_menu_anchor
                .map(|point| Bounds::new(point, Size::default()));
        }
        self.trigger.as_ref().and_then(MenuTriggerMetadata::bounds)
    }

    pub fn popup_bounds(&self) -> Option<Bounds<Pixels>> {
        self.popup_bounds
    }

    pub fn item_bounds(&self, index: usize) -> Option<Bounds<Pixels>> {
        self.item(index).and_then(MenuItemMetadata::bounds)
    }

    pub fn effective_side(&self) -> Option<MenuSide> {
        self.effective_side
    }

    /// Returns whether the point falls inside this menu's own popup or trigger
    /// (one node of the outside-press tree union).
    pub fn own_tree_contains(&self, point: Point<Pixels>) -> bool {
        let in_popup = self.open
            && self
                .popup_bounds
                .map(|bounds| bounds.contains(&point))
                .unwrap_or(false);
        let in_trigger = self
            .trigger_bounds()
            .map(|bounds| bounds.contains(&point))
            .unwrap_or(false);
        in_popup || in_trigger
    }

    pub fn popup_scroll_handle(&self) -> ScrollHandle {
        self.popup_scroll_handle.clone()
    }

    fn scroll_highlight_into_view(&self) {
        if let Some(index) = self.highlighted_index {
            self.popup_scroll_handle.scroll_to_item(index);
        }
    }

    fn item(&self, index: usize) -> Option<&MenuItemMetadata> {
        self.items.iter().find(|item| item.index == index)
    }

    // --- Style-state queries -------------------------------------------------

    pub fn root_state(&self, props: &MenuProps<P>) -> MenuRootStyleState {
        MenuRootStyleState::new(self.open, props.disabled(), self.parent_kind, self.instant)
    }

    pub fn trigger_state(&self, disabled: bool, payload_present: bool) -> MenuTriggerStyleState<P> {
        MenuTriggerStyleState::new(
            self.open,
            disabled,
            self.trigger.is_some(),
            payload_present || self.active_payload().is_some(),
            self.active_payload(),
        )
    }

    pub fn portal_state(&self, keep_mounted: bool) -> MenuPortalStyleState {
        MenuPortalStyleState::new(self.open, self.mounted_value(keep_mounted))
    }

    pub fn backdrop_state(&self, keep_mounted: bool) -> MenuBackdropStyleState {
        MenuBackdropStyleState::new(
            self.open,
            self.mounted_value(keep_mounted),
            !self.opened_by_hover(),
        )
    }

    pub fn positioner_state(
        &self,
        side: MenuSide,
        align: MenuAlign,
        keep_mounted: bool,
    ) -> MenuPositionerStyleState {
        MenuPositionerStyleState::new(
            self.open,
            self.mounted_value(keep_mounted),
            self.effective_side.unwrap_or(side),
            self.effective_align.unwrap_or(align),
            false,
            self.parent_kind == MenuParentKind::Submenu,
            self.instant,
            self.trigger_bounds(),
            self.popup_bounds,
            self.available_size,
        )
    }

    pub fn popup_state(
        &self,
        side: MenuSide,
        align: MenuAlign,
        keep_mounted: bool,
    ) -> MenuPopupStyleState {
        MenuPopupStyleState::new(
            self.open,
            self.mounted_value(keep_mounted),
            self.effective_side.unwrap_or(side),
            self.effective_align.unwrap_or(align),
            self.parent_kind == MenuParentKind::Submenu,
            self.instant,
        )
    }

    pub fn arrow_state(&self, side: MenuSide, align: MenuAlign) -> MenuArrowStyleState {
        let side = self.effective_side.unwrap_or(side);
        let align = self.effective_align.unwrap_or(align);
        let padding = px(4.0);
        let (offset_x, offset_y, uncentered) = self.arrow_offsets(side, align, padding);
        MenuArrowStyleState::new(self.open, side, align, offset_x, offset_y, uncentered)
    }

    /// Number of registered items, exposed for `aria_size_of_set`.
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    pub fn item_state(&self, index: Option<usize>, disabled: bool) -> MenuItemStyleState {
        let highlighted = index.is_some() && self.highlighted_index == index;
        MenuItemStyleState::new(highlighted, disabled)
    }

    pub fn link_item_state(&self, index: Option<usize>) -> MenuLinkItemStyleState {
        let highlighted = index.is_some() && self.highlighted_index == index;
        MenuLinkItemStyleState::new(highlighted)
    }

    pub fn checkbox_item_state(
        &self,
        index: Option<usize>,
        disabled: bool,
        checked: bool,
    ) -> MenuCheckboxItemStyleState {
        let highlighted = index.is_some() && self.highlighted_index == index;
        MenuCheckboxItemStyleState::new(checked, highlighted, disabled)
    }

    pub fn checkbox_indicator_state(
        &self,
        index: Option<usize>,
        disabled: bool,
        checked: bool,
        keep_mounted: bool,
    ) -> MenuCheckboxItemIndicatorStyleState {
        let highlighted = index.is_some() && self.highlighted_index == index;
        MenuCheckboxItemIndicatorStyleState::new(
            checked,
            highlighted,
            disabled,
            checked || keep_mounted,
        )
    }

    pub fn radio_group_state(&self, disabled: bool) -> MenuRadioGroupStyleState {
        MenuRadioGroupStyleState::new(disabled)
    }

    pub fn radio_item_state(
        &self,
        index: Option<usize>,
        disabled: bool,
        checked: bool,
    ) -> MenuRadioItemStyleState {
        let highlighted = index.is_some() && self.highlighted_index == index;
        MenuRadioItemStyleState::new(checked, highlighted, disabled)
    }

    pub fn radio_indicator_state(
        &self,
        index: Option<usize>,
        disabled: bool,
        checked: bool,
        keep_mounted: bool,
    ) -> MenuRadioItemIndicatorStyleState {
        let highlighted = index.is_some() && self.highlighted_index == index;
        MenuRadioItemIndicatorStyleState::new(
            checked,
            highlighted,
            disabled,
            checked || keep_mounted,
        )
    }

    pub fn submenu_trigger_state(
        &self,
        index: Option<usize>,
        disabled: bool,
    ) -> MenuSubmenuTriggerStyleState {
        let highlighted = index.is_some() && self.highlighted_index == index;
        let open = index.is_some() && self.open_submenu_item == index;
        MenuSubmenuTriggerStyleState::new(open, highlighted, disabled)
    }

    pub fn group_state(&self) -> MenuGroupStyleState {
        MenuGroupStyleState::new(false)
    }

    pub fn group_label_state(&self) -> MenuGroupLabelStyleState {
        MenuGroupLabelStyleState
    }

    /// Returns whether the highlighted item is the tab-stop (roving tabindex).
    pub fn item_is_tab_stop(&self, index: Option<usize>) -> bool {
        index.is_some() && self.highlighted_index == index
    }

    fn arrow_offsets(
        &self,
        side: MenuSide,
        align: MenuAlign,
        padding: Pixels,
    ) -> (Option<Pixels>, Option<Pixels>, bool) {
        let Some(popup_bounds) = self.popup_bounds else {
            return (None, None, false);
        };
        let Some(arrow_bounds) = self.arrow_bounds else {
            return (None, None, false);
        };

        let popup_width = popup_bounds.size.width;
        let popup_height = popup_bounds.size.height;
        let arrow_width = arrow_bounds.size.width;
        let arrow_height = arrow_bounds.size.height;
        let arrow_half_width = arrow_width * 0.5;
        let arrow_half_height = arrow_height * 0.5;

        let anchor_bounds = self.trigger_bounds();
        match side {
            MenuSide::Top | MenuSide::Bottom => {
                let target = anchor_bounds
                    .map(|bounds| bounds.center().x - popup_bounds.left())
                    .unwrap_or(match align {
                        MenuAlign::Start => padding + arrow_half_width,
                        MenuAlign::Center => popup_width * 0.5,
                        MenuAlign::End => popup_width - padding - arrow_half_width,
                    });
                let clamped = clamp_arrow_center(target, padding, popup_width, arrow_half_width);
                let y = match side {
                    MenuSide::Top => popup_height - arrow_height,
                    _ => px(0.0),
                };
                (Some(clamped - arrow_half_width), Some(y), clamped != target)
            }
            MenuSide::Left | MenuSide::Right | MenuSide::InlineStart | MenuSide::InlineEnd => {
                let target = anchor_bounds
                    .map(|bounds| bounds.center().y - popup_bounds.top())
                    .unwrap_or(match align {
                        MenuAlign::Start => padding + arrow_half_height,
                        MenuAlign::Center => popup_height * 0.5,
                        MenuAlign::End => popup_height - padding - arrow_half_height,
                    });
                let clamped = clamp_arrow_center(target, padding, popup_height, arrow_half_height);
                let x = match side {
                    MenuSide::Left | MenuSide::InlineStart => popup_width - arrow_width,
                    _ => px(0.0),
                };
                (
                    Some(x),
                    Some(clamped - arrow_half_height),
                    clamped != target,
                )
            }
        }
    }
}

fn clamp_arrow_center(
    target: Pixels,
    padding: Pixels,
    axis_length: Pixels,
    arrow_half_length: Pixels,
) -> Pixels {
    let min = padding + arrow_half_length;
    let max = axis_length - padding - arrow_half_length;
    if max < min {
        return min;
    }
    if target < min {
        min
    } else if target > max {
        max
    } else {
        target
    }
}

pub fn scoped_menu_id(root_id: &ElementId, part: &str) -> ElementId {
    ElementId::from((root_id.clone(), SharedString::from(part.to_string())))
}
