//! Shared palette for demo styling, matched to the docs site's carbon theme
//! (see scripts/site-template.mjs). Components themselves are headless; all
//! color lives here.
//!
//! Surfaces are white-on-carbon translucency rather than solid greys, so a
//! demo composited over the page background reads as the same material as the
//! HTML panels around it.

use gpui::{rgb, rgba, Rgba};

/// App background — the only canvas color. Matches the docs page background.
pub fn background() -> Rgba {
    rgb(0x0E0E0E)
}

/// Panel fill: white at 4.6%.
pub fn surface() -> Rgba {
    rgba(0xFFFFFF0C)
}

/// `surface()` pre-composited over `background()` — the same color, but
/// opaque. Floating panels (dialogs, menus, popovers, toasts) must use this
/// so the content underneath doesn't show through.
pub fn surface_solid() -> Rgba {
    rgb(0x191919)
}

/// Hairline border: white at 5%.
pub fn border() -> Rgba {
    rgba(0xFFFFFF0D)
}

/// Emphasised border: white at 15%.
pub fn border_strong() -> Rgba {
    rgba(0xFFFFFF26)
}

pub fn text() -> Rgba {
    rgb(0xFFFFFF)
}

pub fn text_muted() -> Rgba {
    rgba(0xFFFFFF80)
}

/// Text drawn on top of `accent()` fills.
pub fn text_inverted() -> Rgba {
    rgb(0x0E0E0E)
}

/// Solid "on" fill for checked tracks, pressed buttons and indicators. The
/// carbon system has no chromatic accent, so filled controls are white.
pub fn accent() -> Rgba {
    rgb(0xFFFFFF)
}

pub fn accent_hover() -> Rgba {
    rgb(0xE5E5E5)
}

/// Unfilled track / control chrome: white at 10%.
pub fn control() -> Rgba {
    rgba(0xFFFFFF1A)
}

/// Focus ring — a thin near-white ring, no glow.
pub fn focus_ring() -> Rgba {
    rgba(0xFFFFFFD9)
}
