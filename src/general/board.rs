use std::collections::HashMap;
use crate::general::piece::Piece;
use crate::general::position::Position;

// Stores every piece currently on the board.
// Allows to manipulate the positions of pieces on the board without error checking.
pub struct Board {
    position_to_piece: HashMap<Position, Piece>
}

impl Board {
    // Returns the piece on a certain position. If there is no position it return None.
    pub fn get_piece(&self, position: &Position) -> Option<&Piece> {
        self.position_to_piece.get(position)
    }

    // Moves the piece from a position to another position, only if there is a piece on the initial position.
    pub fn move_piece(&mut self, from: &Position, to: Position) {
        if let Some(piece) = self.position_to_piece.remove(from) {
            self.position_to_piece.insert(to, piece);
        }
    }

    // Sets a piece on a certain position. Returns the piece that was there before.
    pub fn set_piece(&mut self, position: Position, piece: Piece) -> Option<Piece> {
        self.position_to_piece.insert(position, piece)
    }
}


// A simple abstraction to creating a board.
pub struct BoardBuilder;
impl BoardBuilder {
    pub fn from_memento(memento: &BoardMemento) -> Board {
        todo!()
    }
}

impl Default for BoardBuilder {
    fn default() -> Self {
        todo!()
    }
}


// A storage to store a certain state of a game.
pub struct BoardMemento {
    fen_string: String,
}

impl BoardMemento {
    pub fn from_board(board: &Board) -> Self {
        todo!()
    }
}