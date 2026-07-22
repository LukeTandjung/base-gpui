use std::rc::Rc;

use gpui::{
    div, App, Bounds, Div, InteractiveElement as _, MouseButton, ParentElement, Pixels, Size,
    Styled, Window,
};

pub type OverlayDismissHandler = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

pub fn modal_backdrop(
    available_size: Size<Pixels>,
    cutout_bounds: Option<Bounds<Pixels>>,
    on_dismiss: OverlayDismissHandler,
) -> Div {
    let Some(cutout_bounds) = cutout_bounds else {
        return div();
    };
    if cutout_bounds.size.width <= Pixels::ZERO || cutout_bounds.size.height <= Pixels::ZERO {
        return div();
    }

    let mut backdrop = div()
        .w(available_size.width)
        .h(available_size.height)
        .absolute()
        .top_0()
        .left_0();
    backdrop = add_backdrop_piece(
        backdrop,
        px_zero(),
        px_zero(),
        available_size.width,
        cutout_bounds.top(),
        Rc::clone(&on_dismiss),
    );
    backdrop = add_backdrop_piece(
        backdrop,
        px_zero(),
        cutout_bounds.bottom(),
        available_size.width,
        available_size.height - cutout_bounds.bottom(),
        Rc::clone(&on_dismiss),
    );
    backdrop = add_backdrop_piece(
        backdrop,
        px_zero(),
        cutout_bounds.top(),
        cutout_bounds.left(),
        cutout_bounds.size.height,
        Rc::clone(&on_dismiss),
    );
    add_backdrop_piece(
        backdrop,
        cutout_bounds.right(),
        cutout_bounds.top(),
        available_size.width - cutout_bounds.right(),
        cutout_bounds.size.height,
        on_dismiss,
    )
}

fn add_backdrop_piece(
    backdrop: Div,
    x: Pixels,
    y: Pixels,
    width: Pixels,
    height: Pixels,
    on_dismiss: OverlayDismissHandler,
) -> Div {
    if width <= Pixels::ZERO || height <= Pixels::ZERO {
        return backdrop;
    }

    backdrop.child(
        div()
            .absolute()
            .left(x)
            .top(y)
            .w(width)
            .h(height)
            .occlude()
            .on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                on_dismiss(window, cx);
                cx.stop_propagation();
            })
            .on_scroll_wheel(move |_event, _window, cx| {
                cx.stop_propagation();
            }),
    )
}

fn px_zero() -> Pixels {
    Pixels::ZERO
}
