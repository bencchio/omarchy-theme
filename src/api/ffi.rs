//! The C ABI: a separate contract from the Rust API's v1 (`docs/API.md`), not an extension of it —
//! see `BRIDGE.md` for the usage flow, the threading contract and what each representation does.
//!
//! `gui/bridge/omarchy_theme.h` is a hand-written mirror of this file and has to be kept in sync by
//! hand, since no generator produces it.

use std::ffi::{CStr, c_char, c_int};
use std::path::PathBuf;
use std::ptr;

use super::theme::ThemeWatch;
use crate::color::{self, Level, Role, Roles};
use crate::modes::{Representation, represent};
use crate::theme::{Rgb, Theme, default_theme_path};

/// An RGB color, one byte per channel — the same precision `colors.toml` declares.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OmarchyColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// One role: its own color, and the three weights of content that read on it.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OmarchyRole {
    pub color: OmarchyColor,
    pub content_main: OmarchyColor,
    pub content_secondary: OmarchyColor,
    pub content_disabled: OmarchyColor,
}

/// How many roles a snapshot carries — `Role::ALL.len()`, restated here so the header does not
/// need to reach back into Rust to size its array.
pub const OMARCHY_ROLE_COUNT: usize = 10;

/// One shade of a color: the shade itself and the content that reads on it.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OmarchyShade {
    pub color: OmarchyColor,
    pub content: OmarchyColor,
}

/// One color of the theme, as the family of three it reads as: the color itself, its dark shade and
/// its bright one, in that order. `declared` is false for `orange` and `brown` when the theme omits
/// them, and such a family carries no shade worth reading.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OmarchyColorFamily {
    pub original: OmarchyShade,
    pub dark: OmarchyShade,
    pub bright: OmarchyShade,
    pub declared: bool,
}

/// How many colors a snapshot carries: the six semantic ones and the two optional ones, each as a
/// family of three shades.
pub const OMARCHY_COLOR_COUNT: usize = 8;

/// Every role resolved against the theme in force, in the fixed order [`omarchy_role_name`]
/// names, and every color the theme declares, in the order [`omarchy_color_name`] names.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OmarchySnapshot {
    pub roles: [OmarchyRole; OMARCHY_ROLE_COUNT],
    pub colors: [OmarchyColorFamily; OMARCHY_COLOR_COUNT],
    pub dark: bool,
}

/// An opaque handle to a running [`ThemeWatch`]. Only ever seen through a pointer on the C side.
pub struct OmarchyWatcher(ThemeWatch);

fn to_color(rgb: crate::theme::Rgb) -> OmarchyColor {
    OmarchyColor {
        r: rgb.red,
        g: rgb.green,
        b: rgb.blue,
    }
}

/// How many representations [`omarchy_representation_name`] can name — `Representation::ALL.len()`.
/// Only the C side loops over this to build a mode switcher; Rust itself iterates
/// `Representation::ALL` directly, so this constant otherwise looks unused outside tests.
#[allow(dead_code)]
pub const OMARCHY_REPRESENTATION_COUNT: usize = 5;

/// The representation at `index`, in the same order [`omarchy_representation_name`] names. An
/// out-of-range index is not a caller error worth failing over — it falls back to `Original`,
/// documented here rather than left to panic across the FFI boundary.
fn representation_at(index: usize) -> Representation {
    Representation::ALL
        .get(index)
        .copied()
        .unwrap_or(Representation::Original)
}

/// Builds the snapshot a theme resolves to under `representation`. Kept apart from the `unsafe`
/// boundary functions so it can be tested directly, with an ordinary [`Theme`] and no raw pointers
/// involved.
fn snapshot(theme: &Theme, representation: Representation) -> OmarchySnapshot {
    let theme = represent(theme, representation);
    let roles = Roles::resolve(&theme);
    let mut entries = [OmarchyRole {
        color: OmarchyColor { r: 0, g: 0, b: 0 },
        content_main: OmarchyColor { r: 0, g: 0, b: 0 },
        content_secondary: OmarchyColor { r: 0, g: 0, b: 0 },
        content_disabled: OmarchyColor { r: 0, g: 0, b: 0 },
    }; OMARCHY_ROLE_COUNT];

    for (entry, role) in entries.iter_mut().zip(Role::ALL) {
        *entry = OmarchyRole {
            color: to_color(roles.color(role)),
            content_main: to_color(roles.content(role, Level::Primary)),
            content_secondary: to_color(roles.content(role, Level::Secondary)),
            content_disabled: to_color(roles.content(role, Level::Disabled)),
        };
    }

    OmarchySnapshot {
        roles: entries,
        colors: declared_colors(&theme),
        dark: theme.mode.is_dark(),
    }
}

