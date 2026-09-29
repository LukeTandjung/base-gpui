use base_gpui::file_tree::{
    FileTree, FileTreeDensity, FileTreeInitialExpansion, FileTreePreparedInput,
};
use gpui::prelude::*;
use gpui::{div, px, rgb, size, App, Bounds, Context, Window, WindowBounds, WindowOptions};

fn main() {
    gpui_platform::application().run(launch);
}

fn launch(cx: &mut App) {
    base_gpui::init(cx);

    let bounds = Bounds::centered(None, size(px(760.), px(640.)), cx);
    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        },
        |_, cx| cx.new(|_| FileTreeDemo::new()),
    )
    .expect("failed to open file-tree window");
    cx.activate(true);
}

struct FileTreeDemo {
    input: FileTreePreparedInput,
}

impl FileTreeDemo {
    fn new() -> Self {
        Self {
            input: FileTreePreparedInput::prepare(workspace_paths())
                .expect("the generated demo paths are valid and unique"),
        }
    }
}

impl Render for FileTreeDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgb(0x0e0e0e))
            .text_color(rgb(0xffffff))
            .child(
                div()
                    .w(px(620.))
                    .h(px(540.))
                    .overflow_hidden()
                    .rounded(px(8.))
                    .border_1()
                    .border_color(rgb(0x303030))
                    .bg(rgb(0x191919))
                    .child(
                        FileTree::new(self.input.clone())
                            .id("standalone-file-tree")
                            .initial_expansion(FileTreeInitialExpansion::Depth(2))
                            .flatten_empty_directories(true)
                            .search(true)
                            .renaming(true)
                            .drag_and_drop(true)
                            .sticky_folders(true)
                            .density(FileTreeDensity::Default),
                    ),
            )
    }
}

fn workspace_paths() -> Vec<String> {
    let mut paths = vec![
        ".github/workflows/ci.yml".to_owned(),
        ".github/workflows/release.yml".to_owned(),
        "Cargo.lock".to_owned(),
        "Cargo.toml".to_owned(),
        "LICENSE".to_owned(),
        "README.md".to_owned(),
        "docs/components/file_tree.md".to_owned(),
        "docs/getting-started.md".to_owned(),
        "examples/file_tree/Cargo.toml".to_owned(),
        "examples/file_tree/src/main.rs".to_owned(),
        "src/file_tree/mod.rs".to_owned(),
        "src/file_tree/store.rs".to_owned(),
        "src/lib.rs".to_owned(),
        "tests/file_tree.rs".to_owned(),
    ];

    for crate_index in 0..24 {
        for module_index in 0..32 {
            paths.push(format!(
                "crates/component-{crate_index:02}/src/module-{module_index:02}.rs"
            ));
        }
    }

    for package_index in 0..8 {
        for file_index in 0..24 {
            paths.push(format!(
                "packages/package-{package_index:02}/src/feature-{file_index:02}.tsx"
            ));
        }
    }

    paths
}
