//! Module for converting a color space and color components to a `Color`.

use std::{convert::TryFrom, error::Error, fmt};

use super::space::*;
use super::{Color, ColorSpace};

/// Error caused by parsing a number in a certain color space.
///
/// This error can occur if the wrong number of color components
/// was supplied (e.g. `rgb` with only 2 components), or if a
/// color component is out of range (for example, `rgb` requires
/// that all components are in 0..=255).
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ParseError {
    NumberOfComponents {
        expected: usize,
        got: usize,
    },
    Negative {
        component: &'static str,
        got: f64,
    },
    OutOfRange {
        component: &'static str,
        min: f64,
        max: f64,
        got: f64,
    },
}

impl Error for ParseError {}

// Note that this could be simplified with `thiserror`, but I'm currently
// reluctant to add more dependencies
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ParseError::NumberOfComponents { expected, got } => write!(
                f,
                "Wrong number of color components (expected {}, got {})",
                expected, got
            ),
            ParseError::Negative { component, got } => write!(
                f,
                "Color component {:?} can't be negative (got {})",
                component, got
            ),
            ParseError::OutOfRange {
                component,
                min,
                max,
                got,
            } => write!(
                f,
                "Color component {:?} out of range (expected {} to {}, got {})",
                component, min, max, got
            ),
        }
    }
}

impl TryFrom<(ColorSpace, &[f64])> for Color {
    type Error = ParseError;

    fn try_from((space, vals): (ColorSpace, &[f64])) -> Result<Self, Self::Error> {
        let required_args = if space == ColorSpace::Cmyk { 4 } else { 3 };

        if vals.len() != required_args {
            return Err(ParseError::NumberOfComponents {
                expected: required_args,
                got: vals.len(),
            });
        }

        // Create the color and check if the values are in the valid range
        match space {
            ColorSpace::Rgb => Color::try_from(Rgb::new(vals[0], vals[1], vals[2])),
            ColorSpace::Cmy => Color::try_from(Cmy::new(vals[0], vals[1], vals[2])),
            ColorSpace::Cmyk => Color::try_from(Cmyk::new(vals[0], vals[1], vals[2], vals[3])),
            ColorSpace::Hsv => Color::try_from(Hsv::new(vals[0], vals[1], vals[2])),
            ColorSpace::Hsl => Color::try_from(Hsl::new(vals[0], vals[1], vals[2])),
            ColorSpace::Lch => Color::try_from(Lch::new(vals[0], vals[1], vals[2])),
            ColorSpace::Luv => Color::try_from(Luv::new(vals[0], vals[1], vals[2])),
            ColorSpace::Lab => Color::try_from(Lab::new(vals[0], vals[1], vals[2])),
            ColorSpace::HunterLab => Color::try_from(HunterLab::new(vals[0], vals[1], vals[2])),
            ColorSpace::Xyz => Color::try_from(Xyz::new(vals[0], vals[1], vals[2])),
            ColorSpace::Yxy => Color::try_from(Yxy::new(vals[0], vals[1], vals[2])),
        }
    }
}

/// Implements `TryFrom<$ty>` for `Color`. The conversion fails if any
/// color component isn't in the valid range.
macro_rules! try_from_color {
    ($ty:ident -> $( $component:ident : $min:literal to $max:literal );* $(;)?) => {
        impl TryFrom<$ty> for Color {
            type Error = ParseError;

            fn try_from(value: $ty) -> Result<Self, Self::Error> {
                $(
                    min_max(stringify!($component), $min, $max, value.$component)?;
                )*
                Ok(Self::$ty(value))
            }
        }
    };
}

try_from_color! { Rgb ->
    r: 0.0 to 255.0;
    g: 0.0 to 255.0;
    b: 0.0 to 255.0;
}
try_from_color! { Cmy ->
    c: 0.0 to 1.0;
    m: 0.0 to 1.0;
    y: 0.0 to 1.0;
}
try_from_color! { Cmyk ->
    c: 0.0 to 1.0;
    m: 0.0 to 1.0;
    y: 0.0 to 1.0;
    k: 0.0 to 1.0;
}
try_from_color! { Hsv ->
    h: 0.0 to 360.0;
    s: 0.0 to 1.0;
    v: 0.0 to 1.0;
}
try_from_color! { Hsl ->
    h: 0.0 to 360.0;
    s: 0.0 to 1.0;
    l: 0.0 to 1.0;
}
try_from_color! { Lch ->
    l: 0.0 to 100.0;
    c: 0.0 to 100.0;
    h: 0.0 to 360.0;
}
try_from_color! { Luv ->
    l: 0.0 to 100.0;
    u: -134.0 to 220.0;
    v: -140.0 to 122.0;
}
try_from_color! { Lab ->
    l: 0.0 to 100.0;
}
try_from_color! { HunterLab ->
    l: 0.0 to 100.0;
}
try_from_color! { Xyz -> }
try_from_color! { Yxy -> }

/// Checks that the value is in the specified range. If it isn't, an error is
/// returned.
fn min_max(component: &'static str, min: f64, max: f64, got: f64) -> Result<(), ParseError> {
    if got < min || got > max {
        if min == 0.0 && got < min {
            Err(ParseError::Negative { component, got })
        } else {
            Err(ParseError::OutOfRange {
                component,
                min,
                max,
                got,
            })
        }
    } else {
        Ok(())
    }
}
