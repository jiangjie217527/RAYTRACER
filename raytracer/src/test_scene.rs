pub use crate::camera::Camera;
pub use crate::data::Data;
pub use crate::sphere::Sphere;
pub use crate::vec3::Vec3;

pub fn init_debug(data: &Data) -> Camera {
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);
    Camera::new(
        data.width as f64 / data.height as f64,
        2.0,
        origin,
        lookat,
        std::f64::consts::PI / 2.0,
        0.0,
        1.0,
    )
}

pub fn sphere_debug() -> Vec<Sphere> {
    vec![
        Sphere::new(
            Vec3::new(0.0, 0.0, -1000.0),
            Vec3::new(0.0, 0.0, -1000.0),
            0.0,
            1.0,
            999.5,
            1,
            [0; 3],
            0.0,
            0.0,
            1,
        ),
        //Sphere::new(center, destinity, time1, time2, r, tp, color, fuzz, etia, texture_type)
    ]
}

pub fn init_debug2(data: &Data) -> Camera {
    let origin = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    Camera::new(
        data.width as f64 / data.height as f64,
        2.0,
        origin,
        lookat,
        std::f64::consts::PI / 9.0,
        0.0,
        10.0,
    )
}

pub fn sphere_debug2() -> Vec<Sphere> {
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
        //Sphere::new(center, destinity, time1, time2, r, tp, color, fuzz, etia, texture_type)
    ]
}
