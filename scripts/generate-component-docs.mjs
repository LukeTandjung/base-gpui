import { readdir, readFile, mkdir, writeFile } from "node:fs/promises";
import path from "node:path";

const root = path.resolve(import.meta.dirname, "..");
const srcRoot = path.join(root, "src");
const docsRoot = path.join(root, "docs", "components");
const excluded = new Set(["primitives", "utils"]);

const titleCase = (name) => name.split("_").map((part) => part === "otp" ? "OTP" : part[0].toUpperCase() + part.slice(1)).join(" ");
const kebab = (name) => name.replaceAll("_", "-");
const sourceLink = (component, file) => `../../src/${component}/${file}`;

function exportedNames(source, moduleName) {
  const marker = `pub use ${moduleName}::`;
  const start = source.indexOf(marker);
  if (start < 0) return [];
  const end = source.indexOf(";", start);
  return source.slice(start + marker.length, end).replace(/[{}]/g, "").split(",").map((value) => value.trim()).filter(Boolean);
}

function firstModuleParagraph(source, fallback) {
  const docs = source.split("\n").filter((line) => line.startsWith("//!")).map((line) => line.replace(/^\/\/!\s?/, "").trim());
  const paragraph = [];
  for (const line of docs) {
    if (!line && paragraph.length) break;
    if (line && !line.startsWith("#")) paragraph.push(line);
  }
  return paragraph.join(" ") || fallback;
}

function matchingBrace(source, open) {
  let depth = 0;
  let string = false;
  for (let index = open; index < source.length; index += 1) {
    const char = source[index];
    if (char === '"' && source[index - 1] !== "\\") string = !string;
    if (string) continue;
    if (char === "{") depth += 1;
    if (char === "}" && --depth === 0) return index;
  }
  return -1;
}

function methodDocs(source, index) {
  const before = source.slice(0, index).split("\n");
  const docs = [];
  for (let line = before.length - 1; line >= 0; line -= 1) {
    const value = before[line].trim();
    if (value.startsWith("///")) docs.unshift(value.replace(/^\/\/\/\s?/, ""));
    else if (!value) continue;
    else break;
  }
  return docs.join(" ");
}

