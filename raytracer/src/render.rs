pub use crate::color::write_color;
pub use crate::data::Data;
pub use crate::ray::Ray;
pub use crate::sphere::Sphere;
pub use crate::vec3::Vec3;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
use rand::{rngs::ThreadRng, Rng};

fn color(x: f64, y: f64, z: f64) -> [u8; 3] {
    //讲0~1之间的数扩大 ，符合RGB
    [(255.0 * x) as u8, (255.0 * y) as u8, (255.0 * z) as u8]
}

fn ray_color(r: Ray, v: &Vec<Sphere>) -> [u8; 3] {
    //处理每条光线返回的颜色
    let mut t = f64::INFINITY;
    let t_min = 0.0;
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
        let n: Vec3 = r.at(t) - sphere.center.clone(); //法向量
        let unit: f64 = 1.0 / n.length(); //长度的倒数
        let n: Vec3 = n * (unit); //乘倒数就是除以长度得到单位向量

        color(
            0.5 * (n.x() + 1.0),
            0.5 * (n.y() + 1.0),
            0.5 * (n.z() + 1.0),
        )
    } else {
        //没交点那就是跟背景板有交点
        let unit: f64 = 1.0 / r.b_direction.length();
        let unit_dir: Vec3 = r.b_direction * unit;
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
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
        Sphere {
            center: (Vec3::new(0.0, -101.5, -1.0)),
            r: (100.0),
        },
    ];

    let mut random: ThreadRng = rand::thread_rng();
    //产生物体
    //let center: Vec3 = Vec3::new(0.0, 0.0, -1.0);
    //let sphere: Sphere = Sphere::new(center, 0.5); //这里center就没了
    for j in 0..height {
        //对于每个像素，发出对应方向的光线
        for i in 0..width {
            let mut sum_pixel_color: [u32; 3] = [0; 3];
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
                let tmp_pixel_color: [u8; 3] = ray_color(r, &sphere_list);
                for l in 0..3 {
                    sum_pixel_color[l] += tmp_pixel_color[l] as u32;
                }
            }
            for element in &mut sum_pixel_color {
                *element /= data.sample_times as u32;
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
