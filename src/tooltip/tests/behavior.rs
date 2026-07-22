use std::{cell::RefCell, rc::Rc, time::Duration};

use gpui::{
    div, point, prelude::*, px, rgb, size, Modifiers, Pixels, Point, Render, TestAppContext,
    VisualTestContext, WindowHandle,
};

use crate::tooltip::{
    create_tooltip_handle,
    tests::support::{
        advance_clock, blur, click_outside_target, click_portal, click_trigger, debug_bounds,
        focus_next, move_mouse_to, move_over_selector, open_tooltip, read_observations,
        simulate_keys, TooltipTestConfig,
    },
    TooltipInstant, TooltipOpenChangeReason, TooltipPopup, TooltipPortal, TooltipPositioner,
    TooltipProvider, TooltipRoot, TooltipRootStyleState, TooltipSide, TooltipTrackCursorAxis,
    TooltipTrigger, TooltipViewport,
};

#[gpui::test]
fn hover_open_and_close_respect_configured_delays(cx: &mut TestAppContext) {
    let window = open_tooltip(
        cx,
        TooltipTestConfig {
            delay: Some(Duration::from_millis(50)),
            close_delay: Some(Duration::from_millis(75)),
            ..TooltipTestConfig::default()
        },
    );

    move_over_selector(cx, window, "tooltip-trigger");
    advance_clock(cx, Duration::from_millis(49));
    assert!(!read_observations(cx, window).root_state().unwrap().open);

    advance_clock(cx, Duration::from_millis(1));
    assert!(read_observations(cx, window).root_state().unwrap().open);

    move_over_selector(cx, window, "tooltip-outside-target");
    advance_clock(cx, Duration::from_millis(74));
    assert!(read_observations(cx, window).root_state().unwrap().open);

    advance_clock(cx, Duration::from_millis(1));
    let observations = read_observations(cx, window);
    assert!(!observations.root_state().unwrap().open);
    assert_eq!(
        observations.open_changes,
        vec![
            (true, TooltipOpenChangeReason::TriggerHover),
            (false, TooltipOpenChangeReason::TriggerHover),
        ]
    );
}

#[gpui::test]
fn provider_delay_applies_to_descendant_trigger(cx: &mut TestAppContext) {
    let window = open_tooltip(
        cx,
        TooltipTestConfig {
            delay: None,
            provider_delay: Duration::from_millis(40),
            ..TooltipTestConfig::default()
        },
    );

    move_over_selector(cx, window, "tooltip-trigger");
    advance_clock(cx, Duration::from_millis(39));
    assert!(!read_observations(cx, window).root_state().unwrap().open);

    advance_clock(cx, Duration::from_millis(1));
    assert!(read_observations(cx, window).root_state().unwrap().open);
}

#[gpui::test]
fn provider_recent_tooltip_opens_adjacent_trigger_immediately_until_timeout(
    cx: &mut TestAppContext,
) {
    let window = open_tooltip(
        cx,
        TooltipTestConfig {
            delay: None,
            provider_delay: Duration::from_millis(80),
            provider_timeout: Duration::from_millis(100),
            second_trigger: true,
            ..TooltipTestConfig::default()
        },
    );

    move_over_selector(cx, window, "tooltip-trigger");
    advance_clock(cx, Duration::from_millis(80));
    assert!(read_observations(cx, window).root_state().unwrap().open);

    move_over_selector(cx, window, "tooltip-outside-target");
    assert!(!read_observations(cx, window).root_state().unwrap().open);

    move_over_selector(cx, window, "tooltip-trigger-secondary");
    let observations = read_observations(cx, window);
    assert!(observations.root_state().unwrap().open);
    assert_eq!(
        observations.root_state().unwrap().instant,
        TooltipInstant::Instant
    );
    assert_eq!(
        observations.open_changes.last(),
        Some(&(true, TooltipOpenChangeReason::None))
    );

    move_over_selector(cx, window, "tooltip-outside-target");
    assert!(!read_observations(cx, window).root_state().unwrap().open);
    advance_clock(cx, Duration::from_millis(101));

    move_over_selector(cx, window, "tooltip-trigger");
    assert!(!read_observations(cx, window).root_state().unwrap().open);
    advance_clock(cx, Duration::from_millis(80));
    assert!(read_observations(cx, window).root_state().unwrap().open);
}

#[gpui::test]
fn rehover_cancels_delayed_hover_close(cx: &mut TestAppContext) {
    let window = open_tooltip(
        cx,
        TooltipTestConfig {
            close_delay: Some(Duration::from_millis(100)),
            ..TooltipTestConfig::default()
        },
    );

    move_over_selector(cx, window, "tooltip-trigger");
    assert!(read_observations(cx, window).root_state().unwrap().open);

    move_over_selector(cx, window, "tooltip-outside-target");
    advance_clock(cx, Duration::from_millis(50));
    assert!(read_observations(cx, window).root_state().unwrap().open);

    move_over_selector(cx, window, "tooltip-trigger");
    advance_clock(cx, Duration::from_millis(100));
    assert!(read_observations(cx, window).root_state().unwrap().open);
}

#[gpui::test]
fn moving_from_trigger_to_popup_keeps_tooltip_open(cx: &mut TestAppContext) {
    let window = open_tooltip(cx, TooltipTestConfig::default());

    move_over_selector(cx, window, "tooltip-trigger");
    assert!(read_observations(cx, window).root_state().unwrap().open);

    move_over_selector(cx, window, "tooltip-popup");
    assert!(read_observations(cx, window).root_state().unwrap().open);

    move_over_selector(cx, window, "tooltip-outside-target");
    let observations = read_observations(cx, window);
    assert!(!observations.root_state().unwrap().open);
}

#[gpui::test]
fn default_positioner_places_popup_body_against_trigger_edge(cx: &mut TestAppContext) {
    let window = open_tooltip(cx, TooltipTestConfig::default());

    move_over_selector(cx, window, "tooltip-trigger");
    assert!(read_observations(cx, window).root_state().unwrap().open);

    let trigger_bounds = debug_bounds(cx, window, "tooltip-trigger").unwrap();
    let popup_bounds = debug_bounds(cx, window, "tooltip-popup").unwrap();
    let touches_vertical_edge = popup_bounds.top() == trigger_bounds.bottom()
        || popup_bounds.bottom() == trigger_bounds.top();
    let touches_horizontal_edge = popup_bounds.left() == trigger_bounds.right()
        || popup_bounds.right() == trigger_bounds.left();

    assert!(touches_vertical_edge || touches_horizontal_edge);
}

