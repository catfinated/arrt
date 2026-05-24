#[derive(Copy, Clone, Debug)]
pub struct Range {
    pub min: f32,
    pub max: f32,
}

pub fn in_range(range: Range, f: f32) -> bool {
    f < range.max && f > range.min
}

#[cfg(test)]
mod tests {
    use super::*;

    fn range() -> Range {
        Range {
            min: 0.001,
            max: 100.0,
        }
    }

    #[test]
    fn value_inside() {
        assert!(in_range(range(), 50.0));
    }

    #[test]
    fn value_below_min() {
        assert!(!in_range(range(), 0.0));
    }

    #[test]
    fn value_above_max() {
        assert!(!in_range(range(), 200.0));
    }

    #[test]
    fn min_is_exclusive() {
        assert!(!in_range(range(), 0.001));
    }

    #[test]
    fn max_is_exclusive() {
        assert!(!in_range(range(), 100.0));
    }
}
