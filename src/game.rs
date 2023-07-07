use crate::board::BoardState;
use crate::castling_rights::{CastlingRights, CastlingRightsKind};
use crate::displacement::Displacement;
use crate::moves::Move;
use crate::pieces::{Piece, Player, Position};

use std::collections::HashSet;

pub type ChessResult = Result<(), ChessError>;
#[derive(Debug)]
pub enum ChessError {
    OutOfBounds,
    NoPieceAtPosition,
    InvalidMove,
    OwnPieceInDestination,
    PlayerInCheck,
    Checkmate,
    Stalemate,
    InvalidPromotion,
    NotPlayersTurn,
    EmptyPieceMove,
}

#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub enum GameStatus {
    #[default]
    Ongoing,
    Stalemate,
    Check,
    Checkmate,
    Replay,
}

impl GameStatus {
    pub fn update(&mut self, status: GameStatus) {
        if *self != status {
            println!("GameStatus changing from {:?} to {:?}", *self, status)
        }
        *self = status;
    }
}

#[derive(Clone, Default)]
pub struct History {
    history: Vec<(BoardState, Move)>,
    current_turn: usize,
}

impl History {
    pub fn with_state(state: BoardState) -> Self {
        Self {
            history: vec![(state, Move::default())],
            ..Default::default()
        }
    }

    fn add_info(&mut self, next_state: BoardState, mv: Move) {
        self.history.push((next_state, mv));
        self.current_turn += 1
    }

    fn get_current_state(&self) -> &BoardState {
        &self.history[self.current_turn - 1].0
    }

    fn get_info_for_turn(&self, turn: usize) -> &(BoardState, Move) {
        &self.history[turn]
    }

    fn resume(&mut self) {
        self.current_turn = self.history.len()
    }

    fn previous_state(&mut self) {
        if self.current_turn > 1 {
            self.current_turn -= 1
        }
    }

    fn next_state(&mut self) {
        if self.current_turn < self.history.len() {
            self.current_turn += 1
        }
    }

    fn initial_state(&mut self) {
        self.current_turn = 1
    }

    fn is_ongoing(&mut self) -> bool {
        self.current_turn == self.history.len()
    }
}

#[derive(Clone)]
pub struct Game {
    valid_moves: HashSet<Move>,
    status: GameStatus,
    history: History,
}

impl Default for Game {
    fn default() -> Self {
        Game::with_history(History::with_state(BoardState::default()))
    }
}

impl Game {
    pub fn new() -> Self {
        Game::default()
    }

    pub fn with_history(history: History) -> Self {
        let mut game = Self {
            history,
            ..Default::default()
        };
        game.add_moves();
        game
    }

    pub fn with_state(state: BoardState) -> Self {
        Game::with_history(History::with_state(state))
    }

    pub fn get_piece(&self, position: &Position) -> Option<Piece> {
        self.history.get_current_state().get_piece(position)
    }

    fn get_current_state(&self) -> &BoardState {
        self.history.get_current_state()
    }

    fn get_current_player(&self) -> Player {
        self.get_current_state().player
    }

    fn is_piece_some(&self, at: &Position) -> ChessResult {
        self.get_current_state().is_piece_some(at)
    }

    pub fn has_piece(&self, position: &Position) -> bool {
        self.history.get_current_state().has_piece(position)
    }

    fn piece_can_snipe(&self, at: &Position) -> bool {
        self.get_piece(at).unwrap().can_snipe()
    }

    fn get_info_for_turn(&self, turn: usize) -> &(BoardState, Move) {
        self.history.get_info_for_turn(turn)
    }

    fn has_castling_right(&self, right: CastlingRightsKind) -> bool {
        self.get_current_state()
            .castling_rights
            .has_castling_right(right)
    }

    pub fn go_back_a_turn(&mut self) {
        self.status.update(GameStatus::Replay);
        self.history.previous_state()
    }

    pub fn go_forward_a_turn(&mut self) {
        if self.status == GameStatus::Replay {
            self.history.next_state();
            if self.history.is_ongoing() {
                self.status.update(GameStatus::Ongoing)
            }
        }
    }

    pub fn go_to_beginning(&mut self) {
        self.status.update(GameStatus::Replay);
        self.history.initial_state()
    }

    pub fn resume(&mut self) {
        self.status.update(GameStatus::Ongoing);
        self.history.resume()
    }

    fn add_info(&mut self, next_state: BoardState, mv: Move) {
        self.history.add_info(next_state, mv);
    }

    pub fn move_piece(&mut self, from: Position, to: Position) -> ChessResult {
        if self.status == GameStatus::Ongoing {
            if let Some(piece) = self.get_piece(&from) {
                let mv = Move::new(from, to);
                self.is_move_valid(&mv)?;

                let mut next_state = self.get_current_state().clone();
                next_state.move_piece(&mv);
                self.history.add_info(next_state, mv);
                self.update();

                println!("{} : {}", piece, mv);
            }
        }
        //do nothing for now if not in GameStatus::Ongoing
        Ok(())
    }

