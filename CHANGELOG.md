# Changelog

What changed for someone consuming this library, newest first.

The package version (`Cargo.toml`, the `.pc` file, the CMake package) is what a
consumer pins, and it is younger than the project: it starts at `1.0.0-beta.1`,
the first release that installed anything. Everything before that is summarized
at the end, under the project's own iteration numbers.

See `docs/BRIDGE.md` for what the SONAME promises and what a beta does not.

## 1.0.0-beta.2

**The snapshot layout changed. Recompile against this header.**

- New role `Border`: a line that marks the interface apart from its canvas — a
  divider, an outline — derived from the background and kept subtler than the
  raised surface, which is a fill and cannot serve as a line.
- `OMARCHY_ROLE_COUNT` is now 10. `Border` sits fourth in the fixed order,
  after `Recessed`, so the six roles after it shifted by one index. Read roles
  through `omarchy_role_name` rather than a hardcoded index and this costs
  nothing.
- The SONAME stays `libomarchy_theme.so.1`: within a beta the layout is still
  being shaped.
- An installed copy now carries `API.md` alongside `BRIDGE.md`. Both had gone
  missing from the install when the documents moved under `docs/`.
- The reference application builds again. It was still linking a static archive
  that `1.0.0-beta.1` had stopped producing.

## 1.0.0-beta.1

First release installable as a real artifact, for a consumer in any language
with a C FFI.

- A shared object with its own SONAME (`libomarchy_theme.so.1`), the C bridge
  header, and the license, installed to a prefix.
- Two ways to find it: `pkg-config` and a CMake package (`find_package(omarchy-theme)`).
- The C bridge got its own documentation: the boundary with the Rust v1
  contract, the full flow in C, the concurrency contract, and what each
  representation does.
- The package version separated from the project's own cycle. It starts at
  beta because the watcher and the palette had not yet been tried by an
  application outside this repository.

A static archive shipped briefly in this line and was withdrawn: it copied the
library whole into every consumer, which is the opposite of loading it once.

## Before the package existed

The library lived inside this repository, consumed only by its own reference
application. These are the iterations that shaped what the package now ships.

- **v0.1.13** — Installation, the C bridge documentation, and the package
  version. Released as `1.0.0-beta.1` above.
- **v0.1.12** — Representations became what they claim to be: `Inverted` flips
  the theme's polarity and keeps each accent as declared, rather than inverting
  colors; `Print` puts black and white on a white page, each color arriving as
  the grey its own weight gives it. Every theme color became a family of three
  shades — the color, its dark and its bright — read from the theme when
  declared and derived when not. The watcher learned that Omarchy replaces the
  theme directory rather than rewriting the file, so a change after the first
  one is noticed.
- **v0.1.11** — The reference application became a real window: stacked,
  scrolling sections drawing their entire chrome from the active theme.
- **v0.1.10** — Theme changes are announced on a file descriptor instead of
  being polled on a timer. `omarchy_signal_fd` exposes it.
- **v0.1.9** — Consumer-facing documentation moved to English.
- **v0.1.8** — `BasicPalette` became `PaletteSeed`, gaining `with_mode` so the
  light/dark mode a theme declares is carried rather than re-inferred. The
  surface roles were renamed by their real depth: `Surface` → `Elevated`,
  `SurfaceVariant` → `Recessed`, because on a light theme the lighter plane is
  the recessed one and the old names lied.
- **v0.1.7** — Internal tooling update, no consumer-visible change.
- **v0.1.6** — The first stable public contract: palettes. A seed in, an
  extended legible palette out, plus readable content on any color at all.
- **v0.1.5** — The five representations — Original, Inverted, High Contrast,
  Mono, Print — each transforming the theme before the roles resolve, so every
  guarantee recomputes rather than being duplicated.
- **v0.1.4** — The C bridge and the reference application began.
- **v0.1.3** — Invented content keeps the hue of the ground it sits on instead
  of falling to pure black or white.
- **v0.1.2** — Related colors are guaranteed to be told apart: a card shows on
  the background even in themes that declare both the same; focus stopped
  resolving to the primary color, and disabled to the muted one.
- **v0.1.1** — The active theme read as the source of truth, resolved into
  semantic roles with readable content, tonal ramps, and live following.
- **v0.1.0** — Project documentation and domain specs.
