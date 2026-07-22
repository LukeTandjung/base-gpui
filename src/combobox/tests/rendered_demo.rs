use std::{cell::RefCell, rc::Rc};

use gpui::{
    div, prelude::*, px, size, Bounds, IntoElement, Modifiers, Pixels, Render, TestAppContext,
    VisualTestContext, WindowHandle,
};

use crate::combobox::{
    ComboboxClear, ComboboxIcon, ComboboxInput, ComboboxInputGroup, ComboboxItem,
    ComboboxItemIndicator, ComboboxList, ComboboxPopup, ComboboxPortal, ComboboxPositioner,
    ComboboxRoot, ComboboxTrigger,
};
use crate::primitives::input::InputStyleState;

/// Windowed regression harness that mirrors the gallery's single-select
/// combobox card: input group with input, clear, and trigger, plus the
/// portal/positioner/popup/list/item stack.
pub struct ComboboxDemoView {
    input_states: Rc<RefCell<Vec<InputStyleState>>>,
}

impl ComboboxDemoView {
    fn new() -> Self {
        Self {
            input_states: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn last_input_state(&self) -> Option<InputStyleState> {
        self.input_states.borrow().last().cloned()
    }
}

impl Render for ComboboxDemoView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        let input_states = Rc::clone(&self.input_states);

        div().size_full().p_4().child(
            ComboboxRoot::<&'static str>::new()
                .id("demo-combobox")
                .item_to_string_value(|value| (*value).into())
                .flex()
                .flex_col()
                .gap_2()
                .w(px(300.0))
                .child(
                    ComboboxInputGroup::new()
                        .w_full()
                        .h(px(34.0))
                        .px_2()
                        .flex()
                        .items_center()
                        .gap_1()
                        .child(
                            ComboboxInput::new()
                                .id("demo-combobox-input")
                                .placeholder("Search fruits…")
                                .flex_1()
                                .style_with_state(move |_state, base| {
                                    base.debug_selector(|| "demo-combobox-input-area".into())
                                })
                                .input_style_with_state(move |state, base| {
                                    input_states.borrow_mut().push(state);
                                    base
                                }),
                        )
                        .child(ComboboxClear::new())
                        .child(
                            ComboboxTrigger::new()
                                .id("demo-combobox-trigger")
                                .child(ComboboxIcon::<&'static str>::new()),
                        ),
                )
                .child(
                    ComboboxPortal::new().child(
                        ComboboxPositioner::new().side_offset(px(4.0)).child(
                            ComboboxPopup::new().w(px(220.0)).child(
                                ComboboxList::new()
                                    .flex()
                                    .flex_col()
                                    .child(demo_item("apple", "Apple"))
                                    .child(demo_item("banana", "Banana")),
                            ),
                        ),
                    ),
                ),
        )
    }
}

fn demo_item(value: &'static str, label: &'static str) -> ComboboxItem<&'static str> {
    ComboboxItem::new()
        .id(format!("demo-item-{value}"))
        .value(value)
        .label(label)
        .px_2()
        .py_1()
        .flex()
        .items_center()
        .gap_2()
        .style_with_state(move |_state, item| {
            item.debug_selector(move || format!("demo-item-{value}"))
        })
        .child(ComboboxItemIndicator::new().keep_mounted(true).w(px(14.0)))
        .child_any(div().child(label))
}

fn open_demo(cx: &mut TestAppContext) -> WindowHandle<ComboboxDemoView> {
    cx.update(|cx| {
        crate::primitives::input::init(cx);
        crate::combobox::init(cx);
    });
    let window = cx.open_window(size(px(640.0), px(480.0)), |_, _| ComboboxDemoView::new());
    cx.run_until_parked();
    window
}

fn demo_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<ComboboxDemoView>,
    selector: &'static str,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);
    visual.debug_bounds(selector)
}

fn click_at(
    cx: &mut TestAppContext,
    window: WindowHandle<ComboboxDemoView>,
    bounds: Bounds<Pixels>,
) {
    let mut visual = VisualTestContext::from_window(window.into(), cx);
    visual.simulate_click(bounds.center(), Modifiers::default());
    visual.run_until_parked();
    cx.run_until_parked();
}

fn last_input_state(
    cx: &mut TestAppContext,
    window: WindowHandle<ComboboxDemoView>,
) -> InputStyleState {
    window
        .update(cx, |view, _window, _cx| view.last_input_state())
        .expect("demo window should be open")
        .expect("input style state should be observed")
}

#[gpui::test]
fn demo_input_area_is_not_collapsed(cx: &mut TestAppContext) {
    let window = open_demo(cx);

    let bounds = demo_bounds(cx, window, "demo-combobox-input-area")
        .expect("input area should have debug bounds");
    assert!(
        bounds.size.width >= px(100.0),
        "input area should occupy the flexible space, got {:?}",
        bounds.size.width
    );
    assert!(
        bounds.size.height >= px(10.0),
        "input area should have text height, got {:?}",
        bounds.size.height
    );
}

#[gpui::test]
fn demo_typing_updates_input_value(cx: &mut TestAppContext) {
    let window = open_demo(cx);

    let bounds = demo_bounds(cx, window, "demo-combobox-input-area")
        .expect("input area should have debug bounds");
    click_at(cx, window, bounds);
    cx.simulate_keystrokes(window.into(), "a p p");
    cx.run_until_parked();

    let state = last_input_state(cx, window);
    assert!(state.focused, "clicking the input should focus it");
    assert_eq!(state.value.as_ref(), "app");
}

#[gpui::test]
fn demo_enter_selects_highlighted_item(cx: &mut TestAppContext) {
    let window = open_demo(cx);

    let input_bounds = demo_bounds(cx, window, "demo-combobox-input-area")
        .expect("input area should have debug bounds");
    click_at(cx, window, input_bounds);
    cx.simulate_keystrokes(window.into(), "a p p");
    cx.run_until_parked();
    cx.simulate_keystrokes(window.into(), "down enter");
    cx.run_until_parked();

    let state = last_input_state(cx, window);
    assert_eq!(
        state.value.as_ref(),
        "Apple",
        "Enter on a highlighted item should select it"
    );
}

#[gpui::test]
fn demo_selecting_item_displays_label_in_input(cx: &mut TestAppContext) {
    let window = open_demo(cx);

    let input_bounds = demo_bounds(cx, window, "demo-combobox-input-area")
        .expect("input area should have debug bounds");
    click_at(cx, window, input_bounds);
    cx.simulate_keystrokes(window.into(), "a p p");
    cx.run_until_parked();

    let item_bounds =
        demo_bounds(cx, window, "demo-item-apple").expect("apple item should be rendered");
    click_at(cx, window, item_bounds);

    let state = last_input_state(cx, window);
    assert_eq!(
        state.value.as_ref(),
        "Apple",
        "selected item label should render in the input"
    );
}