#[gpui::test]
fn disable_hoverable_popup_does_not_keep_tooltip_open(cx: &mut TestAppContext) {
    let window = open_tooltip(
        cx,
        TooltipTestConfig {
            disable_hoverable_popup: true,
            ..TooltipTestConfig::default()
        },
    );

    move_over_selector(cx, window, "tooltip-trigger");
    assert!(read_observations(cx, window).root_state().unwrap().open);

    move_over_selector(cx, window, "tooltip-popup");
    let observations = read_observations(cx, window);
    assert!(!observations.root_state().unwrap().open);
}

#[gpui::test]
fn focus_opens_and_blur_closes(cx: &mut TestAppContext) {
    let window = open_tooltip(cx, TooltipTestConfig::default());

    focus_next(cx, window);
    let observations = read_observations(cx, window);
    assert!(observations.root_state().unwrap().open);
    assert_eq!(
        observations.open_changes,
        vec![(true, TooltipOpenChangeReason::TriggerFocus)]
    );

    blur(cx, window);
    let observations = read_observations(cx, window);
    assert!(!observations.root_state().unwrap().open);
    assert_eq!(
        observations.open_changes.last(),
        Some(&(false, TooltipOpenChangeReason::TriggerFocus))
    );
}

#[gpui::test]
fn escape_closes_when_trigger_has_focus(cx: &mut TestAppContext) {
    let window = open_tooltip(cx, TooltipTestConfig::default());

    focus_next(cx, window);
    simulate_keys(cx, window, "escape");

    let observations = read_observations(cx, window);
    assert!(!observations.root_state().unwrap().open);
    assert_eq!(
        observations.open_changes.last(),
        Some(&(false, TooltipOpenChangeReason::EscapeKey))
    );
}

#[gpui::test]
fn clicking_open_active_trigger_closes_by_default(cx: &mut TestAppContext) {
    let window = open_tooltip(cx, TooltipTestConfig::default());

    move_over_selector(cx, window, "tooltip-trigger");
    assert!(read_observations(cx, window).root_state().unwrap().open);

    click_trigger(cx, window);
    let observations = read_observations(cx, window);
    assert!(!observations.root_state().unwrap().open);
    assert_eq!(
        observations.open_changes.last(),
        Some(&(false, TooltipOpenChangeReason::TriggerPress))
    );
}

#[gpui::test]
fn clicking_open_active_trigger_can_keep_tooltip_open(cx: &mut TestAppContext) {
    let window = open_tooltip(
        cx,
        TooltipTestConfig {
            close_on_click: false,
            ..TooltipTestConfig::default()
        },
    );

    move_over_selector(cx, window, "tooltip-trigger");
    assert!(read_observations(cx, window).root_state().unwrap().open);

    click_trigger(cx, window);
    assert!(read_observations(cx, window).root_state().unwrap().open);
}

#[gpui::test]
fn controlled_root_reports_open_change_without_internal_mutation(cx: &mut TestAppContext) {
    let window = open_tooltip(
        cx,
        TooltipTestConfig {
            controlled_open: Some(false),
            ..TooltipTestConfig::default()
        },
    );

    move_over_selector(cx, window, "tooltip-trigger");

    let observations = read_observations(cx, window);
    assert!(!observations.root_state().unwrap().open);
    assert_eq!(
        observations.open_changes,
        vec![(true, TooltipOpenChangeReason::TriggerHover)]
    );
}

#[gpui::test]
fn canceled_uncontrolled_open_does_not_commit_or_complete(cx: &mut TestAppContext) {
    let window = open_tooltip(
        cx,
        TooltipTestConfig {
            cancel_open: true,
            ..TooltipTestConfig::default()
        },
    );

    move_over_selector(cx, window, "tooltip-trigger");

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.open_changes,
        vec![(true, TooltipOpenChangeReason::TriggerHover)]
    );
    assert!(observations.open_change_completes.is_empty());
    assert!(!observations.root_state().unwrap().open);
}

#[gpui::test]
fn canceled_uncontrolled_close_keeps_tooltip_open_without_complete(cx: &mut TestAppContext) {
    let window = open_tooltip(
        cx,
        TooltipTestConfig {
            cancel_close: true,
            ..TooltipTestConfig::default()
        },
    );

    move_over_selector(cx, window, "tooltip-trigger");
    move_over_selector(cx, window, "tooltip-outside-target");

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.open_changes,
        vec![
            (true, TooltipOpenChangeReason::TriggerHover),
            (false, TooltipOpenChangeReason::TriggerHover),
        ]
    );
    assert_eq!(
        observations.open_change_completes,
        vec![(true, TooltipOpenChangeReason::TriggerHover)]
    );
    assert!(observations.root_state().unwrap().open);
}

#[gpui::test]
fn keep_mounted_portal_reports_closed_mounted_noninteractive_popup(cx: &mut TestAppContext) {
    let window = open_tooltip(
        cx,
        TooltipTestConfig {
            keep_mounted: true,
            ..TooltipTestConfig::default()
        },
    );

    let observations = read_observations(cx, window);
    let popup_state = observations.popup_state().unwrap();
    assert!(!popup_state.open);
    assert!(popup_state.mounted);
    assert!(debug_bounds(cx, window, "tooltip-portal").is_some());

    click_portal(cx, window);
    let observations = read_observations(cx, window);
    assert_eq!(observations.portal_clicks, 0);
}

#[gpui::test]
fn outside_click_dismisses_open_tooltip(cx: &mut TestAppContext) {
    let window = open_tooltip(
        cx,
        TooltipTestConfig {
            default_open: true,
            ..TooltipTestConfig::default()
        },
    );

    click_outside_target(cx, window);

    let observations = read_observations(cx, window);
    assert!(!observations.root_state().unwrap().open);
    assert_eq!(
        observations.open_changes,
        vec![(false, TooltipOpenChangeReason::OutsidePress)]
    );
}

#[gpui::test]
fn detached_handle_opens_and_closes(cx: &mut TestAppContext) {
    let handle = create_tooltip_handle::<()>();
    let window = open_tooltip(
        cx,
        TooltipTestConfig {
            handle: Some(handle.clone()),
            ..TooltipTestConfig::default()
        },
    );

    window
        .update(cx, |_view, window, cx| {
            assert!(handle.open("tooltip-trigger", window, cx));
        })
        .expect("tooltip test window should be open");
    cx.run_until_parked();
    assert!(read_observations(cx, window).root_state().unwrap().open);

    window
        .update(cx, |_view, window, cx| {
            assert!(handle.close(window, cx));
        })
        .expect("tooltip test window should be open");
    cx.run_until_parked();

    let observations = read_observations(cx, window);
    assert!(!observations.root_state().unwrap().open);
    assert_eq!(
        observations.open_changes,
        vec![
            (true, TooltipOpenChangeReason::ImperativeAction),
            (false, TooltipOpenChangeReason::ImperativeAction),
        ]
    );
}