/// The colors the theme declares, in the order [`omarchy_color_name`] names, each as the family of
/// three shades it reads as, and each shade with the content that reads on it. The two optional
/// colors keep their slot when absent, so the array's shape never depends on what a theme brought.
fn declared_colors(theme: &Theme) -> [OmarchyColorFamily; OMARCHY_COLOR_COUNT] {
    let colors = &theme.colors;
    let shade = |color: Rgb| OmarchyShade {
        color: to_color(color),
        content: to_color(color::on(color, colors)),
    };
    let family = |original: Rgb, dark: Rgb, bright: Rgb| OmarchyColorFamily {
        original: shade(original),
        dark: shade(dark),
        bright: shade(bright),
        declared: true,
    };
    let blank = OmarchyShade {
        color: OmarchyColor { r: 0, g: 0, b: 0 },
        content: OmarchyColor { r: 0, g: 0, b: 0 },
    };
    let absent = OmarchyColorFamily {
        original: blank,
        dark: blank,
        bright: blank,
        declared: false,
    };
    let optional = |original: Option<Rgb>, dark: Option<Rgb>, bright: Option<Rgb>| match (
        original, dark, bright,
    ) {
        (Some(original), Some(dark), Some(bright)) => family(original, dark, bright),
        _ => absent,
    };

    [
        family(colors.red, colors.dark_red, colors.bright_red),
        family(colors.green, colors.dark_green, colors.bright_green),
        family(colors.yellow, colors.dark_yellow, colors.bright_yellow),
        family(colors.blue, colors.dark_blue, colors.bright_blue),
        family(colors.cyan, colors.dark_cyan, colors.bright_cyan),
        family(colors.magenta, colors.dark_magenta, colors.bright_magenta),
        optional(colors.orange, colors.dark_orange, colors.bright_orange),
        optional(colors.brown, colors.dark_brown, colors.bright_brown),
    ]
}

/// The display name of the color at `index`, in the same order [`OmarchySnapshot::colors`] uses.
///
/// A static string, owned by the library — never freed by the caller. Returns null for an index at
/// or past [`OMARCHY_COLOR_COUNT`].
#[unsafe(no_mangle)]
pub extern "C" fn omarchy_color_name(index: usize) -> *const c_char {
    const NAMES: [&CStr; OMARCHY_COLOR_COUNT] = [
        c"Red", c"Green", c"Yellow", c"Blue", c"Cyan", c"Magenta", c"Orange", c"Brown",
    ];

    NAMES.get(index).map_or(ptr::null(), |name| name.as_ptr())
}

/// The display name of the role at `index`, in the same order [`OmarchySnapshot::roles`] uses.
///
/// A static string, owned by the library — never freed by the caller. Returns null for an index
/// past [`OMARCHY_ROLE_COUNT`].
#[unsafe(no_mangle)]
pub extern "C" fn omarchy_role_name(index: usize) -> *const c_char {
    let name: &CStr = match Role::ALL.get(index) {
        Some(Role::Background) => c"Background",
        Some(Role::Elevated) => c"Elevated",
        Some(Role::Recessed) => c"Recessed",
        Some(Role::Border) => c"Border",
        Some(Role::Primary) => c"Primary",
        Some(Role::Selection) => c"Selection",
        Some(Role::Muted) => c"Muted",
        Some(Role::Error) => c"Error",
        Some(Role::Focus) => c"Focus",
        Some(Role::Disabled) => c"Disabled",
        None => return ptr::null(),
    };
    name.as_ptr()
}

/// The display name of the representation at `index`, in the same order [`representation_at`]
/// reads. A static string, owned by the library — never freed by the caller. Returns null for an
/// index at or past [`OMARCHY_REPRESENTATION_COUNT`].
#[unsafe(no_mangle)]
pub extern "C" fn omarchy_representation_name(index: usize) -> *const c_char {
    let name: &CStr = match Representation::ALL.get(index) {
        Some(Representation::Original) => c"Original",
        Some(Representation::Inverted) => c"Inverted",
        Some(Representation::HighContrast) => c"HighContrast",
        Some(Representation::Mono) => c"Mono",
        Some(Representation::Print) => c"Print",
        None => return ptr::null(),
    };
    name.as_ptr()
}

