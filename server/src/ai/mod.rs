use crate::game::engine::BoardLogic;
use crate::game::Move;

pub mod negamax;
mod hash;

pub trait ChessAI {
    fn get_best_move(&mut self, board: BoardLogic, depth: i32) -> Option<Move>;
}

