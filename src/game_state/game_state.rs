use std::sync::RwLock;
use lazy_static::lazy_static;
use crate::game_board::piece::Color;

#[derive(Clone, Eq, PartialEq)]
pub enum State {
    Setup,
    Turn(Color),
    Over(Result),
}

#[derive(Clone, Eq, PartialEq)]
pub enum Result {
    Won(Color),
    Stalemate
}

lazy_static! {
    static ref GAME_STATE: RwLock<GameState> = RwLock::new(GameState::new());
}

pub struct GameState {
    pub current_state: State,
}

impl GameState {
    pub const fn new() -> Self {
        Self { current_state: State::Setup }
    }


    pub fn state() -> State {
       GAME_STATE.read().unwrap().current_state.clone()
    }
}