# Fonts

`PPMori-Variable.woff2` is [PP Mori](https://pangrampangram.com/products/mori)
by Pangram Pangram — a single variable font (weight axis 100–900) covering the
regular, semibold and black weights the site uses.

PP Mori is free for personal and educational use; commercial use requires a
license from Pangram Pangram. Fonts must be self-hosted because the wasm demos
require `COEP: require-corp`, which blocks cross-origin subresources.

The build copies every file in this directory into `site/dist/fonts/`. If the
directory is missing, pages fall back to `system-ui`; the `@font-face` rules
live in `scripts/site-template.mjs`.
