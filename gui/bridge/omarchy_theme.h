// The C ABI: a separate contract from the Rust API's v1 (docs/API.md), not an extension of it — see
// BRIDGE.md for the usage flow, the threading contract and what each representation does.
//
// Hand-written to mirror src/api/ffi.rs exactly; nothing generates it, so a change on the Rust side
// has to be copied here by hand.
#pragma once

// This header is plain C, not C++: `typedef struct` and `#define` are its idiomatic, portable
// form, valid from a C compiler too, and NOT what CODE.cpp.md's clang-tidy checks are written for
// — silence the modernize/cppcoreguidelines suggestions that would otherwise push this file
// towards C++-only syntax it cannot use.
// NOLINTBEGIN(modernize-use-using,modernize-avoid-c-arrays,cppcoreguidelines-avoid-c-arrays,cppcoreguidelines-macro-usage)

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// An RGB color, one byte per channel — the same precision colors.toml declares.
typedef struct {
    uint8_t r;
    uint8_t g;
    uint8_t b;
} OmarchyColor;

// One role: its own color, and the three weights of content that read on it.
typedef struct {
    OmarchyColor color;
    OmarchyColor content_main;
    OmarchyColor content_secondary;
    OmarchyColor content_disabled;
} OmarchyRole;

// How many roles a snapshot carries.
#define OMARCHY_ROLE_COUNT 10

// One shade of a color: the shade itself and the content that reads on it.
typedef struct {
    OmarchyColor color;
    OmarchyColor content;
} OmarchyShade;

// One color of the theme, as the family of three it reads as: the color itself, its dark shade and
// its bright one, in that order. `declared` is false for orange and brown when the theme omits
// them.
typedef struct {
    OmarchyShade original;
    OmarchyShade dark;
    OmarchyShade bright;
    bool declared;
} OmarchyColorFamily;

// How many colors a snapshot carries: the six semantic ones and the two optional ones, each as a
// family of three shades.
#define OMARCHY_COLOR_COUNT 8

// Every role resolved against the theme in force, in the fixed order omarchy_role_name names, and
// every color the theme declares, in the order omarchy_color_name names.
typedef struct {
    OmarchyRole roles[OMARCHY_ROLE_COUNT];
    OmarchyColorFamily colors[OMARCHY_COLOR_COUNT];
    bool dark;
} OmarchySnapshot;

// An opaque handle to a running watcher. Only ever seen through a pointer.
typedef struct OmarchyWatcher OmarchyWatcher;

// The display name of the role at `index`, in the same order OmarchySnapshot::roles uses.
//
// A static string, owned by the library — never freed by the caller. Returns NULL for an index at
// or past OMARCHY_ROLE_COUNT.
const char *omarchy_role_name(size_t index);

// The display name of the color at `index`, in the same order OmarchySnapshot::colors uses.
//
// A static string, owned by the library — never freed by the caller. Returns NULL for an index at
// or past OMARCHY_COLOR_COUNT.
const char *omarchy_color_name(size_t index);

// How many representations omarchy_representation_name can name.
#define OMARCHY_REPRESENTATION_COUNT 5

// The display name of the representation at `index` — Original, Inverted, HighContrast, Mono or
// Print, in that order.
//
// A static string, owned by the library — never freed by the caller. Returns NULL for an index at
// or past OMARCHY_REPRESENTATION_COUNT.
const char *omarchy_representation_name(size_t index);

// Starts watching a theme: `path` a null-terminated path to a colors.toml, or NULL for the
// system's active theme.
//
// Returns NULL on failure, and — when `error_buffer` is non-NULL — writes a description into it,
// truncated to fit `buffer_len` and always null-terminated when `buffer_len` is at least 1.
OmarchyWatcher *omarchy_watch(const char *path, char *error_buffer, size_t buffer_len);

// Writes the theme currently in force into `*out`, taken through the representation at
// `representation` (an out-of-range index falls back to Original).
void omarchy_current(const OmarchyWatcher *watcher, size_t representation, OmarchySnapshot *out);

// Fills `*out` and returns true when a new theme arrived since the last call, taken through the
// representation at `representation`; leaves `*out` untouched and returns false otherwise.
bool omarchy_poll_changed(const OmarchyWatcher *watcher, size_t representation, OmarchySnapshot *out);

// A file descriptor the caller integrates into its own event loop (e.g. QSocketNotifier),
// readable whenever a valid theme change landed. Reading from it never blocks; draining it fully
// before the next wait is the caller's job.
int omarchy_signal_fd(const OmarchyWatcher *watcher);

// Stops watching and releases `watcher`. A no-op on NULL; never call it twice on the same pointer.
void omarchy_free(OmarchyWatcher *watcher);

#ifdef __cplusplus
}
#endif

// NOLINTEND(modernize-use-using,modernize-avoid-c-arrays,cppcoreguidelines-avoid-c-arrays,cppcoreguidelines-macro-usage)