#[gpui::test]
fn multiple_triggers_update_active_state_and_viewport_direction(cx: &mut TestAppContext) {
    let handle = create_tooltip_handle::<()>();
    let window = open_tooltip(
        cx,
        TooltipTestConfig {
            handle: Some(handle.clone()),
            second_trigger: true,
            ..TooltipTestConfig::default()
        },
    );

    window
        .update(cx, |_view, window, cx| {
            assert!(handle.open("tooltip-trigger", window, cx));
            assert!(handle.open("tooltip-trigger-secondary", window, cx));
        })
        .expect("tooltip test window should be open");
    cx.run_until_parked();

    let observations = read_observations(cx, window);
    assert!(observations.root_state().unwrap().open);
    assert!(!observations.trigger_state().unwrap().active_trigger);
    assert!(observations.second_trigger_state().unwrap().active_trigger);
    assert_eq!(
        observations
            .viewport_states
            .last()
            .unwrap()
            .activation_direction,
        crate::tooltip::TooltipActivationDirection::Down
    );
}

#[gpui::test]
fn hovering_disabled_trigger_closes_previous_active_tooltip(cx: &mut TestAppContext) {
    let window = open_tooltip(
        cx,
        TooltipTestConfig {
            second_trigger: false,
            ..TooltipTestConfig::default()
        },
    );

    move_over_selector(cx, window, "tooltip-trigger");
    assert!(read_observations(cx, window).root_state().unwrap().open);

    move_over_selector(cx, window, "tooltip-trigger-secondary");
    let observations = read_observations(cx, window);
    assert!(!observations.root_state().unwrap().open);
    assert!(observations.second_trigger_state().unwrap().disabled);
    assert_eq!(
        observations.open_changes.last(),
        Some(&(false, TooltipOpenChangeReason::Disabled))
    );
}

#[gpui::test]
fn rendered_positioner_state_is_reported(cx: &mut TestAppContext) {
    let window = open_tooltip(
        cx,
        TooltipTestConfig {
            default_open: true,
            positioner_side: TooltipSide::Top,
            ..TooltipTestConfig::default()
        },
    );

    let observations = read_observations(cx, window);
    assert_eq!(
        observations
            .positioner_states
            .last()
            .unwrap()
            .transform_origin_y_percent,
        100.0
    );
    assert!(observations.positioner_states.last().unwrap().open);
}

#[gpui::test]
fn rendered_cursor_tracking_updates_positioner_anchor(cx: &mut TestAppContext) {
    let window = open_tooltip(
        cx,
        TooltipTestConfig {
            track_cursor_axis: TooltipTrackCursorAxis::Both,
            ..TooltipTestConfig::default()
        },
    );
    let trigger_bounds = debug_bounds(cx, window, "tooltip-trigger").unwrap();
    let cursor = gpui::point(
        trigger_bounds.left() + px(27.0),
        trigger_bounds.top() + px(11.0),
    );

    move_over_selector(cx, window, "tooltip-trigger");
    let _ = read_observations(cx, window);
    move_mouse_to(cx, window, cursor);

    let _ = read_observations(cx, window);
    let observations = read_observations(cx, window);
    let anchor_bounds = observations
        .positioner_states
        .last()
        .unwrap()
        .anchor_bounds
        .unwrap();
    assert_eq!(anchor_bounds.origin, cursor);
    assert_eq!(anchor_bounds.size, size(px(0.0), px(0.0)));
}

#[gpui::test]
fn multiple_detached_triggers_share_one_handle(cx: &mut TestAppContext) {
    let handle = create_tooltip_handle::<()>();
    let window = open_detached_tooltip(cx, handle.clone());

    window
        .update(cx, |_view, window, cx| {
            assert!(handle.open("detached-one", window, cx));
            assert!(handle.open("detached-two", window, cx));
        })
        .expect("detached tooltip test window should be open");
    cx.run_until_parked();

    let observations = read_detached_observations(cx, window);
    let root_state = observations.root_states.last().unwrap();
    assert!(root_state.open);
    assert_eq!(
        root_state
            .active_trigger_id
            .as_ref()
            .map(ToString::to_string),
        Some("detached-two".to_string())
    );
}

#[gpui::test]
fn active_detached_trigger_unmount_closes_uncontrolled_tooltip(cx: &mut TestAppContext) {
    let handle = create_tooltip_handle::<()>();
    let window = open_detached_tooltip(cx, handle.clone());

    window
        .update(cx, |_view, window, cx| {
            assert!(handle.open("detached-one", window, cx));
        })
        .expect("detached tooltip test window should be open");
    cx.run_until_parked();
    assert!(
        read_detached_observations(cx, window)
            .root_states
            .last()
            .unwrap()
            .open
    );

    window
        .update(cx, |view, _window, cx| {
            view.set_first_trigger_visible(false);
            cx.notify();
        })
        .expect("detached tooltip test window should be open");
    cx.run_until_parked();
    let _ = read_detached_observations(cx, window);
    let observations = read_detached_observations(cx, window);

    assert!(!observations.root_states.last().unwrap().open);
    assert_eq!(
        observations.open_changes.last(),
        Some(&(false, TooltipOpenChangeReason::TriggerHover))
    );
}

#[derive(Clone, Default)]
struct DetachedTooltipObservations {
    open_changes: Vec<(bool, TooltipOpenChangeReason)>,
    root_states: Vec<TooltipRootStyleState<()>>,
}

struct DetachedTooltipView {
    handle: crate::tooltip::TooltipHandle<()>,
    observations: Rc<RefCell<DetachedTooltipObservations>>,
    show_first_trigger: bool,
    show_second_trigger: bool,
}

impl DetachedTooltipView {
    fn new(handle: crate::tooltip::TooltipHandle<()>) -> Self {
        Self {
            handle,
            observations: Rc::new(RefCell::new(DetachedTooltipObservations::default())),
            show_first_trigger: true,
            show_second_trigger: true,
        }
    }

    fn set_first_trigger_visible(&mut self, visible: bool) {
        self.show_first_trigger = visible;
    }

    fn read_observations(&self) -> DetachedTooltipObservations {
        self.observations.borrow().clone()
    }
}

