use csc411_image::{Read, GrayImage}; 
use std::env;
use std::io;

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
        println!("Please enter some text:");
        let mut temp_input = String::new();
        io::stdin()
            .read_line(&mut temp_input)
            .expect("Failed to read line");

        input = Some(temp_input.trim().to_string());
            
    }
    let img = GrayImage::read(input.as_deref()).unwrap();
    let mut counter: f32 = 0.0;
    let vec_length: f32 = img.pixels.len() as f32;
    for pixel in img.pixels {
        counter = counter + (pixel.value as f32 / img.denominator as f32 );
    }
    println!("{}", counter as f32 / vec_length as f32);

    return ();
}