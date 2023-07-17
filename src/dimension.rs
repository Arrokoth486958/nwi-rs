#![allow(unused)]

#[derive(Copy, Clone)]
pub struct Size {
    width: f64,
    height: f64
}

impl Size {
    pub const ZERO: Size = Size::new(0.0, 0.0);

    pub const fn new(width: f64, height: f64) -> Self {
        Size {
            width,
            height
        }
    }
}