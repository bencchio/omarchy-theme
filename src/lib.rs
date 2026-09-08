//! Colors for Omarchy applications, inherited from the system's active theme.

pub mod api;
pub mod color;
pub mod modes;
pub mod theme;

pub use api::{Palette, PaletteSeed, ThemeWatch};

pub use color::{Level, Ramp, Role, Roles, chroma_of, desaturate, hue_gap, separation, tone_of};
pub use modes::{Representation, represent};
pub use theme::{
    Mode, Rgb, Theme, ThemeColors, ThemeError, ThemeWatcher, default_theme_path, read_active_theme,
    read_theme,
};
