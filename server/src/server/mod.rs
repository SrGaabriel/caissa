use std::thread;

use axum::{Extension, Json};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::board::PossibleMove;
use crate::engine::ChessEngine;
use crate::game::{fen, Vector};
use crate::SharedState;

pub async fn get_team_moves(
    Json(payload): Json<TeamMovesRequest>
) -> (StatusCode, Json<Moves>) {
    let board = fen::new_board(&payload.fen);
    if board.is_none() {
        return (StatusCode::BAD_REQUEST, Json(Moves { moves: Vec::new() }))
    }
    let board = board.unwrap();
    let moves = board.generate_moves(payload.team as usize);
    (StatusCode::OK, Json(Moves { moves }))
}

pub async fn get_piece_moves(
    Json(payload): Json<PieceMovesRequest>
) -> (StatusCode, Json<Moves>) {
    let board = fen::new_board(&payload.fen);
    if board.is_none() {
        return (StatusCode::BAD_REQUEST, Json(Moves { moves: Vec::new() }))
    }
    let board = board.unwrap();
    let moves = board.calculate_move_for_piece(payload.coordinates);
    (StatusCode::BAD_REQUEST, Json(Moves { moves: Vec::new() }))
}

pub async fn get_best_move(
    Extension(state): Extension<SharedState>,
    Json(payload): Json<BestMoveRequest>,
) -> Result<Json<PossibleMove>, StatusCode> {
    let board = fen::new_board(&payload.fen);
    let thread = thread::Builder::new()
        .name("negamax".to_string())
        .stack_size(32 * 1024 * 1024)
        .spawn(move || {
            state.write().unwrap().engine.get_best_move(&board.unwrap(), 3)
        }).unwrap();
    let best_move = thread.join().unwrap();
    Ok(Json(best_move))
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
    coordinates: Vector
}

#[derive(Serialize)]
pub struct Moves {
    moves: Vec<PossibleMove>
}