use image::RgbImage;
use serde::{Deserialize, Serialize};

use super::ColorRGB;
use crate::math::Vec3;

pub trait Texture: Send + Sync {
    fn color(&self, uv: Option<(f32, f32)>, point: Vec3) -> ColorRGB;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TextureConfig {
    Image { file: String },
    Checker { even: ColorRGB, odd: ColorRGB, scale: f32 },
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
    fn color(&self, uv: Option<(f32, f32)>, _point: Vec3) -> ColorRGB {
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
    fn color(&self, uv: Option<(f32, f32)>, point: Vec3) -> ColorRGB {
        let (u, v) = uv.unwrap_or_else(|| (point.x(), point.z()));
        let sines = (u * self.scale).floor() + (v * self.scale).floor();
        if sines.rem_euclid(2.0) < 1.0 {
            self.even
        } else {
            self.odd
        }
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

    #[test]
    fn checker_even_at_origin() {
        let c = checker().color(Some((0.0, 0.0)), Vec3::zeros());
        assert!((c.r - 1.0).abs() < 1e-5);
        assert!((c.g - 1.0).abs() < 1e-5);
        assert!((c.b - 1.0).abs() < 1e-5);
    }

    #[test]
    fn checker_odd_in_second_cell() {
        // scale=2: cells are 0.5 wide in UV. u=0.6 => floor(1.2)=1, v=0.1 => floor(0.2)=0, sum=1 -> odd
        let c = checker().color(Some((0.6, 0.1)), Vec3::zeros());
        assert!((c.r - 0.0).abs() < 1e-5);
    }

    #[test]
    fn checker_even_at_diagonal_cell() {
        // u=0.6, v=0.6: floor(1.2)+floor(1.2) = 2 -> even
        let c = checker().color(Some((0.6, 0.6)), Vec3::zeros());
        assert!((c.r - 1.0).abs() < 1e-5);
    }

    #[test]
    fn checker_falls_back_to_world_space() {
        // uv=None uses point.x() and point.z()
        let c = checker().color(None, Vec3::new(0.0, 0.0, 0.0));
        assert!((c.r - 1.0).abs() < 1e-5);
    }

    #[test]
    fn checker_world_space_odd() {
        // x=0.6, z=0.1: floor(1.2)+floor(0.2) = 1 -> odd
        let c = checker().color(None, Vec3::new(0.6, 0.0, 0.1));
        assert!((c.r - 0.0).abs() < 1e-5);
    }
}
