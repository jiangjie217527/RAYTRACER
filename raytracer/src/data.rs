pub use crate::aarec::{Xyrect, Xzrect, Yzrect};
pub use crate::boxx::Boxx;
pub use crate::rotate::{Rotatey,Translate};
pub use crate::constant_medium::Fog;
pub use crate::sphere::Sphere;
pub use crate::util::color;
pub use crate::util::{unit_vec,random_cen_165,random_f64_101};
pub use crate::vec3::Vec3;
pub use crate::world::Object;
pub use crate::ray::Ray;
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
    let offset2 = Vec3::new(265.0,0.0,295.0);
    let offset1 = Vec3::new(130.0,0.0,65.0);
    v.push(Object::Fg(Fog::new(Translate::new(Rotatey::new(
        -std::f64::consts::PI / 10.0,
        Boxx::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(165.0,165.0,165.0),
            white,
            1,
        ),
    )
,offset1),0.01,[1.0;3])));
    v.push(Object::Fg(Fog::new(Translate::new(Rotatey::new(
        std::f64::consts::PI / 12.0,
        Boxx::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(165.0, 330.0, 165.0),
            white,
            1,
        ),
    ),offset2),0.01,[0.0;3])));
    v
}
*/


pub fn final_scene()-> Vec<Object>{
    let mut v=Vec::new();
    let zero = Vec3::zero();
    let ground = [0.48, 0.83, 0.53];
    for i in 0..20{
        for j in 0..20{
            let w = 100.0;
            let x0 = -1000.0 + i as f64*w;
            let z0 = -1000.0 + j as f64*w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_f64_101();
            let z1 = z0 + w;
            v.push(Object::Tr(Translate::new(Rotatey::new(0.0, Boxx::new(Vec3::new(x0, y0, z0), Vec3::new(x1, y1, z1), ground, 1)), zero)))
        }
    }

    let light = [7.0;3];
    v.push(Object::Xz(Xzrect::new(123.0, 423.0, 147.0, 412.0, 554.0, light,2)));
    let center1=Vec3::new(400.0, 400.0, 200.0);
    let center2=center1+Vec3::new(30.0, 0.0, 0.0);
    
    let center3 = Vec3::new(260.0, 150.0, 45.0);
    let center4 = Vec3::new(0.0, 150.0, 145.0);
    let center5 = Vec3::new(360.0,150.0,145.0);
    let center6 = Vec3::zero();
    
    let center7 = Vec3::new(400.0,200.0,400.0);
    let center8 = Vec3::new(220.0,280.0,300.0);

    v.push(Object::Sphere(Sphere { tp: (1), center: (center1), destinity: (center2), time1: (0.0), time2: (1.0), r: (50.0), color: ([178, 76, 25]), emit: ([0.0;3]), fuzz: (0.0), etia: (0.0), texture_type: (0) }));
    v.push(Object::Sphere(Sphere { tp: (3), center: (center3), destinity: (center3), time1: (0.0), time2: (1.0), r: (50.0), color: ([0;3]), emit: ([0.0;3]), fuzz: (0.0), etia: (1.5), texture_type: (0) }));
    v.push(Object::Sphere(Sphere { tp: (2), center: (center4), destinity: (center4), time1: (0.0), time2: (1.0), r: (50.0), color: ([204, 204,229]), emit: ([0.0;3]), fuzz: (1.0), etia: (0.0), texture_type: (0) }));

    v.push(Object::Sphere(Sphere { tp: (3), center: (center5), destinity: (center5), time1: (0.0), time2: (1.0), r: (70.0), color: ([0;3]), emit: ([1.0;3]), fuzz: (0.0), etia: (1.5), texture_type: (0) }));
    v.push(Object::Fg(Fog::new(Sphere { tp: (3), center: (center5), destinity: (center5), time1: (0.0), time2: (1.0), r: (70.0), color: ([0;3]), emit: ([0.0;3]), fuzz: (0.0), etia: (1.5), texture_type: (0) }, 0.2, [0.2, 0.4, 0.9])));
    v.push(Object::Fg(Fog::new(Sphere { tp: (3), center: (center6), destinity: (center6), time1: (0.0), time2: (1.0), r: (5000.0), color: ([0;3]), emit: ([0.0;3]), fuzz: (0.0), etia: (1.5), texture_type: (0) }, 0.0001, [1.0;3])));
    // let t =Sphere { tp: (3), center: (center6), destinity: (center6), time1: (0.0), time2: (1.0), r: (5000.0), color: ([0;3]), emit: ([0.0;3]), fuzz: (0.0), etia: (1.5), texture_type: (0) }.hit_sphere(&Ray { a_origin: (Vec3::new(478.0, 278.0, -600.0)), b_direction: (Vec3::new(278.0, 278.0, 0.0)-Vec3::new(478.0, 278.0, -600.0)), time: (0.0) }, -f64::INFINITY, f64::INFINITY);
    v.push(Object::Sphere(Sphere { tp: (1), center: (center7), destinity: (center7), time1: (0.0), time2: (1.0), r: (100.0), color: ([0;3]), emit: ([0.0;3]), fuzz: (0.0), etia: (0.0), texture_type: (3) }));
    v.push(Object::Sphere(Sphere { tp: (1), center: (center8), destinity: (center8), time1: (0.0), time2: (1.0), r: (80.0), color: ([0;3]), emit: ([0.0;3]), fuzz: (0.0), etia: (0.0), texture_type: (2) }));

    let white: [u8; 3] = [186;3];
    let mut sphere_box = Boxx::empty();
    //let c = random_cen_165()+Vec3::new(-100.0,270.0,395.0);
    let offset = Vec3::new(-100.0,270.0,395.0);
    //v.push(Object::Sphere(Sphere { tp: (4), center: (c), destinity: (c), time1: (0.0), time2: (1.0), r: (10.0), color: (white), emit: ([7.0;3]), fuzz: (0.0), etia: (0.0), texture_type: (0) }));
    for _ in 0..1000{
        let c = random_cen_165();
        sphere_box.add(Object::Sphere(Sphere { tp: (1), center: (c), destinity: (c), time1: (0.0), time2: (1.0), r: (10.0), color: (white), emit: ([1.0;3]), fuzz: (0.0), etia: (0.0), texture_type: (0) }));
    }
    sphere_box.reset_box();
    v.push(Object::Tr(Translate::new(Rotatey::new(std::f64::consts::PI/12.0,sphere_box), offset)));

    v
}
//Vec3::new(-100.0,270.0,395.0)