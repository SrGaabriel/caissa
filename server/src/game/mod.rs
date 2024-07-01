use std::fmt::Display;
use serde::{Deserialize, Serialize};
use crate::board::Piece;

pub mod fen;
mod square;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
// Goes from 0 to 7
pub struct Vector {
    pub x: u8,
    pub y: u8,
}

impl Vector {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn from_bit_position_index(index: usize) -> Self {
        let x = index % 8;
        let y = index / 8;
        Self { x: x as u8, y: y as u8 }
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

pub struct Move {
    pub origin: Vector,
    pub target: Vector,
    pub check: bool,
    pub checkmate: bool,
    pub promotion: Option<Piece>,
    pub en_passant: bool,
    pub castling_side: Option<u8>
}

pub fn get_pawn_rank_for_team(team: u8) -> u8 {
    if team == 0 { 1 } else { 6 }
}

pub fn get_en_passant_rank_for_team(team: u8) -> u8 {
    if team == 0 { 3 } else { 4 }
}