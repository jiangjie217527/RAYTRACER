pub use crate::rotate::{Translate};
pub use crate::aabb::Aabb;
pub use crate::ray::Ray;
pub use crate::util::random_f64_0_1;
pub use crate::vec3::Vec3;
pub use crate::sphere::Sphere;

#[derive(Clone, Debug, PartialEq)]
pub struct Fog {
    boundary:Sphere,
    neg_inv_density: f64,
    pub color:[f64;3],
}

impl Fog {
    // pub fn new1(b: Option<Arc<dyn Hiitable>>, d: f64, a: Option<Arc<dyn Texture>>) -> Self {
    //     Self {
    //         boundary: b,
    //         neg_inv_density: -1.0 / d,
    //         phase_function: Some(Arc::new(Isotropic::new2(a))),
    //     }
    // }
    pub fn new(boundary: Sphere, d: f64, color: [f64;3]) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / d,
            color,
        }
    }
    pub fn bounding_box(&self) -> Aabb {
        self.boundary
            .bound_box()
    }

    pub fn hit(
        &self,
        r: &Ray,
        t_min: f64,
        t_max: f64,
    ) -> f64 {
        // let enabledebug = false;
        // let debugging = enabledebug && random_f64() < 0.00001;
        //println!("{}",self.boundary.r);
        //r.info();
        let mut t1 =self
        .boundary
        .hit_sphere(r, -f64::INFINITY, f64::INFINITY);
        //println!("{}",t1);
        if t1==f64::INFINITY{
            return f64::INFINITY;
        }
        let mut t2 = self
        .boundary
        .hit_sphere(r, t1+0.0001, f64::INFINITY);

        if t2 == f64::INFINITY{
            return f64::INFINITY;
        }
        //println!("{},{}",t1,t2);
        // if debugging {
        //     println!("\nt_min={},\nt_max={}", rec1.t, rec2.t);
        // }

        if t1 < t_min {
           t1 = t_min;
        }

        if t2 > t_max {
            t2 = t_max;
        }

        if t1 >= t2 {
            return f64::INFINITY;
        }

        let ray_length = r.b_direction.length();
        let distance_inside_boundary = (t2 - t1) * ray_length;
        let hit_distance = self.neg_inv_density * random_f64_0_1().ln();
        
        if hit_distance > distance_inside_boundary {
            return f64::INFINITY;
        }
        
        t1+hit_distance / ray_length
        // rec.t = rec1.t + ;
        // rec.point3 = r.at(rec.t);

        // if debugging {
        //     println!("hit_distance = {}\n", hit_distance);
        //     println!("rec.t = {}\n", rec.t);
        //     println!(
        //         "rec.p.x = {},rec.p.y = {}. rec.p.z = {}.",
        //         rec.point3.x(),
        //         rec.point3.y(),
        //         rec.point3.z()
        //     );
        // }

        // rec.normal = Vec3::new(1.0, 0.0, 0.0);
        // rec.front_size = true;
        // rec.mat = self.phase_function.clone();

        // true
    }

    pub fn p_nor(&self,t: f64, r: &Ray)->(Vec3,Vec3){
        (r.at(t),Vec3::new(1.0,0.0,0.0))
    }
}


