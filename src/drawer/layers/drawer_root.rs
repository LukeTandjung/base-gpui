use std::rc::Rc;

use gpui::{
    div, px, App, Div, ElementId, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window,
};

use crate::dialog::{
    scoped_dialog_part_id, scoped_dialog_trigger_id, DialogContext, DialogHandle, DialogModalMode,
    DialogOpenChangeCompleteHandler, DialogOpenChangeDetails, DialogOpenChangeHandler, DialogProps,
    DialogRootStyleState,
};
use crate::drawer::{
    child_wiring::wire_children, drawer_provider_registry, DrawerChild, DrawerContext,
    DrawerNestedReport, DrawerNestedReporter, DrawerProps, DrawerRuntime, DrawerSnapPoint,
    DrawerSnapPointChangeDetails, DrawerSnapPointChangeHandler, DrawerSwipeDirection,
};

type DrawerRootStyle<P> = Rc<dyn Fn(DialogRootStyleState<P>, Div) -> Div + 'static>;

/// The drawer root: composes the existing Dialog root machinery (open/close,
/// triggers, focus, dismissal) beneath the drawer marker and drawer runtime,
/// mirroring Base UI's `IsDrawerContext` + `useRenderDialogRoot(props, 'drawer')`.
#[derive(IntoElement)]
pub struct DrawerRoot<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<DrawerChild<P>>,
    default_open: bool,
    open: Option<bool>,
    default_trigger_id: Option<ElementId>,
    trigger_id: Option<Option<ElementId>>,
    modal_mode: DialogModalMode,
    disable_pointer_dismissal: bool,
    handle: Option<DialogHandle<P>>,
    on_open_change: Option<DialogOpenChangeHandler<P>>,
    on_open_change_complete: Option<DialogOpenChangeCompleteHandler<P>>,
    swipe_direction: DrawerSwipeDirection,
    snap_points: Vec<DrawerSnapPoint>,
    snap_to_sequential_points: bool,
    default_snap_point: Option<DrawerSnapPoint>,
    snap_point: Option<Option<DrawerSnapPoint>>,
    on_snap_point_change: Option<DrawerSnapPointChangeHandler>,
    nested_in: Option<DrawerNestedReporter>,
    style_with_state: Option<DrawerRootStyle<P>>,
}

impl<P: Clone + 'static> Default for DrawerRoot<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("drawer"),
            base: div(),
            children: Vec::new(),
            default_open: false,
            open: None,
            default_trigger_id: None,
            trigger_id: None,
            modal_mode: DialogModalMode::Modal,
            disable_pointer_dismissal: false,
            handle: None,
            on_open_change: None,
            on_open_change_complete: None,
            swipe_direction: DrawerSwipeDirection::Down,
            snap_points: Vec::new(),
            snap_to_sequential_points: false,
            default_snap_point: None,
            snap_point: None,
            on_snap_point_change: None,
            nested_in: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for DrawerRoot<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for DrawerRoot<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let root_id = self.id.clone();
        let controlled_trigger_id = self.trigger_id.clone().map(|trigger_id| {
            trigger_id.map(|trigger_id| scoped_dialog_trigger_id(&root_id, &trigger_id))
        });
        let default_trigger_id = self
            .default_trigger_id
            .clone()
            .map(|trigger_id| scoped_dialog_trigger_id(&root_id, &trigger_id));
        let dialog_context = DialogContext::new(
            self.id.clone(),
            cx,
            window,
            self.open,
            self.default_open,
            controlled_trigger_id,
            default_trigger_id,
            DialogProps::new(
                self.modal_mode,
                self.disable_pointer_dismissal,
                self.on_open_change,
                self.on_open_change_complete,
            ),
        );

        let swipe_direction = self.swipe_direction;
        let runtime = window.use_keyed_state(
            scoped_dialog_part_id(&root_id, "drawer-runtime"),
            cx,
            |_, _| DrawerRuntime::new(swipe_direction),
        );
        let context = DrawerContext::new(
            runtime,
            dialog_context.clone(),
            DrawerProps::new(
                self.swipe_direction,
                self.snap_points.clone(),
                self.snap_to_sequential_points,
                self.default_snap_point,
                self.on_snap_point_change,
            ),
            self.snap_point,
        );

        if let Some(handle) = self.handle.as_ref() {
            handle.bind(dialog_context.clone());
        }

        let wired_children = wire_children(self.children, context.clone(), window, cx);
        let dialog_wiring = wired_children.dialog;
        let children = wired_children.children;

        dialog_context.update(cx, |runtime| {
            runtime.sync_triggers(dialog_wiring.triggers);
            runtime.sync_title_ids(dialog_wiring.title_ids);
            runtime.sync_description_ids(dialog_wiring.description_ids);
            runtime.sync_popup_focus_handles(dialog_wiring.popup_focus_handles);
        });

        let open = dialog_context.read(cx, |runtime, _| runtime.open_value());
        let mounted = dialog_context.read(cx, |runtime, _| runtime.portal_state(false).mounted);
        let rem_size = f32::from(window.rem_size());
        let window_height = f32::from(window.viewport_size().height);
        let default_snap_point = self.default_snap_point;
        let nested = self.nested_in.is_some();
        let (swiping, swipe_progress, swipe_movement, frontmost_height) =
            context.update(cx, |runtime| {
                runtime.set_rem_size(rem_size);
                // Window resize re-resolves snap points on the next query.
                runtime.set_viewport_height(window_height);
                runtime.mark_nested(nested);
                runtime.observe_open(open);
                if !open && !runtime.swiping() {
                    runtime.reset_snap_point_to_default(default_snap_point);
                }
                (
                    runtime.swiping(),
                    runtime.swipe_progress(),
                    runtime.gesture_directional_movement(),
                    runtime.frontmost_height(),
                )
            });

        if let Some(focus_handle) =
            dialog_context.update(cx, |runtime| runtime.take_popup_focus_on_open())
        {
            focus_handle.focus(window, cx);
        }

        // Report into the app-shell provider registry (no-op without a mounted
        // DrawerProvider; Indent parts stay inactive).
        let registry = drawer_provider_registry(window, cx);
        registry.update(cx, |registry, _| {
            registry.set_drawer_open(root_id.clone(), open);
            if !nested {
                if swiping || open {
                    registry.set_visual_state(swipe_progress, frontmost_height.map(px));
                } else {
                    registry.set_visual_state(0.0, None);
                }
            }
        });

        // Report presence/height/swipe state to the parent drawer while open or
        // transitioning out; absence once unmounted.
        if let Some(reporter) = self.nested_in.as_ref() {
            reporter.report(
                &DrawerNestedReport {
                    id: root_id,
                    present: mounted,
                    frontmost_height: frontmost_height.map(px),
                    swiping_movement: swipe_movement,
                    swipe_progress,
                },
                cx,
            );
        }

        let style_state = dialog_context.read(cx, |runtime, props| runtime.root_state(props));
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        base.children(children)
    }
}

