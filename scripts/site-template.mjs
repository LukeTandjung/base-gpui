// Pure rendering layer for the base-gpui showcase site: markdown → HTML,
// page shell, styles, nav, table of contents, search index. No filesystem
// access lives here so the module can be imported and unit-tested anywhere.
//
// Consumed by scripts/build-site.mjs.

export const repoUrl = "https://github.com/LukeTandjung/base-gpui";
export const baseUiUrl = "https://base-ui.com";

export const titleCase = (name) =>
  name
    .split("_")
    .map((part) => (part === "otp" ? "OTP" : part[0].toUpperCase() + part.slice(1)))
    .join(" ");

export const escapeHtml = (text) =>
  text.replaceAll("&", "&amp;").replaceAll("<", "&lt;").replaceAll(">", "&gt;");

export const escapeAttr = (text) => escapeHtml(text).replaceAll('"', "&quot;");

export const anchorId = (text) =>
  text
    .replace(/`/g, "")
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-|-$/g, "");

// ── Inline markdown ───────────────────────────────────────────────────────────

export function inline(text) {
  let html = escapeHtml(text);
  html = html.replace(/`([^`]+)`/g, (_, code) => `<code>${code}</code>`);
  html = html.replace(/\*\*([^*]+)\*\*/g, "<strong>$1</strong>");
  html = html.replace(/\[([^\]]+)\]\(([^)]+)\)/g, (_, label, href) => {
    const resolved = href.startsWith("../../")
      ? `${repoUrl}/blob/main/${href.replace("../../", "")}`
      : href.startsWith("../")
        ? `${repoUrl}/blob/main/docs/${href.replace("../", "")}`
        : href;
    const external = /^https?:/.test(resolved);
    const attrs = external ? ' target="_blank" rel="noreferrer"' : "";
    return `<a href="${escapeAttr(resolved)}"${attrs}>${label}</a>`;
  });
  return html;
}

// ── Tokenizer for the constrained markdown the docs generator emits ───────────
// Handles: h1–h4, paragraphs, fenced code, blockquotes, dash lists, ordered
// lists, pipe tables and thematic breaks.

