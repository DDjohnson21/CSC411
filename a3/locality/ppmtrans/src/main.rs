// use csc411_image::{Read, Rgb, RgbImage};
use csc411_image::{Read, Rgb, RgbImage, Write};

// Actually needed for assignment
// use csc411_image::{Read, Rgb, RgbImage, Write};
use clap::Parser;
// use std::process;
use std::io;
use std::env;
extern crate array2;
use array2::Array2;
use std::fmt;



#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args { 
    // Flip
    #[clap(long = "flip", required = false)]
    flip: Option<String>,
    // Rotation
    #[clap(short = 'r', long = "rotate")] 
    rotate: Option<u32>,
}

// #[derive(Clone)]
// struct RgbPixel {
//     red: u16,   // 0-255 representing the intensity of red
//     green: u16, // 0-255 representing the intensity of green
//     blue: u16,  // 0-255 representing the intensity of blue
// }

// impl fmt::Display for RgbPixel {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{} {} {}", self.red, self.green, self.blue)
//     }
// }

fn main() {
    // let args = Args::parse(); 
    // let rotate = args.rotate;
    // let input = args.flip;

    // if let Some(flip_value) = args.flip {
    //     println!("Flip value provided: {}", flip_value);
    // }
    // if let Some(rotate_value) = args.rotate {
    //     println!("Rotate value provided: {}", rotate_value);
    // }

        // Get the number of arguments
        let argument_count: usize = env::args().len();
        // Create a variable to store the input
        let input: Option<String>;
        // Error if there is more than one argument
        if argument_count > 2 {
            assert!(env::args().len() == 2);
            return;
        }
        // Take the first argument
        else if argument_count == 2 {
            input = env::args().nth(1);
        }
        // Read from stdin
        else {
            // Creates a variable to store stdin input due to variable type conflicts
            let mut temp_input = String::new();
            io::stdin()
                .read_line(&mut temp_input)
                .expect("Failed to read line");
            input = Some(temp_input.trim().to_string());        
        }

        // Access image
        let img = RgbImage::read(input.as_deref()).unwrap();


        let width = img.width as usize;
        let height = img.height as usize;
 
        
        // Create a vector to store the pixel values
        // let mut my_vec: Vec<RgbPixel> = Vec::new();
        // for pixel in img.pixels {
        //     // my_vec.push(pixel.value as i32);
        //     let currentPixel = RgbPixel { red: pixel.red, green: pixel.green, blue: pixel.blue };

        //     my_vec.push(currentPixel);

        // }

    
        let width = img.width as usize;
        let height = img.height as usize;
        let array = Array2::from_row_major(width, height, img.pixels).expect("Vector size does not match the specified dimensions.");

        println!("Original array:");
        // array.display();
        //let result = array.write(None);
    
        // Rotate 90 degrees
        let rotated90 = array.rotate90_row_major();
        println!("\nArray after 90 degree rotation:");
        // rotated90.display();
    
        // Rotate 180 degrees from the original
        // WORKS
        //let rotated180 = array.rotate180_row_major();

        // WORKS
        // let rotated180 = array.rotate180_col_major();


        // println!("\nArray after 180 degree rotation:");
        // rotated180.display();
        // let result180 = rotated180.write(None);

        let image = RgbImage {
            pixels: rotated90.data(),
            width: width as u32,
            height: height as u32,
            denominator: 255, // Assuming the denominator is used for color depth
        };

        
        let _image_output = image.write(Some("output180.ppm"));
        
}