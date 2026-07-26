// Builds the static showcase site from docs/ into site/dist/.
//
// Pages: /overview/quick-start.html, /overview/component-architecture.html and
// one page per docs/components/*.md. Components with a registered wasm demo
// embed it in an iframe above the guide. All rendering lives in
// scripts/site-template.mjs.
//
// Usage: deno run --allow-read --allow-write scripts/build-site.mjs [--base /base-gpui]

import { readdir, readFile, mkdir, writeFile, rm, copyFile, stat } from "node:fs/promises";
import path from "node:path";

import {
  anchorId,
  breadcrumbHtml,
  chip,
  demoHtml,
  headings,
  inline,
  page,
  pagerHtml,
  plainText,
  quickStartMarkdown,
  redirectPage,
  renderTokens,
  repoUrl,
  sidebar,
  titleCase,
  tocHtml,
  tokenize,
} from "./site-template.mjs";
import { highlightCode, initHighlighter } from "./highlight.mjs";

const root = path.resolve(import.meta.dirname, "..");
const docsRoot = path.join(root, "docs", "components");
const siteRoot = path.join(root, "site", "dist");

const baseFlag = process.argv.indexOf("--base");
const base = baseFlag >= 0 ? process.argv[baseFlag + 1].replace(/\/$/, "") : "";

const exists = (target) =>
  stat(target).then(
    () => true,
    () => false,
  );

async function crateVersion() {
  try {
    const manifest = await readFile(path.join(root, "Cargo.toml"), "utf8");
    const match = manifest.match(/^version\s*=\s*"([^"]+)"/m);
    return match ? `v${match[1]}` : "";
  } catch {
    return "";
  }
}

async function demoSlugs() {
  const registry = await readFile(path.join(root, "site", "src", "demos", "mod.rs"), "utf8");
  return new Set([...registry.matchAll(/slug: "([a-z-]+)"/g)].map((match) => match[1]));
}

// Splits a component guide into the pieces the page shell needs: title, lede,
// the reference-link row, and the tokens that make up the body.
function splitGuide(markdown) {
  const tokens = tokenize(markdown);
  let title = "";
  let lede = "";
  const links = [];
  let index = 0;

  if (tokens[index]?.type === "heading" && tokens[index].level === 1) {
    title = tokens[index].text;
    index += 1;
  }
  if (tokens[index]?.type === "paragraph") {
    lede = tokens[index].text;
    index += 1;
  }
  if (tokens[index]?.type === "paragraph" && /^\[[^\]]+\]\(/.test(tokens[index].text.trim())) {
    for (const match of tokens[index].text.matchAll(/\[([^\]]+)\]\(([^)]+)\)/g)) {
      const href = match[2].startsWith("../../")
        ? `${repoUrl}/blob/main/${match[2].replace("../../", "")}`
        : match[2];
      links.push({ label: match[1], href });
    }
    index += 1;
  }
  return { title, lede, links, body: tokens.slice(index) };
}

const components = (await readdir(docsRoot))
  .filter((file) => file.endsWith(".md") && file !== "README.md")
  .map((file) => file.replace(/\.md$/, ""))
  .sort();

const demos = await demoSlugs();
const version = await crateVersion();
const hasLogo = await exists(path.join(root, "assets", "logo.png"));

const overviewNav = [
  { title: "Quick start", url: "/overview/quick-start.html" },
  { title: "Component architecture", url: "/overview/component-architecture.html" },
];
const componentNav = components.map((name) => ({
  title: titleCase(name),
  url: `/components/${name.replaceAll("_", "-")}.html`,
  demo: demos.has(name.replaceAll("_", "-")),
  name,
}));
const order = [...overviewNav, ...componentNav];

const searchIndex = [];
const written = [];

function shell({ url, title, description, headHtml, toc }) {
  return page({
    base,
    title,
    description,
    sidebar: sidebar({
      base,
      version,
      overview: overviewNav,
      components: componentNav,
      current: url,
      logo: hasLogo ? "logo.png" : null,
    }),
    toc: tocHtml(toc),
    content: headHtml,
    searchIndex,
  });
}

function pagerFor(url) {
  const at = order.findIndex((item) => item.url === url);
  return { prev: at > 0 ? order[at - 1] : null, next: at >= 0 && at < order.length - 1 ? order[at + 1] : null };
}

// ── Collect content ───────────────────────────────────────────────────────────

const pages = [];

{
  const markdown = quickStartMarkdown(base);
  const { title, lede, body } = splitGuide(markdown);
  pages.push({
    url: "/overview/quick-start.html",
    group: "Overview",
    title,
    lede,
    links: [
      { label: "GitHub repository", href: repoUrl, external: true },
      { label: "Base UI", href: "https://base-ui.com", external: true },
    ],
    body,
    toc: headings(markdown),
  });
}

