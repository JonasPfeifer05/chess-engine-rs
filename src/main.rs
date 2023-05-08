use std::any::Any;
use chess_engine_rs::game_board::board::Board;
use chess_engine_rs::game_board::piece::{Color, Movable, Piece, Rook};
use chess_engine_rs::game_board::position::{HorizontalPosition, Position, VerticalPosition};
use chess_engine_rs::movement::evaluater::MoveEvaluator;

fn main() {
    let mut board = Board::default();
    board.set_piece(Position::new(HorizontalPosition::A, VerticalPosition::One), Piece::new(Box::new(Rook), Color::White));
    board.set_piece(Position::new(HorizontalPosition::E, VerticalPosition::One), Piece::new(Box::new(Rook), Color::White));

    let rook = Rook;
    for moveset in rook.get_move_sets() {
        println!("{:?}", MoveEvaluator::validate_moves(&moveset.get_relative_moves(), &Position::new(HorizontalPosition::A, VerticalPosition::One), &board).len());
    }
}