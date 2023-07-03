pub use crate::aabb::{Aabb, BvhNode};
pub use crate::camera::Camera;
pub use crate::color::write_color;
pub use crate::data::{final_scene, Data};
pub use crate::ray::Ray;
pub use crate::sphere::Sphere;
pub use crate::world::Object;
//pub use crate::test_scene::{init_debug2, sphere_debug2};
pub use crate::texture::{checher_color_value, get_uv, ImageTexture, Perlin};
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

pub fn ray_color(
    r: Ray,
    bvh_tree: &BvhNode,
    depth: u32,
    perlin: &Perlin,
    earth: &ImageTexture,
) -> [f64; 3] {
    if depth == 0 {
        return [0.0; 3];
    }
    let (t, obj) = hittable(r.clone(), bvh_tree); //处理最近的光线交点
    match obj {
        Object::Sphere(sphere) => {
            sphere.sphere_color(t, &r, bvh_tree, depth, perlin, earth)
        }
        Object::Xy(o) => {
            if o.tp == 1 {
                let p: Vec3 = r.at(t);
                let n = o.normal(&r);
                let scatter = n + random_in_unit_shpere();
                let mut tmp = ray_color(
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
                for (l, _) in tmp.clone().iter_mut().enumerate() {
                    tmp[l] *= o.emit[l];
                }
                tmp
            } else {
                o.emit
            }
        }
        Object::Xz(o) => {
            if o.tp == 1 {
                let p: Vec3 = r.at(t);
                let n = o.normal(&r);
                let scatter = n + random_in_unit_shpere();
                let mut tmp = ray_color(
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
                for (l, _) in tmp.clone().iter_mut().enumerate() {
                    tmp[l] *= o.emit[l];
                }
                tmp
            } else {
                o.emit
            }
        }
        Object::Yz(o) => {
            if o.tp == 1 {
                let p: Vec3 = r.at(t);
                let n = o.normal(&r);
                let scatter = n + random_in_unit_shpere();
                let mut tmp = ray_color(
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
                for (l, _) in tmp.clone().iter_mut().enumerate() {
                    tmp[l] *= o.emit[l];
                }
                tmp
            } else {
                o.emit
            }
        }
        Object::Tr(tr) => {
            // println!("hit rotate");
            // let (p, normal) = tr.p_nor(t, &r);
            // p.info();
            let (t,s) = tr.hit(&r, 0.001, f64::INFINITY);
            
            match s {
                Object::Sphere(sph)=>{
                    // println!("{}",t);
                    // sph.info();
                    // return [1.0;3];
                    //sph.info();
                    sph.sphere_color(t, &r, bvh_tree, depth, perlin, earth)
                }
                _=>{
                    let (p, normal) = tr.p_nor(t, &r);
                    let scatter = normal + random_in_unit_shpere();
                    let mut tmp = ray_color(
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
                    for (l, _) in tmp.clone().iter_mut().enumerate() {
                        tmp[l] *= tr.bx_tr.bx_ro.emit[l];
                    }
                    tmp                    
                }
            }

            // return ;
            // let (p, normal) = tr.p_nor(t, &r);
            // let scatter = normal + random_in_unit_shpere();
            // let mut tmp = ray_color(
            //     Ray {
            //         a_origin: (p),
            //         b_direction: (scatter),
            //         time: r.time,
            //     },
            //     bvh_tree,
            //     depth - 1,
            //     perlin,
            //     earth,
            // );
            // for (l, _) in tmp.clone().iter_mut().enumerate() {
            //     tmp[l] *= tr.bx_tr.bx_ro.emit[l];
            // }
            // tmp
        }
        Object::Fg(fg)=>{
            // println!("hit fog");
            let (p, _) = fg.p_nor(t, &r);
            let scatter =  random_in_unit_shpere();
            let mut tmp = ray_color(
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
            for (l, _) in tmp.clone().iter_mut().enumerate() {
                tmp[l] *= fg.color[l];
            }
            tmp            
        }
    }
}

pub fn pixel_color(
    (i, j): (usize, usize),
    camera: &Camera,
    bvh_tree: &BvhNode,
    (width, height): (usize, usize),
    depth: u32,
    perlin: &Perlin,
    earth: &ImageTexture,
) -> [f64; 3] {
    let mut random: ThreadRng = rand::thread_rng();
    let s: f64 = ((i as f64) + random.gen::<f64>()) / ((width - 1) as f64); //行不变，竖直
    let t: f64 = (((height - 1 - j) as f64) + random.gen::<f64>()) / ((height - 1) as f64);
    //由于要产生新的变量所以不能引用
    let rd = random_in_unit_disk() * camera.len_radius;
    let offset = camera.u * rd.x() + camera.v * rd.y();
    let r = Ray::new(
        camera.origin + offset,
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
    ray_color(r, bvh_tree, depth, perlin, earth)
}

pub fn render(data: &Data, camera: Camera, bar: ProgressBar) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let width = data.width;
    let height = data.height;
    let depth = data.depth;
    let sample = data.sample_times;
    let gamma = data.gamma;

    let img: RgbImage = ImageBuffer::new(width.try_into().unwrap(), height.try_into().unwrap());
    let object_list: Vec<Object> = final_scene();
    let mut bvh_tree = BvhNode::new(&Object::empty());
    bvh_tree.build(object_list.clone(), 0, object_list.len());
    // return img;
    let perlin = Perlin::init();
    let earth = ImageTexture::new("earthmap.jpg");
    //bvh_tree.info();
    //有个data也要共享内存，要么就把值都提取出来
    //上互斥锁
    let image = Arc::new(Mutex::new(img));
    let cam = Arc::new(camera);
    //let sph_lst = Arc::new(sphere_list);
    let bar = Arc::new(bar);
    let bvh_tree = Arc::new(bvh_tree);
    let perlin = Arc::new(perlin);
    let earth = Arc::new(earth);

    let mut handles = vec![];
    for j in 0..height {
        //以下开始一个线程  对共享内存的复制
        let c = Arc::clone(&cam);
        //let sph = Arc::clone(&sph_lst);
        let bar = Arc::clone(&bar);
        let bvh_node = Arc::clone(&bvh_tree);
        let perlin = Arc::clone(&perlin);
        let earth = Arc::clone(&earth);
        //共享可写内存：上互斥锁
        let image = Arc::clone(&image);
        //开始线程
        let handle = thread::spawn(move || {
            for i in 0..width {
                let mut sum_pixel_color: [f64; 3] = [0.0; 3];

                for _k in 0..sample {
                    let tmp_pixel_color: [f64; 3] = pixel_color(
                        (i, j),
                        &c,
                        &bvh_node,
                        (width, height),
                        depth,
                        &perlin,
                        &earth,
                    );
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
