use std::collections::HashMap;
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

impl Piece {
    pub fn movable(&self) -> &Box<dyn Movable> {
        &self.movable
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn set_color(&mut self, color: Color) { self.color = color; }

    pub fn new(movable: Box<dyn Movable>, color: Color) -> Self {
        Self { movable, color }
    }
}

#[repr(u8)]
#[derive(Clone, Eq, PartialEq)]
pub enum Color {
    White,
    Black,
}

/// The trait that every figure in the game must implement
pub trait Movable: DynClone + Send + Sync {
    /// Gets the move-sets for the figure
    fn get_move_sets(&self) -> Vec<Box<dyn MoveSet>>;

    /// Gets the symbol for the figure
    fn get_symbol(&self) -> &char;

    fn moved(&mut self) {}
}
dyn_clone::clone_trait_object!(Movable);

lazy_static! {
    static ref PIECE_BUILDER: RwLock<PieceRegistry> = RwLock::new(PieceRegistry::default());
}

/// Used to register pieces for serialization
pub struct PieceRegistry {
    symbol_to_piece: HashMap<char, Piece>,
}

impl Default for PieceRegistry {
    fn default() -> Self {
        Self {
            symbol_to_piece: Default::default(),
        }
    }
}

impl PieceRegistry {
    /// Register a symbol mapped to a figure for serialization. If it already exists, returns None
    pub fn register_symbol(symbol: char, piece: Piece) -> Option<()> {
        if PIECE_BUILDER.read().unwrap().symbol_to_piece.contains_key(&symbol) { return None; }
        PIECE_BUILDER.write().unwrap().symbol_to_piece.insert(symbol, piece);

        Some(())
    }

    /// Returns a copy of the piece associated with the passed symbol
    pub fn get_from_symbol(symbol: &char) -> Option<Piece> {
        PIECE_BUILDER.read().unwrap().symbol_to_piece.get(symbol).cloned()
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

#[derive(Clone)]
pub struct Rook;

#[derive(Clone)]
pub struct King;

#[derive(Clone)]
pub struct Queen;

#[derive(Clone)]
pub struct Bishop;

#[derive(Clone)]
pub struct Knight;

impl Movable for Pawn {
    fn get_move_sets(&self) -> Vec<Box<dyn MoveSet>> {
        if self.already_moved {
            vec![Box::new(PawnOne), Box::new(PawnKill)]
        } else {
            vec![Box::new(PawnOne), Box::new(PawnTwo), Box::new(PawnKill)]
        }
    }

    fn get_symbol(&self) -> &char {
        &'p'
    }

    fn moved(&mut self) {
        self.already_moved = true;
    }
}

impl Movable for Rook {
    fn get_move_sets(&self) -> Vec<Box<dyn MoveSet>> {
        vec![Box::new(HorizontalEightLeft), Box::new(HorizontalEightRight), Box::new(VerticalEightUp), Box::new(VerticalEightBottom)]
    }

    fn get_symbol(&self) -> &char {
        &'r'
    }
}

impl Movable for King {
    fn get_move_sets(&self) -> Vec<Box<dyn MoveSet>> {
        vec![Box::new(DiagonalOneTopLeft),Box::new(DiagonalOneTopRight),Box::new(DiagonalOneBottomLeft),Box::new(DiagonalOneBottomRight), Box::new(VerticalOneUp),Box::new(VerticalOneBottom), Box::new(HorizontalOneLeft),Box::new(HorizontalOneRight)]
    }

    fn get_symbol(&self) -> &char {
        &'k'
    }
}

impl Movable for Queen {
    fn get_move_sets(&self) -> Vec<Box<dyn MoveSet>> {
        vec![Box::new(DiagonalEightTopLeft), Box::new(DiagonalEightTopRight), Box::new(DiagonalEightBottomLeft), Box::new(DiagonalEightBottomRight), Box::new(VerticalEightUp), Box::new(VerticalEightBottom), Box::new(HorizontalEightLeft), Box::new(HorizontalEightRight)]
    }

    fn get_symbol(&self) -> &char {
        &'q'
    }
}

impl Movable for Bishop {
    fn get_move_sets(&self) -> Vec<Box<dyn MoveSet>> {
        vec![Box::new(DiagonalEightTopLeft),Box::new(DiagonalEightTopRight),Box::new(DiagonalEightBottomLeft),Box::new(DiagonalEightBottomRight)]
    }

    fn get_symbol(&self) -> &char {
        &'b'
    }
}

impl Movable for Knight {
    fn get_move_sets(&self) -> Vec<Box<dyn MoveSet>> {
        vec![Box::new(KnightMove)]
    }

    fn get_symbol(&self) -> &char {
        &'k'
    }
}