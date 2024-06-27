use crate::board::{BitBoard, BitPosition, GamePiece, MailBox, Pieces, Teams};
use crate::board::board::ChessBoard;
use crate::board::state::{CastlingRights, CastlingSides, ChessState};
use crate::game::square::square_to_vector;

pub fn new_board(fen: &str) -> Option<ChessBoard> {
    let parts: Vec<&str> = fen.split_whitespace().collect();
    if parts.len() != 6 {
        return None;
    }
    let mut mail_box = MailBox::new();
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
                mail_box.set_piece_at(rank * 16 + file, Some(GamePiece::from(piece, team)));
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
    let team_to_play = match parts[1] {
        "w" => Teams::WHITE,
        "b" => Teams::BLACK,
        _ => return None,
    };

    let castling_rights_part = parts[2];
    let mut castling_rights = CastlingRights::none();
    for c in castling_rights_part.chars() {
        match c {
            'K' => castling_rights.allow(Teams::WHITE, CastlingSides::KINGSIDE),
            'Q' => castling_rights.allow(Teams::WHITE, CastlingSides::QUEENSIDE),
            'k' => castling_rights.allow(Teams::BLACK, CastlingSides::KINGSIDE),
            'q' => castling_rights.allow(Teams::BLACK, CastlingSides::QUEENSIDE),
            '-' => break,
            _ => return None,
        }
    }
    let en_passant_square = match parts[3] {
        "-" => None,
        square => Some(square_to_vector(square).bit_position_index()),
    };
    let state = ChessState {
        castling_rights,
        en_passant_square,
        team_to_play
    };

    Some(ChessBoard::new(bit_position, mail_box, state))
}