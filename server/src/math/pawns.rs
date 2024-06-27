use crate::board::{BitBoard, Team, Teams};

pub fn mask_all_pawn_moves(pawns: &BitBoard, empty: &BitBoard, enemy_pieces: &BitBoard, en_passant_square: &Option<usize>, team: Team) -> BitBoard {
    let pawn_march_moves = mask_pawn_march_moves(pawns, *empty, team);
    let pawn_capture_moves = mask_pawn_capture_moves(pawns, enemy_pieces, en_passant_square, team);
    pawn_march_moves | pawn_capture_moves
}

pub fn mask_pawn_march_moves(pawns: &BitBoard, empty: BitBoard, team: Team) -> BitBoard {
    let pawn_one_forward_moves = pawns.shift_up(8, &team) & empty;

    let third_rank_mask = if team == Teams::WHITE { BitBoard(0xFF << 16) } else { BitBoard(0xFF << 40) };
    let third_rank_pawns = pawn_one_forward_moves & third_rank_mask;
    let pawn_two_forward_moves = third_rank_pawns.shift_up(8, &team) & empty;

    pawn_one_forward_moves | pawn_two_forward_moves
}

pub fn mask_pawn_capture_moves(pawns: &BitBoard, enemy_pieces: &BitBoard, en_passant_square: &Option<usize>, team: Team) -> BitBoard {
    let left_pawns = pawns.shift_up(9, &team);
    let right_pawns = pawns.shift_up(7, &team);
    match en_passant_square {
        None => BitBoard((left_pawns | right_pawns).0 & enemy_pieces.0),
        Some(square) => (left_pawns | right_pawns) & BitBoard(1u64 << square)
    }
}