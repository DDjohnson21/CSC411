use array2::Array2;
pub use crate::rgbfloat::Rgbfloat;
use crate::ypbpr::YPbPr;

/// Convert an Array2 of Rgbfloat to an Array2 of YPbPr
pub fn float_to_ypbpr(rgbfloat_array2: &Array2<Rgbfloat>) -> Array2<YPbPr>{
    // Create a new Vec of YPbPr to store the converted values
    let mut ypbpr_vec:Vec<YPbPr> = Vec::new();
    // Iterate through the Rgbfloat Array2 and convert each pixel to YPbPr
    for (_, _, pixel) in rgbfloat_array2.iter_row_major() {
        // Convert the pixel to YPbPr and push it to the Vec
        let ypbpr_obj = YPbPr{
            y: 0.299 * pixel.red + 0.587 * pixel.green + 0.114 * pixel.blue,
            pb: -0.168736 * pixel.red - 0.331264 * pixel.green + 0.5 * pixel.blue,
            pr: 0.5 * pixel.red - 0.418688 * pixel.green - 0.081312 * pixel.blue
        };
        ypbpr_vec.push(ypbpr_obj);
    }
    // Return the new Array2 of YPbPr
    Array2::from_row_major(rgbfloat_array2.width(), rgbfloat_array2.height(), ypbpr_vec).unwrap()
}

// Convert an Array2 of YPbPr to an Array2 of Rgbfloat
pub fn ypbpr_to_float(ypbpr_array2: &Array2<YPbPr>) -> Array2<Rgbfloat>{
    // Create a new Vec of Rgbfloat to store the converted values
    let mut rgbfloat_vec: Vec<Rgbfloat> = Vec::new();
    // Iterate through the YPbPr Array2 and convert each pixel to Rgbfloat
    for (_, _, pixel) in ypbpr_array2.iter_row_major() {
        // Convert the pixel to Rgbfloat and push it to the Vec
        let rgbfloat = Rgbfloat{
            red: pixel.y + 1.402 * pixel.pr,
            green: pixel.y - 0.344136 * pixel.pb - 0.714136 * pixel.pr,
            blue: pixel.y + 1.772 * pixel.pb
        };
        rgbfloat_vec.push(rgbfloat);
    }
    // Return the new Array2 of Rgbfloat
    Array2::from_row_major(ypbpr_array2.width(), ypbpr_array2.height(), rgbfloat_vec).unwrap()
}

