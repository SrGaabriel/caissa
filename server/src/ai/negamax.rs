use crate::ai::ChessAI;
use crate::game::engine::BoardLogic;
use crate::game::{Move, PieceType};
use std::collections::HashMap;
use std::cmp;

pub struct Negamax {
    transposition_table: HashMap<u64, (i32, i32)>, // (depth, eval)
}

impl Negamax {
    pub fn new() -> Negamax {
        Negamax {
            transposition_table: HashMap::new(),
        }
    }

    fn negamax(&mut self, board: &BoardLogic, depth: i32, mut alpha: i32, beta: i32, color: i32) -> i32 {
        if depth == 0 {
            return self.quiescence(board, alpha, beta, color);
        }

        let mut max_eval = i32::MIN;
        let moves = board.get_all_moves_for_team(board.state.team_to_play);
        let mut ordered_moves = self.order_moves(board, moves);

        for mov in ordered_moves {
            let mut new_board = board.clone();
            new_board.play_move(mov.from.x, mov.from.y, mov.to.x, mov.to.y, false, true);
            let eval = -self.negamax(&new_board, depth - 1, -beta, -alpha, -color);
            max_eval = cmp::max(max_eval, eval);
            alpha = cmp::max(alpha, eval);
            if alpha >= beta {
                break;
            }
        }

        max_eval
    }

    fn quiescence(&self, board: &BoardLogic, mut alpha: i32, beta: i32, color: i32) -> i32 {
        let stand_pat = color * board.evaluate_position(board.state.team_to_play);
        if stand_pat >= beta {
            return beta;
        }
        if alpha < stand_pat {
            alpha = stand_pat;
        }

        let captures = board.get_all_capture_moves_for_team(board.state.team_to_play);
        for mov in captures {
            let mut new_board = board.clone();
            new_board.play_move(mov.from.x, mov.from.y, mov.to.x, mov.to.y, false, true);
            let eval = -self.quiescence(&new_board, -beta, -alpha, -color);
            if eval >= beta {
                return beta;
            }
            if eval > alpha {
                alpha = eval;
            }
        }
        alpha
    }

    fn order_moves(&self, board: &BoardLogic, moves: Vec<Move>) -> Vec<Move> {
        let mut scored_moves: Vec<(i32, Move)> = moves.into_iter()
            .map(|m| (self.score_move(board, &m), m))
            .collect();
        scored_moves.sort_by(|a, b| b.0.cmp(&a.0));
        scored_moves.into_iter().map(|(_, m)| m).collect()
    }

    fn score_move(&self, board: &BoardLogic, mov: &Move) -> i32 {
        // Simple move ordering heuristic: captures and promotions
        if mov.capture {
            return 1000 + match board.get_piece_at(mov.to.x, mov.to.y) {
                None => 0,
                Some(piece) => match piece.piece_type {
                    PieceType::Pawn => 1,
                    PieceType::Knight => 3,
                    PieceType::Bishop => 3,
                    PieceType::Rook => 5,
                    PieceType::Queen => 9,
                    PieceType::King => 0,
                }
            };
        }
        if mov.promotion {
            return 900;
        }
        0
    }
}

impl ChessAI for Negamax {
    fn get_best_move(&mut self, board: BoardLogic, depth: i32) -> Option<Move> {
        let mut best_move: Option<Move> = None;
        let mut max_eval = i32::MIN;

        let moves = board.get_all_moves_for_team(board.state.team_to_play);
        let ordered_moves = self.order_moves(&board, moves);

        for mov in ordered_moves {
            let mut new_board = board.clone();
            new_board.play_move(mov.from.x, mov.from.y, mov.to.x, mov.to.y, false, true);
            let eval = -self.negamax(&new_board, depth - 1, i32::MIN, i32::MAX, -1);
            if eval > max_eval {
                max_eval = eval;
                best_move = Some(mov);
            }
        }
        best_move
    }
}
