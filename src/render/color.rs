use std::ops::{Add, AddAssign, Div, Mul, Sub};

use serde::{Deserialize, Serialize};

fn clamp(val: f32, lo: f32, hi: f32) -> f32 {
    if val < lo {
        lo
    } else if val > hi {
        hi
    } else {
        val
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ColorRGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl ColorRGB {
    #[must_use]
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        ColorRGB { r, g, b }
    }

    #[must_use]
    pub fn fill(val: f32) -> Self {
        ColorRGB::new(val, val, val)
    }

    #[must_use]
    pub fn black() -> Self {
        ColorRGB::fill(0.0_f32)
    }

    #[must_use]
    pub fn white() -> Self {
        ColorRGB::fill(1.0_f32)
    }

    #[must_use]
    pub fn red() -> Self {
        ColorRGB::new(1.0_f32, 0.0_f32, 0.0_f32)
    }

    #[must_use]
    pub fn green() -> Self {
        ColorRGB::new(0.0_f32, 1.0_f32, 0.0_f32)
    }

    #[must_use]
    pub fn blue() -> Self {
        ColorRGB::new(0.0_f32, 0.0_f32, 1.0_f32)
    }

    #[must_use]
    pub fn clamp(&self, lo: f32, hi: f32) -> Self {
        ColorRGB {
            r: clamp(self.r, lo, hi),
            g: clamp(self.g, lo, hi),
            b: clamp(self.b, lo, hi),
        }
    }

    // Colors are clamped to [0,1] before calling this; truncation and sign loss are expected.
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn to_irgb(&self) -> [u8; 3] {
        [
            (self.r * 255.0).round() as u8,
            (self.g * 255.0).round() as u8,
            (self.b * 255.0).round() as u8,
        ]
    }
}

impl AddAssign for ColorRGB {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        };
    }
}

impl Add for ColorRGB {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        ColorRGB {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Sub for ColorRGB {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        ColorRGB {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl Mul for ColorRGB {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        ColorRGB {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl Mul<f32> for ColorRGB {
    type Output = Self;

    fn mul(self, f: f32) -> Self {
        ColorRGB {
            r: self.r * f,
            g: self.g * f,
            b: self.b * f,
        }
    }
}

impl Mul<ColorRGB> for f32 {
    type Output = ColorRGB;

    fn mul(self, c: ColorRGB) -> ColorRGB {
        c * self
    }
}

impl Div<f32> for ColorRGB {
    type Output = Self;

    fn div(self, f: f32) -> ColorRGB {
        ColorRGB {
            r: self.r / f,
            g: self.g / f,
            b: self.b / f,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black_to_irgb() {
        assert_eq!(ColorRGB::black().to_irgb(), [0, 0, 0]);
    }

    #[test]
    fn white_to_irgb() {
        assert_eq!(ColorRGB::white().to_irgb(), [255, 255, 255]);
    }

    #[test]
    fn red_to_irgb() {
        assert_eq!(ColorRGB::red().to_irgb(), [255, 0, 0]);
    }

    #[test]
    fn clamp_above_one() {
        let c = ColorRGB::new(2.0, 0.5, -0.5).clamp(0.0, 1.0);
        assert!((c.r - 1.0).abs() < 1e-5);
        assert!((c.g - 0.5).abs() < 1e-5);
        assert!((c.b - 0.0).abs() < 1e-5);
    }

    #[test]
    fn fill_sets_all_channels() {
        let c = ColorRGB::fill(0.4);
        assert!((c.r - 0.4).abs() < 1e-5);
        assert!((c.g - 0.4).abs() < 1e-5);
        assert!((c.b - 0.4).abs() < 1e-5);
    }

    #[test]
    fn add_colors() {
        let c = ColorRGB::new(0.2, 0.3, 0.4) + ColorRGB::new(0.1, 0.2, 0.1);
        assert!((c.r - 0.3).abs() < 1e-5);
        assert!((c.g - 0.5).abs() < 1e-5);
        assert!((c.b - 0.5).abs() < 1e-5);
    }

    #[test]
    fn scalar_mul() {
        let c = ColorRGB::new(0.5, 0.25, 0.1) * 2.0;
        assert!((c.r - 1.0).abs() < 1e-5);
        assert!((c.g - 0.5).abs() < 1e-5);
        assert!((c.b - 0.2).abs() < 1e-5);
    }

    #[test]
    fn component_mul() {
        let c = ColorRGB::new(0.5, 0.5, 0.5) * ColorRGB::new(0.5, 1.0, 0.0);
        assert!((c.r - 0.25).abs() < 1e-5);
        assert!((c.g - 0.5).abs() < 1e-5);
        assert!((c.b - 0.0).abs() < 1e-5);
    }
}
