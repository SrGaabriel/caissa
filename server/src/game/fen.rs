use crate::board::{BitBoard, BitPosition, Pieces, Teams};
use crate::board::board::ChessBoard;

pub fn new_board(fen: &str) -> Option<ChessBoard> {
    let parts: Vec<&str> = fen.split_whitespace().collect();
    if parts.len() != 6 {
        return None;
    }

    let mut bit_position = BitPosition::new();

    let mut rank = 7;
    let mut file = 0;

    for c in parts[0].chars() {
        match c {
            'p' | 'r' | 'n' | 'b' | 'q' | 'k' | 'P' | 'R' | 'N' | 'B' | 'Q' | 'K' => {
                let piece = match c.to_ascii_lowercase() {
                    'p' => Pieces::PAWN,
                    'r' => Pieces::ROOK,
                    'n' => Pieces::KNIGHT,
                    'b' => Pieces::BISHOP,
                    'q' => Pieces::QUEEN,
                    'k' => Pieces::KING,
                    _ => unreachable!(),
                };
                let team = if c.is_ascii_uppercase() { Teams::WHITE } else { Teams::BLACK };
                let bitboard = bit_position.get_pieces(team, piece) | BitBoard(1 << (rank * 8 + file));
                bit_position.set_bitboard(team, piece, bitboard);
                file += 1;
            }
            '/' => {
                rank -= 1;
                file = 0;
            }
            digit if digit.is_ascii_digit() => {
                file += digit.to_digit(10).unwrap() as usize;
            }
            _ => return None,
        }
    }

    Some(ChessBoard::new(bit_position))
}