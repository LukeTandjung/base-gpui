use std::rc::Rc;

use gpui::{App, ElementId, Entity, Window};

use crate::menu::{MenuOpenChangeReason, MenuOpenChangeSource};
use crate::menubar::{MenubarMove, MenubarProps, MenubarRuntime};

/// Thin injection vehicle for the menubar runtime and props, plus the
/// menubar commands that need a window (roving focus movement and the open
/// menu handoff between sibling triggers).
pub struct MenubarContext {
    runtime: Entity<MenubarRuntime>,
    props: Rc<MenubarProps>,
}

impl Clone for MenubarContext {
    fn clone(&self) -> Self {
        Self {
            runtime: self.runtime.clone(),
            props: Rc::clone(&self.props),
        }
    }
}

impl MenubarContext {
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        props: MenubarProps,
    ) -> Self {
        let runtime = window.use_keyed_state(id.into(), cx, |_, _| MenubarRuntime::new());

        Self {
            runtime,
            props: Rc::new(props),
        }
    }

    /// Render-stable props facts exposed to child wiring without a runtime
    /// read.
    pub fn props_disabled(&self) -> bool {
        self.props.disabled()
    }

    pub fn props_modal(&self) -> bool {
        self.props.modal()
    }

    pub fn props_horizontal(&self) -> bool {
        self.props.orientation() == crate::menubar::MenubarOrientation::Horizontal
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&MenubarRuntime, &MenubarProps) -> Output,
    ) -> Output {
        read(self.runtime.read(cx), self.props.as_ref())
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut MenubarRuntime) -> Output,
    ) -> Output {
        self.runtime.update(cx, |runtime, cx| {
            let output = update(runtime);
            cx.notify();
            output
        })
    }

    /// Moves the roving highlight and real focus in one step; while a menu
    /// is open, the open menu follows the highlight (keyboard handoff).
    pub fn move_focus(&self, direction: MenubarMove, window: &mut Window, cx: &mut App) {
        let loop_focus = self.props.loop_focus();
        let (focus_handle, target, has_open) = self.update(cx, |runtime| {
            runtime.move_highlight(direction, loop_focus);
            (
                runtime.highlighted_focus_handle(),
                runtime.highlighted_index(),
                runtime.has_submenu_open(),
            )
        });
        if let Some(focus_handle) = focus_handle {
            focus_handle.focus(window, cx);
        }
        if has_open {
            if let Some(target) = target {
                self.hand_off(target, MenuOpenChangeReason::ListNavigation, window, cx);
            }
        }
    }

    /// Keyboard relay entry (seam 8): re-anchors the highlight at the
    /// relaying menu's trigger, then moves the highlight/focus and hands the
    /// open menu off to the new trigger.
    pub fn relay(&self, from: usize, direction: MenubarMove, window: &mut Window, cx: &mut App) {
        self.update(cx, |runtime| runtime.set_highlight(from));
        self.move_focus(direction, window, cx);
    }

    /// Hands the open menu off to `target`: closes the open sibling with the
    /// handoff reason (retaining `has_submenu_open`) and opens the target
    /// menu through its registered typed command.
    pub fn hand_off(
        &self,
        target: usize,
        reason: MenuOpenChangeReason,
        window: &mut Window,
        cx: &mut App,
    ) {
        let (already_open, disabled, open) = self.read(cx, |runtime, props| {
            (
                runtime.open_menu_index() == Some(target),
                props.disabled() || runtime.trigger_disabled(target),
                runtime.open_command(target),
            )
        });
        if already_open || disabled {
            return;
        }
        let source = match reason {
            MenuOpenChangeReason::ListNavigation => MenuOpenChangeSource::Keyboard,
            _ => MenuOpenChangeSource::Pointer,
        };
        // Close with the handoff reason first so the closing side records it
        // (`ListNavigation` for keyboard relays); the subsequent open then
        // finds no open sibling left to close.
        self.close_open_sibling(target, reason, source, window, cx);
        if let Some(open) = open {
            open(reason, source, window, cx);
        }
    }

    /// Closes the open menu other than `except` (a menubar sibling opening).
    pub fn close_open_sibling(
        &self,
        except: usize,
        reason: MenuOpenChangeReason,
        source: MenuOpenChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) {
        let close = self.read(cx, |runtime, _| runtime.open_sibling_close_command(except));
        if let Some(close) = close {
            close(reason, source, window, cx);
        }
    }
}