/// Starts watching a theme: `path` a null-terminated path to a `colors.toml`, or null for the
/// system's active theme.
///
/// Returns null on failure, and — when `error_buffer` is non-null — writes a description into it,
/// truncated to fit `buffer_len` and always null-terminated when `buffer_len` is at least 1.
///
/// # Safety
///
/// `path`, if non-null, must point to a valid null-terminated C string. `error_buffer`, if
/// non-null, must be writable for `buffer_len` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn omarchy_watch(
    path: *const c_char,
    error_buffer: *mut c_char,
    buffer_len: usize,
) -> *mut OmarchyWatcher {
    let requested_path: Option<PathBuf> = if path.is_null() {
        None
    } else {
        // SAFETY: the caller promises `path` is a valid C string when non-null.
        let text = unsafe { CStr::from_ptr(path) }.to_string_lossy();
        Some(PathBuf::from(text.into_owned()))
    };
    let resolved = requested_path.unwrap_or_else(default_theme_path);

    match ThemeWatch::watch(&resolved) {
        Ok(watch) => Box::into_raw(Box::new(OmarchyWatcher(watch))),
        Err(error) => {
            // SAFETY: the caller promises `error_buffer` is writable for `buffer_len` bytes when
            // non-null.
            unsafe { write_error(error_buffer, buffer_len, &error.to_string()) };
            ptr::null_mut()
        }
    }
}

/// Writes `message` into `buffer`, truncated to fit and null-terminated. A no-op when `buffer` is
/// null or `buffer_len` is zero.
///
/// # Safety
///
/// `buffer` must be writable for `buffer_len` bytes when non-null.
unsafe fn write_error(buffer: *mut c_char, buffer_len: usize, message: &str) {
    if buffer.is_null() || buffer_len == 0 {
        return;
    }

    let capacity = buffer_len - 1;
    let truncated = message
        .as_bytes()
        .get(..capacity)
        .unwrap_or(message.as_bytes());

    // SAFETY: the caller promises `buffer` is writable for `buffer_len` bytes; `truncated` fits in
    // `capacity` and leaves room for the trailing null the line after writes.
    unsafe {
        ptr::copy_nonoverlapping(truncated.as_ptr().cast(), buffer, truncated.len());
        *buffer.add(truncated.len()) = 0;
    }
}

/// The theme currently in force, taken through the representation at `representation` (see
/// [`omarchy_representation_name`]; an out-of-range index falls back to `Original`).
///
/// # Safety
///
/// `watcher` must point to a live [`OmarchyWatcher`], and `out` must be writable for one
/// [`OmarchySnapshot`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn omarchy_current(
    watcher: *const OmarchyWatcher,
    representation: usize,
    out: *mut OmarchySnapshot,
) {
    // SAFETY: the caller promises both pointers are valid for this call.
    unsafe {
        let watch = &(*watcher).0;
        *out = snapshot(
            &watch.watcher().current(),
            representation_at(representation),
        );
    }
}

/// Fills `out` and returns `true` when a new theme arrived since the last call, taken through the
/// representation at `representation`; leaves `out` untouched and returns `false` otherwise.
///
/// # Safety
///
/// `watcher` must point to a live [`OmarchyWatcher`], and `out` must be writable for one
/// [`OmarchySnapshot`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn omarchy_poll_changed(
    watcher: *const OmarchyWatcher,
    representation: usize,
    out: *mut OmarchySnapshot,
) -> bool {
    // SAFETY: the caller promises both pointers are valid for this call.
    unsafe {
        let watch = &(*watcher).0;
        match watch.watcher().changed() {
            Some(theme) => {
                *out = snapshot(&theme, representation_at(representation));
                true
            }
            None => false,
        }
    }
}

/// The file descriptor the caller integrates into its own event loop, readable whenever a valid
/// theme change landed. Reading from it never blocks; draining it fully is the caller's job.
///
/// # Safety
///
/// `watcher` must point to a live [`OmarchyWatcher`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn omarchy_signal_fd(watcher: *const OmarchyWatcher) -> c_int {
    // SAFETY: the caller promises `watcher` is valid for this call.
    unsafe { (*watcher).0.signal_fd() }
}

