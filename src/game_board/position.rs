use std::fmt::{Debug, Formatter};
use crate::general::position::HorizontalPosition::{A, B, C, D, E, F, G};
use crate::general::position::VerticalPosition::{Eight, Five, Four, One, Seven, Six, Three, Two};

/// Given a chess board, VerticalPosition defines the vertical alignment of anything on that board. \
///  \
/// # Example
/// Chess board:
/// 8 . . . . . . . .
/// 7 . . . . . . . .
/// 6 . . . . . . . .
/// 5 . . . . . P . .
/// 4 . . . . . . . .
/// 3 . . . . . . . .
/// 2 . . . . . . . .
/// 1 . . . . . . . .
///   A B C D E F G H
///
/// VerticalPosition of P would be Five
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
    /// Returns a list of every vertical position of a chess engine
    pub fn get_list() -> Vec<VerticalPosition> {
        vec![One, Two, Three, Four, Five, Six, Seven, Eight]
    }
}


/// Given a chess board, HorizontalPosition defines the horizontal alignment of anything on that board. \
///  \
/// # Example
/// Chess board: \
/// 8 . . . . . . . . \
/// 7 . . . . . . . . \
/// 6 . . . . . . . . \
/// 5 . . . . . P . . \
/// 4 . . . . . . . . \
/// 3 . . . . . . . . \
/// 2 . . . . . . . . \
/// 1 . . . . . . . . \
///   A B C D E F G H \
/// \
/// HorizontalPosition of P would be F
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
    /// Returns a list of every horizontal position of a chess engine
    pub fn get_list() -> Vec<HorizontalPosition> {
        vec![A, B, C, D, E, F, G]
    }
}

/// Given a chess board, Position represents any position on that. \
/// \
/// # Example
/// Chess board: \
/// 8 . . . . . . . . \
/// 7 . . . . . . . . \
/// 6 . . . . . . . . \
/// 5 . . . . . P . . \
/// 4 . . . . . . . . \
/// 3 . . . . . . . . \
/// 2 . . . . . . . . \
/// 1 . . . . . . . . \
///   A B C D E F G H \
/// \
/// Position of P would be { horizontal: F; vertical: Five; }
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

/// Used to build an position from certain data
pub struct PositionBuilder;
impl PositionBuilder {
    /// Used to clone a position \
    /// \
    /// # Example
    /// ```
    /// use chess_engine_rs::general::position::{HorizontalPosition, Position, PositionBuilder, VerticalPosition};
    /// let position1 = Position::new(HorizontalPosition::A, VerticalPosition::One);
    /// let position2 = PositionBuilder::clone(&position1);
    ///
    /// assert_eq!(position1, position2);
    /// ```
    /// let pos1 = Posit
    pub fn clone(position: &Position) -> Position {
        Position {
            vertical: position.vertical,
            horizontal: position.horizontal,
        }
    }
}

