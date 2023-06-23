pub use crate::sphere::Sphere;
pub use crate::util::color;
pub use crate::util::unit_vec;
pub use crate::vec3::Vec3;
use rand::{rngs::ThreadRng, Rng};

#[derive(Clone, Debug, PartialEq)]
pub struct Data {
    pub height: usize,
    pub width: usize,
    pub quality: u8,
    pub gamma: u8,

    pub sample_times: u32,
    pub depth: u32,

    pub sphere_list: Vec<Sphere>,
}

impl Data {
    pub fn new(
        height: usize,
        width: usize,
        quality: u8,
        gamma: u8,
        sample_times: u32,
        depth: u32,
    ) -> Self {
        Self {
            height,
            width,
            quality,
            gamma,
            depth,
            sample_times,
            sphere_list: Vec::new(),
        }
    }
}

pub fn init() -> Vec<Sphere> {
    let mut v = vec![
        Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, 1, [128, 0, 0], 0.0, 0.0),
        Sphere {
            center: (Vec3::new(0.0, -1000.0, 0.0)),
            r: (1000.0),
            tp: (1),
            color: ([0, 0, 128]),
            fuzz: (0.0),
            etia: (0.0),
        },
        Sphere {
            center: (Vec3::new(-4.0, 1.0, 0.0)),
            r: (1.0),
            tp: (3),
            color: ([0, 128, 128]),
            fuzz: (0.5),
            etia: (1.5),
        },
        Sphere {
            center: (Vec3::new(4.0, 1.0, 0.0)),
            r: (1.0),
            tp: (2),
            color: ([0, 128, 0]),
            fuzz: (0.2),
            etia: (0.0),
        },
        Sphere::new(Vec3::new(0.0, 0.0, 0.0), 0.5, 1, [255, 0, 0], 0.0, 0.0),
    ];
    let mut random: ThreadRng = rand::thread_rng();
    for i in -11..11 {
        for j in -11..11 {
            let choose = random.gen::<f64>();
            let center = Vec3::new(
                i as f64 + 0.9 * random.gen::<f64>(),
                0.2,
                j as f64 + 0.9 * random.gen::<f64>(),
            );
            let color = color(
                random.gen::<f64>(),
                random.gen::<f64>(),
                random.gen::<f64>(),
            );

            if choose < 0.8 {
                v.push(Sphere {
                    tp: (1),
                    center: (center),
                    r: (0.2),
                    color: (color),
                    fuzz: (0.2),
                    etia: (0.0),
                });
            } else if choose < 0.95 {
                v.push(Sphere {
                    tp: (2),
                    center: (center),
                    r: (0.2),
                    color: (color),
                    fuzz: (random.gen_range(0.0..0.5)),
                    etia: (0.0),
                });
            } else {
                v.push(Sphere {
                    tp: (3),
                    center: (center),
                    r: (0.2),
                    color: (color),
                    fuzz: (0.2),
                    etia: (1.5),
                })
            }
        }
    }
    v
}

//     let height: usize = 800;
// let width: usize = 800;
// let path:&str = "output/test.jpg";
// let quality:usize = 60; // From 0 to 100, suggested value: 60

// let ratio:f64 = 1.0;
// let viewport_height:f64 = 2.0;
// let viewport_width:f64 = viewport_height*ratio;

// let origin:Vec3 = Vec3::ones();
// let lower_left_corner:Vec3 = origin + Vec3::new(-0.5*viewport_width,-0.5*viewport_height,-1.0);
// let horizontal:Vec3 = Vec3::new(viewport_height,0.0,0.0);
// let vertical:Vec3 = Vec3::new(0.0,viewport_width,0.0);

// let center:Vec3 = Vec3::new(0.0,0.0,-1.0);
// let sphere:Vec3 = Sphere::new(center,0.5);
