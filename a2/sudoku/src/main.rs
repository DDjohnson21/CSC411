use csc411_image::{Read, GrayImage}; 
use std::io;
use std::env;
extern crate array2;


fn main() {
    let argument_count: usize = env::args().len();
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

    // Create a 3x3 array from the image
    let mut my_vec: Vec<i32> = Vec::new();

    for pixel in img.pixels {
        my_vec.push(pixel.value as i32);
    }


 

    let col_array = array2::Array2::from_col_major(my_vec.clone(),9,9);
    let row_array = array2::Array2::from_row_major(my_vec, 9, 9);

    println!("{:?}", col_array);

    // let _value = array2::Array2::find_element(&row_array, &3);
    
    // for element in row_array.iter_row_major() {
    //     println!("{:?}", element);
    // }

    for element in col_array.iter_row_major() {
        println!("{:?}", element);
    }
    
    println!("{:?}", row_array);
    // println!("{:?}", row_array);
    // println!("{:?}", value);
}