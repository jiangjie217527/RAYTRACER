pub use crate::aabb::{Aabb, BvhNode};
pub use crate::camera::Camera;
pub use crate::color::write_color;
pub use crate::data::{two_perlin_spheres, Data};
pub use crate::ray::Ray;
pub use crate::sphere::Sphere;
//pub use crate::test_scene::{init_debug2, sphere_debug2};
pub use crate::texture::{checher_color_value, Perlin};
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

fn ray_color(r: Ray, bvh_tree: &BvhNode, depth: u32, perlin: &Perlin) -> [f64; 3] {
    if depth == 0 {
        return [0.0; 3];
    }
    let (t, sphere) = hittable(r.clone(), bvh_tree); //处理最近的光线交点

    if t != f64::INFINITY {
        //有正确的交点
        let p: Vec3 = r.at(t);

        let normal: Vec3 = unit_vec(p.clone() - sphere.center.clone());
        let mut tmp: [f64; 3];

        //漫反射材料
        if sphere.tp == 1 {
            let scatter: Vec3 = normal.clone() + random_in_unit_shpere();

            tmp = ray_color(
                Ray {
                    a_origin: (p),
                    b_direction: (scatter),
                    time: r.time,
                },
                bvh_tree,
                depth - 1,
                perlin,
            );
        }
        //金属材料
        else if sphere.tp == 2 {
            let reflect: Vec3 = reflect(unit_vec(r.b_direction), normal.clone())
                + random_in_unit_shpere() * sphere.fuzz;

            tmp = ray_color(
                Ray {
                    a_origin: (p),
                    b_direction: (reflect),
                    time: r.time,
                },
                bvh_tree,
                depth - 1,
                perlin,
            );
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
            let refract = refract(unit_vec(r.b_direction), normal.clone() * dir, ratio);

            tmp = ray_color(
                Ray {
                    a_origin: (p),
                    b_direction: (refract),
                    time: r.time,
                },
                bvh_tree,
                depth - 1,
                perlin,
            );
        }
        if sphere.tp != 3 {
            if sphere.texture_type == 0 {
                for (l, _) in tmp.clone().iter_mut().enumerate() {
                    tmp[l] *= sphere.color[l] as f64 / 255.0;
                }
            } else if sphere.texture_type == 1 {
                //let (u,v) = get_uv(normal.clone());
                let sphere_texture = checher_color_value(normal * sphere.r);
                for (l, _) in tmp.clone().iter_mut().enumerate() {
                    tmp[l] *= sphere_texture[l];
                }
            } else {
                let sphere_texture = perlin.noise(&(normal * sphere.r));
                for (l, _) in tmp.clone().iter_mut().enumerate() {
                    tmp[l] *= sphere_texture;
                }
            }
        }
        tmp
    } else {
        //没交点那就是跟背景板（完全发光）有交点
        let unit_dir: Vec3 = unit_vec(r.b_direction);
        let t: f64 = 0.5 * (unit_dir.y() + 1.0);
        [
            (1.0 - t) * 1.0 + 0.5 * t,
            (1.0 - t) * 1.0 + 0.7 * t,
            (1.0 - t) * 1.0 + 1.0 * t,
        ]
    }
}

pub fn pixel_color(
    (i, j): (usize, usize),
    camera: &Camera,
    bvh_tree: &BvhNode,
    width: usize,
    height: usize,
    depth: u32,
    perlin: &Perlin,
) -> [f64; 3] {
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
        random.gen_range(camera.time1..camera.time2),
    );
    //r.b_direction.info();
    ray_color(r, bvh_tree, depth, perlin)
}

pub fn render(data: &Data, camera: Camera, bar: ProgressBar) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let width = data.width;
    let height = data.height;
    let depth = data.depth;
    let sample = data.sample_times;
    let gamma = data.gamma;

    let img: RgbImage = ImageBuffer::new(width.try_into().unwrap(), height.try_into().unwrap());
    let sphere_list: Vec<Sphere> = two_perlin_spheres();
    let mut bvh_tree = BvhNode::new(&Sphere::empty_sphere());
    bvh_tree.build(sphere_list.clone(), 0, sphere_list.len());

    let perlin = Perlin::init();
    //bvh_tree.info();
    //有个data也要共享内存，要么就把值都提取出来
    //上互斥锁
    let image = Arc::new(Mutex::new(img));
    let cam = Arc::new(camera);
    //let sph_lst = Arc::new(sphere_list);
    let bar = Arc::new(bar);
    let bvh_tree = Arc::new(bvh_tree);
    let perlin = Arc::new(perlin);

    let mut handles = vec![];
    for j in 0..height {
        //以下开始一个线程  对共享内存的复制
        let c = Arc::clone(&cam);
        //let sph = Arc::clone(&sph_lst);
        let bar = Arc::clone(&bar);
        let bvh_node = Arc::clone(&bvh_tree);
        let perlin = Arc::clone(&perlin);
        //共享可写内存：上互斥锁
        let image = Arc::clone(&image);
        //开始线程
        let handle = thread::spawn(move || {
            for i in 0..width {
                let mut sum_pixel_color: [f64; 3] = [0.0; 3];

                for _k in 0..sample {
                    let tmp_pixel_color: [f64; 3] =
                        pixel_color((i, j), &c, &bvh_node, width, height, depth, &perlin);
                    for i in 0..3 {
                        sum_pixel_color[i] += tmp_pixel_color[i];
                    }
                }
                for element in &mut sum_pixel_color {
                    *element = (*element / sample as f64).powf(1.0 / (gamma as f64));
                }
                let pixel_color: [u8; 3] =
                    color(sum_pixel_color[0], sum_pixel_color[1], sum_pixel_color[2]);
                let mut img = image.lock().unwrap();
                write_color(pixel_color, &mut img, i, j);
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
