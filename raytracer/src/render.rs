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
use std::sync::{Arc, Mutex};
use std::thread;

fn ray_color(r: Ray, v: &Vec<Sphere>, depth: u32) -> [u8; 3] {
    if depth == 0 {
        return [0; 3];
    }
    let (t, sphere) = hittable(r.clone(), v); //处理最近的光线交点

    if t != f64::INFINITY {
        //有正确的交点
        let p: Vec3 = r.at(t);
        let normal: Vec3 = unit_vec(p.clone() - sphere.center.clone());
        let mut tmp: [u8; 3];

        //漫反射材料
        if sphere.tp == 1 {
            let scatter: Vec3 = normal + random_in_unit_shpere();

            tmp = ray_color(
                Ray {
                    a_origin: (p),
                    b_direction: (scatter),
                },
                v,
                depth - 1,
            );
            for l in 0..3 {
                tmp[l] = (tmp[l] as f64 * ((sphere.color[l] as f64) / 255.0)) as u8;
            }
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
                tmp[l] = (tmp[l] as f64 * ((sphere.color[l] as f64) / 255.0)) as u8;
            }
        } else {
            //折射
            let ratio;
            let dir;
            if sphere.front_back(r.b_direction.clone(), normal.clone()) {
                ratio = 1.0 / sphere.etia;
                dir = 1.0;
            } else {
                ratio = sphere.etia;
                dir = -1.0;
            }
            let refract = refract(unit_vec(r.b_direction), normal * dir, ratio);

            tmp = ray_color(
                Ray {
                    a_origin: (p),
                    b_direction: (refract),
                },
                v,
                depth - 1,
            );
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

pub fn pixel_color(
    i: usize,
    j: usize,
    camera: &Camera,
    sphere_list: &Vec<Sphere>,
    width: usize,
    height: usize,
    depth: u32,
) -> [u8; 3] {
    let mut random: ThreadRng = rand::thread_rng();
    let s: f64 = ((i as f64) + random.gen::<f64>()) / ((width - 1) as f64); //行不变，竖直
    let t: f64 = (((height - 1 - j) as f64) + random.gen::<f64>()) / ((height - 1) as f64);
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
    ray_color(r, &sphere_list, depth)
}

pub fn render(data: &Data, camera: Camera, bar: ProgressBar) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let width = data.width;
    let height = data.height;
    let depth = data.depth;
    let sample = data.sample_times;
    let gamma = data.gamma;

    let img: RgbImage = ImageBuffer::new(width.try_into().unwrap(), height.try_into().unwrap());
    let sphere_list: Vec<Sphere> = init();
    //有个data也要共享内存，要么就把值都提取出来
    let image = Arc::new(Mutex::new(img));
    let cam = Arc::new(camera);
    let sph_lst = Arc::new(sphere_list);
    let bar = Arc::new(bar);

    let mut handles = vec![];
    for j in 0..height {
        //以下开始一个线程  对共享内存的复制
        let c = Arc::clone(&cam);
        let sph = Arc::clone(&sph_lst);
        let bar = Arc::clone(&bar);
        //共享可写内存：上互斥锁
        let image = Arc::clone(&image);
        //开始线程
        let handle = thread::spawn(move || {
            for i in 0..width {
                let mut sum_pixel_color: [f64; 3] = [0.0; 3];

                for _k in 0..sample {
                    let tmp_pixel_color: [u8; 3] =
                        pixel_color(i, j, &c, &sph, width, height, depth);
                    for i in 0..3 {
                        sum_pixel_color[i] += tmp_pixel_color[i] as f64;
                    }
                }
                for element in &mut sum_pixel_color {
                    *element =
                        (*element / (sample * 255) as f64).powf(1.0 / (gamma as f64)) * 255.0;
                }
                let pixel_color: [u8; 3] = [
                    sum_pixel_color[0] as u8,
                    sum_pixel_color[1] as u8,
                    sum_pixel_color[2] as u8,
                ];
                let mut img = image.lock().unwrap();
                write_color(pixel_color, &mut (*img), i, j);
                (*bar).inc(1);
            }
        });
        //加入线程
        handles.push(handle);
    }
    //等待所有线程结束
    for i in handles {
        i.join().unwrap();
    }
    bar.finish();
    let img = image.lock().unwrap();
    (*img).clone()
}
