//! Pulling apart the roles a theme backs with a single color.

use crate::theme::Rgb;

use super::ramp::shifted;
use super::tone::tone;

/// Focus and primary both come from the theme's accent, and a keyboard-driven interface is unusable
/// while they look the same; disabled and muted collide the same way.
const FROM_PRIMARY: f64 = 12.0;
const FROM_MUTED: f64 = 10.0;
const FROM_BACKGROUND: f64 = 8.0;

/// The accent, moved away from the background so a focused element outshines a primary one.
pub(super) fn focus(accent: Rgb, background: Rgb) -> Rgb {
    let accent_tone = tone(accent);
    let outward = if accent_tone >= tone(background) {
        accent_tone + FROM_PRIMARY
    } else {
        accent_tone - FROM_PRIMARY
    };

    if in_scale(outward) {
        return shifted(accent, outward);
    }

    shifted(accent, mirrored(accent_tone, outward))
}

/// The muted color, moved toward the background so a disabled element reads as switched off —
/// unless the two sit too close to fit anything between them, where it steps back the other way.
pub(super) fn disabled(muted: Rgb, background: Rgb) -> Rgb {
    let muted_tone = tone(muted);
    let background_tone = tone(background);
    let inward = if muted_tone > background_tone {
        muted_tone - FROM_MUTED
    } else {
        muted_tone + FROM_MUTED
    };

    if in_scale(inward) && (inward - background_tone).abs() >= FROM_BACKGROUND {
        return shifted(muted, inward);
    }

    shifted(muted, mirrored(muted_tone, inward))
}

fn mirrored(origin: f64, rejected: f64) -> f64 {
    origin - (rejected - origin)
}

fn in_scale(tone: f64) -> bool {
    (0.0..=100.0).contains(&tone)
}
