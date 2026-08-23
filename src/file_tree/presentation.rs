//! Public presentation data and composition callbacks for `FileTree`.
//!
//! This module uses GPUI elements for caller-provided icons and composition
//! surfaces. It does not expose Pierre's SVG sprite or CSS contracts.

use std::rc::Rc;

use gpui::{px, AnyElement, App, Bounds, Div, Pixels, Window};

/// A built-in spacing preset or a caller-provided spacing factor.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum FileTreeDensity {
    Compact,
    #[default]
    Default,
    Relaxed,
    Factor(f32),
}

/// Concrete row height and spacing factor derived from a density setting.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FileTreeDensityPreset {
    pub item_height: Pixels,
    pub factor: f32,
}

impl FileTreeDensity {
    pub const COMPACT: FileTreeDensityPreset = FileTreeDensityPreset {
        item_height: px(24.0),
        factor: 0.8,
    };
    pub const DEFAULT: FileTreeDensityPreset = FileTreeDensityPreset {
        item_height: px(30.0),
        factor: 1.0,
    };
    pub const RELAXED: FileTreeDensityPreset = FileTreeDensityPreset {
        item_height: px(36.0),
        factor: 1.2,
    };

    /// Resolves density with Pierre's item-height precedence.
    ///
    /// An explicit height always wins. A numeric factor uses the default
    /// 30-pixel height when no explicit height is present.
    pub fn resolve(self, explicit_item_height: Option<Pixels>) -> FileTreeDensityPreset {
        let preset = match self {
            Self::Compact => Self::COMPACT,
            Self::Default => Self::DEFAULT,
            Self::Relaxed => Self::RELAXED,
            Self::Factor(factor) => FileTreeDensityPreset {
                item_height: Self::DEFAULT.item_height,
                factor,
            },
        };
        FileTreeDensityPreset {
            item_height: explicit_item_height.unwrap_or(preset.item_height),
            factor: preset.factor,
        }
    }
}

/// Resolves an optional density. An absent density uses the default preset.
pub fn resolve_file_tree_density(
    density: Option<FileTreeDensity>,
    explicit_item_height: Option<Pixels>,
) -> FileTreeDensityPreset {
    density.unwrap_or_default().resolve(explicit_item_height)
}

/// The kind of a canonical file-tree item.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FileTreeItemKind {
    Directory,
    File,
}

/// One segment in a compact directory-chain row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileTreeVisibleSegment {
    pub path: String,
    pub name: String,
    pub is_terminal: bool,
}

/// Public, path-first state for one projected row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileTreeVisibleRow {
    pub ancestor_paths: Vec<String>,
    pub depth: usize,
    pub flattened_segments: Vec<FileTreeVisibleSegment>,
    pub has_children: bool,
    pub index: usize,
    pub is_expanded: bool,
    pub is_flattened: bool,
    pub is_focused: bool,
    pub is_selected: bool,
    pub kind: FileTreeItemKind,
    pub level: usize,
    pub name: String,
    pub path: String,
    pub position_in_set: usize,
    pub set_size: usize,
}

/// The icon tier supplied by the crate.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum FileTreeBuiltInIconSet {
    Minimal,
    Standard,
    #[default]
    Complete,
}

/// A config-level icon set selection. `None` disables all built-in file icons.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FileTreeIconSet {
    None,
    Minimal,
    Standard,
    Complete,
}

impl From<FileTreeBuiltInIconSet> for FileTreeIconSet {
    fn from(value: FileTreeBuiltInIconSet) -> Self {
        match value {
            FileTreeBuiltInIconSet::Minimal => Self::Minimal,
            FileTreeBuiltInIconSet::Standard => Self::Standard,
            FileTreeBuiltInIconSet::Complete => Self::Complete,
        }
    }
}

impl FileTreeIconSet {
    fn built_in(self) -> Option<FileTreeBuiltInIconSet> {
        match self {
            Self::None => None,
            Self::Minimal => Some(FileTreeBuiltInIconSet::Minimal),
            Self::Standard => Some(FileTreeBuiltInIconSet::Standard),
            Self::Complete => Some(FileTreeBuiltInIconSet::Complete),
        }
    }
}

/// A semantic file icon in Pierre's built-in tiers.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FileTreeBuiltInFileIcon {
    Astro,
    Babel,
    Bash,
    Biome,
    Bootstrap,
    Browserslist,
    Bun,
    C,
    Claude,
    Cpp,
    Css,
    Database,
    Default,
    Docker,
    Eslint,
    Font,
    Git,
    Go,
    Graphql,
    Html,
    Image,
    Javascript,
    Json,
    Markdown,
    Mcp,
    Nextjs,
    Npm,
    Oxc,
    Postcss,
    Prettier,
    Python,
    React,
    Ruby,
    Rust,
    Sass,
    Stylelint,
    Svelte,
    Svg,
    Svgo,
    Swift,
    Table,
    Tailwind,
    Terraform,
    Text,
    Typescript,
    Vite,
    Vscode,
    Vue,
    Wasm,
    Webpack,
    Yml,
    Zig,
    Zip,
}

