use image::RgbImage;
use serde::{Deserialize, Serialize};

use super::perlin::PerlinNoise;
use super::ColorRGB;
use crate::math::Vec3;

pub trait Texture: Send + Sync {
    fn color(&self, uv: Option<(f32, f32)>, point: Vec3, diffuse: ColorRGB) -> ColorRGB;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TextureConfig {
    Image { file: String },
    Checker { even: ColorRGB, odd: ColorRGB, scale: f32 },
    Marble { scale: f32, frequency: f32, amplitude: f32 },
}

impl TextureConfig {
    #[must_use]
    pub fn build(&self) -> Box<dyn Texture> {
        match self {
            Self::Image { file } => Box::new(ImageTexture::new(file)),
            Self::Checker { even, odd, scale } => Box::new(CheckerTexture {
                even: *even,
                odd: *odd,
                scale: *scale,
            }),
            Self::Marble { scale, frequency, amplitude } => Box::new(MarbleTexture {
                perlin: PerlinNoise::new(),
                scale: *scale,
                frequency: *frequency,
                amplitude: *amplitude,
            }),
        }
    }
}

struct ImageTexture {
    image: RgbImage,
}

impl ImageTexture {
    fn new(path: &str) -> Self {
        let image = image::open(path)
            .unwrap_or_else(|e| panic!("failed to open texture {path}: {e}"))
            .to_rgb8();
        ImageTexture { image }
    }
}

impl Texture for ImageTexture {
    #[allow(
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        clippy::cast_precision_loss,
        clippy::many_single_char_names
    )]
    fn color(&self, uv: Option<(f32, f32)>, _point: Vec3, _diffuse: ColorRGB) -> ColorRGB {
        let (u, v) = uv.unwrap_or((0.0, 0.0));
        let u = u.rem_euclid(1.0);
        let v = v.rem_euclid(1.0);
        let w = self.image.width();
        let h = self.image.height();
        let x = (u * (w - 1) as f32) as u32;
        let y = ((1.0 - v) * (h - 1) as f32) as u32; // flip: UV origin is bottom-left
        let pixel = self.image.get_pixel(x, y);
        ColorRGB::new(
            f32::from(pixel[0]) / 255.0,
            f32::from(pixel[1]) / 255.0,
            f32::from(pixel[2]) / 255.0,
        )
    }
}

struct CheckerTexture {
    even: ColorRGB,
    odd: ColorRGB,
    scale: f32,
}

impl Texture for CheckerTexture {
    fn color(&self, uv: Option<(f32, f32)>, point: Vec3, _diffuse: ColorRGB) -> ColorRGB {
        let (u, v) = uv.unwrap_or_else(|| (point.x(), point.z()));
        let sines = (u * self.scale).floor() + (v * self.scale).floor();
        if sines.rem_euclid(2.0) < 1.0 {
            self.even
        } else {
            self.odd
        }
    }
}

struct MarbleTexture {
    perlin: PerlinNoise,
    scale: f32,
    /// Frequency of the sine wave along the x-axis.
    frequency: f32,
    /// Turbulence amplitude — controls how much the veins are distorted.
    amplitude: f32,
}

impl Texture for MarbleTexture {
    fn color(&self, _uv: Option<(f32, f32)>, point: Vec3, diffuse: ColorRGB) -> ColorRGB {
        // Marble formula from Perlin / Suffern:
        //   n = sin(frequency * (x * scale + amplitude * turbulence(p)))
        // Normalized from [-1,1] to [0,1] and applied as a multiplier on the diffuse color.
        let n = (self.frequency
            * (point.x() * self.scale + self.amplitude * self.perlin.turbulence(point)))
            .sin();
        diffuse * ((n + 1.0) * 0.5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vec3;

    fn checker() -> CheckerTexture {
        CheckerTexture {
            even: ColorRGB::white(),
            odd: ColorRGB::black(),
            scale: 2.0,
        }
    }

    fn white() -> ColorRGB {
        ColorRGB::white()
    }

    #[test]
    fn checker_even_at_origin() {
        let c = checker().color(Some((0.0, 0.0)), Vec3::zeros(), white());
        assert!((c.r - 1.0).abs() < 1e-5);
        assert!((c.g - 1.0).abs() < 1e-5);
        assert!((c.b - 1.0).abs() < 1e-5);
    }

    #[test]
    fn checker_odd_in_second_cell() {
        // scale=2: cells are 0.5 wide in UV. u=0.6 => floor(1.2)=1, v=0.1 => floor(0.2)=0, sum=1 -> odd
        let c = checker().color(Some((0.6, 0.1)), Vec3::zeros(), white());
        assert!((c.r - 0.0).abs() < 1e-5);
    }

    #[test]
    fn checker_even_at_diagonal_cell() {
        // u=0.6, v=0.6: floor(1.2)+floor(1.2) = 2 -> even
        let c = checker().color(Some((0.6, 0.6)), Vec3::zeros(), white());
        assert!((c.r - 1.0).abs() < 1e-5);
    }

    #[test]
    fn checker_falls_back_to_world_space() {
        // uv=None uses point.x() and point.z()
        let c = checker().color(None, Vec3::new(0.0, 0.0, 0.0), white());
        assert!((c.r - 1.0).abs() < 1e-5);
    }

    #[test]
    fn checker_world_space_odd() {
        // x=0.6, z=0.1: floor(1.2)+floor(0.2) = 1 -> odd
        let c = checker().color(None, Vec3::new(0.6, 0.0, 0.1), white());
        assert!((c.r - 0.0).abs() < 1e-5);
    }

    #[test]
    fn marble_output_in_range() {
        // Result must be in [0, diffuse] — each channel in [0, diffuse_channel].
        let tex = MarbleTexture {
            perlin: PerlinNoise::new(),
            scale: 1.0,
            frequency: 1.0,
            amplitude: 50.0,
        };
        let diffuse = ColorRGB::new(0.076, 0.614, 0.076); // emerald-like
        for i in 0..20 {
            #[allow(clippy::cast_precision_loss)]
            let p = Vec3::new(i as f32 * 0.37, i as f32 * 0.61, i as f32 * 0.19);
            let c = tex.color(None, p, diffuse);
            assert!(c.r >= 0.0 && c.r <= diffuse.r + 1e-5);
            assert!(c.g >= 0.0 && c.g <= diffuse.g + 1e-5);
            assert!(c.b >= 0.0 && c.b <= diffuse.b + 1e-5);
        }
    }

    #[test]
    fn marble_varies_with_position() {
        let tex = MarbleTexture {
            perlin: PerlinNoise::new(),
            scale: 1.0,
            frequency: 1.0,
            amplitude: 50.0,
        };
        let diffuse = ColorRGB::white();
        let c1 = tex.color(None, Vec3::new(0.3, 0.5, 0.1), diffuse);
        let c2 = tex.color(None, Vec3::new(1.7, 0.2, 0.9), diffuse);
        assert!((c1.r - c2.r).abs() > 1e-3);
    }
}