impl Render for DetachedTooltipView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().root_states.clear();
        let root_observations = Rc::clone(&self.observations);
        let open_observations = Rc::clone(&self.observations);
        let handle = self.handle.clone();

        let mut root = div().size_full().p_4().child(
            TooltipRoot::<()>::new()
                .id("detached-tooltip-root")
                .handle(handle.clone())
                .on_open_change(move |open, details, _window, _cx| {
                    open_observations
                        .borrow_mut()
                        .open_changes
                        .push((open, details.reason()));
                })
                .style_with_state(move |state, root| {
                    root_observations.borrow_mut().root_states.push(state);
                    root
                })
                .child(
                    TooltipPortal::<()>::new().child(
                        TooltipPositioner::<()>::new().side_offset(px(4.0)).child(
                            TooltipPopup::<()>::new()
                                .w(px(140.0))
                                .h(px(48.0))
                                .bg(rgb(0xffffff))
                                .child_any("Detached tooltip"),
                        ),
                    ),
                ),
        );

        if self.show_first_trigger {
            root = root.child(
                TooltipTrigger::<()>::new()
                    .id("detached-one")
                    .handle(handle.clone())
                    .absolute()
                    .top(px(24.0))
                    .left(px(24.0))
                    .w(px(120.0))
                    .h(px(32.0))
                    .border_1()
                    .border_color(rgb(0xd1d5db))
                    .child("Detached one"),
            );
        }

        if self.show_second_trigger {
            root = root.child(
                TooltipTrigger::<()>::new()
                    .id("detached-two")
                    .handle(handle)
                    .absolute()
                    .top(px(64.0))
                    .left(px(24.0))
                    .w(px(120.0))
                    .h(px(32.0))
                    .border_1()
                    .border_color(rgb(0xd1d5db))
                    .child("Detached two"),
            );
        }

        root
    }
}

fn open_detached_tooltip(
    cx: &mut TestAppContext,
    handle: crate::tooltip::TooltipHandle<()>,
) -> WindowHandle<DetachedTooltipView> {
    cx.update(crate::tooltip::init);
    let window = cx.open_window(size(px(640.0), px(360.0)), move |_, _| {
        DetachedTooltipView::new(handle)
    });
    cx.run_until_parked();
    window
}

fn read_detached_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<DetachedTooltipView>,
) -> DetachedTooltipObservations {
    window
        .update(cx, |view, _window, cx| {
            cx.notify();
            view.read_observations()
        })
        .expect("detached tooltip test window should be open");
    cx.run_until_parked();
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("detached tooltip test window should be open")
}

#[gpui::test]
fn viewport_payload_content_tracks_programmatic_trigger_switches(cx: &mut TestAppContext) {
    let handle = create_tooltip_handle::<String>();
    let window = open_payload_viewport_tooltip(cx, handle.clone());

    window
        .update(cx, |_view, window, cx| {
            assert!(handle.open("first", window, cx));
        })
        .expect("payload viewport tooltip test window should be open");
    cx.run_until_parked();
    let observations = read_payload_viewport_observations(cx, window);
    assert!(observations.root_states.last().unwrap().open);
    assert_eq!(
        observations
            .root_states
            .last()
            .unwrap()
            .active_payload
            .as_deref(),
        Some("alpha")
    );
    assert_eq!(
        observations
            .viewport_payloads
            .last()
            .cloned()
            .flatten()
            .as_deref(),
        Some("alpha")
    );

    window
        .update(cx, |_view, window, cx| {
            assert!(handle.open("second", window, cx));
        })
        .expect("payload viewport tooltip test window should be open");
    cx.run_until_parked();
    let observations = read_payload_viewport_observations(cx, window);
    assert!(observations.root_states.last().unwrap().open);
    assert_eq!(
        observations
            .root_states
            .last()
            .unwrap()
            .active_payload
            .as_deref(),
        Some("beta")
    );
    assert_eq!(
        observations
            .viewport_payloads
            .last()
            .cloned()
            .flatten()
            .as_deref(),
        Some("beta")
    );

    window
        .update(cx, |_view, window, cx| {
            assert!(handle.close(window, cx));
        })
        .expect("payload viewport tooltip test window should be open");
    cx.run_until_parked();
    assert!(
        !read_payload_viewport_observations(cx, window)
            .root_states
            .last()
            .unwrap()
            .open
    );
}

#[derive(Clone, Default)]
struct PayloadViewportObservations {
    root_states: Vec<TooltipRootStyleState<String>>,
    viewport_payloads: Vec<Option<String>>,
}

struct PayloadViewportView {
    handle: crate::tooltip::TooltipHandle<String>,
    observations: Rc<RefCell<PayloadViewportObservations>>,
}

impl PayloadViewportView {
    fn new(handle: crate::tooltip::TooltipHandle<String>) -> Self {
        Self {
            handle,
            observations: Rc::new(RefCell::new(PayloadViewportObservations::default())),
        }
    }

    fn read_observations(&self) -> PayloadViewportObservations {
        self.observations.borrow().clone()
    }
}

impl Render for PayloadViewportView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().root_states.clear();
        let root_observations = Rc::clone(&self.observations);
        let payload_observations = Rc::clone(&self.observations);
        let handle = self.handle.clone();

        TooltipRoot::<String>::new()
            .id("payload-viewport-tooltip")
            .handle(handle)
            .style_with_state(move |state, root| {
                root_observations.borrow_mut().root_states.push(state);
                root
            })
            .child(
                TooltipTrigger::<String>::new()
                    .id("first")
                    .payload("alpha".to_string())
                    .w(px(80.0))
                    .h(px(32.0))
                    .child("First"),
            )
            .child(
                TooltipTrigger::<String>::new()
                    .id("second")
                    .payload("beta".to_string())
                    .w(px(80.0))
                    .h(px(32.0))
                    .child("Second"),
            )
            .child(
                TooltipPortal::<String>::new().child(TooltipPositioner::<String>::new().child(
                    TooltipPopup::<String>::new().child(
                        TooltipViewport::<String>::new().payload_content(
                            move |payload: Option<&String>, _window, _cx| {
                                payload_observations
                                    .borrow_mut()
                                    .viewport_payloads
                                    .push(payload.cloned());
                                div()
                                    .child(payload.cloned().unwrap_or_else(|| "none".to_string()))
                                    .into_any_element()
                            },
                        ),
                    ),
                )),
            )
    }
}

fn open_payload_viewport_tooltip(
    cx: &mut TestAppContext,
    handle: crate::tooltip::TooltipHandle<String>,
) -> WindowHandle<PayloadViewportView> {
    cx.update(crate::tooltip::init);
    let window = cx.open_window(size(px(640.0), px(360.0)), move |_, _| {
        PayloadViewportView::new(handle)
    });
    cx.run_until_parked();
    window
}

fn read_payload_viewport_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<PayloadViewportView>,
) -> PayloadViewportObservations {
    window
        .update(cx, |view, _window, cx| {
            cx.notify();
            view.read_observations()
        })
        .expect("payload viewport tooltip test window should be open");
    cx.run_until_parked();
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("payload viewport tooltip test window should be open")
}