impl FileTreeBuiltInFileIcon {
    fn is_standard(self) -> bool {
        matches!(
            self,
            Self::Bash
                | Self::C
                | Self::Cpp
                | Self::Css
                | Self::Database
                | Self::Default
                | Self::Font
                | Self::Git
                | Self::Go
                | Self::Html
                | Self::Image
                | Self::Javascript
                | Self::Json
                | Self::Markdown
                | Self::Mcp
                | Self::Python
                | Self::Ruby
                | Self::Rust
                | Self::Swift
                | Self::Table
                | Self::Text
                | Self::Typescript
                | Self::Zip
        )
    }
}

/// A semantic tree icon slot that a caller can replace.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FileTreeIconSlot {
    File,
    Folder,
    OpenFolder,
    DisclosureCollapsed,
    DisclosureExpanded,
    Search,
    Rename,
    ContextMenu,
    GitChangeDot,
    Lock,
}

/// Input passed to a caller-provided GPUI icon renderer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileTreeIconRenderContext {
    pub slot: FileTreeIconSlot,
    pub path: Option<String>,
    pub file_name: Option<String>,
    pub item_kind: Option<FileTreeItemKind>,
    pub expanded: bool,
}

/// A GPUI-native icon renderer.
pub type FileTreeIconRenderer =
    Rc<dyn Fn(FileTreeIconRenderContext, &mut Window, &mut App) -> AnyElement + 'static>;

/// An icon source used by a slot or matching rule.
#[derive(Clone)]
pub enum FileTreeIcon {
    BuiltIn(FileTreeBuiltInFileIcon),
    Render(FileTreeIconRenderer),
}

/// One ordered icon matching rule.
#[derive(Clone)]
pub struct FileTreeIconRule {
    pub matcher: String,
    pub icon: FileTreeIcon,
}

impl FileTreeIconRule {
    pub fn new(matcher: impl Into<String>, icon: FileTreeIcon) -> Self {
        Self {
            matcher: matcher.into(),
            icon,
        }
    }
}

/// GPUI-native icon configuration.
///
/// Rule vectors retain insertion order. This is observable for
/// `by_file_name_contains`, where the first matching rule wins.
#[derive(Clone, Default)]
pub struct FileTreeIconConfig {
    pub set: Option<FileTreeIconSet>,
    pub colored: Option<bool>,
    pub remap: Vec<(FileTreeIconSlot, FileTreeIcon)>,
    pub by_file_name: Vec<FileTreeIconRule>,
    pub by_file_extension: Vec<FileTreeIconRule>,
    pub by_file_name_contains: Vec<FileTreeIconRule>,
}

impl FileTreeIconConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(mut self, set: FileTreeIconSet) -> Self {
        self.set = Some(set);
        self
    }

    pub fn colored(mut self, colored: bool) -> Self {
        self.colored = Some(colored);
        self
    }

    pub fn remap(mut self, slot: FileTreeIconSlot, icon: FileTreeIcon) -> Self {
        self.remap.push((slot, icon));
        self
    }

    pub fn by_file_name(mut self, matcher: impl Into<String>, icon: FileTreeIcon) -> Self {
        self.by_file_name.push(FileTreeIconRule::new(matcher, icon));
        self
    }

    pub fn by_file_extension(mut self, matcher: impl Into<String>, icon: FileTreeIcon) -> Self {
        self.by_file_extension
            .push(FileTreeIconRule::new(matcher, icon));
        self
    }

    pub fn by_file_name_contains(mut self, matcher: impl Into<String>, icon: FileTreeIcon) -> Self {
        self.by_file_name_contains
            .push(FileTreeIconRule::new(matcher, icon));
        self
    }

    fn has_custom_rules(&self) -> bool {
        !self.remap.is_empty()
            || !self.by_file_name.is_empty()
            || !self.by_file_extension.is_empty()
            || !self.by_file_name_contains.is_empty()
    }
}

/// A built-in icon tier or a complete icon configuration.
#[derive(Clone)]
pub enum FileTreeIcons {
    BuiltIn(FileTreeBuiltInIconSet),
    Config(FileTreeIconConfig),
}

impl From<FileTreeBuiltInIconSet> for FileTreeIcons {
    fn from(value: FileTreeBuiltInIconSet) -> Self {
        Self::BuiltIn(value)
    }
}

impl From<FileTreeIconConfig> for FileTreeIcons {
    fn from(value: FileTreeIconConfig) -> Self {
        Self::Config(value)
    }
}

/// Normalized icon settings used by the resolver.
#[derive(Clone)]
pub struct NormalizedFileTreeIconConfig {
    /// `None` means custom-only icons.
    pub set: Option<FileTreeBuiltInIconSet>,
    pub colored: bool,
    pub remap: Vec<(FileTreeIconSlot, FileTreeIcon)>,
    pub by_file_name: Vec<FileTreeIconRule>,
    pub by_file_extension: Vec<FileTreeIconRule>,
    pub by_file_name_contains: Vec<FileTreeIconRule>,
}

