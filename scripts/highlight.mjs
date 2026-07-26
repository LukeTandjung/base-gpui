// Build-time syntax highlighting for the docs site. Runs Shiki during the
// static build, so pages ship pre-colored spans and no highlighter JS ever
// reaches the browser (which COEP:require-corp would complicate anyway).
//
// The vesper theme's near-black/peach palette sits close to the site's carbon
// theme; the emitted background is stripped so blocks keep the site's own
// `.code` surface.

import { createHighlighter } from "npm:shiki@3";

const LANGS = ["rust", "toml", "bash"];

let highlighter = null;

export async function initHighlighter() {
  highlighter = await createHighlighter({ themes: ["vesper"], langs: LANGS });
}

// Returns the inner `<code>…</code>` markup for a fenced block, or null when
// the language is unknown (caller falls back to plain escaped text).
export function highlightCode(text, lang) {
  if (!highlighter || !LANGS.includes(lang)) return null;
  const html = highlighter.codeToHtml(text, { lang, theme: "vesper" });
  const inner = html.match(/^<pre[^>]*>(<code[^>]*>[\s\S]*<\/code>)<\/pre>\s*$/);
  return inner ? inner[1] : null;
}
