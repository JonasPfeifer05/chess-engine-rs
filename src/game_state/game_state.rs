use crate::game_board::piece::Color;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum State {
    Setup,
    Turn(Color),
    Over(Result),
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Result {
    Won(Color),
    Stalemate
}

#[derive(Debug)]
pub struct GameState {
    pub current_state: State,
    pub next_player: Color,
}

impl GameState {
    pub const fn new() -> Self {
        Self { current_state: State::Setup, next_player: Color::White }
    }

    pub fn stop_game(&mut self) {
        self.current_state = State::Setup;
    }

    pub fn start_game(&mut self) {
        self.current_state = State::Turn(self.next_player.clone());
    }

    pub fn switch_color(&mut self) {
        match &self.current_state {
            State::Turn(color) => {
                if color == &Color::White {
                    self.current_state = State::Turn(Color::Black);
                    self.next_player = Color::Black;
                } else {
                    self.current_state = State::Turn(Color::White);
                    self.next_player = Color::White;
                }
            }
            _ => {}
        }
    }

    pub fn state(&self) -> State {
       self.current_state.clone()
    }
}