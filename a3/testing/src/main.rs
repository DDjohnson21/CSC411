use std::fmt;

// Define the RGB struct
struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

// Implement the Debug trait using derive
#[derive(Debug)]
struct RGBDebug {
    r: u8,
    g: u8,
    b: u8,
}

// Implement the Display trait for RGB
impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.g, self.b)
    }
}

fn main() {
    let color = RGB { r: 255, g: 165, b: 0 };
    let color_debug = RGBDebug { r: 255, g: 165, b: 0 };

    // Print using Display
    println!("{}", color);

    // Print using Debug
    println!("{:?}", color_debug);
}