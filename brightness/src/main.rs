use csc411_image::{Read, GrayImage}; 
use std::env;

fn main() {
    let argument_count: usize = env::args().len();
    let input: Option<String>;
    if argument_count > 2 {
        assert!(env::args().len() == 2);
        return;
        // panic! ("Can't have more than two arguments");
        // return 0;
        // input = env::args().nth(1);
    }
    else if argument_count == 2 {
        input = env::args().nth(1);
    }
    else {
        input = env::args().nth(1);
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