pub use crate::vec3::Vec3;

pub fn unit_vec(v:Vec3)->Vec3{
    v.clone()/v.length()
}