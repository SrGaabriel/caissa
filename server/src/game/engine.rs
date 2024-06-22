use std::collections::HashSet;
use crate::game::moves::{calculate_moves_for_bishop, calculate_moves_for_king, calculate_moves_for_knight, calculate_moves_for_pawn, calculate_moves_for_queen, calculate_moves_for_rook, is_space_threatened};
use crate::game::{BoardState, CastlingRights, Coordinates, GameEnding, get_opposite_team, Move, Piece, PieceType, Side, Team, update_castling_state};
use crate::util::parse_square_name;

#[derive(Clone, Copy)]
pub struct BoardLogic {
    pub board: [[Option<Piece>; 8]; 8],
    pub state: BoardState,
}

impl BoardLogic {
    pub fn new(board: [[Option<Piece>; 8]; 8], state: BoardState) -> Self {
        Self { board, state }
    }

    pub fn get_piece_at(&self, x: i8, y: i8) -> Option<Piece> {
        self.board[(y - 1) as usize][(x - 1) as usize]
    }

    pub fn is_position_valid(&self, x: i8, y: i8) -> bool {
        x > 0 && x <= 8 && y > 0 && y <= 8
    }

    pub fn get_all_moves_for_team(&self, team: Team) -> Vec<Move> {
        let mut moves = Vec::new();
        for y in 1..=8 {
            for x in 1..=8 {
                if let Some(piece) = self.get_piece_at(x, y) {
                    if piece.team == team {
                        moves.extend(self.hypothetically_calculate_moves_for(x, y, piece, false));
                    }
                }
            }
        }
        moves
    }

    pub fn get_all_capture_moves_for_team(&self, team: Team) -> Vec<Move> {
        let mut moves = Vec::new();
        for y in 1..=8 {
            for x in 1..=8 {
                if let Some(piece) = self.get_piece_at(x, y) {
                    if piece.team == team {
                        moves.extend(self.hypothetically_calculate_moves_for(x, y, piece, true).into_iter().filter(|mov| mov.capture));
                    }
                }
            }
        }
        moves
    }

    pub fn get_threatened_spaces(&self, team: Team) -> HashSet<Coordinates> {
        let mut spaces: HashSet<Coordinates> = HashSet::new();
        for y in 1..=8 {
            for x in 1..=8 {
                if let Some(piece) = self.get_piece_at(x, y) {
                    if piece.team != team {
                        spaces.extend(self.optimistically_calculate_piece_moves(x, y, piece, true));
                    }
                }
            }
        }
        spaces
    }

    pub fn is_castling_available(&self, team: Team) -> bool {
        match team {
            Team::White => self.state.castling.white_king_side || self.state.castling.white_queen_side,
            Team::Black => self.state.castling.black_king_side || self.state.castling.black_queen_side,
        }
    }

    pub fn get_side(&self, x: i8) -> Side {
        if x < 5 { Side::QueenSide } else { Side::KingSide }
    }

    pub fn get_y_orientation(&self, team: Team) -> i8 {
        if team == Team::White { 1 } else { -1 }
    }

    pub fn get_board_y_start_for_team(&self, team: Team) -> i8 {
        if team == Team::White { 1 } else { 8 }
    }

    pub fn get_board_y_end_for_team(&self, team: Team) -> i8 {
        if team == Team::White { 8 } else { 1 }
    }

    pub fn is_team_in_check(&self, team: Team) -> bool {
        let threats = self.get_threatened_spaces(team);
        threats.iter().any(|&coord| {
            if let Some(piece) = self.get_piece_at(coord.x, coord.y) {
                piece.piece_type == PieceType::King && piece.team == team
            } else {
                false
            }
        })
    }

    pub fn get_game_ending(&self, check: Option<bool>) -> Option<GameEnding> {
        let opponent = get_opposite_team(self.state.team_to_play);
        let possible_moves = self.get_all_moves_for_team(opponent).len();
        if possible_moves != 0 { None } else if check.unwrap_or_else(|| self.is_team_in_check(opponent)) { Some(GameEnding::Checkmate) } else { Some(GameEnding::Stalemate) }
    }

    pub fn check_ending(&mut self) {
        self.state.ending = self.get_game_ending(None);
    }

    pub fn calculate_moves_for(&self, x: i8, y: i8) -> Vec<Move> {
        self.get_piece_at(x, y)
            .map(|piece| self.hypothetically_calculate_moves_for(x, y, piece, false))
            .unwrap_or_else(Vec::new)
    }

