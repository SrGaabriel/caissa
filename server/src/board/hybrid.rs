use crate::board::{BitBoard, BitPosition, MailBox, Piece, Pieces, Team, Teams};
use crate::game::Vector;

pub struct HybridChessBoard {
    pub mail_box: MailBox,
    pub bit_position: BitPosition,
}

impl HybridChessBoard {
    pub fn new() -> Self {
        let mail_box = MailBox::new();
        let bit_position = BitPosition::new();
        Self { mail_box, bit_position }
    }

    pub fn get_piece_at(&self, vector: Vector) -> Option<usize> {
        let index = vector.mail_box_index();
        self.mail_box.get_piece_at(index)
    }

    pub fn get_piece_location(&self, team: Team, piece: Piece) -> Option<Vector> {
        None
    }

    pub fn from_fen(fen: &str) -> Option<Self> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() != 6 {
            return None;
        }

        let mut mail_box = MailBox::new();
        let mut bit_position = BitPosition::new();

        let mut rank = 7;
        let mut file = 0;

        for c in parts[0].chars() {
            let index = rank * 16 + file;

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

                    mail_box.set_piece_at(index, piece);
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

        Some(Self { mail_box, bit_position })
    }
}