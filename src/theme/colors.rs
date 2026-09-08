//! The color keys `colors.toml` declares, handed over as they are — with one completion: a bright
//! variant the theme leaves out is derived from the color it belongs to, so every color a theme
//! brings has one and no consumer has to decide what to do about a hole.

use toml::Table;

use super::error::ThemeError;
use super::rgb::Rgb;
use super::shades::{brighter, darker};

/// `orange` and `brown` are optional because part of the Omarchy catalog omits them; their shades
/// follow them, since a shade of a color the theme never declared means nothing.
///
/// Each color comes as a family of three, in the order they read: the color itself, its dark shade
/// and its bright one. Every shade is taken from the theme when it declares it and derived from the
/// color when it does not — the catalog declares no dark shade at all, and no bright shade for
/// `orange` or `brown`.
///
/// Keys that integrate with other applications (`hyprland_active_border`,
/// `active_tab_background`, …) are ignored: they are not part of the color contract and carry
/// forms this domain does not read, such as `rgba()` or gradients.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemeColors {
    pub accent: Rgb,
    pub selection: Rgb,
    pub muted: Rgb,

    pub background: Rgb,
    pub dark_background: Rgb,
    pub darker_background: Rgb,
    pub lighter_background: Rgb,

    pub foreground: Rgb,
    pub dark_foreground: Rgb,
    pub light_foreground: Rgb,
    pub bright_foreground: Rgb,

    pub red: Rgb,
    pub dark_red: Rgb,
    pub bright_red: Rgb,

    pub yellow: Rgb,
    pub dark_yellow: Rgb,
    pub bright_yellow: Rgb,

    pub green: Rgb,
    pub dark_green: Rgb,
    pub bright_green: Rgb,

    pub cyan: Rgb,
    pub dark_cyan: Rgb,
    pub bright_cyan: Rgb,

    pub blue: Rgb,
    pub dark_blue: Rgb,
    pub bright_blue: Rgb,

    pub magenta: Rgb,
    pub dark_magenta: Rgb,
    pub bright_magenta: Rgb,

    pub orange: Option<Rgb>,
    pub dark_orange: Option<Rgb>,
    pub bright_orange: Option<Rgb>,

    pub brown: Option<Rgb>,
    pub dark_brown: Option<Rgb>,
    pub bright_brown: Option<Rgb>,
}

impl ThemeColors {
    pub(super) fn from_table(table: &Table) -> Result<Self, ThemeError> {
        let red = required(table, "red")?;
        let yellow = required(table, "yellow")?;
        let green = required(table, "green")?;
        let cyan = required(table, "cyan")?;
        let blue = required(table, "blue")?;
        let magenta = required(table, "magenta")?;
        let orange = optional(table, "orange")?;
        let brown = optional(table, "brown")?;

        Ok(Self {
            accent: required(table, "accent")?,
            selection: required(table, "selection")?,
            muted: required(table, "muted")?,

            background: required(table, "background")?,
            dark_background: required(table, "dark_background")?,
            darker_background: required(table, "darker_background")?,
            lighter_background: required(table, "lighter_background")?,

            foreground: required(table, "foreground")?,
            dark_foreground: required(table, "dark_foreground")?,
            light_foreground: required(table, "light_foreground")?,
            bright_foreground: required(table, "bright_foreground")?,

            red,
            dark_red: shade(table, "dark_red", red, darker)?,
            bright_red: shade(table, "bright_red", red, brighter)?,

            yellow,
            dark_yellow: shade(table, "dark_yellow", yellow, darker)?,
            bright_yellow: shade(table, "bright_yellow", yellow, brighter)?,

            green,
            dark_green: shade(table, "dark_green", green, darker)?,
            bright_green: shade(table, "bright_green", green, brighter)?,

            cyan,
            dark_cyan: shade(table, "dark_cyan", cyan, darker)?,
            bright_cyan: shade(table, "bright_cyan", cyan, brighter)?,

            blue,
            dark_blue: shade(table, "dark_blue", blue, darker)?,
            bright_blue: shade(table, "bright_blue", blue, brighter)?,

            magenta,
            dark_magenta: shade(table, "dark_magenta", magenta, darker)?,
            bright_magenta: shade(table, "bright_magenta", magenta, brighter)?,

            orange,
            dark_orange: optional_shade(table, "dark_orange", orange, darker)?,
            bright_orange: optional_shade(table, "bright_orange", orange, brighter)?,

            brown,
            dark_brown: optional_shade(table, "dark_brown", brown, darker)?,
            bright_brown: optional_shade(table, "bright_brown", brown, brighter)?,
        })
    }
}