    fn update(&mut self) {
        self.add_moves();
        self.remove_self_checks();
        self.update_status()
    }

    fn update_status(&mut self) {
        if self.status == GameStatus::Replay {
            return;
        }

        let king_under_attack = self.is_king_under_attack();
        let valid_moves_empty = self.valid_moves.is_empty();
        let attacking_king = self.is_attacking_king();

        if !king_under_attack && valid_moves_empty {
            self.status.update(GameStatus::Stalemate);
        } else if king_under_attack && valid_moves_empty {
            self.status.update(GameStatus::Checkmate);
        } else if attacking_king {
            self.status.update(GameStatus::Check);
        }
    }

    fn is_attacking_king(&self) -> bool {
        self.valid_moves
            .iter()
            .any(|mv| self.get_piece(&mv.to) == Some(Piece::King(!self.get_current_player())))
    }

    fn is_king_under_attack(&self) -> bool {
        let mut enemy_board = self.get_current_state().clone();
        enemy_board.player = !enemy_board.player;
        Game::with_state(enemy_board).is_attacking_king()
    }

    fn remove_self_checks(&mut self) {
        let current_board = self.get_current_state().clone();
        self.valid_moves.retain(|mv| {
            let mut future_board = current_board.clone();
            future_board.move_piece(mv);
            !Game::with_state(future_board).is_attacking_king()
        })
    }

    fn is_move_valid(&self, mv: &Move) -> ChessResult {
        BoardState::is_in_bounds(&mv.from)?;
        BoardState::is_in_bounds(&mv.to)?;
        self.is_piece_some(&mv.from)?;

        if self.valid_moves.contains(mv) {
            Ok(())
        } else {
            Err(ChessError::InvalidMove)
        }
    }

    fn add_pawn_advance_moves(&mut self, from: Position) {
        let v = Displacement::get_pawn_advance_vector(self.get_current_player());
        let mut to = from + v;
        if BoardState::is_in_bounds(&to).is_ok() && self.get_piece(&to).is_none() {
            self.valid_moves.insert(Move { from, to });
            to += v;
            if self.get_piece(&to).is_none() && self.can_double_move(&from) {
                self.valid_moves.insert(Move { from, to });
            }
        }
    }

    fn can_double_move(&self, from: &Position) -> bool {
        if let Piece::Pawn(player) = self.get_piece(from).unwrap() {
            return match player {
                Player::White => from.y == 1,
                Player::Black => from.y == 6,
            };
        }
        false
    }

    fn add_pawn_capture_moves(&mut self, from: Position) {
        for &v in Displacement::get_pawn_capture_vectors(self.history.get_current_state().player) {
            let to = from + v;
            if BoardState::is_in_bounds(&to).is_ok() {
                if let Some(piece) = self.get_piece(&to) {
                    if piece.get_player() != self.get_current_player() {
                        self.valid_moves.insert(Move::new(from, to));
                    }
                }
                if Some(to) == self.get_current_state().en_passant_position {
                    self.valid_moves.insert(Move::new(from, to));
                }
            }
        }
    }

    fn add_moves_for_piece(&mut self, from: Position) {
        if let Some(piece) = self.get_piece(&from) {
            if piece.get_player() == self.get_current_player() {
                if piece.is_pawn() {
                    self.add_pawn_advance_moves(from);
                    self.add_pawn_capture_moves(from);
                } else {
                    for &v in piece.get_vectors() {
                        let mut to = from + v;
                        while BoardState::is_in_bounds(&to).is_ok() {
                            if let Some(piece) = self.get_piece(&to) {
                                if piece.get_player() != self.get_current_player() {
                                    self.valid_moves.insert(Move { from, to });
                                }
                                break;
                            }
                            self.valid_moves.insert(Move { from, to });
                            if !self.piece_can_snipe(&from) {
                                break;
                            }
                            to += v;
                        }
                    }
                }
            }
        }
    }

    fn add_moves(&mut self) {
        self.valid_moves.clear();
        for y in 0..8 {
            for x in 0..8 {
                self.add_moves_for_piece(Position::new(x, y))
            }
        }
        self.add_castling_moves()
    }

    fn add_castling_moves(&mut self) {
        let (king_square, kingside, queenside) =
            CastlingRights::get_castling_info(self.get_current_player());

        if self.has_castling_right(kingside)
            && !(1..=2).any(|i| self.has_piece(&(king_square + Displacement::RIGHT * i)))
        {
            self.valid_moves.insert(Move {
                from: king_square,
                to: king_square + Displacement::RIGHT * 2,
            });
        }

        if self.has_castling_right(queenside)
            && !(1..=3).any(|i| self.has_piece(&(king_square + Displacement::LEFT * i)))
        {
            self.valid_moves.insert(Move {
                from: king_square,
                to: king_square + Displacement::LEFT * 2,
            });
        }
    }
}
