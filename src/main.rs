use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use chess_engine_rs::csp::csp_parser::CSPParser;
use chess_engine_rs::game_board::piece::{Bishop, King, Knight, Pawn, Piece, PieceRegistry, Queen, Rook};
use chess_engine_rs::game_board::piece::Color::Black;
use chess_engine_rs::networking::application_map::ApplicationMap;

fn main() {
    PieceRegistry::register_symbol('p', Piece::new(Box::new(Pawn::default()), Black));
    PieceRegistry::register_symbol('r', Piece::new(Box::new(Rook), Black));
    PieceRegistry::register_symbol('n', Piece::new(Box::new(Knight), Black));
    PieceRegistry::register_symbol('b', Piece::new(Box::new(Bishop), Black));
    PieceRegistry::register_symbol('q', Piece::new(Box::new(Queen), Black));
    PieceRegistry::register_symbol('k', Piece::new(Box::new(King), Black));

    let mut application_map = ApplicationMap::default();

    let code = application_map.new_game().unwrap();

    let application = application_map.get_application(code.clone()).unwrap();

    let listener = TcpListener::bind("127.0.0.1:2222").unwrap();

    println!("Waiting for user one!");
    let (mut user_one, _) = listener.accept().unwrap();
    println!("User one connected!");
    println!("Waiting for user two!");
    let (mut user_two, _) = listener.accept().unwrap();
    println!("User two connected!");

    let mut user_one_disconnected = false;
    let mut user_two_disconnected = false;

    let mut command_input_buffer = [0u8; 255];
    let mut peek = [0; 1];

    let mut input;
    let mut command;

    loop {
        // Reconnect use one
        if user_one_disconnected {
            println!("Waiting to reconnect user one!");
            let (stream,_) = listener.accept().unwrap();
            println!("Reconnected user one!");
            user_one = stream;
            user_one_disconnected = false;
        }

        // Reconnect use two
        if user_two_disconnected {
            println!("Waiting to reconnect user two!");
            let (stream,_) = listener.accept().unwrap();
            println!("Reconnected user two!");
            user_two = stream;
            user_two_disconnected = false;
        }

        loop {
            // Check if streams are closed

            if let Err(err) = user_one.peek(&mut peek) {
                println!("User one disconnected!: {err}");
                user_one_disconnected = true;
                break;
            } else if peek[0] == 0 {
                println!("User one disconnected!");
                user_one_disconnected = true;
                break;
            }

            let size = user_one.read(&mut command_input_buffer).expect("Fatal error while reading from user one!");
            input = String::from_utf8_lossy(&command_input_buffer[0..size]).to_string();
            command = CSPParser::parse_client_command(&input);
            println!("User one: {:?}", command);

            if let Err(err) = user_two.peek(&mut peek) {
                println!("User two disconnected!: {err}");
                user_two_disconnected = true;
                break;
            } else if peek[0] == 0 {
                println!("User two disconnected!");
                user_two_disconnected = true;
                break;
            }

            let size = user_two.read(&mut command_input_buffer).expect("Fatal error while reading from user one!");
            input = String::from_utf8_lossy(&command_input_buffer[0..size]).to_string();
            command = CSPParser::parse_client_command(&input);
            println!("User two: {:?}", command);

            command_input_buffer.fill(0);
            peek.fill(0);
        }
    }
}