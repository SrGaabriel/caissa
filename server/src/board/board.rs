use crate::board::{BitBoard, BitPosition, GamePiece, get_opposite_team, MailBox, Pieces, PossibleMove, PossibleMoves, Team};
use crate::board::state::{CastlingSides, ChessState};
use crate::engine::get_piece_value;
use crate::game::Vector;
use crate::math::{individually_mask_piece_moves, map_bits};
use crate::math::kings::{calculate_king_castling_moves, mask_king_moves};
use crate::math::knights::mask_all_knight_moves;
use crate::math::pawns::mask_all_pawn_moves;
use crate::math::sliding::{properly_mask_all_bishop_moves, properly_mask_all_queen_moves, properly_mask_all_rook_moves};

#[derive(Debug, Clone, Copy, Hash)]
pub struct ChessBoard {
    pub bits: BitPosition,
    pub mailbox: MailBox,
    pub state: ChessState
}

impl ChessBoard {
    pub fn new(bits: BitPosition, mailbox: MailBox, state: ChessState) -> ChessBoard {
        ChessBoard {
            bits,
            mailbox,
            state
        }
    }

    pub fn play_move(
        &mut self,
        from: Vector,
        to: Vector,
        update_state: bool
    ) {
        let piece = self.mailbox.get_piece_at(from.mail_box_index());
        if piece.is_none() {
            return;
        }
        let piece = piece.unwrap();
        if update_state {
            match piece {
                GamePiece::Pawn(_team) => {
                    if (to.y - from.y).abs() == 2 {
                        self.state.en_passant_square = Some(to.bit_position_index());
                    } else if to.x != from.x {
                        let target_index = to.mail_box_index();
                        let target = self.mailbox.get_piece_at(target_index);
                        if target.is_none() {
                            self.remove_piece(Vector::new(to.x, from.y));
                        }
                    }
                }
                GamePiece::King(team) => {
                    if (from.x as i8 - to.x as i8).abs() == 2 {
                        let rook_from = match to.x {
                            2 => Vector::new(0, from.y),
                            6 => Vector::new(7, from.y),
                            _ => unreachable!()
                        };
                        let rook_to = match to.x {
                            2 => Vector::new(3, from.y),
                            6 => Vector::new(5, from.y),
                            _ => unreachable!()
                        };
                        self.move_piece(rook_from, rook_to);
                    } else {
                        self.state.castling_rights.disallow_all(team);
                    }
                }
                GamePiece::Rook(team) => {
                    match from.x {
                        0 => self.state.castling_rights.disallow(team, CastlingSides::QUEENSIDE),
                        7 => self.state.castling_rights.disallow(team, CastlingSides::KINGSIDE),
                        _ => {}
                    }
                }
                _ => {}
            };
            self.state.team_to_play = get_opposite_team(self.state.team_to_play);
        }
        self.move_piece(from, to);
    }

    pub fn move_piece(
        &mut self,
        from: Vector,
        to: Vector
    ) {
        let piece = self.mailbox.get_piece_at(from.mail_box_index());
        if piece.is_none() {
            return;
        }
        let target = self.mailbox.get_piece_at(to.mail_box_index());
        if let Some(target) = target {
            let target_team = target.get_team();
            let target_piece = target.get_piece();
            self.bits.set_bitboard(target_team, target_piece, self.bits.get_pieces(target_team, target_piece) & !BitBoard(1 << to.bit_position_index()));
        }
        let piece = piece.unwrap();
        self.mailbox.set_piece_at(from.mail_box_index(), None);
        self.mailbox.set_piece_at(to.mail_box_index(), Some(piece));
        let team = piece.get_team();
        let piece = piece.get_piece();
        self.bits.set_bitboard(team, piece, self.bits.get_pieces(team, piece) & !BitBoard(1 << from.bit_position_index()));
        self.bits.set_bitboard(team, piece, self.bits.get_pieces(team, piece) | BitBoard(1 << to.bit_position_index()));
    }

    pub fn is_in_check(
        &self,
        team: Team
    ) -> bool {
        let king = self.bits.get_pieces(team, Pieces::KING);
        let opponent_threats = self.threats(get_opposite_team(team));
        (opponent_threats.0 & king.0) != 0
    }

