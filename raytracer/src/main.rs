mod camera;
mod color;
mod data;
mod ray;
mod render;
mod sphere;
mod util;
mod vec3;

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
    let data = Data::new(800, 800, 60, 2, 8, 20);
    let origin = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::zero();
    let camera = Camera::new(
        1.0,
        2.0,
        origin.clone(),
        lookat.clone(),
        std::f64::consts::PI / 5.0,
        0.1,
        //(origin-lookat).length(),
        10.0,
    );
    let bar: ProgressBar = if is_ci {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((data.height * data.width) as u64)
    };

    let img: RgbImage = render(&data, camera, bar); //data一定要引用

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
    Command::new("cmd")
        .args(["/C", ".\\sound.exe"])
        .output()
        .expect("failed to execute process");
}