{
  const markdown = await readFile(path.join(root, "docs", "component-architecture.md"), "utf8");
  const { lede, body } = splitGuide(markdown);
  pages.push({
    url: "/overview/component-architecture.html",
    group: "Overview",
    title: "Component architecture",
    lede,
    links: [
      {
        label: "docs/component-architecture.md",
        href: `${repoUrl}/blob/main/docs/component-architecture.md`,
        external: true,
      },
    ],
    body,
    toc: headings(markdown),
  });
}

for (const item of componentNav) {
  const markdown = await readFile(path.join(docsRoot, `${item.name}.md`), "utf8");
  const { lede, links, body } = splitGuide(markdown);
  const slug = item.url.replace("/components/", "").replace(".html", "");
  pages.push({
    url: item.url,
    group: "Components",
    title: item.title,
    breadcrumb: [{ title: "Components" }, { title: item.title }],
    lede,
    links: links.map((link) => ({ ...link, external: /^https?:/.test(link.href) })),
    demo: demos.has(slug) ? { slug, title: item.title } : null,
    body,
    toc: headings(markdown),
  });
}

// Pre-highlight every fenced code block. Builder-signature code tokens are
// unaffected — that path reads `token.text`, not the rendered block.
await initHighlighter();
for (const entry of pages) {
  for (const token of entry.body) {
    if (token.type === "code") {
      token.html = highlightCode(token.text, token.lang);
    }
  }
}

for (const entry of pages) {
  searchIndex.push({
    t: entry.title,
    u: entry.url,
    g: entry.group,
    s: entry.toc.map((heading) => ({
      n: heading.parent ? `${heading.parent} · ${heading.text}` : heading.text,
      a: heading.id,
    })),
  });
}

// ── Emit ──────────────────────────────────────────────────────────────────────

// Clean previous output but keep demo/ — that's trunk's wasm bundle, built
// separately and not something a docs rebuild should discard.
await mkdir(siteRoot, { recursive: true });
for (const entry of await readdir(siteRoot)) {
  if (entry !== "demo") await rm(path.join(siteRoot, entry), { recursive: true, force: true });
}
await mkdir(path.join(siteRoot, "components"), { recursive: true });
await mkdir(path.join(siteRoot, "overview"), { recursive: true });

for (const entry of pages) {
  const { prev, next } = pagerFor(entry.url);
  const content = [
    breadcrumbHtml(entry.breadcrumb || []),
    `<h1 class="page-title" id="${anchorId(entry.title)}">${entry.title}</h1>`,
    entry.lede ? `<p class="lede">${inline(entry.lede)}</p>` : "",
    entry.links?.length
      ? `<div class="meta-links">${entry.links
          .map((link) => chip(link.label, link.href, { external: link.external }))
          .join("")}</div>`
      : "",
    entry.demo ? demoHtml({ base, slug: entry.demo.slug, title: entry.demo.title }) : "",
    `<div class="prose">${renderTokens(entry.body)}</div>`,
    pagerHtml({ base, prev, next }),
  ].join("\n");

  const html = shell({
    url: entry.url,
    title: `${entry.title} · base-gpui`,
    description: plainText(entry.lede || ""),
    headHtml: content,
    toc: entry.toc,
  });
  const target = path.join(siteRoot, entry.url.replace(/^\//, ""));
  await writeFile(target, html);
  written.push(entry.url);
}

await writeFile(
  path.join(siteRoot, "index.html"),
  redirectPage({ base, to: "/overview/quick-start.html", title: "base-gpui" }),
);

// GitHub Pages cannot send the COOP/COEP headers the threaded wasm build needs,
// so every page registers this header-injecting service worker.
await copyFile(
  path.join(root, "site", "coi-serviceworker.js"),
  path.join(siteRoot, "coi-serviceworker.js"),
);

// The stylesheet and client script every page links to.
await copyFile(path.join(root, "site", "site.css"), path.join(siteRoot, "site.css"));
await copyFile(path.join(root, "site", "site.js"), path.join(siteRoot, "site.js"));

if (hasLogo) {
  await copyFile(path.join(root, "assets", "logo.png"), path.join(siteRoot, "logo.png"));
}

// Self-hosted fonts: COEP:require-corp blocks cross-origin subresources, so
// nothing can come from a CDN. Missing files degrade to system-ui.
const fontsSrc = path.join(root, "site", "fonts");
if (await exists(fontsSrc)) {
  await mkdir(path.join(siteRoot, "fonts"), { recursive: true });
  for (const file of await readdir(fontsSrc)) {
    await copyFile(path.join(fontsSrc, file), path.join(siteRoot, "fonts", file));
  }
} else {
  console.warn("site/fonts/ missing — pages will fall back to system-ui (see site/fonts/README.md).");
}

console.log(
  `Built ${written.length} pages into site/dist/ (${components.length} components, ${demos.size} with live demos).`,
);
