use crate::color::write_color;
pub use crate::data::Data;
pub use crate::ray::Ray;
pub use crate::sphere::Sphere;
pub use crate::vec3::Vec3;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;

fn color(x: f64, y: f64, z: f64) -> [u8; 3] {
    let a = [(255.0 * x) as u8, (255.0 * y) as u8, (255.0 * z) as u8];
    a
}

fn ray_color(r: Ray, sphere: Sphere) -> [u8; 3] {
    let t = sphere.hit_sphere(r.clone());
    if t > 0.0 {
        let n: Vec3 = r.at(t) - Vec3::new(0.0, 0.0, -1.0);
        let unit: f64 = 1.0 / n.length();
        let n: Vec3 = n * unit;

        color(0.5 * n.x() + 1.0, 0.5 * n.y() + 1.0, 0.5 * n.z() + 1.0)
    } else {
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

pub fn render(data: Data, bar: ProgressBar) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img: RgbImage = ImageBuffer::new(
        data.width.try_into().unwrap(),
        data.height.try_into().unwrap(),
    );
    let center: Vec3 = Vec3::new(0.0, 0.0, -1.0);
    let sphere: Sphere = Sphere::new(center, 0.5);
    for j in 0..data.height {
        for i in 0..data.width {
            let u = (i as f64) / ((data.width - 1) as f64);
            let v = (j as f64) / ((data.height - 1) as f64);
            let dir = data.lower_left_corner.clone()
                + data.horizontal.clone() * u
                + data.vertical.clone() * v;
            let r = Ray::new(data.origin.clone(), dir.clone());
            let pixel_color: [u8; 3] = ray_color(r, sphere.clone());
            write_color(pixel_color, &mut img, i, j);

            bar.inc(1);
        }
    }
    bar.finish();
    return img;
}
