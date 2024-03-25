use csc411_image::{Read, Rgb, RgbImage};
use array2::Array2;
use bitpack::bitpack;
use csc411_rpegio::output_rpeg_data;

use std::io::{self};

pub fn compress(filename: Option<&str>) -> io::Result<()> {
    let trimmed_image = prepare_array(filename);

    let height = trimmed_image.height();
    let width = trimmed_image.width();

    let ypbpr_array = convert_rgb_to_ypbpr(trimmed_image);

    let block_major_array = ypbpr_array.to_2x2_block_major();

    // Call process_blocks
    let codewords = process_blocks(block_major_array);
    // Works
    let words: Vec<u32> = vec![0x00112233, 0x44556677];

    // let hex_strings: Vec<String> = codewords.iter().map(|&num| format!("{:08X}", num)).collect();

    // let compressed: Vec<u8; 4> = codewords.clone().into_iter().map(u32::to_be_bytes).flatten().collect();
    // let compressed: Vec<[u8; 4]> = codewords.clone().into_iter().map(u32::to_be_bytes).flatten().collect();
    // let compressed_data: Vec<[u8; 4]> = hex_strings.into_iter().map(u32::to_be_bytes).collect();
    let compressed_data: Vec<[u8; 4]> = words.into_iter().map(u32::to_be_bytes).collect();

    // Works
    let width = 4;
    let height = 2;

    output_rpeg_data(&compressed_data, width, height).unwrap();
    // output_rpeg_data(&codewords, width, height);
    Ok(())
}


pub fn decompress(filename: Option<&str>) -> io::Result<()> {
    prepare_array(filename);

    Ok(())
}

fn prepare_array(filename: Option<&str>)->Array2<Rgb> {
    let img = RgbImage::read(filename.as_deref()).unwrap(); // Read the image
    let width = img.width as usize;
    let height = img.height as usize;
    let array = Array2::from_row_major(width, height, img.pixels).expect("Vector size does not match the specified dimensions.");
    let trimmed_image = array.trim_to_even();
    return trimmed_image;
}

fn convert_rgb_to_ypbpr(rgb_array: Array2<Rgb>) -> Array2<(f32, f32, f32)> {
    // Convert each RGB tuple from u8 to f32
    let converted: Vec<(f32, f32, f32)> = rgb_array.data().iter().map(|&Rgb { red: r, green: g, blue: b }| {
        let r_f32 = r as f32 / 255.0;
        let g_f32 = g as f32 / 255.0;
        let b_f32 = b as f32 / 255.0;

        // Calculate YPbPr components
        let y = 0.299 * r_f32 + 0.587 * g_f32 + 0.114 * b_f32;
        let pb = -0.168736 * r_f32 - 0.331264 * g_f32 + 0.5 * b_f32;
        let pr = 0.5 * r_f32 - 0.418688 * g_f32 - 0.081312 * b_f32;

        (y, pb, pr)
    }).collect();

    Array2::from_row_major(rgb_array.width(), rgb_array.height(), converted).unwrap()
}

// Reverse of the above function (Part 3)
// NEEDS WORK
// DECOMPRESSION
// pub fn ypbpr_to_rgb(y: f32, pb: f32, pr: f32) -> (u8, u8, u8) {
    
//     let r = y + 1.402 * pr;
//     let g = y - 0.344136 * pb - 0.714136 * pr;
//     let b = y + 1.772 * pb;

//     return (r as u8, g as u8, b as u8)
// }


pub fn pack_values(y_values: [i64; 4], pb_average: f32, pr_average: f32) -> Option<u64> {
    let mut word: u64 = 0;
    
    let y1 = y_values[0];
    let y2 = y_values[1];
    let y3 = y_values[2];
    let y4 = y_values[3];


    // Calculate the average of the Y values
    let a = (y4 + y3 + y2 + y1) / 4;
    let b = (y4 + y3 - y2 - y1) / 4;
    let c = (y4 - y3 + y2 - y1) / 4;
    let d = (y4 - y3 - y2 + y1) / 4;
    let index_pb = csc411_arith::index_of_chroma(pb_average) as u64;
    let index_pr = csc411_arith::index_of_chroma(pr_average) as u64;


    // quantized values
    word = bitpack::newu(word, 4, 0, index_pr)?;
    word = bitpack::newu(word, 4, 4, index_pb)?;
    word = bitpack::news(word, 5, 8, d)?;
    word = bitpack::news(word, 5, 13, c)?;
    word = bitpack::news(word, 5, 18, b)?;
    word = bitpack::newu(word, 9, 23, a as u64)?;

    Some(word)
}
//-> Array2<Option<u64>>
//-> Vec<Option<u64>>
fn process_blocks(array: Array2<(f32, f32, f32)>) -> Vec<Option<u64>>{
    let mut codewords = Vec::new();
    let blocks_across = array.width() / 2;
    let blocks_down = array.height() / 2;

    for block_row in 0..blocks_down {
        for block_col in 0..blocks_across {
            let mut y_values = [0i64; 4];
            let mut pb_values = vec![];
            let mut pr_values = vec![];

            for row_offset in 0..2 {
                for col_offset in 0..2 {
                    let index = (block_row * 2 + row_offset) * array.width() + (block_col * 2 + col_offset);
                    if let Some(pixel) = array.data().get(index) {
                        y_values[row_offset * 2 + col_offset] = pixel.0 as i64;
                        pb_values.push(pixel.1);
                        pr_values.push(pixel.2);
                    }
                }
            }

            let pb_average: f32 = pb_values.iter().sum::<f32>() / pb_values.len() as f32;
            let pr_average: f32 = pr_values.iter().sum::<f32>() / pr_values.len() as f32;
            
            if let Some(codeword) = pack_values(y_values, pb_average, pr_average) {
                codewords.push(Some(codeword));
            } else {
                codewords.push(None);
            }
        }
    }

    codewords
    // Array2::from_row_major(array.width() / 2, array.height() / 2, codewords).unwrap()
    
}


// To create new array, we add values to a new array and then constuct from Row major order



// COMPRESS

// Read in file
// Create array2 DONE
// Trim array DONE
// Chnage values fom RGB to YPbPr DONE
// Create a new array in 2x2 blocks NEEDS WORK
// Iterate iover the blocks and compress them into a single 32-bit word THINK IS DONE
// Add the word to a vector DONE
// Return the vector DONE

// DECOMPRESS
// TODO