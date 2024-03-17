# Locality

## Partners
Matt Boekamp and Damien Johnson

## Help
We used the official array2 solution. We did this to simplify the process as we knew that our array2 lacked some of the functions the official one had.

## Correctly Implemented
The majority of our program has been corrected implemented. We could not get 90 degree rotation to properly work as we rotate pixels by 90 but the rotated image outputs incorrectly. The new rotated image has 4 copies on it. We spent significant time figuring out what this was but we were unsure.

We managed to get image transformations to work to an extent. One of the last things we implemented was the command line argument handling. After doing this, we were unable to get the program to work to the same point again. We are unsure of the reason why. 

## Part C
For reference, our intial estimates were:

1. 180-Degree Rotation (Row-Major Access): Likely the fastest due to optimal spatial locality.
2. 90-Degree Rotation (Row-Major Access): Slower due to less efficient access pattern but still better than column-major.
3. 180-Degree Rotation (column-Major Access): Slower due to poor spatial locality compared to row-major.
4. 90-Degree Rotation (Column-Major Access): The slowest due to the most inefficient access pattern.

### Results

1. Mallory.ppm

90 Degree Column Major: 1.15s
90 Degree Row Major: 809.26ms
180 Degree Column Major: 1.17s
180 Degree Row Major: 729.34ms

2. Moss.ppm

90 Degree Column Major: 55.97ms
90 Degree Row Major: 55.78ms
180 Degree Column Major: 58.75ms
180 Degree Row Major: 46.81ms

3. Additional.ppm

90 Degree Column Major: 48.79ms
90 Degree Row Major: 48.98ms
180 Degree Column Major: 50.56ms
180 Degree Row Major: 38.32ms

4. forest.ppm

90 Degree Column Major: 1.44s
90 Degree Row Major: 1.41s
180 Degree Column Major: 1.44s
180 Degree Row Major: 958.04ms


Overall, we mostly confirmed our intiali estimates. Using row major iteration was always faster than column major. The only difference between the estimates and our results was 90 degree column major was slightly faster than 180 degree column major. We believed 90 degree column major rotation had the worst access pattern and therefore would have the longest runtime. Overall, the difference between the two was not significant as the time difference was only 20ms for mallory.ppm. These estimates are based off our image rotation functions which functioned as expected for both 180 rotations but not for 90 degree rotations. This could contribute to the difference between our expectation and the results. When we used a larger file, forest.ppm, the difference more significant. 180 degree row major was 500ms faster than 180 degree column major and both 90 degree rotations. We tested several other ppm images other than the four listed here. The difficulty was finding one large enough to have a significant rotation time. Overall, our intiali confirmations were mostly correct. 

## Part D
A Row and Column Major Hybrid layout that alternates between row-major and column-major for different sections of the array. For example, if you divide the array into larger fragments and store each fragment in row-major or column-major. This could balance cache locality for both patterns. 

## Time
We spent approximately 20 hours completeing this assignment.

## Area of Issues 
We spent a lot of time working on bugs and ran into some issues we were not able to solve. The issues we were unable to solve revolved around rotation 90. Rotation 180 works for both versions of row and column rotation. But for neither one of our Rotation 90 was working where we where geting 4  merged images. We also had major issues using CLAP as we where unfamilar with it and cound not properley get it to work.