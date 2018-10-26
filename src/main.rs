#[macro_use]
extern crate clap;
extern crate image;

use clap::{App, AppSettings, Arg};
use image::{DynamicImage, GenericImageView, ImageResult};
use std::path::{Path, PathBuf};

fn read_file(filename: &Path) -> ImageResult<DynamicImage> {
    image::open(filename)
}

fn save_file(path: &Path, img: DynamicImage) -> Result<(), std::io::Error> {
    image::save_buffer(
        path,
        &img.raw_pixels(),
        img.width(),
        img.height(),
        img.color(),
    )
}

fn parse_args() -> (PathBuf, PathBuf, f32) {
    let matches = App::new("blur-image")
        .setting(AppSettings::AllowLeadingHyphen)
        .version("1.0")
        .author("Tamas T. <tancos@outlook.hu>")
        .about("Blurs images")
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("FILE")
            .help("Sets the output file")
            .takes_value(true))
        .arg(Arg::with_name("sigma")
            .short("s")
            .long("sigma")
            .help("Sets the blur sigma")
            .default_value("10.0")
            .takes_value(true))
        .arg(Arg::from_usage("<image> 'the image to be blurred'"))
        .get_matches();

    let input: String = value_t_or_exit!(matches.value_of("image"), String);
    let input_path = PathBuf::from(input.as_str());

    // TODO: parent to have full path
    let default_output = format!(
        "{}_blurred.{}",
        input_path.file_stem().unwrap().to_str().unwrap(),
        input_path.extension().unwrap().to_str().unwrap()
    );

    let output: String = value_t!(matches.value_of("output"), String)
        .unwrap_or(String::from(default_output.as_str()));

    let sigma = value_t_or_exit!(matches.value_of("sigma"), f32);

    return (input_path, PathBuf::from(output.as_str()), sigma);
}

fn main() {
    let (input, output, sigma) = parse_args();

    println!("Reading image {:?}", input.to_str().unwrap());
    let img: DynamicImage = match read_file(input.as_path()) {
        Result::Ok(i) => i,
        Result::Err(err) => {
            println!("something went wrong reading the file {:?}", err);
            std::process::exit(1);
        }
    };

    match save_file(output.as_path(), img.blur(sigma)) {
        Result::Ok(_) => println!("Saved blurred ({:?}) image to {:?}", sigma, output),
        Result::Err(err) => println!("Error: {:?}", err),
    }

    println!(
        "Saved blurred ({:?}) image to {:?}",
        sigma,
        output.to_str().unwrap()
    );
}
