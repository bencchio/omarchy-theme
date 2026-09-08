# Omarchy Theming

The theming library for **Omarchy** applications. It takes the active theme as
the source of truth and hands an application everything it needs to look like
it belongs: the colors as the roles an interface plays, the same theme through
five representations, and the change notification that keeps all of it live.

An application that wants to fit into Omarchy does not build its own theming
system, generate shades, work out readable combinations or watch the theme for
changes. It gets all of that from one coherent source.

**The principle:** the Omarchy theme defines the visual identity; the library
expands it and makes it consumable by applications.

Install it and link it as `omarchy-theme` — the shorter name it carries in
`pkg-config`, in CMake and in the shared object.

## Which contract do I need?

The library ships two separate contracts. They are not layers of one another —
pick by what the application has to do.

| If the application needs to… | Use | Documented in |
| --- | --- | --- |
| Resolve a palette once, from Rust — roles, readable content, tonal ramps | The v1 Rust API | [`docs/API.md`](docs/API.md) |
| Follow the active theme as it changes, or read it through more than one representation, from any language with a C FFI | The C bridge | [`docs/BRIDGE.md`](docs/BRIDGE.md) |

A palette is a snapshot, which is why following the theme and the alternate
representations live on the bridge instead: v1 is palettes only.

## Installing

### Requirements

- Rust with `cargo` — the library itself is Rust, and CMake invokes it.
- CMake 3.20 or newer.
- Qt 6 with the Quick module — only to build the reference application.

### On Arch Linux

`packaging/arch/PKGBUILD` builds and installs the library with `makepkg`:

```sh
cd packaging/arch
makepkg -si
```

### Anywhere else

`install.sh` builds and installs the library without going through a
distribution package — the quick way to try a build before packaging it, or
on a system with no AUR:

```sh
./install.sh --prefix "$HOME/.local"
```

### Prebuilt binaries

Each release attaches the shared object and the C bridge header, built from
that tag, for `x86_64` — no toolchain needed to link against them:

<https://github.com/bencchio/omarchy-theme/releases>

They carry no `pkg-config` or CMake package, so a build consuming them points
at the two files directly.

### Build and install manually

For full control over the build, run the same steps `install.sh` wraps:

```sh
cmake -S . -B build -DCMAKE_INSTALL_PREFIX="$HOME/.local"
cmake --build build
cmake --install build
```

That installs the shared object under its full version with the two symlinks a
consumer expects, the C bridge header, the `pkg-config` file, the CMake
package, the license, and both contract documents.

The package version is `1.0.0-beta.N` and the SONAME is
`libomarchy_theme.so.1`. Within a beta the snapshot layout may change without
the SONAME moving, so pin the package version and recompile against the header
of the beta you link — see `docs/BRIDGE.md` § Versioning.

### Use it from an application

With `pkg-config`:

```sh
pkg-config --cflags --libs omarchy-theme
```

With CMake:

```cmake
find_package(omarchy-theme REQUIRED)
target_link_libraries(my-app PRIVATE omarchy-theme::shared)
```

If the prefix is not one the tools already search, point them at it —
`PKG_CONFIG_PATH` for the first, `CMAKE_PREFIX_PATH` for the second.

### Run the reference application

The fastest way to see the system working before integrating it. This one
builds against the repository rather than the installed package.

```sh
cmake -S gui -B gui/build
cmake --build gui/build
./gui/build/omarchy-theme-showcase
```

It opens a real interface — styles, typography, roles and the declared palette
— that follows the active theme and redraws when it changes. If an older copy
of the library is installed somewhere on `LD_LIBRARY_PATH`, that copy wins over
the one just built; run it with `env -u LD_LIBRARY_PATH` to be sure of which
one is loaded.

## What it includes

- **Semantic colors** — the interface recommended for applications. The app
  states what part a color plays (`background`, `elevated`, `recessed`,
  `primary`, …) instead of knowing how it was produced.
- **Tonal ramps** — the tonal variants of the base colors, for cases that need
  finer control.
- **Representations** — the same theme readable in several ways: Original,
  Inverted, High Contrast, Mono and Print. They express visual intent, not
  arithmetic on RGB.
- **Live updates** — when the Omarchy theme changes, applications receive the
  change and redraw without watching files, without polling and without
  restarting.
- **A reference application** — a real interface that demonstrates the whole
  system and serves as an integration example.

## Stack

- **Library:** Rust.
- **Reference application:** C++.

## Outside this scope

Advanced accessibility — explicit contrast-level compliance, modes for the
different color vision deficiencies, automatic validation of combinations — is
later work. The library does avoid producing obviously problematic color
combinations.