    pub fn is_move_possible(&self, current_x: i8, current_y: i8, future_x: i8, future_y: i8) -> bool {
        self.calculate_moves_for(current_x, current_y)
            .iter()
            .any(|mov| mov.to.x == future_x && mov.to.y == future_y)
    }

    pub fn play_move(
        &mut self,
        current_x: i8,
        current_y: i8,
        future_x: i8,
        future_y: i8,
        legally: bool,
        check_state: bool,
    ) -> Option<Move> {
        if legally && !self.is_move_possible(current_x, current_y, future_x, future_y) {
            return None;
        }
        let piece = self.get_piece_at(current_x, current_y)?;
        let mut move_ = Move {
            piece,
            from: Coordinates { x: current_x, y: current_y },
            to: Coordinates { x: future_x, y: future_y },
            check: false,
            capture: self.get_piece_at(future_x, future_y).is_some(),
            castle: false,
            ending: None,
            en_passant: false,
            promotion: false,
        };

        if piece.piece_type == PieceType::Pawn && future_y == self.get_board_y_end_for_team(piece.team) {
            move_.promotion = true;
        }

        if piece.piece_type == PieceType::Rook && current_y == self.get_board_y_start_for_team(piece.team) {
            if current_x == 1 || current_x == 8 {
                let side = self.get_side(current_x);
                update_castling_state(&mut self.state, piece.team, side, false);
            }
        }

        if piece.piece_type == PieceType::King {
            if (future_x as isize - current_x as isize).abs() == 2 {
                let rook_x = if future_x > current_x { 8 } else { 1 };
                let rook_future_x = if future_x > current_x { 6 } else { 4 };
                self.move_piece(rook_x, current_y, rook_future_x, current_y);
                move_.castle = true;
            }
            if piece.team == Team::White {
                self.state.castling.white_queen_side = false;
                self.state.castling.white_king_side = false;
            } else {
                self.state.castling.black_queen_side = false;
                self.state.castling.black_king_side = false;
            }
        }

        if piece.piece_type == PieceType::Pawn && (future_y as isize - current_y as isize).abs() == 2 {
            self.state.en_passant_target_square = Some(Coordinates {
                x: current_x,
                y: future_y - self.get_y_orientation(piece.team),
            });
            move_.en_passant = true;
        } else {
            self.state.en_passant_target_square = None;
        }

        if piece.piece_type == PieceType::Pawn && future_x != current_x {
            if self.get_piece_at(future_x, future_y).is_none() {
                if let Some(en_passant_target) = self.get_piece_at(future_x, current_y) {
                    if en_passant_target.piece_type == PieceType::Pawn && en_passant_target.team != piece.team {
                        self.board[(current_y - 1) as usize][(future_x - 1) as usize] = None;
                        move_.capture = true;
                        move_.en_passant = true;
                    }
                }
            }
        }

        self.board[(current_y - 1) as usize][(current_x - 1) as usize] = None;
        self.board[(future_y - 1) as usize][(future_x - 1) as usize] = Some(piece);

        let check = check_state && self.is_team_in_check(get_opposite_team(piece.team));
        move_.check = check;

        if check_state {
            move_.ending = self.get_game_ending(Some(check));
        }

        self.state.team_to_play = get_opposite_team(piece.team);
        Some(move_)
    }

    pub fn move_piece(&mut self, current_x: i8, current_y: i8, future_x: i8, future_y: i8) {
        if let Some(piece) = self.get_piece_at(current_x, current_y) {
            self.board[(current_y - 1) as usize][(current_x - 1) as usize] = None;
            self.board[(future_y - 1) as usize][(future_x - 1) as usize] = Some(piece);
        }
    }

    pub fn find_king(&self, team: Team) -> Option<Coordinates> {
        for y in 1..=8 {
            for x in 1..=8 {
                if let Some(piece) = self.get_piece_at(x, y) {
                    if piece.piece_type == PieceType::King && piece.team == team {
                        return Some(Coordinates { x, y });
                    }
                }
            }
        }
        None
    }

    pub fn hypothetically_calculate_moves_for(&self, x: i8, y: i8, piece: Piece, threats_only: bool) -> Vec<Move> {
        let mut legal_moves = vec![];
        let coordinates = self.optimistically_calculate_piece_moves(x, y, piece, threats_only);
        for coordinate in coordinates.iter() {
            let mut hypothetical = self.clone();
            let mov = hypothetical.play_move(x, y, coordinate.x, coordinate.y, false, false);
            let king = hypothetical.find_king(piece.team);
            if let Some(king) = king {
                if !is_space_threatened(&king, self, piece.team) {
                    legal_moves.push(mov.unwrap());
                }
            }
        }
        legal_moves
    }

