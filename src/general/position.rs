use std::fmt::{Debug, Formatter};
use crate::general::piece::Piece;

#[repr(u8)]
#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum HorizontalPosition {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

#[repr(u8)]
#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum VerticalPosition {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H
}

#[derive(Eq, PartialEq, Hash)]
pub struct Position {
    pub vertical: VerticalPosition,
    pub horizontal: HorizontalPosition,
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

