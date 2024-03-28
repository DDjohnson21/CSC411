// Struct to represent YPbPr color space
pub struct YPbPr {
    pub y: f32,
    pub pb: f32,
    pub pr: f32,
}
// Implement the Clone trait for YPbPr
impl Clone for YPbPr {
    fn clone(&self) -> Self {
        Self {
            y: self.y,
            pb: self.pb,
            pr: self.pr,
        }
    }
}