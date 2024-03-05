extern crate array2;
use csc411_image::{Read, Rgb, RgbImage, Write};
use clap::Parser;
use std::{char::TryFromCharError, io};
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

    // let mut input: Option<String>;

    let rotate = args.rotate;
    let input = args.file;
    let row_major = args.row_major;
    let col_major = args.col_major;

    
    let img = RgbImage::read(input.as_deref()).unwrap();
    let width = img.width as usize;
    let height = img.height as usize;
    let array = Array2::from_row_major(width, height, img.pixels).expect("Vector size does not match the specified dimensions.");

    // let transformation;

    //     if args.rotate == Some(90) {
    //         if args.row_major {
    //             transformation = array.rotate90_row_major();
    //         }
    //         else {
    //             transformation = array.rotate90_col_major();
    //         }
    //     }
    //     else {
    //         if args.row_major == true {
    //             transformation = array.rotate180_row_major();
    //         }
    //         else {
    //             transformation = array.rotate180_col_major();
    //         }
    //     }

        array.rotate90_row_major();

        let image = RgbImage {
            pixels: array.data(),
            width: width as u32,
            height: height as u32,
            denominator: 255,
        };

        let _image_output = image.write(Some("output.ppm"));
    }


    // if .rotate.is_sone() && (rotate.uunrwpp == 90 || rotate.unwrap() == 180) || args.transpose {
        // Dest = arra2::new
    // }
    // else {
        // Dest = arra2::new