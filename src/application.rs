use crate::game_board::board::Board;

pub struct Application {
    board: Board,
}

impl Default for Application {
    fn default() -> Self {
        Application {
            board: Board::default(),
        }
    }
}

impl Application {
    pub fn process_event(&mut self, input: String) {

    }
}