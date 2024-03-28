use array2::Array2;
use bitpack::bitpack;
use crate::ypbpr::YPbPr;

pub fn pack_values(y_values: [f64; 4], pb_average: f32, pr_average: f32) -> Option<u64> {
    let mut word: u64 = 0;

    // Access Y values
    let y1 = y_values[0];
    let y2 = y_values[1];
    let y3 = y_values[2];
    let y4 = y_values[3];
    
    // Calculate the average of the Y values
    let a = (y4 + y3 + y2 + y1) / 4.0;
    let b = (y4 + y3 - y2 - y1) / 4.0;
    let c = (y4 - y3 + y2 - y1) / 4.0;
    let d = (y4 - y3 - y2 + y1) / 4.0;
    let index_pb = csc411_arith::index_of_chroma(pb_average) as u64;
    let index_pr = csc411_arith::index_of_chroma(pr_average) as u64;

    // Quantized values
    word = bitpack::newu(word, 4, 0, index_pr)?;
    word = bitpack::newu(word, 4, 4, index_pb)?;
    word = bitpack::news(word, 5, 8, float_to_signed_int(d))?;
    word = bitpack::news(word, 5, 13, float_to_signed_int(c))?;
    word = bitpack::news(word, 5, 18, float_to_signed_int(b))?;
    word = bitpack::newu(word, 9, 23, a as u64)?;

    Some(word)
}

// Function to process a single block and return the Y values, and average Pb and Pr values
fn get_block_values(array: &Array2<YPbPr>, block_index: usize, block_size: usize) -> ([f64; 4], f32, f32) {
    let mut y_values = [0f64; 4];
    let mut pb_values = Vec::new();
    let mut pr_values = Vec::new();

    for row_offset in 0..2 {
        for col_offset in 0..2 {
            let index = block_index * block_size + row_offset * 2 + col_offset;
            if let Some(pixel) = array.data().get(index) {
                y_values[row_offset * 2 + col_offset] = pixel.y as f64;
                pb_values.push(pixel.pb);
                pr_values.push(pixel.pr);
            }
        }
    }

    let pb_average: f32 = pb_values.iter().sum::<f32>() / pb_values.len() as f32;
    let pr_average: f32 = pr_values.iter().sum::<f32>() / pr_values.len() as f32;

    (y_values, pb_average, pr_average)
}

//  function to process all blocks

pub fn process_blocks(array: Array2<YPbPr>) {
    let mut codewords: Vec<[u8; 4]> = Vec::new();
    let blocks_across = array.width() / 2;
    let blocks_down = array.height() / 2;
    let total_blocks = blocks_across * blocks_down;

    for block_index in 0..total_blocks {
        let (y_values, pb_average, pr_average) = get_block_values(&array, block_index, 4);
        if let Some(codeword) = pack_values(y_values, pb_average, pr_average) {
            codewords.push((codeword as u32).to_be_bytes());
        } else {
            codewords.push([0, 0, 0, 0]);
        }
    }

// Output the codewords for the entire image
    csc411_rpegio::output_rpeg_data(&codewords, array.width(), array.height()).unwrap();
}



// ADD TO NEW FILE
fn float_to_signed_int(value: f64) -> i64 {
    // Define the boundaries
    let min_val = -0.3;
    let max_val = 0.3;
    let max_int = 15i64;

    // Check if the value is outside the boundaries and clamp
    if value <= min_val {
        return -max_int;
    } else if value >= max_val {
        return max_int;
    }

    // Normalize the value to the interval [-15, 15]
    let normalized = (value / max_val) * max_int as f64;

    // Round the result to get an integer and clamp to the interval [-15, 15]
    normalized.round().clamp(-max_int as f64, max_int as f64) as i64
}

pub fn unpack_values(word: u64) -> ([f64; 4], f32, f32) {
    // Extract Pb and Pr indices and Y component differences
    let index_pr = bitpack::getu(word, 4, 0).unwrap() as f32;
    let index_pb = bitpack::getu(word, 4, 4).unwrap() as f32;
    let d = signed_int_to_float(bitpack::gets(word, 5, 8).unwrap());
    let c = signed_int_to_float(bitpack::gets(word, 5, 13).unwrap());
    let b = signed_int_to_float(bitpack::gets(word, 5, 18).unwrap());
    let a = bitpack::getu(word, 9, 23).unwrap() as f64 / (1 << 9) as f64;

    // Convert indices back to average Pb and Pr values
    let pb_average = csc411_arith::chroma_of_index(index_pb as usize);
    let pr_average = csc411_arith::chroma_of_index(index_pr as usize);

    // Reconstruct the Y values
    let y1 = a - b - c + d;
    let y2 = a - b + c - d;
    let y3 = a + b - c - d;
    let y4 = a + b + c + d;

    ([y1, y2, y3, y4], pb_average, pr_average)
}

fn signed_int_to_float(value: i64) -> f64 {
    // Inverse of `float_to_signed_int`
    let max_val = 0.3;
    let max_int = 15i64;

    // Convert the value back to the original scale
    let normalized = value as f64 / max_int as f64 * max_val;

    normalized
}

pub fn reconstruct_blocks(codewords: Vec<[u8; 4]>, width: usize, height: usize) -> Array2<YPbPr> {
    let default_ypbpr = YPbPr { y: 0.0, pb: 0.0, pr: 0.0 };
    
    let mut array = Vec::with_capacity(width * height); // Initialize the vector with the correct capacity
    array.resize(width * height, default_ypbpr.clone()); // Resize and fill with default YPbPr values
    
    let blocks_across = width / 2;
    let blocks_down = height / 2;
    let total_blocks = blocks_across * blocks_down;

    for block_index in 0..total_blocks {
        if let Some(codeword_bytes) = codewords.get(block_index) {
            // Convert the byte array back into a u32, then into a u64 codeword
            let codeword = u32::from_be_bytes(*codeword_bytes) as u64;

            // Decompress the codeword to get the original Y values and average Pb and Pr values
            let (y_values, pb_average, pr_average) = unpack_values(codeword);

            // Calculate the top-left corner of the current block in the image
            let block_row = (block_index / blocks_across) * 2;
            let block_col = (block_index % blocks_across) * 2;

            // Assign the values back to the array
            for row_offset in 0..2 {
                for col_offset in 0..2 {
                    let pixel_index = ((block_row + row_offset) * width + (block_col + col_offset)) as usize;
                    if pixel_index < array.len() {
                        let pixel = &mut array[pixel_index];
                        pixel.y = y_values[row_offset * 2 + col_offset] as f32;
                        pixel.pb = pb_average;
                        pixel.pr = pr_average;
                    }
                }
            }
        }
    }

    // Assuming Array2 has a constructor like `from_row_major`
    Array2::from_row_major(width, height, array).unwrap()
}