use std::{cell::RefCell, collections::HashMap};

use super::{
    prepared_input::{split_path, FileTreeInputError, FileTreePreparedInput, PreparedInputKind},
    FileTreeInitialExpansion,
};

pub type NodeId = usize;
type SegmentId = usize;

const ROOT_ID: NodeId = 0;
const CHILD_SUMMARY_CHUNK_SIZE: usize = 128;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NodeKind {
    File,
    Directory,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub parent_id: NodeId,
    pub name_id: SegmentId,
    pub depth: usize,
    pub kind: NodeKind,
    pub explicit: bool,
    pub expanded: bool,
    pub subtree_node_count: usize,
    pub visible_subtree_count: usize,
}

#[derive(Debug)]
struct DirectoryIndex {
    child_ids: Vec<NodeId>,
    child_id_by_name_id: Option<HashMap<SegmentId, NodeId>>,
    child_position_by_id: Option<HashMap<NodeId, usize>>,
    child_visible_chunk_sums: Option<Vec<usize>>,
    total_child_subtree_node_count: usize,
    total_child_visible_subtree_count: usize,
}

impl DirectoryIndex {
    fn new() -> Self {
        Self {
            child_ids: Vec::new(),
            child_id_by_name_id: None,
            child_position_by_id: None,
            child_visible_chunk_sums: None,
            total_child_subtree_node_count: 0,
            total_child_visible_subtree_count: 0,
        }
    }
}

#[derive(Debug)]
struct SegmentTable {
    id_by_value: HashMap<String, SegmentId>,
    value_by_id: Vec<String>,
}

impl SegmentTable {
    fn new() -> Self {
        let mut id_by_value = HashMap::new();
        id_by_value.insert(String::new(), 0);
        Self {
            id_by_value,
            value_by_id: vec![String::new()],
        }
    }

    fn intern(&mut self, value: &str) -> SegmentId {
        if let Some(id) = self.id_by_value.get(value) {
            return *id;
        }

        let id = self.value_by_id.len();
        let value = value.to_owned();
        self.id_by_value.insert(value.clone(), id);
        self.value_by_id.push(value);
        id
    }

    fn id(&self, value: &str) -> Option<SegmentId> {
        self.id_by_value.get(value).copied()
    }

