use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::sync::RwLock;
use dyn_clone::DynClone;
use lazy_static::lazy_static;
use crate::movement::movement::{DiagonalEightBottomLeft, DiagonalEightBottomRight, DiagonalEightTopLeft, DiagonalEightTopRight, DiagonalOneBottomLeft, DiagonalOneBottomRight, DiagonalOneTopLeft, DiagonalOneTopRight, HorizontalEightLeft, HorizontalEightRight, HorizontalOneLeft, HorizontalOneRight, KnightMove, MoveSet, PawnKill, PawnOne, PawnTwo, VerticalEightBottom, VerticalEightUp, VerticalOneBottom, VerticalOneUp};

#[derive(Clone)]
/// A composition of the color of the piece and the movable associated with the piece
pub struct Piece {
    movable: Box<dyn Movable>,
    color: Color,
}

impl Debug for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.movable.get_symbol().to_string().as_str()).expect("TODO: panic message");
        f.write_str(", ").expect("TODO: panic message");
        f.write_str(&format!("{:?}", self.color))
    }
}

impl Piece {
    pub fn movable(&self) -> &Box<dyn Movable> {
        &self.movable
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub(crate) fn set_color(&mut self, color: Color) { self.color = color; }

    pub fn new(movable: Box<dyn Movable>, color: Color) -> Self {
        Self { movable, color }
    }
}

#[repr(u8)]
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

/// The trait that every figure in the game must implement
pub trait Movable: DynClone + Send + Sync {
    /// Gets the move-sets for the figure
    fn get_move_sets(&self) -> Vec<Box<dyn MoveSet>>;

    /// Gets the symbol for the figure
    fn get_symbol(&self) -> char;
    fn get_ascii(&self, color: &Color) -> char;

    fn moved(&mut self) {}
}
dyn_clone::clone_trait_object!(Movable);

lazy_static! {
    static ref PIECE_BUILDER: RwLock<PieceRegistry> = RwLock::new(PieceRegistry::default());
}

pub const UPPER_CASE_IS_WHITE: bool = true;

/// Used to register pieces for serialization
pub struct PieceRegistry {
    symbol_to_piece: HashMap<char, Piece>,
}

impl Default for PieceRegistry {
    fn default() -> Self {
        let mut map: HashMap<char, Piece> = Default::default();

        map.insert('p', Piece::new(Box::new(Pawn::default()), Color::Black));
        map.insert('r', Piece::new(Box::new(Rook), Color::Black));
        map.insert('n', Piece::new(Box::new(Knight), Color::Black));
        map.insert('b', Piece::new(Box::new(Bishop), Color::Black));
        map.insert('q', Piece::new(Box::new(Queen), Color::Black));
        map.insert('k', Piece::new(Box::new(King), Color::Black));

        Self {
            symbol_to_piece: map,
        }
    }
}

impl PieceRegistry {
    /// Returns a copy of the piece associated with the passed symbol
    pub fn get(symbol: &char) -> Option<Piece> {
        let lowercase: char = symbol.to_lowercase().next().unwrap();
        let piece = PIECE_BUILDER.read().unwrap().symbol_to_piece.get(&lowercase).cloned();
        piece.map(|mut piece| {
            if symbol.is_uppercase() && UPPER_CASE_IS_WHITE {
                piece.set_color(Color::White);
            }
            piece
        })
    }
}

#[derive(Clone)]
pub struct Pawn {
    already_moved: bool,
}

impl Default for Pawn {
    fn default() -> Self {
        Self { already_moved: false }
    }
}

#[derive(Clone, Default)]
pub struct Rook;

#[derive(Clone, Default)]
pub struct King;

#[derive(Clone, Default)]
pub struct Queen;

#[derive(Clone, Default)]
pub struct Bishop;

#[derive(Clone, Default)]
pub struct Knight;

impl Movable for Pawn {
    fn get_move_sets(&self) -> Vec<Box<dyn MoveSet>> {
        if self.already_moved {
            vec![Box::new(PawnOne), Box::new(PawnKill)]
        } else {
            vec![Box::new(PawnOne), Box::new(PawnTwo), Box::new(PawnKill)]
        }
    }

    fn get_symbol(&self) -> char {
        'p'
    }

    fn get_ascii(&self, color: &Color) -> char {
        match color {
            Color::White => { '\u{265F}' }
            Color::Black => { '\u{2659}' }
        }
    }

    fn moved(&mut self) {
        self.already_moved = true;
    }
}

impl Movable for Rook {
    fn get_move_sets(&self) -> Vec<Box<dyn MoveSet>> {
        vec![Box::new(HorizontalEightLeft), Box::new(HorizontalEightRight), Box::new(VerticalEightUp), Box::new(VerticalEightBottom)]
    }

    fn get_symbol(&self) -> char {
        'r'
    }

    fn get_ascii(&self, color: &Color) -> char {
        match color {
            Color::White => { '\u{265C}' }
            Color::Black => { '\u{2656}' }
        }
    }
}

impl Movable for King {
    fn get_move_sets(&self) -> Vec<Box<dyn MoveSet>> {
        vec![Box::new(DiagonalOneTopLeft), Box::new(DiagonalOneTopRight), Box::new(DiagonalOneBottomLeft), Box::new(DiagonalOneBottomRight), Box::new(VerticalOneUp), Box::new(VerticalOneBottom), Box::new(HorizontalOneLeft), Box::new(HorizontalOneRight)]
    }

    fn get_symbol(&self) -> char {
        'k'
    }

    fn get_ascii(&self, color: &Color) -> char {
        match color {
            Color::White => { '\u{265A}' }
            Color::Black => { '\u{2654}' }
        }
    }
}

impl Movable for Queen {
    fn get_move_sets(&self) -> Vec<Box<dyn MoveSet>> {
        vec![Box::new(DiagonalEightTopLeft), Box::new(DiagonalEightTopRight), Box::new(DiagonalEightBottomLeft), Box::new(DiagonalEightBottomRight), Box::new(VerticalEightUp), Box::new(VerticalEightBottom), Box::new(HorizontalEightLeft), Box::new(HorizontalEightRight)]
    }

    fn get_symbol(&self) -> char {
        'q'
    }

    fn get_ascii(&self, color: &Color) -> char {
        match color {
            Color::White => { '\u{265B}' }
            Color::Black => { '\u{2655}' }
        }
    }
}

impl Movable for Bishop {
    fn get_move_sets(&self) -> Vec<Box<dyn MoveSet>> {
        vec![Box::new(DiagonalEightTopLeft), Box::new(DiagonalEightTopRight), Box::new(DiagonalEightBottomLeft), Box::new(DiagonalEightBottomRight)]
    }

    fn get_symbol(&self) -> char {
        'b'
    }

    fn get_ascii(&self, color: &Color) -> char {
        match color {
            Color::White => { '\u{265D}' }
            Color::Black => { '\u{2657}' }
        }
    }
}

impl Movable for Knight {
    fn get_move_sets(&self) -> Vec<Box<dyn MoveSet>> {
        vec![Box::new(KnightMove)]
    }

    fn get_symbol(&self) -> char {
        'k'
    }

    fn get_ascii(&self, color: &Color) -> char {
        match color {
            Color::White => { '\u{265A}' }
            Color::Black => { '\u{2654}' }
        }
    }
}