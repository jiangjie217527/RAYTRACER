pub use crate::aabb::Aabb;
pub use crate::boxx::Boxx;
pub use crate::ray::Ray;
pub use crate::util::{fmax, fmin};
pub use crate::vec3::Vec3;

#[derive(Clone, Debug, PartialEq)]
pub struct Rotatey {
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
    pub bx_ro: Boxx,
}
impl Rotatey {
    pub fn new(radians: f64, bx_ro: Boxx) -> Self {
        let mut bounbox = bx_ro.bounding_box();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Vec3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bounbox.clone().maximum.x()
                        + (1 - i) as f64 * bounbox.clone().minimum.x();
                    let y = j as f64 * bounbox.clone().maximum.y()
                        + (1 - j) as f64 * bounbox.clone().minimum.y();
                    let z = k as f64 * bounbox.clone().maximum.z()
                        + (1 - k) as f64 * bounbox.clone().minimum.z();

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let taster = Vec3::new(newx, y, newz);

                    min.x = fmin(min.x(), taster.x());
                    min.y = fmin(min.y(), taster.y());
                    min.z = fmin(min.z(), taster.z());

                    max.x = fmax(max.x(), taster.x());
                    max.y = fmax(max.y(), taster.y());
                    max.z = fmax(max.z(), taster.z());
                }
            }
        }
        bounbox = Aabb::new(min, max);
        //bounbox.info();
        Self {
            sin_theta,
            cos_theta,
            bbox: bounbox,
            bx_ro,
        }
    }
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> f64 {
        let mut origin = r.a_origin;
        let mut direction = r.b_direction;
        origin.x = self.cos_theta * r.a_origin.x() - self.sin_theta * r.a_origin.z();
        origin.z = self.sin_theta * r.a_origin.x() + self.cos_theta * r.a_origin.z();

        direction.x = self.cos_theta * r.b_direction.x() - self.sin_theta * r.b_direction.z();
        direction.z = self.sin_theta * r.b_direction.x() + self.cos_theta * r.b_direction.z();

        let rotated_r = Ray::new(origin, direction, r.time);

        self.bx_ro.hit(&rotated_r, t_min, t_max)
    }

    pub fn p_nor(&self, t: f64, r: &Ray) -> (Vec3, Vec3) {
        let mut origin = r.a_origin;
        let mut direction = r.b_direction;
        origin.x = self.cos_theta * r.a_origin.x() - self.sin_theta * r.a_origin.z();
        origin.z = self.sin_theta * r.a_origin.x() + self.cos_theta * r.a_origin.z();

        direction.x = self.cos_theta * r.b_direction.x() - self.sin_theta * r.b_direction.z();
        direction.z = self.sin_theta * r.b_direction.x() + self.cos_theta * r.b_direction.z();

        let rotated_r = Ray::new(origin, direction, r.time);

        let p = rotated_r.at(t);
        let mut pp = p;
        let normal = self.bx_ro.normal(&rotated_r);
        let mut nnormal = normal;

        pp.x = self.cos_theta * p.x() + self.sin_theta * p.z();
        pp.z = -self.sin_theta * p.x() + self.cos_theta * p.z();

        nnormal.x = self.cos_theta * normal.x() + self.sin_theta * normal.z();
        nnormal.z = -self.sin_theta * normal.x() + self.cos_theta * normal.z();

        (pp, nnormal)
    }

    pub fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Translate{
    pub bx_tr:Rotatey,
    offset:Vec3
}

impl Translate{
    pub fn new(bx_tr:Rotatey,offset:Vec3)->Self{
        Self { bx_tr, offset}
    }
    pub fn bounding_box(&self) -> Aabb {
        Aabb::new(
        self.bx_tr.bbox.minimum+self.offset,
        self.bx_tr.bbox.maximum+self.offset)
    }

    pub fn hit(&self,r:&Ray,t_min:f64,t_max:f64)->f64{
        let moved_ray = Ray::new(r.a_origin-self.offset, r.b_direction, r.time);
        self.bx_tr.hit(&moved_ray, t_min, t_max)
    }

    pub fn p_nor(&self, t: f64, r: &Ray) -> (Vec3, Vec3){
        let moved_ray = Ray::new(r.a_origin-self.offset, r.b_direction, r.time);
        let (p,normal) = self.bx_tr.p_nor(t, &moved_ray);
        (p+self.offset,normal)
    }
}
