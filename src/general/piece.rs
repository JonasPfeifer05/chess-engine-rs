use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use dyn_clone::DynClone;
use lazy_static::lazy_static;
use crate::general::movement::MoveSet;

#[derive(Clone)]
// A collection of values defining a piece on the board
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
}

#[repr(u8)]
#[derive(Clone)]
pub enum Color {
    White,
    Black,
}

// Any figure in the game
pub trait Movable: DynClone + Send + Sync {
    // Get the move-sets for the figure
    fn get_move_sets(&self) -> Vec<Box<dyn MoveSet>>;

    // Get the symbol for a certain movable
    fn get_symbol(&self) -> &str;
}
dyn_clone::clone_trait_object!(Movable);

lazy_static! {
    static ref PIECE_BUILDER: RwLock<PieceBuilder> = RwLock::new(PieceBuilder::default());
}

pub struct PieceBuilder {
    symbol_to_piece: HashMap<char, Piece>,
}

impl Default for PieceBuilder {
    fn default() -> Self {
        Self {
            symbol_to_piece: Default::default(),
        }
    }
}

impl PieceBuilder {
    // Register a symbol for serialization. If it already exists, returns None
    pub fn register_symbol(symbol: char, piece: Piece) -> Option<()> {
        if PIECE_BUILDER.read().unwrap().symbol_to_piece.contains_key(&symbol) { return None; }
        PIECE_BUILDER.write().unwrap().symbol_to_piece.insert(symbol, piece);

        Some(())
    }

    // Returns a copy of the piece associated with the symbol
    pub fn get_from_symbol(symbol: &char) -> Option<Piece> {
        PIECE_BUILDER.read().unwrap().symbol_to_piece.get(symbol).cloned()
    }
}