/// Applies Pierre's icon defaults without copying its sprite representation.
pub fn normalize_file_tree_icons(icons: Option<&FileTreeIcons>) -> NormalizedFileTreeIconConfig {
    match icons {
        None => NormalizedFileTreeIconConfig {
            set: Some(FileTreeBuiltInIconSet::Complete),
            colored: true,
            remap: Vec::new(),
            by_file_name: Vec::new(),
            by_file_extension: Vec::new(),
            by_file_name_contains: Vec::new(),
        },
        Some(FileTreeIcons::BuiltIn(set)) => NormalizedFileTreeIconConfig {
            set: Some(*set),
            colored: true,
            remap: Vec::new(),
            by_file_name: Vec::new(),
            by_file_extension: Vec::new(),
            by_file_name_contains: Vec::new(),
        },
        Some(FileTreeIcons::Config(config)) => {
            let set = match config.set {
                Some(set) => set.built_in(),
                None if config.has_custom_rules() => None,
                None => Some(FileTreeBuiltInIconSet::Complete),
            };
            NormalizedFileTreeIconConfig {
                set,
                colored: config.colored.unwrap_or(true),
                remap: config.remap.clone(),
                by_file_name: normalize_rules(&config.by_file_name, false),
                by_file_extension: normalize_rules(&config.by_file_extension, true),
                by_file_name_contains: normalize_rules(&config.by_file_name_contains, false),
            }
        }
    }
}

fn normalize_rules(rules: &[FileTreeIconRule], trim: bool) -> Vec<FileTreeIconRule> {
    rules
        .iter()
        .map(|rule| FileTreeIconRule {
            matcher: if trim {
                rule.matcher.trim().to_lowercase()
            } else {
                rule.matcher.to_lowercase()
            },
            icon: rule.icon.clone(),
        })
        .collect()
}

/// The source selected for a requested tree icon.
#[derive(Clone)]
pub enum FileTreeResolvedIconSource {
    Slot(FileTreeIconSlot),
    BuiltIn(FileTreeBuiltInFileIcon),
    Render(FileTreeIconRenderer),
}

/// A resolved icon and the slot from which it was resolved.
#[derive(Clone)]
pub struct FileTreeResolvedIcon {
    pub source: FileTreeResolvedIconSource,
    pub remapped_from: FileTreeIconSlot,
}

impl FileTreeResolvedIcon {
    fn from_icon(icon: &FileTreeIcon, slot: FileTreeIconSlot) -> Self {
        let source = match icon {
            FileTreeIcon::BuiltIn(icon) => FileTreeResolvedIconSource::BuiltIn(*icon),
            FileTreeIcon::Render(render) => FileTreeResolvedIconSource::Render(render.clone()),
        };
        Self {
            source,
            remapped_from: slot,
        }
    }
}

/// A reusable icon resolver with normalized, ordered rules.
#[derive(Clone)]
pub struct FileTreeIconResolver {
    config: NormalizedFileTreeIconConfig,
}

impl FileTreeIconResolver {
    pub fn new(icons: Option<&FileTreeIcons>) -> Self {
        Self {
            config: normalize_file_tree_icons(icons),
        }
    }

    pub fn config(&self) -> &NormalizedFileTreeIconConfig {
        &self.config
    }

    /// Resolves a semantic slot. File matching applies only to the file slot.
    pub fn resolve(&self, slot: FileTreeIconSlot, file_path: Option<&str>) -> FileTreeResolvedIcon {
        if slot == FileTreeIconSlot::File {
            if let Some(file_path) = file_path {
                let file_name = base_file_name(file_path);
                let lower_file_name = file_name.to_lowercase();

                if let Some(rule) = self
                    .config
                    .by_file_name
                    .iter()
                    .find(|rule| rule.matcher == lower_file_name)
                {
                    return FileTreeResolvedIcon::from_icon(&rule.icon, slot);
                }

                if let Some(rule) = self
                    .config
                    .by_file_name_contains
                    .iter()
                    .find(|rule| lower_file_name.contains(&rule.matcher))
                {
                    return FileTreeResolvedIcon::from_icon(&rule.icon, slot);
                }

                let extension_candidates = extension_candidates(file_name);
                if let Some(rule) = extension_candidates.iter().find_map(|extension| {
                    self.config
                        .by_file_extension
                        .iter()
                        .find(|rule| rule.matcher == *extension)
                }) {
                    return FileTreeResolvedIcon::from_icon(&rule.icon, slot);
                }

                if let Some(set) = self.config.set {
                    if let Some(icon) =
                        resolve_builtin_file_icon(set, file_name, &extension_candidates)
                    {
                        if icon == FileTreeBuiltInFileIcon::Default {
                            if let Some(icon) = self.remapped_slot(slot) {
                                return FileTreeResolvedIcon::from_icon(icon, slot);
                            }
                        }
                        return FileTreeResolvedIcon {
                            source: FileTreeResolvedIconSource::BuiltIn(icon),
                            remapped_from: slot,
                        };
                    }
                }
            }
        }

        if let Some(icon) = self.remapped_slot(slot) {
            return FileTreeResolvedIcon::from_icon(icon, slot);
        }

        FileTreeResolvedIcon {
            source: FileTreeResolvedIconSource::Slot(slot),
            remapped_from: slot,
        }
    }

