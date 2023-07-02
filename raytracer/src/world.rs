pub use crate::aabb::Aabb;
pub use crate::aarec::{Xyrect, Xzrect, Yzrect};
pub use crate::ray::Ray;
pub use crate::sphere::Sphere;
#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    Sphere(Sphere),
    Xy(Xyrect),
    Xz(Xzrect),
    Yz(Yzrect),
}

impl Object {
    pub fn bo_box(&self) -> Aabb {
        match self {
            Object::Sphere(s) => s.bound_box(),
            Object::Xy(z) => z.bounding_box(),
            Object::Xz(y) => y.bounding_box(),
            Object::Yz(x) => x.bounding_box(),
        }
    }

    pub fn empty() -> Self {
        Object::Sphere(Sphere::empty_sphere())
    }

    pub fn hit_object(&self, r: &Ray, t_min: f64, t_max: f64) -> f64 {
        match self {
            Object::Sphere(s) => s.hit_sphere(r, t_min, t_max),
            Object::Xy(z) => z.hit(r, t_min, t_max),
            Object::Xz(z) => z.hit(r, t_min, t_max),
            Object::Yz(z) => z.hit(r, t_min, t_max),
        }
    }
}
