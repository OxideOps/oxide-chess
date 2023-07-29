use crate::board_state::BoardState;
use crate::moves::Move;
use crate::turn::Turn;

#[derive(Clone, Default)]
pub struct History {
    pub turns: Vec<Turn>,
    current_turn: usize,
    fifty_move_count: u8,
    initial_state: BoardState,
}

impl History {
    pub fn with_state(initial_state: BoardState) -> Self {
        Self {
            initial_state,
            ..Default::default()
        }
    }

    pub fn get_board_state(&self, turn: usize) -> &BoardState {
        if turn == 0 {
            &self.initial_state
        } else {
            &self.turns[turn - 1].board_state
        }
    }

    pub fn add_info(&mut self, next_state: BoardState, mv: Move) {
        self.turns.push(Turn::new(next_state, mv));
        self.current_turn += 1;
        let previous_board_state = self.get_board_state(self.turns.len() - 1);
        let is_pawn = previous_board_state.get_piece(&mv.from).unwrap().is_pawn();
        let is_capture_move = previous_board_state.get_piece(&mv.to).is_some();
        if !is_pawn && !is_capture_move {
            self.fifty_move_count += 1;
        } else {
            self.fifty_move_count = 0;
        }
    }

    pub fn get_fifty_move_count(&self) -> u8 {
        self.fifty_move_count / 2
    }

    pub fn get_current_state(&self) -> &BoardState {
        self.get_board_state(self.current_turn)
    }

    pub fn clone_current_state(&self) -> BoardState {
        self.get_current_state().clone()
    }

    pub fn get_real_state(&self) -> &BoardState {
        &self.turns.last().unwrap().board_state
    }

    pub fn get_info_for_move(&self, turn: usize) -> &Turn {
        &self.turns[turn]
    }

    pub fn resume(&mut self) {
        self.current_turn = self.turns.len()
    }

    pub fn previous_move(&mut self) {
        if self.current_turn > 0 {
            self.current_turn -= 1
        }
    }

    pub fn next_move(&mut self) {
        if self.current_turn < self.turns.len() {
            self.current_turn += 1
        }
    }

    pub fn go_to_start(&mut self) {
        self.current_turn = 0
    }

    pub fn is_replaying(&self) -> bool {
        self.current_turn != self.turns.len()
    }

    pub fn current_round(&self) -> usize {
        self.current_turn / 2 + 1
    }
}
