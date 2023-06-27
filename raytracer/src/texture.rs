pub use crate::util::color;
pub use crate::vec3::Vec3;

// pub struct Checker{
//     pub odd_color:[u8;3],
//     pub even_color:[u8;3],
// }
// impl Checker {
//     pub fn color_value(u:f64,v:f64)->[u8;3]{
//         if()
//     }
// }

// pub fn get_uv(n:Vec3)->(f64,f64){
//     let theta = (-n.y()).acos();
//     let phi = f64::atan2(-n.z(), n.x())+std::f64::consts::PI;
//     (theta/(2.0*std::f64::consts::PI),phi/std::f64::consts::PI)
//     //(theta,phi)
// }

pub fn checher_color_value(n: Vec3) -> [u8; 3] {
    // let a = (u * 100.0) as i32;
    // let b= (v * 100.0) as i32;

    let x = ((16000.0 * n.x()).sin()) * ((16000.0 * n.y()).sin()) * ((16000.0 * n.z()).sin());

    //let y = ((100.0*n.x()).sin())*((100.0*n.y()).sin())*((100.0*n.z()).sin());
    if x < 0.0 {
        //println!("0:{},{}",a,b);
        color(0.9, 0.9, 0.9)
    } else {
        //println!("1:{},{}",a,b);
        color(0.2, 0.3, 0.1)
    }
}
