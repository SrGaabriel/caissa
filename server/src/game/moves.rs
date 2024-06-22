use std::collections::HashSet;

use crate::game::engine::BoardLogic;
use crate::game::{Coordinates, get_opposite_team, PieceType, Side, Team};

fn has_castling_space(board: &BoardLogic, x: i8, y: i8, side: Side) -> bool {
    let range = match side {
        Side::KingSide => x + 1..8,
        Side::QueenSide => 2..x,
    };
    for i in range {
        if board.get_piece_at(i, y).is_some() {
            return false;
        }
    }
    true
}

pub fn calculate_moves_for_pawn(
    board: &BoardLogic,
    team: Team,
    x: i8,
    y: i8,
    threats_only: bool,
) -> Vec<Coordinates> {
    let mut moves = Vec::new();
    let y_direction = board.get_y_orientation(team);
    if !threats_only && can_pawn_march(board, x, y, y_direction) {
        moves.push(Coordinates {
            x,
            y: y + y_direction,
        });
        if (team == Team::White && y == 2) || (team == Team::Black && y == 7) {
            if can_pawn_march(board, x, y, 2 * y_direction) {
                moves.push(Coordinates {
                    x,
                    y: (y + 2 * y_direction),
                });
            }
        }
    }
    let pawn_taking_directions: Vec<(i8, i8)> = vec![(1, y_direction), (-1, y_direction)];
    for (dx, dy) in pawn_taking_directions {
        let new_x = x + dx;
        let new_y = y + dy;
        if !board.is_position_valid(new_x, new_y) {
            continue;
        }
        if let Some(targeting_piece) = board.get_piece_at(new_x, new_y) {
            if targeting_piece.team != team {
                moves.push(Coordinates { x: new_x, y: new_y });
            }
        } else if let Some(en_passant_target) = board.state.en_passant_target_square.clone() {
            if en_passant_target.x == new_x && en_passant_target.y == new_y {
                moves.push(Coordinates { x: new_x, y: new_y });
            }
        }
    }
    moves
}

fn can_pawn_march(board: &BoardLogic, x: i8, y: i8, steps: i8) -> bool {
    let new_y = y + steps;
    if new_y > 8 || new_y < 1 {
        return false;
    }
    board.get_piece_at(x, new_y).is_none()
}

pub fn calculate_moves_for_knight(
    board: &BoardLogic,
    team: Team,
    x: i8,
    y: i8,
) -> Vec<Coordinates> {
    let knight_directions: Vec<(i8, i8)> = vec![
        (-1, 2),
        (1, 2),
        (-2, 1),
        (-2, -1),
        (2, 1),
        (2, -1),
        (-1, -2),
        (1, -2),
    ];
    let mut moves = Vec::new();
    for (dx, dy) in knight_directions {
        let new_x = (x as i8 + dx) as i8;
        let new_y = (y as i8 + dy) as i8;
        if board.is_position_valid(new_x, new_y) {
            if let Some(targeting_piece) = board.get_piece_at(new_x, new_y) {
                if targeting_piece.team != team {
                    moves.push(Coordinates { x: new_x, y: new_y });
                }
            } else {
                moves.push(Coordinates { x: new_x, y: new_y });
            }
        }
    }
    moves
}

pub fn calculate_moves_for_king(
    board: &BoardLogic,
    team: Team,
    x: i8,
    y: i8,
) -> Vec<Coordinates> {
    let king_queen_directions: Vec<(i8, i8)> = vec![
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
    ];
    let mut moves = Vec::new();
    for (dx, dy) in king_queen_directions {
        let new_x = x + dx;
        let new_y = y + dy;
        if board.is_position_valid(new_x, new_y) {
            if board.is_castling_available(team) {
                if has_castling_space(board, x, y, Side::QueenSide) {
                    moves.push(Coordinates { x: x - 2, y });
                }
                if has_castling_space(board, x, y, Side::KingSide) {
                    moves.push(Coordinates { x: x + 2, y });
                }
            }
            if let Some(targeting_piece) = board.get_piece_at(new_x, new_y) {
                if targeting_piece.team == team {
                    continue;
                }
            }
            moves.push(Coordinates { x: new_x, y: new_y });
        }
    }
    moves
}

pub fn calculate_moves_for_bishop(
    board: &BoardLogic,
    team: Team,
    x: i8,
    y: i8,
) -> Vec<Coordinates> {
    let bishop_directions = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];
    calculate_progressive_moves(board, team, x, y, bishop_directions)
}

pub fn calculate_moves_for_rook(board: &BoardLogic, team: Team, x: i8, y: i8) -> Vec<Coordinates> {
    let rook_directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    calculate_progressive_moves(board, team, x, y, rook_directions)
}

pub fn calculate_moves_for_queen(board: &BoardLogic, team: Team, x: i8, y: i8) -> Vec<Coordinates> {
    let queen_directions = vec![
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
    ];
    calculate_progressive_moves(board, team, x, y, queen_directions)
}

pub fn calculate_progressive_moves(
    board: &BoardLogic,
    team: Team,
    x: i8,
    y: i8,
    directions: Vec<(i8, i8)>,
) -> Vec<Coordinates> {
    let mut moves = Vec::new();
    for (dx, dy) in directions {
        let mut new_x = x;
        let mut new_y = y;
        loop {
            new_x += dx;
            new_y += dy;
            if !board.is_position_valid(new_x, new_y) {
                break;
            }
            if let Some(targeting_piece) = board.get_piece_at(new_x, new_y) {
                if targeting_piece.team != team {
                    moves.push(Coordinates { x: new_x, y: new_y });
                }
                break;
            } else {
                moves.push(Coordinates { x: new_x, y: new_y });
            }
        }
    }
    moves
}

// Here we'll check in all directions: knights, queens, pawns and kings
// Then, if one of the directions finds an opponent threat, we'll return true
pub fn is_space_threatened(space: &Coordinates, board: &BoardLogic, team: Team) -> bool {
    let pawn_moves: Vec<Coordinates> = calculate_moves_for_pawn(board, get_opposite_team(team), space.x, space.y, true);
    if pawn_moves.contains(space) {
        return true;
    }

    let rook_moves: Vec<Coordinates> = calculate_moves_for_queen(board, team, space.x, space.y);
    if check_if_piece_is_inside_coords(board, rook_moves, vec![PieceType::Rook, PieceType::Queen]) {
        return true;
    }
    let bishop_moves: Vec<Coordinates> = calculate_moves_for_bishop(board, team, space.x, space.y);
    if check_if_piece_is_inside_coords(board, bishop_moves, vec![PieceType::Bishop, PieceType::Queen]) {
        return true;
    }
    let knight_moves: Vec<Coordinates> = calculate_moves_for_knight(board, team, space.x, space.y);
    if check_if_piece_is_inside_coords(board, knight_moves, vec![PieceType::Knight]) {
        return true;
    }
    let king_moves: Vec<Coordinates> = calculate_moves_for_king(board, team, space.x, space.y);
    if check_if_piece_is_inside_coords(board, king_moves, vec![PieceType::King]) {
        return true;
    }
    return false;
}

fn check_if_piece_is_inside_coords(board: &BoardLogic, coordinates: Vec<Coordinates>, pieces: Vec<PieceType>) -> bool {
    for coord in coordinates {
        if !board.is_position_valid(coord.x, coord.y) {
            continue;
        }
        if let Some(piece) = board.get_piece_at(coord.x, coord.y) {
            if pieces.contains(&piece.piece_type) {
                return true;
            }
        }
    }
    false
}