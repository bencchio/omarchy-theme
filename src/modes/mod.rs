//! MODES — the same theme, taken through a different representation before COLOR resolves it.

mod accent;
mod high_contrast;
mod invert;
mod mono;
mod print;

use crate::theme::{Rgb, Theme, ThemeColors};

/// A way of taking the theme's raw colors before [`Roles::resolve`](crate::color::Roles::resolve)
/// sees them. Each variant returns a full [`Theme`], so every guarantee COLOR already makes
/// (surfaces that stand off the background, content that stays in its ground's family, roles that
/// do not collide) is recomputed fresh from the transformed base — never duplicated here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Representation {
    /// The theme as declared, untouched.
    Original,
    /// Every color's tone reflected, and the theme's mode flipped with it.
    Inverted,
    /// Every color pushed further from the background than it already stood.
    HighContrast,
    /// Every color desaturated: same tone, no hue.
    Mono,
    /// Forced onto a page: white behind, black in front, and every color as its own grey.
    Print,
}

impl Representation {
    pub const ALL: [Self; 5] = [
        Self::Original,
        Self::Inverted,
        Self::HighContrast,
        Self::Mono,
        Self::Print,
    ];
}

/// Takes `theme` through `representation`, returning a full theme COLOR can resolve as usual.
pub fn represent(theme: &Theme, representation: Representation) -> Theme {
    match representation {
        Representation::Original => theme.clone(),
        Representation::Inverted => invert::invert(theme),
        Representation::HighContrast => high_contrast::amplify(theme),
        Representation::Mono => mono::desaturate(theme),
        Representation::Print => print::print(theme),
    }
}

/// Applies `f` to every color the theme declares, optional keys included.
pub(super) fn map_colors(colors: &ThemeColors, f: impl Fn(Rgb) -> Rgb) -> ThemeColors {
    map_split(colors, &f, &f)
}

/// Applies `structure` to the colors that carry the page — the grounds, the text, and the surfaces
/// the roles rest on — and `accent` to the ones that accent it. The split a representation needs
/// whenever the two cannot take the same transform.
pub(super) fn map_split(
    colors: &ThemeColors,
    structure: impl Fn(Rgb) -> Rgb,
    accent: impl Fn(Rgb) -> Rgb,
) -> ThemeColors {
    ThemeColors {
        accent: accent(colors.accent),
        selection: structure(colors.selection),
        muted: structure(colors.muted),

        background: structure(colors.background),
        dark_background: structure(colors.dark_background),
        darker_background: structure(colors.darker_background),
        lighter_background: structure(colors.lighter_background),

        foreground: structure(colors.foreground),
        dark_foreground: structure(colors.dark_foreground),
        light_foreground: structure(colors.light_foreground),
        bright_foreground: structure(colors.bright_foreground),

        red: accent(colors.red),
        dark_red: accent(colors.dark_red),
        bright_red: accent(colors.bright_red),

        yellow: accent(colors.yellow),
        dark_yellow: accent(colors.dark_yellow),
        bright_yellow: accent(colors.bright_yellow),

        green: accent(colors.green),
        dark_green: accent(colors.dark_green),
        bright_green: accent(colors.bright_green),

        cyan: accent(colors.cyan),
        dark_cyan: accent(colors.dark_cyan),
        bright_cyan: accent(colors.bright_cyan),

        blue: accent(colors.blue),
        dark_blue: accent(colors.dark_blue),
        bright_blue: accent(colors.bright_blue),

        magenta: accent(colors.magenta),
        dark_magenta: accent(colors.dark_magenta),
        bright_magenta: accent(colors.bright_magenta),

        orange: colors.orange.map(&accent),
        dark_orange: colors.dark_orange.map(&accent),
        bright_orange: colors.bright_orange.map(&accent),

        brown: colors.brown.map(&accent),
        dark_brown: colors.dark_brown.map(&accent),
        bright_brown: colors.bright_brown.map(&accent),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{Level, Role, Roles, separation};
    use crate::theme::read_theme;
    use std::path::{Path, PathBuf};

    fn fixture(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(name)
    }

    /// Every guarantee COLOR already makes must survive every representation: this is what makes
    /// "transform the theme, reuse Roles::resolve" a safe design instead of a hopeful one.
    #[test]
    fn every_representation_still_resolves_a_coherent_palette() {
        for fixture_name in ["everforest.toml", "flexoki-light.toml"] {
            let theme = read_theme(&fixture(fixture_name)).unwrap();

            for representation in Representation::ALL {
                let transformed = represent(&theme, representation);
                let roles = Roles::resolve(&transformed);

                for role in Role::ALL {
                    let ground = roles.color(role);
                    let main = roles.content(role, Level::Primary);
                    let gap = separation(main, ground);
                    assert!(
                        gap >= 40.0,
                        "{fixture_name} {representation:?} {role:?}: content separates by only {gap:.1}"
                    );
                }
            }
        }
    }

    #[test]
    fn original_changes_nothing() {
        let theme = read_theme(&fixture("everforest.toml")).unwrap();
        assert_eq!(represent(&theme, Representation::Original), theme);
    }
}
