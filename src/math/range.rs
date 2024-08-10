#[derive(Copy, Clone, Debug)]
pub struct Range {
    pub min: f32,
    pub max: f32
}

pub fn in_range(range: Range, f: f32) -> bool {
    f < range.max && f > range.min
}