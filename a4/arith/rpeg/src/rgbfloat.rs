// Struct of Rgbfloat
pub struct Rgbfloat {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}
// Implement the Clone trait for Rgbfloat
impl Clone for Rgbfloat {
    fn clone(&self) -> Self {
        Self {
            red: self.red,
            green: self.green,
            blue: self.blue,
        }
    }
}