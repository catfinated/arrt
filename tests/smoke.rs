use std::path::PathBuf;

use arrt::args::CliArgs;
use arrt::render::render_with_args;

fn smoke_framebuffer() -> arrt::render::Framebuffer {
    let args = CliArgs {
        scene: PathBuf::from("scenes/smoke_test.yaml"),
        image: None,
        sampling_depth: 0,
    };
    render_with_args(&args)
}

#[test]
fn dimensions_match_scene() {
    let fb = smoke_framebuffer();
    assert_eq!(fb.width, 64);
    assert_eq!(fb.height, 64);
}

#[test]
fn corner_is_background_color() {
    let fb = smoke_framebuffer();
    // smoke_test.yaml bgcolor is pure blue; corners miss the sphere
    let c = fb.get_color(1, 1);
    assert!(c.b > 0.9, "expected blue background at corner, got r={} g={} b={}", c.r, c.g, c.b);
    assert!(c.r < 0.1, "expected no red at corner, got r={}", c.r);
}

#[test]
fn center_hits_sphere() {
    let fb = smoke_framebuffer();
    // ruby sphere is centered in the image; center pixel should be reddish, not blue
    let c = fb.get_color(32, 32);
    assert!(
        c.r > c.b,
        "expected sphere (ruby=red) at center, got r={} g={} b={}",
        c.r,
        c.g,
        c.b
    );
}