impl<P: Clone + 'static> DrawerRoot<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<DrawerChild<P>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<DrawerChild<P>>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(DrawerChild::Any(child.into_any_element()));
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn on_open_change(
        mut self,
        on_open_change: impl Fn(bool, &mut DialogOpenChangeDetails<P>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(on_open_change));
        self
    }

    pub fn on_open_change_complete(
        mut self,
        on_open_change_complete: impl Fn(bool, &DialogOpenChangeDetails<P>, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.on_open_change_complete = Some(Rc::new(on_open_change_complete));
        self
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.modal_mode = if modal {
            DialogModalMode::Modal
        } else {
            DialogModalMode::NonModal
        };
        self
    }

    pub fn modal_mode(mut self, modal_mode: DialogModalMode) -> Self {
        self.modal_mode = modal_mode;
        self
    }

    pub fn trap_focus(mut self) -> Self {
        self.modal_mode = DialogModalMode::TrapFocus;
        self
    }

    pub fn disable_pointer_dismissal(mut self, disable_pointer_dismissal: bool) -> Self {
        self.disable_pointer_dismissal = disable_pointer_dismissal;
        self
    }

    pub fn trigger_id(mut self, trigger_id: impl Into<ElementId>) -> Self {
        self.trigger_id = Some(Some(trigger_id.into()));
        self
    }

    pub fn no_trigger_id(mut self) -> Self {
        self.trigger_id = Some(None);
        self
    }

    pub fn default_trigger_id(mut self, trigger_id: impl Into<ElementId>) -> Self {
        self.default_trigger_id = Some(trigger_id.into());
        self
    }

    pub fn handle(mut self, handle: DialogHandle<P>) -> Self {
        self.handle = Some(handle);
        self
    }

    pub fn swipe_direction(mut self, swipe_direction: DrawerSwipeDirection) -> Self {
        self.swipe_direction = swipe_direction;
        self
    }

    pub fn snap_points(mut self, snap_points: Vec<DrawerSnapPoint>) -> Self {
        self.snap_points = snap_points;
        self
    }

    pub fn snap_to_sequential_points(mut self, snap_to_sequential_points: bool) -> Self {
        self.snap_to_sequential_points = snap_to_sequential_points;
        self
    }

    pub fn default_snap_point(mut self, default_snap_point: Option<DrawerSnapPoint>) -> Self {
        self.default_snap_point = default_snap_point;
        self
    }

    /// Calling this builder marks the snap point controlled even when `None`.
    pub fn snap_point(mut self, snap_point: Option<DrawerSnapPoint>) -> Self {
        self.snap_point = Some(snap_point);
        self
    }

    pub fn on_snap_point_change(
        mut self,
        on_snap_point_change: impl Fn(Option<DrawerSnapPoint>, &mut DrawerSnapPointChangeDetails, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.on_snap_point_change = Some(Rc::new(on_snap_point_change));
        self
    }

    /// Links this drawer as nested inside a parent drawer; obtain the reporter
    /// from the parent's `DrawerContext::nested_reporter()`.
    pub fn nested_in(mut self, reporter: DrawerNestedReporter) -> Self {
        self.nested_in = Some(reporter);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(DialogRootStyleState<P>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
