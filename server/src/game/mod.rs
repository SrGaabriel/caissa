pub mod fen;
mod square;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
// Goes from 0 to 7
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn mail_box_index(&self) -> usize {
        (self.y * 16 + self.x) as usize
    }

    pub fn bit_position_index(&self) -> usize {
        (self.y * 8 + self.x) as usize
    }
}