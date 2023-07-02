pub use crate::util::{color, random_f64_0_1};
pub use crate::vec3::Vec3;
use rand::{rngs::ThreadRng, Rng};
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

pub fn checher_color_value(n: Vec3) -> [f64; 3] {
    // let a = (u * 100.0) as i32;
    // let b= (v * 100.0) as i32;

    //let x = ((16000.0 * n.x()).sin()) * ((16000.0 * n.y()).sin()) * ((16000.0 * n.z()).sin());

    let y = ((10.0 * n.x()).sin()) * ((10.0 * n.y()).sin()) * ((10.0 * n.z()).sin());
    if y < 0.0 {
        //println!("0:{},{}",a,b);
        [0.9; 3]
    } else {
        //println!("1:{},{}",a,b);
        [0.2, 0.3, 0.1]
    }
}

pub struct Perlin {
    ranfloat: Vec<f64>,
    pub perm_x: Vec<i32>,
    pub perm_y: Vec<i32>,
    pub perm_z: Vec<i32>,
}
impl Perlin {
    // pub fn new()->Self{
    //     Self {
    //         ranfloat: vec![0.0; 0],
    //         perm_x: vec![0;0],
    //         perm_y: vec![0;0],
    //         perm_z: vec![0;0],
    //     }
    // }

    pub fn init() -> Self {
        let mut ranfloat: Vec<f64> = Vec::new();
        for _i in 0..256 {
            ranfloat.push(random_f64_0_1());
        }
        Self {
            ranfloat,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn perlin_generate_perm() -> Vec<i32> {
        let mut tmp = Vec::new();
        for i in 0..256 {
            tmp.push(i);
        }
        Self::permute(&mut tmp);
        tmp
    }

    pub fn permute(v: &mut Vec<i32>) {
        let mut random: ThreadRng = rand::thread_rng();
        for i in (1..255).rev() {
            let target = random.gen_range(0..i);

            v.swap(i, target);
        }
    }

    pub fn noise(&self, n: &Vec3) -> f64 {
        let i = (((n.x() * 4.0) as i32) & 255) as usize;
        let j = (((n.y() * 4.0) as i32) & 255) as usize;
        let k = (((n.z() * 4.0) as i32) & 255) as usize;

        self.ranfloat[self.perm_x[i] as usize ^ self.perm_y[j] as usize ^ self.perm_z[k] as usize]
    }
    /*
    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], _u: f64, _v: f64, _w: f64) -> f64 {
        let uu = _u * _u * (3. - 2. * _u);
        let vv = _v * _v * (3. - 2. * _v);
        let ww = _w * _w * (3. - 2. * _w);

        let mut accum = 0.;
        #[allow(clippy::needless_range_loop)]
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(_u - i as f64, _v - j as f64, _w - k as f64);
                    accum += (i as f64 * uu as f64 + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv as f64 + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww as f64 + (1.0 - k as f64) * (1. - ww))
                        * (c[i][j][k] * weight_v);
                }
            }
        }
        accum
    }

    pub fn turb(&self, p: &point3) -> f64 {
        let depth = 7;
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }
    */
}
