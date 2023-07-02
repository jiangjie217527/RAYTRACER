pub use crate::util::{color, random_vec3};
pub use crate::vec3::Vec3;
use image::ImageBuffer;
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

pub fn get_uv(n: Vec3) -> (f64, f64) {
    let theta = (-n.y()).acos();
    let phi = f64::atan2(-n.z(), n.x()) + std::f64::consts::PI;
    (
        phi / (2.0 * std::f64::consts::PI),
        theta / std::f64::consts::PI,
    )
    //(theta,phi)
}

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
    ranvec: Vec<Vec3>,
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
        let mut ranvec: Vec<Vec3> = Vec::new();
        for _i in 0..256 {
            ranvec.push(random_vec3());
        }
        Self {
            ranvec,
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

    pub fn permute(v: &mut [i32]) {
        let mut random: ThreadRng = rand::thread_rng();
        for i in (1..255).rev() {
            let target = random.gen_range(0..i);

            v.swap(i, target);
        }
    }

    // pub fn noise(&self, n: &Vec3) -> f64 {

    //     let i = (((n.x() * 4.0) as i32) & 255) as usize;
    //     let j = (((n.y() * 4.0) as i32) & 255) as usize;
    //     let k = (((n.z() * 4.0) as i32) & 255) as usize;

    //     self.ranfloat[self.perm_x[i] as usize ^ self.perm_y[j] as usize ^ self.perm_z[k] as usize]
    // }
    pub fn noise(&self, n: &Vec3) -> f64 {
        let mut u = n.x() - n.x().floor();
        let mut v = n.y() - n.y().floor();
        let mut w = n.z() - n.z().floor();
        u = u * u * (3.0 - 2.0 * u);
        v = v * v * (3.0 - 2.0 * v);
        w = w * w * (3.0 - 2.0 * w);
        let i = n.x().floor() as i32;
        let j = n.y().floor() as i32;
        let k = n.z().floor() as i32;

        let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        for (x, item) in c.clone().iter().enumerate() {
            for (y, item2) in item.iter().enumerate() {
                for (z, _) in item2.iter().enumerate() {
                    c[x][y][z] = self.ranvec[self.perm_x[(i + x as i32) as usize & 255] as usize
                        ^ self.perm_y[(j + y as i32) as usize & 255] as usize
                        ^ self.perm_z[(k + z as i32) as usize & 255] as usize];
                }
            }
        }
        // for x in 0..2 {
        //     for y in 0..2 {
        //         for z in 0..2 {
        //             c[x as usize][y as usize][z as usize] =
        //                 self.ranfloat[self.perm_x[(i + x) as usize & 255] as usize
        //                     ^ self.perm_y[(j + y) as usize & 255] as usize
        //                     ^ self.perm_z[(k + z) as usize & 255] as usize];
        //         }
        //     }
        // }

        Perlin::trilinear_interp(c, u, v, w)
    }
    pub fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for (i, item) in c.clone().iter().enumerate() {
            for (j, item2) in item.iter().enumerate() {
                for (k, _) in item2.iter().enumerate() {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * u + (1 - i) as f64 * (1.0 - u))
                        * (j as f64 * v + (1 - j) as f64 * (1.0 - v))
                        * (k as f64 * w + (1 - k) as f64 * (1.0 - w))
                        * (c[i][j][k] * weight_v);
                }
            }
        }

        accum
    }
    pub fn turb(&self, p: &Vec3) -> f64 {
        let depth = 7;
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;
        let mut i = 0;
        while i < depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
            i += 1;
        }

        accum.abs()
    }
}

/*
pub struct ImageTexture {
    data: Vec<u8>,
    width: i32,
    height: i32,
    bytes_per_scanline: i32,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let photo = image::open(filename).unwrap();

        Self {
            data: photo.clone().into_bytes(),
            width: photo.width() as i32,
            height: photo.height() as i32,
            bytes_per_scanline: (photo.width() * 3) as i32,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, mut u: f64, mut v: f64, _p: &Vec3) -> Vec3 {
        if self.data == [0_u8, 0] {
            return Vec3::new(0.0, 1.0, 1.0);
        }

        u = clamp(u, 0.0, 1.0);
        v = 1.0 - clamp(v, 0.0, 1.0);

        let mut i = (u * self.width as f64) as i32;
        let mut j = (v * self.height as f64) as i32;
        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }

        let color_scale = 1.0 / 255.0;
        let mut pixel: [f64; 3] = [0.0; 3];
        pixel[0] = self.data[(j * self.bytes_per_scanline + i * 3) as usize] as f64;
        pixel[1] = self.data[(j * self.bytes_per_scanline + i * 3 + 1) as usize] as f64;
        pixel[2] = self.data[(j * self.bytes_per_scanline + i * 3 + 2) as usize] as f64;

        Vec3::new(
            color_scale * pixel[0],
            color_scale * pixel[1],
            color_scale * pixel[2],
        )
    }
}
*/
pub struct ImageTexture {
    pub data: ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>,
    pub width: i64,
    pub height: i64,
    pub bytes_per_scanline: i64,
    pub bytes_per_pixel: i64,
}
impl ImageTexture {
    /*pub fn new0() -> Self {
        Self {
            data: Vec::new(),
            bytes_per_pixel: 3,
            width: 0,
            height: 0,
            bytes_per_scanline: 0,
        }
    }*/

    pub fn new(filename: &str) -> Self {
        let bytes_per_pixel: i64 = 3;
        //let components_per_pixel = bytes_per_pixel;
        let data2 = image::open(filename).unwrap().to_rgb8();
        let width2 = data2.width();
        let height2 = data2.height();

        let bytes_per_scanline = bytes_per_pixel * (width2 as i64);
        Self {
            bytes_per_pixel,
            data: data2,
            width: width2 as i64,
            height: height2 as i64,
            bytes_per_scanline,
        }
    }
}
impl ImageTexture {
    pub fn value(&self, u: f64, v: f64) -> [f64; 3] {
        let u2 = u;
        let v2 = 1.0 - v;

        let i = (u2 * self.width as f64) as usize;
        let j = (v2 * self.height as f64) as usize;

        let color_scale = 1.0 / 255.0;

        let pixel = self.data.get_pixel(i as u32, j as u32);
        let [red, green, blue] = pixel.0;
        [
            color_scale * (red as f64),
            color_scale * (green as f64),
            color_scale * (blue as f64),
        ]
    }
}
