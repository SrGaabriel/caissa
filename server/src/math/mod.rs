use crate::board::{BitBoard, Piece, Pieces, Team};

pub mod pawns;
pub mod knights;
pub mod sliding;
pub mod kings;

const NOT_A_FILE: u64 = 0xfefefefefefefefe;
const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;

pub fn individually_mask_piece_moves(
    bit: u64,
    piece: Piece,
    team: Team,
    empty_spaces: &BitBoard,
    occupied_spaces: &BitBoard,
    opponent_pieces: &BitBoard,
    en_passant_square: &Option<usize>,
    opponent_threats: Option<&BitBoard>
) -> BitBoard {
    match piece {
        Pieces::PAWN => pawns::mask_pawn_moves(bit, empty_spaces, opponent_pieces, en_passant_square, team),
        Pieces::KNIGHT => knights::mask_knight_moves(bit),
        Pieces::BISHOP => sliding::properly_mask_bishop_moves(bit, occupied_spaces),
        Pieces::ROOK => sliding::properly_mask_rook_moves(bit, occupied_spaces),
        Pieces::QUEEN => sliding::properly_mask_queen_moves(bit, occupied_spaces),
        Pieces::KING => kings::mask_king_moves(bit, opponent_threats),
        _ => panic!("Invalid piece")
    }
}

pub fn iterate_bits<F>(mut bitboard: u64, mut f: F)
where
    F: FnMut(u64)
{
    while bitboard != 0 {
        let bit = 1 << bitboard.trailing_zeros();
        bitboard ^= bit;
        f(bit);
    }
}

pub fn map_bits<F, M>(mut bitboard: u64, mut f: F) -> Vec<M>
where
    F: FnMut(u64) -> M
{
    let mut result = vec![];
    while bitboard != 0 {
        let bit = 1 << bitboard.trailing_zeros();
        bitboard ^= bit;
        result.push(f(bit));
    }
    result
}