function methodsForStruct(source, structName) {
  const methods = [];
  const implPattern = new RegExp(`impl(?:\\s*<[^{}]*>)?\\s+${structName}(?:\\s*<[^{}]*>)?\\s*\\{`, "g");
  for (const implMatch of source.matchAll(implPattern)) {
    const open = source.indexOf("{", implMatch.index);
    const close = matchingBrace(source, open);
    if (close < 0) continue;
    const body = source.slice(open + 1, close);
    const methodPattern = /pub fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*(?:<[^{};]*?>)?\s*\(/g;
    for (const match of body.matchAll(methodPattern)) {
      const absolute = open + 1 + match.index;
      let paren = body.indexOf("(", match.index);
      let depth = 0;
      let endParen = -1;
      for (let index = paren; index < body.length; index += 1) {
        if (body[index] === "(") depth += 1;
        if (body[index] === ")" && --depth === 0) { endParen = index; break; }
      }
      if (endParen < 0) continue;
      const after = body.slice(endParen + 1);
      const returnMatch = after.match(/^\s*(?:->\s*([^\{]+))?\s*\{/);
      if (!returnMatch) continue;
      const params = body.slice(paren + 1, endParen).trim();
      const signature = `pub fn ${match[1]}(${params})${returnMatch[1] ? ` -> ${returnMatch[1].trim()}` : ""}`.replace(/\s+/g, " ");
      methods.push({ name: match[1], params, signature, docs: methodDocs(source, absolute) });
    }
  }
  return methods.filter((method, index, all) => all.findIndex((candidate) => candidate.signature === method.signature) === index);
}

function parameters(params) {
  const values = [];
  let depth = 0;
  let start = 0;
  for (let index = 0; index <= params.length; index += 1) {
    const char = params[index];
    if ("(<[{".includes(char)) depth += 1;
    if (")>]}".includes(char)) depth -= 1;
    if ((char === "," && depth === 0) || index === params.length) {
      const value = params.slice(start, index).trim();
      if (value && !value.includes("self")) values.push(value);
      start = index + 1;
    }
  }
  return values;
}

function purposeForPart(name) {
  const suffixes = [
    ["Root", "Coordinates the component's state and supplies context to its child parts."],
    ["Trigger", "Interactive control that opens, closes, or activates the component."],
    ["Popup", "Contains the floating interactive content."],
    ["Portal", "Hosts overlay content outside the normal child layout."],
    ["Positioner", "Measures the anchor and positions floating content."],
    ["Backdrop", "Covers content behind a modal overlay and handles outside interaction."],
    ["Viewport", "Defines the viewport used to lay out or constrain overlay content."],
    ["List", "Contains and coordinates the component's collection items."],
    ["ItemIndicator", "Visual indicator for an item's selected or checked state."],
    ["Indicator", "Visual representation of the component's current state or value."],
    ["Item", "Represents one interactive item in the component's collection."],
    ["Label", "Provides a visible label and associated accessibility semantics."],
    ["Description", "Provides supporting descriptive content."],
    ["Title", "Provides the component's visible title."],
    ["Close", "Interactive control that closes the component."],
    ["Panel", "Contains content associated with the active item."],
    ["Input", "Text input integrated with the component's state and behavior."],
    ["Value", "Displays or edits the component's current value."],
    ["Group", "Groups related child parts and coordinates their shared behavior."],
    ["Separator", "Visual separator between neighboring items or groups."],
    ["Arrow", "Optional arrow that visually points toward the overlay anchor."],
    ["Track", "Visual track containing the component's indicator or thumb."],
    ["Thumb", "Interactive or visual handle representing the current value."],
    ["Content", "Contains the component's user-provided content."],
    ["Icon", "Optional visual icon for the component."],
  ];
  return suffixes.find(([suffix]) => name.endsWith(suffix))?.[1] ?? `Public renderable part of the ${name.replace(/([a-z])([A-Z])/g, "$1 $2")} component.`;
}

function methodDescription(method, part) {
  if (method.docs) return method.docs;
  const name = method.name;
  const descriptions = {
    new: `Creates a ${part} with its default configuration.`,
    id: "Sets the stable GPUI element identity. Use a unique value when multiple instances can appear in the same view.",
    child: "Adds one typed child to this part.",
    child_any: "Adds one arbitrary renderable child to this part.",
    children: "Adds multiple typed children in iteration order.",
    style_with_state: "Styles the part from its current behavioral state while preserving separation between behavior and visual design.",
    value: "Sets the current controlled value or the value represented by this part, depending on the part's role.",
    default_value: "Sets the initial value for uncontrolled state. Later user changes are retained by the component.",
    open: "Controls whether the component is open.",
    default_open: "Sets the initial open state for an uncontrolled component.",
    disabled: "When true, prevents user interaction with this part.",
    read_only: "When true, allows presentation and focus behavior without permitting value changes.",
    required: "Marks the control as required for validation and accessibility state.",
    keep_mounted: "Keeps the part mounted when inactive or closed so child state can be preserved.",
    orientation: "Sets the component's horizontal or vertical orientation and corresponding keyboard behavior.",
    aria_label: "Sets a literal accessible name for this part.",
    loop_focus: "Controls whether keyboard focus wraps from the last enabled item to the first and vice versa.",
    name: "Sets the form field name used when serializing or submitting the value.",
    placeholder: "Sets the content shown when the component has no current value.",
    modal: "Controls whether the overlay behaves modally and blocks interaction outside it.",
    side: "Sets the preferred side of the anchor on which floating content appears.",
    align: "Sets floating content alignment along the selected side.",
    side_offset: "Sets the distance between floating content and its anchor.",
    align_offset: "Offsets floating content along its alignment axis.",
    collision_padding: "Sets the minimum space retained between floating content and viewport edges.",
    delay: "Sets the delay before the related transition or interaction begins.",
    close_delay: "Sets the delay before an open component closes after the closing interaction.",
    timeout: "Sets the duration before the component automatically performs its timeout behavior.",
    multiple: "Controls whether more than one value may be selected at the same time.",
  };
  if (descriptions[name]) return descriptions[name];
  if (name.startsWith("on_")) return `Registers a callback invoked when ${name.slice(3).replaceAll("_", " ")} occurs.`;
  if (name.startsWith("default_")) return `Sets the initial ${name.slice(8).replaceAll("_", " ")} for uncontrolled state.`;
  if (/^(is_|has_|force_|auto_|disable_|close_|open_|select_|highlight_|trap_)/.test(name)) return `Controls whether ${name.replace(/^(is_|has_)/, "").replaceAll("_", " ")} behavior is enabled.`;
  return `Sets the ${name.replaceAll("_", " ")} configuration for this part.`;
}

function placeholderFor(parameter) {
  const separator = parameter.indexOf(":");
  const name = separator < 0 ? "value" : parameter.slice(0, separator).trim().replace(/^mut\s+/, "");
  const type = separator < 0 ? parameter : parameter.slice(separator + 1).trim();
  if (type === "bool") return "true";
  if (/usize|u(8|16|32|64|128)|i(8|16|32|64|128)/.test(type)) return "0";
  if (/f32|f64/.test(type)) return "0.0";
  if (name.includes("id")) return '"example-id"';
  if (name.includes("label") || name.includes("placeholder") || name === "name") return `"${name.replaceAll("_", " ")}"`;
  if (type.startsWith("Option<")) return "None";
  if (type.includes("Fn(")) return "|/* callback arguments */| { /* handle change */ }";
  if (name === "child") return "/* child */";
  if (name === "children") return "/* children */";
  return `/* ${name}: ${type} */`;
}

function partSnippet(part, methods) {
  const builders = methods.filter((method) => !["new", "child", "child_any", "children"].includes(method.name));
  const lines = [`${part}::new()`];
  for (const method of builders) {
    const args = parameters(method.params).map(placeholderFor).join(", ");
    lines.push(`    .${method.name}(${args})`);
  }
  return `${lines.join("\n")};`;
}

function childEnums(source) {
  const enums = new Map();
  for (const match of source.matchAll(/pub enum\s+([A-Za-z0-9_]+Child)(?:<[^{}]*>)?\s*\{/g)) {
    const open = source.indexOf("{", match.index);
    const close = matchingBrace(source, open);
    const body = source.slice(open + 1, close);
    const children = [...body.matchAll(/(?:Box<)?([A-Z][A-Za-z0-9_]+)(?:<[^>]*>)?>?\s*(?:,|\))/g)]
      .map((item) => item[1])
      .filter((name) => name !== "AnyElement");
    enums.set(match[1], [...new Set(children)]);
  }
  return enums;
}

function constructor(part) {
  const method = part.methods.find(({ name }) => name === "new");
  const args = method ? parameters(method.params).map(placeholderFor).join(", ") : "";
  return `${part.name}::new(${args})`;
}

function anatomy(partData, childSource) {
  if (!partData.length) return "// This module has no renderable compound parts.";
  const names = partData.map(({ name }) => name);
  const enums = childEnums(childSource);
  const rootPart = partData.find(({ name }) => name.endsWith("Root") && !name.includes("Submenu")) ?? partData[0];
  const parentFor = new Map();

  for (const child of partData) {
    if (child.name === rootPart.name || child.name.includes("Provider")) continue;
    const candidates = [];
    for (const parent of partData) {
      if (parent.name === child.name) continue;
      const childMethod = parent.methods.find(({ name }) => name === "child");
      const enumName = childMethod?.signature.match(/Into<([A-Za-z0-9_]+Child)/)?.[1];
      if (enumName && enums.get(enumName)?.includes(child.name)) {
        const specificity = enumName.startsWith(parent.name) ? parent.name.length + 1000 : parent.name.length;
        candidates.push({ parent: parent.name, specificity });
      }
    }
    candidates.sort((a, b) => b.specificity - a.specificity);
    parentFor.set(child.name, candidates[0]?.parent ?? rootPart.name);
  }

  const children = new Map(names.map((name) => [name, []]));
  for (const [child, parent] of parentFor) {
    if (children.has(parent) && parent !== child) children.get(parent).push(child);
  }
  const attached = new Set(parentFor.keys());
  for (const part of partData) {
    if (part.name !== rootPart.name && !part.name.includes("Provider") && !attached.has(part.name)) children.get(rootPart.name).push(part.name);
  }

  const render = (name, depth = 0, seen = new Set()) => {
    const part = partData.find((candidate) => candidate.name === name);
    if (!part || seen.has(name)) return `${name}::new()`;
    const nextSeen = new Set(seen).add(name);
    let value = constructor(part);
    for (const child of children.get(name) ?? []) {
      const rendered = render(child, depth + 1, nextSeen);
      const indent = "    ".repeat(depth + 1);
      value += `\n${indent}.child(\n${indent}    ${rendered.replaceAll("\n", `\n${indent}    `)},\n${indent})`;
    }
    return value;
  };

  return `${render(rootPart.name)};`;
}

await mkdir(docsRoot, { recursive: true });
const components = (await readdir(srcRoot, { withFileTypes: true })).filter((entry) => entry.isDirectory() && !excluded.has(entry.name)).sort((a, b) => a.name.localeCompare(b.name));
const indexRows = [];

for (const entry of components) {
  const component = entry.name;
  const componentRoot = path.join(srcRoot, component);
  const modSource = await readFile(path.join(componentRoot, "mod.rs"), "utf8");
  const title = titleCase(component);
  const exportedLayers = exportedNames(modSource, "layers");
  const partData = [];
  try {
    for (const file of (await readdir(path.join(componentRoot, "layers"))).filter((name) => name.endsWith(".rs") && name !== "mod.rs").sort()) {
      const source = await readFile(path.join(componentRoot, "layers", file), "utf8");
      for (const match of source.matchAll(/\bpub struct\s+([A-Z][A-Za-z0-9_]*)/g)) {
        if (exportedLayers.includes(match[1])) partData.push({ name: match[1], file, methods: methodsForStruct(source, match[1]) });
      }
    }
  } catch {}
  const parts = partData.map(({ name }) => name).sort();
  indexRows.push(`| [${title}](${component}.md) | ${parts.map((part) => `\`${part}\``).join(", ") || "—"} |`);

  const guidePath = path.join(docsRoot, `${component}.md`);
  let existing = "";
  try { existing = await readFile(guidePath, "utf8"); } catch {}
  if (existing.startsWith("<!-- curated-component-guide -->")) continue;

  let childSource = "";
  try { childSource = await readFile(path.join(componentRoot, "child.rs"), "utf8"); } catch {}
  const summary = firstModuleParagraph(modSource, `${title} ports Base UI's ${title} component model to GPUI.`);
  const lines = [
    `# ${title}`, "", summary, "",
    `[Base UI reference](https://base-ui.com/react/components/${kebab(component)}) · [Source](${sourceLink(component, "mod.rs")})`, "",
    "## Anatomy", "",
    `Import the component's parts and compose them under its root:`, "",
    "```rust", `use base_gpui::${component}::{`, ...parts.map((part) => `    ${part},`), "};", "", anatomy(partData, childSource), "```", "",
    "> The anatomy is a structural overview. Parts with mutually exclusive modes may need separate instances; each part's section below documents its configuration.", "",
  ];

  for (const part of partData.sort((a, b) => {
    const rootOrder = (name) => name.endsWith("Root") ? 0 : name.endsWith("Trigger") ? 1 : name.endsWith("Portal") ? 2 : name.endsWith("Positioner") ? 3 : name.endsWith("Popup") ? 4 : 5;
    return rootOrder(a.name) - rootOrder(b.name) || a.name.localeCompare(b.name);
  })) {
    lines.push(`## \`${part.name}\``, "", purposeForPart(part.name), "", `[Source](${sourceLink(component, `layers/${part.file}`)})`, "", "```rust", `use base_gpui::${component}::${part.name};`, "", partSnippet(part.name, part.methods), "```", "", "### Builders", "");
    for (const method of part.methods) {
      lines.push(`#### \`.${method.name}(...)\``, "", "```rust", method.signature, "```", "");
      const params = parameters(method.params);
      if (params.length) {
        lines.push("**Accepts**", "");
        for (const parameter of params) {
          const separator = parameter.indexOf(":");
          lines.push(separator < 0 ? `- \`${parameter}\`` : `- \`${parameter.slice(0, separator).trim()}\`: \`${parameter.slice(separator + 1).trim()}\``);
        }
        lines.push("");
      }
      lines.push(methodDescription(method, part.name), "");
    }
    lines.push(`${part.name} also supports the GPUI traits implemented in its source, such as standard styling or child composition.`, "");
  }

  lines.push("## Accessibility", "", "Keyboard interaction and accessibility semantics are implemented by the component, independently of visual styling. Known limitations caused by missing GPUI accessibility primitives are documented in the module source and are not silently approximated.", "", "## Stability", "", "Base GPUI is pre-1.0. Builder names and state types may evolve as GPUI and this port mature.", "");
  await writeFile(guidePath, lines.join("\n"));
}

await writeFile(path.join(docsRoot, "README.md"), [
  "# Component guides", "",
  "Each guide starts with the component anatomy, then documents every public layer from a consumer's perspective: how to instantiate it, which builders it exposes, the exact values those builders accept, and what each builder controls.", "",
  "The public API portions are generated from source. Guides marked as curated contain reviewed composition, behavior, defaults, keyboard interaction, and accessibility guidance.", "",
  "| Component | Compound parts |", "| --- | --- |", ...indexRows, "",
].join("\n"));
