use crate::board::{BitBoard, BitPosition, get_opposite_team, Pieces, Team, Teams};

pub fn calculate_pawn_move(position: &BitPosition, team: Team, pawn_bit: usize) -> BitBoard {
    let pawn = BitBoard(1 << pawn_bit);
    let empty = position.empty_squares();
    let enemy_pieces = position.get_team_pieces(get_opposite_team(team));
    let pawn_march_moves = mask_pawn_march_moves(pawn, empty, team);
    let pawn_capture_moves = mask_pawn_capture_moves(pawn, enemy_pieces, team);
    pawn_march_moves | pawn_capture_moves
}

pub fn calculate_all_pawn_moves(position: BitPosition, team: Team) -> BitBoard {
    let pawns = position.get_pieces(team, Pieces::PAWN);
    let empty = position.empty_squares();
    let enemy_pieces = position.get_team_pieces(get_opposite_team(team));
    let pawn_march_moves = mask_pawn_march_moves(pawns, empty, team);
    let pawn_capture_moves = mask_pawn_capture_moves(pawns, enemy_pieces, team);
    pawn_march_moves | pawn_capture_moves
}

pub fn mask_pawn_march_moves(pawns: BitBoard, empty: BitBoard, team: Team) -> BitBoard {
    let pawn_one_forward_moves = pawns.shift_up(8, &team) & empty;

    let third_rank_mask = if team == Teams::WHITE { BitBoard(0xFF << 16) } else { BitBoard(0xFF << 40) };
    let third_rank_pawns = pawn_one_forward_moves & third_rank_mask;
    let pawn_two_forward_moves = third_rank_pawns.shift_up(8, &team) & empty;

    pawn_one_forward_moves | pawn_two_forward_moves
}

pub fn mask_pawn_capture_moves(pawns: BitBoard, enemy_pieces: BitBoard, team: Team) -> BitBoard {
    let left_pawns = pawns.shift_up(9, &team);
    let right_pawns = pawns.shift_up(7, &team);
    (left_pawns | right_pawns) & enemy_pieces
}