use crate::board::{BitBoard, BitPosition, Pieces, Team};

pub fn calculate_king_moves(position: &BitPosition, team: Team) -> BitBoard {
    let king = position.get_pieces(team, Pieces::KING);
    let king_moves = mask_king_moves(king.0);
    king_moves
}

pub fn mask_king_moves(king_bit: u64) -> BitBoard {
    let not_a_file = 0xfefefefefefefefe;
    let not_h_file = 0x7f7f7f7f7f7f7f7f;

    let mut moves = 0;
    if (king_bit >> 1) & not_h_file != 0 { moves |= king_bit >> 1; }
    if (king_bit << 1) & not_a_file != 0 { moves |= king_bit << 1; }
    if king_bit >> 8 != 0 { moves |= king_bit >> 8; }
    if king_bit << 8 != 0 { moves |= king_bit << 8; }
    if (king_bit >> 9) & not_h_file != 0 { moves |= king_bit >> 9; }
    if (king_bit << 7) & not_a_file != 0 { moves |= king_bit << 7; }
    if (king_bit >> 7) & not_a_file != 0 { moves |= king_bit >> 7; }
    if (king_bit << 9) & not_h_file != 0 { moves |= king_bit << 9; }

    BitBoard(moves)
}