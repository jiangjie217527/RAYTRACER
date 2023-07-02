pub use crate::aabb::Aabb;
pub use crate::ray::Ray;
pub use crate::sphere::Sphere;
pub use crate::vec3::Vec3;
//use std::sync::Arc;
#[derive(Clone, Debug, PartialEq)]

//tp == 1 漫反射
//tp == 2 发光

pub struct Xyrect {
    pub emit: [f64; 3],
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    pub tp: u8,
}

impl Xyrect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, emit: [f64; 3], tp: u8) -> Self {
        Self {
            emit,
            x0,
            x1,
            y0,
            y1,
            k,
            tp,
        }
    }
    pub fn bounding_box(&self) -> Aabb {
        // The bounding box must have non-zero width in each dimension, so pad the Z
        // dimension a small amount.
        Aabb::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        )
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> f64 {
        let t = (self.k - r.a_origin.z()) / r.b_direction.z();
        if t < t_min || t > t_max {
            return f64::INFINITY;
        }
        let x = r.a_origin.x() + t * r.b_direction.x();
        let y = r.a_origin.y() + t * r.b_direction.y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return f64::INFINITY;
        }
        t
    }
    pub fn normal(&self, r: &Ray) -> Vec3 {
        let n = Vec3::new(0.0, 0.0, 1.0);
        if n * r.b_direction > 0.0 {
            n * (-1.0)
        } else {
            n
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Xzrect {
    pub emit: [f64; 3],
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    pub tp: u8,
    pub k: f64,
}

impl Xzrect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, emit: [f64; 3], tp: u8) -> Self {
        Self {
            emit,
            x0,
            x1,
            z0,
            z1,
            k,
            tp,
        }
    }
    pub fn bounding_box(&self) -> Aabb {
        // The bounding box must have non-zero width in each dimension, so pad the Z
        // dimension a small amount.
        Aabb::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        )
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> f64 {
        let t = (self.k - r.a_origin.y()) / r.b_direction.y();
        if t < t_min || t > t_max {
            return f64::INFINITY;
        }
        let x = r.a_origin.x() + t * r.b_direction.x();
        let z = r.a_origin.z() + t * r.b_direction.z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return f64::INFINITY;
        }
        t
    }
    pub fn normal(&self, r: &Ray) -> Vec3 {
        let n = Vec3::new(0.0, 1.0, 0.0);
        if n * r.b_direction > 0.0 {
            n * (-1.0)
        } else {
            n
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Yzrect {
    pub emit: [f64; 3],
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    pub tp: u8,
}

impl Yzrect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, emit: [f64; 3], tp: u8) -> Self {
        Self {
            emit,
            y0,
            y1,
            z0,
            z1,
            k,
            tp,
        }
    }
    pub fn bounding_box(&self) -> Aabb {
        // The bounding box must have non-zero width in each dimension, so pad the Z
        // dimension a small amount.
        Aabb::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        )
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> f64 {
        let t = (self.k - r.a_origin.x()) / r.b_direction.x();
        if t < t_min || t > t_max {
            return f64::INFINITY;
        }
        let y = r.a_origin.y() + t * r.b_direction.y();
        let z = r.a_origin.z() + t * r.b_direction.z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return f64::INFINITY;
        }
        t
    }
    pub fn normal(&self, r: &Ray) -> Vec3 {
        let n = Vec3::new(1.0, 0.0, 0.0);
        if n * r.b_direction > 0.0 {
            n * (-1.0)
        } else {
            n
        }
    }
}
