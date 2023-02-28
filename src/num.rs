use std::ops::Add;

pub const PI: f32 = 3.14159265358979323846;
pub const TWO_PI: f32 = 2.0 * 3.14159265358979323846;
pub const HALF_PI: f32 = 3.14159265358979323846 / 2.0;

#[derive(Clone, Copy, PartialEq)]
pub struct Range {
    pub left: f32,
    pub right: f32,
}
impl Range {
    pub fn contain(&self, value: f32) -> bool {
        self.left <= value && value <= self.right
    }

    pub fn clamp(&self, value: f32) -> f32 {
        self.left.max(self.right.min(value))
    }
}

#[test]
fn test_contain() {
    let mut range = Range {
        left: 0.0,
        right: 1.0,
    };
    assert!(range.contain(0.5));
    assert!(!range.contain(1.5));

    range.left = -0.5;
    assert!(range.contain(0.5));
    assert!(range.contain(-0.5));
    assert!(!range.contain(-1.5));
}

#[test]
fn test_clamp() {
    let mut range = Range {
        left: 0.0,
        right: 1.0,
    };
    assert_eq!(range.left, range.clamp(-0.5));
    assert_eq!(0.5, range.clamp(0.5));
}
