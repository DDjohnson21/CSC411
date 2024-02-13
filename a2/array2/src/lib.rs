#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Array2<T: Clone> {
    width: usize,
    height: usize,
    elements: Vec<T>, // 1D array to store elements
}

impl<T: Clone> Array2<T> {
    // Creates a new Array with uninitialized elements.
    pub fn new(width: usize, height: usize) -> Self {
    }

    // Constructs an Array in column-major
    pub fn from_col_major(elements: Vec<T>, width: usize, height: usize) -> Self {
    }

    // Constructs an Array in row-major 
    pub fn from_row_major(elements: Vec<T>, width: usize, height: usize) -> Self {
    }

    // Iterator over the elements in row-major order.
    pub fn iter_row_major(&self) -> impl Iterator<Item = &T> {
    }

    // Iterator over the elements in column-major order.
    pub fn iter_col_major(&self) -> impl Iterator<Item = &T> {
    }

    // Finds an element in the array and returns its position.
    pub fn find_element(&self, target: &T) -> Option<(usize, usize)> {
    }
}