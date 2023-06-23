pub use crate::camera::Camera;
pub use crate::color::write_color;
pub use crate::data::{init, Data};
pub use crate::ray::Ray;
pub use crate::sphere::Sphere;
pub use crate::util::{
    color, hittable, random_in_unit_disk, random_in_unit_shpere, ray_dir, reflect, refract,
    unit_vec,
};
pub use crate::vec3::Vec3;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
use rand::{rngs::ThreadRng, Rng};

fn ray_color(r: Ray, v: &Vec<Sphere>, depth: u32) -> [u8; 3] {
    if depth == 0 {
        return [0; 3];
    }
    let (t, sphere) = hittable(r.clone(), v); //处理最近的光线交点

    if t != f64::INFINITY {
        //有正确的交点
        let p = r.at(t);
        let n: Vec3 = p.clone() - sphere.center.clone(); //法向量
        let normal: Vec3 = unit_vec(n.clone());
        let mut tmp: [u8; 3];

        //漫反射材料
        if sphere.tp == 1 {
            let scatter: Vec3 = normal.clone() + random_in_unit_shpere();

            tmp = ray_color(
                Ray {
                    a_origin: (p),
                    b_direction: (scatter),
                },
                v,
                depth - 1,
            );
            for l in 0..3 {
                tmp[l] = tmp[l] / 2 + sphere.color[l] / 2;
            } //一半反光，一般自己的颜色
        }
        //金属材料
        else if sphere.tp == 2 {
            let reflect: Vec3 =
                reflect(unit_vec(r.b_direction), normal) + random_in_unit_shpere() * sphere.fuzz;

            tmp = ray_color(
                Ray {
                    a_origin: (p),
                    b_direction: (reflect),
                },
                v,
                depth - 1,
            );
            for l in 0..3 {
                tmp[l] = tmp[l] / 5 * 4 + sphere.color[l] / 5;
            } //反光更多一点
        } else {
            //折射
            let refract = refract(unit_vec(r.b_direction), normal, 1.0 / sphere.etia);

            tmp = ray_color(
                Ray {
                    a_origin: (p),
                    b_direction: (refract),
                },
                v,
                depth - 1,
            );
            // for l in 0..3 {
            //     tmp[l] = tmp[l];
            // } //反光更多一点
        }
        tmp
    } else {
        //没交点那就是跟背景板（完全发光）有交点
        let unit_dir: Vec3 = unit_vec(r.b_direction);
        let t: f64 = 0.5 * (unit_dir.y() + 1.0);
        color(
            (1.0 - t) * 1.0 + 0.5 * t,
            (1.0 - t) * 1.0 + 0.7 * t,
            (1.0 - t) * 1.0 + 1.0 * t,
        )
    }
}

pub fn render(data: &Data, camera: &Camera, bar: ProgressBar) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let width = data.width;
    let height = data.height;
    let mut img: RgbImage = ImageBuffer::new(width.try_into().unwrap(), height.try_into().unwrap());
    let sphere_list: Vec<Sphere> = init();
    let mut random: ThreadRng = rand::thread_rng();

    for j in 0..height {
        //对于每个像素，发出对应方向的光线
        for i in 0..width {
            let mut sum_pixel_color: [f64; 3] = [0.0; 3];
            for _k in 0..data.sample_times {
                let s: f64 = ((i as f64) + random.gen::<f64>()) / ((width - 1) as f64); //行不变，竖直
                let t: f64 =
                    (((height - 1 - j) as f64) + random.gen::<f64>()) / ((height - 1) as f64);
                //由于要产生新的变量所以不能引用
                let rd = random_in_unit_disk() * camera.len_radius;
                let offset = camera.u.clone() * rd.x() + camera.v.clone() * rd.y();
                let r = Ray::new(
                    camera.origin.clone() + offset.clone(),
                    ray_dir(
                        &camera.lower_left_corner,
                        &camera.horizontal,
                        &camera.vertical,
                        s,
                        t,
                        offset,
                    ),
                );
                //r.b_direction.info();
                let tmp_pixel_color: [u8; 3] = ray_color(r, &sphere_list, data.depth);
                for l in 0..3 {
                    sum_pixel_color[l] += tmp_pixel_color[l] as f64;
                }
            }

            for element in &mut sum_pixel_color {
                *element = (*element / (data.sample_times * 255) as f64)
                    .powf(1.0 / (data.gamma as f64))
                    * 255.0;
            }
            let pixel_color: [u8; 3] = [
                sum_pixel_color[0] as u8,
                sum_pixel_color[1] as u8,
                sum_pixel_color[2] as u8,
            ];
            write_color(pixel_color, &mut img, i, j);

            bar.inc(1);
        }
    }
    bar.finish();
    img
}
