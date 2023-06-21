pub use crate::color::write_color;
pub use crate::data::Data;
pub use crate::ray::Ray;
pub use crate::sphere::Sphere;
pub use crate::vec3::Vec3;
pub use crate::util::unit_vec;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
use rand::{rngs::ThreadRng, Rng};

fn color(x: f64, y: f64, z: f64) -> [u8; 3] {
    //讲0~1之间的数扩大 ，符合RGB
    [(255.0 * x) as u8, (255.0 * y) as u8, (255.0 * z) as u8]
}

fn random_in_unit_shpere()->Vec3{
    let mut random: ThreadRng = rand::thread_rng();
    loop {
        let p = Vec3::new
        (random.gen_range(-1.0..1.0), random.gen_range(-1.0..1.0), random.gen_range(-1.0..1.0));
        if p.squared_length() >= 1.0{
            continue;
        }
        return unit_vec(p);
    }
    
}

fn reflect(v:Vec3,n:Vec3)->Vec3{
    println!("!");
    v.clone()-n.clone()*(v*n)*2.0
}

fn ray_color(r: Ray, v: &Vec<Sphere>,depth:u32) -> [u8; 3] {
    if depth == 0{
        return [0;3];
    }
    //处理每条光线返回的颜色
    let mut t = f64::INFINITY;
    let t_min = 0.001;
    let mut sphere = &v[0];
    for i in v {
        let tmp = i.hit_sphere(r.clone());
        if tmp < t_min || tmp > t {
            continue;
        } else {
            //这里要注意不能为0
            t = tmp;
            sphere = i;
        }
    }

    //let t = sphere.hit_sphere(r.clone());
    if t != f64::INFINITY {
        //有正确的交点
        let p = r.at(t);
        let n: Vec3 = p.clone() - sphere.center.clone(); //法向量
        let normal:Vec3 = unit_vec(n.clone()); 
        let mut tmp: [u8; 3];
        if sphere.tp==1{
            let mut scatter: Vec3 = normal.clone()+random_in_unit_shpere();
            if scatter.near_zero(){
                scatter = normal;
            }
            tmp = ray_color(Ray { a_origin: (p), b_direction: (scatter) },
            v,depth -1);
        }
        else {
            let reflect_scatter = reflect(r.b_direction, n);
            tmp = ray_color(Ray { a_origin: (p), b_direction: (reflect_scatter) },
             v, depth-1);
        }
        for l in 0..3{
            tmp[l]/=2;
        }
        tmp 

    } else {
        //没交点那就是跟背景板有交点
        let unit_dir: Vec3 = unit_vec(r.b_direction);
        let t: f64 = 0.5 * (unit_dir.y() + 1.0);
        color(
            (1.0 - t) * 1.0 + 0.5 * t,
            (1.0 - t) * 1.0 + 0.7 * t,
            (1.0 - t) * 1.0 + 1.0 * t,
        )
    }
}

fn ray_dir(lower_left_corner: &Vec3, horizontal: &Vec3, vertical: &Vec3, u: f64, v: f64) -> Vec3 {
    lower_left_corner.clone()//从左下角开始
    + horizontal.clone() * u //水平方向
    + vertical.clone() * v //竖直方向
}

pub fn render(data: &Data, bar: ProgressBar) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let width = data.width;
    let height = data.height;
    let mut img: RgbImage = ImageBuffer::new(width.try_into().unwrap(), height.try_into().unwrap());

    let sphere_list: Vec<Sphere> = vec![
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5,1),
        Sphere {
            center: (Vec3::new(0.0, -100.5, -1.0)),
            r: (100.0),
            tp:(1),
        },
        // Sphere {
        //     center: (Vec3::new(100.0, 0.0, -1.0)),
        //     r: (0.5),
        //     tp:(2),
        // },
    ];

    let mut random: ThreadRng = rand::thread_rng();
    for j in 0..height {
        //对于每个像素，发出对应方向的光线
        for i in 0..width {
            let mut sum_pixel_color: [f64; 3] = [0.0; 3];
            for _k in 0..data.sample_times {
                let u = ((i as f64) + random.gen::<f64>()) / ((width - 1) as f64); //行不变，竖直
                let v = (((height - 1 - j) as f64) + random.gen::<f64>()) / ((height - 1) as f64);
                let dir = ray_dir(
                    &data.lower_left_corner,
                    &data.horizontal,
                    &data.vertical,
                    u,
                    v,
                );
                //由于要产生新的变量所以不能引用
                let r = Ray::new(data.origin.clone(), dir); //dir不需要克隆
                let tmp_pixel_color: [u8; 3] = ray_color(r, &sphere_list,data.depth);
                for l in 0..3 {
                    sum_pixel_color[l] += tmp_pixel_color[l] as f64;
                }
            }
            for element in &mut sum_pixel_color {
                *element=(*element / (data.sample_times*255) as f64)
                .powf(1.0/(data.gamma as f64))*255.0;
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
/*
          let n: Vec3 = r.at(t) - sphere.center.clone(); //法向量
 let unit: f64 = 1.0 / n.length(); //长度的倒数
 let n: Vec3 = n * (unit); //乘倒数就是除以长度得到单位向量

 color(
     0.5 * (n.x() + 1.0),
     0.5 * (n.y() + 1.0),
     0.5 * (n.z() + 1.0),
 )
 */