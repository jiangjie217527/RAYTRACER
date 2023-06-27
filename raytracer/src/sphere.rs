pub use crate::aabb::Aabb;
pub use crate::ray::Ray;
pub use crate::util::fabs;
pub use crate::vec3::Vec3;
#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    pub tp: u8,
    pub center: Vec3,
    pub destinity: Vec3, //移动目的地
    pub time1: f64,
    pub time2: f64,
    pub r: f64,

    pub color: [u8; 3], //颜色
    pub fuzz: f64,      //金属磨砂效果
    pub etia: f64,      //折射率

    pub texture_type:u8,
}

impl Sphere {
    pub fn center(&self, time: f64) -> Vec3 {
        self.center.clone()
            + (self.destinity.clone() - self.center.clone()) * (time - self.time1)
                / (self.time2 - self.time1)
    }
    pub fn hit_sphere(&self, r: Ray) -> (f64, f64) {
        let ac = r.a_origin - self.center(r.time);
        let a = r.b_direction.squared_length();
        let half_b = ac.clone() * r.b_direction;
        let c = ac.squared_length() - self.r * self.r;
        let dos = half_b * half_b - a * c;
        if dos > 0.0 {
            ((-half_b - f64::sqrt(dos)) / a, 2.0 * f64::sqrt(dos) / a)
        } else {
            (-1.0, 0.0)
        }
    }
    // pub fn new(
    //     center: Vec3,
    //     destinity: Vec3,
    //     time1: f64,
    //     time2: f64,
    //     r: f64,
    //     tp: u8,
    //     color: [u8; 3],
    //     fuzz: f64,
    //     etia: f64,
    // ) -> Self {
    //     Self {
    //         center,
    //         time1,
    //         time2,
    //         r,
    //         tp,
    //         fuzz,
    //         color,
    //         etia,
    //         destinity,
    //     }
    // }
    pub fn empty_sphere() -> Self {
        Self {
            tp: (255),
            center: (Vec3::zero()),
            time1: 0.0,
            time2: 0.0,
            r: (0.0),
            color: ([0; 3]),
            fuzz: 0.0,
            etia: 0.0,
            destinity: Vec3::zero(),
            texture_type:0,
        }
    }

    pub fn front_back(&self, ray_direction: Vec3, normal: Vec3) -> bool {
        // true is front and false is back
        ray_direction * normal * self.r / fabs(self.r) < 0.0
    }
    //产生bound_box
    pub fn bound_box(&self) -> Aabb {

            Aabb::surround_box(
                Aabb::new(
                    self.center(self.time1) - Vec3::new(self.r, self.r, self.r),
                    self.center(self.time1) + Vec3::new(self.r, self.r, self.r),
                ),
                Aabb::new(
                    self.center(self.time2) - Vec3::new(self.r, self.r, self.r),
                    self.center(self.time2) + Vec3::new(self.r, self.r, self.r),
                ),
            )

    }
}
