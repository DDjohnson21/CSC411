use array2::Array2;
use std::env;
use csc411_image::{Read, Rgb, RgbImage};
use std::io;

fn ppmdiff(img1: &Array2<Rgb>, img2: &Array2<Rgb>) -> f64 {
    let width = img1.width().min(img2.width());
    let height = img1.height().min(img2.height());
    let mut error_sum = 0.0;

    for i in 0..width {
        for j in 0..height {
            let pixel1 = img1.get(i, j).unwrap();
            let pixel2 = img2.get(i, j).unwrap();

            let r_diff = (pixel1.red as f64 / 255.0 - pixel2.red as f64 / 255.0 ).powi(2);
            let g_diff = (pixel1.green as f64 / 255.0 - pixel2.green as f64 / 255.0 ).powi(2);
            let b_diff = (pixel1.blue as f64 / 255.0 - pixel2.blue as f64 / 255.0 ).powi(2);

            error_sum += r_diff + g_diff + b_diff;
        }
    }

    return (error_sum / (3.0 * width as f64 * height as f64)).sqrt()
}


fn main() {
    let input1: Option<String>;
    let input2: Option<String>;

    if env::args().nth(1).as_deref() == Some("-") {
        let mut temp_input = String::new();
        io::stdin()
            .read_line(&mut temp_input)
            .expect("Failed to read line");
        input1 = Some(temp_input.trim().to_string());  
    }
    else  {
        input1 = env::args().nth(1);
    }

    if env::args().nth(2).as_deref() == Some("-") {
        let mut temp_input = String::new();
        io::stdin()
            .read_line(&mut temp_input)
            .expect("Failed to read line");
        input2 = Some(temp_input.trim().to_string());  
    }
    else  {
        input2 = env::args().nth(2);
    }

    // Handle Image 1
    let img1 = RgbImage::read(input1.as_deref()).unwrap();
    let width1 = img1.width as usize;
    let height1 = img1.height as usize;
    let array1 = Array2::from_row_major(width1, height1, img1.pixels).expect("Vector size does not match the specified dimensions.");

    // Handle Image 2
    let img2 = RgbImage::read(input2.as_deref()).unwrap();
    let width2 = img2.width as usize;
    let height2 = img2.height as usize;
    let array2 = Array2::from_row_major(width2, height2, img2.pixels).expect("Vector size does not match the specified dimensions.");

    let trimmed_array1 = array1.trim_to_even();
    let trimmed_array2 = array2.trim_to_even();

    let rmsd = ppmdiff(&trimmed_array1, &trimmed_array2);

    println!("{:.4}", rmsd);
}