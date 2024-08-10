use super::{ColorRGB, TraceContext, Framebuffer, XYCoord};

/// A super pixel capable of doing
/// 0, 1, or 2 levels of adaptive supersampling
#[derive(Debug)]
pub struct Pixel {
    j: usize,
    k: usize,
    stash: [[Option<ColorRGB>; 5]; 5]
}

impl Pixel {
    pub fn new(j: usize, k: usize) -> Self {
        Pixel { j, k, stash: [[None; 5]; 5]}
    }

    pub fn sample(&mut self, tracer: &mut TraceContext, framebuf: &Framebuffer, max_depth: u8) -> ColorRGB {
        self.stash[4][0] = Some(framebuf.get_color(self.j, self.k));
        self.stash[4][4] = Some(framebuf.get_color(self.j + 1, self.k));
        self.stash[0][4] = Some(framebuf.get_color(self.j + 1, self.k - 1));
        self.stash[0][0] = Some(framebuf.get_color(self.j, self.k - 1));
        let bottom_left = XYCoord{ x: self.j as f32, y: self.k as f32 };

        match max_depth {
            0 => self.subdivide(bottom_left, (4, 0), 4, tracer, 4),
            1 => self.subdivide(bottom_left, (4, 0), 4, tracer, 2),
            _ => self.subdivide(bottom_left,(4, 0), 4, tracer, 1),
        }
    }

    fn subdivide(&mut self, coord: XYCoord, bottom_left: (usize, usize), depth: usize, tracer: &mut TraceContext, min_depth: usize) -> ColorRGB {
        let off = depth;
        let adjust = depth as f32 / 4.0;

        let a = *self.stash[bottom_left.0][bottom_left.1]
            .get_or_insert_with(|| tracer.sample_coord(coord));

        let b = *self.stash[bottom_left.0][bottom_left.1 + off]
            .get_or_insert_with(|| tracer.sample_coord(XYCoord{x: coord.x + adjust, y: coord.y}));

        let e = *self.stash[bottom_left.0 - off][bottom_left.1 + off]
            .get_or_insert_with(|| tracer.sample_coord(XYCoord{x: coord.x + adjust, y: coord.y - adjust}));

        let d = *self.stash[bottom_left.0 - off][bottom_left.1]
            .get_or_insert_with(|| tracer.sample_coord(XYCoord{x: coord.x, y: coord.y - adjust}));

        let samples = [a, b, e, d];

        if !samples_differ(&samples) || depth == min_depth {
            average_color(&samples)
        } else {
            let d = depth / 2;
            let off = d;
            let adjust = depth as f32 / 4.0;

            let a = self.subdivide(coord, bottom_left, d, tracer, min_depth);

            let b = self.subdivide(XYCoord{x: coord.x + adjust, y: coord.y}, (bottom_left.0, bottom_left.1 + off), d, tracer, min_depth);

            let e = self.subdivide(XYCoord{x: coord.x + adjust, y: coord.y - adjust}, (bottom_left.0 - off, bottom_left.1 + off), d, tracer, min_depth);

            let d = self.subdivide(XYCoord{x: coord.x, y: coord.y - adjust}, (bottom_left.0 - off, bottom_left.1), d, tracer, min_depth);

            let samples = [a, b, e, d];
            average_color(&samples)
        }
    }
}

fn average_color(samples: &[ColorRGB; 4]) -> ColorRGB {

    let mut sum = ColorRGB{ r: 0.0, g: 0.0, b: 0.0 };
    for &sample in samples {
        sum += sample;
    }
    sum / 4.0
}

fn samples_differ(samples: &[ColorRGB; 4]) -> bool {
    colors_differ(&samples[0], &samples[1]) ||
        colors_differ(&samples[0], &samples[3]) ||
        colors_differ(&samples[2], &samples[3]) ||
        colors_differ(&samples[2], &samples[1])
}

fn colors_differ(a: &ColorRGB, b: &ColorRGB) -> bool {
    let diff = *a - *b;
    const TOLERANCE: f32 = 0.05;
    diff.r.abs() > TOLERANCE || diff.g.abs() > TOLERANCE || diff.b.abs() > TOLERANCE

}