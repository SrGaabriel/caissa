use crate::board::{BitBoard, BitPosition};

pub struct ChessBoard {
    pub bits: BitPosition
}

impl ChessBoard {
    pub fn new(bits: BitPosition) -> ChessBoard {
        ChessBoard {
            bits
        }
    }
}