/// Stops watching and releases `watcher`. A no-op on null; never call it twice on the same
/// pointer.
///
/// # Safety
///
/// `watcher` must be either null or a pointer this module returned from [`omarchy_watch`] that has
/// not already been freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn omarchy_free(watcher: *mut OmarchyWatcher) {
    if watcher.is_null() {
        return;
    }
    // SAFETY: the caller promises `watcher` is a live pointer this module handed out.
    drop(unsafe { Box::from_raw(watcher) });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::read_theme;
    use std::ffi::CString;
    use std::path::Path;

    fn fixture(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(name)
    }

    #[test]
    fn carries_the_colors_the_theme_declares() {
        let theme = read_theme(&fixture("everforest.toml")).unwrap();
        let snapshot = snapshot(&theme, Representation::Original);

        let red = snapshot.colors[0];
        assert_eq!(red.original.color, to_color(theme.colors.red));
        assert_eq!(red.dark.color, to_color(theme.colors.dark_red));
        assert_eq!(red.bright.color, to_color(theme.colors.bright_red));
        assert!(snapshot.colors.iter().take(6).all(|family| family.declared));
    }

    #[test]
    fn marks_an_optional_color_the_theme_leaves_out() {
        let theme = read_theme(&fixture("without-optional-keys.toml")).unwrap();
        let snapshot = snapshot(&theme, Representation::Original);

        let orange = snapshot.colors[6];
        assert!(!orange.declared);
        assert!(theme.colors.orange.is_none());
    }

    #[test]
    fn names_every_color_it_carries() {
        for index in 0..OMARCHY_COLOR_COUNT {
            assert!(!omarchy_color_name(index).is_null());
        }
        assert!(omarchy_color_name(OMARCHY_COLOR_COUNT).is_null());
    }

    #[test]
    fn lists_roles_in_the_order_role_all_declares() {
        let theme = read_theme(&fixture("everforest.toml")).unwrap();
        let roles = Roles::resolve(&theme);
        let built = snapshot(&theme, Representation::Original);

        for (index, role) in Role::ALL.into_iter().enumerate() {
            assert_eq!(built.roles[index].color, to_color(roles.color(role)));
        }
    }

    #[test]
    fn carries_the_theme_mode() {
        let dark = read_theme(&fixture("everforest.toml")).unwrap();
        let light = read_theme(&fixture("flexoki-light.toml")).unwrap();

        assert!(snapshot(&dark, Representation::Original).dark);
        assert!(!snapshot(&light, Representation::Original).dark);
    }

    #[test]
    fn applies_the_requested_representation() {
        let dark = read_theme(&fixture("everforest.toml")).unwrap();

        // Inverted flips the mode; Original leaves it as declared.
        assert!(!snapshot(&dark, Representation::Inverted).dark);
        assert!(snapshot(&dark, Representation::Original).dark);
    }

    #[test]
    fn names_every_role_and_stops_at_the_end() {
        for (index, role) in Role::ALL.into_iter().enumerate() {
            let name = unsafe { CStr::from_ptr(omarchy_role_name(index)) }
                .to_str()
                .unwrap();
            assert_eq!(name, format!("{role:?}"));
        }

        assert!(omarchy_role_name(OMARCHY_ROLE_COUNT).is_null());
    }

    #[test]
    fn names_every_representation_and_stops_at_the_end() {
        for (index, representation) in Representation::ALL.into_iter().enumerate() {
            let name = unsafe { CStr::from_ptr(omarchy_representation_name(index)) }
                .to_str()
                .unwrap();
            assert_eq!(name, format!("{representation:?}"));
        }

        assert!(omarchy_representation_name(OMARCHY_REPRESENTATION_COUNT).is_null());
    }

    #[test]
    fn falls_back_to_original_for_an_out_of_range_representation() {
        assert_eq!(
            representation_at(OMARCHY_REPRESENTATION_COUNT),
            Representation::Original
        );
    }

    #[test]
    fn opens_reads_and_closes_a_watcher() {
        let path = CString::new(fixture("everforest.toml").to_str().unwrap()).unwrap();

        unsafe {
            let watcher = omarchy_watch(path.as_ptr(), ptr::null_mut(), 0);
            assert!(!watcher.is_null());

            let mut out = std::mem::zeroed::<OmarchySnapshot>();
            omarchy_current(watcher, 0, &mut out);
            assert!(out.dark);

            assert!(omarchy_signal_fd(watcher) >= 0);

            omarchy_free(watcher);
        }
    }

    #[test]
    fn reports_a_missing_theme_through_the_error_buffer() {
        let path = CString::new(fixture("does-not-exist.toml").to_str().unwrap()).unwrap();
        let mut buffer = [0 as c_char; 128];

        unsafe {
            let watcher = omarchy_watch(path.as_ptr(), buffer.as_mut_ptr(), buffer.len());
            assert!(watcher.is_null());

            let message = CStr::from_ptr(buffer.as_ptr()).to_str().unwrap();
            assert!(message.contains("no theme found"), "got: {message}");
        }
    }

    #[test]
    fn truncates_a_message_too_long_for_the_buffer() {
        let path = CString::new(fixture("does-not-exist.toml").to_str().unwrap()).unwrap();
        let mut buffer = [1 as c_char; 8];

        unsafe {
            omarchy_watch(path.as_ptr(), buffer.as_mut_ptr(), buffer.len());
            assert_eq!(buffer[7], 0, "the last byte must stay the null terminator");
        }
    }
}
