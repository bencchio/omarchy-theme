//! THEME — reads the active Omarchy theme and exposes its keys without interpreting them.

mod colors;
mod error;
mod mode;
mod reader;
mod rgb;
mod shades;
mod watcher;

pub use colors::ThemeColors;
pub use error::ThemeError;
pub use mode::Mode;
pub use reader::{default_theme_path, read_active_theme, read_theme};
pub use rgb::Rgb;
pub use watcher::ThemeWatcher;

/// A theme as the system declares it: its mode and its base colors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
    pub mode: Mode,
    pub colors: ThemeColors,
}
