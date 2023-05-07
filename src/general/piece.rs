use crate::general::movement::Movable;

pub struct Piece {
    movable: Box<dyn Movable>,
    color: Color,
}

#[repr(u8)]
pub enum Color {
    White,
    Black
}