mod aabb;
mod aarec;
mod boxx;
mod camera;
mod color;
mod constant_medium;
mod data;
mod ray;
mod render;
mod rotate;
mod sphere;
mod texture;
mod util;
mod vec3;
mod world;

use image::RgbImage; //接收render传回来的图片，在main中文件输出
                     //main中产生进度条并传给render
use std::fs::File;
use std::time::Instant;

use camera::Camera;
use data::Data; //数据层
use render::render; //渲染层
use vec3::Vec3;

use std::process::Command;

const AUTHOR: &str = "seeker🤯";
fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn main() {
    //f64::INFINITY;std::f64::consts::PI;
    // get environment variable CI, which is true for GitHub Actions
    let now = Instant::now();
    println!("🏳️‍🌈🏳️‍🌈🏳️‍🌈🏳️‍🌈🏳️‍🌈🏳️‍🌈🏳️‍🌈🏳️‍🌈🏳️‍🌈🏳️‍🌈🏳️‍🌈🏳️‍🌈🏳️‍🌈🏳️‍🌈");
    println!("游戏开始");

    let is_ci: bool = is_ci();
    let path = "output/test_9t.jpg";
    let height = 800;
    let width = 800;
    let gamma = 2;
    let sample_times = 250;
    let depth = 50;
    println!("🚩图片参数");
    println!("CI: {}", is_ci);
    println!("图片大小:{}*{}", height, width);
    println!("伽马值:{}", gamma);
    println!("每个像素点采样次数:{}", sample_times);
    println!("反射次数:{}", depth);
    //let data = Data::new(1000, 1500, 60, 2, 100, 40);
    let data = Data::new(height, width, 60, gamma, sample_times, depth);
    let origin = Vec3::new(478.0, 278.0, -600.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let camera0 = Camera::new(
        data.width as f64 / data.height as f64,
        2.0,
        origin,
        lookat,
        std::f64::consts::PI / 4.5,
        0.0,
        10.0,
    );
    let threadnum = 9;
    println!("🤡渲染线程数:{}", threadnum);
    let img: RgbImage = render(&data, camera0, is_ci, threadnum); //data一定要引用
                                                                  // Output image to file
    println!("💌Ouput image as \"{}\"\n😻Author: {}", path, AUTHOR);
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
    println!("🤖程序运行{}秒", now.elapsed().as_secs());
}
