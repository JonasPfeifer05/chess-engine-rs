use std::collections::HashMap;
use crate::game_board::piece::{Piece, PieceRegistry};
use crate::game_board::position::{HorizontalPosition, Position, VerticalPosition};

/// Stores every piece currently on the board.
/// Allows to manipulate the positions of pieces on the board without error checking.
pub struct Board {
    position_to_piece: HashMap<Position, Piece>,
}

impl Board {
    /// Returns the piece on a certain position. If there is no position it return None.
    pub fn get_piece(&self, position: &Position) -> Option<&Piece> {
        self.position_to_piece.get(position)
    }

    /// Moves the piece from a position to another position, only if there is a piece on the initial position.
    pub fn move_piece(&mut self, from: &Position, to: Position) {
        if let Some(piece) = self.position_to_piece.remove(from) {
            self.position_to_piece.insert(to, piece);
        }
    }

    /// Sets a piece on a certain position. Returns the piece that was there before.
    pub fn set_piece(&mut self, position: Position, piece: Piece) -> Option<Piece> {
        self.position_to_piece.insert(position, piece)
    }
}

impl Default for Board {
    /// Empty board
    fn default() -> Self {
        Self {
            position_to_piece: Default::default(),
        }
    }
}


/// A simple abstraction to creating a board.
pub struct BoardBuilder;
impl BoardBuilder {
    /// From FEM String
    pub fn from_memento(memento: &BoardMemento) -> Board {
        let mut board = Board::default();

        let rows: Vec<_> = memento.fen_string.split('/').collect();

        let vertical_positions: Vec<VerticalPosition> = VerticalPosition::get_list();
        let horizontal_positions: Vec<HorizontalPosition> = HorizontalPosition::get_list();

        let mut current_y: usize = 0;
        let mut current_x: usize = 0;
        for row in &rows[..7] {
            for symbol in row.chars() {
                // If its a digit, this means n empty fields, otherwise its a piece
                if let Some(count) = symbol.to_digit(10) {
                    current_x += count as usize;
                } else {
                    let piece = PieceRegistry::get_from_symbol(&symbol).expect("Error while parsing symbol to piece!");
                    board.set_piece(Position::new(horizontal_positions[current_x], vertical_positions[current_y]), piece);
                    current_x += 1;
                }
            }
            current_x = 0;
            current_y += 1;
        }

        board
    }
}

impl Default for BoardBuilder {
    fn default() -> Self {
        todo!()
    }
}


/// A storage to store a certain state of a game.
pub struct BoardMemento {
    fen_string: String,
}

impl BoardMemento {
    /// Generate a FEM string from an existing board
    pub fn from_board(board: &Board) -> Self {
        let mut fen_string = String::new();

        let mut empty_count: u8 = 0;

        for y in VerticalPosition::get_list() {
            for x in HorizontalPosition::get_list() {
                let piece = board.get_piece(&Position::new(x, y));
                // Empty field
                if piece.is_none() {
                    empty_count += 1;
                    continue;
                }

                // We dont want to print 0
                if empty_count != 0 {
                    fen_string.push_str(&empty_count.to_string());
                    empty_count = 0;
                }

                let piece = piece.unwrap();
                fen_string.push(*piece.movable().get_symbol());
            }
            // If the whole row is empty this is needed
            if empty_count != 0 { fen_string.push_str(&empty_count.to_string()); }

            fen_string.push('/');
            empty_count = 0;
        }

        BoardMemento {
            fen_string,
        }
    }
}