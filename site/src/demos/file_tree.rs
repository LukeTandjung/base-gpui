use base_gpui::file_tree::{
    FileTree, FileTreeDensity, FileTreeInitialExpansion, FileTreePreparedInput,
};
use gpui::prelude::*;
use gpui::{div, px};

use crate::theme;

pub fn render() -> impl IntoElement {
    let input = FileTreePreparedInput::presorted(large_workspace_paths())
        .expect("the generated demo paths are unique and presorted");

    div()
        .w(px(620.))
        .h(px(520.))
        .rounded(px(8.))
        .border_1()
        .border_color(theme::border())
        .bg(theme::surface())
        .text_color(theme::text())
        .child(
            FileTree::new(input)
                .id("demo-file-tree")
                .initial_expansion(FileTreeInitialExpansion::Depth(2))
                .flatten_empty_directories(true)
                .search(true)
                .renaming(true)
                .drag_and_drop(true)
                .sticky_folders(true)
                .density(FileTreeDensity::Default),
        )
}

fn large_workspace_paths() -> Vec<String> {
    let mut paths = Vec::with_capacity(12_503);

    for app in ["desktop", "web"] {
        for module in 0..20 {
            for file in 0..100 {
                paths.push(format!(
                    "apps/{app}/src/module-{module:02}/feature-{file:03}.tsx"
                ));
            }
        }
    }

    for crate_index in 0..80 {
        for file in 0..100 {
            paths.push(format!(
                "crates/crate-{crate_index:03}/src/subsystem-{file:03}.rs"
            ));
        }
    }

    for section in 0..5 {
        for page in 0..100 {
            paths.push(format!("docs/section-{section:02}/page-{page:03}.md"));
        }
    }

    paths.extend([
        "Cargo.toml".to_owned(),
        "LICENSE".to_owned(),
        "README.md".to_owned(),
    ]);
    paths
}
