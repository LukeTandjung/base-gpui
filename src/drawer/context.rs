use std::rc::Rc;

use gpui::{App, ElementId, Entity, Pixels, Window};

use crate::dialog::{DialogContext, DialogOpenChangeReason};
use crate::drawer::{DrawerProps, DrawerRuntime, DrawerSnapPoint, DrawerSnapPointChangeDetails};

/// Thin injection for drawer parts: the drawer runtime entity, the inner
/// `DialogContext<P>`, drawer props, and the controlled snap-point marker.
/// Open/close continues to go through `DialogContext<P>` methods.
pub struct DrawerContext<P: Clone + 'static> {
    drawer: Entity<DrawerRuntime>,
    dialog: DialogContext<P>,
    props: Rc<DrawerProps>,
    controlled_snap_point: Rc<Option<Option<DrawerSnapPoint>>>,
}

impl<P: Clone + 'static> Clone for DrawerContext<P> {
    fn clone(&self) -> Self {
        Self {
            drawer: self.drawer.clone(),
            dialog: self.dialog.clone(),
            props: Rc::clone(&self.props),
            controlled_snap_point: Rc::clone(&self.controlled_snap_point),
        }
    }
}

impl<P: Clone + 'static> DrawerContext<P> {
    pub fn new(
        drawer: Entity<DrawerRuntime>,
        dialog: DialogContext<P>,
        props: DrawerProps,
        controlled_snap_point: Option<Option<DrawerSnapPoint>>,
    ) -> Self {
        Self {
            drawer,
            dialog,
            props: Rc::new(props),
            controlled_snap_point: Rc::new(controlled_snap_point),
        }
    }

    pub fn dialog(&self) -> &DialogContext<P> {
        &self.dialog
    }

    pub fn root_id(&self) -> ElementId {
        self.dialog.root_id()
    }

    pub fn snap_point_controlled(&self) -> bool {
        self.controlled_snap_point.is_some()
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&DrawerRuntime, &DrawerProps) -> Output,
    ) -> Output {
        read(self.drawer.read(cx), self.props.as_ref())
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut DrawerRuntime) -> Output,
    ) -> Output {
        let props = Rc::clone(&self.props);
        let controlled = self.controlled_snap_point.as_ref().clone();
        self.drawer.update(cx, |runtime, cx| {
            runtime.sync_props(
                props.swipe_direction(),
                props.snap_points().clone(),
                props.snap_to_sequential_points(),
            );
            runtime.reconcile_snap_point(controlled, props.default_snap_point());
            let output = update(runtime);
            cx.notify();
            output
        })
    }

    /// The one value-changing method: resolves controlled/uncontrolled, fires
    /// `on_snap_point_change` with cancelable details, and commits the
    /// uncontrolled mutation when accepted. Returns whether the change was
    /// accepted.
    pub fn set_snap_point(
        &self,
        snap_point: Option<DrawerSnapPoint>,
        reason: DialogOpenChangeReason,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let current = self.read(cx, |runtime, _| runtime.snap_point_value());
        if current == snap_point {
            return false;
        }

        let mut details = DrawerSnapPointChangeDetails::new(reason, true);
        if let Some(on_snap_point_change) = self.props.on_snap_point_change() {
            on_snap_point_change(snap_point, &mut details, window, cx);
        }
        if details.is_canceled() {
            return false;
        }
        if self.controlled_snap_point.is_none() {
            self.update(cx, |runtime| {
                runtime.set_snap_point_uncontrolled(snap_point)
            });
        }
        true
    }

    /// A type-erased reporter a nested drawer root uses to report presence,
    /// frontmost height, and live swipe state to this (parent) drawer.
    pub fn nested_reporter(&self) -> DrawerNestedReporter {
        let context = self.clone();
        DrawerNestedReporter {
            report: Rc::new(move |report: &DrawerNestedReport, cx: &mut App| {
                context.update(cx, |runtime| {
                    runtime.report_nested(
                        report.id.clone(),
                        report.present,
                        report.frontmost_height.map(f32::from),
                        report.swiping_movement,
                        report.swipe_progress,
                    );
                });
            }),
        }
    }
}

/// A nested drawer's report to its parent drawer.
#[derive(Clone, Debug)]
pub struct DrawerNestedReport {
    pub id: ElementId,
    pub present: bool,
    pub frontmost_height: Option<Pixels>,
    pub swiping_movement: f32,
    pub swipe_progress: f32,
}

/// Parent-linked reporting channel for nested drawers, type-erased so the
/// parent and nested drawer payload types may differ.
pub struct DrawerNestedReporter {
    report: Rc<dyn Fn(&DrawerNestedReport, &mut App) + 'static>,
}

impl Clone for DrawerNestedReporter {
    fn clone(&self) -> Self {
        Self {
            report: Rc::clone(&self.report),
        }
    }
}

impl DrawerNestedReporter {
    pub fn report(&self, report: &DrawerNestedReport, cx: &mut App) {
        (self.report)(report, cx);
    }
}
