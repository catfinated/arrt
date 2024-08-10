use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use std::time::Instant;

use png;

use rayon::prelude::*;

use super::color::ColorRGB;

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