fn required(table: &Table, key: &'static str) -> Result<Rgb, ThemeError> {
    let value = table.get(key).ok_or(ThemeError::MissingKey(key))?;
    let text = value
        .as_str()
        .ok_or_else(|| ThemeError::InvalidColor(key, value.to_string()))?;
    Rgb::parse(key, text)
}

/// The shade `key` names, read when the theme declares it and derived from `base` with `derive`
/// otherwise.
///
/// A shade declared identical to the color it belongs to has not been distinguished from it, which
/// over half the Omarchy catalog does, so the derived one stands in — a family that showed the same
/// color twice would say nothing about the theme.
fn shade(
    table: &Table,
    key: &'static str,
    base: Rgb,
    derive: fn(Rgb) -> Rgb,
) -> Result<Rgb, ThemeError> {
    let declared = optional(table, key)?.filter(|shade| *shade != base);

    Ok(declared.unwrap_or_else(|| derive(base)))
}

/// The same, for a shade whose color is itself optional: no color, no shade.
fn optional_shade(
    table: &Table,
    key: &'static str,
    base: Option<Rgb>,
    derive: fn(Rgb) -> Rgb,
) -> Result<Option<Rgb>, ThemeError> {
    match base {
        None => Ok(None),
        Some(base) => shade(table, key, base, derive).map(Some),
    }
}

fn optional(table: &Table, key: &'static str) -> Result<Option<Rgb>, ThemeError> {
    match table.contains_key(key) {
        false => Ok(None),
        true => required(table, key).map(Some),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    /// The reference fixture, optionally with one key dropped, to stand for a theme that leaves it
    /// out — no Omarchy theme ships without the bright keys, so none can be borrowed for this.
    fn table(without: &str) -> Table {
        text(without, "")
            .parse()
            .expect("the fixture is valid toml")
    }

    /// The same, with `line` put back in place of the key that was dropped — for the theme that
    /// declares a shade of its own, which the catalog has none of either.
    fn table_declaring(key: &str, line: &str) -> Table {
        text(key, line).parse().expect("the fixture is valid toml")
    }

    fn text(without: &str, extra: &str) -> String {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/everforest.toml");
        let fixture = std::fs::read_to_string(path).expect("the fixture is readable");

        let mut kept: Vec<&str> = fixture
            .lines()
            .filter(|line| without.is_empty() || !line.starts_with(without))
            .collect();
        if !extra.is_empty() {
            kept.push(extra);
        }

        kept.join("\n")
    }

    #[test]
    fn reads_a_shade_the_theme_tells_apart_from_its_color() {
        let colors = ThemeColors::from_table(&table_declaring(
            "bright_red",
            r##"bright_red = "#ff0000""##,
        ))
        .unwrap();

        assert_eq!(
            colors.bright_red,
            Rgb::parse("bright_red", "#ff0000").unwrap()
        );
    }

    /// The fixture declares bright_red equal to red, as over half the Omarchy catalog does. A theme
    /// that repeats a color has not given it a shade, so the derived one stands in.
    #[test]
    fn derives_a_shade_the_theme_declares_equal_to_its_color() {
        let colors = ThemeColors::from_table(&table("")).unwrap();

        assert_ne!(colors.bright_red, colors.red);
        assert_ne!(colors.dark_red, colors.red);
        assert_ne!(colors.dark_red, colors.bright_red);
    }

    #[test]
    fn derives_a_shade_the_theme_leaves_out() {
        let colors = ThemeColors::from_table(&table("bright_red")).unwrap();

        assert_ne!(colors.bright_red, colors.red);
    }

    /// No Omarchy theme declares these two, so they are always derived when their base is there.
    #[test]
    fn derives_the_shades_of_the_optional_colors() {
        let colors = ThemeColors::from_table(&table("")).unwrap();

        assert!(colors.orange.is_some());
        assert!(colors.bright_orange.is_some());
        assert!(colors.dark_orange.is_some());
        assert_ne!(colors.bright_orange, colors.orange);
        assert_ne!(colors.dark_orange, colors.orange);
    }

    #[test]
    fn leaves_out_the_shades_of_a_color_the_theme_never_declared() {
        let colors = ThemeColors::from_table(&table("orange")).unwrap();

        assert!(colors.orange.is_none());
        assert!(colors.dark_orange.is_none());
        assert!(colors.bright_orange.is_none());
    }
}
