/*
The Array2<T> structure is a generic two-dimensional array implementation in Rust, designed to efficiently store 
and manipulate grid-like data structures. It is particularly suited for applications such as game development and 
scientific computing where two-dimensional data structures are commonly used. The structure ensures that all rows
and columns maintain consistent lengths, thereby preserving the rectangular shape of the data.
*/

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
// `T`: The type of elements stored in the array. `T` must implement the `Clone`, `Default`, `PartialEq`, and
// `std::fmt::Display` traits. These constraints ensure that elements can be duplicated, a default value can be
// generated, elements can be compared for equality, and they can be displayed.
pub struct Array2<T: Clone> {
    width: usize,
    height: usize,
    // 1D array to store elements
    // This will satisfy the invariant that in the two dimensional array, each row will have the same number of columns. 
    // If this is followed, the two dimensional array will maintain the correct rectangle/square shape and be accessible through both row-major and column-major.
    elements: Vec<T>,
}

impl<T> Array2<T> where T: Clone + Default + PartialEq + std::fmt::Display{
    // Creates a new Array with uninitialized elements.
    pub fn new(elements: Vec<T>, rows: usize, cols: usize) -> Self {
        Array2 {
            width: cols,
            height: rows,
            elements
        }
    }

    // Public getter methods
    pub fn elements(&self) -> &Vec<T> {
        &self.elements
    }
    pub fn width(&self) -> &usize {
        &self.width
    }
    pub fn height(&self) -> &usize {
        &self.height
    }

    // Constructs an Array in column-major
    // `elements`: A vector of elements in column-major order.
    // `width`: The number of columns in the array.
    // `height`: The number of rows in the array.
    // Returns an `Array2<T>` instance with elements ordered in column-major.
    pub fn from_col_major(elements: Vec<T>, width: usize, height: usize) -> Self {
        
        // Create a new vector to store the elements in column-major order
        let mut col_major_elements = vec![T::default(); width * height];
        for i in 0..height {
            for j in 0..width {
                // Copy the elements from the input vector to the new vector
                col_major_elements[i * width + j] = elements[j * height + i].clone();
            }
        }
        // Construct the new Array
        Array2 {
            width,
            height,
            elements: col_major_elements,
        }
    }

    // Constructs an Array in row-major 
    // `elements`: A vector of elements in row-major order.
    // `width`: The number of columns in the array.
    // `height`: The number of rows in the array.
    // Returns an `Array2<T>` instance with elements ordered in row-major.
    pub fn from_row_major(elements: Vec<T>, width: usize, height: usize) -> Self {
        Array2 {
            width,
            height,
            elements,
        }
    }

    // Iterator over the elements in row-major order.
    // Returns an iterator over the elements in row-major order.
    pub fn iter_row_major(&self) -> impl Iterator<Item = &T> {
        self.elements.iter()
    }

    // Iterator over the elements in column-major order.
    // Returns an iterator over the elements in column-major order.
    pub fn iter_col_major(&self) -> impl Iterator<Item = &T> {
        (0..self.width * self.height).map(move |index| {
            let col = index / self.height;
            let row = index % self.height;
            &self.elements[row * self.width + col]
        })
    }   

    // Finds an element in the array and returns its position.
    // `target`: The element to search for.
    // Returns the position of the element in the array if found, otherwise `None`.
    pub fn find_element(&self, target: &T) -> Option<(usize, usize)> {
        for i in 0..self.height {
            for j in 0..self.width {
                if &self.elements[i * self.width + j] == target {
                    return Some((i, j));
                }
            }
        }
        None
    }

