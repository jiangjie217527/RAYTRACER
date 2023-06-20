mod color;
mod data;
mod ray;
mod render;
mod sphere;
mod vec3;

use image::RgbImage;
use indicatif::ProgressBar;
use std::fs::File;

use data::Data;
use render::render;

const AUTHOR: &str = "停云别叫恩公,叫___";
fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn main() {
    // get environment variable CI, which is true for GitHub Actions
    let is_ci: bool = is_ci();
    println!("CI: {}", is_ci);
    let path = "output/test.jpg";
    let data = Data::new(800, 800, 60, 1.0, 2.0);

    let bar: ProgressBar = if is_ci {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((data.height * data.width) as u64)
    };

    let img: RgbImage = render(data.clone(), bar);

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
}
