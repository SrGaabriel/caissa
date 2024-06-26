use crate::board::{BitBoard, BitPosition, get_opposite_team, Piece, Pieces, Team};
use crate::math::kings::mask_king_moves;
use crate::math::knights::mask_all_knight_moves;
use crate::math::pawns::mask_all_pawn_moves;
use crate::math::sliding::{properly_mask_all_bishop_moves, properly_mask_all_queen_moves, properly_mask_all_rook_moves};
use crate::print;

pub struct ChessBoard {
    pub bits: BitPosition
}

impl ChessBoard {
    pub fn new(bits: BitPosition) -> ChessBoard {
        ChessBoard {
            bits
        }
    }


    pub fn calculate_all_team_moves(&self, team: Team) -> BitBoard {
        let mut moves = BitBoard(0);
        let opponent_pieces = self.bits.get_team_pieces(get_opposite_team(team));
        let team_pieces = self.bits.get_team_pieces(team);
        let empty_squares = !(team_pieces | opponent_pieces);
        let occupied_squares = !empty_squares;

        let knight_moves = mask_all_knight_moves(&self.bits.get_pieces(team, Pieces::KNIGHT));
        moves.0 |= knight_moves.0;

        let bishop_moves = properly_mask_all_bishop_moves(&self.bits.get_pieces(team, Pieces::BISHOP), &occupied_squares);
        moves.0 |= bishop_moves.0;

        let rook_moves = properly_mask_all_rook_moves(&self.bits.get_pieces(team, Pieces::ROOK), &occupied_squares);
        moves.0 |= rook_moves.0;

        let pawn_moves = mask_all_pawn_moves(&self.bits.get_pieces(team, Pieces::PAWN), &empty_squares, &opponent_pieces, team);
        moves.0 |= pawn_moves.0;

        let queen_moves = properly_mask_all_queen_moves(&self.bits.get_pieces(team, Pieces::QUEEN), &occupied_squares);
        moves.0 |= queen_moves.0;

        let king_moves = mask_king_moves(self.bits.get_pieces(team, Pieces::KING).0);
        moves.0 |= king_moves.0;

        moves & !team_pieces
    }
}