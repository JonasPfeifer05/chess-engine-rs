use std::fmt::{Debug, Formatter};
use crate::general::position::HorizontalPosition::{A, B, C, D, E, F, G};
use crate::general::position::VerticalPosition::{Eight, Five, Four, One, Seven, Six, Three, Two};

#[repr(u8)]
#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum VerticalPosition {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl VerticalPosition {
    pub fn get_list() -> Vec<VerticalPosition> {
        vec![One, Two, Three, Four, Five, Six, Seven, Eight]
    }
}


#[repr(u8)]
#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum HorizontalPosition {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl HorizontalPosition {
    pub fn get_list() -> Vec<HorizontalPosition> {
        vec![A, B, C, D, E, F, G]
    }
}

#[derive(Eq, PartialEq, Hash)]
pub struct Position {
    pub vertical: VerticalPosition,
    pub horizontal: HorizontalPosition,
}

impl Position {
    pub fn new(horizontal: HorizontalPosition, vertical: VerticalPosition) -> Self {
        Self { vertical, horizontal }
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}{:?}", self.horizontal, self.vertical))
    }
}

pub struct PositionBuilder;

impl PositionBuilder {
    pub fn clone(position: &Position) -> Position {
        Position {
            vertical: position.vertical,
            horizontal: position.horizontal,
        }
    }
}

