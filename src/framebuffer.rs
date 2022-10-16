use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use std::convert::TryInto;
use std::ops::{Add, Mul};

use serde::{Serialize, Deserialize};

use png;

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
    r: f32,
    g: f32,
    b: f32
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
}

impl Add for ColorRGB {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        ColorRGB { r: self.r + other.r,
                   g: self.g + other.g,
                   b: self.b + other.b }
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


pub struct Framebuffer {
    width: u32,
    height: u32,
    data: Vec<f32>
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Framebuffer {
        assert!(width > 0);
        assert!(height > 0);

        let mut data = Vec::new();
        data.resize((width * height * 3).try_into().unwrap(), 0.0);

        Framebuffer {
            width,
            height,
            data
        }
    }

    pub fn save_image(&self, path: &Path) -> () {
        //let path = Path::new(r"image.png");
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        let srgb: Vec<u8> = self.data.iter().map(|f| (f * 255.0).round() as u8).collect();

        writer.write_image_data(&srgb).unwrap(); // Save
        println!("wrote {}", path.display());
    }

    pub fn set_color(&mut self, x: u32, y: u32, color: &ColorRGB) -> () {
        assert!(x < self.width);
        assert!(y < self.height);
        let idx: usize = ((y * self.width + x) * 3).try_into().unwrap();
        self.data[idx] = color.r;
        self.data[idx + 1] = color.g;
        self.data[idx + 2] = color.b;
    }
}
