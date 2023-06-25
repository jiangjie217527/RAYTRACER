use rand::Rng;

pub use crate::ray::Ray;
pub use crate::sphere::Sphere;
pub use crate::util::{fabs, fmax, fmin};
pub use crate::vec3::Vec3;
//因为要sort所以要排序的函数所以要order
use std::cmp::Ordering;
pub struct Aabb {
    pub minimum: Vec3,
    pub maximum: Vec3,
}

impl Aabb {
    pub fn new(minimum: Vec3, maximum: Vec3) -> Self {
        Self { minimum, maximum }
    }
    pub fn new_empty() -> Self {
        Self {
            minimum: (Vec3::zero()),
            maximum: (Vec3::zero()),
        }
    }
    //check whether the box was hit by the ray
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut t_min_mut = t_min;
        let mut t_max_mut = t_max;
        for i in 0..3 {
            let mut t0 = (self.maximum.lp(i) - r.a_origin.lp(i)) / r.b_direction.lp(i);
            let mut t1 = (self.minimum.lp(i) - r.a_origin.lp(i)) / r.b_direction.lp(i);
            if t1 < t0 {
                //t1更大,即远的交点
                std::mem::swap(&mut t1, &mut t0);
            }
            t_min_mut = fmax(t0, t_min_mut); //近的点
            t_max_mut = fmin(t1, t_max_mut); //远的点
            if t_max_mut <= t_min_mut {
                return false;
            }
        }
        true
    }
    //to merge two small box to a large one
    pub fn surround_box(b1: Aabb, b2: Aabb) -> Aabb {
        Self {
            minimum: Vec3::merge_min(&b1.minimum, &b2.minimum),
            maximum: Vec3::merge_max(&b1.maximum, &b2.maximum),
        }
    }
    //using surround box to get a big box of a list of sphere
    pub fn bound_box(sphere_list: &Vec<Sphere>) -> (Self, bool) {
        if sphere_list.is_empty() {
            return (Aabb::new_empty(), false);
        }

        let mut first_box = true;
        let mut output_box = Aabb::new_empty();
        for i in sphere_list {
            let (tmp_box, allow) = i.bound_box();
            if !allow {
                return (Aabb::new_empty(), false);
            }
            if !first_box {
                output_box = Aabb::surround_box(output_box, tmp_box);
            } else {
                output_box = tmp_box;
                first_box = false;
            }
        }
        (output_box, true)
    }
    //compare the box return a Ordering
    pub fn box_compare(a: Aabb, b: Aabb, axis: u8) -> Ordering {
        if a.minimum.lp(axis) > b.minimum.lp(axis) {
            Ordering::Greater
        } else if a.minimum.lp(axis) < b.minimum.lp(axis) {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
    // pub fn info(&self) {
    //     println!("minimum:");
    //     self.minimum.info();
    //     println!("maximum:");
    //     self.maximum.info();
    // }
}
pub struct BvhNode {
    pub bd_box: Aabb,
    pub left: Option<Box<BvhNode>>,
    pub right: Option<Box<BvhNode>>,
    pub sphere: Sphere,
}

impl BvhNode {
    // pub fn info(&self) {
    //     self.bd_box.info();
    //     match self.left {
    //         Some(ref x) => {
    //             x.info();
    //         }
    //         _ => {}
    //     }
    //     match self.right {
    //         Some(ref x) => {
    //             x.info();
    //         }
    //         _ => {}
    //     }
    // }
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, f64, Sphere) {
        //if the big box didn't hit the ray then retrn false

        if !self.bd_box.hit(r, t_min, t_max) {
            return (false, f64::INFINITY, Sphere::empty_sphere());
        }
        //if yes then check out which box and return the hit point t on ray
        let mut hitleft = false;
        let mut hitright = false;
        let mut t_left = f64::INFINITY;
        let mut t_right = f64::INFINITY;
        let mut sphere_left = Sphere::empty_sphere();
        let mut sphere_right = Sphere::empty_sphere();
        // match self.left {
        //     Some(ref x) => {
        //         (hitleft, t_left, sphere_left) = x.hit(r, t_min, t_max);
        //     }
        //     _ => {}
        // }
        // match self.right {
        //     Some(ref x) => {
        //         (hitright, t_right, sphere_right) = x.hit(r, t_min, t_left);
        //     }
        //     _ => {}
        // }
        if let Some(ref x) = self.left {
            (hitleft, t_left, sphere_left) = x.hit(r, t_min, t_max);
        }
        if let Some(ref x) = self.right {
            (hitright, t_right, sphere_right) = x.hit(r, t_min, t_left);
        }
        if !hitleft && !hitright && self.sphere.r != 0.0 {
            let mut tmp;
            let delta;
            (tmp, delta) = self.sphere.hit_sphere(r.clone());
            if tmp < t_min || tmp > t_max {
                tmp += delta;
                //可能影响折射
                if tmp < t_min || tmp > t_max {
                    tmp = f64::INFINITY;
                }
            }
            if tmp < t_max && tmp > t_min {
                (true, tmp, self.sphere.clone())
            } else {
                (false, tmp, Sphere::empty_sphere())
            }
        } else if t_left < t_right {
            (hitleft || hitright, t_left, sphere_left)
        } else {
            (hitleft || hitright, t_right, sphere_right)
        }
    }

    pub fn new(sphere: &Sphere) -> Self {
        let (bd_box, _) = sphere.bound_box();
        Self {
            bd_box: (bd_box),
            left: (None),
            right: (None),
            sphere: sphere.clone(),
        }
    }
    pub fn build(&mut self, mut sphere_list: Vec<Sphere>, start: usize, end: usize) {
        //println!("start:{},end:{}", start, end);
        let axis = rand::thread_rng().gen_range(0..3);
        let object_span = end - start;
        if object_span == 0 {
            return;
        }

        (self.bd_box, _) = Aabb::bound_box(&sphere_list);

        if object_span == 1 {
            self.sphere = sphere_list[0].clone();
        } else if object_span == 2 {
            let (box_1, _) = sphere_list[0].bound_box();
            let (box_2, _) = sphere_list[1].bound_box();
            if Aabb::box_compare(box_1, box_2, axis) == Ordering::Less {
                self.left = Some(Box::new(BvhNode::new(&sphere_list[0])));
                self.right = Some(Box::new(BvhNode::new(&sphere_list[1])));
            } else {
                self.left = Some(Box::new(BvhNode::new(&sphere_list[1])));
                self.right = Some(Box::new(BvhNode::new(&sphere_list[0])));
            }
        } else {
            sphere_list.sort_by(|a, b| {
                let (box_a, _) = a.bound_box();
                let (box_b, _) = b.bound_box();
                Aabb::box_compare(box_a, box_b, axis)
            });
            let mid = start + object_span / 2;
            let mut left_list = vec![];
            let mut right_list = vec![];
            for i in start..mid {
                left_list.push(sphere_list[i - start].clone());
            }
            for i in mid..end {
                right_list.push(sphere_list[i - start].clone());
            }
            self.left = Some(Box::new(BvhNode::new(&Sphere::empty_sphere())));
            self.right = Some(Box::new(BvhNode::new(&Sphere::empty_sphere())));
            (*self.left.as_mut().unwrap()).build(left_list, start, mid);
            (*self.right.as_mut().unwrap()).build(right_list, mid, end);
        }
    }
}