    fn remapped_slot(&self, slot: FileTreeIconSlot) -> Option<&FileTreeIcon> {
        self.config
            .remap
            .iter()
            .find_map(|(candidate, icon)| (*candidate == slot).then_some(icon))
    }
}

fn base_file_name(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}

fn extension_candidates(file_name: &str) -> Vec<String> {
    let segments = file_name.to_lowercase();
    let segments: Vec<_> = segments.split('.').collect();
    (1..segments.len())
        .map(|index| segments[index..].join("."))
        .collect()
}

fn resolve_builtin_file_icon(
    set: FileTreeBuiltInIconSet,
    file_name: &str,
    extension_candidates: &[String],
) -> Option<FileTreeBuiltInFileIcon> {
    if set == FileTreeBuiltInIconSet::Minimal {
        return None;
    }

    let complete = set == FileTreeBuiltInIconSet::Complete;
    if let Some(icon) = builtin_file_name_icon(&file_name.to_lowercase()) {
        if complete || icon.is_standard() {
            return Some(icon);
        }
    }

    for extension in extension_candidates {
        if complete {
            if let Some(icon) = complete_extension_override(extension) {
                return Some(icon);
            }
        }
        if let Some(icon) = builtin_extension_icon(extension) {
            if complete || icon.is_standard() {
                return Some(icon);
            }
        }
    }

    Some(FileTreeBuiltInFileIcon::Default)
}

fn builtin_file_name_icon(file_name: &str) -> Option<FileTreeBuiltInFileIcon> {
    use FileTreeBuiltInFileIcon as Icon;
    Some(match file_name {
        ".babelrc" | ".babelrc.json" | "babel.config.cjs" | "babel.config.js"
        | "babel.config.json" | "babel.config.mjs" => Icon::Babel,
        ".bash_profile" | ".bashrc" | ".zprofile" | ".zshenv" | ".zshrc" => Icon::Bash,
        ".browserslistrc" => Icon::Browserslist,
        ".dockerignore"
        | "compose.yaml"
        | "compose.yml"
        | "docker-compose.override.yml"
        | "docker-compose.yaml"
        | "docker-compose.yml"
        | "dockerfile" => Icon::Docker,
        ".eslintignore" | ".eslintrc" | ".eslintrc.cjs" | ".eslintrc.js" | ".eslintrc.json"
        | ".eslintrc.yaml" | ".eslintrc.yml" | "eslint.config.cjs" | "eslint.config.js"
        | "eslint.config.mjs" | "eslint.config.mts" | "eslint.config.ts" => Icon::Eslint,
        ".gitattributes" | ".gitignore" | ".gitkeep" | ".gitmodules" => Icon::Git,
        ".oxlintrc.json" => Icon::Oxc,
        ".postcssrc" | ".postcssrc.json" | ".postcssrc.yaml" | ".postcssrc.yml"
        | "postcss.config.cjs" | "postcss.config.js" | "postcss.config.mjs"
        | "postcss.config.ts" => Icon::Postcss,
        ".prettierignore"
        | ".prettierrc"
        | ".prettierrc.cjs"
        | ".prettierrc.js"
        | ".prettierrc.json"
        | ".prettierrc.mjs"
        | ".prettierrc.toml"
        | ".prettierrc.yaml"
        | ".prettierrc.yml"
        | "prettier.config.cjs"
        | "prettier.config.js"
        | "prettier.config.mjs" => Icon::Prettier,
        ".stylelintignore"
        | ".stylelintrc"
        | ".stylelintrc.cjs"
        | ".stylelintrc.js"
        | ".stylelintrc.json"
        | ".stylelintrc.mjs"
        | ".stylelintrc.yaml"
        | ".stylelintrc.yml"
        | "stylelint.config.cjs"
        | "stylelint.config.js"
        | "stylelint.config.mjs" => Icon::Stylelint,
        ".terraform.lock.hcl" => Icon::Terraform,
        "biome.json" | "biome.jsonc" => Icon::Biome,
        "bootstrap.bundle.js"
        | "bootstrap.bundle.min.js"
        | "bootstrap.css"
        | "bootstrap.js"
        | "bootstrap.min.css"
        | "bootstrap.min.js" => Icon::Bootstrap,
        "bun.lock" | "bun.lockb" | "bunfig.toml" => Icon::Bun,
        "claude.md" => Icon::Claude,
        "gemfile" | "rakefile" => Icon::Ruby,
        "next.config.js" | "next.config.mjs" | "next.config.mts" | "next.config.ts" => Icon::Nextjs,
        "readme.md" => Icon::Markdown,
        "svgo.config.cjs" | "svgo.config.js" | "svgo.config.mjs" | "svgo.config.ts" => Icon::Svgo,
        "tailwind.config.cjs"
        | "tailwind.config.js"
        | "tailwind.config.mjs"
        | "tailwind.config.ts" => Icon::Tailwind,
        "vite.config.js" | "vite.config.mjs" | "vite.config.mts" | "vite.config.ts" => Icon::Vite,
        "webpack.config.babel.js"
        | "webpack.config.cjs"
        | "webpack.config.js"
        | "webpack.config.mjs"
        | "webpack.config.ts" => Icon::Webpack,
        _ => return None,
    })
}