#[gpui::test]
fn controlled_programmatic_trigger_switch_preserves_payload(cx: &mut TestAppContext) {
    let window = open_controlled_payload_tooltip(cx);

    set_controlled_payload_state(cx, window, true, Some("first"));
    let observations = read_controlled_payload_observations(cx, window);
    let root_state = observations.root_states.last().unwrap();
    assert!(root_state.open);
    assert_eq!(root_state.active_payload.as_deref(), Some("alpha"));

    set_controlled_payload_state(cx, window, true, Some("second"));
    let observations = read_controlled_payload_observations(cx, window);
    let root_state = observations.root_states.last().unwrap();
    assert!(root_state.open);
    assert_eq!(root_state.active_payload.as_deref(), Some("beta"));

    set_controlled_payload_state(cx, window, false, None);
    let observations = read_controlled_payload_observations(cx, window);
    let root_state = observations.root_states.last().unwrap();
    assert!(!root_state.open);
    assert!(root_state.active_payload.is_none());
}

#[derive(Clone, Default)]
struct ControlledPayloadObservations {
    root_states: Vec<TooltipRootStyleState<String>>,
}

struct ControlledPayloadView {
    open: bool,
    trigger_id: Option<&'static str>,
    observations: Rc<RefCell<ControlledPayloadObservations>>,
}

impl ControlledPayloadView {
    fn new() -> Self {
        Self {
            open: false,
            trigger_id: None,
            observations: Rc::new(RefCell::new(ControlledPayloadObservations::default())),
        }
    }

    fn set_state(&mut self, open: bool, trigger_id: Option<&'static str>) {
        self.open = open;
        self.trigger_id = trigger_id;
    }

    fn read_observations(&self) -> ControlledPayloadObservations {
        self.observations.borrow().clone()
    }
}

impl Render for ControlledPayloadView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().root_states.clear();
        let root_observations = Rc::clone(&self.observations);

        let mut root = TooltipRoot::<String>::new()
            .id("controlled-payload-tooltip")
            .open(self.open)
            .style_with_state(move |state, root| {
                root_observations.borrow_mut().root_states.push(state);
                root
            })
            .child(
                TooltipTrigger::<String>::new()
                    .id("first")
                    .payload("alpha".to_string())
                    .w(px(80.0))
                    .h(px(32.0))
                    .child("First"),
            )
            .child(
                TooltipTrigger::<String>::new()
                    .id("second")
                    .payload("beta".to_string())
                    .w(px(80.0))
                    .h(px(32.0))
                    .child("Second"),
            )
            .child(
                TooltipPortal::<String>::new().child(
                    TooltipPositioner::<String>::new().child(
                        TooltipPopup::<String>::new()
                            .child(TooltipViewport::<String>::new().child("Controlled payload")),
                    ),
                ),
            );

        if let Some(trigger_id) = self.trigger_id {
            root = root.trigger_id(trigger_id);
        } else {
            root = root.no_trigger_id();
        }

        root
    }
}

fn open_controlled_payload_tooltip(cx: &mut TestAppContext) -> WindowHandle<ControlledPayloadView> {
    cx.update(crate::tooltip::init);
    let window = cx.open_window(size(px(640.0), px(360.0)), move |_, _| {
        ControlledPayloadView::new()
    });
    cx.run_until_parked();
    window
}

fn set_controlled_payload_state(
    cx: &mut TestAppContext,
    window: WindowHandle<ControlledPayloadView>,
    open: bool,
    trigger_id: Option<&'static str>,
) {
    window
        .update(cx, |view, _window, cx| {
            view.set_state(open, trigger_id);
            cx.notify();
        })
        .expect("controlled payload tooltip test window should be open");
    cx.run_until_parked();
}

fn read_controlled_payload_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<ControlledPayloadView>,
) -> ControlledPayloadObservations {
    window
        .update(cx, |view, _window, cx| {
            cx.notify();
            view.read_observations()
        })
        .expect("controlled payload tooltip test window should be open");
    cx.run_until_parked();
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("controlled payload tooltip test window should be open")
}

#[gpui::test]
fn opening_provider_sibling_closes_previous_tooltip_instantly(cx: &mut TestAppContext) {
    let first_handle = create_tooltip_handle::<()>();
    let second_handle = create_tooltip_handle::<()>();
    let window = open_sibling_provider_tooltip(cx, first_handle.clone(), second_handle.clone());

    window
        .update(cx, |_view, window, cx| {
            assert!(first_handle.open("first-trigger", window, cx));
        })
        .expect("sibling provider tooltip test window should be open");
    cx.run_until_parked();
    assert!(
        read_sibling_provider_observations(cx, window)
            .first_root_states
            .last()
            .unwrap()
            .open
    );

    window
        .update(cx, |_view, window, cx| {
            assert!(second_handle.open("second-trigger", window, cx));
        })
        .expect("sibling provider tooltip test window should be open");
    cx.run_until_parked();
    let _ = read_sibling_provider_observations(cx, window);
    let observations = read_sibling_provider_observations(cx, window);

    let first_state = observations.first_root_states.last().unwrap();
    let second_state = observations.second_root_states.last().unwrap();
    assert!(!first_state.open);
    assert!(second_state.open);
    assert_eq!(first_state.instant, TooltipInstant::Instant);
    assert_eq!(
        observations.first_open_changes.last(),
        Some(&(false, TooltipOpenChangeReason::None))
    );
}

#[derive(Clone, Default)]
struct SiblingProviderObservations {
    first_root_states: Vec<TooltipRootStyleState<()>>,
    second_root_states: Vec<TooltipRootStyleState<()>>,
    first_open_changes: Vec<(bool, TooltipOpenChangeReason)>,
    second_open_changes: Vec<(bool, TooltipOpenChangeReason)>,
}

struct SiblingProviderView {
    first_handle: crate::tooltip::TooltipHandle<()>,
    second_handle: crate::tooltip::TooltipHandle<()>,
    observations: Rc<RefCell<SiblingProviderObservations>>,
}

impl SiblingProviderView {
    fn new(
        first_handle: crate::tooltip::TooltipHandle<()>,
        second_handle: crate::tooltip::TooltipHandle<()>,
    ) -> Self {
        Self {
            first_handle,
            second_handle,
            observations: Rc::new(RefCell::new(SiblingProviderObservations::default())),
        }
    }

    fn read_observations(&self) -> SiblingProviderObservations {
        self.observations.borrow().clone()
    }
}

