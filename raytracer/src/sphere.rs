pub use crate::ray::Ray;
pub use crate::vec3::Vec3;
#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    pub tp: u8,
    pub center: Vec3,
    pub r: f64,
    pub color: [u8; 3],
    pub fuzz: f64, //金属磨砂效果
    pub etia: f64, //折射率
}

impl Sphere {
    pub fn hit_sphere(&self, r: Ray) -> f64 {
        let ac = r.a_origin - self.center.clone();
        let a = r.b_direction.squared_length();
        let half_b = ac.clone() * r.b_direction;
        let c = ac.squared_length() - self.r * self.r;
        let dos = half_b * half_b - a * c;
        if dos > 0.0 {
            (-half_b - f64::sqrt(dos)) / a
        } else {
            -1.0
        }
    }
    pub fn new(center: Vec3, r: f64, tp: u8, color: [u8; 3], fuzz: f64, etia: f64) -> Self {
        Self {
            center,
            r,
            tp,
            fuzz,
            color,
            etia,
        }
    }
    pub fn empty_sphere() -> Self {
        Self {
            tp: (255),
            center: (Vec3::zero()),
            r: (0.0),
            color: ([0; 3]),
            fuzz: 0.0,
            etia: 0.0,
        }
    }
}
