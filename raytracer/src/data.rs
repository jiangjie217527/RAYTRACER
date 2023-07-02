pub use crate::aarec::{Xyrect, Xzrect, Yzrect};
pub use crate::boxx::Boxx;
pub use crate::rotate::Rotatey;
pub use crate::sphere::Sphere;
pub use crate::util::color;
pub use crate::util::unit_vec;
pub use crate::vec3::Vec3;
pub use crate::world::Object;
// use rand::{rngs::ThreadRng, Rng};

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
/*
pub fn init() -> Vec<Sphere> {
    let mut v = vec![
        Sphere {
            //middle
            center: (Vec3::new(0.0, 1.0, 0.0)),
            destinity: (Vec3::new(0.0, 1.0, 0.0)),
            r: (1.0),
            tp: (3),
            color: ([128, 128, 128]),
            fuzz: (0.0),
            etia: (1.5),
            time1: (0.0),
            time2: (1.0),
            texture_type: (0),
        },
        Sphere {
            //ground
            center: (Vec3::new(0.0, -1000.0, 0.0)),
            destinity: (Vec3::new(0.0, -1000.0, 0.0)),
            r: (1000.0),
            tp: (1),
            color: ([128, 128, 128]),
            fuzz: (0.0),
            etia: (0.0),
            time1: (0.0),
            texture_type: (2),
            time2: (1.0),
        },
        Sphere {
            //diff
            center: (Vec3::new(-4.0, 1.0, 0.0)),
            destinity: (Vec3::new(-4.0, 1.0, 0.0)),
            r: (1.0),
            tp: (1),
            color: ([102, 51, 25]),
            fuzz: (0.3),
            etia: (1.5),
            time1: (0.0),
            texture_type: (0),
            time2: (1.0),
        },
        Sphere {
            //metal
            center: (Vec3::new(4.0, 1.0, 0.0)),
            destinity: (Vec3::new(4.0, 1.0, 0.0)),
            r: (1.0),
            tp: (2),
            color: ([178, 153, 128]),
            fuzz: (0.0),
            etia: (1.5),
            time1: (0.0),
            texture_type: (0),
            time2: (1.0),
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
                    center: (center.clone()),
                    destinity: (center + Vec3::new(0.0, 0.0, 0.0)),
                    r: (0.2),
                    color: (color),
                    fuzz: (0.2),
                    etia: (0.0),
                    time1: (0.0),
                    texture_type: (0),
                    time2: (1.0),
                });
            } else if choose < 0.95 {
                let color = color(
                    random.gen_range(0.5..1.0),
                    random.gen_range(0.5..1.0),
                    random.gen_range(0.5..1.0),
                );
                v.push(Sphere {
                    tp: (2),
                    center: (center.clone()),
                    destinity: (center + Vec3::new(0.0, 0.0, 0.0)),
                    r: (0.2),
                    color: (color),
                    fuzz: (random.gen_range(0.0..0.5)),
                    etia: (0.0),
                    texture_type: (0),
                    time1: (0.0),
                    time2: (1.0),
                });
            } else {
                v.push(Sphere {
                    tp: (3),
                    center: (center.clone()),
                    destinity: (center + Vec3::new(0.0, 0.0, 0.0)),
                    r: (0.2),
                    color: ([0; 3]),
                    fuzz: (0.2),
                    texture_type: (0),
                    etia: (1.5),
                    time1: (0.0),
                    time2: (1.0),
                })
            }
        }
    }
    v
}


pub fn two_perlin_spheres() -> Vec<Sphere> {
    vec![
        Sphere {
            //ground
            center: (Vec3::new(0.0, -1000.0, 0.0)),
            destinity: (Vec3::new(0.0, -1000.0, 0.0)),
            r: (1000.0),
            tp: (1),
            color: ([128, 128, 128]),
            fuzz: (0.0),
            etia: (0.0),
            time1: (0.0),
            texture_type: (2),
            time2: (1.0),
        },
        Sphere {
            center: (Vec3::new(0.0, 2.0, 0.0)),
            destinity: (Vec3::new(0.0, 2.0, 0.0)),
            r: (2.0),
            tp: (1),
            color: ([128, 128, 128]),
            fuzz: (0.0),
            etia: (0.0),
            time1: (0.0),
            texture_type: (2),
            time2: (1.0),
        },
    ]
}


pub fn earth() -> Vec<Sphere> {
    vec![Sphere {
        center: (Vec3::new(0.0, 0.0, 0.0)),
        destinity: (Vec3::new(0.0, 0.0, 0.0)),
        r: (2.0),
        tp: (1),
        color: ([128, 128, 128]),
        fuzz: (0.0),
        etia: (0.0),
        time1: (0.0),
        texture_type: (3),
        time2: (1.0),
    }
    ]
}
*/
/*
pub fn ty() -> Vec<Object> {
    vec![
        Object::Sphere(Sphere {
            center: (Vec3::new(0.0, 2.0, 0.0)),
            destinity: (Vec3::new(0.0, 2.0, 0.0)),
            r: (2.0),
            tp: (1),
            color: ([128, 128, 128]),
            emit: ([0.0; 3]),
            fuzz: (0.0),
            etia: (0.0),
            time1: (0.0),
            texture_type: (2),
            time2: (1.0),
        }),
        Object::Sphere(Sphere {
            center: (Vec3::new(0.0, 7.0, 0.0)),
            destinity: (Vec3::new(0.0, 7.0, 0.0)),
            r: (2.0),
            tp: (4),
            color: ([128, 128, 128]),
            emit: ([4.0; 3]),
            fuzz: (0.0),
            etia: (0.0),
            time1: (0.0),
            texture_type: (2),
            time2: (1.0),
        }),
        Object::Sphere(Sphere {
            //ground
            center: (Vec3::new(0.0, -1000.0, 0.0)),
            destinity: (Vec3::new(0.0, -1000.0, 0.0)),
            r: (1000.0),
            tp: (1),
            color: ([128, 128, 128]),
            fuzz: (0.0),
            etia: (0.0),
            time1: (0.0),
            texture_type: (2),
            time2: (1.0),
            emit: ([0.0; 3]),
        }),
        Object::Xy(Xyrect::new(3.0, 5.0, 1.0, 3.0, -2.0, 1)),
        Object::Xz(Xzrect::new(-2000.0, -2000.0, -2000.0, -2000.0, -2000.0, 1)),
        Object::Yz(Yzrect::new(-2000.0, -2000.0, -2000.0, -2000.0, -2000.0, 1)),
    ]
}
*/

