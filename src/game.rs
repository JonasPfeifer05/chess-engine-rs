use crate::game_board::board::{Board, BoardBuilder, BoardMemento};
use crate::game_board::piece::Color;
use crate::game_board::position::Position;
use crate::game_state::game_history::GameHistory;
use crate::game_state::game_state::{GameState, State};
use crate::movement::evaluator::MoveEvaluator;

#[derive(Debug)]
pub struct Game {
    board: Board,
    history: GameHistory,
    pub state: GameState,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            board: Board::default(),
            history: Default::default(),
            state: GameState::new(),
        }
    }
}

impl Game {
    pub fn load_fen(&mut self, fen: &str) -> Result<(),String> {
        self.board = BoardBuilder::from_memento(&BoardMemento::new(fen.to_string()))?;

        Ok(())
    }

    pub fn fen(&self) -> String {
        BoardMemento::from_board(&self.board).fen_string().to_string()
    }

    pub fn movee(&mut self, from: &Position, to: &Position, color: &Color) -> Result<(), String> {
        let figure = if let Some(piece) = self.board.get_piece(from) { piece }
        else { return Err("No piece at that position!".to_string()) };

        if figure.color() != color {
            return Err("Cant move figure of other color".to_string());
        }

        if let State::Turn(current_color) = self.state.state() {
            if &current_color != figure.color() { return Err(format!("It not {color:?}s turn!")) }
        } else { return Err("Game is hasnt started or is already over".to_string())}

        let valid_moves = MoveEvaluator::validate_moves(from, &self.board);

        if !valid_moves.contains(to) {
            return Err("Invalid move passed!".to_string())
        }

        self.board.move_piece(from, to);
        self.state.switch_color();

        Ok(())
    }
}