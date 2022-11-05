use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use std::ops::{Add, Mul, Sub, Div, AddAssign};
use std::time::Instant;

use serde::{Serialize, Deserialize};

use png;

use rayon::prelude::*;

fn clamp(val: f32, lo: f32, hi: f32) -> f32 {
    if val < lo {
        lo
    }
    else if val > hi {
        hi
    }
    else {
        val
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ColorRGB {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl ColorRGB {
    pub fn new(r: f32, g: f32, b: f32) -> ColorRGB {
        ColorRGB{ r, g, b }
    }

    pub fn fill(val: f32) -> ColorRGB {
        ColorRGB::new(val, val, val )
    }

    pub fn black() -> ColorRGB {
        ColorRGB::fill(0.0_f32)
    }

    pub fn white() -> ColorRGB {
        ColorRGB::fill(1.0_f32)
    }

    pub fn clamp(&self, lo: f32, hi: f32) -> ColorRGB {
        ColorRGB{ r: clamp(self.r, lo, hi),
                  g: clamp(self.g, lo, hi),
                  b: clamp(self.b, lo, hi) }
    }

    pub fn to_irgb(&self) -> [u8; 3] {
        [(self.r * 255.0).round() as u8,
         (self.g * 255.0).round() as u8,
         (self.b * 255.0).round() as u8,]
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
        ColorRGB { r: self.r + other.r,
                   g: self.g + other.g,
                   b: self.b + other.b }
    }
}

impl Sub for ColorRGB {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        ColorRGB { r: self.r - other.r,
                   g: self.g - other.g,
                   b: self.b - other.b }
    }
}

impl Mul for ColorRGB {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        ColorRGB { r: self.r * other.r,
                   g: self.g * other.g,
                   b: self.b * other.b }
    }
}

impl Mul<f32> for ColorRGB {
    type Output = Self;

    fn mul(self, f: f32) -> Self {
        ColorRGB { r: self.r * f,
                   g: self.g * f,
                   b: self.b * f }
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
        ColorRGB { r: self.r / f,
                   g: self.g / f,
                   b: self.b / f }
    }
}

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub data: Vec<ColorRGB>
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Framebuffer {
        assert!(width > 0 && width < u32::MAX as usize);
        assert!(height > 0 && height < u32::MAX as usize);

        let mut data = Vec::new();
        data.resize(width * height, ColorRGB{r: 0.0, g: 0.0, b: 0.0});

        Framebuffer {
            width,
            height,
            data
        }
    }

    pub fn save_image(&self, path: &Path) {
        let start = Instant::now();
        let file = File::create(path).unwrap();
        let bufwriter = &mut BufWriter::new(file);

        let mut encoder = png::Encoder::new(bufwriter, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        let srgb: Vec<u8> = self.data.par_iter().flat_map(|c| c.to_irgb()).collect();
        writer.write_image_data(&srgb).unwrap();
        let stop = Instant::now();
        println!("wrote {} in {:?}", path.display(), stop - start);
    }

    pub fn set_color(&mut self, x: usize, y: usize, color: &ColorRGB) {
        assert!(x < self.width, "{} {}", x, self.width);
        assert!(y < self.height, "{} {}", y, self.height);
        let idx: usize = (y * self.width) + x;
        self.data[idx] = *color;
    }

    pub fn get_color(&self, x: usize, y: usize) -> ColorRGB {
        assert!(x < self.width, "{} {}", x, self.width);
        assert!(y < self.height, "{} {}", y, self.height);
        let idx: usize = (y * self.width) + x;
         self.data[idx]
    }
}
