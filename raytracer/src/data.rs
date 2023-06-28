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
        Sphere {
            //middle
            center: (Vec3::new(0.0, 1.0, 0.0)),
            r: (1.0),
            tp: (3),
            color: ([128, 128, 128]),
            fuzz: (0.0),
            etia: (1.5),
        },
        Sphere {
            //ground
            center: (Vec3::new(0.0, -1000.0, 0.0)),
            r: (1000.0),
            tp: (1),
            color: ([128, 128, 128]),
            fuzz: (0.0),
            etia: (0.0),
        },
        Sphere {
            //diff
            center: (Vec3::new(-4.0, 1.0, 0.0)),
            r: (1.0),
            tp: (1),
            color: ([102, 51, 25]),
            fuzz: (0.3),
            etia: (1.5),
        },
        Sphere {
            //metal
            center: (Vec3::new(4.0, 1.0, 0.0)),
            r: (1.0),
            tp: (2),
            color: ([178, 153, 128]),
            fuzz: (0.0),
            etia: (1.5),
        },
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

            if choose < 0.8 {
                let color = color(
                    random.gen::<f64>(),
                    random.gen::<f64>(),
                    random.gen::<f64>(),
                );
                v.push(Sphere {
                    tp: (1),
                    center: (center),
                    r: (0.2),
                    color: (color),
                    fuzz: (0.2),
                    etia: (0.0),
                });
            } else if choose < 0.95 {
                let color = color(
                    random.gen_range(0.5..1.0),
                    random.gen_range(0.5..1.0),
                    random.gen_range(0.5..1.0),
                );
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
                    color: ([0; 3]),
                    fuzz: (0.2),
                    etia: (1.5),
                })
            }
        }
    }
    v
}
