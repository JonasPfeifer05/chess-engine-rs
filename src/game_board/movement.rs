use dyn_clone::DynClone;
use crate::game_board::piece::Color;
use crate::game_state::game_state::{GameState, State};

pub struct Move {
    relative_move: (i8, i8),
    can_kill: bool,
    only_with_kill: bool,
}

impl Move {
    pub fn new_detailed(relative_move: (i8, i8), can_kill: bool, only_with_kill: bool) -> Self {
        Self { relative_move, can_kill, only_with_kill }
    }

    pub fn new(relative_move: (i8,i8)) -> Self {
        Self {
            relative_move,
            can_kill: true,
            only_with_kill: false,
        }
    }
}

/// A descriptor for a movement pattern like horizontal LeftEight or UpOne and so on
pub trait MoveSet: DynClone {
    fn get_relative_moves(&self) -> Vec<Move>;
}
dyn_clone::clone_trait_object!(MoveSet);

#[derive(Clone)]
pub struct HorizontalOne;

#[derive(Clone)]
pub struct VerticalOne;

#[derive(Clone)]
pub struct HorizontalEight;

#[derive(Clone)]
pub struct VerticalEight;

#[derive(Clone)]
pub struct DiagonalOne;

#[derive(Clone)]
pub struct DiagonalEight;

#[derive(Clone)]
pub struct KnightMove;

#[derive(Clone)]
pub struct PawnOne;

#[derive(Clone)]
pub struct PawnTwo;

#[derive(Clone)]
pub struct PawnKill;

impl MoveSet for PawnKill {
    fn get_relative_moves(&self) -> Vec<Move> {
        match GameState::state() {
            State::Setup => { vec![] }
            State::Over(_) => { vec![] }
            State::Turn(color) => {
                if color == Color::White {
                    vec![Move::new_detailed((1,1), true, true),Move::new_detailed((-1,1), true, true)]
                } else {
                    vec![Move::new_detailed((1,-1), true, true),Move::new_detailed((-1,-1), true, true)]
                }
            }
        }
    }
}

impl MoveSet for PawnOne {
    fn get_relative_moves(&self) -> Vec<Move> {
        match GameState::state() {
            State::Setup => { vec![] }
            State::Over(_) => { vec![] }
            State::Turn(color) => {
                if color == Color::White {
                    vec![Move::new_detailed((0,1), false, false)]
                } else {
                    vec![Move::new_detailed((0,-1), false, false)]
                }
            }
        }
    }
}

impl MoveSet for PawnTwo {
    fn get_relative_moves(&self) -> Vec<Move> {
        match GameState::state() {
            State::Setup => { vec![] }
            State::Over(_) => { vec![] }
            State::Turn(color) => {
                if color == Color::White {
                    vec![Move::new_detailed((0,2), false, false)]
                } else {
                    vec![Move::new_detailed((0,-2), false, false)]
                }
            }
        }
    }
}

impl MoveSet for HorizontalOne {
    fn get_relative_moves(&self) -> Vec<Move> {
        vec![Move::new((-1, 0)), Move::new((1, 0))]
    }
}

impl MoveSet for HorizontalEight {
    fn get_relative_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        for i in 0..8 {
            moves.push(Move::new((i, 0)));
            moves.push(Move::new((-i, 0)));
        }
        moves
    }
}

impl MoveSet for VerticalOne {
    fn get_relative_moves(&self) -> Vec<Move> {
        vec![Move::new((0, -1)), Move::new((0, 1))]
    }
}

impl MoveSet for VerticalEight {
    fn get_relative_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        for i in 0..8 {
            moves.push(Move::new((0, i)));
            moves.push(Move::new((0, -i)));
        }
        moves
    }
}

impl MoveSet for DiagonalOne {
    fn get_relative_moves(&self) -> Vec<Move> {
        vec![Move::new((1, 1)), Move::new((1, -1)), Move::new((-1, 1)), Move::new((-1, -1))]
    }
}

impl MoveSet for DiagonalEight {
    fn get_relative_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        for i in 0..8 {
            moves.push(Move::new((i, i)));
            moves.push(Move::new((i, -i)));
            moves.push(Move::new((-i, i)));
            moves.push(Move::new((-i, -i)));
        }
        moves
    }
}

impl MoveSet for KnightMove {
    fn get_relative_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        moves.push(Move::new((1,2)));
        moves.push(Move::new((1,-2)));
        moves.push(Move::new((-1,2)));
        moves.push(Move::new((-1,-2)));

        moves.push(Move::new((2, 1)));
        moves.push(Move::new((2, -1)));
        moves.push(Move::new((-2, 1)));
        moves.push(Move::new((-2, -1)));

        moves
    }
}