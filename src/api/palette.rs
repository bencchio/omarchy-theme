//! The palette contract: basic in, extended and readable out.

use crate::color::{Level, Ramp, Role, Roles, tone_of};
use crate::theme::{Mode, Rgb, Theme, ThemeColors, ThemeError, read_theme};

use crate::color;

/// The palette an application or the theme reader starts from: a few core colors.
///
/// `background`, `foreground` and `accent` are required; everything else is optional and, when
/// left unstated, the library derives a readable value for it during [`extend`](Self::extend). This
/// is the one loading surface: both the active Omarchy theme and an application-built palette enter
/// through these constructors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaletteSeed {
    pub(crate) background: Rgb,
    pub(crate) foreground: Rgb,
    pub(crate) accent: Rgb,
    selection: Option<Rgb>,
    muted: Option<Rgb>,
    error: Option<Rgb>,
    surface_light: Option<Rgb>,
    surface_dark: Option<Rgb>,
    mode: Option<Mode>,
}

impl PaletteSeed {
    /// The basic palette of the system's active Omarchy theme.
    pub fn from_active_theme() -> Result<Self, ThemeError> {
        Self::from_path(&crate::theme::default_theme_path())
    }

    /// The basic palette of an Omarchy-style theme read from `path`.
    pub fn from_path(path: &std::path::Path) -> Result<Self, ThemeError> {
        let theme = read_theme(path)?;
        Ok(Self::from_theme(&theme))
    }

    /// The basic palette of a [`Theme`] already read, such as one a [`ThemeWatcher`](crate::theme::ThemeWatcher) holds.
    pub(crate) fn from_theme(theme: &Theme) -> Self {
        let colors = &theme.colors;
        Self {
            background: colors.background,
            foreground: colors.foreground,
            accent: colors.accent,
            selection: Some(colors.selection),
            muted: Some(colors.muted),
            error: Some(colors.red),
            surface_light: Some(colors.lighter_background),
            surface_dark: Some(colors.dark_background),
            mode: Some(theme.mode),
        }
    }

    /// A basic palette built by the application itself, from the three colors it owns.
    pub fn new(background: Rgb, foreground: Rgb, accent: Rgb) -> Self {
        Self {
            background,
            foreground,
            accent,
            selection: None,
            muted: None,
            error: None,
            surface_light: None,
            surface_dark: None,
            mode: None,
        }
    }

    /// The ground behind selected content. Defaults to the accent.
    pub fn with_selection(mut self, color: Rgb) -> Self {
        self.selection = Some(color);
        self
    }

    /// De-emphasized content. Defaults to the foreground.
    pub fn with_muted(mut self, color: Rgb) -> Self {
        self.muted = Some(color);
        self
    }

    /// The color that signals something went wrong. Defaults to a documented red.
    pub fn with_error(mut self, color: Rgb) -> Self {
        self.error = Some(color);
        self
    }

    /// The raised surface. Defaults to a lighter step of the background's own ramp.
    pub fn with_surface_light(mut self, color: Rgb) -> Self {
        self.surface_light = Some(color);
        self
    }

    /// The recessed surface. Defaults to a darker step of the background's own ramp.
    pub fn with_surface_dark(mut self, color: Rgb) -> Self {
        self.surface_dark = Some(color);
        self
    }

    /// The mode the palette reads as, instead of the one inferred from the background's tone.
    pub fn with_mode(mut self, mode: Mode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// The extended legible palette this basic one reads as: roles, content and tonal ramps.
    pub fn extend(&self) -> Palette {
        let mode = self.mode.unwrap_or_else(|| {
            if tone_of(self.background) < 50.0 {
                Mode::Dark
            } else {
                Mode::Light
            }
        });
        let colors = self.derived_colors();
        let roles = Roles::resolve(&Theme {
            mode,
            colors: colors.clone(),
        });
        Palette {
            roles,
            colors,
            mode,
        }
    }

    fn derived_colors(&self) -> ThemeColors {
        let background = self.background;
        let background_tone = tone_of(background);
        let foreground = self.foreground;

        ThemeColors {
            accent: self.accent,
            selection: self.selection.unwrap_or(self.accent),
            muted: self.muted.unwrap_or(foreground),

            background,
            dark_background: self
                .surface_dark
                .unwrap_or_else(|| Ramp::of(background).tone((background_tone - 15.0) as u8)),
            darker_background: Ramp::of(background).tone((background_tone - 30.0) as u8),
            lighter_background: self
                .surface_light
                .unwrap_or_else(|| Ramp::of(background).tone((background_tone + 15.0) as u8)),

            foreground,
            dark_foreground: foreground,
            light_foreground: foreground,
            bright_foreground: foreground,

            red: self.error.unwrap_or(DEFAULT_ERROR),
            dark_red: self.error.unwrap_or(DEFAULT_ERROR),
            bright_red: self.error.unwrap_or(DEFAULT_ERROR),

            yellow: foreground,
            dark_yellow: foreground,
            bright_yellow: foreground,

            green: foreground,
            dark_green: foreground,
            bright_green: foreground,

            cyan: foreground,
            dark_cyan: foreground,
            bright_cyan: foreground,

            blue: foreground,
            dark_blue: foreground,
            bright_blue: foreground,

            magenta: foreground,
            dark_magenta: foreground,
            bright_magenta: foreground,

            orange: None,
            dark_orange: None,
            bright_orange: None,

            brown: None,
            dark_brown: None,
            bright_brown: None,
        }
    }
}

/// The error color a basic palette gets when the application does not state one. Red, distinct
/// from the accent, so a failing element reads as such no matter the theme.
const DEFAULT_ERROR: Rgb = Rgb {
    red: 229,
    green: 72,
    blue: 77,
};

/// The extended legible palette: what an application actually draws with.
///
/// Resolved once from a [`PaletteSeed`]; every color and ramp here already guarantees its content
/// stays readable on its ground. This is the v1 public contract of the library.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palette {
    roles: Roles,
    colors: ThemeColors,
    mode: Mode,
}

impl Palette {
    /// The concrete color of a semantic role, such as the accent or the canvas.
    pub fn color(&self, role: Role) -> Rgb {
        self.roles.color(role)
    }

    /// The main content that stays readable on `role`.
    pub fn on(&self, role: Role) -> Rgb {
        self.roles.on(role)
    }

    /// The content of a given weight that reads on `role`.
    pub fn content(&self, role: Role, level: Level) -> Rgb {
        self.roles.content(role, level)
    }

    /// The tonal ramp a role scales along, from black to white at its hue.
    pub fn ramp(&self, role: Role) -> Ramp {
        self.roles.ramp(role)
    }

    /// Whether the palette is built on a dark ground.
    pub fn dark(&self) -> bool {
        self.mode.is_dark()
    }

    /// Content that stays readable on `color`, whether or not it plays a role here — the same
    /// guarantee the roles carry, lifted onto any color the application wants to draw with.
    pub fn on_color(&self, color: Rgb) -> Rgb {
        color::on(color, &self.colors)
    }
}
