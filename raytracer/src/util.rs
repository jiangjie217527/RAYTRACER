pub use crate::aabb::BvhNode;
pub use crate::ray::Ray;
pub use crate::sphere::Sphere;
pub use crate::vec3::Vec3;
use rand::{rngs::ThreadRng, Rng};

pub fn ray_dir(
    lower_left_corner: &Vec3,
    horizontal: &Vec3,
    vertical: &Vec3,
    u: f64,
    v: f64,
    offset: Vec3,
) -> Vec3 {
    lower_left_corner.clone()//从左下角开始
    + horizontal.clone() * u //水平方向
    + vertical.clone() * v //竖直方向
    -offset
}
//单位向量的工具
pub fn unit_vec(v: Vec3) -> Vec3 {
    v.clone() / v.length()
}
pub fn fabs(num: f64) -> f64 {
    if num < 0.0 {
        -num
    } else {
        num
    }
}
pub fn fmin(v1: f64, v2: f64) -> f64 {
    if v1 > v2 {
        v2
    } else {
        v1
    }
}

pub fn fmax(v1: f64, v2: f64) -> f64 {
    if v1 > v2 {
        v1
    } else {
        v2
    }
}
//折射模块，还有点问题
pub fn reflectance(cos_theta: f64, ratio: f64) -> f64 {
    let r0 = (1.0 - ratio) / (1.0 + ratio);
    let t = r0 * r0;
    t + (1.0 - t) * f64::powf(1.0 - cos_theta, 5.0)
}

//ratio is etia / etia prime i.e the sphere is under the fraction
pub fn refract(v: Vec3, n: Vec3, ratio: f64) -> Vec3 {
    //v,n为单位向量
    //按道理应该不会有cos比1大
    let cos_theta = (Vec3::zero() - v.clone()) * n.clone();
    let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);
    let mut random: ThreadRng = rand::thread_rng();
    if ratio * sin_theta >= 1.0 || reflectance(cos_theta, ratio) > random.gen::<f64>() {
        //println!("reflect");
        reflect(v, n)
    } else {
        let perp = (v + n.clone() * cos_theta) * ratio;
        let para = Vec3::zero() - n * f64::sqrt(fabs(1.0 - perp.squared_length()));
        perp + para
    }
}
//反射模块，简单
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    //v,n为单位向量
    v.clone() - n.clone() * (v * n) * 2.0
}

pub fn random_in_unit_shpere() -> Vec3 {
    let mut random: ThreadRng = rand::thread_rng();
    loop {
        let p = Vec3::new(
            random.gen_range(-1.0..1.0),
            random.gen_range(-1.0..1.0),
            random.gen_range(-1.0..1.0),
        );
        if p.squared_length() >= 1.0 {
            continue;
        }
        let tmp: Vec3 = unit_vec(p);
        if tmp.near_zero() {
            return Vec3::zero();
        } else {
            return tmp;
        }
    }
}

pub fn random_f64_0_1() -> f64 {
    let mut random: ThreadRng = rand::thread_rng();
    random.gen::<f64>()
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut random: ThreadRng = rand::thread_rng();
    loop {
        let p = Vec3::new(
            random.gen_range(-1.0..1.0),
            random.gen_range(-1.0..1.0),
            0.0,
        );
        if p.squared_length() >= 1.0 {
            continue;
        }
        //let tmp = unit_vec(p);
        if p.near_zero() {
            return Vec3::zero();
        } else {
            return p;
        }
    }
}

pub fn color(x: f64, y: f64, z: f64) -> [u8; 3] {
    //讲0~1之间的数扩大 ，符合RGB
    [(255.0 * x) as u8, (255.0 * y) as u8, (255.0 * z) as u8]
}

//处理最近的光线交点
pub fn hittable(r: Ray, bvh_tree: &BvhNode) -> (f64, Sphere) {
    //let mut t = f64::INFINITY;
    let t_min = 0.001;
    //let mut sphere = &Sphere::empty_sphere();
    let (t, sphere) = bvh_tree.hit(&r, t_min, f64::INFINITY);
    (t, sphere)
}
// for i in v {
//     let (tmp, delta) = i.hit_sphere(r.clone());
//     if tmp < t_min || tmp > t {
//         let tmp = tmp + delta;
//         //可能影响折射
//         if tmp < t_min || tmp > t {
//             continue;
//         } else {
//             t = tmp;
//             sphere = i;
//         }
//     } else {
//         t = tmp;
//         sphere = i;
//     }
// }
