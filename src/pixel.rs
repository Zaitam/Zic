use std::hash::{Hash, Hasher};

// The general RGB (To be RGBA) pixel
#[derive(Debug, Copy, Eq, Ord, PartialOrd)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl Hash for Pixel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.r.hash(state);
        self.g.hash(state);
        self.b.hash(state);
    }
}
impl PartialEq for Pixel {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}
impl Clone for Pixel {
    fn clone(&self) -> Pixel {
        Pixel { r: self.r, g: self.g, b: self.b }
    }
}