use std::any::Any;
use std::collections::HashMap;
use std::io::stdin;
use chess_engine_rs::application::Application;
use chess_engine_rs::game_board::board::Board;
use chess_engine_rs::game_board::piece::{Color, Movable, Piece, Rook};
use chess_engine_rs::game_board::position::{HorizontalPosition, Position, VerticalPosition};
use chess_engine_rs::movement::evaluater::MoveEvaluator;

fn main() {
    loop {
        let mut application = Application::default();
        let mut input: String = String::new();

        loop {
            stdin().read_line(&mut input).unwrap();

            application.process_event(input.clone());
        }

    }

}