use crate::game_board::board::Board;
use crate::game_state::game_history::GameHistory;
use crate::game_state::game_state::GameState;

pub struct Application {
    board: Board,
    history: GameHistory,
    state: GameState,
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

}