    // here we'll count the material difference
    pub fn evaluate_position(&self, team: Team) -> i32 {
        let mut score = 0;
        for y in 1..=8 {
            for x in 1..=8 {
                if let Some(piece) = self.get_piece_at(x, y) {
                    let value = match piece.piece_type {
                        PieceType::Pawn => 1,
                        PieceType::Knight => 3,
                        PieceType::Bishop => 3,
                        PieceType::Rook => 5,
                        PieceType::Queen => 9,
                        PieceType::King => 1000,
                    };
                    score += if piece.team == team { value } else { -value };
                }
            }
        }
        score
    }

    pub fn optimistically_calculate_piece_moves(
        &self,
        x: i8,
        y: i8,
        piece: Piece,
        threats_only: bool
    ) -> Vec<Coordinates> {
        let mut moves = Vec::new();
        match piece.piece_type {
            PieceType::King => moves.extend(calculate_moves_for_king(self, piece.team, x, y)),
            PieceType::Queen => moves.extend(calculate_moves_for_queen(self, piece.team, x, y)),
            PieceType::Rook => moves.extend(calculate_moves_for_rook(self, piece.team, x, y)),
            PieceType::Bishop => moves.extend(calculate_moves_for_bishop(self, piece.team, x, y)),
            PieceType::Knight => moves.extend(calculate_moves_for_knight(self, piece.team, x, y)),
            PieceType::Pawn => moves.extend(calculate_moves_for_pawn(self, piece.team, x, y, threats_only)),
        }
        moves
    }

    pub fn from_fen(fen: &str) -> BoardLogic {
        let sectors: Vec<&str> = fen.split(' ').collect();
        let rows: Vec<&str> = sectors[0].split('/').collect();
        let mut board = [[None; 8]; 8];

        let piece_factory: fn(char) -> Option<Piece> = |ch| match ch {
            'P' => Some(Piece { piece_type: PieceType::Pawn, team: Team::White }),
            'N' => Some(Piece { piece_type: PieceType::Knight, team: Team::White }),
            'B' => Some(Piece { piece_type: PieceType::Bishop, team: Team::White }),
            'R' => Some(Piece { piece_type: PieceType::Rook, team: Team::White }),
            'Q' => Some(Piece { piece_type: PieceType::Queen, team: Team::White }),
            'K' => Some(Piece { piece_type: PieceType::King, team: Team::White }),
            'p' => Some(Piece { piece_type: PieceType::Pawn, team: Team::Black }),
            'n' => Some(Piece { piece_type: PieceType::Knight, team: Team::Black }),
            'b' => Some(Piece { piece_type: PieceType::Bishop, team: Team::Black }),
            'r' => Some(Piece { piece_type: PieceType::Rook, team: Team::Black }),
            'q' => Some(Piece { piece_type: PieceType::Queen, team: Team::Black }),
            'k' => Some(Piece { piece_type: PieceType::King, team: Team::Black }),
            _ => None,
        };

        for (i, &row) in rows.iter().rev().enumerate() {
            let mut col_index = 0;
            for ch in row.chars() {
                if let Some(piece) = piece_factory(ch) {
                    board[i][col_index] = Some(piece);
                    col_index += 1;
                } else if let Some(num) = ch.to_digit(10) {
                    col_index += num as usize;
                }
            }
        }

        let turn = if sectors[1] == "b" { Team::Black } else { Team::White };
        let castling_availability = sectors[2];
        let en_passant_target_square_notation = sectors[3];
        let en_passant_target_square = if en_passant_target_square_notation == "-" {
            None
        } else {
            Some(parse_square_name(en_passant_target_square_notation))
        };

        let castling = CastlingRights {
            white_king_side: castling_availability.contains('K'),
            white_queen_side: castling_availability.contains('Q'),
            black_king_side: castling_availability.contains('k'),
            black_queen_side: castling_availability.contains('q'),
        };

        let state = BoardState {
            team_to_play: turn,
            en_passant_target_square,
            ending: None,
            castling,
        };
        let mut engine = BoardLogic {
            board,
            state,
        };
        engine.check_ending();
        engine
    }
}