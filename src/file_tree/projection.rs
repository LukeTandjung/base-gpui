use std::{collections::HashSet, ops::Range};

use super::{
    presentation::{FileTreeItemKind, FileTreeVisibleRow, FileTreeVisibleSegment},
    store::{FileTreeStore, NodeKind},
};

pub fn materialize_range(
    store: &FileTreeStore,
    range: Range<usize>,
    selected_paths: &HashSet<String>,
    focused_path: Option<&str>,
) -> Vec<FileTreeVisibleRow> {
    let start = range.start.min(store.visible_count());
    let end = range.end.min(store.visible_count());
    let mut rows = Vec::with_capacity(end.saturating_sub(start));
    let Some(mut cursor) = store.visible_cursor_at(start) else {
        return rows;
    };

    for visible_index in start..end {
        let resolved = store.resolved_cursor(&cursor);
        let node = store.node(resolved.node_id);
        let path = store.path_for_node(resolved.node_id);
        let flattened_node_ids = store.flattened_chain_node_ids(resolved.head_node_id);
        let is_flattened = flattened_node_ids.len() > 1;
        let flattened_segments = if is_flattened {
            flattened_node_ids
                .into_iter()
                .map(|node_id| FileTreeVisibleSegment {
                    path: store.path_for_node(node_id),
                    name: store.node_name(node_id).to_owned(),
                    is_terminal: node_id == resolved.node_id,
                })
                .collect()
        } else {
            Vec::new()
        };
        rows.push(FileTreeVisibleRow {
            index: visible_index,
            is_selected: selected_paths.contains(&path),
            is_focused: focused_path == Some(path.as_str()),
            path,
            name: store.node_name(resolved.node_id).to_owned(),
            kind: match node.kind {
                NodeKind::Directory => FileTreeItemKind::Directory,
                NodeKind::File => FileTreeItemKind::File,
            },
            depth: resolved.visible_depth,
            level: resolved.visible_depth.saturating_sub(1),
            ancestor_paths: store
                .visible_ancestor_node_ids(&cursor)
                .into_iter()
                .map(|node_id| store.path_for_node(node_id))
                .collect(),
            is_expanded: node.kind == NodeKind::Directory && node.expanded,
            has_children: store.has_children(resolved.node_id),
            position_in_set: resolved.sibling_index + 1,
            set_size: resolved.sibling_count,
            is_flattened,
            flattened_segments,
        });

        if visible_index + 1 < end && !store.advance_visible_cursor(&mut cursor) {
            break;
        }
    }

    rows
}

pub fn materialize_indices(
    store: &FileTreeStore,
    visible_start: usize,
    indices: impl IntoIterator<Item = usize>,
    selected_paths: &HashSet<String>,
    focused_path: Option<&str>,
) -> Vec<FileTreeVisibleRow> {
    let mut indices = indices.into_iter().peekable();
    let mut rows = Vec::new();
    let mut visible_index = visible_start;
    while let Some(run_start) = indices.next() {
        let mut run_end = run_start.saturating_add(1);
        while indices.peek().copied() == Some(run_end) {
            indices.next();
            run_end = run_end.saturating_add(1);
        }
        for mut row in materialize_range(store, run_start..run_end, selected_paths, focused_path) {
            row.index = visible_index;
            visible_index += 1;
            rows.push(row);
        }
    }
    rows
}
