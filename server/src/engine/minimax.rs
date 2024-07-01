use std::collections::HashMap;

use crate::board::{CompletedMove, get_opposite_team, PossibleMove};
use crate::board::board::ChessBoard;
use crate::engine::ChessEngine;
use crate::hash::ZobristHash;

#[derive(Clone)]
pub struct MinimaxEngine {
    zobrist: ZobristHash,
    min_transposition_table: HashMap<u64, i32>,
    max_transposition_table: HashMap<u64, i32>,
}

impl MinimaxEngine {
    fn minimax(&mut self, board: &mut ChessBoard, depth: u8, maximizing_player: bool) -> i32 {
        let hash = self.zobrist.hash(board);
        if maximizing_player {
            if let Some(score) = self.max_transposition_table.get(&hash) {
                return *score;
            }
        } else {
            if let Some(score) = self.min_transposition_table.get(&hash) {
                return *score;
            }
        }
        if depth == 0 {
            let score = board.evaluate(board.state.team_to_play);
            if maximizing_player {
                self.max_transposition_table.insert(hash, score);
            } else {
                self.min_transposition_table.insert(hash, score);
            }
            return score;
        }

        if maximizing_player {
            let mut max_eval = i32::MIN;
            for mv in board.generate_moves(board.state.team_to_play) {
                let mv = board.play_move(mv.origin, mv.target, true);
                if let Some(mov) = mv {
                    let eval = -self.minimax(board, depth - 1, false);
                    board.undo_move(mov);
                    max_eval = max_eval.max(eval);
                }
            }
            self.max_transposition_table.insert(hash, max_eval);
            max_eval
        } else {
            let mut min_eval = i32::MAX;
            for mv in board.generate_moves(get_opposite_team(board.state.team_to_play)) {
                let mv = board.play_move(mv.origin, mv.target, true);
                if let Some(mov) = mv {
                    let eval = -self.minimax(board, depth - 1, true);
                    board.undo_move(mov);
                    min_eval = min_eval.min(eval);
                }
            }
            self.min_transposition_table.insert(hash, min_eval);
            min_eval
        }
    }
}

impl ChessEngine for MinimaxEngine {
    fn new() -> Self {
        MinimaxEngine {
            zobrist: ZobristHash::new(),
            min_transposition_table: HashMap::new(),
            max_transposition_table: HashMap::new(),
        }
    }

    fn get_best_move(&mut self, board: &ChessBoard, depth: u8) -> CompletedMove {
        let mut board_clone = &mut board.clone();
        let mut best_move = None;
        let mut best_score = i32::MIN;

        for mv in board.generate_moves(board.state.team_to_play) {
            let mv = board_clone.play_move(mv.origin, mv.target, true);
            if let Some(mov) = mv {
                let score = -self.minimax(&mut board_clone, depth - 1, false);
                board_clone.undo_move(mov.clone());
                if score > best_score {
                    best_score = score;
                    best_move = Some(mov);
                }
            }
        }
        best_move.unwrap()
    }
}