impl Render for SiblingProviderView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().first_root_states.clear();
        self.observations.borrow_mut().second_root_states.clear();
        let first_state_observations = Rc::clone(&self.observations);
        let second_state_observations = Rc::clone(&self.observations);
        let first_change_observations = Rc::clone(&self.observations);
        let second_change_observations = Rc::clone(&self.observations);

        TooltipProvider::<()>::new()
            .id("sibling-provider")
            .flex()
            .items_center()
            .gap_2()
            .child(
                TooltipRoot::<()>::new()
                    .id("first-sibling-tooltip")
                    .handle(self.first_handle.clone())
                    .on_open_change(move |open, details, _window, _cx| {
                        first_change_observations
                            .borrow_mut()
                            .first_open_changes
                            .push((open, details.reason()));
                    })
                    .style_with_state(move |state, root| {
                        first_state_observations
                            .borrow_mut()
                            .first_root_states
                            .push(state);
                        root
                    })
                    .child(
                        TooltipTrigger::<()>::new()
                            .id("first-trigger")
                            .w(px(80.0))
                            .h(px(32.0))
                            .child("First"),
                    )
                    .child(sibling_popup("First tooltip")),
            )
            .child(
                TooltipRoot::<()>::new()
                    .id("second-sibling-tooltip")
                    .handle(self.second_handle.clone())
                    .on_open_change(move |open, details, _window, _cx| {
                        second_change_observations
                            .borrow_mut()
                            .second_open_changes
                            .push((open, details.reason()));
                    })
                    .style_with_state(move |state, root| {
                        second_state_observations
                            .borrow_mut()
                            .second_root_states
                            .push(state);
                        root
                    })
                    .child(
                        TooltipTrigger::<()>::new()
                            .id("second-trigger")
                            .w(px(80.0))
                            .h(px(32.0))
                            .child("Second"),
                    )
                    .child(sibling_popup("Second tooltip")),
            )
    }
}

fn sibling_popup(content: &'static str) -> TooltipPortal<()> {
    TooltipPortal::<()>::new().child(
        TooltipPositioner::<()>::new().child(
            TooltipPopup::<()>::new()
                .w(px(120.0))
                .h(px(40.0))
                .bg(rgb(0xffffff))
                .child_any(content),
        ),
    )
}

fn open_sibling_provider_tooltip(
    cx: &mut TestAppContext,
    first_handle: crate::tooltip::TooltipHandle<()>,
    second_handle: crate::tooltip::TooltipHandle<()>,
) -> WindowHandle<SiblingProviderView> {
    cx.update(crate::tooltip::init);
    let window = cx.open_window(size(px(640.0), px(360.0)), move |_, _| {
        SiblingProviderView::new(first_handle, second_handle)
    });
    cx.run_until_parked();
    window
}

fn read_sibling_provider_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<SiblingProviderView>,
) -> SiblingProviderObservations {
    window
        .update(cx, |view, _window, cx| {
            cx.notify();
            view.read_observations()
        })
        .expect("sibling provider tooltip test window should be open");
    cx.run_until_parked();
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("sibling provider tooltip test window should be open")
}

#[gpui::test]
fn nested_trigger_hover_suppresses_parent_tooltip(cx: &mut TestAppContext) {
    let window = open_nested_tooltip(cx, NestedTooltipConfig::default());

    move_nested_over_selector(cx, window, "child-two-trigger");
    let observations = read_nested_observations(cx, window);

    assert!(!observations.parent_state().unwrap().open);
    assert!(observations.child_two_state().unwrap().open);
}

#[gpui::test]
fn moving_between_sibling_nested_triggers_does_not_open_parent_tooltip(cx: &mut TestAppContext) {
    let window = open_nested_tooltip(cx, NestedTooltipConfig::default());

    move_nested_over_selector(cx, window, "child-one-label");
    move_nested_over_selector(cx, window, "child-two-trigger");
    let observations = read_nested_observations(cx, window);

    assert!(!observations.parent_state().unwrap().open);
    assert!(!observations.child_one_state().unwrap().open);
    assert!(observations.child_two_state().unwrap().open);
}

#[gpui::test]
fn third_level_nested_trigger_suppresses_all_ancestor_tooltips(cx: &mut TestAppContext) {
    let window = open_nested_tooltip(cx, NestedTooltipConfig::default());

    move_nested_over_selector(cx, window, "grandchild-trigger");
    let observations = read_nested_observations(cx, window);

    assert!(!observations.parent_state().unwrap().open);
    assert!(!observations.child_one_state().unwrap().open);
    assert!(observations.grandchild_state().unwrap().open);
}

#[gpui::test]
fn moving_from_nested_trigger_back_to_parent_area_reopens_parent_after_delay(
    cx: &mut TestAppContext,
) {
    let window = open_nested_tooltip(
        cx,
        NestedTooltipConfig {
            parent_delay: Duration::from_millis(50),
            ..NestedTooltipConfig::default()
        },
    );

    move_nested_over_selector(cx, window, "child-two-trigger");
    assert!(
        !read_nested_observations(cx, window)
            .parent_state()
            .unwrap()
            .open
    );

    move_nested_to_parent_area(cx, window);
    advance_clock(cx, Duration::from_millis(49));
    assert!(
        !read_nested_observations(cx, window)
            .parent_state()
            .unwrap()
            .open
    );

    advance_clock(cx, Duration::from_millis(1));
    let observations = read_nested_observations(cx, window);
    assert!(observations.parent_state().unwrap().open);
    assert!(!observations.child_two_state().unwrap().open);
}

#[gpui::test]
fn pending_parent_reopen_is_canceled_when_pointer_leaves_parent_trigger(cx: &mut TestAppContext) {
    let window = open_nested_tooltip(
        cx,
        NestedTooltipConfig {
            parent_delay: Duration::from_millis(50),
            ..NestedTooltipConfig::default()
        },
    );

    move_nested_over_selector(cx, window, "child-two-trigger");
    move_nested_to_parent_area(cx, window);
    advance_clock(cx, Duration::from_millis(25));
    move_nested_over_selector(cx, window, "nested-outside-target");
    advance_clock(cx, Duration::from_millis(50));

    assert!(
        !read_nested_observations(cx, window)
            .parent_state()
            .unwrap()
            .open
    );
}

#[gpui::test]
fn pending_parent_reopen_is_canceled_when_pointer_moves_back_to_nested_trigger(
    cx: &mut TestAppContext,
) {
    let window = open_nested_tooltip(
        cx,
        NestedTooltipConfig {
            parent_delay: Duration::from_millis(50),
            ..NestedTooltipConfig::default()
        },
    );

    move_nested_to_parent_area(cx, window);
    advance_clock(cx, Duration::from_millis(25));
    move_nested_over_selector(cx, window, "child-two-trigger");
    advance_clock(cx, Duration::from_millis(50));
    let observations = read_nested_observations(cx, window);

    assert!(!observations.parent_state().unwrap().open);
    assert!(observations.child_two_state().unwrap().open);
}

