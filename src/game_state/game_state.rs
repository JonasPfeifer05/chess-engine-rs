use std::sync::RwLock;
use lazy_static::lazy_static;
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
}

impl GameState {
    pub const fn new() -> Self {
        Self { current_state: State::Setup }
    }

    pub fn state(&self) -> State {
       self.current_state.clone()
    }
}