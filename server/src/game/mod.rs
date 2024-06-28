use std::fmt::Display;
use serde::{Deserialize, Serialize};

pub mod fen;
mod square;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
// Goes from 0 to 7
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_bit_position_index(index: usize) -> Self {
        let x = index % 8;
        let y = index / 8;
        Self { x: x as i32, y: y as i32 }
    }

    pub fn mail_box_index(&self) -> usize {
        (self.y * 16 + self.x) as usize
    }

    pub fn bit_position_index(&self) -> usize {
        (self.y * 8 + self.x) as usize
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x+1, self.y+1)
    }
}