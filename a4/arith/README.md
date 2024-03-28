# At4: Arith 
### Partners

Matt Boekamp and Damien Johnson

### Help

We attended TA Help hours for some of the issues we had with compression. This helped resolve some of our issues but we were unsure of several others.

### Correctly Implemented

We believe we have correctly implemented the majority of the following:
1. Newu
2. News
3. Convert RGB to RGB Float
4. Convert RGB Float to YPbPr
5. Convert YPbPr array to block major order
6. Use quantization to compress the image
7. Fits
8. Fitu

In our compression, we ran into issues somewhere in the proces above. After reviewing our code, we are unsure of where we went wrong. We believe that the issue lies in converting the array to block major order or within accessing data in the quantization. We spent many hours trying to resolve this issue but were unable to.

Due to this, we did not attempt decompression. Firstly, we had some trouble grasping the concepts behind decompression and how to correctly implement them. Secondly, we prioritized getting compression to work over decompression. We believed that if we can get compression to work, decompression will be a much simpler task.

 ### Architecture

1. **Image Reading and Preprocessing**: 
   - csc411_image and prepare_array function.
   - Functionality:: Reads an image file and preprocesses it by trimming to even dimensions.

2. **Color Space Conversion**:
   - rgb_float_conversion and rgb_float_ypbpr_conversion.
   - Functionality:: Converts RGB values to floating-point representations and then to YPbPr color space.

3. **Block Processing**:
   - process.
   - Functionality:: Processes the image in 2x2 blocks, converting YPbPr values to a compressed format.

4. **Compression and Decompression**:
   -  codec module with compress and decompress functions.
   - Functionality:: Encodes the processed blocks into a compressed format.

5. **Utility Structures and Functions**:
   - Structures like Rgb, Rgbfloat, and YPbPr represent different color formats.
   - Functions for conversion between formats and for packing the data into compressed form.

### Time

This assignment took us approximately 10 hours to analyze and 15 hours to implement.





