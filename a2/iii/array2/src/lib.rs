#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
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
    pub fn from_row_major(elements: Vec<T>, width: usize, height: usize) -> Self {
        Array2 {
            width,
            height,
            elements,
        }
    }

    // Iterator over the elements in row-major order.
    pub fn iter_row_major(&self) -> impl Iterator<Item = &T> {
        self.elements.iter()
    }

    // Iterator over the elements in column-major order.
    pub fn iter_col_major(&self) -> impl Iterator<Item = &T> {
        (0..self.width * self.height).map(move |index| {
            let col = index / self.height;
            let row = index % self.height;
            &self.elements[row * self.width + col]
        })
    }   

    // Finds an element in the array and returns its position.
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
}