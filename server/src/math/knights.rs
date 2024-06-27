use crate::board::{BitBoard, BitPosition, Pieces};
use crate::math::{NOT_A_FILE, NOT_H_FILE};

const NOT_AB_FILE: u64 = 0xfcfcfcfcfcfcfcfc;
const NOT_HG_FILE: u64 = 0x3f3f3f3f3f3f3f3f;

pub fn calculate_all_knight_moves(position: &BitPosition, team: usize) -> BitBoard {
    let team_pieces = position.get_team_pieces(team);
    let knights = position.get_pieces(team, Pieces::KNIGHT);
    mask_all_knight_moves(&knights) & !team_pieces
}

pub fn mask_knight_moves(knight_bit: u64) -> BitBoard {
    let mut moves = 0;
    if ((knight_bit >> 17) & NOT_H_FILE) != 0 { moves |= knight_bit >> 17; }
    if ((knight_bit >> 15) & NOT_A_FILE) != 0 { moves |= knight_bit >> 15; }
    if ((knight_bit >> 10) & NOT_HG_FILE) != 0 { moves |= knight_bit >> 10; }
    if ((knight_bit >> 6) & NOT_AB_FILE) != 0 { moves |= knight_bit >> 6; }
    if ((knight_bit << 17) & NOT_A_FILE) != 0 { moves |= knight_bit << 17; }
    if ((knight_bit << 15) & NOT_H_FILE) != 0 { moves |= knight_bit << 15; }
    if ((knight_bit << 10) & NOT_AB_FILE) != 0 { moves |= knight_bit << 10; }
    if ((knight_bit << 6) & NOT_HG_FILE) != 0 { moves |= knight_bit << 6; }
    BitBoard(moves)
}

pub fn mask_all_knight_moves(knights: &BitBoard) -> BitBoard {
    let mut moves = BitBoard(0);
    let mut bitboard = knights.0;

    while bitboard != 0 {
        let knight = 1 << bitboard.trailing_zeros();
        bitboard ^= knight;

        let attacks = mask_knight_moves(knight);
        moves.0 |= attacks.0;
    }

    moves
}