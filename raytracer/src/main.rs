mod aabb;
mod camera;
mod color;
mod data;
mod ray;
mod render;
mod sphere;
//mod test_scene;
mod aarec;
mod texture;
mod util;
mod vec3;
mod world;

use image::RgbImage; //接收render传回来的图片，在main中文件输出
use indicatif::ProgressBar; //main中产生进度条并传给render
use std::fs::File;

use camera::Camera;
use data::Data; //数据层
use render::render; //渲染层
use vec3::Vec3;

use std::process::Command;

const AUTHOR: &str = "停云别叫恩公,叫___";
fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn main() {
    //f64::INFINITY;std::f64::consts::PI;
    // get environment variable CI, which is true for GitHub Actions

    let is_ci: bool = is_ci();
    println!("CI: {}", is_ci);
    let path = "output/test.jpg";
    let data = Data::new(1000, 1500, 60, 2, 100, 40);
    //let data = Data::new(800, 800, 60, 2, 50, 40);
    let origin = Vec3::new(26.0, 3.0, 6.0);
    let lookat = Vec3::new(0.0, 2.0, 0.0);
    let camera0 = Camera::new(
        data.width as f64 / data.height as f64,
        2.0,
        origin,
        lookat,
        std::f64::consts::PI / 9.0,
        0.0,
        10.0,
    );
    let bar: ProgressBar = if is_ci {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((data.height * data.width) as u64)
    };

    let img: RgbImage = render(&data, camera0, bar); //data一定要引用

    // Output image to file
    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
    let mut output_file: File = File::create(path).unwrap();
    match output_image.write_to(
        &mut output_file,
        image::ImageOutputFormat::Jpeg(data.quality),
    ) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
    //play the sound
    if !is_ci {
        Command::new("cmd")
            .args(["/C", ".\\sound.exe"])
            .output()
            .expect("failed to execute process");
    }
}
