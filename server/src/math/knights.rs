use lazy_static::lazy_static;
use crate::board::{BitBoard, BitPosition, Piece, Pieces, Team};

pub fn calculate_knight_moves(position: &BitPosition, team: usize, knight_bit: usize) -> BitBoard {
    let knight = BitBoard(1 << knight_bit);
    mask_knight_moves(&knight)
}

pub fn calculate_all_knight_moves(position: &BitPosition, team: usize) -> BitBoard {
    let knights = position.get_pieces(team, Pieces::KNIGHT);
    mask_knight_moves(&knights)
}


pub fn mask_knight_moves(knights: &BitBoard) -> BitBoard {
    let knights = knights.0;

    let not_a_file = 0xfefefefefefefefe;
    let not_h_file = 0x7f7f7f7f7f7f7f7f;
    let not_ab_file = 0xfcfcfcfcfcfcfcfc;
    let not_gh_file = 0x3f3f3f3f3f3f3f3f;

    // Calculate the potential moves
    let mut moves = 0;

    moves |= (knights & not_a_file) << 6; // 2 up 1 left
    moves |= (knights & not_h_file) << 10; // 2 up 1 right
    moves |= (knights & not_a_file) >> 10; // 2 down 1 left
    moves |= (knights & not_h_file) >> 6; // 2 down 1 right
    moves |= (knights & not_ab_file) << 15; // 1 up 2 left
    moves |= (knights & not_gh_file) << 17; // 1 up 2 right
    moves |= (knights & not_ab_file) >> 17; // 1 down 2 left
    moves |= (knights & not_gh_file) >> 15; // 1 down 2 right

    BitBoard(moves)
}
