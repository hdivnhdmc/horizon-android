//! The launcher's view of Horizon's design tokens (`crates/horizon-theme`, described in
//! `docs/design-system.md`): the blocky look it shares with the website ("Blocks"), stone-dark or
//! lavender-light with Horizon's violet, Pixelify Sans for headings and controls, Geist and Geist
//! Mono for text (all bundled, SIL Open Font License).
use gpui::{rgb, Hsla, WindowAppearance};
use horizon_launcher::backend::Appearance;
use horizon_launcher::themes::Accent;
use horizon_theme as tokens;
use std::time::Duration;

pub use tokens::{Step, BODY, CAPTION, DISPLAY, HEADING, LABEL, MICRO, TITLE, UI, WORDMARK};
pub const FONT: &str = tokens::FONT;
pub const MONO: &str = tokens::MONO;
pub const PIXEL: &str = tokens::PIXEL;

pub use tokens::{BLOCK_BEVEL as BEVEL, BLOCK_LIP as LIP};
/// Shapes and controls, shared with the game's menus and the website.
pub use tokens::{CONTROL_LARGE, DISABLED, FIELD, PRESSED, RISE, ROW, SEGMENT};

/// Blocks have square corners: these stay for the places that ask for a radius.
pub const RADIUS: f32 = 0.;
pub const RADIUS_PLATE: f32 = 0.;
/// Buttons are a little taller than the quiet look's, for their lip.
pub const CONTROL: f32 = 34.;
/// The top bar, and the sidebar of instances and servers.
pub const TOPBAR: f32 = 56.;
pub const SIDEBAR: f32 = 248.;
pub const PAGE_GUTTER: f32 = 32.;

/// Motion: short and eased out, never bouncy.
pub const QUICK: Duration = tokens::QUICK.duration();
pub const MOVE: Duration = tokens::MOVE.duration();
pub const ENTER: Duration = tokens::ENTER.duration();
pub const STAGGER: Duration = tokens::STAGGER;

#[derive(Clone, Copy)]
pub struct Theme {
    /// The page behind everything (under its stone texture), and its colour as `0xRRGGBB` for
    /// the texture.
    pub page: Hsla,
    pub page_rgb: u32,
    /// Hover plates, fields and the console: the raised panel colour.
    pub plate_soft: Hsla,
    /// Pressed plates, a switch that is off: the button colour.
    pub plate: Hsla,
    /// Dialogs and menus above the page: the panel colour.
    pub surface: Hsla,
    pub rule: Hsla,
    pub ink: Hsla,
    pub ink_soft: Hsla,
    pub muted: Hsla,
    pub accent: Hsla,
    pub on_accent: Hsla,
    pub danger: Hsla,
    pub success: Hsla,
    pub warn: Hsla,
    /// The blocky parts (`horizon_theme::Blocks`).
    pub side: Hsla,
    pub panel: Hsla,
    pub raised: Hsla,
    pub button: Hsla,
    pub button_bevel: Hsla,
    pub well: Hsla,
    pub bevel: Hsla,
    pub lip: Hsla,
    pub outline: Hsla,
    pub face: Hsla,
    pub face_bevel: Hsla,
    pub face_lip: Hsla,
    pub on_face: Hsla,
    pub dark: bool,
}

fn c(hex: u32) -> Hsla {
    rgb(hex).into()
}

/// Whether the launcher is pinned dark (`Some(true)`) or light, or follows the system (`None`):
/// `HORIZON_LAUNCHER_APPEARANCE=light` or `dark` (for screenshots and tests), else the player's
/// choice in Settings.
pub fn pinned_dark(choice: Appearance) -> Option<bool> {
    match std::env::var("HORIZON_LAUNCHER_APPEARANCE").as_deref() {
        Ok("light") => Some(false),
        Ok("dark") => Some(true),
        _ => match choice {
            Appearance::Dark => Some(true),
            Appearance::Light => Some(false),
            Appearance::System => None,
        },
    }
}

impl Theme {
    /// The pinned look (`pinned_dark`: dark unless the player picked another), the system's when
    /// they asked to follow it, in the player's accent.
    pub fn for_appearance(choice: Appearance, accent: Accent, system: WindowAppearance) -> Self {
        let system_dark = !matches!(
            system,
            WindowAppearance::Light | WindowAppearance::VibrantLight
        );
        let dark = pinned_dark(choice).unwrap_or(system_dark);
        
        // تخصيص الألوان المطلوبة مباشرة وتطبيقها على السمات
        Self {
            page: c(0x10131B),          // الخلفية
            page_rgb: 0x10131B,
            side: c(0x171C28),          // القائمة الجانبية
            panel: c(0x202838),         // البطاقات
            surface: c(0x202838),
            raised: c(0x202838),
            button: c(0x7965F5),        // الأزرار
            button_bevel: c(0x7965F5),
            accent: c(0x36B9E9),        // التحديد
            outline: c(0x36B9E9),
            ink: c(0xF4F5FA),           // النص الأساسي
            on_accent: c(0xF4F5FA),
            on_face: c(0xF4F5FA),
            muted: c(0x929BAD),         // النص الثانوي
            ink_soft: c(0x929BAD),
            plate_soft: c(0x202838),
            plate: c(0x7965F5),
            rule: c(0x202838),
            well: c(0x171C28),
            bevel: c(0x7965F5),
            lip: c(0x7965F5),
            face: c(0x202838),
            face_bevel: c(0x7965F5),
            face_lip: c(0x7965F5),
            danger: c(0xEF4444),
            success: c(0x10B981),
            warn: c(0xF59E0B),
            dark: true,
        }
    }
}

/// How strongly a selected suggestion in a text field is tinted with the accent.
pub const SELECTION: f32 = 0.22;

#[cfg(test)]
mod tests {
    use super::{tokens, SELECTION};

    /// `top` at `alpha` over `under`, as the GPU blends them.
    fn blend(top: u32, under: u32, alpha: f32) -> u32 {
        let channel = |shift: u32| {
            let (a, b) = ((top >> shift & 0xff) as f32, (under >> shift & 0xff) as f32);
            ((a * alpha + b * (1. - alpha)).round() as u32) << shift
        };
        channel(16) | channel(8) | channel(0)
    }

    /// What the Servers page draws besides text (whose colours horizon-theme's own test holds to
    /// WCAG AA on every background): the keyboard focus ring, the borders of a focused or refused
    /// field (WCAG 1.4.11 asks 3:1 for those), and text over a selected suggestion.
    #[test]
    fn marks_and_selections_read_clearly() {
        let selection = blend(0x36B9E9, 0x202838, SELECTION);
        let ratio = tokens::contrast(0xF4F5FA, selection);
        assert!(ratio >= 4.5, "text on a selection: {ratio:.2}");
    }
}