    fn value(&self, id: SegmentId) -> &str {
        &self.value_by_id[id]
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ResolvedVisibleNode {
    pub head_node_id: NodeId,
    pub node_id: NodeId,
    pub sibling_index: usize,
    pub sibling_count: usize,
    pub visible_depth: usize,
}

#[derive(Clone, Copy, Debug)]
struct CursorFrame {
    directory_id: NodeId,
    child_index: usize,
}

#[derive(Clone, Debug)]
pub struct VisibleCursor {
    frames: Vec<CursorFrame>,
    node_id: NodeId,
}

#[derive(Debug)]
pub struct FileTreeStore {
    nodes: Vec<Node>,
    directories: Vec<Option<DirectoryIndex>>,
    segments: SegmentTable,
    path_cache: RefCell<HashMap<NodeId, String>>,
    flatten_empty_directories: bool,
}

impl FileTreeStore {
    pub fn build_with_flattening(
        input: &FileTreePreparedInput,
        initial_expansion: FileTreeInitialExpansion,
        initial_expanded_paths: &[String],
        flatten_empty_directories: bool,
    ) -> Result<Self, FileTreeInputError> {
        let mut builder = StoreBuilder::new();

        match input.kind() {
            PreparedInputKind::Prepared { paths, .. } => {
                for path in paths.iter() {
                    builder.append(
                        &path.path,
                        path.segments.iter().map(String::as_str),
                        path.is_directory,
                    )?;
                }
            }
            PreparedInputKind::Presorted(paths) => {
                for path in paths.iter() {
                    let split = split_path(path);
                    builder.append(path, split.segments, split.is_directory)?;
                }
            }
        }

        Ok(builder.finish(
            initial_expansion,
            initial_expanded_paths,
            flatten_empty_directories,
        ))
    }

    pub fn visible_count(&self) -> usize {
        self.nodes[ROOT_ID].visible_subtree_count
    }

    pub fn node(&self, node_id: NodeId) -> &Node {
        &self.nodes[node_id]
    }

    pub fn node_name(&self, node_id: NodeId) -> &str {
        self.segments.value(self.nodes[node_id].name_id)
    }

    pub fn has_children(&self, node_id: NodeId) -> bool {
        self.directories
            .get(node_id)
            .and_then(Option::as_ref)
            .is_some_and(|directory| !directory.child_ids.is_empty())
    }

    pub fn path_for_node(&self, node_id: NodeId) -> String {
        if let Some(path) = self.path_cache.borrow().get(&node_id) {
            return path.clone();
        }

        let mut segment_ids = Vec::with_capacity(self.nodes[node_id].depth);
        let mut current_id = node_id;
        while current_id != ROOT_ID {
            let node = &self.nodes[current_id];
            segment_ids.push(node.name_id);
            current_id = node.parent_id;
        }
        segment_ids.reverse();

        let mut path = String::new();
        for (index, segment_id) in segment_ids.into_iter().enumerate() {
            if index > 0 {
                path.push('/');
            }
            path.push_str(self.segments.value(segment_id));
        }
        if self.nodes[node_id].kind == NodeKind::Directory {
            path.push('/');
        }

        self.path_cache.borrow_mut().insert(node_id, path.clone());
        path
    }

    pub fn visible_cursor_at(&self, index: usize) -> Option<VisibleCursor> {
        if index >= self.visible_count() {
            return None;
        }

        let mut remaining = index;
        let mut directory_id = ROOT_ID;
        let mut frames = Vec::new();

        loop {
            let (head_node_id, child_index) = self.select_visible_child(directory_id, remaining)?;
            let skipped = self.visible_count_before_child(directory_id, child_index);
            remaining -= skipped;
            frames.push(CursorFrame {
                directory_id,
                child_index,
            });
            let terminal_node_id = self.flattened_terminal_directory_id(head_node_id);

            if remaining == 0 {
                return Some(VisibleCursor {
                    frames,
                    node_id: terminal_node_id,
                });
            }

            let terminal = &self.nodes[terminal_node_id];
            if terminal.kind != NodeKind::Directory || !terminal.expanded {
                return None;
            }

            remaining -= 1;
            directory_id = terminal_node_id;
        }
    }

    pub fn resolved_cursor(&self, cursor: &VisibleCursor) -> ResolvedVisibleNode {
        let frame = cursor
            .frames
            .last()
            .expect("visible cursor always contains one selection frame");
        let directory = self.directories[frame.directory_id]
            .as_ref()
            .expect("visible cursor parent must be a directory");
        ResolvedVisibleNode {
            head_node_id: directory.child_ids[frame.child_index],
            node_id: cursor.node_id,
            sibling_index: frame.child_index,
            sibling_count: directory.child_ids.len(),
            visible_depth: cursor.frames.len(),
        }
    }

    pub fn advance_visible_cursor(&self, cursor: &mut VisibleCursor) -> bool {
        let current = &self.nodes[cursor.node_id];
        if current.kind == NodeKind::Directory && current.expanded {
            let directory = self.directories[cursor.node_id]
                .as_ref()
                .expect("directory node must have a child index");
            if let Some(first_child) = directory.child_ids.first().copied() {
                cursor.frames.push(CursorFrame {
                    directory_id: cursor.node_id,
                    child_index: 0,
                });
                cursor.node_id = self.flattened_terminal_directory_id(first_child);
                return true;
            }
        }

        while let Some(frame) = cursor.frames.last_mut() {
            let directory = self.directories[frame.directory_id]
                .as_ref()
                .expect("visible cursor parent must be a directory");
            let next_index = frame.child_index + 1;
            if let Some(next_child) = directory.child_ids.get(next_index).copied() {
                frame.child_index = next_index;
                cursor.node_id = self.flattened_terminal_directory_id(next_child);
                return true;
            }
            cursor.frames.pop();
        }

        false
    }

    pub fn set_expanded(&mut self, path: &str, expanded: bool) -> bool {
        let Some(node_id) = self.find_node(path) else {
            return false;
        };
        if self.nodes[node_id].kind != NodeKind::Directory
            || self.nodes[node_id].expanded == expanded
        {
            return false;
        }

        if self.flatten_empty_directories {
            self.nodes[node_id].expanded = expanded;
            self.recompute_visible_counts();
            return true;
        }

        let old_visible_count = self.nodes[node_id].visible_subtree_count;
        let child_visible_count = self.directories[node_id]
            .as_ref()
            .expect("directory node must have a child index")
            .total_child_visible_subtree_count;
        self.nodes[node_id].expanded = expanded;
        self.nodes[node_id].visible_subtree_count =
            if expanded { 1 + child_visible_count } else { 1 };
        let new_visible_count = self.nodes[node_id].visible_subtree_count;
        self.propagate_visible_delta(node_id, old_visible_count, new_visible_count);
        true
    }

    pub fn canonical_path(&mut self, path: &str) -> Option<String> {
        let node_id = self.find_node(path)?;
        (node_id != ROOT_ID).then(|| self.path_for_node(node_id))
    }

    pub fn is_directory_path(&mut self, path: &str) -> bool {
        self.find_node(path)
            .filter(|node_id| *node_id != ROOT_ID)
            .is_some_and(|node_id| self.nodes[node_id].kind == NodeKind::Directory)
    }

    pub fn is_expanded_path(&mut self, path: &str) -> bool {
        self.find_node(path)
            .filter(|node_id| *node_id != ROOT_ID)
            .is_some_and(|node_id| {
                self.nodes[node_id].kind == NodeKind::Directory && self.nodes[node_id].expanded
            })
    }

    pub fn parent_path(&mut self, path: &str) -> Option<String> {
        let node_id = self.find_node(path)?;
        let parent_id = self.nodes[node_id].parent_id;
        (parent_id != ROOT_ID).then(|| self.path_for_node(parent_id))
    }

    pub fn expanded_paths(&self) -> Vec<String> {
        self.nodes
            .iter()
            .enumerate()
            .skip(1)
            .filter(|(_, node)| node.kind == NodeKind::Directory && node.expanded)
            .map(|(node_id, _)| self.path_for_node(node_id))
            .collect()
    }

    pub fn directory_paths(&self) -> Vec<String> {
        self.nodes
            .iter()
            .enumerate()
            .skip(1)
            .filter(|(_, node)| node.kind == NodeKind::Directory)
            .map(|(node_id, _)| self.path_for_node(node_id))
            .collect()
    }

    pub fn flattened_chain_node_ids(&self, head_node_id: NodeId) -> Vec<NodeId> {
        let mut node_ids = vec![head_node_id];
        let mut current_id = head_node_id;
        while let Some(next_id) = self.flattened_child_directory_id(current_id) {
            node_ids.push(next_id);
            current_id = next_id;
        }
        node_ids
    }

    pub fn visible_ancestor_node_ids(&self, cursor: &VisibleCursor) -> Vec<NodeId> {
        cursor
            .frames
            .iter()
            .map(|frame| frame.directory_id)
            .filter(|node_id| *node_id != ROOT_ID)
            .collect()
    }

    pub fn path_at_visible_index(&self, index: usize) -> Option<String> {
        let cursor = self.visible_cursor_at(index)?;
        Some(self.path_for_node(cursor.node_id))
    }

    pub fn visible_paths(&self, range: std::ops::Range<usize>) -> Vec<String> {
        let start = range.start.min(self.visible_count());
        let end = range.end.min(self.visible_count());
        let mut paths = Vec::with_capacity(end.saturating_sub(start));
        let Some(mut cursor) = self.visible_cursor_at(start) else {
            return paths;
        };

        for index in start..end {
            paths.push(self.path_for_node(cursor.node_id));
            if index + 1 < end && !self.advance_visible_cursor(&mut cursor) {
                break;
            }
        }
        paths
    }

    pub fn visible_index_for_path(&mut self, path: &str) -> Option<usize> {
        let node_id = self.find_node(path)?;
        self.visible_index_for_node(node_id)
    }

    pub fn visible_subtree_end_index_for_path(&mut self, path: &str) -> Option<usize> {
        let node_id = self.find_node(path)?;
        let visible_index = self.visible_index_for_node(node_id)?;
        Some(
            visible_index
                .saturating_add(self.nodes[node_id].visible_subtree_count)
                .saturating_sub(1),
        )
    }

    pub fn nearest_visible_index_for_path(&mut self, path: &str) -> Option<usize> {
        let mut node_id = self.find_node(path)?;

        loop {
            if let Some(index) = self.visible_index_for_node(node_id) {
                return Some(index);
            }
            if node_id == ROOT_ID {
                return (self.visible_count() > 0).then_some(0);
            }
            node_id = self.nodes[node_id].parent_id;
        }
    }

    fn find_node(&mut self, path: &str) -> Option<NodeId> {
        if path.is_empty() {
            return Some(ROOT_ID);
        }

        let parsed = split_path(path);
        let mut directory_id = ROOT_ID;
        let segment_count = parsed.segments.len();

        for (index, segment) in parsed.segments.iter().enumerate() {
            let name_id = self.segments.id(segment)?;
            let child_id = self.find_child(directory_id, name_id)?;
            let child = &self.nodes[child_id];
            let terminal = index + 1 == segment_count;

            if terminal {
                if parsed.is_directory && child.kind != NodeKind::Directory {
                    return None;
                }
                return Some(child_id);
            }
            if child.kind != NodeKind::Directory {
                return None;
            }
            directory_id = child_id;
        }

        None
    }

    fn find_child(&mut self, directory_id: NodeId, name_id: SegmentId) -> Option<NodeId> {
        if self.directories[directory_id]
            .as_ref()
            .expect("lookup parent must be a directory")
            .child_id_by_name_id
            .is_none()
        {
            let child_ids = self.directories[directory_id]
                .as_ref()
                .expect("lookup parent must be a directory")
                .child_ids
                .clone();
            let map = child_ids
                .into_iter()
                .map(|child_id| (self.nodes[child_id].name_id, child_id))
                .collect();
            self.directories[directory_id]
                .as_mut()
                .expect("lookup parent must be a directory")
                .child_id_by_name_id = Some(map);
        }

        self.directories[directory_id]
            .as_ref()
            .expect("lookup parent must be a directory")
            .child_id_by_name_id
            .as_ref()
            .and_then(|map| map.get(&name_id).copied())
    }

    fn select_visible_child(
        &self,
        directory_id: NodeId,
        visible_index: usize,
    ) -> Option<(NodeId, usize)> {
        let directory = self.directories[directory_id]
            .as_ref()
            .expect("visible lookup parent must be a directory");
        let mut remaining = visible_index;
        let mut child_start = 0;

        if let Some(chunk_sums) = directory.child_visible_chunk_sums.as_ref() {
            for chunk_sum in chunk_sums {
                if remaining < *chunk_sum {
                    break;
                }
                remaining -= *chunk_sum;
                child_start += CHILD_SUMMARY_CHUNK_SIZE;
            }
        }

        let child_end = (child_start + CHILD_SUMMARY_CHUNK_SIZE).min(directory.child_ids.len());
        for child_index in child_start..child_end {
            let child_id = directory.child_ids[child_index];
            let child_visible_count = self.nodes[child_id].visible_subtree_count;
            if remaining < child_visible_count {
                return Some((child_id, child_index));
            }
            remaining -= child_visible_count;
        }

        None
    }

    fn visible_count_before_child(&self, directory_id: NodeId, child_index: usize) -> usize {
        let directory = self.directories[directory_id]
            .as_ref()
            .expect("visible lookup parent must be a directory");
        let mut visible_count = 0;
        let mut scan_start = 0;

        if let Some(chunk_sums) = directory.child_visible_chunk_sums.as_ref() {
            let chunk_index = child_index / CHILD_SUMMARY_CHUNK_SIZE;
            visible_count += chunk_sums[..chunk_index].iter().sum::<usize>();
            scan_start = chunk_index * CHILD_SUMMARY_CHUNK_SIZE;
        }

        for position in scan_start..child_index {
            visible_count += self.nodes[directory.child_ids[position]].visible_subtree_count;
        }
        visible_count
    }

    fn propagate_visible_delta(
        &mut self,
        mut child_id: NodeId,
        old_child_count: usize,
        new_child_count: usize,
    ) {
        let mut delta = new_child_count as isize - old_child_count as isize;

        while delta != 0 && child_id != ROOT_ID {
            let parent_id = self.nodes[child_id].parent_id;
            let child_position = self.child_position(parent_id, child_id);
            let parent_directory = self.directories[parent_id]
                .as_mut()
                .expect("node parent must be a directory");
            parent_directory.total_child_visible_subtree_count =
                add_signed(parent_directory.total_child_visible_subtree_count, delta);
            if let Some(chunk_sums) = parent_directory.child_visible_chunk_sums.as_mut() {
                let chunk_index = child_position / CHILD_SUMMARY_CHUNK_SIZE;
                chunk_sums[chunk_index] = add_signed(chunk_sums[chunk_index], delta);
            }

            let parent_visible = parent_id == ROOT_ID || self.nodes[parent_id].expanded;
            if !parent_visible {
                break;
            }

            let old_parent_count = self.nodes[parent_id].visible_subtree_count;
            self.nodes[parent_id].visible_subtree_count = add_signed(old_parent_count, delta);
            let new_parent_count = self.nodes[parent_id].visible_subtree_count;
            delta = new_parent_count as isize - old_parent_count as isize;
            child_id = parent_id;
        }
    }

    fn child_position(&mut self, directory_id: NodeId, child_id: NodeId) -> usize {
        if self.directories[directory_id]
            .as_ref()
            .expect("node parent must be a directory")
            .child_position_by_id
            .is_none()
        {
            let positions = self.directories[directory_id]
                .as_ref()
                .expect("node parent must be a directory")
                .child_ids
                .iter()
                .copied()
                .enumerate()
                .map(|(position, child_id)| (child_id, position))
                .collect();
            self.directories[directory_id]
                .as_mut()
                .expect("node parent must be a directory")
                .child_position_by_id = Some(positions);
        }

        self.directories[directory_id]
            .as_ref()
            .expect("node parent must be a directory")
            .child_position_by_id
            .as_ref()
            .and_then(|positions| positions.get(&child_id).copied())
            .expect("child must belong to its parent")
    }

    fn visible_index_for_node(&mut self, node_id: NodeId) -> Option<usize> {
        if node_id == ROOT_ID {
            return None;
        }
        if self.flatten_empty_directories {
            let mut cursor = self.visible_cursor_at(0)?;
            for visible_index in 0..self.visible_count() {
                let resolved = self.resolved_cursor(&cursor);
                if self
                    .flattened_chain_node_ids(resolved.head_node_id)
                    .contains(&node_id)
                    || resolved.node_id == node_id
                {
                    return Some(visible_index);
                }
                if !self.advance_visible_cursor(&mut cursor) {
                    break;
                }
            }
            return None;
        }

        let mut lineage = Vec::with_capacity(self.nodes[node_id].depth);
        let mut current_id = node_id;
        while current_id != ROOT_ID {
            lineage.push(current_id);
            current_id = self.nodes[current_id].parent_id;
        }
        lineage.reverse();

        let mut visible_index = 0;
        for child_id in lineage {
            let parent_id = self.nodes[child_id].parent_id;
            if parent_id != ROOT_ID {
                if !self.nodes[parent_id].expanded {
                    return None;
                }
                visible_index += 1;
            }
            let child_position = self.child_position(parent_id, child_id);
            visible_index += self.visible_count_before_child(parent_id, child_position);
        }
        Some(visible_index)
    }

    fn flattened_child_directory_id(&self, directory_id: NodeId) -> Option<NodeId> {
        if !self.flatten_empty_directories
            || directory_id == ROOT_ID
            || self.nodes[directory_id].kind != NodeKind::Directory
        {
            return None;
        }
        let directory = self.directories[directory_id].as_ref()?;
        if directory.child_ids.len() != 1 {
            return None;
        }
        let child_id = directory.child_ids[0];
        (self.nodes[child_id].kind == NodeKind::Directory).then_some(child_id)
    }

    fn flattened_terminal_directory_id(&self, node_id: NodeId) -> NodeId {
        let mut current_id = node_id;
        while let Some(next_id) = self.flattened_child_directory_id(current_id) {
            current_id = next_id;
        }
        current_id
    }

    fn recompute_visible_counts(&mut self) {
        for node_id in (0..self.nodes.len()).rev() {
            if self.nodes[node_id].kind != NodeKind::Directory {
                continue;
            }
            let child_ids = self.directories[node_id]
                .as_ref()
                .expect("directory node must have a child index")
                .child_ids
                .clone();
            let total = child_ids
                .iter()
                .map(|child_id| self.nodes[*child_id].visible_subtree_count)
                .sum();
            let chunk_sums = (child_ids.len() >= CHILD_SUMMARY_CHUNK_SIZE).then(|| {
                child_ids
                    .chunks(CHILD_SUMMARY_CHUNK_SIZE)
                    .map(|chunk| {
                        chunk
                            .iter()
                            .map(|child_id| self.nodes[*child_id].visible_subtree_count)
                            .sum()
                    })
                    .collect()
            });
            let directory = self.directories[node_id]
                .as_mut()
                .expect("directory node must have a child index");
            directory.total_child_visible_subtree_count = total;
            directory.child_visible_chunk_sums = chunk_sums;

            self.nodes[node_id].visible_subtree_count = if node_id == ROOT_ID {
                total
            } else {
                let terminal_id = self.flattened_terminal_directory_id(node_id);
                let terminal = &self.nodes[terminal_id];
                if terminal.expanded {
                    1 + self.directories[terminal_id]
                        .as_ref()
                        .expect("directory node must have a child index")
                        .total_child_visible_subtree_count
                } else {
                    1
                }
            };
        }
    }

    #[cfg(test)]
    pub fn directory_maps_are_lazy(&self, path: &str) -> bool {
        let node_id = if path.is_empty() {
            ROOT_ID
        } else {
            let parsed = split_path(path);
            let mut current = ROOT_ID;
            for segment in parsed.segments.iter() {
                let Some(name_id) = self.segments.id(segment) else {
                    return false;
                };
                let Some(directory) = self.directories[current].as_ref() else {
                    return false;
                };
                let Some(next) = directory
                    .child_ids
                    .iter()
                    .copied()
                    .find(|child_id| self.nodes[*child_id].name_id == name_id)
                else {
                    return false;
                };
                current = next;
            }
            current
        };
        self.directories[node_id].as_ref().is_some_and(|directory| {
            directory.child_id_by_name_id.is_none() && directory.child_position_by_id.is_none()
        })
    }

    #[cfg(test)]
    pub fn has_wide_directory_summary(&self, path: &str) -> bool {
        let parsed = split_path(path);
        let mut current = ROOT_ID;
        for segment in parsed.segments.iter() {
            let Some(name_id) = self.segments.id(segment) else {
                return false;
            };
            let Some(directory) = self.directories[current].as_ref() else {
                return false;
            };
            let Some(next) = directory
                .child_ids
                .iter()
                .copied()
                .find(|child_id| self.nodes[*child_id].name_id == name_id)
            else {
                return false;
            };
            current = next;
        }
        self.directories[current]
            .as_ref()
            .is_some_and(|directory| directory.child_visible_chunk_sums.is_some())
    }

    #[cfg(test)]
    pub fn cached_path_count(&self) -> usize {
        self.path_cache.borrow().len()
    }
}

struct StoreBuilder {
    nodes: Vec<Node>,
    directories: Vec<Option<DirectoryIndex>>,
    segments: SegmentTable,
    directory_stack: Vec<NodeId>,
    previous_segment_ids: Vec<SegmentId>,
    previous_directory_depth: usize,
}

impl StoreBuilder {
    fn new() -> Self {
        Self {
            nodes: vec![Node {
                parent_id: ROOT_ID,
                name_id: 0,
                depth: 0,
                kind: NodeKind::Directory,
                explicit: true,
                expanded: true,
                subtree_node_count: 1,
                visible_subtree_count: 0,
            }],
            directories: vec![Some(DirectoryIndex::new())],
            segments: SegmentTable::new(),
            directory_stack: vec![ROOT_ID],
            previous_segment_ids: Vec::new(),
            previous_directory_depth: 0,
        }
    }

    fn append<'a>(
        &mut self,
        path: &str,
        segments: impl IntoIterator<Item = &'a str>,
        is_directory: bool,
    ) -> Result<(), FileTreeInputError> {
        let segment_ids = segments
            .into_iter()
            .map(|segment| self.segments.intern(segment))
            .collect::<Vec<_>>();
        let current_directory_depth = if is_directory {
            segment_ids.len()
        } else {
            segment_ids.len().saturating_sub(1)
        };
        let shared_prefix_length = self
            .previous_segment_ids
            .iter()
            .zip(segment_ids.iter())
            .take_while(|(left, right)| left == right)
            .count();
        let shared_directory_depth = shared_prefix_length
            .min(current_directory_depth)
            .min(self.previous_directory_depth);
        self.directory_stack.truncate(shared_directory_depth + 1);

        for name_id in segment_ids
            .iter()
            .take(current_directory_depth)
            .skip(shared_directory_depth)
            .copied()
        {
            let parent_id = *self
                .directory_stack
                .last()
                .expect("directory stack always contains the root");
            let child_id = self.create_directory_child(parent_id, name_id, path)?;
            self.directory_stack.push(child_id);
        }

        if is_directory {
            let directory_id = *self
                .directory_stack
                .last()
                .expect("directory stack always contains the root");
            if self.nodes[directory_id].explicit {
                return Err(FileTreeInputError::DuplicatePath(path.to_owned()));
            }
            self.nodes[directory_id].explicit = true;
        } else {
            let parent_id = *self
                .directory_stack
                .last()
                .expect("directory stack always contains the root");
            let name_id = *segment_ids
                .last()
                .expect("paths always contain one segment");
            self.create_file_child(parent_id, name_id, path)?;
        }

        self.previous_segment_ids = segment_ids;
        self.previous_directory_depth = current_directory_depth;
        Ok(())
    }

    fn create_directory_child(
        &mut self,
        parent_id: NodeId,
        name_id: SegmentId,
        path: &str,
    ) -> Result<NodeId, FileTreeInputError> {
        if let Some(existing_id) = self.last_child_with_name(parent_id, name_id) {
            if self.nodes[existing_id].kind == NodeKind::Directory {
                return Ok(existing_id);
            }
            return Err(FileTreeInputError::PathCollidesWithFile {
                file: self.path_for_build_node(existing_id),
                path: path.to_owned(),
            });
        }

        Ok(self.push_node(parent_id, name_id, NodeKind::Directory))
    }

    fn create_file_child(
        &mut self,
        parent_id: NodeId,
        name_id: SegmentId,
        path: &str,
    ) -> Result<NodeId, FileTreeInputError> {
        if let Some(existing_id) = self.last_child_with_name(parent_id, name_id) {
            let file = if self.nodes[existing_id].kind == NodeKind::File {
                self.path_for_build_node(existing_id)
            } else {
                path.to_owned()
            };
            return Err(FileTreeInputError::PathCollidesWithFile {
                file,
                path: path.to_owned(),
            });
        }

        Ok(self.push_node(parent_id, name_id, NodeKind::File))
    }

    fn push_node(&mut self, parent_id: NodeId, name_id: SegmentId, kind: NodeKind) -> NodeId {
        let node_id = self.nodes.len();
        self.nodes.push(Node {
            parent_id,
            name_id,
            depth: self.nodes[parent_id].depth + 1,
            kind,
            explicit: kind == NodeKind::File,
            expanded: false,
            subtree_node_count: 1,
            visible_subtree_count: 1,
        });
        self.directories
            .push((kind == NodeKind::Directory).then(DirectoryIndex::new));
        self.directories[parent_id]
            .as_mut()
            .expect("node parent must be a directory")
            .child_ids
            .push(node_id);
        node_id
    }

    fn last_child_with_name(&self, parent_id: NodeId, name_id: SegmentId) -> Option<NodeId> {
        self.directories[parent_id]
            .as_ref()
            .expect("node parent must be a directory")
            .child_ids
            .last()
            .copied()
            .filter(|child_id| self.nodes[*child_id].name_id == name_id)
    }

    fn path_for_build_node(&self, node_id: NodeId) -> String {
        let mut segments = Vec::new();
        let mut current = node_id;
        while current != ROOT_ID {
            segments.push(self.segments.value(self.nodes[current].name_id));
            current = self.nodes[current].parent_id;
        }
        segments.reverse();
        let mut path = segments.join("/");
        if self.nodes[node_id].kind == NodeKind::Directory {
            path.push('/');
        }
        path
    }

    fn find_directory_node(&self, path: &str) -> Option<NodeId> {
        let parsed = split_path(path);
        let mut directory_id = ROOT_ID;

        for segment in parsed.segments {
            let name_id = self.segments.id(segment)?;
            let child_id = self.directories[directory_id]
                .as_ref()
                .expect("lookup parent must be a directory")
                .child_ids
                .iter()
                .copied()
                .find(|child_id| self.nodes[*child_id].name_id == name_id)?;
            if self.nodes[child_id].kind != NodeKind::Directory {
                return None;
            }
            directory_id = child_id;
        }

        (directory_id != ROOT_ID).then_some(directory_id)
    }

    fn finish(
        mut self,
        initial_expansion: FileTreeInitialExpansion,
        initial_expanded_paths: &[String],
        flatten_empty_directories: bool,
    ) -> FileTreeStore {
        for node_id in 1..self.nodes.len() {
            if self.nodes[node_id].kind != NodeKind::Directory {
                continue;
            }
            self.nodes[node_id].expanded = match initial_expansion {
                FileTreeInitialExpansion::Closed => false,
                FileTreeInitialExpansion::Open => true,
                FileTreeInitialExpansion::Depth(depth) => self.nodes[node_id].depth <= depth,
            };
        }
        for path in initial_expanded_paths {
            if let Some(node_id) = self.find_directory_node(path) {
                self.nodes[node_id].expanded = true;
            }
        }

        for node_id in (0..self.nodes.len()).rev() {
            if self.nodes[node_id].kind != NodeKind::Directory {
                continue;
            }
            let child_ids = self.directories[node_id]
                .as_ref()
                .expect("directory node must have a child index")
                .child_ids
                .clone();
            let total_child_subtree_node_count = child_ids
                .iter()
                .map(|child_id| self.nodes[*child_id].subtree_node_count)
                .sum();
            let total_child_visible_subtree_count = child_ids
                .iter()
                .map(|child_id| self.nodes[*child_id].visible_subtree_count)
                .sum();
            let chunk_sums = (child_ids.len() >= CHILD_SUMMARY_CHUNK_SIZE).then(|| {
                child_ids
                    .chunks(CHILD_SUMMARY_CHUNK_SIZE)
                    .map(|chunk| {
                        chunk
                            .iter()
                            .map(|child_id| self.nodes[*child_id].visible_subtree_count)
                            .sum()
                    })
                    .collect()
            });
            let directory = self.directories[node_id]
                .as_mut()
                .expect("directory node must have a child index");
            directory.total_child_subtree_node_count = total_child_subtree_node_count;
            directory.total_child_visible_subtree_count = total_child_visible_subtree_count;
            directory.child_visible_chunk_sums = chunk_sums;
            self.nodes[node_id].subtree_node_count = 1 + total_child_subtree_node_count;
            self.nodes[node_id].visible_subtree_count = if node_id == ROOT_ID {
                total_child_visible_subtree_count
            } else if self.nodes[node_id].expanded {
                1 + total_child_visible_subtree_count
            } else {
                1
            };
        }

        let mut store = FileTreeStore {
            nodes: self.nodes,
            directories: self.directories,
            segments: self.segments,
            path_cache: RefCell::new(HashMap::from([(ROOT_ID, String::new())])),
            flatten_empty_directories,
        };
        if flatten_empty_directories {
            store.recompute_visible_counts();
        }
        store
    }
}

fn add_signed(value: usize, delta: isize) -> usize {
    if delta.is_negative() {
        value - delta.unsigned_abs()
    } else {
        value + delta as usize
    }
}
