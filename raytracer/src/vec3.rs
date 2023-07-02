use crate::util::{fmax, fmin};
use std::ops::{Add, AddAssign, Div, Mul, Sub};
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}
impl Mul for Vec3 {
    type Output = f64;
    fn mul(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}
impl Vec3 {
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }
    pub fn lp(&self, index: u8) -> f64 {
        if index == 0 {
            self.x
        } else if index == 1 {
            self.y
        } else {
            self.z
        }
    }
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn merge_min(v1: &Vec3, v2: &Vec3) -> Self {
        Self {
            x: (fmin(v1.x, v2.x)),
            y: fmin(v1.y, v2.y),
            z: fmin(v1.z, v2.z),
        }
    }
    pub fn merge_max(v1: &Vec3, v2: &Vec3) -> Self {
        Self {
            x: (fmax(v1.x, v2.x)),
            y: fmax(v1.y, v2.y),
            z: fmax(v1.z, v2.z),
        }
    }
    // pub fn ones() -> Self {
    //     Self::new(1.0, 1.0, 1.0)
    // }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn near_zero(&self) -> bool {
        self.x() < 0.00000001
            && self.x() > -0.00000001
            && self.y() < 0.00000001
            && self.y() > -0.00000001
            && self.z() < 0.00000001
            && self.z() > -0.00000001
    }

    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn cross(&self, other: Vec3) -> Self {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        Vec3 {
            x: (x),
            y: (y),
            z: (z),
        }
    }

    // pub fn info(&self) {
    //     println!("x={},y={},z={}", self.x, self.y, self.z);
    // }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0));
    }
    #[test]
    fn test_add() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) + Vec3::new(2.0, 4.0, 6.0),
            Vec3::new(3.0, 4.0, 5.0)
        )
    }
    #[test]
    fn test_add_assign() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x += Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(x, Vec3::new(3.0, 4.0, 5.0))
    }
    #[test]
    fn test_add_f64() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) + 233.0,
            Vec3::new(234.0, 233.0, 232.0)
        )
    }
    // #[test]
    // fn test_add_assign_f64() {
    //     let mut x = Vec3::new(1.0, 0.0, -1.0);
    //     x += 233.0;
    //     assert_eq!(x, Vec3::new(234.0, 233.0, 232.0))
    // }
    // #[test]
    // fn test_sub() {
    //     assert_eq!(
    //         Vec3::new(1.0, 0.0, -1.0) - Vec3::new(2.0, 4.0, 6.0),
    //         Vec3::new(-1.0, -4.0, -7.0)
    //     )
    // }
    // #[test]
    // fn test_sub_assign() {
    //     let mut x = Vec3::new(1.0, 0.0, -1.0);
    //     x -= Vec3::new(2.0, 4.0, 6.0);
    //     assert_eq!(x, Vec3::new(-1.0, -4.0, -7.0))
    // }
    // #[test]
    // fn test_sub_f64() {
    //     assert_eq!(Vec3::new(1.0, 0.0, -1.0) - 1.0, Vec3::new(0.0, -1.0, -2.0))
    // }
    // #[test]
    // fn test_sub_assign_f64() {
    //     let mut x = Vec3::new(1.0, 0.0, -1.0);
    //     x -= 1.0;
    //     assert_eq!(x, Vec3::new(0.0, -1.0, -2.0))
    // }
    // #[test]
    // fn test_mul() {
    //     assert_eq!(Vec3::new(1.0, 0.0, -1.0) * Vec3::ones(), 0.0);
    // }
    // #[test]
    // fn test_mul_assign() {
    //     let mut x = Vec3::new(1.0, 0.0, -1.0);
    //     x *= 2.0;
    //     assert_eq!(x, Vec3::new(2.0, 0.0, -2.0));
    // }
    // #[test]
    // fn test_mul_f64() {
    //     assert_eq!(Vec3::new(1.0, 0.0, -1.0) * 1.0, Vec3::new(1.0, 0.0, -1.0));
    // }
    // #[test]
    // fn test_div() {
    //     assert_eq!(Vec3::new(1.0, -2.0, 0.0) / 2.0, Vec3::new(0.5, -1.0, 0.0));
    // }
    // #[test]
    // fn test_elemul() {
    //     assert_eq!(
    //         Vec3::elemul(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0)),
    //         Vec3::new(1.0, 4.0, 9.0)
    //     );
    // }
    // #[test]
    // fn test_cross() {
    //     assert_eq!(
    //         Vec3::cross(Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, 3.0, 4.0)),
    //         Vec3::new(8.0 - 9.0, 6.0 - 4.0, 3.0 - 4.0)
    //     );
    // }
    // #[test]
    // fn test_neg() {
    //     assert_eq!(-Vec3::new(1.0, -2.0, 3.0), Vec3::new(-1.0, 2.0, -3.0));
    // }
    #[test]
    fn test_squared_length() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).squared_length(), 14.0);
    }
    // #[test]
    // fn test_length() {
    //     assert_eq!(
    //         Vec3::new(3.0, 4.0, 5.0).length(),
    //         ((3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0) as f64).sqrt()
    //     );
    // }
    // #[test]
    // fn test_unit() {
    //     assert_eq!(Vec3::new(233.0, 0.0, 0.0).unit(), Vec3::new(1.0, 0.0, 0.0));
    //     assert_eq!(
    //         Vec3::new(-233.0, 0.0, 0.0).unit(),
    //         Vec3::new(-1.0, 0.0, 0.0)
    //     );
    // }
    // #[test]
    // #[should_panic]
    // fn test_unit_panic() {
    //     Vec3::new(0.0, 0.0, 0.0).unit();
    // }
}
