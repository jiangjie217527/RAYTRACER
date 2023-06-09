pub use crate::util::unit_vec;
pub use crate::vec3::Vec3;
#[derive(Clone, Debug, PartialEq)]
pub struct Camera {
    pub viewport_height: f64,
    pub viewport_width: f64,

    pub origin: Vec3,
    pub lookat: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub vup: Vec3,
    pub fov: f64,

    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,

    pub len_radius: f64,
    pub focus_dist: f64,

    pub time1: f64,
    pub time2: f64,
}

impl Camera {
    pub fn new(
        ratio: f64,
        viewport_height: f64,
        origin: Vec3,
        lookat: Vec3,
        fov: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let w = unit_vec(origin - lookat);
        let u = unit_vec(Vec3::new(0.0, 1.0, 0.0).cross(w));
        let v = w.cross(u);
        let h = (fov / 2.0).tan();
        Self {
            u,
            v,
            w,
            viewport_height: viewport_height * h,
            viewport_width: viewport_height * ratio * h,
            origin,
            lookat,
            vup: Vec3::new(0.0, 1.0, 0.0),
            fov,
            time1: 0.0,
            time2: 1.0,
            len_radius: aperture / 2.0,
            focus_dist,
            horizontal: u * viewport_height * ratio * h * focus_dist,
            vertical: v * viewport_height * h * focus_dist,
            lower_left_corner: Vec3::zero()
                - u * viewport_height * h * ratio * focus_dist / 2.0
                - v * viewport_height * h * focus_dist / 2.0
                - w * focus_dist,
        }
    }
}
