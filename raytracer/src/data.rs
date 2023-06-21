pub use crate::vec3::Vec3;
#[derive(Clone, Debug, PartialEq)]
pub struct Data {
    pub height: usize,
    pub width: usize,
    pub quality: u8,
    pub gamma:u8,

    pub sample_times: u32,
    pub depth:u32,

    pub ratio: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,

    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Data {
    pub fn new(
        height: usize,
        width: usize,
        quality: u8,
        gamma:u8,
        sample_times: u32,
        depth:u32,
        ratio: f64,
        viewport_height: f64,
    ) -> Self {
        Self {
            height,
            width,
            quality,
            gamma,
            depth,
            sample_times,
            ratio,
            viewport_height,
            origin: Vec3::zero(),
            viewport_width: viewport_height * ratio,
            horizontal: Vec3::new(viewport_height, 0.0, 0.0),
            vertical: Vec3::new(0.0, viewport_height * ratio, 0.0),
            lower_left_corner: Vec3::zero()
                + Vec3::new(-0.5 * viewport_height * ratio, -0.5 * viewport_height, -1.0),
        }
    }
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
