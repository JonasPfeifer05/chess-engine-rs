use std::any::Any;
use std::collections::HashMap;
use std::io::stdin;
use std::net::{Ipv4Addr, SocketAddrV4};
use chess_engine_rs::application::Application;
use chess_engine_rs::csp::command::ClientCommand::Killed;
use chess_engine_rs::csp::csp_parser::CSPParser;
use chess_engine_rs::game_board::board::{Board, BoardBuilder, BoardMemento};
use chess_engine_rs::game_board::piece::{Bishop, King, Knight, Pawn, Piece, PieceRegistry, Queen, Rook};
use chess_engine_rs::game_board::piece::Color::Black;
use chess_engine_rs::game_board::position::HorizontalPosition::A;
use chess_engine_rs::game_board::position::Position;
use chess_engine_rs::game_board::position::VerticalPosition::{One, Three, Two};
use chess_engine_rs::game_state::game_state::{GameState, State};

fn main() {
    PieceRegistry::register_symbol('p', Piece::new(Box::new(Pawn::default()), Black));
    PieceRegistry::register_symbol('r', Piece::new(Box::new(Rook), Black));
    PieceRegistry::register_symbol('n', Piece::new(Box::new(Knight), Black));
    PieceRegistry::register_symbol('b', Piece::new(Box::new(Bishop), Black));
    PieceRegistry::register_symbol('q', Piece::new(Box::new(Queen), Black));
    PieceRegistry::register_symbol('k', Piece::new(Box::new(King), Black));

    let mut application = Application::default();
    application.state.current_state =State::Turn(Black);

    application.movee(&Position::new(A, Two), &Position::new(A, Three)).expect("TODO: panic message");
}