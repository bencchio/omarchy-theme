//! Reading the active theme: from a path on disk to a `Theme`, or to an error that says why not.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use toml::Table;

use super::Theme;
use super::colors::ThemeColors;
use super::error::ThemeError;
use super::mode::Mode;

const THEME_FILE: &str = ".config/omarchy/current/theme/colors.toml";

/// The `current` symlink is deliberately left unresolved: the path is stable across themes and
/// switching theme rewrites the file behind it, so following it would pin the theme in place.
pub fn default_theme_path() -> PathBuf {
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_default();
    home.join(THEME_FILE)
}

pub fn read_active_theme() -> Result<Theme, ThemeError> {
    read_theme(&default_theme_path())
}

pub fn read_theme(path: &Path) -> Result<Theme, ThemeError> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(cause) if cause.kind() == io::ErrorKind::NotFound => {
            return Err(ThemeError::NotFound(path.to_owned()));
        }
        Err(cause) => return Err(ThemeError::Unreadable(path.to_owned(), cause)),
    };

    let table: Table =
        toml::from_str(&text).map_err(|cause| ThemeError::InvalidToml(path.to_owned(), cause))?;

    Ok(Theme {
        mode: read_mode(&table)?,
        colors: ThemeColors::from_table(&table)?,
    })
}

fn read_mode(table: &Table) -> Result<Mode, ThemeError> {
    let value = table.get("mode").ok_or(ThemeError::MissingKey("mode"))?;
    let text = value
        .as_str()
        .ok_or_else(|| ThemeError::InvalidMode(value.to_string()))?;
    Mode::parse(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Rgb;

    fn fixture(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(name)
    }

    fn read_fixture(name: &str) -> Result<Theme, ThemeError> {
        read_theme(&fixture(name))
    }

    #[test]
    fn reads_a_complete_dark_theme() {
        let theme = read_fixture("everforest.toml").unwrap();

        assert_eq!(theme.mode, Mode::Dark);
        assert_eq!(theme.colors.background, Rgb::parse("k", "#2d353b").unwrap());
        assert_eq!(theme.colors.foreground, Rgb::parse("k", "#d3c6aa").unwrap());
        assert_eq!(theme.colors.accent, Rgb::parse("k", "#7fbbb3").unwrap());
        assert_eq!(theme.colors.selection, Rgb::parse("k", "#3d484d").unwrap());
    }

    #[test]
    fn reads_a_light_theme() {
        let theme = read_fixture("flexoki-light.toml").unwrap();

        assert_eq!(theme.mode, Mode::Light);
        assert!(!theme.mode.is_dark());
    }

    #[test]
    fn accepts_a_theme_without_the_optional_keys() {
        let theme = read_fixture("without-optional-keys.toml").unwrap();

        assert_eq!(theme.colors.orange, None);
        assert_eq!(theme.colors.brown, None);
        assert_eq!(theme.colors.background, Rgb::parse("k", "#2d353b").unwrap());
    }

    #[test]
    fn reads_the_optional_keys_when_present() {
        let theme = read_fixture("everforest.toml").unwrap();

        assert_eq!(
            theme.colors.orange,
            Some(Rgb::parse("k", "#e09d7f").unwrap())
        );
        assert_eq!(
            theme.colors.brown,
            Some(Rgb::parse("k", "#704e3f").unwrap())
        );
    }

    #[test]
    fn ignores_keys_meant_for_other_applications() {
        let theme = read_fixture("with-extra-keys.toml").unwrap();

        assert_eq!(theme.colors.background, Rgb::parse("k", "#2d353b").unwrap());
    }

    #[test]
    fn tells_a_missing_theme_from_an_invalid_file() {
        let missing = read_fixture("does-not-exist.toml").unwrap_err();
        assert!(matches!(missing, ThemeError::NotFound(_)));

        let invalid = read_fixture("malformed.toml").unwrap_err();
        assert!(matches!(invalid, ThemeError::InvalidToml(_, _)));
    }

    #[test]
    fn reports_the_missing_key() {
        let error = read_fixture("missing-background.toml").unwrap_err();
        assert!(matches!(error, ThemeError::MissingKey("background")));
    }

    #[test]
    fn reports_the_invalid_mode() {
        let error = read_fixture("invalid-mode.toml").unwrap_err();
        assert!(matches!(error, ThemeError::InvalidMode(value) if value == "sepia"));
    }

    #[test]
    fn reports_the_invalid_color_with_its_key() {
        let error = read_fixture("invalid-color.toml").unwrap_err();
        assert!(
            matches!(error, ThemeError::InvalidColor("accent", value) if value == "rgb(1e1e1e)")
        );
    }

    #[test]
    fn the_default_path_points_at_the_active_theme() {
        let path = default_theme_path();

        assert!(path.ends_with(THEME_FILE));
        if std::env::var_os("HOME").is_some() {
            assert!(path.is_absolute());
        }
    }
}
