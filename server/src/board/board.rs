use crate::board::{BitBoard, BitPosition, CompletedMove, GamePiece, get_opposite_team, MailBox, Piece, Pieces, PossibleMove, PossibleMoves, Team, Teams};
use crate::board::state::{CastlingSides, ChessState};
use crate::engine::get_piece_value;
use crate::game::{Move, Vector};
use crate::math::{individually_mask_piece_moves, iterate_bits, map_bits};
use crate::math::kings::{calculate_king_castling_moves, mask_king_moves};
use crate::math::knights::mask_all_knight_moves;
use crate::math::pawns::mask_all_pawn_moves;
use crate::math::sliding::{properly_mask_all_bishop_moves, properly_mask_all_queen_moves, properly_mask_all_rook_moves};
use crate::print;

#[derive(Debug, Clone, Hash)]
pub struct ChessBoard {
    pub bits: BitPosition,
    pub mailbox: MailBox,
    pub state: ChessState,
    experimental: ExperimentalData
}

#[derive(Debug, Clone, Hash)]
struct ExperimentalData {
    last_move: Option<CompletedMove>
}

impl ChessBoard {
    pub fn new(bits: BitPosition, mailbox: MailBox, state: ChessState) -> ChessBoard {
        ChessBoard {
            bits,
            mailbox,
            state,
            experimental: ExperimentalData::new()
        }
    }

    pub fn play_move(
        &mut self,
        from: u8,
        to: u8,
        update_state: bool
    ) -> Option<CompletedMove> {
        let from_y = from / 8;
        let mailbox_index  = (from_y * 16 + from % 8) as usize;
        let piece = self.mailbox.get_piece_at(mailbox_index);
        if piece.is_none() {
            return None;
        }
        let piece = piece.unwrap();
        let mut mv = CompletedMove::clean(from,to);
        if update_state {
            match piece {
                GamePiece::Pawn(_team) => {
                    if (to as i8 - from as i8).abs() == 16 {
                        self.state.en_passant_square = Some(to as usize);
                    } else if from % 8 != to % 8 {
                        let x = to % 8;
                        let y = to / 8;
                        let target_index = y * 16 + x;
                        let target = self.mailbox.get_piece_at(target_index as usize);
                        if target.is_none() {
                            self.remove_piece(Vector::new(x, from_y));
                            mv.set_en_passant();
                        }
                    }
                }
                GamePiece::King(team) => {
                    if (to as i8 - from as i8).abs() == 2 {
                        let rook_from = if to == 2 { 0 } else { 7 };
                        let rook_to = if to == 2 { 3 } else { 5 };
                        let offset = team * 56;
                        mv.set_castling();
                        self.move_piece((rook_from + offset) as u8, (rook_to + offset) as u8);
                    } else {
                        self.state.castling_rights.disallow_all(team);
                    }
                }
                GamePiece::Rook(team) => {
                    match from % 8 {
                        0 => self.state.castling_rights.disallow(team, CastlingSides::QUEENSIDE),
                        7 => self.state.castling_rights.disallow(team, CastlingSides::KINGSIDE),
                        _ => {}
                    }
                }
                _ => {}

            };
            let target_x = to % 8;
            let target_y = to / 8;
            let target_mailbox_index = (target_y * 16 + target_x) as usize;
            let target = self.mailbox.get_piece_at(target_mailbox_index);
            if let Some(target) = target {
                mv.set_capture(target.get_piece());
            }
            self.state.team_to_play = get_opposite_team(self.state.team_to_play);
            // self.experimental.set_last_move(mv.clone());
        }
        self.move_piece(from, to);
        Some(mv)
    }

    pub fn move_piece(
        &mut self,
        from: u8,
        to: u8
    ) {
        let from_y = from / 8;
        let from_x = from % 8;
        let from_mailbox_index  = (from_y * 16 + from_x) as usize;
        let to_y = to / 8;
        let to_x = to % 8;
        let to_mailbox_index = (to_y * 16 + to_x) as usize;
        let piece = self.mailbox.get_piece_at(from_mailbox_index);
        if piece.is_none() {
            return;
        }
        let target = self.mailbox.get_piece_at(to_mailbox_index);
        if let Some(target) = target {
            let target_team = target.get_team();
            let target_piece = target.get_piece();
            self.bits.move_and(target_team, target_piece, !BitBoard(1 << to as usize));
        }
        let piece = piece.unwrap();
        self.mailbox.set_piece_at(from_mailbox_index, None);
        self.mailbox.set_piece_at(to_mailbox_index, Some(piece));
        let team = piece.get_team();
        let piece = piece.get_piece();
        self.bits.move_and(team, piece, !BitBoard(1 << from as usize));
        self.bits.move_or(team, piece, BitBoard(1 << to as usize));
    }