fn builtin_extension_icon(extension: &str) -> Option<FileTreeBuiltInFileIcon> {
    use FileTreeBuiltInFileIcon as Icon;
    Some(match extension {
        "7z" | "bz2" | "gz" | "jar" | "rar" | "tar" | "tgz" | "war" | "xz" | "zip" => Icon::Zip,
        "astro" => Icon::Astro,
        "avif" | "bmp" | "gif" | "icns" | "ico" | "jpeg" | "jpg" | "png" | "tif" | "tiff"
        | "webp" => Icon::Image,
        "bash" | "csh" | "fish" | "ksh" | "sh" | "zsh" => Icon::Bash,
        "c" | "h" => Icon::C,
        "cc" | "cpp" | "cxx" | "hh" | "hpp" | "hxx" | "inl" | "mm" => Icon::Cpp,
        "cfg" | "conf" | "editorconfig" | "env" | "env.development" | "env.local"
        | "env.production" | "ini" | "license" | "log" | "rst" | "rtf" | "txt" => Icon::Text,
        "cjs" | "js" | "jsx" | "mjs" => Icon::Javascript,
        "code-workspace" => Icon::Vscode,
        "css" | "less" | "sass" | "scss" | "styl" => Icon::Css,
        "csv" | "ods" | "tsv" | "xls" | "xlsx" => Icon::Table,
        "cts" | "mts" | "ts" | "tsx" => Icon::Typescript,
        "db" | "sql" | "sqlite" | "sqlite3" => Icon::Database,
        "eot" | "otf" | "ttf" | "woff" | "woff2" => Icon::Font,
        "erb" | "gemspec" | "rake" | "rb" => Icon::Ruby,
        "go" => Icon::Go,
        "gql" | "graphql" => Icon::Graphql,
        "htm" | "html" | "xhtml" => Icon::Html,
        "json" | "json5" | "jsonc" | "jsonl" => Icon::Json,
        "markdown" | "md" | "mdx" | "mdx.tsx" => Icon::Markdown,
        "mcp" => Icon::Mcp,
        "postcss" => Icon::Postcss,
        "py" | "pyi" | "pyw" | "pyx" => Icon::Python,
        "rs" => Icon::Rust,
        "svelte" => Icon::Svelte,
        "svg" => Icon::Svg,
        "swift" => Icon::Swift,
        "tf" | "tfstate" | "tfvars" => Icon::Terraform,
        "vue" => Icon::Vue,
        "wasm" | "wast" | "wat" => Icon::Wasm,
        "yaml" | "yml" => Icon::Yml,
        "zig" => Icon::Zig,
        _ => return None,
    })
}

fn complete_extension_override(extension: &str) -> Option<FileTreeBuiltInFileIcon> {
    use FileTreeBuiltInFileIcon as Icon;
    Some(match extension {
        "jsx" | "tsx" => Icon::React,
        "sass" | "scss" => Icon::Sass,
        _ => return None,
    })
}

/// A row's current drag/drop presentation state.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum FileTreeDropState {
    #[default]
    None,
    Dragging,
    RootTarget,
    RowTarget {
        path: String,
    },
    FlattenedSegmentTarget {
        path: String,
    },
    Rejected,
}

/// State supplied to the tree-level style callback.
#[derive(Clone, Debug, PartialEq)]
pub struct FileTreeStyleState {
    pub density: FileTreeDensityPreset,
    pub has_focus: bool,
    pub search_open: bool,
    pub renaming: bool,
    pub context_menu_open: bool,
    pub drop_state: FileTreeDropState,
}

/// State supplied to the row-level style callback.
#[derive(Clone, Debug, PartialEq)]
pub struct FileTreeRowStyleState {
    pub row: FileTreeVisibleRow,
    pub active: bool,
    pub hovered: bool,
    pub pressed: bool,
    pub sticky: bool,
    pub occluded: bool,
    pub renaming: bool,
    pub context_menu_open: bool,
    pub git_status: Option<super::git_status::FileTreeGitStatus>,
    pub contains_git_change: bool,
    pub drop_state: FileTreeDropState,
}

/// A tree style callback that can use live presentation state.
pub type FileTreeStyleWithState = Rc<dyn Fn(FileTreeStyleState, Div) -> Div + 'static>;

/// A row style callback that can use live row state.
pub type FileTreeRowStyleWithState = Rc<dyn Fn(FileTreeRowStyleState, Div) -> Div + 'static>;

/// A lightweight path-first item used by composition surfaces.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileTreeCompositionItem {
    pub kind: FileTreeItemKind,
    pub name: String,
    pub path: String,
}

impl From<&FileTreeVisibleRow> for FileTreeCompositionItem {
    fn from(row: &FileTreeVisibleRow) -> Self {
        Self {
            kind: row.kind,
            name: row.name.clone(),
            path: row.path.clone(),
        }
    }
}

/// Typed context supplied to a row-decoration renderer.
#[derive(Clone, Debug, PartialEq)]
pub struct FileTreeRowDecorationContext {
    pub item: FileTreeCompositionItem,
    pub row: FileTreeVisibleRow,
    pub drop_state: FileTreeDropState,
}

