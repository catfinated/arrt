pub mod color;
pub mod framebuffer;

mod pixel;
mod tracer;

pub use color::ColorRGB;
pub use framebuffer::Framebuffer;

use std::time::Instant;

use rayon::prelude::*;
use rayon::current_num_threads;

use crate::args::CliArgs;
use crate::scene::Scene;

use tracer::{TraceContext, TraceResult, RayTracer};
use pixel::Pixel;

/// A 2d view plane coordinate
#[derive(Debug, Copy, Clone)]
pub struct XYCoord {
    pub x: f32,
    pub y: f32,
}

pub fn render_scene(scene: Scene, anti_aliasing_depth: u8) -> Framebuffer {
    println!("bg color {:?} num threads {}", scene.bgcolor(), current_num_threads());
    let setup_start = Instant::now();
    let mut fb = Framebuffer::new(scene.width() as usize, scene.height() as usize);
    let tracer = RayTracer::new(scene);
    let setup_end = Instant::now();
    println!("setup time: {:?}", setup_end - setup_start);

    let begin = Instant::now();
    let result = fb.data.par_chunks_mut(fb.height)
        .enumerate()
        .map(|(k, row)| {
            let mut ctxt = TraceContext::new(&tracer);
            for (j, c) in row.iter_mut().enumerate() {
                let color = ctxt.sample_point(j, k);
                *c = color;
            }
            ctxt.result
        })
        .reduce(TraceResult::new,
                |a, b| a.combine(&b));

    let trace_end = Instant::now();

    // anti-aliasing
    let mut fb2 = Framebuffer::new(fb.width, fb.height);
    let result2 = fb2.data.par_chunks_mut(fb.height)
        .skip(1)
        .enumerate()
        .map(|(k, row)| {
            let mut ctxt = TraceContext::new(&tracer);
            for (j, c) in row.iter_mut().enumerate() {
                if j == fb.width - 1 { break; }
                let mut pixel = Pixel::new(j, k + 1);
                let color = pixel.sample(&mut ctxt, &fb, anti_aliasing_depth);
                *c = color;
            }
            ctxt.result
    })
    .reduce(TraceResult::new,
            |a, b| a.combine(&b));

    let render_end = Instant::now();

    result.print_stats();
    result2.print_stats();
    println!("total tracing time: {:?}", trace_end - begin);
    println!("total render time: {:?}", render_end - begin);
    fb2
}

pub fn render_with_args(args: &CliArgs) -> Framebuffer {
    let scene = Scene::new(&args.scene);
    render_scene(scene, args.sampling_depth)
}