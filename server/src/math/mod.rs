use crate::board::BitBoard;

pub mod pawns;
pub mod knights;
pub mod sliding;
pub mod kings;

const NOT_A_FILE: u64 = 0xfefefefefefefefe;
const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;

fn individually_mask_all_piece_moves(
    pieces: &BitBoard,
    blocks: &BitBoard,
    mask_moves_fn: fn(u64, BitBoard) -> BitBoard,
) -> BitBoard {
    let mut moves = BitBoard(0);
    let mut bitboard = pieces.0;

    while bitboard != 0 {
        let piece = 1 << bitboard.trailing_zeros();
        bitboard ^= piece;

        let attacks = mask_moves_fn(piece, *blocks);
        moves.0 |= attacks.0;
    }
    moves
}