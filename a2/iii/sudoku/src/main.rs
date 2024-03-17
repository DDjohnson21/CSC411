use csc411_image::{Read, GrayImage}; 
use std::io;
use std::env;
use std::process;
extern crate array2;

fn main() {
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
    let img = GrayImage::read(input.as_deref()).unwrap();
    // Create a vector to store the pixel values
    let mut my_vec: Vec<i32> = Vec::new();
    for pixel in img.pixels {
        my_vec.push(pixel.value as i32);
    }
    // Ensure the vector is of the expected size for a Sudoku (9x9)
    if my_vec.len() != 81 {
        eprintln!("Image does not represent a 9x9 Sudoku puzzle.");
        process::exit(1);
    }
    // Create a 9x9 array from the vector
    let array = array2::Array2::new(my_vec,9,9);
    
    // Function to convert an i32 to a usize
    let to_usize = |x: &i32| Some(*x as usize);

    if array.valid_sudoku(to_usize) {
        process::exit(0);
    } else {
        process::exit(1);
    }
}