    // Function to determine if a row is valid
    // `row`: The row to check for validity.
    // `to_usize`: A function that converts an element to a `usize`.
    // Returns `true` if the row is valid, otherwise `false`.
    pub fn valid_row<F>(&self, row: usize, to_usize: F) -> bool
    // Trait specifying that F must be a function or a closure that takes a reference to T and returns an Option<usize>
    where
        F: Fn(&T) -> Option<usize>,
    {
        // Using a frequency array to check for duplicates
        let mut frequency = [0; 9];
        for i in 0..self.width {
            let index = row * self.width + i;
            // Access the element at the index and attempt to convert it to a usize
            if let Some(value) = to_usize(&self.elements[index]).map(|v| v - 1) {
                // Check if the value is within the valid range for a Sudoku number
                if value >= 9 {
                    return false;
                }
                // Check if the number has already been encountered in the row
                if frequency[value] > 0 {
                    return false;
                }
                // If not, increment
                frequency[value] += 1;
            } else {
                // Handle the case where conversion is not possible or not in the expected range
                return false;
            }
        }
        true
    }
    
    // Function to determine if a column is valid
    // `col`: The column to check for validity.
    // `to_usize`: A function that converts an element to a `usize`.
    // Returns `true` if the column is valid, otherwise `false`.
    pub fn valid_col<F>(&self, col: usize, to_usize: F) -> bool
    where
        F: Fn(&T) -> Option<usize>,
    {
        // Using a frequency array to check for duplicates
        // Functions the same as the frequency array in valid_row
        let mut frequency = [0; 9];
        for i in 0..self.height {
            let index = i * self.width + col;
            if let Some(value) = to_usize(&self.elements[index]).map(|v| v - 1) {
                if value >= 9 {
                    return false;
                }
                if frequency[value] > 0 {
                    return false;
                }
                frequency[value] += 1;
            } else {
                return false;
            }
        }
        true
    }

    // Function to determine if a subgrid is valid
    // `row`: The row of the top-left corner of the subgrid.
    // `col`: The column of the top-left corner of the subgrid.
    // `to_usize`: A function that converts an element to a `usize`.
    pub fn valid_subgrid<F>(&self, row: usize, col: usize, to_usize: F) -> bool
    where
        F: Fn(&T) -> Option<usize>,
    {
        let mut frequency = [0; 9];
        for i in 0..3 {
            for j in 0..3 {
                // Calculate the index for the current element in the subgrid
                let index = (row + i) * self.width + (col + j);
                // Attempt to convert the element to a usize and adjust for 1-based indexing
                if let Some(value) = to_usize(&self.elements[index]).map(|v| v - 1) {
                    // Check if the value is within the valid range for a Sudoku number
                    if value >= 9 {
                        return false;
                    }
                    // Check if the number has already been encountered in the subgrid
                    if frequency[value] > 0 {
                        return false;
                    }
                    frequency[value] += 1;
                } else {
                    // Conversion failed or value out of range
                    return false;
                }
            }
        }
        true
    }

    // Function to determine if the entire Sudoku is valid
    // `to_usize`: A function that converts an element to a `usize`.
    // Returns `true` if the Sudoku is valid, otherwise `false`.
    pub fn valid_sudoku<F>(&self, to_usize: F) -> bool
    where
        F: Fn(&T) -> Option<usize> + Copy,
    {
        // Check each row, column, and subgrid
        for i in 0..self.height {
            if !self.valid_row(i, to_usize) {
                return false;
            }
        }
        for i in 0..self.width {
            if !self.valid_col(i, to_usize) {
                return false;
            }
        }
        for i in (0..self.height).step_by(3) {
            for j in (0..self.width).step_by(3) {
                if !self.valid_subgrid(i, j, to_usize) {
                    return false;
                }
            }
        }
        true
    }




    // added for testing 
    // convert rgb to graysclae image? 
    pub fn rgb_to_gray(&self) -> Array2<f64> {
        let mut gray_elements = vec![0.0; self.width * self.height];
        for i in 0..self.height {
            for j in 0..self.width {
                let index = i * self.width + j;
                let r = self.elements[index].clone();
                let g = self.elements[index + self.width * self.height].clone();
                let b = self.elements[index + 2 * self.width * self.height].clone();
                gray_elements[index] = 0.299 * r + 0.587 * g + 0.114 * b;
            }
        }
        Array2 {
            width: self.width,
            height: self.height,
            elements: gray_elements,
        }
    }
}