/*
    Traits are collections of methods defined for an unknown type "Self".
*/
pub trait Sqrt {
    // "Self" refers to the implementor type.
    fn sqrt(self) -> Self;
}

// Implement Sqrt trait for primitive Rust types
impl Sqrt for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

impl Sqrt for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}