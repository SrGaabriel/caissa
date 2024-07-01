use crate::board::{BitBoard, Team, Teams};
use crate::math::{NOT_A_FILE, NOT_H_FILE};

pub fn mask_pawn_moves(pawn_bit: u64, empty: &BitBoard, opponent_pieces: &BitBoard, en_passant_square: &Option<usize>, team: Team) -> BitBoard {
    let pawn_march_moves = mask_pawn_march_moves(pawn_bit, empty, team);
    let pawn_capture_moves = mask_pawn_capture_moves(pawn_bit, opponent_pieces, en_passant_square, team);
    pawn_march_moves | pawn_capture_moves
}

pub fn mask_pawn_march_moves(pawn_bit: u64, empty: &BitBoard, team: Team) -> BitBoard {
    let pawn_one_forward_moves = shift_bit_up(pawn_bit, 8, &team) & empty.0;
    if pawn_one_forward_moves == 0 { return BitBoard(0) };

    let third_rank_mask = if team == Teams::WHITE { 0xFF << 16 } else { 0xFF << 40 };
    let third_rank_pawns = pawn_one_forward_moves & third_rank_mask;
    let pawn_two_forward_moves = shift_bit_up(third_rank_pawns, 8, &team) & empty.0;

    BitBoard(pawn_one_forward_moves | pawn_two_forward_moves)
}

fn shift_bit_up(bit: u64, shift: u8, team: &Team) -> u64 {
    match *team {
        Teams::WHITE => bit << shift,
        Teams::BLACK => bit >> shift,
        _ => panic!("Invalid team")
    }
}

pub fn mask_pawn_capture_moves(pawn_bit: u64, enemy_pieces: &BitBoard, en_passant_square: &Option<usize>, team: Team) -> BitBoard {
    let left_edge_mask = if team == Teams::WHITE { NOT_H_FILE } else { NOT_A_FILE };
    let right_edge_mask = if team == Teams::WHITE { NOT_A_FILE } else { NOT_H_FILE };
    let left_pawn = BitBoard(pawn_bit & left_edge_mask).shift_up(9, &team);
    let right_pawn = BitBoard(pawn_bit & right_edge_mask).shift_up(7, &team);
    match en_passant_square {
        None => BitBoard((left_pawn.0 | right_pawn.0) & enemy_pieces.0),
        Some(square) => (left_pawn | right_pawn) & BitBoard(1u64 << square)
    }
}

pub fn mask_all_pawn_moves(pawns: &BitBoard, empty: &BitBoard, enemy_pieces: &BitBoard, en_passant_square: &Option<usize>, team: Team) -> BitBoard {
    let pawn_march_moves = mask_all_pawn_march_moves(pawns, empty, team);
    let pawn_capture_moves = mask_all_pawn_capture_moves(pawns, enemy_pieces, en_passant_square, team);
    pawn_march_moves | pawn_capture_moves
}

pub fn mask_all_pawn_march_moves(pawns: &BitBoard, empty: &BitBoard, team: Team) -> BitBoard {
    let pawn_one_forward_moves = pawns.shift_up(8, &team) & empty;
    if pawn_one_forward_moves.0 == 0 { return pawn_one_forward_moves };

    let third_rank_mask = if team == Teams::WHITE { BitBoard(0xFF << 16) } else { BitBoard(0xFF << 40) };
    let third_rank_pawns = pawn_one_forward_moves & third_rank_mask;
    let pawn_two_forward_moves = third_rank_pawns.shift_up(8, &team) & empty;

    pawn_one_forward_moves | pawn_two_forward_moves
}

pub fn mask_all_pawn_capture_moves(pawns: &BitBoard, enemy_pieces: &BitBoard, en_passant_square: &Option<usize>, team: Team) -> BitBoard {
    let left_edge_mask = if team == Teams::WHITE { NOT_H_FILE } else { NOT_A_FILE };
    let right_edge_mask = if team == Teams::WHITE { NOT_A_FILE } else { NOT_H_FILE };
    let left_pawns = BitBoard(pawns.0 & left_edge_mask).shift_up(9, &team);
    let right_pawns = BitBoard(pawns.0 & right_edge_mask).shift_up(7, &team);
    match en_passant_square {
        None => BitBoard((left_pawns | right_pawns).0 & enemy_pieces.0),
        Some(square) => (left_pawns | right_pawns) & BitBoard(1u64 << square)
    }
}