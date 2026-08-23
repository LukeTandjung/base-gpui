#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileTreeRowClickMode {
    Flow,
    Sticky,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileTreeSelectionDirective {
    Single,
    Toggle,
    Range { additive: bool },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FileTreeRowClickPlan {
    pub selection: FileTreeSelectionDirective,
    pub toggle_directory: bool,
    pub close_search: bool,
    pub reveal_canonical: bool,
}

pub fn compute_file_tree_row_click_plan(
    shift: bool,
    control: bool,
    platform: bool,
    mode: FileTreeRowClickMode,
    is_directory: bool,
) -> FileTreeRowClickPlan {
    let additive = control || platform;
    let has_modifier = shift || additive;
    let selection = if shift {
        FileTreeSelectionDirective::Range { additive }
    } else if additive {
        FileTreeSelectionDirective::Toggle
    } else {
        FileTreeSelectionDirective::Single
    };

    FileTreeRowClickPlan {
        selection,
        toggle_directory: !has_modifier && is_directory,
        close_search: false,
        reveal_canonical: mode == FileTreeRowClickMode::Sticky,
    }
}
