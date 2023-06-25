pub use crate::vec3::Vec3;
#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    pub a_origin: Vec3,
    pub b_direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(a_origin: Vec3, b_direction: Vec3, time: f64) -> Self {
        Self {
            a_origin,
            b_direction,
            time,
        }
    }
    pub fn at(&self, t: f64) -> Vec3 {
        self.a_origin.clone() + self.b_direction.clone() * t
    }
}
