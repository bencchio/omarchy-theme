//! Print: forced onto a page — white behind, black in front, and nothing but greys between.

use crate::color::{Ramp, desaturate, tone_of};
use crate::theme::{Mode, Rgb, Theme, ThemeColors};

use super::accent::{restated, tinted};

const WHITE: Rgb = Rgb {
    red: 255,
    green: 255,
    blue: 255,
};
const BLACK: Rgb = Rgb {
    red: 0,
    green: 0,
    blue: 0,
};

/// The greys the theme's own surfaces take on paper: light enough to belong to the page the
/// background and foreground families are already forced onto, dark enough to be told apart.
const SELECTION_TINT: u8 = 92;
const MUTED_TINT: u8 = 84;

/// A page carries no hue: every color the theme brought arrives as the grey of its own weight, and
/// what tells two of them apart is where each falls between the white and the black.
///
/// Everything is brought over, not only the ground. COLOR derives readable content on top of each
/// surface but never restates the surface or the accent itself, so a color written for a dark
/// screen would otherwise arrive whole on paper — the accent washed out against the white, the
/// theme's surfaces still dark blocks.
pub(super) fn print(theme: &Theme) -> Theme {
    let colors = &theme.colors;
    let was = theme.mode;
    let ink = |color: Rgb| desaturate(restated(color, was, Mode::Light));
    let surface = |color: Rgb, tone: u8| desaturate(tinted(color, tone));

    let mut greys = Greys::new();
    let accent = greys.place(ink(colors.accent));
    let red = greys.family(
        ink(colors.red),
        ink(colors.dark_red),
        ink(colors.bright_red),
    );
    let green = greys.family(
        ink(colors.green),
        ink(colors.dark_green),
        ink(colors.bright_green),
    );
    let yellow = greys.family(
        ink(colors.yellow),
        ink(colors.dark_yellow),
        ink(colors.bright_yellow),
    );
    let blue = greys.family(
        ink(colors.blue),
        ink(colors.dark_blue),
        ink(colors.bright_blue),
    );
    let cyan = greys.family(
        ink(colors.cyan),
        ink(colors.dark_cyan),
        ink(colors.bright_cyan),
    );
    let magenta = greys.family(
        ink(colors.magenta),
        ink(colors.dark_magenta),
        ink(colors.bright_magenta),
    );
    let orange = greys.optional(colors.orange, colors.dark_orange, colors.bright_orange, ink);
    let brown = greys.optional(colors.brown, colors.dark_brown, colors.bright_brown, ink);

    Theme {
        mode: Mode::Light,
        colors: ThemeColors {
            background: WHITE,
            dark_background: WHITE,
            darker_background: WHITE,
            lighter_background: WHITE,

            foreground: BLACK,
            dark_foreground: BLACK,
            light_foreground: BLACK,
            bright_foreground: BLACK,

            accent,
            selection: surface(colors.selection, SELECTION_TINT),
            muted: surface(colors.muted, MUTED_TINT),

            red: red.0,
            dark_red: red.1,
            bright_red: red.2,

            yellow: yellow.0,
            dark_yellow: yellow.1,
            bright_yellow: yellow.2,

            green: green.0,
            dark_green: green.1,
            bright_green: green.2,

            cyan: cyan.0,
            dark_cyan: cyan.1,
            bright_cyan: cyan.2,

            blue: blue.0,
            dark_blue: blue.1,
            bright_blue: blue.2,

            magenta: magenta.0,
            dark_magenta: magenta.1,
            bright_magenta: magenta.2,

            orange: orange.0,
            dark_orange: orange.1,
            bright_orange: orange.2,

            brown: brown.0,
            dark_brown: brown.1,
            bright_brown: brown.2,
        },
    }
}

/// How far two greys stand apart to still read as two.
const APART: f64 = 5.0;

/// The band the colors keep to: dark enough to carry on the white, light enough not to be taken for
/// the text, which the page already forces to black.
const DARKEST: f64 = 15.0;
const LIGHTEST: f64 = 65.0;

/// The greys already spoken for.
///
/// Hue is what told a theme's colors apart and a page has none, so two of them can land on the same
/// grey — a theme whose red and blue read alike would print an error the way it prints a primary.
/// Each color keeps the grey its own weight gives it, and only one landing where another already
/// sits is moved, as little as moving takes.
struct Greys {
    taken: Vec<f64>,
}

impl Greys {
    fn new() -> Self {
        Self { taken: Vec::new() }
    }

    fn place(&mut self, grey: Rgb) -> Rgb {
        let free = self.free(tone_of(grey));
        self.taken.push(free);

        at(grey, free)
    }