#[gpui::test]
fn hovering_nested_trigger_closes_hover_opened_parent_tooltip(cx: &mut TestAppContext) {
    let window = open_nested_tooltip(cx, NestedTooltipConfig::default());

    move_nested_to_parent_area(cx, window);
    assert!(
        read_nested_observations(cx, window)
            .parent_state()
            .unwrap()
            .open
    );

    move_nested_over_selector(cx, window, "child-two-trigger");
    let observations = read_nested_observations(cx, window);

    assert!(!observations.parent_state().unwrap().open);
    assert!(observations.child_two_state().unwrap().open);
    assert_eq!(
        observations.parent_open_changes.last(),
        Some(&(false, TooltipOpenChangeReason::TriggerHover))
    );
}

#[gpui::test]
fn hovering_nested_trigger_preserves_focus_opened_parent_tooltip(cx: &mut TestAppContext) {
    let window = open_nested_tooltip(cx, NestedTooltipConfig::default());

    focus_nested_next(cx, window);
    let observations = read_nested_observations(cx, window);
    assert!(observations.parent_state().unwrap().open);
    assert_eq!(
        observations.parent_open_changes.last(),
        Some(&(true, TooltipOpenChangeReason::TriggerFocus))
    );

    move_nested_over_selector(cx, window, "child-two-trigger");
    let observations = read_nested_observations(cx, window);

    assert!(observations.parent_state().unwrap().open);
    assert!(observations.child_two_state().unwrap().open);
    assert_ne!(
        observations.parent_open_changes.last(),
        Some(&(false, TooltipOpenChangeReason::TriggerHover))
    );
}

#[gpui::test]
fn hovering_nested_trigger_preserves_controlled_open_parent_tooltip(cx: &mut TestAppContext) {
    let window = open_nested_tooltip(
        cx,
        NestedTooltipConfig {
            parent_controlled_open: Some(true),
            ..NestedTooltipConfig::default()
        },
    );

    move_nested_over_selector(cx, window, "child-two-trigger");
    let observations = read_nested_observations(cx, window);

    assert!(observations.parent_state().unwrap().open);
    assert!(observations.child_two_state().unwrap().open);
}

#[gpui::test]
fn focusing_nested_trigger_does_not_open_parent_tooltip(cx: &mut TestAppContext) {
    let window = open_nested_tooltip(cx, NestedTooltipConfig::default());

    focus_nested_next(cx, window);
    focus_nested_next(cx, window);
    let observations = read_nested_observations(cx, window);

    assert!(!observations.parent_state().unwrap().open);
    assert!(observations.child_one_state().unwrap().open);
}

#[derive(Clone)]
struct NestedTooltipConfig {
    parent_delay: Duration,
    parent_controlled_open: Option<bool>,
}

impl Default for NestedTooltipConfig {
    fn default() -> Self {
        Self {
            parent_delay: Duration::ZERO,
            parent_controlled_open: None,
        }
    }
}

#[derive(Clone, Default)]
struct NestedTooltipObservations {
    parent_states: Vec<TooltipRootStyleState<()>>,
    child_one_states: Vec<TooltipRootStyleState<()>>,
    child_two_states: Vec<TooltipRootStyleState<()>>,
    grandchild_states: Vec<TooltipRootStyleState<()>>,
    parent_open_changes: Vec<(bool, TooltipOpenChangeReason)>,
}

impl NestedTooltipObservations {
    fn begin_render(&mut self) {
        self.parent_states.clear();
        self.child_one_states.clear();
        self.child_two_states.clear();
        self.grandchild_states.clear();
    }

    fn parent_state(&self) -> Option<TooltipRootStyleState<()>> {
        self.parent_states.last().cloned()
    }

    fn child_one_state(&self) -> Option<TooltipRootStyleState<()>> {
        self.child_one_states.last().cloned()
    }

    fn child_two_state(&self) -> Option<TooltipRootStyleState<()>> {
        self.child_two_states.last().cloned()
    }

    fn grandchild_state(&self) -> Option<TooltipRootStyleState<()>> {
        self.grandchild_states.last().cloned()
    }
}

struct NestedTooltipView {
    config: NestedTooltipConfig,
    observations: Rc<RefCell<NestedTooltipObservations>>,
}

impl NestedTooltipView {
    fn new(config: NestedTooltipConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(NestedTooltipObservations::default())),
        }
    }

    fn read_observations(&self) -> NestedTooltipObservations {
        self.observations.borrow().clone()
    }
}

impl Render for NestedTooltipView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().begin_render();
        let parent_state_observations = Rc::clone(&self.observations);
        let child_one_state_observations = Rc::clone(&self.observations);
        let child_two_state_observations = Rc::clone(&self.observations);
        let grandchild_state_observations = Rc::clone(&self.observations);
        let parent_change_observations = Rc::clone(&self.observations);

        let mut parent_root = TooltipRoot::<()>::new()
            .id("nested-parent-tooltip")
            .on_open_change(move |open, details, _window, _cx| {
                parent_change_observations
                    .borrow_mut()
                    .parent_open_changes
                    .push((open, details.reason()));
            })
            .style_with_state(move |state, root| {
                parent_state_observations
                    .borrow_mut()
                    .parent_states
                    .push(state);
                root
            });
        if let Some(open) = self.config.parent_controlled_open {
            parent_root = parent_root.open(open);
        }

        TooltipProvider::<()>::new()
            .id("nested-tooltip-provider")
            .child(
                parent_root
                    .child(
                        TooltipTrigger::<()>::new()
                            .id("parent-trigger")
                            .delay(self.config.parent_delay)
                            .close_delay(Duration::ZERO)
                            .w(px(300.0))
                            .h(px(96.0))
                            .p_2()
                            .flex()
                            .items_center()
                            .gap_2()
                            .border_1()
                            .style_with_state(|_state, trigger| {
                                trigger.debug_selector(|| "parent-trigger".into())
                            })
                            .child(
                                div()
                                    .w(px(64.0))
                                    .h(px(32.0))
                                    .debug_selector(|| "parent-trigger-area".into())
                                    .child("Parent"),
                            )
                            .child(nested_child_one(
                                child_one_state_observations,
                                grandchild_state_observations,
                            ))
                            .child(nested_child_two(child_two_state_observations)),
                    )
                    .child(nested_popup("Parent tooltip")),
            )
            .child_any(
                div()
                    .absolute()
                    .top(px(220.0))
                    .left(px(16.0))
                    .w(px(80.0))
                    .h(px(32.0))
                    .debug_selector(|| "nested-outside-target".into())
                    .child("Outside"),
            )
    }
}

