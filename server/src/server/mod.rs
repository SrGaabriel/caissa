use axum::http::StatusCode;
use axum::Json;
use serde::{Serialize, Deserialize};
use crate::ai::ChessAI;
use crate::game::engine::BoardLogic;
use crate::game::{Coordinates, Move};

pub async fn get_team_moves(
    Json(payload): Json<TeamMovesRequest>
) -> (StatusCode, Json<Moves>) {
    let board = BoardLogic::from_fen(&payload.fen);
    let team = match payload.team {
        0 => crate::game::Team::White,
        1 => crate::game::Team::Black,
        _ => return (StatusCode::BAD_REQUEST, Json(Moves { moves: Vec::new() }))
    };
    let moves = board.get_all_moves_for_team(team);
    (StatusCode::OK, Json(Moves { moves }))
}

pub async fn get_piece_moves(
    Json(payload): Json<PieceMovesRequest>
) -> (StatusCode, Json<Moves>) {
    let board = BoardLogic::from_fen(&payload.fen);
    let moves = board.calculate_moves_for(payload.coordinates.x, payload.coordinates.y);
    (StatusCode::OK, Json(Moves { moves }))
}

pub async fn get_best_move(
    Json(payload): Json<BestMoveRequest>
) -> (StatusCode, Json<Move>) {
    let board = BoardLogic::from_fen(&payload.fen);
    let mut ai = crate::ai::negamax::Negamax::new();
    let best_move = ai.get_best_move(board, 1).unwrap();
    (StatusCode::OK, Json(best_move))
}

#[derive(Deserialize)]
pub struct TeamMovesRequest {
    fen: String,
    team: u8
}

#[derive(Deserialize)]
pub struct BestMoveRequest {
    fen: String
}

#[derive(Deserialize)]
pub struct PieceMovesRequest {
    fen: String,
    coordinates: Coordinates
}

#[derive(Serialize)]
pub struct Moves {
    moves: Vec<Move>
}