    /// A color and the shades that belong to it: the shades follow it, keeping the distance the
    /// theme gave them, so a family moves without coming apart.
    fn family(&mut self, original: Rgb, dark: Rgb, bright: Rgb) -> (Rgb, Rgb, Rgb) {
        let placed = self.place(original);
        let shift = tone_of(placed) - tone_of(original);

        (
            placed,
            at(dark, tone_of(dark) + shift),
            at(bright, tone_of(bright) + shift),
        )
    }

    /// The same, for a color the theme may not have declared.
    fn optional(
        &mut self,
        original: Option<Rgb>,
        dark: Option<Rgb>,
        bright: Option<Rgb>,
        ink: impl Fn(Rgb) -> Rgb,
    ) -> (Option<Rgb>, Option<Rgb>, Option<Rgb>) {
        match (original, dark, bright) {
            (Some(original), Some(dark), Some(bright)) => {
                let (original, dark, bright) = self.family(ink(original), ink(dark), ink(bright));
                (Some(original), Some(dark), Some(bright))
            }
            _ => (None, None, None),
        }
    }

    /// The tone nearest `tone` that no color has taken, searched outward a step at a time so one
    /// already sitting clear does not move at all.
    fn free(&self, tone: f64) -> f64 {
        let wanted = tone.clamp(DARKEST, LIGHTEST);
        let steps = ((LIGHTEST - DARKEST) / APART).ceil() as i32;

        (0..=steps)
            .flat_map(|step| {
                let offset = f64::from(step) * APART;
                [wanted - offset, wanted + offset]
            })
            .filter(|candidate| (DARKEST..=LIGHTEST).contains(candidate))
            .find(|candidate| self.is_free(*candidate))
            .unwrap_or(wanted)
    }

    fn is_free(&self, tone: f64) -> bool {
        !self.taken.iter().any(|taken| (taken - tone).abs() < APART)
    }
}

fn at(color: Rgb, tone: f64) -> Rgb {
    Ramp::of(color).tone(tone.clamp(0.0, 100.0).round() as u8)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{chroma_of, separation};
    use crate::theme::read_theme;
    use std::path::Path;

    fn fixture(name: &str) -> Theme {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(name);
        read_theme(&path).unwrap()
    }

    #[test]
    fn forces_a_white_background_and_black_content() {
        let theme = fixture("everforest.toml");
        let printed = print(&theme);

        assert_eq!(printed.colors.background, WHITE);
        assert_eq!(printed.colors.lighter_background, WHITE);
        assert_eq!(printed.colors.foreground, BLACK);
        assert_eq!(printed.mode, Mode::Light);
    }

    /// The defect this transform was rewritten for: a theme's accent arrived on the page exactly as
    /// a dark screen had it, and a title carrying it washed out against the white.
    #[test]
    fn brings_the_accent_down_to_ink() {
        for name in ["everforest.toml", "flexoki-light.toml"] {
            let printed = print(&fixture(name));

            for ink in [printed.colors.accent, printed.colors.red] {
                let gap = separation(ink, WHITE);
                assert!(
                    gap >= 40.0,
                    "{name}: an accent separates from the page by {gap:.1}"
                );
            }
        }
    }

    #[test]
    fn leaves_no_hue_on_the_page() {
        let printed = print(&fixture("everforest.toml"));
        let colors = &printed.colors;

        for grey in [
            colors.accent,
            colors.red,
            colors.green,
            colors.selection,
            colors.muted,
        ] {
            assert!(chroma_of(grey) < 5.0, "{grey} still carries hue");
        }
    }

    /// Without hue there is only weight left to tell two colors apart, and a theme can easily give
    /// two of them the same.
    #[test]
    fn tells_apart_the_colors_that_only_their_hue_told_apart() {
        let printed = print(&fixture("everforest.toml"));
        let colors = &printed.colors;

        let mut greys = [
            colors.accent,
            colors.red,
            colors.green,
            colors.yellow,
            colors.blue,
            colors.cyan,
            colors.magenta,
        ]
        .map(tone_of);
        greys.sort_by(|one, other| one.partial_cmp(other).expect("a tone is never NaN"));

        for pair in greys.windows(2) {
            let gap = pair[1] - pair[0];
            assert!(gap >= APART - 1.0, "two colors print {gap:.1} apart");
        }
    }

    #[test]
    fn brings_the_theme_surfaces_onto_the_page() {
        let printed = print(&fixture("everforest.toml"));

        assert!(tone_of(printed.colors.selection) > 80.0);
        assert!(tone_of(printed.colors.muted) > 70.0);
    }
}
