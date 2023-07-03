pub use crate::aabb::{Aabb, BvhNode};
pub use crate::aarec::{Xyrect, Xzrect, Yzrect};
pub use crate::ray::Ray;
pub use crate::sphere::Sphere;
pub use crate::vec3::Vec3;
pub use crate::world::Object;
/*
pub fn add_box(p0: Vec3, p1: Vec3, object_list:&mut Vec<Object>,emit:[f64;3],tp:u8){
    object_list.push( Object::Xy(Xyrect::new(
        p0.x(),
        p1.x(),
        p0.y(),
        p1.y(),
        p1.z(),
        emit,
        tp
    )));
    object_list.push(Object::Xy(Xyrect::new(
        p0.x(),
        p1.x(),
        p0.y(),
        p1.y(),
        p0.z(),
        emit,
        tp
    )));
    object_list.push(Object::Xz(Xzrect::new(
        p0.x(),
        p1.x(),
        p0.z(),
        p1.z(),
        p1.y(),
        emit,
        tp
    )));
    object_list.push(Object::Xz(Xzrect::new(
        p0.x(),
        p1.x(),
        p0.z(),
        p1.z(),
        p0.y(),
                emit,
        tp
    )));
    object_list.push(Object::Yz(Yzrect::new(
        p0.y(),
        p1.y(),
        p0.z(),
        p1.z(),
        p1.x(),
                emit,
        tp
    )));
    object_list.push(Object::Yz(Yzrect::new(
        p0.y(),
        p1.y(),
        p0.z(),
        p1.z(),
        p0.x(),
                emit,
        tp
    )));
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct Boxx {
    box_min: Vec3,
    box_max: Vec3,
    pub sides: Vec<Object>,
    pub emit: [f64; 3],
}

impl Boxx {
    pub fn empty() -> Self {
        Self {
            box_min: Vec3::zero(),
            box_max: Vec3::zero(),
            sides: Vec::new(),
            //bvh:BvhNode::new(Sphere::empty_sphere()),
            emit: [0.0; 3],
        }
    }

    pub fn add(&mut self, obj: Object) {
        self.sides.push(obj);
    }
    pub fn new(p0: Vec3, p1: Vec3, emit: [f64; 3], tp: u8) -> Self {
        let mut object_list = vec![Object::Xy(Xyrect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            emit,
            tp,
        ))];
        // object_list.push( Object::Xy(Xyrect::new(
        //     p0.x(),
        //     p1.x(),
        //     p0.y(),
        //     p1.y(),
        //     p1.z(),
        //     emit,
        //     tp
        // )));
        object_list.push(Object::Xy(Xyrect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            emit,
            tp,
        )));
        object_list.push(Object::Xz(Xzrect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
            emit,
            tp,
        )));
        object_list.push(Object::Xz(Xzrect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
            emit,
            tp,
        )));
        object_list.push(Object::Yz(Yzrect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
            emit,
            tp,
        )));
        object_list.push(Object::Yz(Yzrect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
            emit,
            tp,
        )));

        Self {
            box_min: p0,
            box_max: p1,
            sides: object_list,
            emit,
        }
    }

    pub fn reset_box(&mut self) {
        let final_box = Aabb::bound_box(&self.sides);
        self.box_min = final_box.minimum;
        self.box_max = final_box.maximum;
    }

    pub fn bounding_box(&self) -> Aabb {
        //Aabb::new(self.box_min, self.box_max)
        Aabb::bound_box(&self.sides)
    }
    // pub fn info(&self){
    //     self.box_min.info();
    //     self.box_max.info();
    // }
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (f64, Object) {
        let mut t = f64::INFINITY;
        let mut obj = Object::empty();
        for i in &self.sides {
            match i {
                Object::Xy(o) => {
                    let tmp = o.hit(r, t_min, t_max);
                    if tmp < t {
                        t = tmp;
                        obj = Object::Xy(o.clone());
                    }
                }
                Object::Xz(o) => {
                    let tmp = o.hit(r, t_min, t_max);
                    if tmp < t {
                        t = tmp;
                        obj = Object::Xz(o.clone());
                    }
                }
                Object::Yz(o) => {
                    let tmp = o.hit(r, t_min, t_max);
                    if tmp < t {
                        t = tmp;
                        obj = Object::Yz(o.clone());
                    }
                } //忘了处理球的情况了
                Object::Sphere(s) => {
                    let tmp = s.hit_sphere(r, t_min, t_max);
                    if tmp < t {
                        //s.info();
                        t = tmp;
                        obj = Object::Sphere(s.clone());
                    }
                }
                _ => {}
            }
        }
        // let o = obj.clone();
        // if t != f64::INFINITY{
        //     match o {
        //         Object::Sphere(s)=>{
        //             s.info();
        //             r.info();
        //         },
        //         _=>{}
        //     }
        // }
        (t, obj)
    }

    pub fn normal(&self, r: &Ray) -> Vec3 {
        let mut n = Vec3::zero();
        let mut t = f64::INFINITY;
        for i in &self.sides {
            match i {
                Object::Xy(o) => {
                    let tmp = o.hit(r, 0.001, f64::INFINITY);
                    if tmp < t {
                        t = tmp;
                        n = o.normal(r);
                    }
                }
                Object::Xz(o) => {
                    let tmp = o.hit(r, 0.001, f64::INFINITY);
                    if tmp < t {
                        t = tmp;
                        n = o.normal(r);
                    }
                }
                Object::Yz(o) => {
                    let tmp = o.hit(r, 0.001, f64::INFINITY);
                    if tmp < t {
                        t = tmp;
                        n = o.normal(r);
                    }
                }
                _ => {}
            }
        }
        n
    }
}
