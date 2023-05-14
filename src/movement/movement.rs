use dyn_clone::DynClone;
use crate::game_board::piece::Color;
use crate::game_state::game_state::{GameState, State};

#[derive(Debug, Clone)]
pub enum Move {
    SingleMove {
        relative_move: (i8,i8),
        can_kill: bool,
        only_with_kill: bool,
    },
    ChainedMove {
        relative_moves: Vec<(i8,i8)>
    }
}

impl Move {
    pub fn new_single(relative_move: (i8, i8)) -> Self {
        Self::SingleMove {
            relative_move,
            can_kill: true,
            only_with_kill: false,
        }
    }

    pub fn new_single_detailed(relative_move: (i8, i8), can_kill: bool, only_with_kill: bool) -> Self {
        Self::SingleMove {
            relative_move,
            can_kill,
            only_with_kill,
        }
    }

    pub fn new_chained(relative_moves: Vec<(i8, i8)>) -> Self {
        Self::ChainedMove {
            relative_moves,
        }
    }

    pub fn relative_move(&self) -> Vec<(i8, i8)> {
        match self {
            Move::SingleMove { relative_move, .. } => { vec![relative_move.clone()] }
            Move::ChainedMove { relative_moves } => { relative_moves.clone() }
        }
    }

    pub fn can_kill(&self) -> bool {
        match self {
            Move::SingleMove { can_kill, .. } => { *can_kill }
            Move::ChainedMove { .. } => { true }
        }
    }

    pub fn only_with_kill(&self) -> bool {
        match self {
            Move::SingleMove { only_with_kill, .. } => { *only_with_kill }
            Move::ChainedMove { .. } => { false }
        }
    }

    pub fn check_in_way(&self) -> bool {
        match self {
            Move::SingleMove { .. } => { false }
            Move::ChainedMove { .. } => { true }
        }
    }
}

/// A descriptor for a movement pattern like horizontal LeftEight or UpOne and so on
/// For moves that can be blocked by pieces in their way like HorizontalEight, its important, that the moves are ordered in their direction
pub trait MoveSet: DynClone {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move>;
}
dyn_clone::clone_trait_object!(MoveSet);

#[derive(Clone)]
pub struct HorizontalOne;

impl MoveSet for HorizontalOne {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        vec![Move::new_single((-1,0)), Move::new_single((1,0))]
    }
}

#[derive(Clone)]
pub struct VerticalOne;

impl MoveSet for VerticalOne {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        vec![Move::new_single((0,-1)), Move::new_single((0,1))]
    }
}

#[derive(Clone)]
pub struct HorizontalEight;

impl MoveSet for HorizontalEight {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        let mut chained_left = Vec::new();
        let mut chained_right = Vec::new();

        for i in 1..8 {
            chained_left.push((-i,0));
            chained_right.push((i,0));
        }

        vec![Move::new_chained(chained_left), Move::new_chained(chained_right)]
    }
}

#[derive(Clone)]
pub struct VerticalEight;

impl MoveSet for VerticalEight {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        let mut chained_up = Vec::new();
        let mut chained_down = Vec::new();

        for i in 1..8 {
            chained_up.push((0, -i));
            chained_down.push((0, i));
        }

        vec![Move::new_chained(chained_up), Move::new_chained(chained_down)]
    }
}

#[derive(Clone)]
pub struct DiagonalOne;

impl MoveSet for DiagonalOne {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        vec![Move::new_single((1,1)),Move::new_single((1,-1)),Move::new_single((-1,1)),Move::new_single((-1,-1))]
    }
}

#[derive(Clone)]
pub struct DiagonalEight;

impl MoveSet for DiagonalEight {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        let mut chained_up_left = Vec::new();
        let mut chained_up_right = Vec::new();
        let mut chained_down_left = Vec::new();
        let mut chained_down_right = Vec::new();

        for i in 1..8 {
            chained_up_left.push((-i, -i));
            chained_up_right.push((-i, i));
            chained_down_left.push((i, -i));
            chained_down_right.push((i, i));
        }

        vec![Move::new_chained(chained_up_left), Move::new_chained(chained_up_right),Move::new_chained(chained_down_left), Move::new_chained(chained_down_right)]
    }
}

#[derive(Clone)]
pub struct KnightMoveSet;

impl MoveSet for KnightMoveSet {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        let mut moves = Vec::new();

        moves.push(Move::new_single((1, 2)));
        moves.push(Move::new_single((1, -2)));
        moves.push(Move::new_single((-1, 2)));
        moves.push(Move::new_single((-1, -2)));

        moves.push(Move::new_single((2, 1)));
        moves.push(Move::new_single((2, -1)));
        moves.push(Move::new_single((-2, 1)));
        moves.push(Move::new_single((-2, -1)));

        moves
    }
}

#[derive(Clone)]
pub struct PawnOne;

impl MoveSet for PawnOne {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        if color == &Color::White {
            vec![Move::new_single_detailed((0, -1), false, false)]
        } else {
            vec![Move::new_single_detailed((0, 1), false, false)]
        }
    }
}

#[derive(Clone)]
pub struct PawnTwo;

impl MoveSet for PawnTwo {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        if color == &Color::White {
            vec![Move::new_single_detailed((0, -2), false, false)]
        } else {
            vec![Move::new_single_detailed((0, 2), false, false)]
        }
    }
}

#[derive(Clone)]
pub struct PawnKill;

impl MoveSet for PawnKill {
    fn get_relative_moves(&self, color: &Color) -> Vec<Move> {
        if color == &Color::White {
            vec![Move::new_single_detailed((1, -1), true, true), Move::new_single_detailed((-1, -1), true, true)]
        } else {
            vec![Move::new_single_detailed((1, 1), true, true), Move::new_single_detailed((-1, 1), true, true)]
        }
    }
}