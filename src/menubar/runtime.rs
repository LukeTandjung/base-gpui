use gpui::{Bounds, FocusHandle, Pixels};

use crate::menu::{MenuMenubarOpenFn, MenuOpenChangeReason};
use crate::menubar::{MenubarProps, MenubarStyleState};

/// One registered trigger slot in the menubar's roving order, filled in two
/// phases: child wiring seeds the slot count and disabled facts; the hosted
/// menu's render registers the focus handle and typed open/close commands.
#[derive(Clone, Default)]
pub struct MenubarTriggerSlot {
    disabled: bool,
    focus_handle: Option<FocusHandle>,
    open: Option<MenuMenubarOpenFn>,
    close: Option<MenuMenubarOpenFn>,
}

impl MenubarTriggerSlot {
    pub fn new(disabled: bool) -> Self {
        Self {
            disabled,
            focus_handle: None,
            open: None,
            close: None,
        }
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn focus_handle(&self) -> Option<FocusHandle> {
        self.focus_handle.clone()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenubarMove {
    Next,
    Previous,
    First,
    Last,
}

/// The single owner of menubar business state: trigger slots in source
/// order, the roving highlight, `has_submenu_open`, the identity of the
/// currently open child menu, the measured row bounds (backdrop cutout),
/// and the shared drag-release window.
#[derive(Default)]
pub struct MenubarRuntime {
    triggers: Vec<MenubarTriggerSlot>,
    highlighted_index: Option<usize>,
    has_submenu_open: bool,
    open_menu_index: Option<usize>,
    bounds: Option<Bounds<Pixels>>,
    allow_mouse_up_trigger: bool,
}

impl MenubarRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    /// Replaces the wired trigger slots, preserving previously registered
    /// focus handles and open/close commands by index.
    pub fn sync_triggers(&mut self, triggers: Vec<MenubarTriggerSlot>) {
        let previous = std::mem::take(&mut self.triggers);
        self.triggers = triggers
            .into_iter()
            .enumerate()
            .map(|(index, mut slot)| {
                if let Some(existing) = previous.get(index) {
                    slot.focus_handle = existing.focus_handle.clone();
                    slot.open = existing.open.clone();
                    slot.close = existing.close.clone();
                }
                slot
            })
            .collect();
    }

    /// Resolves the roving tab stop after a sync: an out-of-range highlight
    /// falls back to the first trigger. Disabled triggers stay reachable,
    /// matching Base UI composite roving.
    pub fn reconcile(&mut self) {
        let valid = self
            .highlighted_index
            .map(|index| index < self.triggers.len())
            .unwrap_or(false);
        if !valid {
            self.highlighted_index = match self.triggers.is_empty() {
                true => None,
                false => Some(0),
            };
        }
        if let Some(open) = self.open_menu_index {
            if open >= self.triggers.len() {
                self.open_menu_index = None;
                self.has_submenu_open = false;
            }
        }
    }

    /// Registers one hosted menu's trigger facts and typed commands.
    pub fn register_trigger(
        &mut self,
        index: usize,
        disabled: bool,
        focus_handle: Option<FocusHandle>,
        open: MenuMenubarOpenFn,
        close: MenuMenubarOpenFn,
    ) {
        let Some(slot) = self.triggers.get_mut(index) else {
            return;
        };
        slot.disabled = disabled;
        slot.focus_handle = focus_handle;
        slot.open = Some(open);
        slot.close = Some(close);
    }

    /// Moves the roving highlight across triggers (disabled triggers
    /// included), wrapping or clamping per `loop_focus`.
    pub fn move_highlight(&mut self, direction: MenubarMove, loop_focus: bool) {
        if self.triggers.is_empty() {
            return;
        }
        let last = self.triggers.len() - 1;
        self.highlighted_index = match direction {
            MenubarMove::First => Some(0),
            MenubarMove::Last => Some(last),
            MenubarMove::Next => match self.highlighted_index {
                None => Some(0),
                Some(index) if index >= last => Some(if loop_focus { 0 } else { last }),
                Some(index) => Some(index + 1),
            },
            MenubarMove::Previous => match self.highlighted_index {
                None => Some(last),
                Some(0) => Some(if loop_focus { last } else { 0 }),
                Some(index) => Some(index - 1),
            },
        };
    }

    /// Sets the roving highlight directly (hover-follow and relay origin).
    pub fn set_highlight(&mut self, index: usize) {
        if index < self.triggers.len() {
            self.highlighted_index = Some(index);
        }
    }

    /// Seam 6: maintains `has_submenu_open` from direct-child open changes
    /// only — set on open; retained across `SiblingOpen` / `ListNavigation`
    /// handoff closes; cleared on every other close reason.
    pub fn note_child_open_change(
        &mut self,
        index: usize,
        open: bool,
        reason: MenuOpenChangeReason,
    ) {
        if open {
            self.has_submenu_open = true;
            self.open_menu_index = Some(index);
            self.set_highlight(index);
            return;
        }
        if self.open_menu_index == Some(index) {
            self.open_menu_index = None;
        }
        if !matches!(
            reason,
            MenuOpenChangeReason::SiblingOpen | MenuOpenChangeReason::ListNavigation
        ) {
            self.has_submenu_open = false;
        }
    }

    pub fn has_submenu_open(&self) -> bool {
        self.has_submenu_open
    }

    pub fn open_menu_index(&self) -> Option<usize> {
        self.open_menu_index
    }

    pub fn highlighted_index(&self) -> Option<usize> {
        self.highlighted_index
    }

    /// Answers whether the trigger at `index` owns the menubar's single
    /// roving tab stop.
    pub fn is_tab_stop(&self, index: usize) -> bool {
        self.highlighted_index == Some(index)
    }

    pub fn trigger_disabled(&self, index: usize) -> bool {
        self.triggers
            .get(index)
            .map(MenubarTriggerSlot::disabled)
            .unwrap_or(false)
    }

    pub fn highlighted_focus_handle(&self) -> Option<FocusHandle> {
        self.highlighted_index
            .and_then(|index| self.triggers.get(index))
            .and_then(MenubarTriggerSlot::focus_handle)
    }

    /// Typed open command for one hosted menu, registered by its render.
    pub fn open_command(&self, index: usize) -> Option<MenuMenubarOpenFn> {
        self.triggers.get(index).and_then(|slot| slot.open.clone())
    }

    /// Typed close command for the currently open menu, excluding `except`.
    pub fn open_sibling_close_command(&self, except: usize) -> Option<MenuMenubarOpenFn> {
        let open = self.open_menu_index.filter(|index| *index != except)?;
        self.triggers.get(open).and_then(|slot| slot.close.clone())
    }

    pub fn set_bounds(&mut self, bounds: Bounds<Pixels>) -> bool {
        if self.bounds == Some(bounds) {
            return false;
        }
        self.bounds = Some(bounds);
        true
    }

    pub fn bounds(&self) -> Option<Bounds<Pixels>> {
        self.bounds
    }

    /// Seam 14 storage: the menubar-wide drag-release window shared by every
    /// hosted trigger.
    pub fn set_allow_mouse_up_trigger(&mut self, allow: bool) {
        self.allow_mouse_up_trigger = allow;
    }

    pub fn allow_mouse_up_trigger(&self) -> bool {
        self.allow_mouse_up_trigger
    }

    pub fn trigger_count(&self) -> usize {
        self.triggers.len()
    }

    /// Returns the style state for the `Menubar` layer.
    pub fn root_state(&self, props: &MenubarProps) -> MenubarStyleState {
        MenubarStyleState::new(
            props.orientation(),
            props.modal(),
            self.has_submenu_open,
            props.disabled(),
        )
    }
}
