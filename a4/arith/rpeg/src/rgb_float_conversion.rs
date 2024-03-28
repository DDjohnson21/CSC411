use csc411_image::Rgb;
use array2::Array2;
pub use crate::rgbfloat::Rgbfloat;

// Convert an Array2 of Rgb to an Array2 of Rgbfloat
pub fn rgb_to_float(rgb_array2: &Array2<Rgb>) -> Array2<Rgbfloat>{
    // Create a new Vec of Rgbfloat to store the converted values
    let mut rgbfloat_vec:Vec<Rgbfloat> = Vec::new();
    // Iterate through the Rgb Array2 and convert each pixel to Rgbfloat
    for (_, _, pixel) in rgb_array2.iter_row_major() {
        // Convert the pixel to Rgbfloat and push it to the Vec
        let rbgfloat = Rgbfloat{
            red: pixel.red as f32 / 255 as f32,
            green: pixel.green as f32 / 255 as f32,
            blue: pixel.blue as f32 / 255 as f32
        };
        rgbfloat_vec.push(rbgfloat);
    }
    // Return the new Array2 of Rgbfloat
    Array2::from_row_major(rgb_array2.width(), rgb_array2.height(), rgbfloat_vec).unwrap()
}

// Convert an Array2 of Rgbfloat to an Array2 of Rgb
pub fn float_to_rgb(rgbfloat_array2: &Array2<Rgbfloat>) -> Array2<Rgb> {
    // Create a new Vec of Rgb to store the converted values
    let mut rgb_vec: Vec<Rgb> = Vec::new();
    // Iterate through the Rgbfloat Array2 and convert each pixel to Rgb
    for (_, _, pixel) in rgbfloat_array2.iter_row_major() {
        // Convert the pixel to Rgb and push it to the Vec
        let rgb = Rgb{
            red: (pixel.red * 255.0).round() as u16,
            green: (pixel.green * 255.0).round() as u16,
            blue: (pixel.blue * 255.0).round() as u16
        };
        rgb_vec.push(rgb);
    }
    // Return the new Array2 of Rgb
    Array2::from_row_major(rgbfloat_array2.width(), rgbfloat_array2.height(), rgb_vec).unwrap()
}