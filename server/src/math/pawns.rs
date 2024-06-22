use crate::board::hybrid::HybridChessBoard;
use crate::board::{BitBoard, BitPosition, get_opposite_team, Pieces, Team, Teams};
use crate::game::Vector;

pub fn calculate_pawn_move(board: &HybridChessBoard, vector: Vector) -> BitBoard {
    let bit = vector.mail_box_index();
    let team = board.mail_box.get_piece_at(bit).unwrap();
    let piece = board.mail_box.get_piece_at(bit).unwrap();

}

pub fn calculate_all_pawn_moves(position: BitPosition, team: Team) -> BitBoard {
    let pawn_march_moves = calculate_all_pawn_march_moves(&position, team);
    let pawn_capture_moves = calculate_all_pawn_capture_moves(&position, team);
    pawn_march_moves | pawn_capture_moves
}

pub fn calculate_all_pawn_march_moves(position: &BitPosition, team: Team) -> BitBoard {
    let pawns = position.get_pieces(team, Pieces::PAWN);
    let empty_squares = position.empty_squares();

    let pawn_one_forward_moves = pawns.shift_up(8, &team) & empty_squares;

    let third_rank_mask = if team == Teams::WHITE { BitBoard(0xFF << 16) } else { BitBoard(0xFF << 40) };
    let third_rank_pawns = pawn_one_forward_moves & third_rank_mask;
    let pawn_two_forward_moves = third_rank_pawns.shift_up(8, &team) & empty_squares;

    pawn_one_forward_moves | pawn_two_forward_moves
}

pub fn calculate_all_pawn_capture_moves(position: &BitPosition, team: Team) -> BitBoard {
    let pawns = position.get_pieces(team, Pieces::PAWN);
    let enemy_pieces = position.get_team_pieces(get_opposite_team(team));
    let left_pawns = pawns.shift_up(9, &team);
    let right_pawns = pawns.shift_up(7, &team);
    (left_pawns | right_pawns) & enemy_pieces
}