export function tokenize(markdown) {
  const lines = markdown.replace(/\r\n/g, "\n").split("\n");
  const tokens = [];
  let i = 0;
  while (i < lines.length) {
    const line = lines[i];

    if (line.startsWith("```")) {
      const lang = line.slice(3).trim();
      const body = [];
      i += 1;
      while (i < lines.length && !lines[i].startsWith("```")) {
        body.push(lines[i]);
        i += 1;
      }
      i += 1;
      tokens.push({ type: "code", lang, text: body.join("\n") });
      continue;
    }

    const heading = line.match(/^(#{1,4}) (.*)$/);
    if (heading) {
      tokens.push({ type: "heading", level: heading[1].length, text: heading[2].trim() });
      i += 1;
      continue;
    }

    if (/^(---+|\*\*\*+)$/.test(line.trim())) {
      tokens.push({ type: "hr" });
      i += 1;
      continue;
    }

    if (line.startsWith(">")) {
      const body = [];
      while (i < lines.length && lines[i].startsWith(">")) {
        body.push(lines[i].replace(/^>\s?/, ""));
        i += 1;
      }
      tokens.push({ type: "quote", text: body.join(" ").trim() });
      continue;
    }

    if (line.startsWith("|")) {
      const rows = [];
      while (i < lines.length && lines[i].startsWith("|")) {
        rows.push(lines[i]);
        i += 1;
      }
      const cells = (row) =>
        row
          .replace(/^\|/, "")
          .replace(/\|$/, "")
          .split("|")
          .map((cell) => cell.trim());
      const head = cells(rows[0]);
      const body = rows.slice(rows[1] && /^[|\s:-]+$/.test(rows[1]) ? 2 : 1).map(cells);
      tokens.push({ type: "table", head, body });
      continue;
    }

    if (/^- /.test(line)) {
      const items = [];
      while (i < lines.length && /^- /.test(lines[i])) {
        const item = [lines[i].slice(2)];
        i += 1;
        while (i < lines.length && /^\s{2,}\S/.test(lines[i])) {
          item.push(lines[i].trim());
          i += 1;
        }
        items.push(item.join(" "));
      }
      tokens.push({ type: "list", ordered: false, items });
      continue;
    }

    if (/^\d+\. /.test(line)) {
      const items = [];
      while (i < lines.length && /^\d+\. /.test(lines[i])) {
        const item = [lines[i].replace(/^\d+\. /, "")];
        i += 1;
        while (i < lines.length && /^\s{2,}\S/.test(lines[i])) {
          item.push(lines[i].trim());
          i += 1;
        }
        items.push(item.join(" "));
      }
      tokens.push({ type: "list", ordered: true, items });
      continue;
    }

    if (line.trim() === "") {
      i += 1;
      continue;
    }

    const paragraph = [];
    while (
      i < lines.length &&
      lines[i].trim() !== "" &&
      !/^(#{1,4} |```|>|- |\d+\. |\|)/.test(lines[i]) &&
      !/^(---+|\*\*\*+)$/.test(lines[i].trim())
    ) {
      paragraph.push(lines[i]);
      i += 1;
    }
    tokens.push({ type: "paragraph", text: paragraph.join(" ") });
  }
  return tokens;
}

// ── Token → HTML ──────────────────────────────────────────────────────────────

// `token.html` is pre-highlighted markup set by the build (scripts/highlight.mjs);
// without it the block falls back to plain escaped text.
const codeBlock = (token) =>
  token.html
    ? `<pre class="code">${token.html}</pre>`
    : `<pre class="code"><code${token.lang ? ` class="lang-${escapeAttr(token.lang)}"` : ""}>${escapeHtml(token.text)}</code></pre>`;

function simpleToken(token) {
  switch (token.type) {
    case "code":
      return codeBlock(token);
    case "quote":
      return `<blockquote>${inline(token.text)}</blockquote>`;
    case "hr":
      return `<hr />`;
    case "list":
      return token.ordered
        ? `<ol>${token.items.map((item) => `<li>${inline(item)}</li>`).join("")}</ol>`
        : `<ul>${token.items.map((item) => `<li>${inline(item)}</li>`).join("")}</ul>`;
    case "table":
      return [
        `<div class="table-scroll"><table class="md-table"><thead><tr>`,
        token.head.map((cell) => `<th>${inline(cell)}</th>`).join(""),
        `</tr></thead><tbody>`,
        token.body
          .map((row) => `<tr>${row.map((cell) => `<td>${inline(cell)}</td>`).join("")}</tr>`)
          .join(""),
        `</tbody></table></div>`,
      ].join("");
    case "paragraph":
      return `<p>${inline(token.text)}</p>`;
    default:
      return "";
  }
}

// Heading ids are scoped to the enclosing part so pages with many compound
// parts (Combobox has 25) don't emit 25 elements with id="builders".
export function assignIds(tokens) {
  let part = "";
  for (const token of tokens) {
    if (token.type !== "heading") continue;
    if (token.level <= 2) {
      token.id = anchorId(token.text);
      part = token.level === 2 ? token.id : "";
    } else {
      const own = anchorId(token.text);
      token.id = part ? `${part}-${own}` : own;
    }
  }
  return tokens;
}

const headingHtml = (token) => {
  const id = token.id || anchorId(token.text);
  const mono = /^`/.test(token.text) ? " heading--mono" : "";
  return `<h${token.level} id="${id}" class="heading heading--${token.level}${mono}"><a class="heading__anchor" href="#${id}" aria-label="Link to this section">#</a>${inline(token.text)}</h${token.level}>`;
};

// A run of `#### \`.builder(...)\`` groups inside a `### Builders` section is
// collapsed into one dense table: builder name, signature, accepted values and
// description in a single row each.
function buildersTable(rows) {
  const cells = rows
    .map((row) => {
      const accepts = row.accepts.length
        ? `<ul class="api__accepts">${row.accepts
            .map((item) => `<li>${inline(item)}</li>`)
            .join("")}</ul>`
        : "";
      const description = row.description.length
        ? `<div class="api__description">${row.description.map((text) => `<p>${inline(text)}</p>`).join("")}</div>`
        : "";
      const signature = row.signature
        ? `<code class="api__signature">${escapeHtml(row.signature)}</code>`
        : "";
      return `<tr id="${row.id}"><th scope="row"><code>${escapeHtml(row.name)}</code></th><td>${signature}${accepts}${description}</td></tr>`;
    })
    .join("");
  return `<div class="table-scroll"><table class="api"><thead><tr><th scope="col">Builder</th><th scope="col">Signature &amp; description</th></tr></thead><tbody>${cells}</tbody></table></div>`;
}

function builderRow(token, body) {
  const name = token.text.replace(/`/g, "").replace(/\(\.\.\.\)$/, "");
  const row = { id: token.id || anchorId(token.text), name, signature: "", accepts: [], description: [] };
  let seenAccepts = false;
  for (const item of body) {
    if (item.type === "code" && !row.signature) {
      row.signature = item.text.replace(/\s+/g, " ").trim();
      continue;
    }
    if (item.type === "paragraph" && /^\*\*Accepts\*\*$/.test(item.text.trim())) {
      seenAccepts = true;
      continue;
    }
    if (item.type === "list" && seenAccepts && row.accepts.length === 0) {
      row.accepts = item.items;
      continue;
    }
    if (item.type === "paragraph") row.description.push(item.text);
    else row.description.push("");
  }
  row.description = row.description.filter(Boolean);
  return row;
}

export function renderTokens(tokens, { skipFirstHeading = false, skipFirstParagraph = false } = {}) {
  assignIds(tokens);
  const out = [];
  let i = 0;
  let inBuilders = false;
  let droppedHeading = !skipFirstHeading;
  let droppedParagraph = !skipFirstParagraph;

  while (i < tokens.length) {
    const token = tokens[i];

    if (token.type === "heading" && token.level === 1 && !droppedHeading) {
      droppedHeading = true;
      i += 1;
      continue;
    }
    if (token.type === "paragraph" && droppedHeading && !droppedParagraph) {
      droppedParagraph = true;
      i += 1;
      continue;
    }

    if (token.type === "heading") {
      if (token.level <= 3) inBuilders = token.level === 3 && /builders/i.test(token.text);
      if (token.level === 3 && /builders/i.test(token.text)) {
        out.push(headingHtml(token));
        i += 1;
        const rows = [];
        while (i < tokens.length && tokens[i].type === "heading" && tokens[i].level === 4) {
          const head = tokens[i];
          i += 1;
          const body = [];
          while (i < tokens.length && !(tokens[i].type === "heading" && tokens[i].level <= 4)) {
            body.push(tokens[i]);
            i += 1;
          }
          rows.push(builderRow(head, body));
        }
        if (rows.length) out.push(buildersTable(rows));
        continue;
      }
      out.push(headingHtml(token));
      i += 1;
      continue;
    }

    out.push(simpleToken(token));
    i += 1;
  }
  void inBuilders;
  return out.join("\n");
}

export const markdownToHtml = (markdown, options) => renderTokens(tokenize(markdown), options);

// ── Page metadata ─────────────────────────────────────────────────────────────

export function headings(markdown) {
  const tokens = assignIds(tokenize(markdown));
  let parent = "";
  const out = [];
  for (const token of tokens) {
    if (token.type !== "heading") continue;
    if (token.level === 2) parent = token.text.replace(/`/g, "");
    if (token.level < 2 || token.level > 3) continue;
    out.push({
      level: token.level,
      text: token.text.replace(/`/g, ""),
      parent: token.level === 3 ? parent : "",
      mono: /^`/.test(token.text),
      id: token.id,
    });
  }
  return out;
}

export function lede(markdown) {
  const tokens = tokenize(markdown);
  const first = tokens.find((token) => token.type === "paragraph");
  return first ? first.text : "";
}

export function plainText(markdown) {
  return lede(markdown)
    .replace(/`([^`]+)`/g, "$1")
    .replace(/\[([^\]]+)\]\([^)]+\)/g, "$1")
    .replace(/\*\*/g, "");
}

// ── Styles & client script ────────────────────────────────────────────────────
// Both live as standalone assets — site/site.css and site/site.js — copied to
// the dist root by build-site.mjs. Pages reference them with <link>/<script>
// tags; the deployment base path travels on <html data-base>.


// ── Chrome fragments ──────────────────────────────────────────────────────────

const iconSearch = `<svg width="12" height="12" viewBox="0 0 20 20" aria-hidden="true"><path fill-rule="evenodd" d="M9 3.5a5.5 5.5 0 1 0 0 11 5.5 5.5 0 0 0 0-11ZM2 9a7 7 0 1 1 12.452 4.391l3.328 3.329a.75.75 0 1 1-1.06 1.06l-3.329-3.328A7 7 0 0 1 2 9Z" clip-rule="evenodd" /></svg>`;
const iconChevron = `<svg width="10" height="10" viewBox="0 0 20 20" aria-hidden="true"><path fill-rule="evenodd" d="M8.22 5.22a.75.75 0 0 1 1.06 0l4.25 4.25a.75.75 0 0 1 0 1.06l-4.25 4.25a.75.75 0 0 1-1.06-1.06L11.94 10 8.22 6.28a.75.75 0 0 1 0-1.06Z" clip-rule="evenodd" /></svg>`;
const iconExternal = `<svg width="11" height="11" viewBox="0 0 20 20" aria-hidden="true"><path d="M12.5 3.5a.75.75 0 0 0 0 1.5h1.94l-5.22 5.22a.75.75 0 1 0 1.06 1.06L15.5 6.06V8a.75.75 0 0 0 1.5 0V4.25a.75.75 0 0 0-.75-.75H12.5Z" /><path d="M5.75 5A2.75 2.75 0 0 0 3 7.75v6.5A2.75 2.75 0 0 0 5.75 17h6.5A2.75 2.75 0 0 0 15 14.25V11.5a.75.75 0 0 0-1.5 0v2.75c0 .69-.56 1.25-1.25 1.25h-6.5c-.69 0-1.25-.56-1.25-1.25v-6.5c0-.69.56-1.25 1.25-1.25H8.5A.75.75 0 0 0 8.5 5H5.75Z" /></svg>`;

export function sidebar({ base, version, overview, components, current, logo }) {
  const link = (item) =>
    `<a class="nav-link" href="${base}${item.url}"${item.url === current ? ' aria-current="page"' : ""}>${escapeHtml(item.title)}</a>`;
  return `<aside class="sidebar">
  <a class="brand" href="${base}/">${logo ? `<img class="brand__mark" src="${base}/${logo}" alt="" width="16" height="16" />` : `<span class="brand__mark"></span>`}<span class="brand__name">base-gpui</span><span class="brand__version">${escapeHtml(version)}</span></a>
  <button class="search" type="button" data-search-open aria-label="Search documentation">${iconSearch}<span>Search...</span><kbd>⌘K</kbd></button>
  <div class="nav-group">
    <div class="nav-group__title">Overview</div>
    ${overview.map(link).join("\n    ")}
  </div>
  <div class="nav-group">
    <div class="nav-group__title">Components <span>${components.length}</span></div>
    ${components.map(link).join("\n    ")}
  </div>
</aside>`;
}

export function tocHtml(items) {
  if (!items.length) return `<nav class="toc" aria-label="On this page"></nav>`;
  return `<nav class="toc" aria-label="On this page">
  <div class="toc__title">On this page</div>
  <div class="toc__list">
    ${items
      .map(
        (item) =>
          `<a class="toc__link toc__link--${item.level}${item.mono ? " toc__link--mono" : ""}" href="#${item.id}">${escapeHtml(item.text)}</a>`,
      )
      .join("\n    ")}
  </div>
</nav>`;
}

export function breadcrumbHtml(trail) {
  if (!trail.length) return "";
  return `<div class="breadcrumb">${trail
    .map((item, index) =>
      [index ? iconChevron : "", item.url ? `<a href="${item.url}">${escapeHtml(item.title)}</a>` : `<span>${escapeHtml(item.title)}</span>`].join(""),
    )
    .join("")}</div>`;
}

export function chip(label, href, { external = false } = {}) {
  const attrs = external ? ' target="_blank" rel="noreferrer"' : "";
  return `<a class="chip" href="${escapeAttr(href)}"${attrs}>${escapeHtml(label)}${external ? iconExternal : ""}</a>`;
}

export function demoHtml({ base, slug, title }) {
  return `<div class="demo" data-demo="${escapeAttr(slug)}" data-state="loading">
  <div class="demo__bar"><span class="demo__label">EXAMPLE</span><span class="demo__meta">wasm · webgpu</span></div>
  <div class="demo__stage">
    <iframe class="demo__frame" loading="lazy" src="${base}/demo/index.html?demo=${escapeAttr(slug)}" title="${escapeAttr(title)} example"></iframe>
    <div class="demo__state demo__skeleton" aria-hidden="true">
      <div class="demo__bones">
        <div class="demo__bone demo__bone--track"></div>
        <div class="demo__bone demo__bone--label"></div>
        <div class="demo__note">Compiling ~18 MB of WebAssembly</div>
      </div>
    </div>
    <div class="demo__state demo__fallback">
      <p>This demo runs the real GPUI component on WebGPU, which this browser does not expose.</p>
      <p>Try Chrome or Edge 113+, or Safari 26+. In Firefox, enable <code>dom.webgpu.enabled</code>.</p>
      <p>You can also run it natively: <code>cargo run -p showcase ${escapeHtml(slug)}</code></p>
    </div>
    <div class="demo__state demo__failed">
      <p>The demo failed to start in this browser.</p>
      <p><code data-demo-error></code></p>
      <p>You can run it natively instead: <code>cargo run -p showcase ${escapeHtml(slug)}</code></p>
    </div>
    <div class="demo__state demo__mobile">
      <p>Live examples are not supported on most mobile browsers yet — open this page on a desktop or tablet to run the demo.</p>
      <p>Recent phones with WebGPU may manage it (downloads ~18 MB):</p>
      <button class="demo__try" type="button" data-demo-try>Try anyway</button>
    </div>
  </div>
</div>`;
}

export function pagerHtml({ base, prev, next }) {
  if (!prev && !next) return "";
  const link = (item, kind) =>
    item
      ? `<a class="pager__link pager__link--${kind}" href="${base}${item.url}"><span class="pager__kicker">${kind === "prev" ? "Previous" : "Next"}</span><span class="pager__title">${escapeHtml(item.title)}</span></a>`
      : `<span class="pager__link" aria-hidden="true"></span>`;
  return `<div class="pager">${link(prev, "prev")}${link(next, "next")}</div>`;
}

const paletteHtml = `<div class="palette" id="palette" role="dialog" aria-modal="true" aria-label="Search documentation">
  <div class="palette__scrim" data-search-close></div>
  <div class="palette__panel">
    <div class="palette__field">${iconSearch}<input class="palette__input" id="palette-input" type="search" placeholder="Search components and sections" autocomplete="off" spellcheck="false" /></div>
    <ul class="palette__results" id="palette-results"></ul>
    <div class="palette__footer"><span><kbd>↑</kbd><kbd>↓</kbd> navigate</span><span><kbd>enter</kbd> open</span><span><kbd>esc</kbd> close</span></div>
  </div>
</div>`;


export function page({ base, title, description, sidebar: sidebarHtml, toc, content, searchIndex }) {
  return `<!doctype html>
<html lang="en" data-base="${escapeAttr(base)}">
<head>
<meta charset="utf-8" />
<meta name="viewport" content="width=device-width, initial-scale=1" />
<title>${escapeHtml(title)}</title>
<meta name="description" content="${escapeAttr(description || "")}" />
<script src="${base}/coi-serviceworker.js"></script>
<link rel="stylesheet" href="${base}/site.css" />
</head>
<body>
<a class="skip" href="#content">Skip to content</a>
<div class="layout">
${sidebarHtml}
<main class="content" id="content">
${content}
</main>
${toc}
</div>
${paletteHtml}
<script>window.__SEARCH_INDEX__ = ${JSON.stringify(searchIndex)};</script>
<script src="${base}/site.js"></script>
</body>
</html>
`;
}

export function redirectPage({ base, to, title }) {
  return `<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8" />
<meta http-equiv="refresh" content="0; url=${base}${to}" />
<link rel="canonical" href="${base}${to}" />
<title>${escapeHtml(title)}</title>
<script src="${base}/coi-serviceworker.js"></script>
<script>window.location.replace(${JSON.stringify(base + to)});</script>
</head>
<body style="background:#0E0E0E"></body>
</html>
`;
}

// ── Authored pages ────────────────────────────────────────────────────────────
// Sourced from README.md and docs/component-architecture.md.

export const quickStartMarkdown = (base = "") => `# Quick start

Base GPUI ports [Base UI](${baseUiUrl}/)'s rich component API surface to GPUI: compound parts, controlled and uncontrolled state, keyboard interaction, state-aware styling, and accessibility semantics. Components are unstyled, so applications retain complete control over their visual design.

> Base GPUI is under active development. APIs may change before version 1.0. Install it from GitHub, as is common for projects using GPUI's actively developed Git revision.

## Installation

Add the crate alongside the GPUI revision it is built against:

\`\`\`toml
[dependencies]
base-gpui = { git = "${repoUrl}" }
gpui = { git = "https://github.com/zed-industries/zed", rev = "1764c2fa6776c545ece60357e0e6dd9856a241bc" }
\`\`\`

## Initialize

Initialize Base GPUI when your application starts. This registers the key bindings components dispatch against.

\`\`\`rust
use gpui::{App, Application};

fn main() {
    Application::new().run(|cx: &mut App| {
        base_gpui::init(cx);
        // Open your application window.
    });
}
\`\`\`

## Compose a component

Components expose compound parts and state-aware styling. Tabs are composed from \`TabsRoot\`, \`TabsList\`, \`TabsTab\`, \`TabsIndicator\`, and \`TabsPanel\` rather than a single pre-styled widget.

\`\`\`rust
use base_gpui::tabs::{TabsList, TabsPanel, TabsRoot, TabsTab};
use gpui::prelude::*;
use gpui::px;

TabsRoot::<&'static str>::new()
    .id("settings")
    .default_value(Some("overview"))
    .child(
        TabsList::new()
            .aria_label("Project sections")
            .child(
                TabsTab::new()
                    .id("overview")
                    .value("overview")
                    .px(px(12.))
                    .py(px(6.))
                    .style_with_state(|state, tab| {
                        if state.active { tab.text_color(gpui::white()) } else { tab }
                    })
                    .child("Overview"),
            ),
    );
\`\`\`

## Styling

Nothing ships with visual design. Every part that draws exposes \`.style_with_state(...)\`, which receives that part's behavioral state and the GPUI \`Div\` being built:

\`\`\`rust
SwitchRoot::new()
    .style_with_state(|state, root| {
        root.bg(if state.checked { accent() } else { control() })
    });
\`\`\`

State-state structs are component-specific public API — the GPUI equivalent of Base UI's state-aware \`className\` and \`render\` callbacks. Data attributes and CSS variable APIs are deliberately not ported.

## Examples

Every component guide embeds an example: the actual GPUI component compiled to WebAssembly and rendered through GPUI's WebGPU backend. The build is threaded, so it needs cross-origin isolation — locally the trunk dev server sends the COOP and COEP headers, and on GitHub Pages a service worker injects them on first visit.

An example needs a browser that exposes WebGPU. Where it is missing, the example region explains how to run the same demo natively instead.

## Component guides

Every component has a guide covering its anatomy, each compound part, every builder with the exact values it accepts, and its accessibility model. Start with [Switch](${base}/components/switch.html) for a small component, or [Tabs](${base}/components/tabs.html) for a compound one with keyboard behavior.

## Project relationships

Base GPUI is an independent community project. It is not affiliated with, endorsed by, or maintained by the Base UI or Zed teams.

- [Base UI](${baseUiUrl}/) inspired the component API design.
- [GPUI](https://gpui.rs/) is the underlying application framework.
`;
