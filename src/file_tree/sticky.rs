use super::presentation::FileTreeVisibleRow;

#[derive(Clone, Debug, PartialEq)]
pub struct FileTreeStickyCandidate {
    pub row: FileTreeVisibleRow,
    pub subtree_end_index: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FileTreeStickyRow {
    pub row: FileTreeVisibleRow,
    pub top: f32,
}

pub fn layout_sticky_rows(
    candidates: Vec<FileTreeStickyCandidate>,
    total_row_count: usize,
    scroll_top: f32,
    row_height: f32,
) -> Vec<FileTreeStickyRow> {
    if total_row_count == 0 || scroll_top <= 0.0 || row_height <= 0.0 {
        return Vec::new();
    }

    candidates
        .into_iter()
        .enumerate()
        .filter_map(|(slot_depth, candidate)| {
            let default_top = slot_depth as f32 * row_height;
            let next_boundary_index = candidate.subtree_end_index.saturating_add(1);
            let top = if next_boundary_index >= total_row_count {
                default_top
            } else {
                let next_boundary_top = next_boundary_index as f32 * row_height - scroll_top;
                default_top.min(next_boundary_top - row_height)
            };
            (top + row_height > 0.0).then_some(FileTreeStickyRow {
                row: candidate.row,
                top,
            })
        })
        .collect()
}

pub fn sticky_height(rows: &[FileTreeStickyRow], row_height: f32) -> f32 {
    rows.iter()
        .map(|entry| entry.top + row_height)
        .fold(0.0, f32::max)
}

pub fn occluded_row_range(
    scroll_top: f32,
    sticky_height: f32,
    row_height: f32,
    total_row_count: usize,
) -> std::ops::Range<usize> {
    if sticky_height <= 0.0 || row_height <= 0.0 || total_row_count == 0 {
        return 0..0;
    }

    let start = (scroll_top / row_height).floor().max(0.0) as usize;
    let end = ((scroll_top + sticky_height) / row_height).ceil().max(0.0) as usize;
    start.min(total_row_count)..end.min(total_row_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_tree::{FileTreeItemKind, FileTreeVisibleSegment};

    fn row(path: &str, index: usize, depth: usize) -> FileTreeVisibleRow {
        FileTreeVisibleRow {
            index,
            path: path.to_owned(),
            name: path.trim_end_matches('/').to_owned(),
            kind: FileTreeItemKind::Directory,
            depth,
            level: depth.saturating_sub(1),
            ancestor_paths: Vec::new(),
            is_expanded: true,
            has_children: true,
            position_in_set: 1,
            set_size: 1,
            is_selected: false,
            is_focused: false,
            is_flattened: false,
            flattened_segments: Vec::<FileTreeVisibleSegment>::new(),
        }
    }

    #[test]
    fn zero_and_top_scroll_have_no_sticky_layout() {
        let candidates = vec![FileTreeStickyCandidate {
            row: row("src/", 0, 1),
            subtree_end_index: 3,
        }];
        assert!(layout_sticky_rows(candidates.clone(), 4, 0.0, 30.0).is_empty());
        assert!(layout_sticky_rows(candidates, 4, -1.0, 30.0).is_empty());
    }

    #[test]
    fn nested_slots_stay_outer_to_inner_and_push_off_at_subtree_end() {
        let candidates = vec![
            FileTreeStickyCandidate {
                row: row("src/", 0, 1),
                subtree_end_index: 8,
            },
            FileTreeStickyCandidate {
                row: row("src/lib/", 1, 2),
                subtree_end_index: 5,
            },
            FileTreeStickyCandidate {
                row: row("src/lib/deep/", 2, 3),
                subtree_end_index: 4,
            },
        ];

        let before = layout_sticky_rows(candidates.clone(), 9, 59.0, 30.0);
        assert_eq!(
            before
                .iter()
                .map(|entry| entry.row.path.as_str())
                .collect::<Vec<_>>(),
            vec!["src/", "src/lib/", "src/lib/deep/"]
        );
        assert_eq!(
            before.iter().map(|entry| entry.top).collect::<Vec<_>>(),
            vec![0.0, 30.0, 60.0]
        );

        let boundary = layout_sticky_rows(candidates.clone(), 9, 60.0, 30.0);
        assert_eq!(boundary[2].top, 60.0);
        let displaced = layout_sticky_rows(candidates, 9, 61.0, 30.0);
        assert_eq!(displaced[2].top, 59.0);
        assert_eq!(sticky_height(&displaced, 30.0), 89.0);
        assert_eq!(occluded_row_range(61.0, 89.0, 30.0, 9), 2..5);
    }
}
