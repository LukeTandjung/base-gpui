use gpui::{App, ElementId, Entity, Pixels, Window};

use crate::drawer::{DrawerIndentBackgroundStyleState, DrawerIndentStyleState};

/// App-level drawer coordination state: per-drawer open flags plus the live
/// visual state (swipe progress, frontmost height) of the frontmost non-nested
/// drawer. Owned by a window-keyed entity; drawers report into it automatically
/// and all drawer parts work without a mounted provider — Indent and
/// IndentBackground are simply inactive until a `DrawerProvider` renders.
pub struct DrawerProviderRegistry {
    provider_mounted: bool,
    open_ids: Vec<ElementId>,
    swipe_progress: f32,
    frontmost_height: Option<Pixels>,
}

impl Default for DrawerProviderRegistry {
    fn default() -> Self {
        Self {
            provider_mounted: false,
            open_ids: Vec::new(),
            swipe_progress: 0.0,
            frontmost_height: None,
        }
    }
}

impl DrawerProviderRegistry {
    /// Marks that a `DrawerProvider` is mounted in this window.
    pub fn mark_provider_mounted(&mut self) {
        self.provider_mounted = true;
    }

    /// Registers/deregisters a drawer's open state by root id.
    pub fn set_drawer_open(&mut self, id: ElementId, open: bool) {
        self.open_ids.retain(|existing| existing != &id);
        if open {
            self.open_ids.push(id);
        }
    }

    /// Reports the live visual state from the frontmost non-nested drawer's
    /// gesture; reset to zero on gesture end, dismissal, and drawer unmount.
    pub fn set_visual_state(&mut self, swipe_progress: f32, frontmost_height: Option<Pixels>) {
        self.swipe_progress = swipe_progress;
        self.frontmost_height = frontmost_height;
    }

    /// Whether any registered drawer is open.
    pub fn active(&self) -> bool {
        !self.open_ids.is_empty()
    }

    pub fn indent_state(&self) -> DrawerIndentStyleState {
        if !self.provider_mounted {
            return DrawerIndentStyleState::default();
        }
        DrawerIndentStyleState {
            active: self.active(),
            swipe_progress: self.swipe_progress,
            frontmost_height: self.frontmost_height,
        }
    }

    pub fn indent_background_state(&self) -> DrawerIndentBackgroundStyleState {
        DrawerIndentBackgroundStyleState {
            active: self.provider_mounted && self.active(),
        }
    }
}

/// The window-scoped provider registry entity shared by `DrawerProvider`,
/// drawer roots, `DrawerIndent`, and `DrawerIndentBackground`.
pub fn drawer_provider_registry(
    window: &mut Window,
    cx: &mut App,
) -> Entity<DrawerProviderRegistry> {
    window.use_keyed_state(ElementId::from("base-gpui-drawer-provider"), cx, |_, _| {
        DrawerProviderRegistry::default()
    })
}
