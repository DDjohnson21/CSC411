extern crate array2;
use csc411_image::{Read, RgbImage, Write};
use clap::Parser;
use array2::Array2;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]

struct Args { 
    // Rotation
    #[clap(short = 'r', long = "rotate")] 
    rotate: Option<u32>,
    // Row major
    #[clap(long = "row-major", required = false)] 
    row_major: bool,
    // Column major
    #[clap(long = "col-major", required = false)]
    col_major: bool,
    // File
    #[clap(required = false)]
    file: Option<String>,
}

fn main() {
    let args = Args::parse(); 
    let input = args.file;
    
    let img = RgbImage::read(input.as_deref()).unwrap();
    let width = img.width as usize;
    let height = img.height as usize;
    let array = Array2::from_row_major(width, height, img.pixels).expect("Vector size does not match the specified dimensions.");

    let transformation;

    // Input handling
    if args.rotate == Some(90) {
        if args.row_major {
            transformation = array.rotate90_row_major();
        }
        else {
            transformation = array.rotate90_col_major();
        }
    }
    else {
        if args.row_major == true {
            transformation = array.rotate180_row_major();
        }
        else {
            transformation = array.rotate180_col_major();
        }
    }

    // Use new array to create a new image
    let image = RgbImage {
        pixels: transformation.data(),
        width: width as u32,
        height: height as u32,
        denominator: 255,
    };
    
    // Print the image to stdout
    let _image_output = image.write(Some("output.ppm"));
    }