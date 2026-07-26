use gpui::prelude::*;
use gpui::{div, px, size, App, Application, Bounds, Window, WindowBounds, WindowOptions};

mod demos;
mod theme;

fn main() {
    // On the web, `Platform::run` returns immediately instead of blocking, so
    // the plain `run` entrypoint would drop the app right after launch;
    // `run_embedded` returns a handle that keeps it alive, which we leak for
    // the lifetime of the page.
    #[cfg(target_family = "wasm")]
    {
        gpui_platform::web_init();
        let handle = application().run_embedded(launch);
        std::mem::forget(handle);
    }

    #[cfg(not(target_family = "wasm"))]
    application().run(launch);
}

fn launch(cx: &mut App) {
    base_gpui::init(cx);

    let slug = requested_demo();
    let bounds = Bounds::centered(None, size(px(960.), px(720.)), cx);
    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        },
        |_, cx| cx.new(|_| Showcase { slug }),
    )
    .expect("failed to open window");
    cx.activate(true);
}

fn application() -> Application {
    gpui_platform::application()
}

/// The demo to display, from `?demo=<slug>` on the web or the first CLI
/// argument natively. `None` renders the full gallery.
#[cfg(target_family = "wasm")]
fn requested_demo() -> Option<String> {
    let search = web_sys::window()?.location().search().ok()?;
    search
        .trim_start_matches('?')
        .split('&')
        .find_map(|pair| pair.strip_prefix("demo="))
        .filter(|slug| !slug.is_empty())
        .map(str::to_owned)
}

#[cfg(not(target_family = "wasm"))]
fn requested_demo() -> Option<String> {
    std::env::args().nth(1)
}

struct Showcase {
    slug: Option<String>,
}

impl Render for Showcase {
    fn render(&mut self, _window: &mut Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let entry = self.slug.as_deref().and_then(demos::find);

        match entry {
            Some(entry) => div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .bg(theme::background())
                .child((entry.render)())
                .into_any_element(),
            None => div()
                .id("gallery")
                .size_full()
                .overflow_y_scroll()
                .bg(theme::background())
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(32.))
                        .p(px(48.))
                        .max_w(px(720.))
                        .mx_auto()
                        .child(
                            div()
                                .text_2xl()
                                .text_color(theme::text())
                                .child("base-gpui showcase"),
                        )
                        .children(demos::all().iter().map(|entry| {
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(12.))
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(theme::text_muted())
                                        .child(entry.title),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .p(px(32.))
                                        .rounded(px(8.))
                                        .border_1()
                                        .border_color(theme::border())
                                        .bg(theme::surface())
                                        .child((entry.render)()),
                                )
                        })),
                )
                .into_any_element(),
        }
    }
}
