use dyn_clone::DynClone;
use crate::game_board::piece::Color;
use crate::game_state::game_state::{GameState, State};

#[derive(Debug, Clone)]
pub struct Move {
    pub relative_move: (i8, i8),
    can_kill: bool,
    only_with_kill: bool,
    check_in_way: bool,
}

impl Move {
    pub fn new_basic(relative_move: (i8, i8)) -> Self {
        Self {
            relative_move,
            can_kill: true,
            only_with_kill: false,
            check_in_way: true,
        }
    }

    pub fn new_detailed(relative_move: (i8, i8), can_kill: bool, only_with_kill: bool, check_in_way: bool) -> Self {
        Self { relative_move, can_kill, only_with_kill, check_in_way }
    }


    pub fn relative_move(&self) -> (i8, i8) {
        self.relative_move
    }
    pub fn can_kill(&self) -> bool {
        self.can_kill
    }
    pub fn only_with_kill(&self) -> bool {
        self.only_with_kill
    }
    pub fn check_in_way(&self) -> bool {
        self.check_in_way
    }
}

/// A descriptor for a movement pattern like horizontal LeftEight or UpOne and so on
/// For moves that can be blocked by pieces in their way like HorizontalEight, its important, that the moves are ordered in their direction
pub trait MoveSet: DynClone {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move>;
}
dyn_clone::clone_trait_object!(MoveSet);

#[derive(Clone)]
pub struct HorizontalOneLeft;

impl MoveSet for HorizontalOneLeft {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        vec![Move::new_basic((-1, 0))]
    }
}

#[derive(Clone)]
pub struct HorizontalOneRight;

impl MoveSet for HorizontalOneRight {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        vec![Move::new_basic((1, 0))]
    }
}

#[derive(Clone)]
pub struct VerticalOneUp;

impl MoveSet for VerticalOneUp {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        vec![Move::new_basic((0, -1))]
    }
}

#[derive(Clone)]
pub struct VerticalOneBottom;

impl MoveSet for VerticalOneBottom {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        vec![Move::new_basic((0, 1))]
    }
}

#[derive(Clone)]
pub struct HorizontalEightLeft;

impl MoveSet for HorizontalEightLeft {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        let mut moves = Vec::new();
        for i in 1..8 {
            moves.push(Move::new_basic((-i, 0)));
        }
        moves
    }
}

#[derive(Clone)]
pub struct HorizontalEightRight;

impl MoveSet for HorizontalEightRight {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        let mut moves = Vec::new();
        for i in 1..8 {
            moves.push(Move::new_basic((i, 0)));
        }
        moves
    }
}

#[derive(Clone)]
pub struct VerticalEightUp;

impl MoveSet for VerticalEightUp {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        let mut moves = Vec::new();
        for i in 1..8 {
            moves.push(Move::new_basic((0, -i)));
        }
        moves
    }
}

#[derive(Clone)]
pub struct VerticalEightBottom;

impl MoveSet for VerticalEightBottom {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        let mut moves = Vec::new();
        for i in 1..8 {
            moves.push(Move::new_basic((0, i)));
        }
        moves
    }
}

#[derive(Clone)]
pub struct DiagonalOneTopLeft;

impl MoveSet for DiagonalOneTopLeft {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        vec![Move::new_basic((-1, -1))]
    }
}

#[derive(Clone)]
pub struct DiagonalOneTopRight;

impl MoveSet for DiagonalOneTopRight {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        vec![Move::new_basic((1, -1))]
    }
}

#[derive(Clone)]
pub struct DiagonalOneBottomLeft;

impl MoveSet for DiagonalOneBottomLeft {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        vec![Move::new_basic((-1, 1))]
    }
}

#[derive(Clone)]
pub struct DiagonalOneBottomRight;

impl MoveSet for DiagonalOneBottomRight {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        vec![Move::new_basic((1, 1))]
    }
}

#[derive(Clone)]
pub struct DiagonalEightTopLeft;

impl MoveSet for DiagonalEightTopLeft {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        let mut moves = Vec::new();
        for i in 1..8 {
            moves.push(Move::new_basic((-i, -i)))
        }
        moves
    }
}

#[derive(Clone)]
pub struct DiagonalEightTopRight;

impl MoveSet for DiagonalEightTopRight {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        let mut moves = Vec::new();
        for i in 1..8 {
            moves.push(Move::new_basic((i, -i)))
        }
        moves
    }
}

#[derive(Clone)]
pub struct DiagonalEightBottomLeft;

impl MoveSet for DiagonalEightBottomLeft {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        let mut moves = Vec::new();
        for i in 1..8 {
            moves.push(Move::new_basic((-i, i)))
        }
        moves
    }
}

#[derive(Clone)]
pub struct DiagonalEightBottomRight;

impl MoveSet for DiagonalEightBottomRight {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        let mut moves = Vec::new();
        for i in 1..8 {
            moves.push(Move::new_basic((i, i)))
        }
        moves
    }
}

#[derive(Clone)]
pub struct KnightMove;

impl MoveSet for KnightMove {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        let mut moves = Vec::new();

        moves.push(Move::new_basic((1, 2)));
        moves.push(Move::new_basic((1, -2)));
        moves.push(Move::new_basic((-1, 2)));
        moves.push(Move::new_basic((-1, -2)));

        moves.push(Move::new_basic((2, 1)));
        moves.push(Move::new_basic((2, -1)));
        moves.push(Move::new_basic((-2, 1)));
        moves.push(Move::new_basic((-2, -1)));

        moves
    }
}

#[derive(Clone)]
pub struct PawnOne;

impl MoveSet for PawnOne {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        if color == &Color::White {
            vec![Move::new_detailed((0, -1), false, false, true)]
        } else {
            vec![Move::new_detailed((0, 1), false, false, true)]
        }
    }
}

#[derive(Clone)]
pub struct PawnTwo;

impl MoveSet for PawnTwo {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        if color == &Color::White {
            vec![Move::new_detailed((0, -2), false, false, true)]
        } else {
            vec![Move::new_detailed((0, 2), false, false, true)]
        }
    }
}

#[derive(Clone)]
pub struct PawnKill;

impl MoveSet for PawnKill {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        if color == &Color::White {
            vec![Move::new_detailed((1, -1), true, true, true), Move::new_detailed((-1, -1), true, true, true)]
        } else {
            vec![Move::new_detailed((1, 1), true, true, true), Move::new_detailed((-1, 1), true, true, true)]
        }
    }
}