pub fn cornell_box() -> Vec<Object> {
    let red = [0.65, 0.05, 0.05];
    let white = [0.73; 3];
    let green = [0.12, 0.45, 0.15];
    let light = [15.0; 3];
    let mut v = vec![
        Object::Yz(Yzrect::new(0.0, 555.0, 0.0, 555.0, 555.0, green, 1)),
        Object::Yz(Yzrect::new(0.0, 555.0, 0.0, 555.0, 0.0, red, 1)),
        Object::Xz(Xzrect::new(213.0, 343.0, 227.0, 332.0, 554.0, light, 2)),
        Object::Xz(Xzrect::new(0.0, 555.0, 0.0, 555.0, 0.0, white, 1)),
        Object::Xz(Xzrect::new(0.0, 555.0, 0.0, 555.0, 555.0, white, 1)),
        Object::Xy(Xyrect::new(0.0, 555.0, 0.0, 555.0, 555.0, white, 1)),
    ];
    //add_box(Vec3::new(130.0, 0.0, 65.0), Vec3::new(295.0, 165.0, 230.0), &mut v, white, 1);
    //v.push(Object::Bx(Boxx::new(Vec3::new(265.0, 0.0, 295.0), Vec3::new(430.0, 330.0, 460.0), white, 1)));
    v.push(Object::Rt(Rotatey::new(
        -std::f64::consts::PI / 10.0,
        Boxx::new(
            Vec3::new(130.0, 0.0, 65.0),
            Vec3::new(295.0, 165.0, 230.0),
            white,
            1,
        ),
    )));
    v.push(Object::Rt(Rotatey::new(
        std::f64::consts::PI / 12.0,
        Boxx::new(
            Vec3::new(205.0, 0.0, 295.0),
            Vec3::new(370.0, 330.0, 460.0),
            white,
            1,
        ),
    )));
    v.push(Object::Bx(Boxx::new(
        Vec3::new(205.0, 0.0, -1295.0),
        Vec3::new(370.0, 330.0, -1460.0),
        white,
        1,
    )));
    v
}
// objects.add(make_shared<yz_rect>(0, 555, 0, 555, 555, green));
// objects.add(make_shared<yz_rect>(0, 555, 0, 555, 0, red));
// objects.add(make_shared<xz_rect>(213, 343, 227, 332, 554, light));
// objects.add(make_shared<xz_rect>(0, 555, 0, 555, 0, white));
// objects.add(make_shared<xz_rect>(0, 555, 0, 555, 555, white));
// objects.add(make_shared<xy_rect>(0, 555, 0, 555, 555, white));