/// Renders an optional element in the row decoration lane.
pub type FileTreeRowDecorationRenderer =
    Rc<dyn Fn(FileTreeRowDecorationContext, &mut Window, &mut App) -> Option<AnyElement> + 'static>;

/// Renders an optional header above the virtualized tree rows.
pub type FileTreeHeaderRenderer = Rc<dyn Fn(&mut Window, &mut App) -> Option<AnyElement> + 'static>;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FileTreeContextMenuTriggerMode {
    Both,
    Button,
    #[default]
    RightClick,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FileTreeContextMenuButtonVisibility {
    Always,
    #[default]
    WhenNeeded,
}

/// The input that opened a context menu.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileTreeContextMenuSource {
    Button,
    Keyboard,
    RightClick,
}

/// Closes a context menu. `restore_focus` controls row focus restoration.
pub type FileTreeContextMenuClose = Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>;

/// Typed context for opening and rendering a context menu.
#[derive(Clone)]
pub struct FileTreeContextMenuOpenContext {
    pub item: FileTreeCompositionItem,
    pub row: FileTreeVisibleRow,
    pub anchor_bounds: Bounds<Pixels>,
    pub source: FileTreeContextMenuSource,
    pub drop_state: FileTreeDropState,
    pub close: FileTreeContextMenuClose,
}

pub type FileTreeContextMenuRenderer = Rc<
    dyn Fn(FileTreeContextMenuOpenContext, &mut Window, &mut App) -> Option<AnyElement> + 'static,
>;

pub type FileTreeContextMenuOpenHandler =
    Rc<dyn Fn(FileTreeContextMenuOpenContext, &mut Window, &mut App) + 'static>;

pub type FileTreeContextMenuCloseHandler = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

#[derive(Clone, Default)]
pub struct FileTreeHeaderComposition {
    pub render: Option<FileTreeHeaderRenderer>,
}

#[derive(Clone)]
pub struct FileTreeContextMenuComposition {
    pub enabled: bool,
    pub trigger_mode: FileTreeContextMenuTriggerMode,
    pub button_visibility: FileTreeContextMenuButtonVisibility,
    pub on_open: Option<FileTreeContextMenuOpenHandler>,
    pub on_close: Option<FileTreeContextMenuCloseHandler>,
    pub render: Option<FileTreeContextMenuRenderer>,
}

impl Default for FileTreeContextMenuComposition {
    fn default() -> Self {
        Self {
            enabled: true,
            trigger_mode: FileTreeContextMenuTriggerMode::RightClick,
            button_visibility: FileTreeContextMenuButtonVisibility::WhenNeeded,
            on_open: None,
            on_close: None,
            render: None,
        }
    }
}

impl FileTreeHeaderComposition {
    pub fn new(render: impl Fn(&mut Window, &mut App) -> Option<AnyElement> + 'static) -> Self {
        Self {
            render: Some(Rc::new(render)),
        }
    }
}

impl FileTreeContextMenuComposition {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn trigger_mode(mut self, mode: FileTreeContextMenuTriggerMode) -> Self {
        self.trigger_mode = mode;
        self
    }

    pub fn button_visibility(mut self, visibility: FileTreeContextMenuButtonVisibility) -> Self {
        self.button_visibility = visibility;
        self
    }

    pub fn on_open(
        mut self,
        handler: impl Fn(FileTreeContextMenuOpenContext, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open = Some(Rc::new(handler));
        self
    }

    pub fn on_close(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Rc::new(handler));
        self
    }

    pub fn render(
        mut self,
        render: impl Fn(FileTreeContextMenuOpenContext, &mut Window, &mut App) -> Option<AnyElement>
            + 'static,
    ) -> Self {
        self.render = Some(Rc::new(render));
        self
    }
}

impl FileTreeComposition {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn header(mut self, header: FileTreeHeaderComposition) -> Self {
        self.header = Some(header);
        self
    }

    pub fn context_menu(mut self, context_menu: FileTreeContextMenuComposition) -> Self {
        self.context_menu = Some(context_menu);
        self
    }
}

#[derive(Clone, Default)]
pub struct FileTreeComposition {
    pub header: Option<FileTreeHeaderComposition>,
    pub context_menu: Option<FileTreeContextMenuComposition>,
}

/// Two slices used by a GPUI middle-truncation layout.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FileTreeMiddleTruncation<'a> {
    pub start: &'a str,
    pub end: &'a str,
}

/// Splits a filename like Pierre's extension-aware `MiddleTruncate` use.
///
/// Names shorter than five characters are not split. For other names, the
/// split follows the final dot when the extension is at most ten characters.
/// Otherwise it uses a balanced, UTF-8-safe boundary.
pub fn split_file_name_for_middle_truncation(file_name: &str) -> FileTreeMiddleTruncation<'_> {
    let char_count = file_name.chars().count();
    if char_count < 5 {
        return FileTreeMiddleTruncation {
            start: file_name,
            end: "",
        };
    }

    if let Some(dot_index) = file_name.rfind('.') {
        let extension_start = dot_index + 1;
        let extension_length = file_name[extension_start..].chars().count();
        if extension_start >= 1 && extension_length <= 10 {
            return FileTreeMiddleTruncation {
                start: &file_name[..extension_start],
                end: &file_name[extension_start..],
            };
        }
    }

    let split_index = balanced_split_index(file_name);
    FileTreeMiddleTruncation {
        start: &file_name[..split_index],
        end: &file_name[split_index..],
    }
}

