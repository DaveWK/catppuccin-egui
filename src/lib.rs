// SPDX-License-Identifier: MIT
//! Soothing pastel theme for [egui](egui).
//!
//! To use, call [`set_theme`](crate::set_theme) with the egui context
//! and a [`Theme`](crate::Theme).
//!
//! # Example
//!
//! ```rust
//! use catppuccin_egui::{egui, MACCHIATO};
//!
//! let ctx = egui::Context::default();
//! catppuccin_egui::set_theme(&ctx, MACCHIATO);
//! ```
//!
//! You can also customize your own theme:
//!
//! ```rust
//! # use catppuccin_egui::egui;
//! use catppuccin_egui::{Theme, MOCHA};
//! const MY_MOCHA: Theme = Theme {
//!     red: egui::Color32::from_rgb(255, 0, 0),
//!     ..MOCHA
//! };
//! ```
//!

#[cfg(not(any(
    feature = "egui26",
    feature = "egui27",
    feature = "egui28",
    feature = "egui29",
    feature = "egui30",
    feature = "egui31",
    feature = "egui32",
    feature = "egui33",
    feature = "egui36",
    feature = "egui37",
)))]
compile_error!("at least one egui version must be enabled");

#[cfg(feature = "egui26")]
pub use egui26 as egui;
#[cfg(feature = "egui27")]
pub use egui27 as egui;
#[cfg(feature = "egui28")]
pub use egui28 as egui;
#[cfg(feature = "egui29")]
pub use egui29 as egui;
#[cfg(feature = "egui30")]
pub use egui30 as egui;
#[cfg(feature = "egui31")]
pub use egui31 as egui;
#[cfg(feature = "egui32")]
pub use egui32 as egui;
#[cfg(feature = "egui33")]
pub use egui33 as egui;
#[cfg(feature = "egui36")]
pub use egui36 as egui;
#[cfg(feature = "egui37")]
pub use egui37 as egui;

use egui::{epaint, style};

mod themes;
pub use themes::*;

/// Apply the given theme to a [`Context`](egui::Context).
pub fn set_theme(ctx: &egui::Context, theme: Theme) {
    #[cfg(any(feature = "egui36", feature = "egui37"))]
    {
        for mode in [egui::Theme::Light, egui::Theme::Dark] {
            let mut style = ctx.style_of(mode).as_ref().clone();
            set_style_theme(&mut style, theme);
            ctx.set_style_of(mode, style);
        }
    }
    #[cfg(not(any(feature = "egui36", feature = "egui37")))]
    {
        let old = ctx.style().visuals.clone();
        ctx.set_visuals(theme.visuals(old));
    }
}

/// Apply the given theme to a [`Style`](egui::Style).
///
/// # Example
///
/// ```rust
/// use catppuccin_egui::{egui, MACCHIATO};
/// let mut style = egui::Style::default();
/// catppuccin_egui::set_style_theme(&mut style, MACCHIATO);
/// ```
pub fn set_style_theme(style: &mut egui::Style, theme: Theme) {
    let old = style.visuals.clone();
    style.visuals = theme.visuals(old);
}

fn make_widget_visual(
    old: style::WidgetVisuals,
    theme: &Theme,
    bg_fill: egui::Color32,
) -> style::WidgetVisuals {
    style::WidgetVisuals {
        bg_fill,
        weak_bg_fill: bg_fill,
        bg_stroke: egui::Stroke {
            color: theme.overlay1,
            ..old.bg_stroke
        },
        fg_stroke: egui::Stroke {
            color: theme.text,
            ..old.fg_stroke
        },
        ..old
    }
}

impl Theme {
    fn visuals(&self, old: egui::Visuals) -> egui::Visuals {
        let is_latte = *self == LATTE;
        let shadow_color = if is_latte {
            egui::Color32::from_black_alpha(25)
        } else {
            egui::Color32::from_black_alpha(96)
        };

        egui::Visuals {
            hyperlink_color: self.rosewater,
            faint_bg_color: self.surface0,
            extreme_bg_color: self.crust,
            code_bg_color: self.mantle,
            warn_fg_color: self.peach,
            error_fg_color: self.maroon,
            window_fill: self.base,
            panel_fill: self.base,
            window_stroke: egui::Stroke {
                color: self.overlay1,
                ..old.window_stroke
            },
            widgets: style::Widgets {
                noninteractive: make_widget_visual(old.widgets.noninteractive, self, self.base),
                inactive: make_widget_visual(old.widgets.inactive, self, self.surface0),
                hovered: make_widget_visual(old.widgets.hovered, self, self.surface2),
                active: make_widget_visual(old.widgets.active, self, self.surface1),
                open: make_widget_visual(old.widgets.open, self, self.surface0),
            },
            selection: style::Selection {
                bg_fill: self.blue.linear_multiply(if is_latte { 0.4 } else { 0.2 }),
                stroke: egui::Stroke {
                    color: self.text,
                    ..old.selection.stroke
                },
            },
            window_shadow: epaint::Shadow {
                color: shadow_color,
                ..old.window_shadow
            },
            popup_shadow: epaint::Shadow {
                color: shadow_color,
                ..old.popup_shadow
            },
            dark_mode: !is_latte,
            ..old
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_palettes_apply_their_actual_colours() {
        for palette in [LATTE, FRAPPE, MACCHIATO, MOCHA] {
            let mut style = egui::Style::default();
            style.spacing.item_spacing = egui::vec2(13.0, 17.0);
            set_style_theme(&mut style, palette);
            assert_eq!(style.visuals.panel_fill, palette.base);
            assert_eq!(style.visuals.widgets.hovered.bg_fill, palette.surface2);
            assert_eq!(style.visuals.error_fg_color, palette.maroon);
            assert_eq!(style.visuals.dark_mode, palette != LATTE);
            assert_eq!(style.spacing.item_spacing, egui::vec2(13.0, 17.0));
        }
    }

    #[test]
    fn context_theme_preserves_nonvisual_settings() {
        let ctx = egui::Context::default();
        #[cfg(any(feature = "egui36", feature = "egui37"))]
        for mode in [egui::Theme::Light, egui::Theme::Dark] {
            let mut style = ctx.style_of(mode).as_ref().clone();
            style.spacing.item_spacing = egui::vec2(19.0, 23.0);
            ctx.set_style_of(mode, style);
        }
        #[cfg(not(any(feature = "egui36", feature = "egui37")))]
        ctx.style_mut(|style| style.spacing.item_spacing = egui::vec2(19.0, 23.0));
        set_theme(&ctx, LATTE);
        #[cfg(any(feature = "egui36", feature = "egui37"))]
        for mode in [egui::Theme::Light, egui::Theme::Dark] {
            assert_eq!(ctx.style_of(mode).visuals.panel_fill, LATTE.base);
            assert_eq!(
                ctx.style_of(mode).spacing.item_spacing,
                egui::vec2(19.0, 23.0)
            );
        }
        #[cfg(not(any(feature = "egui36", feature = "egui37")))]
        {
            assert_eq!(ctx.style().visuals.panel_fill, LATTE.base);
            assert_eq!(ctx.style().spacing.item_spacing, egui::vec2(19.0, 23.0));
        }
    }
}