    pub fn is_in_check(
        &self,
        team: Team
    ) -> bool {
        let king = self.bits.get_pieces(team, Pieces::KING);
        let opponent_threats = self.attacks(get_opposite_team(team));
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
            self.bits.move_and(team, piece, !BitBoard(1 << location.bit_position_index()));
            self.mailbox.set_piece_at(mailbox_index, None);
        }
    }

    pub fn generate_moves(&self, team: Team) -> Vec<CompletedMove> {
        let hypothetical_board = &mut self.clone();
        let opponent_threats = None;
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

                let mut attacks =
                    individually_mask_piece_moves(piece_bit, piece, team, &empty_squares, &occupied_squares, &opponent_pieces, &self.state.en_passant_square, opponent_threats).0
                        & !team_pieces.0;
                if attacks == 0 { continue; }
                iterate_bits(attacks, |target| {
                    let hypothetical_move = hypothetical_board.play_move(piece_bit.trailing_zeros() as u8, target.trailing_zeros() as u8, true);
                    if let Some(hypothetical_move) = hypothetical_move {
                        if !hypothetical_board.is_in_check(team) {
                            moves.push(hypothetical_move.clone());
                        }
                        hypothetical_board.undo_move(hypothetical_move);
                    }
                });
            }
        }
        moves
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

    pub fn attacks(&self, team: Team) -> BitBoard {
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

    pub fn undo_move(
        &mut self,
        completed_move: CompletedMove
    ) {
        let origin_x = completed_move.origin % 8;
        let origin_y = completed_move.origin / 8;
        let target_x = completed_move.target % 8;
        let target_y = completed_move.target / 8;
        let from_mailbox_index = (origin_y * 16 + origin_x) as usize;
        let to_mailbox_index = (target_y * 16 + target_x) as usize;
        let piece = self.mailbox.get_piece_at(to_mailbox_index);
        if piece.is_none() {
            return;
        }
        let piece = piece.unwrap();
        let team = piece.get_team();
        let piece = piece.get_piece();
        self.mailbox.set_piece_at(from_mailbox_index, Some(GamePiece::from(piece, team)));
        self.mailbox.set_piece_at(to_mailbox_index, None);
        self.bits.move_and(team, piece, !BitBoard(1 << completed_move.target as usize));
        self.bits.move_or(team, piece, BitBoard(1 << completed_move.origin as usize));
        if completed_move.is_en_passant() {
            let target_y = if team == Teams::WHITE { 3 } else { 4 };
            let target_mailbox_index = (target_y * 16 + target_x) as usize;
            let target_piece = GamePiece::Pawn(get_opposite_team(team));
            self.mailbox.set_piece_at(target_mailbox_index, Some(target_piece));
            self.bits.move_or(get_opposite_team(team), Pieces::PAWN, BitBoard(1 << (target_y * 8 + target_x)));
        }
        if completed_move.is_castling() {
            let rook_from = if completed_move.target == 2 { 0 } else { 7 };
            let rook_to = if completed_move.target == 2 { 3 } else { 5 };
            let offset = team * 56;
            self.move_piece((rook_to + offset) as u8, (rook_from + offset) as u8);
        }
        if completed_move.is_capture() {
            let capture: Piece = completed_move.get_capture() as usize;
            let target_y = completed_move.target / 8;
            let target_x = completed_move.target % 8;
            let target_mailbox_index = (target_y * 16 + target_x) as usize;
            self.mailbox.set_piece_at(target_mailbox_index, Some(GamePiece::from(capture, get_opposite_team(team))));
            self.bits.move_or(get_opposite_team(team), capture, BitBoard(1 << completed_move.target as usize));
        }
        self.state.team_to_play = get_opposite_team(self.state.team_to_play);
    }
}


impl ExperimentalData {
    fn new() -> Self {
        Self {
            last_move: None
        }
    }

    pub fn set_last_move(&mut self, last_move: CompletedMove) {
        self.last_move = Some(last_move);
    }

    pub fn get_last_move(&self) -> &Option<CompletedMove> {
        &self.last_move
    }
}