fn balanced_split_index(value: &str) -> usize {
    let characters: Vec<_> = value.chars().collect();
    let mut boundaries: Vec<_> = value.char_indices().map(|(index, _)| index).collect();
    boundaries.push(value.len());
    let center = characters.len().div_ceil(2);
    let is_whitespace_boundary = |index: usize| {
        characters
            .get(index.wrapping_sub(1))
            .is_some_and(|character| character.is_whitespace())
            || characters
                .get(index)
                .is_some_and(|character| character.is_whitespace())
    };

    if !is_whitespace_boundary(center) {
        return boundaries[center];
    }
    for offset in 1..characters.len() {
        let before = center.saturating_sub(offset);
        if before > 0 && !is_whitespace_boundary(before) {
            return boundaries[before];
        }
        let after = center + offset;
        if after < characters.len() && !is_whitespace_boundary(after) {
            return boundaries[after];
        }
    }
    boundaries[center]
}

/// Returns a character-budgeted filename with an extension-preserving ellipsis.
pub fn middle_truncate_file_name(file_name: &str, max_characters: usize) -> String {
    let char_count = file_name.chars().count();
    if char_count <= max_characters {
        return file_name.to_owned();
    }
    if max_characters == 0 {
        return String::new();
    }
    if max_characters == 1 {
        return "…".to_owned();
    }

    let split = split_file_name_for_middle_truncation(file_name);
    let end_count = split.end.chars().count().min(max_characters / 2);
    let start_count = max_characters - end_count - 1;
    let start: String = split.start.chars().take(start_count).collect();
    let end: String = split
        .end
        .chars()
        .rev()
        .take(end_count)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    format!("{start}…{end}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn builtin(source: &FileTreeResolvedIconSource) -> Option<FileTreeBuiltInFileIcon> {
        match source {
            FileTreeResolvedIconSource::BuiltIn(icon) => Some(*icon),
            FileTreeResolvedIconSource::Slot(_) | FileTreeResolvedIconSource::Render(_) => None,
        }
    }

    #[test]
    fn density_presets_and_explicit_height_match_pierre() {
        assert_eq!(
            FileTreeDensity::Compact.resolve(None),
            FileTreeDensityPreset {
                item_height: px(24.0),
                factor: 0.8,
            }
        );
        assert_eq!(
            FileTreeDensity::Default.resolve(None),
            FileTreeDensityPreset {
                item_height: px(30.0),
                factor: 1.0,
            }
        );
        assert_eq!(
            FileTreeDensity::Relaxed.resolve(None),
            FileTreeDensityPreset {
                item_height: px(36.0),
                factor: 1.2,
            }
        );
        assert_eq!(
            FileTreeDensity::Factor(1.5).resolve(None),
            FileTreeDensityPreset {
                item_height: px(30.0),
                factor: 1.5,
            }
        );
        assert_eq!(
            FileTreeDensity::Compact.resolve(Some(px(41.0))),
            FileTreeDensityPreset {
                item_height: px(41.0),
                factor: 0.8,
            }
        );
        assert_eq!(
            FileTreeDensity::Factor(1.5).resolve(Some(px(42.0))),
            FileTreeDensityPreset {
                item_height: px(42.0),
                factor: 1.5,
            }
        );
    }

    #[test]
    fn context_menu_defaults_to_pierre_right_click_mode() {
        let composition = FileTreeContextMenuComposition::default();
        assert!(composition.enabled);
        assert_eq!(
            composition.trigger_mode,
            FileTreeContextMenuTriggerMode::RightClick
        );
        assert_eq!(
            composition.button_visibility,
            FileTreeContextMenuButtonVisibility::WhenNeeded
        );
    }

    #[test]
    fn icon_normalization_matches_pierre_defaults() {
        let absent = normalize_file_tree_icons(None);
        assert_eq!(absent.set, Some(FileTreeBuiltInIconSet::Complete));
        assert!(absent.colored);

        let standard = FileTreeIcons::BuiltIn(FileTreeBuiltInIconSet::Standard);
        let standard = normalize_file_tree_icons(Some(&standard));
        assert_eq!(standard.set, Some(FileTreeBuiltInIconSet::Standard));
        assert!(standard.colored);

        let empty = FileTreeIcons::Config(FileTreeIconConfig::default());
        assert_eq!(
            normalize_file_tree_icons(Some(&empty)).set,
            Some(FileTreeBuiltInIconSet::Complete)
        );

        let custom = FileTreeIcons::Config(FileTreeIconConfig {
            by_file_name: vec![FileTreeIconRule::new(
                "README.md",
                FileTreeIcon::BuiltIn(FileTreeBuiltInFileIcon::Text),
            )],
            ..FileTreeIconConfig::default()
        });
        assert_eq!(normalize_file_tree_icons(Some(&custom)).set, None);

        let explicitly_none = FileTreeIcons::Config(FileTreeIconConfig {
            set: Some(FileTreeIconSet::None),
            ..FileTreeIconConfig::default()
        });
        assert_eq!(normalize_file_tree_icons(Some(&explicitly_none)).set, None);
    }

    #[test]
    fn icon_resolution_uses_pierre_rule_precedence() {
        let icons = FileTreeIcons::Config(FileTreeIconConfig {
            set: Some(FileTreeIconSet::Complete),
            remap: vec![(
                FileTreeIconSlot::File,
                FileTreeIcon::BuiltIn(FileTreeBuiltInFileIcon::Text),
            )],
            by_file_name: vec![FileTreeIconRule::new(
                "readme.md",
                FileTreeIcon::BuiltIn(FileTreeBuiltInFileIcon::Git),
            )],
            by_file_name_contains: vec![
                FileTreeIconRule::new(
                    "config",
                    FileTreeIcon::BuiltIn(FileTreeBuiltInFileIcon::Json),
                ),
                FileTreeIconRule::new("vite", FileTreeIcon::BuiltIn(FileTreeBuiltInFileIcon::Vite)),
            ],
            by_file_extension: vec![
                FileTreeIconRule::new(
                    "ts",
                    FileTreeIcon::BuiltIn(FileTreeBuiltInFileIcon::Typescript),
                ),
                FileTreeIconRule::new(
                    "spec.ts",
                    FileTreeIcon::BuiltIn(FileTreeBuiltInFileIcon::Rust),
                ),
            ],
            colored: None,
        });
        let resolver = FileTreeIconResolver::new(Some(&icons));

        assert_eq!(
            builtin(
                &resolver
                    .resolve(FileTreeIconSlot::File, Some("DOCS/README.MD"))
                    .source
            ),
            Some(FileTreeBuiltInFileIcon::Git)
        );
        assert_eq!(
            builtin(
                &resolver
                    .resolve(FileTreeIconSlot::File, Some("VITE.CONFIG.TS"))
                    .source
            ),
            Some(FileTreeBuiltInFileIcon::Json),
            "the first contains rule wins"
        );
        assert_eq!(
            builtin(
                &resolver
                    .resolve(FileTreeIconSlot::File, Some("unit.SPEC.TS"))
                    .source
            ),
            Some(FileTreeBuiltInFileIcon::Rust),
            "the longest compound extension is checked first"
        );
        assert_eq!(
            builtin(
                &resolver
                    .resolve(FileTreeIconSlot::File, Some("src/index.tsx"))
                    .source
            ),
            Some(FileTreeBuiltInFileIcon::React),
            "a specific built-in token wins over the generic file remap"
        );
        assert_eq!(
            builtin(
                &resolver
                    .resolve(FileTreeIconSlot::File, Some("unknown.xyz"))
                    .source
            ),
            Some(FileTreeBuiltInFileIcon::Text),
            "the generic file remap wins over the built-in default token"
        );
    }

    #[test]
    fn standard_and_minimal_built_in_tiers_match_pierre() {
        let standard = FileTreeIconResolver::new(Some(&FileTreeIcons::BuiltIn(
            FileTreeBuiltInIconSet::Standard,
        )));
        assert_eq!(
            builtin(
                &standard
                    .resolve(FileTreeIconSlot::File, Some("component.tsx"))
                    .source
            ),
            Some(FileTreeBuiltInFileIcon::Typescript)
        );
        assert_eq!(
            builtin(
                &standard
                    .resolve(FileTreeIconSlot::File, Some("vite.config.ts"))
                    .source
            ),
            Some(FileTreeBuiltInFileIcon::Typescript),
            "complete-only exact icons fall through to a standard extension"
        );

        let minimal = FileTreeIconResolver::new(Some(&FileTreeIcons::BuiltIn(
            FileTreeBuiltInIconSet::Minimal,
        )));
        assert!(matches!(
            minimal
                .resolve(FileTreeIconSlot::File, Some("index.ts"))
                .source,
            FileTreeResolvedIconSource::Slot(FileTreeIconSlot::File)
        ));
    }

    #[test]
    fn filename_middle_truncation_is_unicode_safe_and_extension_aware() {
        assert_eq!(
            split_file_name_for_middle_truncation("component.spec.ts"),
            FileTreeMiddleTruncation {
                start: "component.spec.",
                end: "ts",
            }
        );
        assert_eq!(
            split_file_name_for_middle_truncation("readme"),
            FileTreeMiddleTruncation {
                start: "rea",
                end: "dme",
            }
        );
        assert_eq!(
            split_file_name_for_middle_truncation("éclair.rs"),
            FileTreeMiddleTruncation {
                start: "éclair.",
                end: "rs",
            }
        );
        assert_eq!(
            split_file_name_for_middle_truncation("hello world"),
            FileTreeMiddleTruncation {
                start: "hello w",
                end: "orld",
            }
        );
        assert_eq!(
            middle_truncate_file_name("component.spec.ts", 10),
            "compone…ts"
        );
        assert_eq!(middle_truncate_file_name("éclair.rs", 6), "écl…rs");
        assert_eq!(middle_truncate_file_name("abc", 8), "abc");
    }
}
