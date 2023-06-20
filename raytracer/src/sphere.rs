use crate::vec3::Vec3;
use crate::ray::Ray;
#[derive(Clone, Debug, PartialEq)]
pub struct Sphere{
    pub center:Vec3,
    pub r:f64,
}

impl Sphere{
    pub fn hit_sphere(&self,r:Ray) -> f64{
        let ac = r.a_origin-self.center.clone();
        let a = r.b_direction.squared_length();
        let b =  (ac.clone() * r.b_direction)*2.0;
        let c = ac.squared_length() - self.r*self.r;
        let dos = b*b - a*c*4.0;
        if dos > 0.0{
            (-b-f64::sqrt(dos))/(2.0*a)
        }
        else {
            -1.0
        }
    }
    pub fn new(center:Vec3,r:f64)->Self{
        Self{
            center,
            r,
        }
    }
}