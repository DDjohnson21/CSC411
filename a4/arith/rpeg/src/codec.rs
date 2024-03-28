use csc411_image::{Read, Rgb, RgbImage, Write};
use array2::Array2;
use crate::rgb_float_conversion::{rgb_to_float, float_to_rgb};
use crate::rgb_float_ypbpr_conversion::{float_to_ypbpr, ypbpr_to_float};
use crate::process::{process_blocks, reconstruct_blocks};
use std::io::{self};

// compress function that takes in an optional filename and returns a Result
// The function reads in an image file, trims the image to an even size, converts the RGB values to YPbPr values, and then compresses the image using the RPEG algorithm.
pub fn compress(filename: Option<&str>) -> io::Result<()> {
    // Trim array
    let trimmed_array = prepare_array(filename);
    // Convert to RGB floats
    let rgb_float_array = rgb_to_float(&trimmed_array);
    // Convert to YPbPr values
    let ypbpr_array = float_to_ypbpr(&rgb_float_array);
    // Convert array to block major order
    // let block_major_array = ypbpr_array.to_2x2_block_major();

    // Process blocks and output results
    process_blocks(ypbpr_array); 
    
    Ok(())
}

// prepare_array function trims array to even size
// The function reads in an image file and trims the image to an even size
fn prepare_array(filename: Option<&str>)->Array2<Rgb> {
    let img = RgbImage::read(filename.as_deref()).unwrap(); // Read the image
    let array = Array2::from_row_major(img.width as usize, img.height as usize, img.pixels).expect("Vector size does not match the specified dimensions.");
    let trimmed_image = array.trim_to_even();
    return trimmed_image;
}


// decompress function that takes in an optional filename and returns a Result
pub fn decompress(_filename: Option<&str>) -> io::Result<()> {   
    let input = _filename;
    let (compressed_data, width, height) = csc411_rpegio::input_rpeg_data(input).unwrap();

    let ypbpr_array = reconstruct_blocks(compressed_data, width, height);
    let rgb_float_array = ypbpr_to_float(&ypbpr_array);
    let rgb_array = float_to_rgb(&rgb_float_array);
    
    let image = RgbImage {
        pixels: rgb_array.data(),
        width: rgb_array.width() as u32,
        height: rgb_array.height() as u32,
        denominator: 255,
    };
    
    // Print the image to stdout
    let _image_output = image.write(Some("output5.ppm"));

    Ok(())
}