fn nested_child_one(
    child_one_state_observations: Rc<RefCell<NestedTooltipObservations>>,
    grandchild_state_observations: Rc<RefCell<NestedTooltipObservations>>,
) -> TooltipRoot<()> {
    TooltipRoot::<()>::new()
        .id("nested-child-one-tooltip")
        .style_with_state(move |state, root| {
            child_one_state_observations
                .borrow_mut()
                .child_one_states
                .push(state);
            root
        })
        .child(
            TooltipTrigger::<()>::new()
                .id("child-one-trigger")
                .delay(Duration::ZERO)
                .close_delay(Duration::ZERO)
                .w(px(104.0))
                .h(px(56.0))
                .p_1()
                .flex()
                .items_center()
                .gap_1()
                .border_1()
                .style_with_state(|_state, trigger| {
                    trigger.debug_selector(|| "child-one-trigger".into())
                })
                .child(
                    div()
                        .w(px(48.0))
                        .h(px(32.0))
                        .debug_selector(|| "child-one-label".into())
                        .child("Child 1"),
                )
                .child(
                    TooltipRoot::<()>::new()
                        .id("nested-grandchild-tooltip")
                        .style_with_state(move |state, root| {
                            grandchild_state_observations
                                .borrow_mut()
                                .grandchild_states
                                .push(state);
                            root
                        })
                        .child(
                            TooltipTrigger::<()>::new()
                                .id("grandchild-trigger")
                                .delay(Duration::ZERO)
                                .close_delay(Duration::ZERO)
                                .w(px(36.0))
                                .h(px(32.0))
                                .border_1()
                                .style_with_state(|_state, trigger| {
                                    trigger.debug_selector(|| "grandchild-trigger".into())
                                })
                                .child("G"),
                        )
                        .child(nested_popup("Grandchild tooltip")),
                ),
        )
        .child(nested_popup("Child one tooltip"))
}

fn nested_child_two(
    child_two_state_observations: Rc<RefCell<NestedTooltipObservations>>,
) -> TooltipRoot<()> {
    TooltipRoot::<()>::new()
        .id("nested-child-two-tooltip")
        .style_with_state(move |state, root| {
            child_two_state_observations
                .borrow_mut()
                .child_two_states
                .push(state);
            root
        })
        .child(
            TooltipTrigger::<()>::new()
                .id("child-two-trigger")
                .delay(Duration::ZERO)
                .close_delay(Duration::ZERO)
                .w(px(88.0))
                .h(px(56.0))
                .border_1()
                .style_with_state(|_state, trigger| {
                    trigger.debug_selector(|| "child-two-trigger".into())
                })
                .child("Child 2"),
        )
        .child(nested_popup("Child two tooltip"))
}

fn nested_popup(content: &'static str) -> TooltipPortal<()> {
    TooltipPortal::<()>::new().child(
        TooltipPositioner::<()>::new().child(
            TooltipPopup::<()>::new()
                .w(px(120.0))
                .h(px(40.0))
                .bg(rgb(0xffffff))
                .child_any(content),
        ),
    )
}

fn open_nested_tooltip(
    cx: &mut TestAppContext,
    config: NestedTooltipConfig,
) -> WindowHandle<NestedTooltipView> {
    cx.update(crate::tooltip::init);
    let window = cx.open_window(size(px(640.0), px(360.0)), move |_, _| {
        NestedTooltipView::new(config)
    });
    cx.run_until_parked();
    window
}

fn read_nested_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<NestedTooltipView>,
) -> NestedTooltipObservations {
    window
        .update(cx, |view, _window, cx| {
            cx.notify();
            view.read_observations()
        })
        .expect("nested tooltip test window should be open");
    cx.run_until_parked();
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("nested tooltip test window should be open")
}

fn focus_nested_next(cx: &mut TestAppContext, window: WindowHandle<NestedTooltipView>) {
    window
        .update(cx, |_view, window, cx| {
            window.focus_next(cx);
        })
        .expect("nested tooltip test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

fn move_nested_over_selector(
    cx: &mut TestAppContext,
    window: WindowHandle<NestedTooltipView>,
    selector: &'static str,
) {
    let position = nested_debug_bounds(cx, window, selector).center();
    move_nested_mouse_to(cx, window, position);
}

fn move_nested_to_parent_area(cx: &mut TestAppContext, window: WindowHandle<NestedTooltipView>) {
    let bounds = nested_debug_bounds(cx, window, "parent-trigger-area");
    move_nested_mouse_to(cx, window, bounds.origin + point(px(8.0), px(8.0)));
}

fn move_nested_mouse_to(
    cx: &mut TestAppContext,
    window: WindowHandle<NestedTooltipView>,
    position: Point<Pixels>,
) {
    let mut visual = VisualTestContext::from_window(window.into(), cx);
    visual.simulate_mouse_move(position, None, Modifiers::default());
    visual.run_until_parked();
    cx.run_until_parked();
    cx.run_until_parked();
}

fn nested_debug_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<NestedTooltipView>,
    selector: &'static str,
) -> gpui::Bounds<Pixels> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);
    visual
        .debug_bounds(selector)
        .expect("debug bounds should exist")
}

#[gpui::test]
fn provider_default_delays_close_after_hover_away(cx: &mut TestAppContext) {
    // Mirrors the gallery demo: no per-trigger delay overrides, so the
    // provider's 600ms open delay and zero close delay apply.
    let window = open_tooltip(
        cx,
        TooltipTestConfig {
            delay: None,
            close_delay: None,
            ..TooltipTestConfig::default()
        },
    );

    move_over_selector(cx, window, "tooltip-trigger");
    advance_clock(cx, Duration::from_millis(600));
    assert!(read_observations(cx, window).root_state().unwrap().open);

    move_over_selector(cx, window, "tooltip-outside-target");
    advance_clock(cx, Duration::from_millis(1000));
    assert!(!read_observations(cx, window).root_state().unwrap().open);
}

#[gpui::test]
fn pointer_focused_trigger_does_not_pin_tooltip_open(cx: &mut TestAppContext) {
    // Clicking focuses the trigger; in GPUI nothing blurs it when the
    // pointer later clicks empty space, so pointer-sourced focus must not
    // hold the tooltip open after the pointer leaves the trigger.
    let window = open_tooltip(cx, TooltipTestConfig::default());

    move_over_selector(cx, window, "tooltip-trigger");
    assert!(read_observations(cx, window).root_state().unwrap().open);

    click_trigger(cx, window);
    assert!(!read_observations(cx, window).root_state().unwrap().open);

    move_over_selector(cx, window, "tooltip-outside-target");
    move_over_selector(cx, window, "tooltip-trigger");
    assert!(read_observations(cx, window).root_state().unwrap().open);

    move_over_selector(cx, window, "tooltip-outside-target");
    advance_clock(cx, Duration::from_millis(1000));
    assert!(!read_observations(cx, window).root_state().unwrap().open);
}