    pub fn remove_piece(
        &mut self,
        location: Vector
    ) {
        let mailbox_index = location.mail_box_index();
        if let Some(piece) = self.mailbox.get_piece_at(mailbox_index) {
            let team = piece.get_team();
            let piece = piece.get_piece();
            self.bits.set_bitboard(team, piece, self.bits.get_pieces(team, piece) & !BitBoard(1 << location.bit_position_index()));
            self.mailbox.set_piece_at(mailbox_index, None);
        }
    }

    pub fn generate_moves(&self, team: Team) -> Vec<PossibleMove> {
        let pseudo_moves = self.pseudo_move_generation(team, None);
        let mapped_moves: Vec<PossibleMove> = pseudo_moves.iter().flat_map(|mv| {
            map_bits(mv.attacks.0, |target| PossibleMove {
                origin: Vector::from_bit_position_index(mv.origin),
                target: Vector::from_bit_position_index(target.trailing_zeros() as usize)
            })
        }).filter(|mv| {
            let hypothetical_board: &mut ChessBoard = &mut (self.clone());
            hypothetical_board.play_move(mv.origin, mv.target, true);
            !hypothetical_board.is_in_check(team)
        }).collect();
        mapped_moves
    }

    pub fn calculate_move_for_piece(&self, location: Vector) -> Vec<Vector> {
        let piece = self.mailbox.get_piece_at(location.mail_box_index());
        if piece.is_none() {
            return vec![];
        }
        let piece = piece.unwrap();
        let team = piece.get_team();
        let opponent_pieces = self.bits.get_team_pieces(get_opposite_team(team));
        let team_pieces = self.bits.get_team_pieces(team);
        let empty_squares = !(team_pieces | opponent_pieces);
        let occupied_squares = !empty_squares;

        let bitboard = individually_mask_piece_moves(location.bit_position_index() as u64, piece.get_piece(), team, &empty_squares, &occupied_squares, &opponent_pieces, &self.state.en_passant_square, None);
        vec![]
    }

    pub fn pseudo_move_generation(&self, team: Team, opponent_threats: Option<&BitBoard>) -> Vec<PossibleMoves> {
        let mut moves = vec![];
        let opponent_pieces = self.bits.get_team_pieces(get_opposite_team(team));
        let team_pieces = self.bits.get_team_pieces(team);
        let empty_squares = !(team_pieces | opponent_pieces);
        let occupied_squares = !empty_squares;

        for piece in Pieces::iter() {
            let mut bitboard = self.bits.get_pieces(team, piece).0.clone();
            if bitboard == 0 { continue; }
            while bitboard != 0 {
                let piece_bit = 1 << bitboard.trailing_zeros();
                bitboard ^= piece_bit;

                let attacks = individually_mask_piece_moves(piece_bit, piece, team, &empty_squares, &occupied_squares, &opponent_pieces, &self.state.en_passant_square, opponent_threats);
                if attacks.0 == 0 { continue; }
                moves.push(
                    PossibleMoves {
                        origin: piece_bit.trailing_zeros() as usize,
                        attacks: attacks & !team_pieces
                    }
                )
            }
        }

        moves
    }

    pub fn threats(&self, team: Team) -> BitBoard {
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

        let pawn_moves = mask_all_pawn_moves(&self.bits.get_pieces(team, Pieces::PAWN), &empty_squares, &opponent_pieces, &self.state.en_passant_square, team);
        moves.0 |= pawn_moves.0;

        let queen_moves = properly_mask_all_queen_moves(&self.bits.get_pieces(team, Pieces::QUEEN), &occupied_squares);
        moves.0 |= queen_moves.0;

        let king_moves = mask_king_moves(self.bits.get_pieces(team, Pieces::KING).0, None);
        let king_castling_moves = calculate_king_castling_moves(&self.state.castling_rights, team, &occupied_squares);
        moves.0 |= king_moves.0;
        moves.0 |= king_castling_moves.0;

        moves & !team_pieces
    }

    // Difference of material
    pub fn evaluate(&self, reference: Team) -> i32 {
        let mut score = 0;
        for piece in Pieces::iter() {
            let team = reference;
            let team_pieces = self.bits.get_pieces(team, piece);
            let opponent_pieces = self.bits.get_pieces(get_opposite_team(team), piece);
            score += (team_pieces.count_ones() as i32 - opponent_pieces.count_ones() as i32) * get_piece_value(piece);
        }
        score
    }
}