pub use crate::aabb::Aabb;
pub use crate::aabb::BvhNode;
pub use crate::ray::Ray;
pub use crate::render::ray_color;
pub use crate::texture::{checher_color_value, get_uv, ImageTexture, Perlin};
pub use crate::util::{
    color, fabs, hittable, random_in_unit_disk, random_in_unit_shpere, ray_dir, reflect, refract,
    unit_vec,
};
pub use crate::vec3::Vec3;
#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    /**
     * tp=1漫反射
     * tp=2金属
     * tp=3透光
     * tp=4光源
     */
    pub tp: u8,
    pub center: Vec3,
    pub destinity: Vec3, //移动目的地
    pub time1: f64,
    pub time2: f64,
    pub r: f64,

    pub color: [u8; 3], //颜色
    pub emit: [f64; 3],

    pub fuzz: f64, //金属磨砂效果
    pub etia: f64, //折射率

    pub texture_type: u8,
}

impl Sphere {
    pub fn center(&self, time: f64) -> Vec3 {
        self.center
            + (self.destinity - self.center) * (time - self.time1) / (self.time2 - self.time1)
    }
    pub fn hit_sphere(&self, r: &Ray, t_min: f64, t_max: f64) -> f64 {
        let ac = r.a_origin - self.center(r.time);
        let a = r.b_direction.squared_length();
        let half_b = ac * r.b_direction;
        let c = ac.squared_length() - self.r * self.r;
        let dos = half_b * half_b - a * c;
        if dos < 0.0 {
            return f64::INFINITY;
        }
        let mut tmp = (-half_b - f64::sqrt(dos)) / a;
        if tmp < t_min || tmp > t_max {
            tmp += 2.0 * f64::sqrt(dos) / a;
            //可能影响折射
            if tmp < t_min || tmp > t_max {
                tmp = f64::INFINITY;
            }
        }
        if tmp < t_max && tmp > t_min {
            tmp
        } else {
            f64::INFINITY
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
    //     texture_type: u8,
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
    //         texture_type,
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
            emit: ([0.0; 3]),
            fuzz: 0.0,
            etia: 0.0,
            destinity: Vec3::zero(),
            texture_type: 0,
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
    pub fn sphere_color(
        &self,
        t: f64,
        r: &Ray,
        bvh_tree: &BvhNode,
        depth: u32,
        perlin: &Perlin,
        earth: &ImageTexture,
    ) -> [f64; 3] {
        if t != f64::INFINITY {
            //有正确的交点
            let p: Vec3 = r.at(t);
            let normal: Vec3 = unit_vec(p - self.center);
            let mut scatter: Vec3 = Vec3::ones();
            if self.tp == 4 {
                return self.emit;
            }
            //漫反射材料
            if self.tp == 1 {
                scatter = normal + random_in_unit_shpere();
            }
            //金属材料
            else if self.tp == 2 {
                scatter =
                    reflect(unit_vec(r.b_direction), normal) + random_in_unit_shpere() * self.fuzz;
            } else if self.tp == 3 {
                //折射
                let ratio;
                let dir;
                if self.front_back(r.b_direction, normal) {
                    ratio = 1.0 / self.etia;
                    dir = 1.0;
                } else {
                    ratio = self.etia;
                    dir = -1.0;
                }
                scatter = refract(unit_vec(r.b_direction), normal * dir, ratio);
            }
            let mut tmp: [f64; 3];
            tmp = ray_color(
                Ray {
                    a_origin: (p),
                    b_direction: (scatter),
                    time: r.time,
                },
                bvh_tree,
                depth - 1,
                perlin,
                earth,
            );
            if self.tp < 3 {
                if self.texture_type == 0 {
                    for (l, _) in tmp.clone().iter_mut().enumerate() {
                        tmp[l] *= self.color[l] as f64 / 255.0;
                    }
                } else if self.texture_type == 1 {
                    let sphere_texture = checher_color_value(normal * self.r);
                    for (l, _) in tmp.clone().iter_mut().enumerate() {
                        tmp[l] *= sphere_texture[l];
                    }
                } else if self.texture_type == 2 {
                    let sphere_texture = perlin.turb(&(normal * self.r));
                    for (l, _) in tmp.clone().iter_mut().enumerate() {
                        tmp[l] *= 0.5 * (1.0 + (0.1 * p.z() + 10.0 * sphere_texture).sin());
                    }
                } else {
                    let (u, v) = get_uv(normal);
                    let color = earth.value(u, v);
                    for (l, _) in tmp.clone().iter_mut().enumerate() {
                        tmp[l] *= color[l];
                    }
                }
            }
            tmp
        } else {
            //t==infity
            //没交点那就是跟背景板（完全不发光）有交点
            //background
            [0.0; 3]
        }
    }

    // pub fn info(&self){
    //     self.center.info();
    // }
}
