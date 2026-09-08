# API — Color palettes

This is one of the two contracts the library ships: palettes resolved once,
from Rust. To follow the active theme as it changes, or to read it through more
than one representation, reach for the C bridge in `BRIDGE.md` instead.

The stable interface of **omarchy-theme**: an application receives a **basic**
palette —taken from the active Omarchy theme or built by itself— and the
library turns it into the **extended legible palette**: the semantic roles,
the content that reads on each one and the tonal ramps to scale from.

The library builds no theme and picks no colors on the application's behalf:
it only completes readable defaults when the basic palette leaves them blank.

## Usage flow

```rust
use omarchy_theme::{PaletteSeed, Role};

let seed = PaletteSeed::from_active_theme()?;   // or from_path, or new() + with_*
let palette = seed.extend();

let canvas = palette.color(Role::Background);   // the canvas
let body   = palette.on(Role::Background);      // main text on the canvas
```

## PaletteSeed

The starting point. `background`, `foreground` and `accent` are required; the
rest is optional and the library derives a readable value during `extend()`.
The fields are private: it is built only through constructors and builders.

```rust
impl PaletteSeed {
    // Loading — pulls from the Omarchy theme, with its mode.
    pub fn from_active_theme() -> Result<Self, ThemeError>;
    pub fn from_path(path: &Path) -> Result<Self, ThemeError>;

    // Built by the app itself, from the three colors that belong to it.
    pub fn new(background: Rgb, foreground: Rgb, accent: Rgb) -> Self;

    // Builders (all optional; when left unstated, the library derives a readable default).
    pub fn with_selection(self, color: Rgb) -> Self;
    pub fn with_muted(self, color: Rgb) -> Self;
    pub fn with_error(self, color: Rgb) -> Self;
    pub fn with_surface_light(self, color: Rgb) -> Self;   // the raised surface
    pub fn with_surface_dark(self, color: Rgb) -> Self;    // the recessed surface
    pub fn with_mode(self, mode: Mode) -> Self;            // instead of inferring from the tone

    // Transformation — produces the extended palette.
    pub fn extend(&self) -> Palette;
}
```

- `from_path`/`from_active_theme` carry the `mode` the theme declares; an app
  building its own seed can set it with `with_mode`, and in its absence the
  library infers it from the `background` tone.

## Palette

What the application draws with: every color already guarantees its content
stays readable on it.

```rust
impl Palette {
    pub fn color(&self, role: Role) -> Rgb;            // the role's concrete color
    pub fn on(&self, role: Role) -> Rgb;               // the main content readable on the role
    pub fn content(&self, role: Role, level: Level) -> Rgb; // the content of a given weight
    pub fn ramp(&self, role: Role) -> Ramp;            // the role's 0–100 tonal ramp
    pub fn dark(&self) -> bool;                        // whether it sits on a dark ground
    pub fn on_color(&self, color: Rgb) -> Rgb;         // readable content on any color
}
```

## Role

The ten roles an interface expresses, instead of a theme key by name. The last
column is one concrete thing each role draws, to save working the mapping out
from the abstract description.

| Role | What it represents | Typical use |
| --- | --- | --- |
| `Background` | The canvas the interface sits on. | The window behind everything; the page a document is laid on. |
| `Elevated` | A surface raised off the canvas: a card, a panel, a popover. | The ground of a side panel, a toolbar, a dropdown menu. |
| `Recessed` | A recessed surface: an input field, a well. | The inside of a search field; the track of a scrollbar. |
| `Border` | A line marking the interface apart from its canvas: a border, a divider. | The line between a toolbar and the document; the outline of a card. |
| `Primary` | The color of the theme's identity, for emphasis. | A confirm button; an active tab's underline; a link. |
| `Selection` | The ground behind selected content. | Highlighted text; the current row of a list. |
| `Muted` | De-emphasized content, such as secondary labels. | A timestamp under a title; a placeholder in an empty field. |
| `Error` | Something went wrong. | The message under a field that failed to validate. |
| `Focus` | The element holding keyboard focus. | The ring around whatever the keyboard is on. |
| `Disabled` | An element that cannot be interacted with. | A button that cannot be pressed yet; a menu entry ruled out. |

`Role::ALL` iterates the ten in fixed order.

## Level

The weight of a piece of content on its ground.

| Level | Use |
| --- | --- |
| `Primary` | Body text and anything meant to be acted on. |
| `Secondary` | Supporting text, such as a caption. |
| `Disabled` | Content that cannot be interacted with. |

## Ramp

A color's tonal scale, from 0 (black) to 100 (white), keeping its hue.
`Ramp::of(color)` gives the ramp of any color; `Ramp::tone(tone)` gives the
color at a step (values above 100 are clamped).

```rust
let hover = palette.ramp(Role::Primary).tone(80);
```

## Mode

The light/dark mode the theme declares.

```rust
pub enum Mode { Dark, Light }
```

## Not part of the v1 contract

Theme-change subscription (`ThemeWatcher`) and the representations
(`Representation`) stay accessible from Rust, but outside v1 — v1 is palettes
only. Applications that need to follow the active theme or see it through
more than one representation reach for the C ABI instead, documented in
`BRIDGE.md` as its own contract.
