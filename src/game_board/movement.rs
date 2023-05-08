use dyn_clone::DynClone;
use crate::game_board::piece::Color;
use crate::game_state::game_state::{GameState, State};

/// A descriptor for a movement pattern like horizontal LeftEight or UpOne and so on
pub trait MoveSet: DynClone {
    fn get_relative_moves(&self) -> Vec<(i8, i8)>;
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

impl MoveSet for PawnOne {
    fn get_relative_moves(&self) -> Vec<(i8, i8)> {
        match GameState::state() {
            State::Setup => { vec![] }
            State::Over(_) => { vec![] }
            State::Turn(color) => {
                if color == Color::White {
                    vec![(0,1)]
                } else {
                    vec![(0,-1)]
                }
            }
        }
    }
}

impl MoveSet for PawnTwo {
    fn get_relative_moves(&self) -> Vec<(i8, i8)> {
        match GameState::state() {
            State::Setup => { vec![] }
            State::Over(_) => { vec![] }
            State::Turn(color) => {
                if color == Color::White {
                    vec![(0,2)]
                } else {
                    vec![(0,-2)]
                }
            }
        }
    }
}

impl MoveSet for HorizontalOne {
    fn get_relative_moves(&self) -> Vec<(i8, i8)> {
        vec![(-1, 0), (1, 0)]
    }
}

impl MoveSet for HorizontalEight {
    fn get_relative_moves(&self) -> Vec<(i8, i8)> {
        let mut moves = Vec::new();
        for i in 0..8 {
            moves.push((i, 0));
            moves.push((-i, 0));
        }
        moves
    }
}

impl MoveSet for VerticalOne {
    fn get_relative_moves(&self) -> Vec<(i8, i8)> {
        vec![(0, -1), (0, 1)]
    }
}

impl MoveSet for VerticalEight {
    fn get_relative_moves(&self) -> Vec<(i8, i8)> {
        let mut moves = Vec::new();
        for i in 0..8 {
            moves.push((0, i));
            moves.push((0, -i));
        }
        moves
    }
}

impl MoveSet for DiagonalOne {
    fn get_relative_moves(&self) -> Vec<(i8, i8)> {
        vec![(1, 1), (1, -1), (-1, 1), (-1, -1)]
    }
}

impl MoveSet for DiagonalEight {
    fn get_relative_moves(&self) -> Vec<(i8, i8)> {
        let mut moves = Vec::new();
        for i in 0..8 {
            moves.push((i, i));
            moves.push((i, -i));
            moves.push((-i, i));
            moves.push((-i, -i));
        }
        moves
    }
}

impl MoveSet for KnightMove {
    fn get_relative_moves(&self) -> Vec<(i8, i8)> {
        let mut moves = Vec::new();

        moves.push((1,2));
        moves.push((1,-2));
        moves.push((-1,2));
        moves.push((-1,-2));

        moves.push((2, 1));
        moves.push((2, -1));
        moves.push((-2, 1));
        moves.push((-2, -1));

        moves
    }
}