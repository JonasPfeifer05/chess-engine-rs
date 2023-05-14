use crate::csp::command::{ClientCommand, ServerCommand};
use crate::game_board::board::{Board, BoardBuilder, BoardMemento};
use crate::game_board::position::Position;
use crate::game_state::game_history::GameHistory;
use crate::game_state::game_state::{GameState, State};
use crate::movement::evaluator::MoveEvaluator;

#[derive(Debug)]
pub struct Application {
    board: Board,
    history: GameHistory,
    pub state: GameState,
}

impl Default for Application {
    fn default() -> Self {
        Application {
            board: Board::default(),
            history: Default::default(),
            state: GameState::new(),
        }
    }
}

impl Application {
    pub fn load_fen(&mut self, fen: &str) -> Result<(),String> {
        self.board = BoardBuilder::from_memento(&BoardMemento::new(fen.to_string()))?;

        Ok(())
    }

    pub fn fen(&self) -> String {
        BoardMemento::from_board(&self.board).fen_string().to_string()
    }

    pub fn movee(&mut self, from: &Position, to: &Position) -> Result<(), String> {
        let figure = if let Some(piece) = self.board.get_piece(from) { piece }
        else { return Err("No piece at that position!".to_string()) };

        if let State::Turn(color) = self.state.state() {
            if &color != figure.color() { return Err(format!("It not {color:?}s turn!")) }
        } else { return Err("Game is hasnt started or is already over".to_string())}

        let mut valid_moves = vec![];

        for moveset in figure.movable().get_move_sets() {
            let relatives = moveset.get_relative_moves(figure.color());
            println!("{:?}", relatives);
            let mut tmp_valid = MoveEvaluator::validate_moves(&relatives, from, &self.board).iter().map(|x| x.relative_move).collect();
            valid_moves.append(&mut tmp_valid);
        }

        let diff = (to.horizontal as u8 as i8 - from.horizontal as u8 as i8, to.vertical as u8 as i8 - from.vertical as u8 as i8);

        if !valid_moves.contains(&diff) { return Err("Invalid move passed!".to_string()) }

        Ok(())
    }
}